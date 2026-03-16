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
// Generated from SVD 1.30.00, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:13:42 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"USB 2.0 High-Speed Module"]
unsafe impl ::core::marker::Send for super::Usbhs {}
unsafe impl ::core::marker::Sync for super::Usbhs {}
impl super::Usbhs {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "System Configuration Control Register"]
    #[inline(always)]
    pub const fn syscfg(
        &self,
    ) -> &'static crate::common::Reg<self::Syscfg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Syscfg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "CPU Bus Wait Register"]
    #[inline(always)]
    pub const fn buswait(
        &self,
    ) -> &'static crate::common::Reg<self::Buswait_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Buswait_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "System Configuration Status Register"]
    #[inline(always)]
    pub const fn syssts0(
        &self,
    ) -> &'static crate::common::Reg<self::Syssts0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Syssts0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "PLL Status Register"]
    #[inline(always)]
    pub const fn pllsta(&self) -> &'static crate::common::Reg<self::Pllsta_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pllsta_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "Device State Control Register 0"]
    #[inline(always)]
    pub const fn dvstctr0(
        &self,
    ) -> &'static crate::common::Reg<self::Dvstctr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dvstctr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "USB Test Mode Register"]
    #[inline(always)]
    pub const fn testmode(
        &self,
    ) -> &'static crate::common::Reg<self::Testmode_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Testmode_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "FIFO Port Register"]
    #[inline(always)]
    pub const fn cfifo(&self) -> &'static crate::common::Reg<self::Cfifo_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfifo_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "FIFO Port Register"]
    #[inline(always)]
    pub const fn cfifol(
        &self,
    ) -> &'static crate::common::Reg<self::Cfifol_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfifol_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "FIFO Port Register"]
    #[inline(always)]
    pub const fn cfifoll(
        &self,
    ) -> &'static crate::common::Reg<self::Cfifoll_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfifoll_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "FIFO Port Register"]
    #[inline(always)]
    pub const fn cfifoh(
        &self,
    ) -> &'static crate::common::Reg<self::Cfifoh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfifoh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[doc = "FIFO Port Register"]
    #[inline(always)]
    pub const fn cfifohh(
        &self,
    ) -> &'static crate::common::Reg<self::Cfifohh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfifohh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(23usize),
            )
        }
    }

    #[doc = "FIFO Port Register"]
    #[inline(always)]
    pub const fn d0fifo(
        &self,
    ) -> &'static crate::common::Reg<self::D0Fifo_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D0Fifo_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "FIFO Port Register"]
    #[inline(always)]
    pub const fn d1fifo(
        &self,
    ) -> &'static crate::common::Reg<self::D1Fifo_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D1Fifo_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "FIFO Port Register"]
    #[inline(always)]
    pub const fn d0fifol(
        &self,
    ) -> &'static crate::common::Reg<self::D0Fifol_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D0Fifol_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "FIFO Port Register"]
    #[inline(always)]
    pub const fn d1fifol(
        &self,
    ) -> &'static crate::common::Reg<self::D1Fifol_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D1Fifol_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "FIFO Port Register"]
    #[inline(always)]
    pub const fn d0fifoll(
        &self,
    ) -> &'static crate::common::Reg<self::D0Fifoll_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D0Fifoll_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "FIFO Port Register"]
    #[inline(always)]
    pub const fn d1fifoll(
        &self,
    ) -> &'static crate::common::Reg<self::D1Fifoll_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D1Fifoll_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "FIFO Port Register"]
    #[inline(always)]
    pub const fn d0fifoh(
        &self,
    ) -> &'static crate::common::Reg<self::D0Fifoh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D0Fifoh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(26usize),
            )
        }
    }

    #[doc = "FIFO Port Register"]
    #[inline(always)]
    pub const fn d1fifoh(
        &self,
    ) -> &'static crate::common::Reg<self::D1Fifoh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D1Fifoh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(26usize),
            )
        }
    }

    #[doc = "FIFO Port Register"]
    #[inline(always)]
    pub const fn d0fifohh(
        &self,
    ) -> &'static crate::common::Reg<self::D0Fifohh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D0Fifohh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(27usize),
            )
        }
    }

    #[doc = "FIFO Port Register"]
    #[inline(always)]
    pub const fn d1fifohh(
        &self,
    ) -> &'static crate::common::Reg<self::D1Fifohh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D1Fifohh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(27usize),
            )
        }
    }

    #[doc = "CFIFO Port Selection Register"]
    #[inline(always)]
    pub const fn cfifosel(
        &self,
    ) -> &'static crate::common::Reg<self::Cfifosel_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfifosel_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "FIFO Port Control Register"]
    #[inline(always)]
    pub const fn cfifoctr(
        &self,
    ) -> &'static crate::common::Reg<self::Cfifoctr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfifoctr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(34usize),
            )
        }
    }

    #[doc = "D%sFIFO Port Selection Register"]
    #[inline(always)]
    pub const fn dfifosel(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Dfifosel_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x28usize))
        }
    }
    #[inline(always)]
    pub const fn d0fifosel(
        &self,
    ) -> &'static crate::common::Reg<self::Dfifosel_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dfifosel_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x28usize),
            )
        }
    }
    #[inline(always)]
    pub const fn d1fifosel(
        &self,
    ) -> &'static crate::common::Reg<self::Dfifosel_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dfifosel_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2cusize),
            )
        }
    }

    #[doc = "FIFO Port Control Register"]
    #[inline(always)]
    pub const fn dfifoctr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Dfifoctr_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2ausize))
        }
    }
    #[inline(always)]
    pub const fn d0fifoctr(
        &self,
    ) -> &'static crate::common::Reg<self::Dfifoctr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dfifoctr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn d1fifoctr(
        &self,
    ) -> &'static crate::common::Reg<self::Dfifoctr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dfifoctr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2eusize),
            )
        }
    }

    #[doc = "Interrupt Enable Register 0"]
    #[inline(always)]
    pub const fn intenb0(
        &self,
    ) -> &'static crate::common::Reg<self::Intenb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intenb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Interrupt Enable Register 1"]
    #[inline(always)]
    pub const fn intenb1(
        &self,
    ) -> &'static crate::common::Reg<self::Intenb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intenb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(50usize),
            )
        }
    }

    #[doc = "BRDY Interrupt Enable Register"]
    #[inline(always)]
    pub const fn brdyenb(
        &self,
    ) -> &'static crate::common::Reg<self::Brdyenb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Brdyenb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(54usize),
            )
        }
    }

    #[doc = "NRDY Interrupt Enable Register"]
    #[inline(always)]
    pub const fn nrdyenb(
        &self,
    ) -> &'static crate::common::Reg<self::Nrdyenb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nrdyenb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "BEMP Interrupt Enable Register"]
    #[inline(always)]
    pub const fn bempenb(
        &self,
    ) -> &'static crate::common::Reg<self::Bempenb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bempenb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(58usize),
            )
        }
    }

    #[doc = "SOF Output Configuration Register"]
    #[inline(always)]
    pub const fn sofcfg(
        &self,
    ) -> &'static crate::common::Reg<self::Sofcfg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sofcfg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "PHY Setting Register"]
    #[inline(always)]
    pub const fn physet(
        &self,
    ) -> &'static crate::common::Reg<self::Physet_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Physet_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(62usize),
            )
        }
    }

    #[doc = "Interrupt Status Register 0"]
    #[inline(always)]
    pub const fn intsts0(
        &self,
    ) -> &'static crate::common::Reg<self::Intsts0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intsts0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "Interrupt Status Register 1"]
    #[inline(always)]
    pub const fn intsts1(
        &self,
    ) -> &'static crate::common::Reg<self::Intsts1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Intsts1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66usize),
            )
        }
    }

    #[doc = "BRDY Interrupt Status Register"]
    #[inline(always)]
    pub const fn brdysts(
        &self,
    ) -> &'static crate::common::Reg<self::Brdysts_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Brdysts_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(70usize),
            )
        }
    }

    #[doc = "NRDY Interrupt Status Register"]
    #[inline(always)]
    pub const fn nrdysts(
        &self,
    ) -> &'static crate::common::Reg<self::Nrdysts_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Nrdysts_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[doc = "BEMP Interrupt Status Register"]
    #[inline(always)]
    pub const fn bempsts(
        &self,
    ) -> &'static crate::common::Reg<self::Bempsts_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bempsts_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(74usize),
            )
        }
    }

    #[doc = "Frame Number Register"]
    #[inline(always)]
    pub const fn frmnum(
        &self,
    ) -> &'static crate::common::Reg<self::Frmnum_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Frmnum_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[doc = "µFrame Number Register"]
    #[inline(always)]
    pub const fn ufrmnum(
        &self,
    ) -> &'static crate::common::Reg<self::Ufrmnum_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ufrmnum_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(78usize),
            )
        }
    }

    #[doc = "USB Address Register"]
    #[inline(always)]
    pub const fn usbaddr(
        &self,
    ) -> &'static crate::common::Reg<self::Usbaddr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Usbaddr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "USB Request Type Register"]
    #[inline(always)]
    pub const fn usbreq(
        &self,
    ) -> &'static crate::common::Reg<self::Usbreq_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Usbreq_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "USB Request Value Register"]
    #[inline(always)]
    pub const fn usbval(
        &self,
    ) -> &'static crate::common::Reg<self::Usbval_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Usbval_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(86usize),
            )
        }
    }

    #[doc = "USB Request Index Register"]
    #[inline(always)]
    pub const fn usbindx(
        &self,
    ) -> &'static crate::common::Reg<self::Usbindx_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Usbindx_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "USB Request Length Register"]
    #[inline(always)]
    pub const fn usbleng(
        &self,
    ) -> &'static crate::common::Reg<self::Usbleng_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Usbleng_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(90usize),
            )
        }
    }

    #[doc = "DCP Configuration Register"]
    #[inline(always)]
    pub const fn dcpcfg(
        &self,
    ) -> &'static crate::common::Reg<self::Dcpcfg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dcpcfg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[doc = "DCP Maximum Packet Size Register"]
    #[inline(always)]
    pub const fn dcpmaxp(
        &self,
    ) -> &'static crate::common::Reg<self::Dcpmaxp_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dcpmaxp_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(94usize),
            )
        }
    }

    #[doc = "DCP Control Register"]
    #[inline(always)]
    pub const fn dcpctr(
        &self,
    ) -> &'static crate::common::Reg<self::Dcpctr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dcpctr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "Pipe Window Select Register"]
    #[inline(always)]
    pub const fn pipesel(
        &self,
    ) -> &'static crate::common::Reg<self::Pipesel_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pipesel_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "Pipe Configuration Register"]
    #[inline(always)]
    pub const fn pipecfg(
        &self,
    ) -> &'static crate::common::Reg<self::Pipecfg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pipecfg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = "Pipe Buffer Register"]
    #[inline(always)]
    pub const fn pipebuf(
        &self,
    ) -> &'static crate::common::Reg<self::Pipebuf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pipebuf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(106usize),
            )
        }
    }

    #[doc = "Pipe Maximum Packet Size Register"]
    #[inline(always)]
    pub const fn pipemaxp(
        &self,
    ) -> &'static crate::common::Reg<self::Pipemaxp_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pipemaxp_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[doc = "Pipe Cycle Control Register"]
    #[inline(always)]
    pub const fn pipeperi(
        &self,
    ) -> &'static crate::common::Reg<self::Pipeperi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pipeperi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(110usize),
            )
        }
    }

    #[doc = "Pipe %s Control Register"]
    #[inline(always)]
    pub const fn pipectr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pipectr_SPEC, crate::common::RW>,
        9,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x70usize))
        }
    }
    #[inline(always)]
    pub const fn pipe1ctr(
        &self,
    ) -> &'static crate::common::Reg<self::Pipectr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pipectr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x70usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pipe2ctr(
        &self,
    ) -> &'static crate::common::Reg<self::Pipectr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pipectr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x74usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pipe3ctr(
        &self,
    ) -> &'static crate::common::Reg<self::Pipectr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pipectr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x78usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pipe4ctr(
        &self,
    ) -> &'static crate::common::Reg<self::Pipectr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pipectr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pipe5ctr(
        &self,
    ) -> &'static crate::common::Reg<self::Pipectr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pipectr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pipe6ctr(
        &self,
    ) -> &'static crate::common::Reg<self::Pipectr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pipectr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pipe7ctr(
        &self,
    ) -> &'static crate::common::Reg<self::Pipectr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pipectr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pipe8ctr(
        &self,
    ) -> &'static crate::common::Reg<self::Pipectr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pipectr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x8cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pipe9ctr(
        &self,
    ) -> &'static crate::common::Reg<self::Pipectr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pipectr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x90usize),
            )
        }
    }

    #[doc = "Pipe %s Transaction Counter Enable Register"]
    #[inline(always)]
    pub const fn pipetre(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pipetre_SPEC, crate::common::RW>,
        5,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x90usize))
        }
    }
    #[inline(always)]
    pub const fn pipe1tre(
        &self,
    ) -> &'static crate::common::Reg<self::Pipetre_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pipetre_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x90usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pipe2tre(
        &self,
    ) -> &'static crate::common::Reg<self::Pipetre_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pipetre_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x94usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pipe3tre(
        &self,
    ) -> &'static crate::common::Reg<self::Pipetre_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pipetre_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x98usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pipe4tre(
        &self,
    ) -> &'static crate::common::Reg<self::Pipetre_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pipetre_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pipe5tre(
        &self,
    ) -> &'static crate::common::Reg<self::Pipetre_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pipetre_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa0usize),
            )
        }
    }

    #[doc = "Pipe %s Transaction Counter Register"]
    #[inline(always)]
    pub const fn pipetrn(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pipetrn_SPEC, crate::common::RW>,
        5,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x92usize))
        }
    }
    #[inline(always)]
    pub const fn pipe1trn(
        &self,
    ) -> &'static crate::common::Reg<self::Pipetrn_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pipetrn_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x92usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pipe2trn(
        &self,
    ) -> &'static crate::common::Reg<self::Pipetrn_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pipetrn_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x96usize),
            )
        }
    }
    #[inline(always)]
    pub const fn pipe3trn(
        &self,
    ) -> &'static crate::common::Reg<self::Pipetrn_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pipetrn_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn pipe4trn(
        &self,
    ) -> &'static crate::common::Reg<self::Pipetrn_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pipetrn_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x9eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pipe5trn(
        &self,
    ) -> &'static crate::common::Reg<self::Pipetrn_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pipetrn_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xa2usize),
            )
        }
    }

    #[doc = "Device Address %s Configuration Register"]
    #[inline(always)]
    pub const fn devadd(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Devadd_SPEC, crate::common::RW>,
        10,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xd0usize))
        }
    }
    #[inline(always)]
    pub const fn devadd0(
        &self,
    ) -> &'static crate::common::Reg<self::Devadd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Devadd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn devadd1(
        &self,
    ) -> &'static crate::common::Reg<self::Devadd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Devadd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd2usize),
            )
        }
    }
    #[inline(always)]
    pub const fn devadd2(
        &self,
    ) -> &'static crate::common::Reg<self::Devadd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Devadd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn devadd3(
        &self,
    ) -> &'static crate::common::Reg<self::Devadd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Devadd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd6usize),
            )
        }
    }
    #[inline(always)]
    pub const fn devadd4(
        &self,
    ) -> &'static crate::common::Reg<self::Devadd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Devadd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xd8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn devadd5(
        &self,
    ) -> &'static crate::common::Reg<self::Devadd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Devadd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdausize),
            )
        }
    }
    #[inline(always)]
    pub const fn devadd6(
        &self,
    ) -> &'static crate::common::Reg<self::Devadd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Devadd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdcusize),
            )
        }
    }
    #[inline(always)]
    pub const fn devadd7(
        &self,
    ) -> &'static crate::common::Reg<self::Devadd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Devadd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xdeusize),
            )
        }
    }
    #[inline(always)]
    pub const fn devadd8(
        &self,
    ) -> &'static crate::common::Reg<self::Devadd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Devadd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn devadd9(
        &self,
    ) -> &'static crate::common::Reg<self::Devadd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Devadd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0xe2usize),
            )
        }
    }

    #[doc = "Device Address A Configuration Register"]
    #[inline(always)]
    pub const fn devadda(
        &self,
    ) -> &'static crate::common::Reg<self::Devadda_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Devadda_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(228usize),
            )
        }
    }

    #[doc = "Low Power Control Register"]
    #[inline(always)]
    pub const fn lpctrl(
        &self,
    ) -> &'static crate::common::Reg<self::Lpctrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lpctrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "Low Power Status Register"]
    #[inline(always)]
    pub const fn lpsts(&self) -> &'static crate::common::Reg<self::Lpsts_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lpsts_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(258usize),
            )
        }
    }

    #[doc = "Battery Charging Control Register"]
    #[inline(always)]
    pub const fn bcctrl(
        &self,
    ) -> &'static crate::common::Reg<self::Bcctrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcctrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(320usize),
            )
        }
    }

    #[doc = "Function L1 Control Register 1"]
    #[inline(always)]
    pub const fn pl1ctrl1(
        &self,
    ) -> &'static crate::common::Reg<self::Pl1Ctrl1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pl1Ctrl1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(324usize),
            )
        }
    }

    #[doc = "Function L1 Control Register 2"]
    #[inline(always)]
    pub const fn pl1ctrl2(
        &self,
    ) -> &'static crate::common::Reg<self::Pl1Ctrl2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pl1Ctrl2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(326usize),
            )
        }
    }

    #[doc = "Host L1 Control Register 1"]
    #[inline(always)]
    pub const fn hl1ctrl1(
        &self,
    ) -> &'static crate::common::Reg<self::Hl1Ctrl1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Hl1Ctrl1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(328usize),
            )
        }
    }

    #[doc = "Host L1 Control Register 2"]
    #[inline(always)]
    pub const fn hl1ctrl2(
        &self,
    ) -> &'static crate::common::Reg<self::Hl1Ctrl2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Hl1Ctrl2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(330usize),
            )
        }
    }

    #[doc = "Deep Software Standby USB Transceiver Control/Pin Monitor Register"]
    #[inline(always)]
    pub const fn dpusr0r(
        &self,
    ) -> &'static crate::common::Reg<self::Dpusr0R_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Dpusr0R_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(352usize),
            )
        }
    }

    #[doc = "Deep Software Standby USB Suspend/Resume Interrupt Register"]
    #[inline(always)]
    pub const fn dpusr1r(
        &self,
    ) -> &'static crate::common::Reg<self::Dpusr1R_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpusr1R_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(356usize),
            )
        }
    }

    #[doc = "Deep Software Standby USB Suspend/Resume Interrupt Register"]
    #[inline(always)]
    pub const fn dpusr2r(
        &self,
    ) -> &'static crate::common::Reg<self::Dpusr2R_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpusr2R_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(360usize),
            )
        }
    }

    #[doc = "Deep Software Standby USB Suspend/Resume Command Register"]
    #[inline(always)]
    pub const fn dpusrcr(
        &self,
    ) -> &'static crate::common::Reg<self::Dpusrcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dpusrcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(362usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscfg_SPEC;
impl crate::sealed::RegSpec for Syscfg_SPEC {
    type DataType = u16;
}

#[doc = "System Configuration Control Register"]
pub type Syscfg = crate::RegValueT<Syscfg_SPEC>;

impl Syscfg {
    #[doc = "USBHS Operation Enable"]
    #[inline(always)]
    pub fn usbe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        syscfg::Usbe,
        syscfg::Usbe,
        Syscfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            syscfg::Usbe,
            syscfg::Usbe,
            Syscfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "D+ Line Resistor Control"]
    #[inline(always)]
    pub fn dprpu(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        syscfg::Dprpu,
        syscfg::Dprpu,
        Syscfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            syscfg::Dprpu,
            syscfg::Dprpu,
            Syscfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "D+/D- Line Resistor Control"]
    #[inline(always)]
    pub fn drpd(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        syscfg::Drpd,
        syscfg::Drpd,
        Syscfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            syscfg::Drpd,
            syscfg::Drpd,
            Syscfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Controller Operation Select"]
    #[inline(always)]
    pub fn dcfm(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        syscfg::Dcfm,
        syscfg::Dcfm,
        Syscfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            syscfg::Dcfm,
            syscfg::Dcfm,
            Syscfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "High-Speed Operation Enable"]
    #[inline(always)]
    pub fn hse(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        syscfg::Hse,
        syscfg::Hse,
        Syscfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            syscfg::Hse,
            syscfg::Hse,
            Syscfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Single-ended Receiver Enable"]
    #[inline(always)]
    pub fn cnen(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        syscfg::Cnen,
        syscfg::Cnen,
        Syscfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            syscfg::Cnen,
            syscfg::Cnen,
            Syscfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Syscfg {
    #[inline(always)]
    fn default() -> Syscfg {
        <crate::RegValueT<Syscfg_SPEC> as RegisterValue<_>>::new(32)
    }
}
pub mod syscfg {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usbe_SPEC;
    pub type Usbe = crate::EnumBitfieldStruct<u8, Usbe_SPEC>;
    impl Usbe {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dprpu_SPEC;
    pub type Dprpu = crate::EnumBitfieldStruct<u8, Dprpu_SPEC>;
    impl Dprpu {
        #[doc = "Disable line pull-up"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable line pull-up"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drpd_SPEC;
    pub type Drpd = crate::EnumBitfieldStruct<u8, Drpd_SPEC>;
    impl Drpd {
        #[doc = "Disable line pull-down"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable line pull-down"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcfm_SPEC;
    pub type Dcfm = crate::EnumBitfieldStruct<u8, Dcfm_SPEC>;
    impl Dcfm {
        #[doc = "Select device controller mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select host controller mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hse_SPEC;
    pub type Hse = crate::EnumBitfieldStruct<u8, Hse_SPEC>;
    impl Hse {
        #[doc = "Disable Device controller mode: full-speed Host controller mode: full- or low-speed"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable The controller detects the communication speed"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cnen_SPEC;
    pub type Cnen = crate::EnumBitfieldStruct<u8, Cnen_SPEC>;
    impl Cnen {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Buswait_SPEC;
impl crate::sealed::RegSpec for Buswait_SPEC {
    type DataType = u16;
}

#[doc = "CPU Bus Wait Register"]
pub type Buswait = crate::RegValueT<Buswait_SPEC>;

impl Buswait {
    #[doc = "CPU Bus Access Wait Specification"]
    #[inline(always)]
    pub fn bwait(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Buswait_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Buswait_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Buswait {
    #[inline(always)]
    fn default() -> Buswait {
        <crate::RegValueT<Buswait_SPEC> as RegisterValue<_>>::new(15)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syssts0_SPEC;
impl crate::sealed::RegSpec for Syssts0_SPEC {
    type DataType = u16;
}

#[doc = "System Configuration Status Register"]
pub type Syssts0 = crate::RegValueT<Syssts0_SPEC>;

impl Syssts0 {
    #[doc = "USB Data Line Status Monitor Flag"]
    #[inline(always)]
    pub fn lnst(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Syssts0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Syssts0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "USBHS_ID Pin Monitor Flag"]
    #[inline(always)]
    pub fn idmon(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        syssts0::Idmon,
        syssts0::Idmon,
        Syssts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            syssts0::Idmon,
            syssts0::Idmon,
            Syssts0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "SOF Active Monitor Flag While Host Controller Operation Is Selected"]
    #[inline(always)]
    pub fn sofea(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        syssts0::Sofea,
        syssts0::Sofea,
        Syssts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            syssts0::Sofea,
            syssts0::Sofea,
            Syssts0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Host Sequencer Status Monitor Flag"]
    #[inline(always)]
    pub fn htact(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        syssts0::Htact,
        syssts0::Htact,
        Syssts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            syssts0::Htact,
            syssts0::Htact,
            Syssts0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "External USBHS_OVRCURA/USBHS_O VRCURB Input Pin Monitor Flag"]
    #[inline(always)]
    pub fn ovcmon(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, Syssts0_SPEC, crate::common::R> {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,Syssts0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Syssts0 {
    #[inline(always)]
    fn default() -> Syssts0 {
        <crate::RegValueT<Syssts0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syssts0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idmon_SPEC;
    pub type Idmon = crate::EnumBitfieldStruct<u8, Idmon_SPEC>;
    impl Idmon {
        #[doc = "USBHS_ID pin is low"]
        pub const _0: Self = Self::new(0);

        #[doc = "USBHS_ID pin is high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sofea_SPEC;
    pub type Sofea = crate::EnumBitfieldStruct<u8, Sofea_SPEC>;
    impl Sofea {
        #[doc = "SOF output stopped"]
        pub const _0: Self = Self::new(0);

        #[doc = "SOF output operating"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Htact_SPEC;
    pub type Htact = crate::EnumBitfieldStruct<u8, Htact_SPEC>;
    impl Htact {
        #[doc = "Host sequencer stopped"]
        pub const _0: Self = Self::new(0);

        #[doc = "Host sequencer operating"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pllsta_SPEC;
impl crate::sealed::RegSpec for Pllsta_SPEC {
    type DataType = u16;
}

#[doc = "PLL Status Register"]
pub type Pllsta = crate::RegValueT<Pllsta_SPEC>;

impl Pllsta {
    #[doc = "PLL Lock Flag"]
    #[inline(always)]
    pub fn plllock(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pllsta::Plllock,
        pllsta::Plllock,
        Pllsta_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pllsta::Plllock,
            pllsta::Plllock,
            Pllsta_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pllsta {
    #[inline(always)]
    fn default() -> Pllsta {
        <crate::RegValueT<Pllsta_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pllsta {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Plllock_SPEC;
    pub type Plllock = crate::EnumBitfieldStruct<u8, Plllock_SPEC>;
    impl Plllock {
        #[doc = "PLL not locked"]
        pub const _0: Self = Self::new(0);

        #[doc = "PLL locked"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dvstctr0_SPEC;
impl crate::sealed::RegSpec for Dvstctr0_SPEC {
    type DataType = u16;
}

#[doc = "Device State Control Register 0"]
pub type Dvstctr0 = crate::RegValueT<Dvstctr0_SPEC>;

impl Dvstctr0 {
    #[doc = "USB Bus Reset Status Flag"]
    #[inline(always)]
    pub fn rhst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        dvstctr0::Rhst,
        dvstctr0::Rhst,
        Dvstctr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            dvstctr0::Rhst,
            dvstctr0::Rhst,
            Dvstctr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "USB Bus Operation Enable for the Host Controller Operation"]
    #[inline(always)]
    pub fn uact(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dvstctr0::Uact,
        dvstctr0::Uact,
        Dvstctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dvstctr0::Uact,
            dvstctr0::Uact,
            Dvstctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Resume Signal Output for the Host Controller Operation"]
    #[inline(always)]
    pub fn resume(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dvstctr0::Resume,
        dvstctr0::Resume,
        Dvstctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dvstctr0::Resume,
            dvstctr0::Resume,
            Dvstctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "USB Bus Reset Output for the Host Controller Operation"]
    #[inline(always)]
    pub fn usbrst(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dvstctr0::Usbrst,
        dvstctr0::Usbrst,
        Dvstctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dvstctr0::Usbrst,
            dvstctr0::Usbrst,
            Dvstctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Remote Wakeup Detection Enable for the Host Controller Operation"]
    #[inline(always)]
    pub fn rwupe(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dvstctr0::Rwupe,
        dvstctr0::Rwupe,
        Dvstctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dvstctr0::Rwupe,
            dvstctr0::Rwupe,
            Dvstctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Remote Wakeup Output for the Device Controller Operation"]
    #[inline(always)]
    pub fn wkup(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        dvstctr0::Wkup,
        dvstctr0::Wkup,
        Dvstctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            dvstctr0::Wkup,
            dvstctr0::Wkup,
            Dvstctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "USBHS_VBUSEN Output Pin Control"]
    #[inline(always)]
    pub fn vbusen(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        dvstctr0::Vbusen,
        dvstctr0::Vbusen,
        Dvstctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            dvstctr0::Vbusen,
            dvstctr0::Vbusen,
            Dvstctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "USBHS_EXICEN Output Pin Control"]
    #[inline(always)]
    pub fn exicen(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        dvstctr0::Exicen,
        dvstctr0::Exicen,
        Dvstctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            dvstctr0::Exicen,
            dvstctr0::Exicen,
            Dvstctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Host Negotiation Protocol (HNP) Control"]
    #[inline(always)]
    pub fn hnpbtoa(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dvstctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Dvstctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dvstctr0 {
    #[inline(always)]
    fn default() -> Dvstctr0 {
        <crate::RegValueT<Dvstctr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dvstctr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rhst_SPEC;
    pub type Rhst = crate::EnumBitfieldStruct<u8, Rhst_SPEC>;
    impl Rhst {
        #[doc = "Communication speed indeterminate (powered state or no connection)"]
        pub const _000: Self = Self::new(0);

        #[doc = "Host controller mode Low-speed connection Device controller mode USB bus reset in progress or low-speed connection"]
        pub const _001: Self = Self::new(1);

        #[doc = "Host controller mode Full-speed connection Device controller mode USB bus reset in progress or full-speed connection"]
        pub const _010: Self = Self::new(2);

        #[doc = "Host controller mode High-speed connection Device controller mode USB bus reset in progress or high-speed connection"]
        pub const _011: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uact_SPEC;
    pub type Uact = crate::EnumBitfieldStruct<u8, Uact_SPEC>;
    impl Uact {
        #[doc = "Disable downstream port (disable SOF or micro-SOF transmission)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable downstream port (enable SOF or micro-SOF transmission)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Resume_SPEC;
    pub type Resume = crate::EnumBitfieldStruct<u8, Resume_SPEC>;
    impl Resume {
        #[doc = "Do not output resume signal"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output resume signal"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usbrst_SPEC;
    pub type Usbrst = crate::EnumBitfieldStruct<u8, Usbrst_SPEC>;
    impl Usbrst {
        #[doc = "Do not output USB bus reset signal"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output USB bus reset signal"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rwupe_SPEC;
    pub type Rwupe = crate::EnumBitfieldStruct<u8, Rwupe_SPEC>;
    impl Rwupe {
        #[doc = "Disable downstream port remote wakeup"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable downstream port remote wakeup"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wkup_SPEC;
    pub type Wkup = crate::EnumBitfieldStruct<u8, Wkup_SPEC>;
    impl Wkup {
        #[doc = "Do not output remote wakeup signal"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output remote wakeup signal"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbusen_SPEC;
    pub type Vbusen = crate::EnumBitfieldStruct<u8, Vbusen_SPEC>;
    impl Vbusen {
        #[doc = "Output low on external USBHS_VBUSEN pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high on external USBHS_VBUSEN pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Exicen_SPEC;
    pub type Exicen = crate::EnumBitfieldStruct<u8, Exicen_SPEC>;
    impl Exicen {
        #[doc = "Output low on external USBHS_EXICEN pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "Output high on external USBHS_EXICEN pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Testmode_SPEC;
impl crate::sealed::RegSpec for Testmode_SPEC {
    type DataType = u16;
}

#[doc = "USB Test Mode Register"]
pub type Testmode = crate::RegValueT<Testmode_SPEC>;

impl Testmode {
    #[doc = "Test Mode"]
    #[inline(always)]
    pub fn utst(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Testmode_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Testmode_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Testmode {
    #[inline(always)]
    fn default() -> Testmode {
        <crate::RegValueT<Testmode_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfifo_SPEC;
impl crate::sealed::RegSpec for Cfifo_SPEC {
    type DataType = u32;
}

#[doc = "FIFO Port Register"]
pub type Cfifo = crate::RegValueT<Cfifo_SPEC>;

impl Cfifo {
    #[doc = "Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
    #[inline(always)]
    pub fn fifoport(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cfifo_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cfifo_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfifo {
    #[inline(always)]
    fn default() -> Cfifo {
        <crate::RegValueT<Cfifo_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfifol_SPEC;
impl crate::sealed::RegSpec for Cfifol_SPEC {
    type DataType = u16;
}

#[doc = "FIFO Port Register"]
pub type Cfifol = crate::RegValueT<Cfifol_SPEC>;

impl NoBitfieldReg<Cfifol_SPEC> for Cfifol {}
impl ::core::default::Default for Cfifol {
    #[inline(always)]
    fn default() -> Cfifol {
        <crate::RegValueT<Cfifol_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfifoll_SPEC;
impl crate::sealed::RegSpec for Cfifoll_SPEC {
    type DataType = u8;
}

#[doc = "FIFO Port Register"]
pub type Cfifoll = crate::RegValueT<Cfifoll_SPEC>;

impl NoBitfieldReg<Cfifoll_SPEC> for Cfifoll {}
impl ::core::default::Default for Cfifoll {
    #[inline(always)]
    fn default() -> Cfifoll {
        <crate::RegValueT<Cfifoll_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfifoh_SPEC;
impl crate::sealed::RegSpec for Cfifoh_SPEC {
    type DataType = u16;
}

#[doc = "FIFO Port Register"]
pub type Cfifoh = crate::RegValueT<Cfifoh_SPEC>;

impl Cfifoh {
    #[doc = "Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
    #[inline(always)]
    pub fn fifoport(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Cfifoh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Cfifoh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfifoh {
    #[inline(always)]
    fn default() -> Cfifoh {
        <crate::RegValueT<Cfifoh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfifohh_SPEC;
impl crate::sealed::RegSpec for Cfifohh_SPEC {
    type DataType = u8;
}

#[doc = "FIFO Port Register"]
pub type Cfifohh = crate::RegValueT<Cfifohh_SPEC>;

impl Cfifohh {
    #[doc = "Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
    #[inline(always)]
    pub fn fifoport(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cfifohh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cfifohh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfifohh {
    #[inline(always)]
    fn default() -> Cfifohh {
        <crate::RegValueT<Cfifohh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D0Fifo_SPEC;
impl crate::sealed::RegSpec for D0Fifo_SPEC {
    type DataType = u32;
}

#[doc = "FIFO Port Register"]
pub type D0Fifo = crate::RegValueT<D0Fifo_SPEC>;

impl D0Fifo {
    #[doc = "Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
    #[inline(always)]
    pub fn fifoport(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D0Fifo_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,D0Fifo_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for D0Fifo {
    #[inline(always)]
    fn default() -> D0Fifo {
        <crate::RegValueT<D0Fifo_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D1Fifo_SPEC;
impl crate::sealed::RegSpec for D1Fifo_SPEC {
    type DataType = u32;
}

#[doc = "FIFO Port Register"]
pub type D1Fifo = crate::RegValueT<D1Fifo_SPEC>;

impl D1Fifo {
    #[doc = "Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
    #[inline(always)]
    pub fn fifoport(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, D1Fifo_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,D1Fifo_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for D1Fifo {
    #[inline(always)]
    fn default() -> D1Fifo {
        <crate::RegValueT<D1Fifo_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D0Fifol_SPEC;
impl crate::sealed::RegSpec for D0Fifol_SPEC {
    type DataType = u16;
}

#[doc = "FIFO Port Register"]
pub type D0Fifol = crate::RegValueT<D0Fifol_SPEC>;

impl NoBitfieldReg<D0Fifol_SPEC> for D0Fifol {}
impl ::core::default::Default for D0Fifol {
    #[inline(always)]
    fn default() -> D0Fifol {
        <crate::RegValueT<D0Fifol_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D1Fifol_SPEC;
impl crate::sealed::RegSpec for D1Fifol_SPEC {
    type DataType = u16;
}

#[doc = "FIFO Port Register"]
pub type D1Fifol = crate::RegValueT<D1Fifol_SPEC>;

impl NoBitfieldReg<D1Fifol_SPEC> for D1Fifol {}
impl ::core::default::Default for D1Fifol {
    #[inline(always)]
    fn default() -> D1Fifol {
        <crate::RegValueT<D1Fifol_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D0Fifoll_SPEC;
impl crate::sealed::RegSpec for D0Fifoll_SPEC {
    type DataType = u8;
}

#[doc = "FIFO Port Register"]
pub type D0Fifoll = crate::RegValueT<D0Fifoll_SPEC>;

impl NoBitfieldReg<D0Fifoll_SPEC> for D0Fifoll {}
impl ::core::default::Default for D0Fifoll {
    #[inline(always)]
    fn default() -> D0Fifoll {
        <crate::RegValueT<D0Fifoll_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D1Fifoll_SPEC;
impl crate::sealed::RegSpec for D1Fifoll_SPEC {
    type DataType = u8;
}

#[doc = "FIFO Port Register"]
pub type D1Fifoll = crate::RegValueT<D1Fifoll_SPEC>;

impl NoBitfieldReg<D1Fifoll_SPEC> for D1Fifoll {}
impl ::core::default::Default for D1Fifoll {
    #[inline(always)]
    fn default() -> D1Fifoll {
        <crate::RegValueT<D1Fifoll_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D0Fifoh_SPEC;
impl crate::sealed::RegSpec for D0Fifoh_SPEC {
    type DataType = u16;
}

#[doc = "FIFO Port Register"]
pub type D0Fifoh = crate::RegValueT<D0Fifoh_SPEC>;

impl D0Fifoh {
    #[doc = "Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
    #[inline(always)]
    pub fn fifoport(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, D0Fifoh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,D0Fifoh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for D0Fifoh {
    #[inline(always)]
    fn default() -> D0Fifoh {
        <crate::RegValueT<D0Fifoh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D1Fifoh_SPEC;
impl crate::sealed::RegSpec for D1Fifoh_SPEC {
    type DataType = u16;
}

#[doc = "FIFO Port Register"]
pub type D1Fifoh = crate::RegValueT<D1Fifoh_SPEC>;

impl D1Fifoh {
    #[doc = "Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
    #[inline(always)]
    pub fn fifoport(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, D1Fifoh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,D1Fifoh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for D1Fifoh {
    #[inline(always)]
    fn default() -> D1Fifoh {
        <crate::RegValueT<D1Fifoh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D0Fifohh_SPEC;
impl crate::sealed::RegSpec for D0Fifohh_SPEC {
    type DataType = u8;
}

#[doc = "FIFO Port Register"]
pub type D0Fifohh = crate::RegValueT<D0Fifohh_SPEC>;

impl D0Fifohh {
    #[doc = "Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
    #[inline(always)]
    pub fn fifoport(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, D0Fifohh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,D0Fifohh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for D0Fifohh {
    #[inline(always)]
    fn default() -> D0Fifohh {
        <crate::RegValueT<D0Fifohh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D1Fifohh_SPEC;
impl crate::sealed::RegSpec for D1Fifohh_SPEC {
    type DataType = u8;
}

#[doc = "FIFO Port Register"]
pub type D1Fifohh = crate::RegValueT<D1Fifohh_SPEC>;

impl D1Fifohh {
    #[doc = "Read receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
    #[inline(always)]
    pub fn fifoport(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, D1Fifohh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,D1Fifohh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for D1Fifohh {
    #[inline(always)]
    fn default() -> D1Fifohh {
        <crate::RegValueT<D1Fifohh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfifosel_SPEC;
impl crate::sealed::RegSpec for Cfifosel_SPEC {
    type DataType = u16;
}

#[doc = "CFIFO Port Selection Register"]
pub type Cfifosel = crate::RegValueT<Cfifosel_SPEC>;

impl Cfifosel {
    #[doc = "FIFO Port Access Pipe Specification"]
    #[inline(always)]
    pub fn curpipe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        cfifosel::Curpipe,
        cfifosel::Curpipe,
        Cfifosel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            cfifosel::Curpipe,
            cfifosel::Curpipe,
            Cfifosel_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "FIFO Port Access Direction when DCP Is Selected"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        cfifosel::Isel,
        cfifosel::Isel,
        Cfifosel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            cfifosel::Isel,
            cfifosel::Isel,
            Cfifosel_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "FIFO Port Endian Control"]
    #[inline(always)]
    pub fn bigend(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        cfifosel::Bigend,
        cfifosel::Bigend,
        Cfifosel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            cfifosel::Bigend,
            cfifosel::Bigend,
            Cfifosel_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CFIFO Port Access Bit Width"]
    #[inline(always)]
    pub fn mbw(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        cfifosel::Mbw,
        cfifosel::Mbw,
        Cfifosel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            cfifosel::Mbw,
            cfifosel::Mbw,
            Cfifosel_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Buffer Pointer Rewind"]
    #[inline(always)]
    pub fn rew(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        cfifosel::Rew,
        cfifosel::Rew,
        Cfifosel_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            cfifosel::Rew,
            cfifosel::Rew,
            Cfifosel_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Read Count Mode"]
    #[inline(always)]
    pub fn rcnt(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        cfifosel::Rcnt,
        cfifosel::Rcnt,
        Cfifosel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            cfifosel::Rcnt,
            cfifosel::Rcnt,
            Cfifosel_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfifosel {
    #[inline(always)]
    fn default() -> Cfifosel {
        <crate::RegValueT<Cfifosel_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfifosel {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Curpipe_SPEC;
    pub type Curpipe = crate::EnumBitfieldStruct<u8, Curpipe_SPEC>;
    impl Curpipe {
        #[doc = "DCP (default control pipe)"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "Pipe 1"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "Pipe 2"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "Pipe 3"]
        pub const _0_X_3: Self = Self::new(3);

        #[doc = "Pipe 4"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "Pipe 5"]
        pub const _0_X_5: Self = Self::new(5);

        #[doc = "Pipe 6"]
        pub const _0_X_6: Self = Self::new(6);

        #[doc = "Pipe 7"]
        pub const _0_X_7: Self = Self::new(7);

        #[doc = "Pipe 8"]
        pub const _0_X_8: Self = Self::new(8);

        #[doc = "Pipe 9"]
        pub const _0_X_9: Self = Self::new(9);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Select reading from the FIFO buffer"]
        pub const _0: Self = Self::new(0);

        #[doc = "Select writing to the FIFO buffer"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bigend_SPEC;
    pub type Bigend = crate::EnumBitfieldStruct<u8, Bigend_SPEC>;
    impl Bigend {
        #[doc = "Little endian"]
        pub const _0: Self = Self::new(0);

        #[doc = "Big endian"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbw_SPEC;
    pub type Mbw = crate::EnumBitfieldStruct<u8, Mbw_SPEC>;
    impl Mbw {
        #[doc = "8-bit width"]
        pub const _00: Self = Self::new(0);

        #[doc = "16-bit width"]
        pub const _01: Self = Self::new(1);

        #[doc = "32-bit width"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rew_SPEC;
    pub type Rew = crate::EnumBitfieldStruct<u8, Rew_SPEC>;
    impl Rew {
        #[doc = "Do not rewind buffer pointer (Writing 0 has no effect.)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Rewind buffer pointer"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcnt_SPEC;
    pub type Rcnt = crate::EnumBitfieldStruct<u8, Rcnt_SPEC>;
    impl Rcnt {
        #[doc = "Clear DTLN\\[11:0\\] flags in the FIFO port control register to 0x000 when all receive data is read from CFIFO"]
        pub const _0: Self = Self::new(0);

        #[doc = "Decrement DTLN\\[11:0\\] flags each time receive data is read from CFIFO"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfifoctr_SPEC;
impl crate::sealed::RegSpec for Cfifoctr_SPEC {
    type DataType = u16;
}

#[doc = "FIFO Port Control Register"]
pub type Cfifoctr = crate::RegValueT<Cfifoctr_SPEC>;

impl Cfifoctr {
    #[doc = "Receive Data Length Flag"]
    #[inline(always)]
    pub fn dtln(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Cfifoctr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Cfifoctr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "FIFO Port Ready Flag"]
    #[inline(always)]
    pub fn frdy(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        cfifoctr::Frdy,
        cfifoctr::Frdy,
        Cfifoctr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            cfifoctr::Frdy,
            cfifoctr::Frdy,
            Cfifoctr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CPU Buffer Clear"]
    #[inline(always)]
    pub fn bclr(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        cfifoctr::Bclr,
        cfifoctr::Bclr,
        Cfifoctr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            cfifoctr::Bclr,
            cfifoctr::Bclr,
            Cfifoctr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "FIFO Buffer Valid Flag"]
    #[inline(always)]
    pub fn bval(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        cfifoctr::Bval,
        cfifoctr::Bval,
        Cfifoctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            cfifoctr::Bval,
            cfifoctr::Bval,
            Cfifoctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cfifoctr {
    #[inline(always)]
    fn default() -> Cfifoctr {
        <crate::RegValueT<Cfifoctr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfifoctr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Frdy_SPEC;
    pub type Frdy = crate::EnumBitfieldStruct<u8, Frdy_SPEC>;
    impl Frdy {
        #[doc = "FIFO port access disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "FIFO port access enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bclr_SPEC;
    pub type Bclr = crate::EnumBitfieldStruct<u8, Bclr_SPEC>;
    impl Bclr {
        #[doc = "No operation (writing 0 has no effect)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear FIFO buffer on the CPU side"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bval_SPEC;
    pub type Bval = crate::EnumBitfieldStruct<u8, Bval_SPEC>;
    impl Bval {
        #[doc = "Invalid (writing 0 has no effect)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Writing ended"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfifosel_SPEC;
impl crate::sealed::RegSpec for Dfifosel_SPEC {
    type DataType = u16;
}

#[doc = "D%sFIFO Port Selection Register"]
pub type Dfifosel = crate::RegValueT<Dfifosel_SPEC>;

impl Dfifosel {
    #[doc = "FIFO Port Access Pipe Specification"]
    #[inline(always)]
    pub fn curpipe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        dfifosel::Curpipe,
        dfifosel::Curpipe,
        Dfifosel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            dfifosel::Curpipe,
            dfifosel::Curpipe,
            Dfifosel_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "FIFO Port Endian Control"]
    #[inline(always)]
    pub fn bigend(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        dfifosel::Bigend,
        dfifosel::Bigend,
        Dfifosel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            dfifosel::Bigend,
            dfifosel::Bigend,
            Dfifosel_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "FIFO Port Access Bit Width"]
    #[inline(always)]
    pub fn mbw(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        dfifosel::Mbw,
        dfifosel::Mbw,
        Dfifosel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            dfifosel::Mbw,
            dfifosel::Mbw,
            Dfifosel_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DMA/DTC Transfer Request Enable"]
    #[inline(always)]
    pub fn dreqe(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        dfifosel::Dreqe,
        dfifosel::Dreqe,
        Dfifosel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            dfifosel::Dreqe,
            dfifosel::Dreqe,
            Dfifosel_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Auto FIFO Buffer Clear Mode after Specified Pipe is Read"]
    #[inline(always)]
    pub fn dclrm(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        dfifosel::Dclrm,
        dfifosel::Dclrm,
        Dfifosel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            dfifosel::Dclrm,
            dfifosel::Dclrm,
            Dfifosel_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Buffer Pointer Rewind"]
    #[inline(always)]
    pub fn rew(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        dfifosel::Rew,
        dfifosel::Rew,
        Dfifosel_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            dfifosel::Rew,
            dfifosel::Rew,
            Dfifosel_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Read Count Mode"]
    #[inline(always)]
    pub fn rcnt(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        dfifosel::Rcnt,
        dfifosel::Rcnt,
        Dfifosel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            dfifosel::Rcnt,
            dfifosel::Rcnt,
            Dfifosel_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dfifosel {
    #[inline(always)]
    fn default() -> Dfifosel {
        <crate::RegValueT<Dfifosel_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dfifosel {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Curpipe_SPEC;
    pub type Curpipe = crate::EnumBitfieldStruct<u8, Curpipe_SPEC>;
    impl Curpipe {
        #[doc = "No pipe specification"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "Pipe 1"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "Pipe 2"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "Pipe 3"]
        pub const _0_X_3: Self = Self::new(3);

        #[doc = "Pipe 4"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "Pipe 5"]
        pub const _0_X_5: Self = Self::new(5);

        #[doc = "Pipe 6"]
        pub const _0_X_6: Self = Self::new(6);

        #[doc = "Pipe 7"]
        pub const _0_X_7: Self = Self::new(7);

        #[doc = "Pipe 8"]
        pub const _0_X_8: Self = Self::new(8);

        #[doc = "Pipe 9"]
        pub const _0_X_9: Self = Self::new(9);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bigend_SPEC;
    pub type Bigend = crate::EnumBitfieldStruct<u8, Bigend_SPEC>;
    impl Bigend {
        #[doc = "Little endian"]
        pub const _0: Self = Self::new(0);

        #[doc = "Big endian"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbw_SPEC;
    pub type Mbw = crate::EnumBitfieldStruct<u8, Mbw_SPEC>;
    impl Mbw {
        #[doc = "8-bit width"]
        pub const _00: Self = Self::new(0);

        #[doc = "16-bit width"]
        pub const _01: Self = Self::new(1);

        #[doc = "32-bit width"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dreqe_SPEC;
    pub type Dreqe = crate::EnumBitfieldStruct<u8, Dreqe_SPEC>;
    impl Dreqe {
        #[doc = "Disable DMA/DTC transfer request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable DMA/DTC transfer request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dclrm_SPEC;
    pub type Dclrm = crate::EnumBitfieldStruct<u8, Dclrm_SPEC>;
    impl Dclrm {
        #[doc = "Disable auto buffer clear mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable auto buffer clear mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rew_SPEC;
    pub type Rew = crate::EnumBitfieldStruct<u8, Rew_SPEC>;
    impl Rew {
        #[doc = "Do not rewind buffer pointer (writing 0 has no effect)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Rewind buffer pointer"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcnt_SPEC;
    pub type Rcnt = crate::EnumBitfieldStruct<u8, Rcnt_SPEC>;
    impl Rcnt {
        #[doc = "Clear DTLN\\[11:0\\] flags in the FIFO port control register to 0x000 when all receive data is read from DnFIFO (after read of a single plane in double buffer mode)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Decrement DTLN\\[11:0\\] flags each time receive data is read from DnFIFO"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfifoctr_SPEC;
impl crate::sealed::RegSpec for Dfifoctr_SPEC {
    type DataType = u16;
}

#[doc = "FIFO Port Control Register"]
pub type Dfifoctr = crate::RegValueT<Dfifoctr_SPEC>;

impl Dfifoctr {
    #[doc = "Receive Data Length Flag"]
    #[inline(always)]
    pub fn dtln(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Dfifoctr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Dfifoctr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "FIFO Port Ready Flag"]
    #[inline(always)]
    pub fn frdy(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        dfifoctr::Frdy,
        dfifoctr::Frdy,
        Dfifoctr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            dfifoctr::Frdy,
            dfifoctr::Frdy,
            Dfifoctr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CPU Buffer Clear"]
    #[inline(always)]
    pub fn bclr(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        dfifoctr::Bclr,
        dfifoctr::Bclr,
        Dfifoctr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            dfifoctr::Bclr,
            dfifoctr::Bclr,
            Dfifoctr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "FIFO Buffer Valid Flag"]
    #[inline(always)]
    pub fn bval(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        dfifoctr::Bval,
        dfifoctr::Bval,
        Dfifoctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            dfifoctr::Bval,
            dfifoctr::Bval,
            Dfifoctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dfifoctr {
    #[inline(always)]
    fn default() -> Dfifoctr {
        <crate::RegValueT<Dfifoctr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dfifoctr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Frdy_SPEC;
    pub type Frdy = crate::EnumBitfieldStruct<u8, Frdy_SPEC>;
    impl Frdy {
        #[doc = "FIFO port access disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "FIFO port access enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bclr_SPEC;
    pub type Bclr = crate::EnumBitfieldStruct<u8, Bclr_SPEC>;
    impl Bclr {
        #[doc = "No operation (writing 0 has no effect)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear FIFO buffer on the CPU side"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bval_SPEC;
    pub type Bval = crate::EnumBitfieldStruct<u8, Bval_SPEC>;
    impl Bval {
        #[doc = "Invalid (writing 0 has no effect)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Writing ended"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenb0_SPEC;
impl crate::sealed::RegSpec for Intenb0_SPEC {
    type DataType = u16;
}

#[doc = "Interrupt Enable Register 0"]
pub type Intenb0 = crate::RegValueT<Intenb0_SPEC>;

impl Intenb0 {
    #[doc = "Buffer Ready Interrupt Request Enable"]
    #[inline(always)]
    pub fn brdye(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        intenb0::Brdye,
        intenb0::Brdye,
        Intenb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            intenb0::Brdye,
            intenb0::Brdye,
            Intenb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Buffer Not Ready Response Interrupt Request Enable"]
    #[inline(always)]
    pub fn nrdye(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        intenb0::Nrdye,
        intenb0::Nrdye,
        Intenb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            intenb0::Nrdye,
            intenb0::Nrdye,
            Intenb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Buffer Empty Interrupt Request Enable"]
    #[inline(always)]
    pub fn bempe(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        intenb0::Bempe,
        intenb0::Bempe,
        Intenb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            intenb0::Bempe,
            intenb0::Bempe,
            Intenb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Control Transfer Stage Transition Interrupt Request Enable"]
    #[inline(always)]
    pub fn ctre(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        intenb0::Ctre,
        intenb0::Ctre,
        Intenb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            intenb0::Ctre,
            intenb0::Ctre,
            Intenb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device State Transition Interrupt Request Enable"]
    #[inline(always)]
    pub fn dvse(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        intenb0::Dvse,
        intenb0::Dvse,
        Intenb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            intenb0::Dvse,
            intenb0::Dvse,
            Intenb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Frame Number Update Interrupt Request Enable"]
    #[inline(always)]
    pub fn sofe(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        intenb0::Sofe,
        intenb0::Sofe,
        Intenb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            intenb0::Sofe,
            intenb0::Sofe,
            Intenb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Resume Interrupt Request Enable"]
    #[inline(always)]
    pub fn rsme(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        intenb0::Rsme,
        intenb0::Rsme,
        Intenb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            intenb0::Rsme,
            intenb0::Rsme,
            Intenb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VBUS Interrupt Request Enable"]
    #[inline(always)]
    pub fn vbse(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        intenb0::Vbse,
        intenb0::Vbse,
        Intenb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            intenb0::Vbse,
            intenb0::Vbse,
            Intenb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Intenb0 {
    #[inline(always)]
    fn default() -> Intenb0 {
        <crate::RegValueT<Intenb0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod intenb0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brdye_SPEC;
    pub type Brdye = crate::EnumBitfieldStruct<u8, Brdye_SPEC>;
    impl Brdye {
        #[doc = "Disable interrupt request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable interrupt request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nrdye_SPEC;
    pub type Nrdye = crate::EnumBitfieldStruct<u8, Nrdye_SPEC>;
    impl Nrdye {
        #[doc = "Disable interrupt request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable interrupt request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bempe_SPEC;
    pub type Bempe = crate::EnumBitfieldStruct<u8, Bempe_SPEC>;
    impl Bempe {
        #[doc = "Disable interrupt request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable interrupt request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctre_SPEC;
    pub type Ctre = crate::EnumBitfieldStruct<u8, Ctre_SPEC>;
    impl Ctre {
        #[doc = "Disable interrupt request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable interrupt request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvse_SPEC;
    pub type Dvse = crate::EnumBitfieldStruct<u8, Dvse_SPEC>;
    impl Dvse {
        #[doc = "Disable interrupt request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable interrupt request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sofe_SPEC;
    pub type Sofe = crate::EnumBitfieldStruct<u8, Sofe_SPEC>;
    impl Sofe {
        #[doc = "Disable interrupt request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable interrupt request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsme_SPEC;
    pub type Rsme = crate::EnumBitfieldStruct<u8, Rsme_SPEC>;
    impl Rsme {
        #[doc = "Disable interrupt request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable interrupt request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbse_SPEC;
    pub type Vbse = crate::EnumBitfieldStruct<u8, Vbse_SPEC>;
    impl Vbse {
        #[doc = "Disable interrupt request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable interrupt request"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenb1_SPEC;
impl crate::sealed::RegSpec for Intenb1_SPEC {
    type DataType = u16;
}

#[doc = "Interrupt Enable Register 1"]
pub type Intenb1 = crate::RegValueT<Intenb1_SPEC>;

impl Intenb1 {
    #[doc = "PDDETINT Detection Interrupt Request Enable"]
    #[inline(always)]
    pub fn pddetinte(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        intenb1::Pddetinte,
        intenb1::Pddetinte,
        Intenb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            intenb1::Pddetinte,
            intenb1::Pddetinte,
            Intenb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Setup Transaction Normal Response Interrupt Request Enable"]
    #[inline(always)]
    pub fn sacke(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        intenb1::Sacke,
        intenb1::Sacke,
        Intenb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            intenb1::Sacke,
            intenb1::Sacke,
            Intenb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Setup Transaction Error Interrupt Request Enable"]
    #[inline(always)]
    pub fn signe(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        intenb1::Signe,
        intenb1::Signe,
        Intenb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            intenb1::Signe,
            intenb1::Signe,
            Intenb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "EOF Error Detection Interrupt Request Enable"]
    #[inline(always)]
    pub fn eoferre(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        intenb1::Eoferre,
        intenb1::Eoferre,
        Intenb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            intenb1::Eoferre,
            intenb1::Eoferre,
            Intenb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LPM Transaction End Interrupt Request Enable"]
    #[inline(always)]
    pub fn lpmende(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        intenb1::Lpmende,
        intenb1::Lpmende,
        Intenb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            intenb1::Lpmende,
            intenb1::Lpmende,
            Intenb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "L1 Resume End Interrupt Enable"]
    #[inline(always)]
    pub fn l1rsmende(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        intenb1::L1Rsmende,
        intenb1::L1Rsmende,
        Intenb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            intenb1::L1Rsmende,
            intenb1::L1Rsmende,
            Intenb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Connection Detection Interrupt Request Enable"]
    #[inline(always)]
    pub fn attche(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        intenb1::Attche,
        intenb1::Attche,
        Intenb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            intenb1::Attche,
            intenb1::Attche,
            Intenb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Disconnection Detection Interrupt Request Enable"]
    #[inline(always)]
    pub fn dtche(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        intenb1::Dtche,
        intenb1::Dtche,
        Intenb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            intenb1::Dtche,
            intenb1::Dtche,
            Intenb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "USB Bus Change Interrupt Request Enable"]
    #[inline(always)]
    pub fn bchge(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        intenb1::Bchge,
        intenb1::Bchge,
        Intenb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            intenb1::Bchge,
            intenb1::Bchge,
            Intenb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "OVRCRE Interrupt Request Enable"]
    #[inline(always)]
    pub fn ovrcre(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        intenb1::Ovrcre,
        intenb1::Ovrcre,
        Intenb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            intenb1::Ovrcre,
            intenb1::Ovrcre,
            Intenb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Intenb1 {
    #[inline(always)]
    fn default() -> Intenb1 {
        <crate::RegValueT<Intenb1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod intenb1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pddetinte_SPEC;
    pub type Pddetinte = crate::EnumBitfieldStruct<u8, Pddetinte_SPEC>;
    impl Pddetinte {
        #[doc = "Disable interrupt request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable interrupt request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sacke_SPEC;
    pub type Sacke = crate::EnumBitfieldStruct<u8, Sacke_SPEC>;
    impl Sacke {
        #[doc = "Disable interrupt request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable interrupt request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Signe_SPEC;
    pub type Signe = crate::EnumBitfieldStruct<u8, Signe_SPEC>;
    impl Signe {
        #[doc = "Disable interrupt request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable interrupt request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eoferre_SPEC;
    pub type Eoferre = crate::EnumBitfieldStruct<u8, Eoferre_SPEC>;
    impl Eoferre {
        #[doc = "Disable interrupt request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable interrupt request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lpmende_SPEC;
    pub type Lpmende = crate::EnumBitfieldStruct<u8, Lpmende_SPEC>;
    impl Lpmende {
        #[doc = "Disable interrupt request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable interrupt request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L1Rsmende_SPEC;
    pub type L1Rsmende = crate::EnumBitfieldStruct<u8, L1Rsmende_SPEC>;
    impl L1Rsmende {
        #[doc = "Disable interrupt request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable interrupt request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Attche_SPEC;
    pub type Attche = crate::EnumBitfieldStruct<u8, Attche_SPEC>;
    impl Attche {
        #[doc = "Disable interrupt request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable interrupt request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtche_SPEC;
    pub type Dtche = crate::EnumBitfieldStruct<u8, Dtche_SPEC>;
    impl Dtche {
        #[doc = "Disable interrupt request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable interrupt request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bchge_SPEC;
    pub type Bchge = crate::EnumBitfieldStruct<u8, Bchge_SPEC>;
    impl Bchge {
        #[doc = "Disable interrupt request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable interrupt request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovrcre_SPEC;
    pub type Ovrcre = crate::EnumBitfieldStruct<u8, Ovrcre_SPEC>;
    impl Ovrcre {
        #[doc = "Disable interrupt request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable interrupt request"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Brdyenb_SPEC;
impl crate::sealed::RegSpec for Brdyenb_SPEC {
    type DataType = u16;
}

#[doc = "BRDY Interrupt Enable Register"]
pub type Brdyenb = crate::RegValueT<Brdyenb_SPEC>;

impl Brdyenb {
    #[doc = "BRDY Interrupt Request Enable for Pipes \\[9:0\\]"]
    #[inline(always)]
    pub fn pipebrdye(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        brdyenb::Pipebrdye,
        brdyenb::Pipebrdye,
        Brdyenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            brdyenb::Pipebrdye,
            brdyenb::Pipebrdye,
            Brdyenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Brdyenb {
    #[inline(always)]
    fn default() -> Brdyenb {
        <crate::RegValueT<Brdyenb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod brdyenb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipebrdye_SPEC;
    pub type Pipebrdye = crate::EnumBitfieldStruct<u8, Pipebrdye_SPEC>;
    impl Pipebrdye {
        #[doc = "Disable interrupt request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable interrupt request"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nrdyenb_SPEC;
impl crate::sealed::RegSpec for Nrdyenb_SPEC {
    type DataType = u16;
}

#[doc = "NRDY Interrupt Enable Register"]
pub type Nrdyenb = crate::RegValueT<Nrdyenb_SPEC>;

impl Nrdyenb {
    #[doc = "NRDY Interrupt Enable for Pipes \\[9:0\\]"]
    #[inline(always)]
    pub fn pipenrdye(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        nrdyenb::Pipenrdye,
        nrdyenb::Pipenrdye,
        Nrdyenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            nrdyenb::Pipenrdye,
            nrdyenb::Pipenrdye,
            Nrdyenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Nrdyenb {
    #[inline(always)]
    fn default() -> Nrdyenb {
        <crate::RegValueT<Nrdyenb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod nrdyenb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipenrdye_SPEC;
    pub type Pipenrdye = crate::EnumBitfieldStruct<u8, Pipenrdye_SPEC>;
    impl Pipenrdye {
        #[doc = "Disable interrupt request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable interrupt request"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bempenb_SPEC;
impl crate::sealed::RegSpec for Bempenb_SPEC {
    type DataType = u16;
}

#[doc = "BEMP Interrupt Enable Register"]
pub type Bempenb = crate::RegValueT<Bempenb_SPEC>;

impl Bempenb {
    #[doc = "BEMP Interrupt Enable for Pipes \\[9:0\\]"]
    #[inline(always)]
    pub fn pipebempe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        bempenb::Pipebempe,
        bempenb::Pipebempe,
        Bempenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            bempenb::Pipebempe,
            bempenb::Pipebempe,
            Bempenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bempenb {
    #[inline(always)]
    fn default() -> Bempenb {
        <crate::RegValueT<Bempenb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bempenb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipebempe_SPEC;
    pub type Pipebempe = crate::EnumBitfieldStruct<u8, Pipebempe_SPEC>;
    impl Pipebempe {
        #[doc = "Disable interrupt request"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable interrupt request"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sofcfg_SPEC;
impl crate::sealed::RegSpec for Sofcfg_SPEC {
    type DataType = u16;
}

#[doc = "SOF Output Configuration Register"]
pub type Sofcfg = crate::RegValueT<Sofcfg_SPEC>;

impl Sofcfg {
    #[doc = "Interrupt Edge Processing Status Flag"]
    #[inline(always)]
    pub fn edgests(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Sofcfg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Sofcfg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt Output Sense Select"]
    #[inline(always)]
    pub fn intl(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        sofcfg::Intl,
        sofcfg::Intl,
        Sofcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            sofcfg::Intl,
            sofcfg::Intl,
            Sofcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PIPEBRDY Interrupt Status Clear Timing"]
    #[inline(always)]
    pub fn brdym(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        sofcfg::Brdym,
        sofcfg::Brdym,
        Sofcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            sofcfg::Brdym,
            sofcfg::Brdym,
            Sofcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transaction-Enabled Time Select"]
    #[inline(always)]
    pub fn trnensel(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        sofcfg::Trnensel,
        sofcfg::Trnensel,
        Sofcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            sofcfg::Trnensel,
            sofcfg::Trnensel,
            Sofcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Sofcfg {
    #[inline(always)]
    fn default() -> Sofcfg {
        <crate::RegValueT<Sofcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sofcfg {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Intl_SPEC;
    pub type Intl = crate::EnumBitfieldStruct<u8, Intl_SPEC>;
    impl Intl {
        #[doc = "Edge detection"]
        pub const _0: Self = Self::new(0);

        #[doc = "Level detection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brdym_SPEC;
    pub type Brdym = crate::EnumBitfieldStruct<u8, Brdym_SPEC>;
    impl Brdym {
        #[doc = "Clear BRDY flag through software"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear BRDY flag by the USBHS through a data read from the FIFO buffer or data write to the FIFO buffer"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trnensel_SPEC;
    pub type Trnensel = crate::EnumBitfieldStruct<u8, Trnensel_SPEC>;
    impl Trnensel {
        #[doc = "Not low-speed communication"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low-speed communication"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Physet_SPEC;
impl crate::sealed::RegSpec for Physet_SPEC {
    type DataType = u16;
}

#[doc = "PHY Setting Register"]
pub type Physet = crate::RegValueT<Physet_SPEC>;

impl Physet {
    #[doc = "Power-Down Control"]
    #[inline(always)]
    pub fn dirpd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        physet::Dirpd,
        physet::Dirpd,
        Physet_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            physet::Dirpd,
            physet::Dirpd,
            Physet_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PLL Reset Control"]
    #[inline(always)]
    pub fn pllreset(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        physet::Pllreset,
        physet::Pllreset,
        Physet_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            physet::Pllreset,
            physet::Pllreset,
            Physet_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Charging Downstream Port Enable"]
    #[inline(always)]
    pub fn cdpen(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        physet::Cdpen,
        physet::Cdpen,
        Physet_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            physet::Cdpen,
            physet::Cdpen,
            Physet_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Input System Clock Frequency"]
    #[inline(always)]
    pub fn clksel(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        physet::Clksel,
        physet::Clksel,
        Physet_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            physet::Clksel,
            physet::Clksel,
            Physet_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Terminating Resistance Adjustment Cycle"]
    #[inline(always)]
    pub fn repsel(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        physet::Repsel,
        physet::Repsel,
        Physet_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            physet::Repsel,
            physet::Repsel,
            Physet_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Forcibly Start Terminating Resistance Adjustment"]
    #[inline(always)]
    pub fn repstart(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        physet::Repstart,
        physet::Repstart,
        Physet_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            physet::Repstart,
            physet::Repstart,
            Physet_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CL-only mode"]
    #[inline(always)]
    pub fn hseb(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        physet::Hseb,
        physet::Hseb,
        Physet_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            physet::Hseb,
            physet::Hseb,
            Physet_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Physet {
    #[inline(always)]
    fn default() -> Physet {
        <crate::RegValueT<Physet_SPEC> as RegisterValue<_>>::new(51)
    }
}
pub mod physet {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirpd_SPEC;
    pub type Dirpd = crate::EnumBitfieldStruct<u8, Dirpd_SPEC>;
    impl Dirpd {
        #[doc = "Do not enter low power mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enter low power mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pllreset_SPEC;
    pub type Pllreset = crate::EnumBitfieldStruct<u8, Pllreset_SPEC>;
    impl Pllreset {
        #[doc = "Disable PLL reset control for UTMI_PHY"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable PLL reset control for UTMI_PHY"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdpen_SPEC;
    pub type Cdpen = crate::EnumBitfieldStruct<u8, Cdpen_SPEC>;
    impl Cdpen {
        #[doc = "Disable downstream port charging"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable downstream port charging"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clksel_SPEC;
    pub type Clksel = crate::EnumBitfieldStruct<u8, Clksel_SPEC>;
    impl Clksel {
        #[doc = "12 MHz"]
        pub const _00: Self = Self::new(0);

        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "20 MHz"]
        pub const _10: Self = Self::new(2);

        #[doc = "24 MHz"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Repsel_SPEC;
    pub type Repsel = crate::EnumBitfieldStruct<u8, Repsel_SPEC>;
    impl Repsel {
        #[doc = "No cycle is set"]
        pub const _00: Self = Self::new(0);

        #[doc = "Adjust terminating resistance at 16-second intervals"]
        pub const _01: Self = Self::new(1);

        #[doc = "Adjust terminating resistance at 64-second intervals"]
        pub const _10: Self = Self::new(2);

        #[doc = "Adjust terminating resistance at 128-second intervals"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Repstart_SPEC;
    pub type Repstart = crate::EnumBitfieldStruct<u8, Repstart_SPEC>;
    impl Repstart {
        #[doc = "Force terminating resistance adjustment to start"]
        pub const _0: Self = Self::new(0);

        #[doc = "Do not force terminating resistance adjustment to start"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hseb_SPEC;
    pub type Hseb = crate::EnumBitfieldStruct<u8, Hseb_SPEC>;
    impl Hseb {
        #[doc = "Disable CL-only mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable CL-only mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intsts0_SPEC;
impl crate::sealed::RegSpec for Intsts0_SPEC {
    type DataType = u16;
}

#[doc = "Interrupt Status Register 0"]
pub type Intsts0 = crate::RegValueT<Intsts0_SPEC>;

impl Intsts0 {
    #[doc = "Control Transfer Stage Flag"]
    #[inline(always)]
    pub fn ctsq(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        intsts0::Ctsq,
        intsts0::Ctsq,
        Intsts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            intsts0::Ctsq,
            intsts0::Ctsq,
            Intsts0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "USB Request Reception Flag"]
    #[inline(always)]
    pub fn valid(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        intsts0::Valid,
        intsts0::Valid,
        Intsts0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            intsts0::Valid,
            intsts0::Valid,
            Intsts0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device State"]
    #[inline(always)]
    pub fn dvsq(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        intsts0::Dvsq,
        intsts0::Dvsq,
        Intsts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            intsts0::Dvsq,
            intsts0::Dvsq,
            Intsts0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "VBUS Input Status Flag"]
    #[inline(always)]
    pub fn vbsts(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        intsts0::Vbsts,
        intsts0::Vbsts,
        Intsts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            intsts0::Vbsts,
            intsts0::Vbsts,
            Intsts0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "BRDY Interrupt Status Flag"]
    #[inline(always)]
    pub fn brdy(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        intsts0::Brdy,
        intsts0::Brdy,
        Intsts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            intsts0::Brdy,
            intsts0::Brdy,
            Intsts0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "NRDY Interrupt Status Flag"]
    #[inline(always)]
    pub fn nrdy(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        intsts0::Nrdy,
        intsts0::Nrdy,
        Intsts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            intsts0::Nrdy,
            intsts0::Nrdy,
            Intsts0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "BEMP Interrupt Status Flag"]
    #[inline(always)]
    pub fn bemp(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        intsts0::Bemp,
        intsts0::Bemp,
        Intsts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            intsts0::Bemp,
            intsts0::Bemp,
            Intsts0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Control Transfer Stage Transition Interrupt Status Flag"]
    #[inline(always)]
    pub fn ctrt(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        intsts0::Ctrt,
        intsts0::Ctrt,
        Intsts0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            intsts0::Ctrt,
            intsts0::Ctrt,
            Intsts0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Device State Transition Interrupt Status Flag"]
    #[inline(always)]
    pub fn dvst(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        intsts0::Dvst,
        intsts0::Dvst,
        Intsts0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            intsts0::Dvst,
            intsts0::Dvst,
            Intsts0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Frame Number Refresh Interrupt Status Flag"]
    #[inline(always)]
    pub fn sofr(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        intsts0::Sofr,
        intsts0::Sofr,
        Intsts0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            intsts0::Sofr,
            intsts0::Sofr,
            Intsts0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Resume Interrupt Status Flag"]
    #[inline(always)]
    pub fn resm(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        intsts0::Resm,
        intsts0::Resm,
        Intsts0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            intsts0::Resm,
            intsts0::Resm,
            Intsts0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VBUS Interrupt Status Flag"]
    #[inline(always)]
    pub fn vbint(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        intsts0::Vbint,
        intsts0::Vbint,
        Intsts0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            intsts0::Vbint,
            intsts0::Vbint,
            Intsts0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Intsts0 {
    #[inline(always)]
    fn default() -> Intsts0 {
        <crate::RegValueT<Intsts0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod intsts0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctsq_SPEC;
    pub type Ctsq = crate::EnumBitfieldStruct<u8, Ctsq_SPEC>;
    impl Ctsq {
        #[doc = "Idle or setup stage"]
        pub const _000: Self = Self::new(0);

        #[doc = "Control read data stage"]
        pub const _001: Self = Self::new(1);

        #[doc = "Control read status stage"]
        pub const _010: Self = Self::new(2);

        #[doc = "Control write data stage"]
        pub const _011: Self = Self::new(3);

        #[doc = "Control write status stage"]
        pub const _100: Self = Self::new(4);

        #[doc = "Control write (no data) status stage"]
        pub const _101: Self = Self::new(5);

        #[doc = "Control transfer sequence error"]
        pub const _110: Self = Self::new(6);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Valid_SPEC;
    pub type Valid = crate::EnumBitfieldStruct<u8, Valid_SPEC>;
    impl Valid {
        #[doc = "Setup packet not received"]
        pub const _0: Self = Self::new(0);

        #[doc = "Setup packet received"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvsq_SPEC;
    pub type Dvsq = crate::EnumBitfieldStruct<u8, Dvsq_SPEC>;
    impl Dvsq {
        #[doc = "Powered state"]
        pub const _000: Self = Self::new(0);

        #[doc = "Default state"]
        pub const _001: Self = Self::new(1);

        #[doc = "Address state"]
        pub const _010: Self = Self::new(2);

        #[doc = "Configured state"]
        pub const _011: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbsts_SPEC;
    pub type Vbsts = crate::EnumBitfieldStruct<u8, Vbsts_SPEC>;
    impl Vbsts {
        #[doc = "USBHS_VBUS pin is low"]
        pub const _0: Self = Self::new(0);

        #[doc = "USBHS_VBUS pin is high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brdy_SPEC;
    pub type Brdy = crate::EnumBitfieldStruct<u8, Brdy_SPEC>;
    impl Brdy {
        #[doc = "No BRDY interrupt occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "BRDY interrupt occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nrdy_SPEC;
    pub type Nrdy = crate::EnumBitfieldStruct<u8, Nrdy_SPEC>;
    impl Nrdy {
        #[doc = "No NRDY interrupt occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "NRDY interrupt occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bemp_SPEC;
    pub type Bemp = crate::EnumBitfieldStruct<u8, Bemp_SPEC>;
    impl Bemp {
        #[doc = "No BEMP interrupt occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "BEMP interrupt occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctrt_SPEC;
    pub type Ctrt = crate::EnumBitfieldStruct<u8, Ctrt_SPEC>;
    impl Ctrt {
        #[doc = "No control transfer stage transition interrupt occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Control transfer stage transition interrupt occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvst_SPEC;
    pub type Dvst = crate::EnumBitfieldStruct<u8, Dvst_SPEC>;
    impl Dvst {
        #[doc = "No device state transition interrupt occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Device state transition interrupt occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sofr_SPEC;
    pub type Sofr = crate::EnumBitfieldStruct<u8, Sofr_SPEC>;
    impl Sofr {
        #[doc = "No SOF interrupt occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "SOF interrupt occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Resm_SPEC;
    pub type Resm = crate::EnumBitfieldStruct<u8, Resm_SPEC>;
    impl Resm {
        #[doc = "No resume interrupt occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Resume interrupt occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbint_SPEC;
    pub type Vbint = crate::EnumBitfieldStruct<u8, Vbint_SPEC>;
    impl Vbint {
        #[doc = "No VBUS interrupt occurred on detecting a change in the USBHS_VBUS pin"]
        pub const _0: Self = Self::new(0);

        #[doc = "VBUS interrupt occurred on detecting a change in the USBHS_VBUS pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intsts1_SPEC;
impl crate::sealed::RegSpec for Intsts1_SPEC {
    type DataType = u16;
}

#[doc = "Interrupt Status Register 1"]
pub type Intsts1 = crate::RegValueT<Intsts1_SPEC>;

impl Intsts1 {
    #[doc = "PDDET Detection Interrupt Status Flag"]
    #[inline(always)]
    pub fn pddetint(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        intsts1::Pddetint,
        intsts1::Pddetint,
        Intsts1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            intsts1::Pddetint,
            intsts1::Pddetint,
            Intsts1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Setup Transaction Normal Response Interrupt Status Flag"]
    #[inline(always)]
    pub fn sack(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        intsts1::Sack,
        intsts1::Sack,
        Intsts1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            intsts1::Sack,
            intsts1::Sack,
            Intsts1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Setup Transaction Error Interrupt Status Flag"]
    #[inline(always)]
    pub fn sign(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        intsts1::Sign,
        intsts1::Sign,
        Intsts1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            intsts1::Sign,
            intsts1::Sign,
            Intsts1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "EOF Error Detection Interrupt Status Flag"]
    #[inline(always)]
    pub fn eoferr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        intsts1::Eoferr,
        intsts1::Eoferr,
        Intsts1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            intsts1::Eoferr,
            intsts1::Eoferr,
            Intsts1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LPM Transaction End Interrupt Status Flag"]
    #[inline(always)]
    pub fn lpmend(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        intsts1::Lpmend,
        intsts1::Lpmend,
        Intsts1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            intsts1::Lpmend,
            intsts1::Lpmend,
            Intsts1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "L1 Resume End Interrupt Status Flag"]
    #[inline(always)]
    pub fn l1rsmend(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        intsts1::L1Rsmend,
        intsts1::L1Rsmend,
        Intsts1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            intsts1::L1Rsmend,
            intsts1::L1Rsmend,
            Intsts1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "USB Connection Detection Interrupt Status Flag"]
    #[inline(always)]
    pub fn attch(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        intsts1::Attch,
        intsts1::Attch,
        Intsts1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            intsts1::Attch,
            intsts1::Attch,
            Intsts1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "USB Disconnection Detection Interrupt Status Flag"]
    #[inline(always)]
    pub fn dtch(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        intsts1::Dtch,
        intsts1::Dtch,
        Intsts1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            intsts1::Dtch,
            intsts1::Dtch,
            Intsts1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "USB Bus Change Interrupt Status Flag"]
    #[inline(always)]
    pub fn bchg(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        intsts1::Bchg,
        intsts1::Bchg,
        Intsts1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            intsts1::Bchg,
            intsts1::Bchg,
            Intsts1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "OVRCR Interrupt Status Flag"]
    #[inline(always)]
    pub fn ovrcr(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        intsts1::Ovrcr,
        intsts1::Ovrcr,
        Intsts1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            intsts1::Ovrcr,
            intsts1::Ovrcr,
            Intsts1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Intsts1 {
    #[inline(always)]
    fn default() -> Intsts1 {
        <crate::RegValueT<Intsts1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod intsts1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pddetint_SPEC;
    pub type Pddetint = crate::EnumBitfieldStruct<u8, Pddetint_SPEC>;
    impl Pddetint {
        #[doc = "No PDDET interrupt occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "PDDET interrupt occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sack_SPEC;
    pub type Sack = crate::EnumBitfieldStruct<u8, Sack_SPEC>;
    impl Sack {
        #[doc = "No SACK interrupt occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "SACK interrupt occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sign_SPEC;
    pub type Sign = crate::EnumBitfieldStruct<u8, Sign_SPEC>;
    impl Sign {
        #[doc = "No SIGN interrupt occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "SIGN interrupt occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eoferr_SPEC;
    pub type Eoferr = crate::EnumBitfieldStruct<u8, Eoferr_SPEC>;
    impl Eoferr {
        #[doc = "No EOFERR interrupt occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "EOFERR interrupt occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lpmend_SPEC;
    pub type Lpmend = crate::EnumBitfieldStruct<u8, Lpmend_SPEC>;
    impl Lpmend {
        #[doc = "No LPMEND interrupt occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "LPMEND interrupt occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L1Rsmend_SPEC;
    pub type L1Rsmend = crate::EnumBitfieldStruct<u8, L1Rsmend_SPEC>;
    impl L1Rsmend {
        #[doc = "No L1RSMEND interrupt occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "L1RSMEND interrupt occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Attch_SPEC;
    pub type Attch = crate::EnumBitfieldStruct<u8, Attch_SPEC>;
    impl Attch {
        #[doc = "No ATTCH interrupt occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "ATTCH interrupt occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtch_SPEC;
    pub type Dtch = crate::EnumBitfieldStruct<u8, Dtch_SPEC>;
    impl Dtch {
        #[doc = "No DTCH interrupt occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "DTCH interrupt occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bchg_SPEC;
    pub type Bchg = crate::EnumBitfieldStruct<u8, Bchg_SPEC>;
    impl Bchg {
        #[doc = "No BCHG interrupt occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "BCHG interrupt occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovrcr_SPEC;
    pub type Ovrcr = crate::EnumBitfieldStruct<u8, Ovrcr_SPEC>;
    impl Ovrcr {
        #[doc = "No OVRCR interrupt occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "OVRCR interrupt occurred"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Brdysts_SPEC;
impl crate::sealed::RegSpec for Brdysts_SPEC {
    type DataType = u16;
}

#[doc = "BRDY Interrupt Status Register"]
pub type Brdysts = crate::RegValueT<Brdysts_SPEC>;

impl Brdysts {
    #[doc = "BRDY Interrupt Status Flag for Pipe\\[9:0\\]"]
    #[inline(always)]
    pub fn pipebrdy(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        brdysts::Pipebrdy,
        brdysts::Pipebrdy,
        Brdysts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            brdysts::Pipebrdy,
            brdysts::Pipebrdy,
            Brdysts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Brdysts {
    #[inline(always)]
    fn default() -> Brdysts {
        <crate::RegValueT<Brdysts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod brdysts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipebrdy_SPEC;
    pub type Pipebrdy = crate::EnumBitfieldStruct<u8, Pipebrdy_SPEC>;
    impl Pipebrdy {
        #[doc = "No BRDY interrupt occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "BRDY interrupt occurred"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nrdysts_SPEC;
impl crate::sealed::RegSpec for Nrdysts_SPEC {
    type DataType = u16;
}

#[doc = "NRDY Interrupt Status Register"]
pub type Nrdysts = crate::RegValueT<Nrdysts_SPEC>;

impl Nrdysts {
    #[doc = "NRDY Interrupt Status Flag for Pipe\\[9:0\\]"]
    #[inline(always)]
    pub fn pipenrdy(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        nrdysts::Pipenrdy,
        nrdysts::Pipenrdy,
        Nrdysts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            nrdysts::Pipenrdy,
            nrdysts::Pipenrdy,
            Nrdysts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Nrdysts {
    #[inline(always)]
    fn default() -> Nrdysts {
        <crate::RegValueT<Nrdysts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod nrdysts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipenrdy_SPEC;
    pub type Pipenrdy = crate::EnumBitfieldStruct<u8, Pipenrdy_SPEC>;
    impl Pipenrdy {
        #[doc = "No NRDY interrupt occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "NRDY interrupt occurred."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bempsts_SPEC;
impl crate::sealed::RegSpec for Bempsts_SPEC {
    type DataType = u16;
}

#[doc = "BEMP Interrupt Status Register"]
pub type Bempsts = crate::RegValueT<Bempsts_SPEC>;

impl Bempsts {
    #[doc = "BEMP Interrupt Status Flag for Pipe\\[9:0\\]"]
    #[inline(always)]
    pub fn pipebemp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        bempsts::Pipebemp,
        bempsts::Pipebemp,
        Bempsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            bempsts::Pipebemp,
            bempsts::Pipebemp,
            Bempsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bempsts {
    #[inline(always)]
    fn default() -> Bempsts {
        <crate::RegValueT<Bempsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bempsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipebemp_SPEC;
    pub type Pipebemp = crate::EnumBitfieldStruct<u8, Pipebemp_SPEC>;
    impl Pipebemp {
        #[doc = "No BEMP interrupt occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "BEMP interrupt occurred."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frmnum_SPEC;
impl crate::sealed::RegSpec for Frmnum_SPEC {
    type DataType = u16;
}

#[doc = "Frame Number Register"]
pub type Frmnum = crate::RegValueT<Frmnum_SPEC>;

impl Frmnum {
    #[doc = "Frame Number Flag"]
    #[inline(always)]
    pub fn frnm(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, Frmnum_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,Frmnum_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "CRC Error Detection Status Flag"]
    #[inline(always)]
    pub fn crce(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        frmnum::Crce,
        frmnum::Crce,
        Frmnum_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            frmnum::Crce,
            frmnum::Crce,
            Frmnum_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Overrun/Underrun Detection Status Flag"]
    #[inline(always)]
    pub fn ovrn(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        frmnum::Ovrn,
        frmnum::Ovrn,
        Frmnum_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            frmnum::Ovrn,
            frmnum::Ovrn,
            Frmnum_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Frmnum {
    #[inline(always)]
    fn default() -> Frmnum {
        <crate::RegValueT<Frmnum_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod frmnum {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crce_SPEC;
    pub type Crce = crate::EnumBitfieldStruct<u8, Crce_SPEC>;
    impl Crce {
        #[doc = "No error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovrn_SPEC;
    pub type Ovrn = crate::EnumBitfieldStruct<u8, Ovrn_SPEC>;
    impl Ovrn {
        #[doc = "No error occurred"]
        pub const _0: Self = Self::new(0);

        #[doc = "Error occurred."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ufrmnum_SPEC;
impl crate::sealed::RegSpec for Ufrmnum_SPEC {
    type DataType = u16;
}

#[doc = "µFrame Number Register"]
pub type Ufrmnum = crate::RegValueT<Ufrmnum_SPEC>;

impl Ufrmnum {
    #[doc = "Microframe number"]
    #[inline(always)]
    pub fn ufrnm(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Ufrmnum_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Ufrmnum_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Device State Change"]
    #[inline(always)]
    pub fn dvchg(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        ufrmnum::Dvchg,
        ufrmnum::Dvchg,
        Ufrmnum_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            ufrmnum::Dvchg,
            ufrmnum::Dvchg,
            Ufrmnum_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ufrmnum {
    #[inline(always)]
    fn default() -> Ufrmnum {
        <crate::RegValueT<Ufrmnum_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ufrmnum {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvchg_SPEC;
    pub type Dvchg = crate::EnumBitfieldStruct<u8, Dvchg_SPEC>;
    impl Dvchg {
        #[doc = "Disable writes to the USBADDR.STSRECOV0\\[2:0\\] and USBADDR.USBADDR\\[6:0\\] bits"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable writes to the USBADDR.STSRECOV0\\[2:0\\] and USBADDR.USBADDR\\[6:0\\] bits"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbaddr_SPEC;
impl crate::sealed::RegSpec for Usbaddr_SPEC {
    type DataType = u16;
}

#[doc = "USB Address Register"]
pub type Usbaddr = crate::RegValueT<Usbaddr_SPEC>;

impl Usbaddr {
    #[doc = "USB Address Flag"]
    #[inline(always)]
    pub fn usbaddr(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Usbaddr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Usbaddr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Status Recovery"]
    #[inline(always)]
    pub fn stsrecov0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        usbaddr::Stsrecov0,
        usbaddr::Stsrecov0,
        Usbaddr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            usbaddr::Stsrecov0,
            usbaddr::Stsrecov0,
            Usbaddr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Usbaddr {
    #[inline(always)]
    fn default() -> Usbaddr {
        <crate::RegValueT<Usbaddr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod usbaddr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stsrecov0_SPEC;
    pub type Stsrecov0 = crate::EnumBitfieldStruct<u8, Stsrecov0_SPEC>;
    impl Stsrecov0 {
        #[doc = "Reserved"]
        pub const _000: Self = Self::new(0);

        #[doc = "\\[D\\] Return to the full-speed connection and Default state"]
        pub const _001: Self = Self::new(1);

        #[doc = "\\[D\\] Return to the full-speed connection and Address state \\[H\\] Return to the low-speed state (bits DVSTCTR0.RHST\\[2:0\\] = 001b)"]
        pub const _010: Self = Self::new(2);

        #[doc = "\\[D\\] Return to the full-speed connection and Configured state"]
        pub const _011: Self = Self::new(3);

        #[doc = "\\[D\\] Return to the suspend connection and Suspend state \\[H\\] Return to the full-speed state (bits DVSTCTR0.RHST\\[2:0\\] = 010b)"]
        pub const _100: Self = Self::new(4);

        #[doc = "\\[D\\] Return to the high-speed connection and Default state"]
        pub const _101: Self = Self::new(5);

        #[doc = "\\[D\\] Return to the high-speed connection and Address state \\[H\\] Return to the high-speed state (bits DVSTCTR0.RHST\\[2:0\\] = 011b)"]
        pub const _110: Self = Self::new(6);

        #[doc = "\\[D\\] Return to the high-speed connection and Configured state"]
        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbreq_SPEC;
impl crate::sealed::RegSpec for Usbreq_SPEC {
    type DataType = u16;
}

#[doc = "USB Request Type Register"]
pub type Usbreq = crate::RegValueT<Usbreq_SPEC>;

impl Usbreq {
    #[doc = "USB request bmRequestType value"]
    #[inline(always)]
    pub fn bmrequesttype(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Usbreq_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Usbreq_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "USB request bRequest value"]
    #[inline(always)]
    pub fn brequest(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Usbreq_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Usbreq_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Usbreq {
    #[inline(always)]
    fn default() -> Usbreq {
        <crate::RegValueT<Usbreq_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbval_SPEC;
impl crate::sealed::RegSpec for Usbval_SPEC {
    type DataType = u16;
}

#[doc = "USB Request Value Register"]
pub type Usbval = crate::RegValueT<Usbval_SPEC>;

impl Usbval {
    #[doc = "USB request wValue value"]
    #[inline(always)]
    pub fn wvalue(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Usbval_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Usbval_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Usbval {
    #[inline(always)]
    fn default() -> Usbval {
        <crate::RegValueT<Usbval_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbindx_SPEC;
impl crate::sealed::RegSpec for Usbindx_SPEC {
    type DataType = u16;
}

#[doc = "USB Request Index Register"]
pub type Usbindx = crate::RegValueT<Usbindx_SPEC>;

impl Usbindx {
    #[doc = "USB request wIndex value"]
    #[inline(always)]
    pub fn windex(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Usbindx_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Usbindx_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Usbindx {
    #[inline(always)]
    fn default() -> Usbindx {
        <crate::RegValueT<Usbindx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbleng_SPEC;
impl crate::sealed::RegSpec for Usbleng_SPEC {
    type DataType = u16;
}

#[doc = "USB Request Length Register"]
pub type Usbleng = crate::RegValueT<Usbleng_SPEC>;

impl Usbleng {
    #[doc = "USB request wLength value"]
    #[inline(always)]
    pub fn wlentuh(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Usbleng_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Usbleng_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Usbleng {
    #[inline(always)]
    fn default() -> Usbleng {
        <crate::RegValueT<Usbleng_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcpcfg_SPEC;
impl crate::sealed::RegSpec for Dcpcfg_SPEC {
    type DataType = u16;
}

#[doc = "DCP Configuration Register"]
pub type Dcpcfg = crate::RegValueT<Dcpcfg_SPEC>;

impl Dcpcfg {
    #[doc = "Transfer Direction"]
    #[inline(always)]
    pub fn dir(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dcpcfg::Dir,
        dcpcfg::Dir,
        Dcpcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dcpcfg::Dir,
            dcpcfg::Dir,
            Dcpcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pipe Blocking on End of Transfer"]
    #[inline(always)]
    pub fn shtnak(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dcpcfg::Shtnak,
        dcpcfg::Shtnak,
        Dcpcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dcpcfg::Shtnak,
            dcpcfg::Shtnak,
            Dcpcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Continuous Transfer Mode"]
    #[inline(always)]
    pub fn cntmd(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        dcpcfg::Cntmd,
        dcpcfg::Cntmd,
        Dcpcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            dcpcfg::Cntmd,
            dcpcfg::Cntmd,
            Dcpcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dcpcfg {
    #[inline(always)]
    fn default() -> Dcpcfg {
        <crate::RegValueT<Dcpcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dcpcfg {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dir_SPEC;
    pub type Dir = crate::EnumBitfieldStruct<u8, Dir_SPEC>;
    impl Dir {
        #[doc = "Data receiving direction"]
        pub const _0: Self = Self::new(0);

        #[doc = "Data transmitting direction"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Shtnak_SPEC;
    pub type Shtnak = crate::EnumBitfieldStruct<u8, Shtnak_SPEC>;
    impl Shtnak {
        #[doc = "Keep pipe open after transfer ends"]
        pub const _0: Self = Self::new(0);

        #[doc = "Disable pipe after transfer ends"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cntmd_SPEC;
    pub type Cntmd = crate::EnumBitfieldStruct<u8, Cntmd_SPEC>;
    impl Cntmd {
        #[doc = "Non-continuous transfer mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Continuous transfer mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcpmaxp_SPEC;
impl crate::sealed::RegSpec for Dcpmaxp_SPEC {
    type DataType = u16;
}

#[doc = "DCP Maximum Packet Size Register"]
pub type Dcpmaxp = crate::RegValueT<Dcpmaxp_SPEC>;

impl Dcpmaxp {
    #[doc = "Maximum Packet Size"]
    #[inline(always)]
    pub fn mxps(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Dcpmaxp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Dcpmaxp_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Device Select"]
    #[inline(always)]
    pub fn devsel(
        self,
    ) -> crate::common::RegisterField<
        12,
        0xf,
        1,
        0,
        dcpmaxp::Devsel,
        dcpmaxp::Devsel,
        Dcpmaxp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0xf,
            1,
            0,
            dcpmaxp::Devsel,
            dcpmaxp::Devsel,
            Dcpmaxp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dcpmaxp {
    #[inline(always)]
    fn default() -> Dcpmaxp {
        <crate::RegValueT<Dcpmaxp_SPEC> as RegisterValue<_>>::new(64)
    }
}
pub mod dcpmaxp {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Devsel_SPEC;
    pub type Devsel = crate::EnumBitfieldStruct<u8, Devsel_SPEC>;
    impl Devsel {
        #[doc = "Address 0x0"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "Address 0x1"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "Address 0x2"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "Address 0x3"]
        pub const _0_X_3: Self = Self::new(3);

        #[doc = "Address 0x4"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "Address 0x5"]
        pub const _0_X_5: Self = Self::new(5);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcpctr_SPEC;
impl crate::sealed::RegSpec for Dcpctr_SPEC {
    type DataType = u16;
}

#[doc = "DCP Control Register"]
pub type Dcpctr = crate::RegValueT<Dcpctr_SPEC>;

impl Dcpctr {
    #[doc = "Response PID"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        dcpctr::Pid,
        dcpctr::Pid,
        Dcpctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            dcpctr::Pid,
            dcpctr::Pid,
            Dcpctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Control Transfer End Enable"]
    #[inline(always)]
    pub fn ccpl(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dcpctr::Ccpl,
        dcpctr::Ccpl,
        Dcpctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dcpctr::Ccpl,
            dcpctr::Ccpl,
            Dcpctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "PING Token Issue Enable"]
    #[inline(always)]
    pub fn pinge(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dcpctr::Pinge,
        dcpctr::Pinge,
        Dcpctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dcpctr::Pinge,
            dcpctr::Pinge,
            Dcpctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pipe Busy Flag"]
    #[inline(always)]
    pub fn pbusy(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dcpctr::Pbusy,
        dcpctr::Pbusy,
        Dcpctr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dcpctr::Pbusy,
            dcpctr::Pbusy,
            Dcpctr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Sequence Toggle Bit Monitor Flag"]
    #[inline(always)]
    pub fn sqmon(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        dcpctr::Sqmon,
        dcpctr::Sqmon,
        Dcpctr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            dcpctr::Sqmon,
            dcpctr::Sqmon,
            Dcpctr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Sequence Toggle Bit Set"]
    #[inline(always)]
    pub fn sqset(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dcpctr::Sqset,
        dcpctr::Sqset,
        Dcpctr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dcpctr::Sqset,
            dcpctr::Sqset,
            Dcpctr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Sequence Toggle Bit Clear"]
    #[inline(always)]
    pub fn sqclr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        dcpctr::Sqclr,
        dcpctr::Sqclr,
        Dcpctr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            dcpctr::Sqclr,
            dcpctr::Sqclr,
            Dcpctr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "SUREQ Bit Clear"]
    #[inline(always)]
    pub fn sureqclr(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        dcpctr::Sureqclr,
        dcpctr::Sureqclr,
        Dcpctr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            dcpctr::Sureqclr,
            dcpctr::Sureqclr,
            Dcpctr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "CSSTS Status Flag"]
    #[inline(always)]
    pub fn cssts(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        dcpctr::Cssts,
        dcpctr::Cssts,
        Dcpctr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            dcpctr::Cssts,
            dcpctr::Cssts,
            Dcpctr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CSSTS Status Flag Clear"]
    #[inline(always)]
    pub fn csclr(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        dcpctr::Csclr,
        dcpctr::Csclr,
        Dcpctr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            dcpctr::Csclr,
            dcpctr::Csclr,
            Dcpctr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "SETUP Token Transmission"]
    #[inline(always)]
    pub fn sureq(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        dcpctr::Sureq,
        dcpctr::Sureq,
        Dcpctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            dcpctr::Sureq,
            dcpctr::Sureq,
            Dcpctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Buffer Status Flag"]
    #[inline(always)]
    pub fn bsts(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        dcpctr::Bsts,
        dcpctr::Bsts,
        Dcpctr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            dcpctr::Bsts,
            dcpctr::Bsts,
            Dcpctr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dcpctr {
    #[inline(always)]
    fn default() -> Dcpctr {
        <crate::RegValueT<Dcpctr_SPEC> as RegisterValue<_>>::new(64)
    }
}
pub mod dcpctr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pid_SPEC;
    pub type Pid = crate::EnumBitfieldStruct<u8, Pid_SPEC>;
    impl Pid {
        #[doc = "NAK response"]
        pub const _00: Self = Self::new(0);

        #[doc = "BUF response (depends on buffer state)"]
        pub const _01: Self = Self::new(1);

        #[doc = "STALL response"]
        pub const _10: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccpl_SPEC;
    pub type Ccpl = crate::EnumBitfieldStruct<u8, Ccpl_SPEC>;
    impl Ccpl {
        #[doc = "Disable control transfer completion"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable control transfer completion"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pinge_SPEC;
    pub type Pinge = crate::EnumBitfieldStruct<u8, Pinge_SPEC>;
    impl Pinge {
        #[doc = "Disable PING token"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable normal PING operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pbusy_SPEC;
    pub type Pbusy = crate::EnumBitfieldStruct<u8, Pbusy_SPEC>;
    impl Pbusy {
        #[doc = "DCP not used for the USB bus"]
        pub const _0: Self = Self::new(0);

        #[doc = "DCP in use for the USB bus"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sqmon_SPEC;
    pub type Sqmon = crate::EnumBitfieldStruct<u8, Sqmon_SPEC>;
    impl Sqmon {
        #[doc = "DATA0"]
        pub const _0: Self = Self::new(0);

        #[doc = "DATA1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sqset_SPEC;
    pub type Sqset = crate::EnumBitfieldStruct<u8, Sqset_SPEC>;
    impl Sqset {
        #[doc = "Invalid (writing 0 has no effect)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Set the expected value for the next transaction to DATA1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sqclr_SPEC;
    pub type Sqclr = crate::EnumBitfieldStruct<u8, Sqclr_SPEC>;
    impl Sqclr {
        #[doc = "Invalid (writing 0 has no effect)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the expected value for the next transaction to DATA0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sureqclr_SPEC;
    pub type Sureqclr = crate::EnumBitfieldStruct<u8, Sureqclr_SPEC>;
    impl Sureqclr {
        #[doc = "Invalid (writing 0 has no effect)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear SUREQ to 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cssts_SPEC;
    pub type Cssts = crate::EnumBitfieldStruct<u8, Cssts_SPEC>;
    impl Cssts {
        #[doc = "Start-split (SSPLIT) transaction, or processing for devices that are not using split transactions, in progress"]
        pub const _0: Self = Self::new(0);

        #[doc = "Complete-split (CSPLIT) transaction in progress"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csclr_SPEC;
    pub type Csclr = crate::EnumBitfieldStruct<u8, Csclr_SPEC>;
    impl Csclr {
        #[doc = "(writing 0 has no effect)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear CSSTS to 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sureq_SPEC;
    pub type Sureq = crate::EnumBitfieldStruct<u8, Sureq_SPEC>;
    impl Sureq {
        #[doc = "Invalid (writing 0 has no effect)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmit setup packet"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsts_SPEC;
    pub type Bsts = crate::EnumBitfieldStruct<u8, Bsts_SPEC>;
    impl Bsts {
        #[doc = "Buffer access disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Buffer access enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pipesel_SPEC;
impl crate::sealed::RegSpec for Pipesel_SPEC {
    type DataType = u16;
}

#[doc = "Pipe Window Select Register"]
pub type Pipesel = crate::RegValueT<Pipesel_SPEC>;

impl Pipesel {
    #[doc = "Pipe Window Select"]
    #[inline(always)]
    pub fn pipesel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        pipesel::Pipesel,
        pipesel::Pipesel,
        Pipesel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            pipesel::Pipesel,
            pipesel::Pipesel,
            Pipesel_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pipesel {
    #[inline(always)]
    fn default() -> Pipesel {
        <crate::RegValueT<Pipesel_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pipesel {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipesel_SPEC;
    pub type Pipesel = crate::EnumBitfieldStruct<u8, Pipesel_SPEC>;
    impl Pipesel {
        #[doc = "No pipe selected"]
        pub const _0_X_0: Self = Self::new(0);

        #[doc = "Pipe 1"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "Pipe 2"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "Pipe 3"]
        pub const _0_X_3: Self = Self::new(3);

        #[doc = "Pipe 4"]
        pub const _0_X_4: Self = Self::new(4);

        #[doc = "Pipe 5"]
        pub const _0_X_5: Self = Self::new(5);

        #[doc = "Pipe 6"]
        pub const _0_X_6: Self = Self::new(6);

        #[doc = "Pipe 7"]
        pub const _0_X_7: Self = Self::new(7);

        #[doc = "Pipe 8"]
        pub const _0_X_8: Self = Self::new(8);

        #[doc = "Pipe 9"]
        pub const _0_X_9: Self = Self::new(9);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pipecfg_SPEC;
impl crate::sealed::RegSpec for Pipecfg_SPEC {
    type DataType = u16;
}

#[doc = "Pipe Configuration Register"]
pub type Pipecfg = crate::RegValueT<Pipecfg_SPEC>;

impl Pipecfg {
    #[doc = "Endpoint Number"]
    #[inline(always)]
    pub fn epnum(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Pipecfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Pipecfg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transfer Direction"]
    #[inline(always)]
    pub fn dir(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        pipecfg::Dir,
        pipecfg::Dir,
        Pipecfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pipecfg::Dir,
            pipecfg::Dir,
            Pipecfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pipe Disabled at End of Transfer"]
    #[inline(always)]
    pub fn shtnak(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pipecfg::Shtnak,
        pipecfg::Shtnak,
        Pipecfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pipecfg::Shtnak,
            pipecfg::Shtnak,
            Pipecfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Continuous Transfer Mode"]
    #[inline(always)]
    pub fn cntmd(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pipecfg::Cntmd,
        pipecfg::Cntmd,
        Pipecfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pipecfg::Cntmd,
            pipecfg::Cntmd,
            Pipecfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Double Buffer Mode"]
    #[inline(always)]
    pub fn dblb(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        pipecfg::Dblb,
        pipecfg::Dblb,
        Pipecfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            pipecfg::Dblb,
            pipecfg::Dblb,
            Pipecfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BRDY Interrupt Operation Specification"]
    #[inline(always)]
    pub fn bfre(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        pipecfg::Bfre,
        pipecfg::Bfre,
        Pipecfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            pipecfg::Bfre,
            pipecfg::Bfre,
            Pipecfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transfer Type"]
    #[inline(always)]
    pub fn r#type(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        pipecfg::Type,
        pipecfg::Type,
        Pipecfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            pipecfg::Type,
            pipecfg::Type,
            Pipecfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pipecfg {
    #[inline(always)]
    fn default() -> Pipecfg {
        <crate::RegValueT<Pipecfg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pipecfg {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dir_SPEC;
    pub type Dir = crate::EnumBitfieldStruct<u8, Dir_SPEC>;
    impl Dir {
        #[doc = "Receiving direction"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmitting direction"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Shtnak_SPEC;
    pub type Shtnak = crate::EnumBitfieldStruct<u8, Shtnak_SPEC>;
    impl Shtnak {
        #[doc = "Continue pipe operation after transfer ends"]
        pub const _0: Self = Self::new(0);

        #[doc = "Disable pipe after transfer ends"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cntmd_SPEC;
    pub type Cntmd = crate::EnumBitfieldStruct<u8, Cntmd_SPEC>;
    impl Cntmd {
        #[doc = "Discontinuous transfer mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Continuous transfer mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dblb_SPEC;
    pub type Dblb = crate::EnumBitfieldStruct<u8, Dblb_SPEC>;
    impl Dblb {
        #[doc = "Single buffer"]
        pub const _0: Self = Self::new(0);

        #[doc = "Double buffer"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfre_SPEC;
    pub type Bfre = crate::EnumBitfieldStruct<u8, Bfre_SPEC>;
    impl Bfre {
        #[doc = "Generate BRDY interrupt on transmitting or receiving data"]
        pub const _0: Self = Self::new(0);

        #[doc = "Generate BRDY interrupt on completion of reading data"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Type_SPEC;
    pub type Type = crate::EnumBitfieldStruct<u8, Type_SPEC>;
    impl Type {
        #[doc = "Pipe not used"]
        pub const _00: Self = Self::new(0);

        #[doc = "(Pipe 1 to 5) Bulk transfer (Pipe 6 to 9) Setting prohibited"]
        pub const _01: Self = Self::new(1);

        #[doc = "(Pipe 1 to 5) Setting prohibited (Pipe 6 to 9) Interrupt transfer"]
        pub const _10: Self = Self::new(2);

        #[doc = "(Pipe 1 to 2) Isochronous transfer (Pipe 3 to 9) Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pipebuf_SPEC;
impl crate::sealed::RegSpec for Pipebuf_SPEC {
    type DataType = u16;
}

#[doc = "Pipe Buffer Register"]
pub type Pipebuf = crate::RegValueT<Pipebuf_SPEC>;

impl Pipebuf {
    #[doc = "Buffer Number"]
    #[inline(always)]
    pub fn bufnmb(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Pipebuf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Pipebuf_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Buffer Size"]
    #[inline(always)]
    pub fn bufsize(
        self,
    ) -> crate::common::RegisterField<10, 0x1f, 1, 0, u8, u8, Pipebuf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x1f,1,0,u8,u8,Pipebuf_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pipebuf {
    #[inline(always)]
    fn default() -> Pipebuf {
        <crate::RegValueT<Pipebuf_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pipemaxp_SPEC;
impl crate::sealed::RegSpec for Pipemaxp_SPEC {
    type DataType = u16;
}

#[doc = "Pipe Maximum Packet Size Register"]
pub type Pipemaxp = crate::RegValueT<Pipemaxp_SPEC>;

impl Pipemaxp {
    #[doc = "Maximum Packet Size"]
    #[inline(always)]
    pub fn mxps(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, Pipemaxp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,Pipemaxp_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Device Select"]
    #[inline(always)]
    pub fn devsel(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Pipemaxp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,Pipemaxp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pipemaxp {
    #[inline(always)]
    fn default() -> Pipemaxp {
        <crate::RegValueT<Pipemaxp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pipeperi_SPEC;
impl crate::sealed::RegSpec for Pipeperi_SPEC {
    type DataType = u16;
}

#[doc = "Pipe Cycle Control Register"]
pub type Pipeperi = crate::RegValueT<Pipeperi_SPEC>;

impl Pipeperi {
    #[doc = "Interval Error Detection Interval"]
    #[inline(always)]
    pub fn iitv(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Pipeperi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Pipeperi_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Isochronous IN Buffer Flush"]
    #[inline(always)]
    pub fn ifis(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        pipeperi::Ifis,
        pipeperi::Ifis,
        Pipeperi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            pipeperi::Ifis,
            pipeperi::Ifis,
            Pipeperi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pipeperi {
    #[inline(always)]
    fn default() -> Pipeperi {
        <crate::RegValueT<Pipeperi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pipeperi {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ifis_SPEC;
    pub type Ifis = crate::EnumBitfieldStruct<u8, Ifis_SPEC>;
    impl Ifis {
        #[doc = "Do not flush buffer"]
        pub const _0: Self = Self::new(0);

        #[doc = "Flush buffer"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pipectr_SPEC;
impl crate::sealed::RegSpec for Pipectr_SPEC {
    type DataType = u16;
}

#[doc = "Pipe %s Control Register"]
pub type Pipectr = crate::RegValueT<Pipectr_SPEC>;

impl Pipectr {
    #[doc = "Response PID"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        pipectr::Pid,
        pipectr::Pid,
        Pipectr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            pipectr::Pid,
            pipectr::Pid,
            Pipectr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Pipe Busy Flag"]
    #[inline(always)]
    pub fn pbusy(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        pipectr::Pbusy,
        pipectr::Pbusy,
        Pipectr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            pipectr::Pbusy,
            pipectr::Pbusy,
            Pipectr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Sequence Toggle Bit Monitor Flag"]
    #[inline(always)]
    pub fn sqmon(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pipectr::Sqmon,
        pipectr::Sqmon,
        Pipectr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pipectr::Sqmon,
            pipectr::Sqmon,
            Pipectr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Sequence Toggle Bit Set"]
    #[inline(always)]
    pub fn sqset(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pipectr::Sqset,
        pipectr::Sqset,
        Pipectr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pipectr::Sqset,
            pipectr::Sqset,
            Pipectr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sequence Toggle Bit Clear"]
    #[inline(always)]
    pub fn sqclr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pipectr::Sqclr,
        pipectr::Sqclr,
        Pipectr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pipectr::Sqclr,
            pipectr::Sqclr,
            Pipectr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Auto Buffer Clear Mode"]
    #[inline(always)]
    pub fn aclrm(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        pipectr::Aclrm,
        pipectr::Aclrm,
        Pipectr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            pipectr::Aclrm,
            pipectr::Aclrm,
            Pipectr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Auto Response Mode"]
    #[inline(always)]
    pub fn atrepm(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        pipectr::Atrepm,
        pipectr::Atrepm,
        Pipectr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            pipectr::Atrepm,
            pipectr::Atrepm,
            Pipectr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CSSTS Status Flag"]
    #[inline(always)]
    pub fn cssts(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        pipectr::Cssts,
        pipectr::Cssts,
        Pipectr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            pipectr::Cssts,
            pipectr::Cssts,
            Pipectr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "CSPLIT Status Clear"]
    #[inline(always)]
    pub fn csclr(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        pipectr::Csclr,
        pipectr::Csclr,
        Pipectr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            pipectr::Csclr,
            pipectr::Csclr,
            Pipectr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Transmit Buffer Monitor Flag"]
    #[inline(always)]
    pub fn inbufm(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        pipectr::Inbufm,
        pipectr::Inbufm,
        Pipectr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pipectr::Inbufm,
            pipectr::Inbufm,
            Pipectr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Buffer Status Flag"]
    #[inline(always)]
    pub fn bsts(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        pipectr::Bsts,
        pipectr::Bsts,
        Pipectr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pipectr::Bsts,
            pipectr::Bsts,
            Pipectr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pipectr {
    #[inline(always)]
    fn default() -> Pipectr {
        <crate::RegValueT<Pipectr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pipectr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pid_SPEC;
    pub type Pid = crate::EnumBitfieldStruct<u8, Pid_SPEC>;
    impl Pid {
        #[doc = "NAK response"]
        pub const _00: Self = Self::new(0);

        #[doc = "BUF response (depends on buffer state)"]
        pub const _01: Self = Self::new(1);

        #[doc = "STALL response"]
        pub const _10: Self = Self::new(2);

        #[doc = "STALL response"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pbusy_SPEC;
    pub type Pbusy = crate::EnumBitfieldStruct<u8, Pbusy_SPEC>;
    impl Pbusy {
        #[doc = "Pipe n not in use for the transaction"]
        pub const _0: Self = Self::new(0);

        #[doc = "Pipe n in use for the transaction"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sqmon_SPEC;
    pub type Sqmon = crate::EnumBitfieldStruct<u8, Sqmon_SPEC>;
    impl Sqmon {
        #[doc = "DATA0"]
        pub const _0: Self = Self::new(0);

        #[doc = "DATA1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sqset_SPEC;
    pub type Sqset = crate::EnumBitfieldStruct<u8, Sqset_SPEC>;
    impl Sqset {
        #[doc = "Invalid (writing 0 has no effect)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Set the expected value for the next transaction to DATA1. This bit is read as 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sqclr_SPEC;
    pub type Sqclr = crate::EnumBitfieldStruct<u8, Sqclr_SPEC>;
    impl Sqclr {
        #[doc = "Invalid (writing 0 has no effect)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear the expected value for the next transaction to DATA0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aclrm_SPEC;
    pub type Aclrm = crate::EnumBitfieldStruct<u8, Aclrm_SPEC>;
    impl Aclrm {
        #[doc = "Disable"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable (initialize all buffers)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Atrepm_SPEC;
    pub type Atrepm = crate::EnumBitfieldStruct<u8, Atrepm_SPEC>;
    impl Atrepm {
        #[doc = "Disable auto response mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable auto response mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cssts_SPEC;
    pub type Cssts = crate::EnumBitfieldStruct<u8, Cssts_SPEC>;
    impl Cssts {
        #[doc = "Start-split (SSPLIT) transaction, or processing for devices that are not using split transactions, in progress."]
        pub const _0: Self = Self::new(0);

        #[doc = "Complete-split (CSPLIT) transaction in progress."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csclr_SPEC;
    pub type Csclr = crate::EnumBitfieldStruct<u8, Csclr_SPEC>;
    impl Csclr {
        #[doc = "Invalid (writing 0 has no effect)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear CSSTS to 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inbufm_SPEC;
    pub type Inbufm = crate::EnumBitfieldStruct<u8, Inbufm_SPEC>;
    impl Inbufm {
        #[doc = "No data to be transmitted is in the FIFO buffer"]
        pub const _0: Self = Self::new(0);

        #[doc = "Data to be transmitted is in the FIFO buffer"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsts_SPEC;
    pub type Bsts = crate::EnumBitfieldStruct<u8, Bsts_SPEC>;
    impl Bsts {
        #[doc = "Buffer access disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Buffer access enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pipetre_SPEC;
impl crate::sealed::RegSpec for Pipetre_SPEC {
    type DataType = u16;
}

#[doc = "Pipe %s Transaction Counter Enable Register"]
pub type Pipetre = crate::RegValueT<Pipetre_SPEC>;

impl Pipetre {
    #[doc = "Transaction Counter Clear"]
    #[inline(always)]
    pub fn trclr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pipetre::Trclr,
        pipetre::Trclr,
        Pipetre_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pipetre::Trclr,
            pipetre::Trclr,
            Pipetre_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Transaction Counter Enable"]
    #[inline(always)]
    pub fn trenb(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        pipetre::Trenb,
        pipetre::Trenb,
        Pipetre_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            pipetre::Trenb,
            pipetre::Trenb,
            Pipetre_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pipetre {
    #[inline(always)]
    fn default() -> Pipetre {
        <crate::RegValueT<Pipetre_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pipetre {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trclr_SPEC;
    pub type Trclr = crate::EnumBitfieldStruct<u8, Trclr_SPEC>;
    impl Trclr {
        #[doc = "Invalid (writing 0 has no effect)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clear current counter value"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trenb_SPEC;
    pub type Trenb = crate::EnumBitfieldStruct<u8, Trenb_SPEC>;
    impl Trenb {
        #[doc = "Disable transaction counter"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable transaction counter"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pipetrn_SPEC;
impl crate::sealed::RegSpec for Pipetrn_SPEC {
    type DataType = u16;
}

#[doc = "Pipe %s Transaction Counter Register"]
pub type Pipetrn = crate::RegValueT<Pipetrn_SPEC>;

impl Pipetrn {
    #[doc = "Transaction Counter"]
    #[inline(always)]
    pub fn trncnt(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Pipetrn_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Pipetrn_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pipetrn {
    #[inline(always)]
    fn default() -> Pipetrn {
        <crate::RegValueT<Pipetrn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Devadd_SPEC;
impl crate::sealed::RegSpec for Devadd_SPEC {
    type DataType = u16;
}

#[doc = "Device Address %s Configuration Register"]
pub type Devadd = crate::RegValueT<Devadd_SPEC>;

impl Devadd {
    #[doc = "Transfer Speed of Communication Target Device"]
    #[inline(always)]
    pub fn usbspd(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        devadd::Usbspd,
        devadd::Usbspd,
        Devadd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            devadd::Usbspd,
            devadd::Usbspd,
            Devadd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Communication Target Connecting Hub Port"]
    #[inline(always)]
    pub fn hubport(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        devadd::Hubport,
        devadd::Hubport,
        Devadd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            devadd::Hubport,
            devadd::Hubport,
            Devadd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Communication Target Connecting Hub Register"]
    #[inline(always)]
    pub fn upphub(
        self,
    ) -> crate::common::RegisterField<
        11,
        0xf,
        1,
        0,
        devadd::Upphub,
        devadd::Upphub,
        Devadd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0xf,
            1,
            0,
            devadd::Upphub,
            devadd::Upphub,
            Devadd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Devadd {
    #[inline(always)]
    fn default() -> Devadd {
        <crate::RegValueT<Devadd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod devadd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usbspd_SPEC;
    pub type Usbspd = crate::EnumBitfieldStruct<u8, Usbspd_SPEC>;
    impl Usbspd {
        #[doc = "Do not use DEVADDm"]
        pub const _00: Self = Self::new(0);

        #[doc = "Low speed"]
        pub const _01: Self = Self::new(1);

        #[doc = "Full speed"]
        pub const _10: Self = Self::new(2);

        #[doc = "High speed"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hubport_SPEC;
    pub type Hubport = crate::EnumBitfieldStruct<u8, Hubport_SPEC>;
    impl Hubport {
        #[doc = "Connect directly to the USBHS port"]
        pub const _000: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Upphub_SPEC;
    pub type Upphub = crate::EnumBitfieldStruct<u8, Upphub_SPEC>;
    impl Upphub {
        #[doc = "Connect directly to the USBHS port"]
        pub const _0_X_0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Devadda_SPEC;
impl crate::sealed::RegSpec for Devadda_SPEC {
    type DataType = u16;
}

#[doc = "Device Address A Configuration Register"]
pub type Devadda = crate::RegValueT<Devadda_SPEC>;

impl Devadda {
    #[doc = "Transfer Speed of Communication Target Device"]
    #[inline(always)]
    pub fn usbspd(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        devadda::Usbspd,
        devadda::Usbspd,
        Devadda_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            devadda::Usbspd,
            devadda::Usbspd,
            Devadda_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Communication Target Connecting Hub Port"]
    #[inline(always)]
    pub fn hubport(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        devadda::Hubport,
        devadda::Hubport,
        Devadda_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            devadda::Hubport,
            devadda::Hubport,
            Devadda_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Communication Target Connecting Hub Register"]
    #[inline(always)]
    pub fn upphub(
        self,
    ) -> crate::common::RegisterField<
        11,
        0xf,
        1,
        0,
        devadda::Upphub,
        devadda::Upphub,
        Devadda_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0xf,
            1,
            0,
            devadda::Upphub,
            devadda::Upphub,
            Devadda_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Devadda {
    #[inline(always)]
    fn default() -> Devadda {
        <crate::RegValueT<Devadda_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod devadda {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usbspd_SPEC;
    pub type Usbspd = crate::EnumBitfieldStruct<u8, Usbspd_SPEC>;
    impl Usbspd {
        #[doc = "Do not use DEVADDm"]
        pub const _00: Self = Self::new(0);

        #[doc = "Low speed"]
        pub const _01: Self = Self::new(1);

        #[doc = "Full speed"]
        pub const _10: Self = Self::new(2);

        #[doc = "High speed"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hubport_SPEC;
    pub type Hubport = crate::EnumBitfieldStruct<u8, Hubport_SPEC>;
    impl Hubport {
        #[doc = "Connect directly to the USBHS port"]
        pub const _000: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Upphub_SPEC;
    pub type Upphub = crate::EnumBitfieldStruct<u8, Upphub_SPEC>;
    impl Upphub {
        #[doc = "Connect directly to the USBHS port"]
        pub const _0_X_0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpctrl_SPEC;
impl crate::sealed::RegSpec for Lpctrl_SPEC {
    type DataType = u16;
}

#[doc = "Low Power Control Register"]
pub type Lpctrl = crate::RegValueT<Lpctrl_SPEC>;

impl Lpctrl {
    #[doc = "Resume Return Mode Setting"]
    #[inline(always)]
    pub fn hwupm(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        lpctrl::Hwupm,
        lpctrl::Hwupm,
        Lpctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            lpctrl::Hwupm,
            lpctrl::Hwupm,
            Lpctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lpctrl {
    #[inline(always)]
    fn default() -> Lpctrl {
        <crate::RegValueT<Lpctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lpctrl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hwupm_SPEC;
    pub type Hwupm = crate::EnumBitfieldStruct<u8, Hwupm_SPEC>;
    impl Hwupm {
        #[doc = "Hardware does not recover while CPU clock inactive"]
        pub const _0: Self = Self::new(0);

        #[doc = "Hardware recovers while CPU clock inactive"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpsts_SPEC;
impl crate::sealed::RegSpec for Lpsts_SPEC {
    type DataType = u16;
}

#[doc = "Low Power Status Register"]
pub type Lpsts = crate::RegValueT<Lpsts_SPEC>;

impl Lpsts {
    #[doc = "UTMI SuspendM Control"]
    #[inline(always)]
    pub fn suspendm(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        lpsts::Suspendm,
        lpsts::Suspendm,
        Lpsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            lpsts::Suspendm,
            lpsts::Suspendm,
            Lpsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lpsts {
    #[inline(always)]
    fn default() -> Lpsts {
        <crate::RegValueT<Lpsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lpsts {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Suspendm_SPEC;
    pub type Suspendm = crate::EnumBitfieldStruct<u8, Suspendm_SPEC>;
    impl Suspendm {
        #[doc = "UTMI suspension mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "UTMI normal mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcctrl_SPEC;
impl crate::sealed::RegSpec for Bcctrl_SPEC {
    type DataType = u16;
}

#[doc = "Battery Charging Control Register"]
pub type Bcctrl = crate::RegValueT<Bcctrl_SPEC>;

impl Bcctrl {
    #[doc = "IDPSRC Control"]
    #[inline(always)]
    pub fn idpsrce(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bcctrl::Idpsrce,
        bcctrl::Idpsrce,
        Bcctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bcctrl::Idpsrce,
            bcctrl::Idpsrce,
            Bcctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IDMSINK Control"]
    #[inline(always)]
    pub fn idmsinke(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        bcctrl::Idmsinke,
        bcctrl::Idmsinke,
        Bcctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            bcctrl::Idmsinke,
            bcctrl::Idmsinke,
            Bcctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VDPSRC Control"]
    #[inline(always)]
    pub fn vdpsrce(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        bcctrl::Vdpsrce,
        bcctrl::Vdpsrce,
        Bcctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            bcctrl::Vdpsrce,
            bcctrl::Vdpsrce,
            Bcctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "IDPSINK Control"]
    #[inline(always)]
    pub fn idpsinke(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        bcctrl::Idpsinke,
        bcctrl::Idpsinke,
        Bcctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            bcctrl::Idpsinke,
            bcctrl::Idpsinke,
            Bcctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VDMSRC Control"]
    #[inline(always)]
    pub fn vdmsrce(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        bcctrl::Vdmsrce,
        bcctrl::Vdmsrce,
        Bcctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            bcctrl::Vdmsrce,
            bcctrl::Vdmsrce,
            Bcctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DCP Mode Control"]
    #[inline(always)]
    pub fn dcpmode(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        bcctrl::Dcpmode,
        bcctrl::Dcpmode,
        Bcctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            bcctrl::Dcpmode,
            bcctrl::Dcpmode,
            Bcctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CHGDET Status Flag"]
    #[inline(always)]
    pub fn chgdetsts(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        bcctrl::Chgdetsts,
        bcctrl::Chgdetsts,
        Bcctrl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            bcctrl::Chgdetsts,
            bcctrl::Chgdetsts,
            Bcctrl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "PDDET Status Flag"]
    #[inline(always)]
    pub fn pddetsts(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        bcctrl::Pddetsts,
        bcctrl::Pddetsts,
        Bcctrl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            bcctrl::Pddetsts,
            bcctrl::Pddetsts,
            Bcctrl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bcctrl {
    #[inline(always)]
    fn default() -> Bcctrl {
        <crate::RegValueT<Bcctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bcctrl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idpsrce_SPEC;
    pub type Idpsrce = crate::EnumBitfieldStruct<u8, Idpsrce_SPEC>;
    impl Idpsrce {
        #[doc = "Disable IDP_SRC circuit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable IDP_SRC circuit"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idmsinke_SPEC;
    pub type Idmsinke = crate::EnumBitfieldStruct<u8, Idmsinke_SPEC>;
    impl Idmsinke {
        #[doc = "Disable IDM_SINK circuit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable IDM_SINK circuit"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vdpsrce_SPEC;
    pub type Vdpsrce = crate::EnumBitfieldStruct<u8, Vdpsrce_SPEC>;
    impl Vdpsrce {
        #[doc = "Disable VDP_SRC circuit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable VDP_SRC circuit"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idpsinke_SPEC;
    pub type Idpsinke = crate::EnumBitfieldStruct<u8, Idpsinke_SPEC>;
    impl Idpsinke {
        #[doc = "Disable IDP_SINK circuit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable IDP_SINK circuit"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vdmsrce_SPEC;
    pub type Vdmsrce = crate::EnumBitfieldStruct<u8, Vdmsrce_SPEC>;
    impl Vdmsrce {
        #[doc = "Disable VDM_SRC circuit"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable VDM_SRC circuit"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcpmode_SPEC;
    pub type Dcpmode = crate::EnumBitfieldStruct<u8, Dcpmode_SPEC>;
    impl Dcpmode {
        #[doc = "Disable RDCP_DAT resistor"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable RDCP_DAT resistor"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chgdetsts_SPEC;
    pub type Chgdetsts = crate::EnumBitfieldStruct<u8, Chgdetsts_SPEC>;
    impl Chgdetsts {
        #[doc = "The CHGDET pin is at low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "The CHGDET pin is at high level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pddetsts_SPEC;
    pub type Pddetsts = crate::EnumBitfieldStruct<u8, Pddetsts_SPEC>;
    impl Pddetsts {
        #[doc = "The PDDET pin is at low level"]
        pub const _0: Self = Self::new(0);

        #[doc = "The PDDET pin is at high level"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pl1Ctrl1_SPEC;
impl crate::sealed::RegSpec for Pl1Ctrl1_SPEC {
    type DataType = u16;
}

#[doc = "Function L1 Control Register 1"]
pub type Pl1Ctrl1 = crate::RegValueT<Pl1Ctrl1_SPEC>;

impl Pl1Ctrl1 {
    #[doc = "L1 Response Enable"]
    #[inline(always)]
    pub fn l1respen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pl1ctrl1::L1Respen,
        pl1ctrl1::L1Respen,
        Pl1Ctrl1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pl1ctrl1::L1Respen,
            pl1ctrl1::L1Respen,
            Pl1Ctrl1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "L1 Response Mode"]
    #[inline(always)]
    pub fn l1respmd(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x3,
        1,
        0,
        pl1ctrl1::L1Respmd,
        pl1ctrl1::L1Respmd,
        Pl1Ctrl1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x3,
            1,
            0,
            pl1ctrl1::L1Respmd,
            pl1ctrl1::L1Respmd,
            Pl1Ctrl1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "L1 Response Negotiation Control"]
    #[inline(always)]
    pub fn l1negomd(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        pl1ctrl1::L1Negomd,
        pl1ctrl1::L1Negomd,
        Pl1Ctrl1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            pl1ctrl1::L1Negomd,
            pl1ctrl1::L1Negomd,
            Pl1Ctrl1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DVSQ Extension Flag"]
    #[inline(always)]
    pub fn dvsq(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        pl1ctrl1::Dvsq,
        pl1ctrl1::Dvsq,
        Pl1Ctrl1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            pl1ctrl1::Dvsq,
            pl1ctrl1::Dvsq,
            Pl1Ctrl1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "L1 Response Negotiation Threshold Value"]
    #[inline(always)]
    pub fn hirdthr(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, Pl1Ctrl1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,Pl1Ctrl1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "PHY Control Mode at L1 Return"]
    #[inline(always)]
    pub fn l1extmd(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        pl1ctrl1::L1Extmd,
        pl1ctrl1::L1Extmd,
        Pl1Ctrl1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pl1ctrl1::L1Extmd,
            pl1ctrl1::L1Extmd,
            Pl1Ctrl1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pl1Ctrl1 {
    #[inline(always)]
    fn default() -> Pl1Ctrl1 {
        <crate::RegValueT<Pl1Ctrl1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pl1ctrl1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L1Respen_SPEC;
    pub type L1Respen = crate::EnumBitfieldStruct<u8, L1Respen_SPEC>;
    impl L1Respen {
        #[doc = "Do not support LPM"]
        pub const _0: Self = Self::new(0);

        #[doc = "Support LPM"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L1Respmd_SPEC;
    pub type L1Respmd = crate::EnumBitfieldStruct<u8, L1Respmd_SPEC>;
    impl L1Respmd {
        #[doc = "NYET response"]
        pub const _00: Self = Self::new(0);

        #[doc = "ACK response"]
        pub const _01: Self = Self::new(1);

        #[doc = "STALL response"]
        pub const _10: Self = Self::new(2);

        #[doc = "Response based on L1NEGOMD setting"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L1Negomd_SPEC;
    pub type L1Negomd = crate::EnumBitfieldStruct<u8, L1Negomd_SPEC>;
    impl L1Negomd {
        #[doc = "Return ACK when received HIRD is larger than HIRDTHR\\[3:0\\]. Otherwise (including when HIRD = HIRDTHR\\[3:0\\]), return NYET"]
        pub const _0: Self = Self::new(0);

        #[doc = "Return ACK when received HIRD is smaller than HIRDTHR\\[3:0\\]. Otherwise (including when HIRD = HIRDTHR\\[3:0\\]), return NYET"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvsq_SPEC;
    pub type Dvsq = crate::EnumBitfieldStruct<u8, Dvsq_SPEC>;
    impl Dvsq {
        #[doc = "Powered state"]
        pub const _0000: Self = Self::new(0);

        #[doc = "Default state"]
        pub const _0001: Self = Self::new(1);

        #[doc = "Address state"]
        pub const _0010: Self = Self::new(2);

        #[doc = "Configured state"]
        pub const _0011: Self = Self::new(3);

        #[doc = "Suspend state"]
        pub const _01_XX: Self = Self::new(4);

        #[doc = "L1 state"]
        pub const _10_XX: Self = Self::new(8);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L1Extmd_SPEC;
    pub type L1Extmd = crate::EnumBitfieldStruct<u8, L1Extmd_SPEC>;
    impl L1Extmd {
        #[doc = "Do not set LPSTS.SUSPENDM bit through hardware when Host K is received"]
        pub const _0: Self = Self::new(0);

        #[doc = "Set LPSTS.SUSPENDM bit through hardware when Host K is received"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pl1Ctrl2_SPEC;
impl crate::sealed::RegSpec for Pl1Ctrl2_SPEC {
    type DataType = u16;
}

#[doc = "Function L1 Control Register 2"]
pub type Pl1Ctrl2 = crate::RegValueT<Pl1Ctrl2_SPEC>;

impl Pl1Ctrl2 {
    #[doc = "HIRD Value Monitor"]
    #[inline(always)]
    pub fn hirdmon(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, Pl1Ctrl2_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,Pl1Ctrl2_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "RWE Value Monitor"]
    #[inline(always)]
    pub fn rwemon(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Pl1Ctrl2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Pl1Ctrl2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Pl1Ctrl2 {
    #[inline(always)]
    fn default() -> Pl1Ctrl2 {
        <crate::RegValueT<Pl1Ctrl2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hl1Ctrl1_SPEC;
impl crate::sealed::RegSpec for Hl1Ctrl1_SPEC {
    type DataType = u16;
}

#[doc = "Host L1 Control Register 1"]
pub type Hl1Ctrl1 = crate::RegValueT<Hl1Ctrl1_SPEC>;

impl Hl1Ctrl1 {
    #[doc = "L1 Transition Request"]
    #[inline(always)]
    pub fn l1req(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Hl1Ctrl1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Hl1Ctrl1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "L1 Request Completion Status"]
    #[inline(always)]
    pub fn l1status(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x3,
        1,
        0,
        hl1ctrl1::L1Status,
        hl1ctrl1::L1Status,
        Hl1Ctrl1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x3,
            1,
            0,
            hl1ctrl1::L1Status,
            hl1ctrl1::L1Status,
            Hl1Ctrl1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Hl1Ctrl1 {
    #[inline(always)]
    fn default() -> Hl1Ctrl1 {
        <crate::RegValueT<Hl1Ctrl1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod hl1ctrl1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L1Status_SPEC;
    pub type L1Status = crate::EnumBitfieldStruct<u8, L1Status_SPEC>;
    impl L1Status {
        #[doc = "ACK received"]
        pub const _00: Self = Self::new(0);

        #[doc = "NYET received"]
        pub const _01: Self = Self::new(1);

        #[doc = "STALL received"]
        pub const _10: Self = Self::new(2);

        #[doc = "Transaction error"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hl1Ctrl2_SPEC;
impl crate::sealed::RegSpec for Hl1Ctrl2_SPEC {
    type DataType = u16;
}

#[doc = "Host L1 Control Register 2"]
pub type Hl1Ctrl2 = crate::RegValueT<Hl1Ctrl2_SPEC>;

impl Hl1Ctrl2 {
    #[doc = "LPM Token DeviceAddress"]
    #[inline(always)]
    pub fn l1addr(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Hl1Ctrl2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Hl1Ctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "LPM Token HIRD"]
    #[inline(always)]
    pub fn hird(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, Hl1Ctrl2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,Hl1Ctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "LPM Token L1 RemoteWake Enable"]
    #[inline(always)]
    pub fn l1rwe(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Hl1Ctrl2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Hl1Ctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "BESL & Alternate HIRD"]
    #[inline(always)]
    pub fn besl(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Hl1Ctrl2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Hl1Ctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Hl1Ctrl2 {
    #[inline(always)]
    fn default() -> Hl1Ctrl2 {
        <crate::RegValueT<Hl1Ctrl2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpusr0R_SPEC;
impl crate::sealed::RegSpec for Dpusr0R_SPEC {
    type DataType = u32;
}

#[doc = "Deep Software Standby USB Transceiver Control/Pin Monitor Register"]
pub type Dpusr0R = crate::RegValueT<Dpusr0R_SPEC>;

impl Dpusr0R {
    #[doc = "OVRCURA Input Flag"]
    #[inline(always)]
    pub fn dovcahm(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Dpusr0R_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Dpusr0R_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "OVRCURB Input Flag"]
    #[inline(always)]
    pub fn dovcbhm(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Dpusr0R_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Dpusr0R_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "VBUS Input Flag"]
    #[inline(always)]
    pub fn dvbstshm(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Dpusr0R_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Dpusr0R_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Dpusr0R {
    #[inline(always)]
    fn default() -> Dpusr0R {
        <crate::RegValueT<Dpusr0R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpusr1R_SPEC;
impl crate::sealed::RegSpec for Dpusr1R_SPEC {
    type DataType = u32;
}

#[doc = "Deep Software Standby USB Suspend/Resume Interrupt Register"]
pub type Dpusr1R = crate::RegValueT<Dpusr1R_SPEC>;

impl Dpusr1R {
    #[doc = "OVRCURA Interrupt Enable Clear"]
    #[inline(always)]
    pub fn dovcahe(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        dpusr1r::Dovcahe,
        dpusr1r::Dovcahe,
        Dpusr1R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            dpusr1r::Dovcahe,
            dpusr1r::Dovcahe,
            Dpusr1R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "OVRCURB Interrupt Enable Clear"]
    #[inline(always)]
    pub fn dovcbhe(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        dpusr1r::Dovcbhe,
        dpusr1r::Dovcbhe,
        Dpusr1R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            dpusr1r::Dovcbhe,
            dpusr1r::Dovcbhe,
            Dpusr1R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "VBUS Interrupt Enable/Clear"]
    #[inline(always)]
    pub fn dvbstshe(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        dpusr1r::Dvbstshe,
        dpusr1r::Dvbstshe,
        Dpusr1R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dpusr1r::Dvbstshe,
            dpusr1r::Dvbstshe,
            Dpusr1R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "OVRCURA Interrupt Source Return Status Flag"]
    #[inline(always)]
    pub fn dovcah(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        dpusr1r::Dovcah,
        dpusr1r::Dovcah,
        Dpusr1R_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            dpusr1r::Dovcah,
            dpusr1r::Dovcah,
            Dpusr1R_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "OVRCURB Interrupt Source Return Status Flag"]
    #[inline(always)]
    pub fn dovcbh(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        dpusr1r::Dovcbh,
        dpusr1r::Dovcbh,
        Dpusr1R_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            dpusr1r::Dovcbh,
            dpusr1r::Dovcbh,
            Dpusr1R_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "VBUS Interrupt Source Return Status Flag"]
    #[inline(always)]
    pub fn dvbstsh(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        dpusr1r::Dvbstsh,
        dpusr1r::Dvbstsh,
        Dpusr1R_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            dpusr1r::Dvbstsh,
            dpusr1r::Dvbstsh,
            Dpusr1R_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpusr1R {
    #[inline(always)]
    fn default() -> Dpusr1R {
        <crate::RegValueT<Dpusr1R_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpusr1r {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dovcahe_SPEC;
    pub type Dovcahe = crate::EnumBitfieldStruct<u8, Dovcahe_SPEC>;
    impl Dovcahe {
        #[doc = "Disable recovery from Deep Software Standby mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable recovery from Deep Software Standby mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dovcbhe_SPEC;
    pub type Dovcbhe = crate::EnumBitfieldStruct<u8, Dovcbhe_SPEC>;
    impl Dovcbhe {
        #[doc = "Disable recovery from Deep Software Standby mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable recovery from Deep Software Standby mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvbstshe_SPEC;
    pub type Dvbstshe = crate::EnumBitfieldStruct<u8, Dvbstshe_SPEC>;
    impl Dvbstshe {
        #[doc = "Disable recovery from Deep Software Standby mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable recovery from Deep Software Standby mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dovcah_SPEC;
    pub type Dovcah = crate::EnumBitfieldStruct<u8, Dovcah_SPEC>;
    impl Dovcah {
        #[doc = "System has not recovered from Deep Software Standby mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "System recovered from Deep Software Standby mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dovcbh_SPEC;
    pub type Dovcbh = crate::EnumBitfieldStruct<u8, Dovcbh_SPEC>;
    impl Dovcbh {
        #[doc = "System has not recovered from Deep Software Standby mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "System recovered from Deep Software Standby mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvbstsh_SPEC;
    pub type Dvbstsh = crate::EnumBitfieldStruct<u8, Dvbstsh_SPEC>;
    impl Dvbstsh {
        #[doc = "System has not recovered from Deep Software Standby mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "System recovered from Deep Software Standby mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpusr2R_SPEC;
impl crate::sealed::RegSpec for Dpusr2R_SPEC {
    type DataType = u16;
}

#[doc = "Deep Software Standby USB Suspend/Resume Interrupt Register"]
pub type Dpusr2R = crate::RegValueT<Dpusr2R_SPEC>;

impl Dpusr2R {
    #[doc = "Indication of Return from DP Interrupt Source"]
    #[inline(always)]
    pub fn dpint(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpusr2r::Dpint,
        dpusr2r::Dpint,
        Dpusr2R_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpusr2r::Dpint,
            dpusr2r::Dpint,
            Dpusr2R_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Indication of Return from DM Interrupt Source"]
    #[inline(always)]
    pub fn dmint(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dpusr2r::Dmint,
        dpusr2r::Dmint,
        Dpusr2R_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dpusr2r::Dmint,
            dpusr2r::Dmint,
            Dpusr2R_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "DP Input"]
    #[inline(always)]
    pub fn dpval(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dpusr2R_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Dpusr2R_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DM Input"]
    #[inline(always)]
    pub fn dmval(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dpusr2R_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Dpusr2R_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "DP Interrupt Enable Clear"]
    #[inline(always)]
    pub fn dpinte(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        dpusr2r::Dpinte,
        dpusr2r::Dpinte,
        Dpusr2R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            dpusr2r::Dpinte,
            dpusr2r::Dpinte,
            Dpusr2R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "DM Interrupt Enable Clear"]
    #[inline(always)]
    pub fn dminte(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        dpusr2r::Dminte,
        dpusr2r::Dminte,
        Dpusr2R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            dpusr2r::Dminte,
            dpusr2r::Dminte,
            Dpusr2R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpusr2R {
    #[inline(always)]
    fn default() -> Dpusr2R {
        <crate::RegValueT<Dpusr2R_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpusr2r {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpint_SPEC;
    pub type Dpint = crate::EnumBitfieldStruct<u8, Dpint_SPEC>;
    impl Dpint {
        #[doc = "System has not recovered from Deep Software Standby mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "System recovered from Deep Software Standby mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dmint_SPEC;
    pub type Dmint = crate::EnumBitfieldStruct<u8, Dmint_SPEC>;
    impl Dmint {
        #[doc = "System has not recovered from Deep Software Standby mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "System recovered from Deep Software Standby mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dpinte_SPEC;
    pub type Dpinte = crate::EnumBitfieldStruct<u8, Dpinte_SPEC>;
    impl Dpinte {
        #[doc = "Disable recovery from Deep Software Standby mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable recovery from Deep Software Standby mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dminte_SPEC;
    pub type Dminte = crate::EnumBitfieldStruct<u8, Dminte_SPEC>;
    impl Dminte {
        #[doc = "Disable recovery from Deep Software Standby mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enable recovery from Deep Software Standby mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpusrcr_SPEC;
impl crate::sealed::RegSpec for Dpusrcr_SPEC {
    type DataType = u16;
}

#[doc = "Deep Software Standby USB Suspend/Resume Command Register"]
pub type Dpusrcr = crate::RegValueT<Dpusrcr_SPEC>;

impl Dpusrcr {
    #[doc = "USB Transceiver Control Fix"]
    #[inline(always)]
    pub fn fixphy(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dpusrcr::Fixphy,
        dpusrcr::Fixphy,
        Dpusrcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dpusrcr::Fixphy,
            dpusrcr::Fixphy,
            Dpusrcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "USB Transceiver Control Fix for PLL"]
    #[inline(always)]
    pub fn fixphypd(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dpusrcr::Fixphypd,
        dpusrcr::Fixphypd,
        Dpusrcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dpusrcr::Fixphypd,
            dpusrcr::Fixphypd,
            Dpusrcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dpusrcr {
    #[inline(always)]
    fn default() -> Dpusrcr {
        <crate::RegValueT<Dpusrcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpusrcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fixphy_SPEC;
    pub type Fixphy = crate::EnumBitfieldStruct<u8, Fixphy_SPEC>;
    impl Fixphy {
        #[doc = "Normal mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Invoke/recover from Deep Software Standby mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fixphypd_SPEC;
    pub type Fixphypd = crate::EnumBitfieldStruct<u8, Fixphypd_SPEC>;
    impl Fixphypd {
        #[doc = "Normal mode"]
        pub const _0: Self = Self::new(0);

        #[doc = "Invoke/recover from Deep Software Standby mode"]
        pub const _1: Self = Self::new(1);
    }
}
