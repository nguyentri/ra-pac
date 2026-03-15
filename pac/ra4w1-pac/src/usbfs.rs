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
#[doc = r"USB 2.0 FS Module"]
unsafe impl ::core::marker::Send for super::Usbfs {}
unsafe impl ::core::marker::Sync for super::Usbfs {}
impl super::Usbfs {
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

    #[doc = "System Configuration Status Register 0"]
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

    #[doc = "CFIFO Port Register"]
    #[inline(always)]
    pub const fn cfifo(&self) -> &'static crate::common::Reg<self::Cfifo_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfifo_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "CFIFO Port Register L"]
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

    #[doc = "D0FIFO Port Register"]
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

    #[doc = "D0FIFO Port Register L"]
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

    #[doc = "D1FIFO Port Register"]
    #[inline(always)]
    pub const fn d1fifo(
        &self,
    ) -> &'static crate::common::Reg<self::D1Fifo_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D1Fifo_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "D1FIFO Port Register L"]
    #[inline(always)]
    pub const fn d1fifol(
        &self,
    ) -> &'static crate::common::Reg<self::D1Fifol_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D1Fifol_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "CFIFO Port Select Register"]
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

    #[doc = "CFIFO Port Control Register"]
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

    #[doc = "D0FIFO Port Select Register"]
    #[inline(always)]
    pub const fn d0fifosel(
        &self,
    ) -> &'static crate::common::Reg<self::D0Fifosel_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D0Fifosel_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "D0FIFO Port Control Register"]
    #[inline(always)]
    pub const fn d0fifoctr(
        &self,
    ) -> &'static crate::common::Reg<self::D0Fifoctr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D0Fifoctr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(42usize),
            )
        }
    }

    #[doc = "D1FIFO Port Select Register"]
    #[inline(always)]
    pub const fn d1fifosel(
        &self,
    ) -> &'static crate::common::Reg<self::D1Fifosel_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D1Fifosel_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "D1FIFO Port Control Register"]
    #[inline(always)]
    pub const fn d1fifoctr(
        &self,
    ) -> &'static crate::common::Reg<self::D1Fifoctr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::D1Fifoctr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(46usize),
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
        4,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x7ausize))
        }
    }
    #[inline(always)]
    pub const fn pipe6ctr(
        &self,
    ) -> &'static crate::common::Reg<self::Pipectr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pipectr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7ausize),
            )
        }
    }
    #[inline(always)]
    pub const fn pipe7ctr(
        &self,
    ) -> &'static crate::common::Reg<self::Pipectr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pipectr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7cusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pipe8ctr(
        &self,
    ) -> &'static crate::common::Reg<self::Pipectr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pipectr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x7eusize),
            )
        }
    }
    #[inline(always)]
    pub const fn pipe9ctr(
        &self,
    ) -> &'static crate::common::Reg<self::Pipectr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pipectr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x80usize),
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
        6,
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

    #[doc = "USB Module Control Register"]
    #[inline(always)]
    pub const fn usbmc(&self) -> &'static crate::common::Reg<self::Usbmc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Usbmc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(204usize),
            )
        }
    }

    #[doc = "BC Control Register 0"]
    #[inline(always)]
    pub const fn usbbcctrl0(
        &self,
    ) -> &'static crate::common::Reg<self::Usbbcctrl0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Usbbcctrl0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(176usize),
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
    #[doc = "USB Clock Enable"]
    #[inline(always)]
    pub fn scke(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        syscfg::Scke,
        syscfg::Scke,
        Syscfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            syscfg::Scke,
            syscfg::Scke,
            Syscfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CNEN Single End Receiver Enable"]
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Syscfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Syscfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Controller Function Select"]
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

    #[doc = "D- Line Resistor Control"]
    #[inline(always)]
    pub fn dmrpu(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        syscfg::Dmrpu,
        syscfg::Dmrpu,
        Syscfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            syscfg::Dmrpu,
            syscfg::Dmrpu,
            Syscfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "USB Operation Enable"]
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
}
impl ::core::default::Default for Syscfg {
    #[inline(always)]
    fn default() -> Syscfg {
        <crate::RegValueT<Syscfg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syscfg {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scke_SPEC;
    pub type Scke = crate::EnumBitfieldStruct<u8, Scke_SPEC>;
    impl Scke {
        #[doc = "Clock supply to the USBFS stopped"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clock supply to the USBFS enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cnen_SPEC;
    pub type Cnen = crate::EnumBitfieldStruct<u8, Cnen_SPEC>;
    impl Cnen {
        #[doc = "Single end receiver disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Single end receiver enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcfm_SPEC;
    pub type Dcfm = crate::EnumBitfieldStruct<u8, Dcfm_SPEC>;
    impl Dcfm {
        #[doc = "Device controller selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Host controller selected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Drpd_SPEC;
    pub type Drpd = crate::EnumBitfieldStruct<u8, Drpd_SPEC>;
    impl Drpd {
        #[doc = "Line pull-down disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Line pull-down enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dprpu_SPEC;
    pub type Dprpu = crate::EnumBitfieldStruct<u8, Dprpu_SPEC>;
    impl Dprpu {
        #[doc = "Line pull-down disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Line pull-down enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dmrpu_SPEC;
    pub type Dmrpu = crate::EnumBitfieldStruct<u8, Dmrpu_SPEC>;
    impl Dmrpu {
        #[doc = "Line pull-up disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Line pull-up enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usbe_SPEC;
    pub type Usbe = crate::EnumBitfieldStruct<u8, Usbe_SPEC>;
    impl Usbe {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syssts0_SPEC;
impl crate::sealed::RegSpec for Syssts0_SPEC {
    type DataType = u16;
}

#[doc = "System Configuration Status Register 0"]
pub type Syssts0 = crate::RegValueT<Syssts0_SPEC>;

impl Syssts0 {
    #[doc = "External USB0_OVRCURA/ USB0_OVRCURB Input Pin MonitorThe OCVMON\\[1\\] bit indicates the status of the USBHS_OVRCURA pin. The OCVMON\\[0\\] bit indicates the status of the USBHS_OVRCURB pin."]
    #[inline(always)]
    pub fn ovcmon(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, Syssts0_SPEC, crate::common::R> {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,Syssts0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "USB Host Sequencer Status Monitor"]
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

    #[doc = "These bits are read as 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, Syssts0_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,Syssts0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "External ID0 Input Pin Monitor"]
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

    #[doc = "USB Data Line Status Monitor"]
    #[inline(always)]
    pub fn lnst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        syssts0::Lnst,
        syssts0::Lnst,
        Syssts0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            syssts0::Lnst,
            syssts0::Lnst,
            Syssts0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
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
    pub struct Htact_SPEC;
    pub type Htact = crate::EnumBitfieldStruct<u8, Htact_SPEC>;
    impl Htact {
        #[doc = "Host sequencer completely stopped"]
        pub const _0: Self = Self::new(0);

        #[doc = "Host sequencer not completely stopped."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idmon_SPEC;
    pub type Idmon = crate::EnumBitfieldStruct<u8, Idmon_SPEC>;
    impl Idmon {
        #[doc = "USB0_ID pin is low"]
        pub const _0: Self = Self::new(0);

        #[doc = "USB0_ID pin is high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lnst_SPEC;
    pub type Lnst = crate::EnumBitfieldStruct<u8, Lnst_SPEC>;
    impl Lnst {
        #[doc = "SE0"]
        pub const _00: Self = Self::new(0);

        #[doc = "K-State (FS) / J-State(LS)"]
        pub const _01: Self = Self::new(1);

        #[doc = "J-State(FS) / K-State(LS)"]
        pub const _10: Self = Self::new(2);

        #[doc = "SE1"]
        pub const _11: Self = Self::new(3);
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
    #[doc = "Host Negotiation Protocol (HNP) Control     This bit is used when switching from device B to device A while in OTG mode. If the HNPBTOA bit is 1, the internal function control keeps the suspended state until the HNP processing ends even  though SYSCFG.DPRPU = 0 or SYSCFG.DCFM = 1 is set."]
    #[inline(always)]
    pub fn hnpbtoa(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        dvstctr0::Hnpbtoa,
        dvstctr0::Hnpbtoa,
        Dvstctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            dvstctr0::Hnpbtoa,
            dvstctr0::Hnpbtoa,
            Dvstctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "USB_EXICEN Output Pin Control"]
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

    #[doc = "USB_VBUSEN Output Pin Control"]
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

    #[doc = "Wakeup Output"]
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

    #[doc = "Wakeup Detection Enable"]
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

    #[doc = "USB Bus Reset Output"]
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

    #[doc = "Resume Output"]
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

    #[doc = "USB Bus Enable"]
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dvstctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Dvstctr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "USB Bus Reset Status"]
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
}
impl ::core::default::Default for Dvstctr0 {
    #[inline(always)]
    fn default() -> Dvstctr0 {
        <crate::RegValueT<Dvstctr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dvstctr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hnpbtoa_SPEC;
    pub type Hnpbtoa = crate::EnumBitfieldStruct<u8, Hnpbtoa_SPEC>;
    impl Hnpbtoa {
        #[doc = "Normal Operation"]
        pub const _0: Self = Self::new(0);

        #[doc = "Switching from device B to device A is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Exicen_SPEC;
    pub type Exicen = crate::EnumBitfieldStruct<u8, Exicen_SPEC>;
    impl Exicen {
        #[doc = "External USB_EXICEN pin outputs low"]
        pub const _0: Self = Self::new(0);

        #[doc = "External USB_EXICEN pin outputs high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbusen_SPEC;
    pub type Vbusen = crate::EnumBitfieldStruct<u8, Vbusen_SPEC>;
    impl Vbusen {
        #[doc = "External USB_VBUSEN pin outputs low"]
        pub const _0: Self = Self::new(0);

        #[doc = "External USB_VBUSEN pin outputs high"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wkup_SPEC;
    pub type Wkup = crate::EnumBitfieldStruct<u8, Wkup_SPEC>;
    impl Wkup {
        #[doc = "Remote wakeup signal is not output."]
        pub const _0: Self = Self::new(0);

        #[doc = "Remote wakeup signal is output."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rwupe_SPEC;
    pub type Rwupe = crate::EnumBitfieldStruct<u8, Rwupe_SPEC>;
    impl Rwupe {
        #[doc = "Downstream port wakeup is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Downstream port wakeup is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usbrst_SPEC;
    pub type Usbrst = crate::EnumBitfieldStruct<u8, Usbrst_SPEC>;
    impl Usbrst {
        #[doc = "USB bus reset signal is not output."]
        pub const _0: Self = Self::new(0);

        #[doc = "USB bus reset signal is output."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Resume_SPEC;
    pub type Resume = crate::EnumBitfieldStruct<u8, Resume_SPEC>;
    impl Resume {
        #[doc = "Resume signal is not output."]
        pub const _0: Self = Self::new(0);

        #[doc = "Resume signal is output."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uact_SPEC;
    pub type Uact = crate::EnumBitfieldStruct<u8, Uact_SPEC>;
    impl Uact {
        #[doc = "Downstream port is disabled (SOF transmission is disabled)."]
        pub const _0: Self = Self::new(0);

        #[doc = "Downstream port is enabled (SOF transmission is enabled)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rhst_SPEC;
    pub type Rhst = crate::EnumBitfieldStruct<u8, Rhst_SPEC>;
    impl Rhst {
        #[doc = "Communication speed not determined"]
        pub const _000: Self = Self::new(0);

        #[doc = "Low-speed connection(When the host controller is selected) /USB bus reset in progress( When the function controller is selected)"]
        pub const _001: Self = Self::new(1);

        #[doc = "Full-speed connection(When the host controller is selected) /USB bus reset in progress or full-speed connection(When the function controller is selected)"]
        pub const _010: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _011: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfifo_SPEC;
impl crate::sealed::RegSpec for Cfifo_SPEC {
    type DataType = u16;
}

#[doc = "CFIFO Port Register"]
pub type Cfifo = crate::RegValueT<Cfifo_SPEC>;

impl Cfifo {
    #[doc = "FIFO PortRead receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
    #[inline(always)]
    pub fn fifoport(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Cfifo_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Cfifo_SPEC,crate::common::RW>::from_register(self,0)
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
    type DataType = u8;
}

#[doc = "CFIFO Port Register L"]
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
pub struct D0Fifo_SPEC;
impl crate::sealed::RegSpec for D0Fifo_SPEC {
    type DataType = u16;
}

#[doc = "D0FIFO Port Register"]
pub type D0Fifo = crate::RegValueT<D0Fifo_SPEC>;

impl D0Fifo {
    #[doc = "FIFO PortRead receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
    #[inline(always)]
    pub fn fifoport(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, D0Fifo_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,D0Fifo_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct D0Fifol_SPEC;
impl crate::sealed::RegSpec for D0Fifol_SPEC {
    type DataType = u8;
}

#[doc = "D0FIFO Port Register L"]
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
pub struct D1Fifo_SPEC;
impl crate::sealed::RegSpec for D1Fifo_SPEC {
    type DataType = u16;
}

#[doc = "D1FIFO Port Register"]
pub type D1Fifo = crate::RegValueT<D1Fifo_SPEC>;

impl D1Fifo {
    #[doc = "FIFO PortRead receive data from the FIFO buffer or write transmit data to the FIFO buffer by accessing these bits."]
    #[inline(always)]
    pub fn fifoport(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, D1Fifo_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,D1Fifo_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct D1Fifol_SPEC;
impl crate::sealed::RegSpec for D1Fifol_SPEC {
    type DataType = u8;
}

#[doc = "D1FIFO Port Register L"]
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
pub struct Cfifosel_SPEC;
impl crate::sealed::RegSpec for Cfifosel_SPEC {
    type DataType = u16;
}

#[doc = "CFIFO Port Select Register"]
pub type Cfifosel = crate::RegValueT<Cfifosel_SPEC>;

impl Cfifosel {
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
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            cfifosel::Rew,
            cfifosel::Rew,
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
        0x1,
        1,
        0,
        cfifosel::Mbw,
        cfifosel::Mbw,
        Cfifosel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            cfifosel::Mbw,
            cfifosel::Mbw,
            Cfifosel_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CFIFO Port Endian Control"]
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

    #[doc = "CFIFO Port Access Direction When DCP is Selected"]
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Cfifosel_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cfifosel_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "CFIFO Port Access Pipe Specification"]
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
}
impl ::core::default::Default for Cfifosel {
    #[inline(always)]
    fn default() -> Cfifosel {
        <crate::RegValueT<Cfifosel_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfifosel {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcnt_SPEC;
    pub type Rcnt = crate::EnumBitfieldStruct<u8, Rcnt_SPEC>;
    impl Rcnt {
        #[doc = "The DTLN\\[8:0\\] bits (CFIFOCRT.DTLN\\[8:0\\], D0FIFOCRT.DTLN\\[8:0\\], D1FIFOCRT.DTLN\\[8:0\\]) are cleared when all of the receive data has been read from the CFIFO.(In double buffer mode, the DTLN\\[8:0\\] bit value is cleared when all the data has been read from only a single plane.)"]
        pub const _0: Self = Self::new(0);

        #[doc = "The DTLN\\[8:0\\] bits are decremented each time the receive data is read from the CFIFO."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rew_SPEC;
    pub type Rew = crate::EnumBitfieldStruct<u8, Rew_SPEC>;
    impl Rew {
        #[doc = "The buffer pointer is not rewound."]
        pub const _0: Self = Self::new(0);

        #[doc = "The buffer pointer is rewound."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbw_SPEC;
    pub type Mbw = crate::EnumBitfieldStruct<u8, Mbw_SPEC>;
    impl Mbw {
        #[doc = "8-bit width"]
        pub const _0: Self = Self::new(0);

        #[doc = "16-bit width"]
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
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Reading from the buffer memory is selected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Writing to the buffer memory is selected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Curpipe_SPEC;
    pub type Curpipe = crate::EnumBitfieldStruct<u8, Curpipe_SPEC>;
    impl Curpipe {
        #[doc = "DCP (Default control pipe)"]
        pub const _0000: Self = Self::new(0);

        #[doc = "Pipe 1"]
        pub const _0001: Self = Self::new(1);

        #[doc = "Pipe 2"]
        pub const _0010: Self = Self::new(2);

        #[doc = "Pipe 3"]
        pub const _0011: Self = Self::new(3);

        #[doc = "Pipe 4"]
        pub const _0100: Self = Self::new(4);

        #[doc = "Pipe 5"]
        pub const _0101: Self = Self::new(5);

        #[doc = "Pipe 6"]
        pub const _0110: Self = Self::new(6);

        #[doc = "Pipe 7"]
        pub const _0111: Self = Self::new(7);

        #[doc = "Pipe 8"]
        pub const _1000: Self = Self::new(8);

        #[doc = "Pipe 9"]
        pub const _1001: Self = Self::new(9);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfifoctr_SPEC;
impl crate::sealed::RegSpec for Cfifoctr_SPEC {
    type DataType = u16;
}

#[doc = "CFIFO Port Control Register"]
pub type Cfifoctr = crate::RegValueT<Cfifoctr_SPEC>;

impl Cfifoctr {
    #[doc = "Buffer Memory Valid Flag"]
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

    #[doc = "CPU Buffer ClearNote: Only 0 can be read."]
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
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            cfifoctr::Bclr,
            cfifoctr::Bclr,
            Cfifoctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "FIFO Port Ready"]
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

    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0xf, 1, 0, u8, u8, Cfifoctr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0xf,1,0,u8,u8,Cfifoctr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Receive Data LengthIndicates the length of the receive data."]
    #[inline(always)]
    pub fn dtln(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Cfifoctr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Cfifoctr_SPEC,crate::common::R>::from_register(self,0)
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
    pub struct Bval_SPEC;
    pub type Bval = crate::EnumBitfieldStruct<u8, Bval_SPEC>;
    impl Bval {
        #[doc = "Invalid"]
        pub const _0: Self = Self::new(0);

        #[doc = "Writing ended"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bclr_SPEC;
    pub type Bclr = crate::EnumBitfieldStruct<u8, Bclr_SPEC>;
    impl Bclr {
        #[doc = "Does not operate"]
        pub const _0: Self = Self::new(0);

        #[doc = "FIFO buffer cleared on the CPU side."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Frdy_SPEC;
    pub type Frdy = crate::EnumBitfieldStruct<u8, Frdy_SPEC>;
    impl Frdy {
        #[doc = "FIFO port access is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "FIFO port access is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D0Fifosel_SPEC;
impl crate::sealed::RegSpec for D0Fifosel_SPEC {
    type DataType = u16;
}

#[doc = "D0FIFO Port Select Register"]
pub type D0Fifosel = crate::RegValueT<D0Fifosel_SPEC>;

impl D0Fifosel {
    #[doc = "Read Count Mode"]
    #[inline(always)]
    pub fn rcnt(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        d0fifosel::Rcnt,
        d0fifosel::Rcnt,
        D0Fifosel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            d0fifosel::Rcnt,
            d0fifosel::Rcnt,
            D0Fifosel_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Buffer Pointer RewindNote: Only 0 can be read."]
    #[inline(always)]
    pub fn rew(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        d0fifosel::Rew,
        d0fifosel::Rew,
        D0Fifosel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            d0fifosel::Rew,
            d0fifosel::Rew,
            D0Fifosel_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Auto Buffer Memory Clear Mode Accessed after Specified Pipe Data is Read"]
    #[inline(always)]
    pub fn dclrm(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        d0fifosel::Dclrm,
        d0fifosel::Dclrm,
        D0Fifosel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            d0fifosel::Dclrm,
            d0fifosel::Dclrm,
            D0Fifosel_SPEC,
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
        d0fifosel::Dreqe,
        d0fifosel::Dreqe,
        D0Fifosel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            d0fifosel::Dreqe,
            d0fifosel::Dreqe,
            D0Fifosel_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "FIFO Port Access Bit Width"]
    #[inline(always)]
    pub fn mbw(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        d0fifosel::Mbw,
        d0fifosel::Mbw,
        D0Fifosel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            d0fifosel::Mbw,
            d0fifosel::Mbw,
            D0Fifosel_SPEC,
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
        d0fifosel::Bigend,
        d0fifosel::Bigend,
        D0Fifosel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            d0fifosel::Bigend,
            d0fifosel::Bigend,
            D0Fifosel_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, D0Fifosel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,D0Fifosel_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "FIFO Port Access Pipe Specification"]
    #[inline(always)]
    pub fn curpipe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        d0fifosel::Curpipe,
        d0fifosel::Curpipe,
        D0Fifosel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            d0fifosel::Curpipe,
            d0fifosel::Curpipe,
            D0Fifosel_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for D0Fifosel {
    #[inline(always)]
    fn default() -> D0Fifosel {
        <crate::RegValueT<D0Fifosel_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod d0fifosel {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcnt_SPEC;
    pub type Rcnt = crate::EnumBitfieldStruct<u8, Rcnt_SPEC>;
    impl Rcnt {
        #[doc = "The DTLN\\[8:0\\] bits (CFIFOCRT.DTLN\\[8:0\\], D0FIFOCRT.DTLN\\[8:0\\], D1FIFOCRT.DTLN\\[8:0\\]) are cleared when all of the receive data has been read from the DnFIFO.(In double buffer mode, the DTLN bit Value is cleared when all the data has been read from only a single plane.)"]
        pub const _0: Self = Self::new(0);

        #[doc = "The DTLN\\[8:0\\] bits are decremented each time the receive data is read from the DnFIFO. (n = 0, 1)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rew_SPEC;
    pub type Rew = crate::EnumBitfieldStruct<u8, Rew_SPEC>;
    impl Rew {
        #[doc = "The buffer pointer is not rewound."]
        pub const _0: Self = Self::new(0);

        #[doc = "The buffer pointer is rewound."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dclrm_SPEC;
    pub type Dclrm = crate::EnumBitfieldStruct<u8, Dclrm_SPEC>;
    impl Dclrm {
        #[doc = "Auto buffer clear mode is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Auto buffer clear mode is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dreqe_SPEC;
    pub type Dreqe = crate::EnumBitfieldStruct<u8, Dreqe_SPEC>;
    impl Dreqe {
        #[doc = "DMA/DTC transfer request is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "DMA/DTC transfer request is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbw_SPEC;
    pub type Mbw = crate::EnumBitfieldStruct<u8, Mbw_SPEC>;
    impl Mbw {
        #[doc = "8-bit width"]
        pub const _0: Self = Self::new(0);

        #[doc = "16-bit width"]
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
    pub struct Curpipe_SPEC;
    pub type Curpipe = crate::EnumBitfieldStruct<u8, Curpipe_SPEC>;
    impl Curpipe {
        #[doc = "DCP (Default control pipe)"]
        pub const _0000: Self = Self::new(0);

        #[doc = "Pipe 1"]
        pub const _0001: Self = Self::new(1);

        #[doc = "Pipe 2"]
        pub const _0010: Self = Self::new(2);

        #[doc = "Pipe 3"]
        pub const _0011: Self = Self::new(3);

        #[doc = "Pipe 4"]
        pub const _0100: Self = Self::new(4);

        #[doc = "Pipe 5"]
        pub const _0101: Self = Self::new(5);

        #[doc = "Pipe 6"]
        pub const _0110: Self = Self::new(6);

        #[doc = "Pipe 7"]
        pub const _0111: Self = Self::new(7);

        #[doc = "Pipe 8"]
        pub const _1000: Self = Self::new(8);

        #[doc = "Pipe 9"]
        pub const _1001: Self = Self::new(9);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D0Fifoctr_SPEC;
impl crate::sealed::RegSpec for D0Fifoctr_SPEC {
    type DataType = u16;
}

#[doc = "D0FIFO Port Control Register"]
pub type D0Fifoctr = crate::RegValueT<D0Fifoctr_SPEC>;

impl D0Fifoctr {
    #[doc = "Buffer Memory Valid Flag"]
    #[inline(always)]
    pub fn bval(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        d0fifoctr::Bval,
        d0fifoctr::Bval,
        D0Fifoctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            d0fifoctr::Bval,
            d0fifoctr::Bval,
            D0Fifoctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CPU Buffer ClearNote: Only 0 can be read."]
    #[inline(always)]
    pub fn bclr(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        d0fifoctr::Bclr,
        d0fifoctr::Bclr,
        D0Fifoctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            d0fifoctr::Bclr,
            d0fifoctr::Bclr,
            D0Fifoctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "FIFO Port Ready"]
    #[inline(always)]
    pub fn frdy(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        d0fifoctr::Frdy,
        d0fifoctr::Frdy,
        D0Fifoctr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            d0fifoctr::Frdy,
            d0fifoctr::Frdy,
            D0Fifoctr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0xf, 1, 0, u8, u8, D0Fifoctr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0xf,1,0,u8,u8,D0Fifoctr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Receive Data LengthIndicates the length of the receive data."]
    #[inline(always)]
    pub fn dtln(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, D0Fifoctr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,D0Fifoctr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for D0Fifoctr {
    #[inline(always)]
    fn default() -> D0Fifoctr {
        <crate::RegValueT<D0Fifoctr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod d0fifoctr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bval_SPEC;
    pub type Bval = crate::EnumBitfieldStruct<u8, Bval_SPEC>;
    impl Bval {
        #[doc = "Invalid"]
        pub const _0: Self = Self::new(0);

        #[doc = "Writing ended"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bclr_SPEC;
    pub type Bclr = crate::EnumBitfieldStruct<u8, Bclr_SPEC>;
    impl Bclr {
        #[doc = "Does not operate"]
        pub const _0: Self = Self::new(0);

        #[doc = "FIFO buffer cleared on the CPU side."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Frdy_SPEC;
    pub type Frdy = crate::EnumBitfieldStruct<u8, Frdy_SPEC>;
    impl Frdy {
        #[doc = "FIFO port access is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "FIFO port access is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D1Fifosel_SPEC;
impl crate::sealed::RegSpec for D1Fifosel_SPEC {
    type DataType = u16;
}

#[doc = "D1FIFO Port Select Register"]
pub type D1Fifosel = crate::RegValueT<D1Fifosel_SPEC>;

impl D1Fifosel {
    #[doc = "Read Count Mode"]
    #[inline(always)]
    pub fn rcnt(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        d1fifosel::Rcnt,
        d1fifosel::Rcnt,
        D1Fifosel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            d1fifosel::Rcnt,
            d1fifosel::Rcnt,
            D1Fifosel_SPEC,
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
        d1fifosel::Rew,
        d1fifosel::Rew,
        D1Fifosel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            d1fifosel::Rew,
            d1fifosel::Rew,
            D1Fifosel_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Auto Buffer Memory Clear Mode Accessed after Specified Pipe Data is Read"]
    #[inline(always)]
    pub fn dclrm(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        d1fifosel::Dclrm,
        d1fifosel::Dclrm,
        D1Fifosel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            d1fifosel::Dclrm,
            d1fifosel::Dclrm,
            D1Fifosel_SPEC,
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
        d1fifosel::Dreqe,
        d1fifosel::Dreqe,
        D1Fifosel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            d1fifosel::Dreqe,
            d1fifosel::Dreqe,
            D1Fifosel_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "FIFO Port Access Bit Width"]
    #[inline(always)]
    pub fn mbw(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        d1fifosel::Mbw,
        d1fifosel::Mbw,
        D1Fifosel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            d1fifosel::Mbw,
            d1fifosel::Mbw,
            D1Fifosel_SPEC,
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
        d1fifosel::Bigend,
        d1fifosel::Bigend,
        D1Fifosel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            d1fifosel::Bigend,
            d1fifosel::Bigend,
            D1Fifosel_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, D1Fifosel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,D1Fifosel_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "FIFO Port Access Pipe Specification"]
    #[inline(always)]
    pub fn curpipe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        d1fifosel::Curpipe,
        d1fifosel::Curpipe,
        D1Fifosel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            d1fifosel::Curpipe,
            d1fifosel::Curpipe,
            D1Fifosel_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for D1Fifosel {
    #[inline(always)]
    fn default() -> D1Fifosel {
        <crate::RegValueT<D1Fifosel_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod d1fifosel {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rcnt_SPEC;
    pub type Rcnt = crate::EnumBitfieldStruct<u8, Rcnt_SPEC>;
    impl Rcnt {
        #[doc = "The DTLN\\[8:0\\] bits (CFIFOCRT.DTLN\\[8:0\\], D0FIFOCRT.DTLN\\[8:0\\], D1FIFOCRT.DTLN\\[8:0\\]) are cleared when all of the receive data has been read from the DnFIFO.(In double buffer mode, the DTLN bit Value is cleared when all the data has been read from only a single plane.)"]
        pub const _0: Self = Self::new(0);

        #[doc = "The DTLN\\[8:0\\] bits are decremented each time the receive data is read from the DnFIFO. (n = 0, 1)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rew_SPEC;
    pub type Rew = crate::EnumBitfieldStruct<u8, Rew_SPEC>;
    impl Rew {
        #[doc = "The buffer pointer is not rewound."]
        pub const _0: Self = Self::new(0);

        #[doc = "The buffer pointer is rewound."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dclrm_SPEC;
    pub type Dclrm = crate::EnumBitfieldStruct<u8, Dclrm_SPEC>;
    impl Dclrm {
        #[doc = "Auto buffer clear mode is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Auto buffer clear mode is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dreqe_SPEC;
    pub type Dreqe = crate::EnumBitfieldStruct<u8, Dreqe_SPEC>;
    impl Dreqe {
        #[doc = "DMA/DTC transfer request is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "DMA/DTC transfer request is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbw_SPEC;
    pub type Mbw = crate::EnumBitfieldStruct<u8, Mbw_SPEC>;
    impl Mbw {
        #[doc = "8-bit width"]
        pub const _0: Self = Self::new(0);

        #[doc = "16-bit width"]
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
    pub struct Curpipe_SPEC;
    pub type Curpipe = crate::EnumBitfieldStruct<u8, Curpipe_SPEC>;
    impl Curpipe {
        #[doc = "DCP (Default control pipe)"]
        pub const _0000: Self = Self::new(0);

        #[doc = "Pipe 1"]
        pub const _0001: Self = Self::new(1);

        #[doc = "Pipe 2"]
        pub const _0010: Self = Self::new(2);

        #[doc = "Pipe 3"]
        pub const _0011: Self = Self::new(3);

        #[doc = "Pipe 4"]
        pub const _0100: Self = Self::new(4);

        #[doc = "Pipe 5"]
        pub const _0101: Self = Self::new(5);

        #[doc = "Pipe 6"]
        pub const _0110: Self = Self::new(6);

        #[doc = "Pipe 7"]
        pub const _0111: Self = Self::new(7);

        #[doc = "Pipe 8"]
        pub const _1000: Self = Self::new(8);

        #[doc = "Pipe 9"]
        pub const _1001: Self = Self::new(9);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct D1Fifoctr_SPEC;
impl crate::sealed::RegSpec for D1Fifoctr_SPEC {
    type DataType = u16;
}

#[doc = "D1FIFO Port Control Register"]
pub type D1Fifoctr = crate::RegValueT<D1Fifoctr_SPEC>;

impl D1Fifoctr {
    #[doc = "Buffer Memory Valid Flag"]
    #[inline(always)]
    pub fn bval(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        d1fifoctr::Bval,
        d1fifoctr::Bval,
        D1Fifoctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            d1fifoctr::Bval,
            d1fifoctr::Bval,
            D1Fifoctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CPU Buffer ClearNote: Only 0 can be read."]
    #[inline(always)]
    pub fn bclr(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        d1fifoctr::Bclr,
        d1fifoctr::Bclr,
        D1Fifoctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            d1fifoctr::Bclr,
            d1fifoctr::Bclr,
            D1Fifoctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "FIFO Port Ready"]
    #[inline(always)]
    pub fn frdy(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        d1fifoctr::Frdy,
        d1fifoctr::Frdy,
        D1Fifoctr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            d1fifoctr::Frdy,
            d1fifoctr::Frdy,
            D1Fifoctr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0xf, 1, 0, u8, u8, D1Fifoctr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0xf,1,0,u8,u8,D1Fifoctr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Receive Data LengthIndicates the length of the receive data."]
    #[inline(always)]
    pub fn dtln(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, D1Fifoctr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,D1Fifoctr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for D1Fifoctr {
    #[inline(always)]
    fn default() -> D1Fifoctr {
        <crate::RegValueT<D1Fifoctr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod d1fifoctr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bval_SPEC;
    pub type Bval = crate::EnumBitfieldStruct<u8, Bval_SPEC>;
    impl Bval {
        #[doc = "Invalid"]
        pub const _0: Self = Self::new(0);

        #[doc = "Writing ended"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bclr_SPEC;
    pub type Bclr = crate::EnumBitfieldStruct<u8, Bclr_SPEC>;
    impl Bclr {
        #[doc = "Does not operate"]
        pub const _0: Self = Self::new(0);

        #[doc = "FIFO buffer cleared on the CPU side."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Frdy_SPEC;
    pub type Frdy = crate::EnumBitfieldStruct<u8, Frdy_SPEC>;
    impl Frdy {
        #[doc = "FIFO port access is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "FIFO port access is enabled."]
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
    #[doc = "VBUS Interrupt Enable"]
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

    #[doc = "Resume Interrupt Enable"]
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

    #[doc = "Frame Number Update Interrupt Enable"]
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

    #[doc = "Device State Transition Interrupt Enable"]
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

    #[doc = "Control Transfer Stage Transition Interrupt Enable"]
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

    #[doc = "Buffer Empty Interrupt Enable"]
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

    #[doc = "Buffer Not Ready Response Interrupt Enable"]
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

    #[doc = "Buffer Ready Interrupt Enable"]
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

    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Intenb0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Intenb0_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Vbse_SPEC;
    pub type Vbse = crate::EnumBitfieldStruct<u8, Vbse_SPEC>;
    impl Vbse {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsme_SPEC;
    pub type Rsme = crate::EnumBitfieldStruct<u8, Rsme_SPEC>;
    impl Rsme {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sofe_SPEC;
    pub type Sofe = crate::EnumBitfieldStruct<u8, Sofe_SPEC>;
    impl Sofe {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvse_SPEC;
    pub type Dvse = crate::EnumBitfieldStruct<u8, Dvse_SPEC>;
    impl Dvse {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctre_SPEC;
    pub type Ctre = crate::EnumBitfieldStruct<u8, Ctre_SPEC>;
    impl Ctre {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bempe_SPEC;
    pub type Bempe = crate::EnumBitfieldStruct<u8, Bempe_SPEC>;
    impl Bempe {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nrdye_SPEC;
    pub type Nrdye = crate::EnumBitfieldStruct<u8, Nrdye_SPEC>;
    impl Nrdye {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brdye_SPEC;
    pub type Brdye = crate::EnumBitfieldStruct<u8, Brdye_SPEC>;
    impl Brdye {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
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
    #[doc = "Overcurrent Input Change Interrupt Enable"]
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

    #[doc = "USB Bus Change Interrupt Enable"]
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

    #[doc = "Disconnection Detection Interrupt Enable"]
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

    #[doc = "Connection Detection Interrupt Enable"]
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

    #[doc = "EOF Error Detection Interrupt Enable"]
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

    #[doc = "Setup Transaction Error Interrupt Enable"]
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

    #[doc = "Setup Transaction Normal Response Interrupt Enable"]
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

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7, 1, 0, u8, u8, Intenb1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7,1,0,u8,u8,Intenb1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "PDDETINT0 Detection Interrupt Enable"]
    #[inline(always)]
    pub fn pddetinte0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        intenb1::Pddetinte0,
        intenb1::Pddetinte0,
        Intenb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            intenb1::Pddetinte0,
            intenb1::Pddetinte0,
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
    pub struct Ovrcre_SPEC;
    pub type Ovrcre = crate::EnumBitfieldStruct<u8, Ovrcre_SPEC>;
    impl Ovrcre {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bchge_SPEC;
    pub type Bchge = crate::EnumBitfieldStruct<u8, Bchge_SPEC>;
    impl Bchge {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtche_SPEC;
    pub type Dtche = crate::EnumBitfieldStruct<u8, Dtche_SPEC>;
    impl Dtche {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Attche_SPEC;
    pub type Attche = crate::EnumBitfieldStruct<u8, Attche_SPEC>;
    impl Attche {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eoferre_SPEC;
    pub type Eoferre = crate::EnumBitfieldStruct<u8, Eoferre_SPEC>;
    impl Eoferre {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Signe_SPEC;
    pub type Signe = crate::EnumBitfieldStruct<u8, Signe_SPEC>;
    impl Signe {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sacke_SPEC;
    pub type Sacke = crate::EnumBitfieldStruct<u8, Sacke_SPEC>;
    impl Sacke {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pddetinte0_SPEC;
    pub type Pddetinte0 = crate::EnumBitfieldStruct<u8, Pddetinte0_SPEC>;
    impl Pddetinte0 {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
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
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<10, 0x3f, 1, 0, u8, u8, Brdyenb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3f,1,0,u8,u8,Brdyenb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "BRDY Interrupt Enable for PIPE9"]
    #[inline(always)]
    pub fn pipe9brdye(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        brdyenb::Pipe9Brdye,
        brdyenb::Pipe9Brdye,
        Brdyenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            brdyenb::Pipe9Brdye,
            brdyenb::Pipe9Brdye,
            Brdyenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BRDY Interrupt Enable for PIPE8"]
    #[inline(always)]
    pub fn pipe8brdye(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        brdyenb::Pipe8Brdye,
        brdyenb::Pipe8Brdye,
        Brdyenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            brdyenb::Pipe8Brdye,
            brdyenb::Pipe8Brdye,
            Brdyenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BRDY Interrupt Enable for PIPE7"]
    #[inline(always)]
    pub fn pipe7brdye(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        brdyenb::Pipe7Brdye,
        brdyenb::Pipe7Brdye,
        Brdyenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            brdyenb::Pipe7Brdye,
            brdyenb::Pipe7Brdye,
            Brdyenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BRDY Interrupt Enable for PIPE6"]
    #[inline(always)]
    pub fn pipe6brdye(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        brdyenb::Pipe6Brdye,
        brdyenb::Pipe6Brdye,
        Brdyenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            brdyenb::Pipe6Brdye,
            brdyenb::Pipe6Brdye,
            Brdyenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BRDY Interrupt Enable for PIPE5"]
    #[inline(always)]
    pub fn pipe5brdye(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        brdyenb::Pipe5Brdye,
        brdyenb::Pipe5Brdye,
        Brdyenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            brdyenb::Pipe5Brdye,
            brdyenb::Pipe5Brdye,
            Brdyenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BRDY Interrupt Enable for PIPE4"]
    #[inline(always)]
    pub fn pipe4brdye(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        brdyenb::Pipe4Brdye,
        brdyenb::Pipe4Brdye,
        Brdyenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            brdyenb::Pipe4Brdye,
            brdyenb::Pipe4Brdye,
            Brdyenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BRDY Interrupt Enable for PIPE3"]
    #[inline(always)]
    pub fn pipe3brdye(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        brdyenb::Pipe3Brdye,
        brdyenb::Pipe3Brdye,
        Brdyenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            brdyenb::Pipe3Brdye,
            brdyenb::Pipe3Brdye,
            Brdyenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BRDY Interrupt Enable for PIPE2"]
    #[inline(always)]
    pub fn pipe2brdye(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        brdyenb::Pipe2Brdye,
        brdyenb::Pipe2Brdye,
        Brdyenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            brdyenb::Pipe2Brdye,
            brdyenb::Pipe2Brdye,
            Brdyenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BRDY Interrupt Enable for PIPE1"]
    #[inline(always)]
    pub fn pipe1brdye(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        brdyenb::Pipe1Brdye,
        brdyenb::Pipe1Brdye,
        Brdyenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            brdyenb::Pipe1Brdye,
            brdyenb::Pipe1Brdye,
            Brdyenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BRDY Interrupt Enable for PIPE0"]
    #[inline(always)]
    pub fn pipe0brdye(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        brdyenb::Pipe0Brdye,
        brdyenb::Pipe0Brdye,
        Brdyenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            brdyenb::Pipe0Brdye,
            brdyenb::Pipe0Brdye,
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
    pub struct Pipe9Brdye_SPEC;
    pub type Pipe9Brdye = crate::EnumBitfieldStruct<u8, Pipe9Brdye_SPEC>;
    impl Pipe9Brdye {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe8Brdye_SPEC;
    pub type Pipe8Brdye = crate::EnumBitfieldStruct<u8, Pipe8Brdye_SPEC>;
    impl Pipe8Brdye {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe7Brdye_SPEC;
    pub type Pipe7Brdye = crate::EnumBitfieldStruct<u8, Pipe7Brdye_SPEC>;
    impl Pipe7Brdye {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe6Brdye_SPEC;
    pub type Pipe6Brdye = crate::EnumBitfieldStruct<u8, Pipe6Brdye_SPEC>;
    impl Pipe6Brdye {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe5Brdye_SPEC;
    pub type Pipe5Brdye = crate::EnumBitfieldStruct<u8, Pipe5Brdye_SPEC>;
    impl Pipe5Brdye {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe4Brdye_SPEC;
    pub type Pipe4Brdye = crate::EnumBitfieldStruct<u8, Pipe4Brdye_SPEC>;
    impl Pipe4Brdye {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe3Brdye_SPEC;
    pub type Pipe3Brdye = crate::EnumBitfieldStruct<u8, Pipe3Brdye_SPEC>;
    impl Pipe3Brdye {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe2Brdye_SPEC;
    pub type Pipe2Brdye = crate::EnumBitfieldStruct<u8, Pipe2Brdye_SPEC>;
    impl Pipe2Brdye {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe1Brdye_SPEC;
    pub type Pipe1Brdye = crate::EnumBitfieldStruct<u8, Pipe1Brdye_SPEC>;
    impl Pipe1Brdye {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe0Brdye_SPEC;
    pub type Pipe0Brdye = crate::EnumBitfieldStruct<u8, Pipe0Brdye_SPEC>;
    impl Pipe0Brdye {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
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
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<10, 0x3f, 1, 0, u8, u8, Nrdyenb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3f,1,0,u8,u8,Nrdyenb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "NRDY Interrupt Enable for PIPE9"]
    #[inline(always)]
    pub fn pipe9nrdye(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        nrdyenb::Pipe9Nrdye,
        nrdyenb::Pipe9Nrdye,
        Nrdyenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            nrdyenb::Pipe9Nrdye,
            nrdyenb::Pipe9Nrdye,
            Nrdyenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NRDY Interrupt Enable for PIPE8"]
    #[inline(always)]
    pub fn pipe8nrdye(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        nrdyenb::Pipe8Nrdye,
        nrdyenb::Pipe8Nrdye,
        Nrdyenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            nrdyenb::Pipe8Nrdye,
            nrdyenb::Pipe8Nrdye,
            Nrdyenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NRDY Interrupt Enable for PIPE7"]
    #[inline(always)]
    pub fn pipe7nrdye(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        nrdyenb::Pipe7Nrdye,
        nrdyenb::Pipe7Nrdye,
        Nrdyenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            nrdyenb::Pipe7Nrdye,
            nrdyenb::Pipe7Nrdye,
            Nrdyenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NRDY Interrupt Enable for PIPE6"]
    #[inline(always)]
    pub fn pipe6nrdye(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        nrdyenb::Pipe6Nrdye,
        nrdyenb::Pipe6Nrdye,
        Nrdyenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            nrdyenb::Pipe6Nrdye,
            nrdyenb::Pipe6Nrdye,
            Nrdyenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NRDY Interrupt Enable for PIPE5"]
    #[inline(always)]
    pub fn pipe5nrdye(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        nrdyenb::Pipe5Nrdye,
        nrdyenb::Pipe5Nrdye,
        Nrdyenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            nrdyenb::Pipe5Nrdye,
            nrdyenb::Pipe5Nrdye,
            Nrdyenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NRDY Interrupt Enable for PIPE4"]
    #[inline(always)]
    pub fn pipe4nrdye(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        nrdyenb::Pipe4Nrdye,
        nrdyenb::Pipe4Nrdye,
        Nrdyenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            nrdyenb::Pipe4Nrdye,
            nrdyenb::Pipe4Nrdye,
            Nrdyenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NRDY Interrupt Enable for PIPE3"]
    #[inline(always)]
    pub fn pipe3nrdye(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        nrdyenb::Pipe3Nrdye,
        nrdyenb::Pipe3Nrdye,
        Nrdyenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            nrdyenb::Pipe3Nrdye,
            nrdyenb::Pipe3Nrdye,
            Nrdyenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NRDY Interrupt Enable for PIPE2"]
    #[inline(always)]
    pub fn pipe2nrdye(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        nrdyenb::Pipe2Nrdye,
        nrdyenb::Pipe2Nrdye,
        Nrdyenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            nrdyenb::Pipe2Nrdye,
            nrdyenb::Pipe2Nrdye,
            Nrdyenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NRDY Interrupt Enable for PIPE1"]
    #[inline(always)]
    pub fn pipe1nrdye(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        nrdyenb::Pipe1Nrdye,
        nrdyenb::Pipe1Nrdye,
        Nrdyenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            nrdyenb::Pipe1Nrdye,
            nrdyenb::Pipe1Nrdye,
            Nrdyenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NRDY Interrupt Enable for PIPE0"]
    #[inline(always)]
    pub fn pipe0nrdye(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        nrdyenb::Pipe0Nrdye,
        nrdyenb::Pipe0Nrdye,
        Nrdyenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            nrdyenb::Pipe0Nrdye,
            nrdyenb::Pipe0Nrdye,
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
    pub struct Pipe9Nrdye_SPEC;
    pub type Pipe9Nrdye = crate::EnumBitfieldStruct<u8, Pipe9Nrdye_SPEC>;
    impl Pipe9Nrdye {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe8Nrdye_SPEC;
    pub type Pipe8Nrdye = crate::EnumBitfieldStruct<u8, Pipe8Nrdye_SPEC>;
    impl Pipe8Nrdye {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe7Nrdye_SPEC;
    pub type Pipe7Nrdye = crate::EnumBitfieldStruct<u8, Pipe7Nrdye_SPEC>;
    impl Pipe7Nrdye {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe6Nrdye_SPEC;
    pub type Pipe6Nrdye = crate::EnumBitfieldStruct<u8, Pipe6Nrdye_SPEC>;
    impl Pipe6Nrdye {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe5Nrdye_SPEC;
    pub type Pipe5Nrdye = crate::EnumBitfieldStruct<u8, Pipe5Nrdye_SPEC>;
    impl Pipe5Nrdye {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe4Nrdye_SPEC;
    pub type Pipe4Nrdye = crate::EnumBitfieldStruct<u8, Pipe4Nrdye_SPEC>;
    impl Pipe4Nrdye {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe3Nrdye_SPEC;
    pub type Pipe3Nrdye = crate::EnumBitfieldStruct<u8, Pipe3Nrdye_SPEC>;
    impl Pipe3Nrdye {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe2Nrdye_SPEC;
    pub type Pipe2Nrdye = crate::EnumBitfieldStruct<u8, Pipe2Nrdye_SPEC>;
    impl Pipe2Nrdye {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe1Nrdye_SPEC;
    pub type Pipe1Nrdye = crate::EnumBitfieldStruct<u8, Pipe1Nrdye_SPEC>;
    impl Pipe1Nrdye {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe0Nrdye_SPEC;
    pub type Pipe0Nrdye = crate::EnumBitfieldStruct<u8, Pipe0Nrdye_SPEC>;
    impl Pipe0Nrdye {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
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
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<10, 0x3f, 1, 0, u8, u8, Bempenb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3f,1,0,u8,u8,Bempenb_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "BEMP Interrupt Enable for PIPE9"]
    #[inline(always)]
    pub fn pipe9bempe(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        bempenb::Pipe9Bempe,
        bempenb::Pipe9Bempe,
        Bempenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            bempenb::Pipe9Bempe,
            bempenb::Pipe9Bempe,
            Bempenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BEMP Interrupt Enable for PIPE8"]
    #[inline(always)]
    pub fn pipe8bempe(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        bempenb::Pipe8Bempe,
        bempenb::Pipe8Bempe,
        Bempenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            bempenb::Pipe8Bempe,
            bempenb::Pipe8Bempe,
            Bempenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BEMP Interrupt Enable for PIPE7"]
    #[inline(always)]
    pub fn pipe7bempe(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        bempenb::Pipe7Bempe,
        bempenb::Pipe7Bempe,
        Bempenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            bempenb::Pipe7Bempe,
            bempenb::Pipe7Bempe,
            Bempenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BEMP Interrupt Enable for PIPE6"]
    #[inline(always)]
    pub fn pipe6bempe(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        bempenb::Pipe6Bempe,
        bempenb::Pipe6Bempe,
        Bempenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            bempenb::Pipe6Bempe,
            bempenb::Pipe6Bempe,
            Bempenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BEMP Interrupt Enable for PIPE5"]
    #[inline(always)]
    pub fn pipe5bempe(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        bempenb::Pipe5Bempe,
        bempenb::Pipe5Bempe,
        Bempenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            bempenb::Pipe5Bempe,
            bempenb::Pipe5Bempe,
            Bempenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BEMP Interrupt Enable for PIPE4"]
    #[inline(always)]
    pub fn pipe4bempe(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        bempenb::Pipe4Bempe,
        bempenb::Pipe4Bempe,
        Bempenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            bempenb::Pipe4Bempe,
            bempenb::Pipe4Bempe,
            Bempenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BEMP Interrupt Enable for PIPE3"]
    #[inline(always)]
    pub fn pipe3bempe(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        bempenb::Pipe3Bempe,
        bempenb::Pipe3Bempe,
        Bempenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            bempenb::Pipe3Bempe,
            bempenb::Pipe3Bempe,
            Bempenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BEMP Interrupt Enable for PIPE2"]
    #[inline(always)]
    pub fn pipe2bempe(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        bempenb::Pipe2Bempe,
        bempenb::Pipe2Bempe,
        Bempenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            bempenb::Pipe2Bempe,
            bempenb::Pipe2Bempe,
            Bempenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BEMP Interrupt Enable for PIPE1"]
    #[inline(always)]
    pub fn pipe1bempe(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        bempenb::Pipe1Bempe,
        bempenb::Pipe1Bempe,
        Bempenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            bempenb::Pipe1Bempe,
            bempenb::Pipe1Bempe,
            Bempenb_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BEMP Interrupt Enable for PIPE0"]
    #[inline(always)]
    pub fn pipe0bempe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bempenb::Pipe0Bempe,
        bempenb::Pipe0Bempe,
        Bempenb_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bempenb::Pipe0Bempe,
            bempenb::Pipe0Bempe,
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
    pub struct Pipe9Bempe_SPEC;
    pub type Pipe9Bempe = crate::EnumBitfieldStruct<u8, Pipe9Bempe_SPEC>;
    impl Pipe9Bempe {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe8Bempe_SPEC;
    pub type Pipe8Bempe = crate::EnumBitfieldStruct<u8, Pipe8Bempe_SPEC>;
    impl Pipe8Bempe {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe7Bempe_SPEC;
    pub type Pipe7Bempe = crate::EnumBitfieldStruct<u8, Pipe7Bempe_SPEC>;
    impl Pipe7Bempe {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe6Bempe_SPEC;
    pub type Pipe6Bempe = crate::EnumBitfieldStruct<u8, Pipe6Bempe_SPEC>;
    impl Pipe6Bempe {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe5Bempe_SPEC;
    pub type Pipe5Bempe = crate::EnumBitfieldStruct<u8, Pipe5Bempe_SPEC>;
    impl Pipe5Bempe {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe4Bempe_SPEC;
    pub type Pipe4Bempe = crate::EnumBitfieldStruct<u8, Pipe4Bempe_SPEC>;
    impl Pipe4Bempe {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe3Bempe_SPEC;
    pub type Pipe3Bempe = crate::EnumBitfieldStruct<u8, Pipe3Bempe_SPEC>;
    impl Pipe3Bempe {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe2Bempe_SPEC;
    pub type Pipe2Bempe = crate::EnumBitfieldStruct<u8, Pipe2Bempe_SPEC>;
    impl Pipe2Bempe {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe1Bempe_SPEC;
    pub type Pipe1Bempe = crate::EnumBitfieldStruct<u8, Pipe1Bempe_SPEC>;
    impl Pipe1Bempe {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe0Bempe_SPEC;
    pub type Pipe0Bempe = crate::EnumBitfieldStruct<u8, Pipe0Bempe_SPEC>;
    impl Pipe0Bempe {
        #[doc = "Interrupt output disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupt output enabled"]
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

    #[doc = "BRDY Interrupt Status Clear Timing"]
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

    #[doc = "Edge Interrupt Output Status Monitor"]
    #[inline(always)]
    pub fn edgests(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        sofcfg::Edgests,
        sofcfg::Edgests,
        Sofcfg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            sofcfg::Edgests,
            sofcfg::Edgests,
            Sofcfg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Sofcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Sofcfg_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Trnensel_SPEC;
    pub type Trnensel = crate::EnumBitfieldStruct<u8, Trnensel_SPEC>;
    impl Trnensel {
        #[doc = "Not low-speed communication"]
        pub const _0: Self = Self::new(0);

        #[doc = "Low-speed communication."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brdym_SPEC;
    pub type Brdym = crate::EnumBitfieldStruct<u8, Brdym_SPEC>;
    impl Brdym {
        #[doc = "BRDY flag cleared by software"]
        pub const _0: Self = Self::new(0);

        #[doc = "BRDY flag cleared by the USBFS through a data read from the FIFO buffer or data write to the FIFO buffer."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Edgests_SPEC;
    pub type Edgests = crate::EnumBitfieldStruct<u8, Edgests_SPEC>;
    impl Edgests {
        #[doc = "before stopping the clock supply to the USB module"]
        pub const _0: Self = Self::new(0);

        #[doc = "the edge interrupt output signal is in the middle of the edge processing"]
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
    #[doc = "VBUS Interrupt Status"]
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

    #[doc = "Resume Interrupt Status"]
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

    #[doc = "Frame Number Refresh Interrupt Status"]
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

    #[doc = "Device State Transition Interrupt Status"]
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

    #[doc = "Control Transfer Stage Transition Interrupt Status"]
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

    #[doc = "Buffer Empty Interrupt Status"]
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

    #[doc = "Buffer Not Ready Interrupt Status"]
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

    #[doc = "Buffer Ready Interrupt Status"]
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

    #[doc = "VBUS Input Status"]
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

    #[doc = "USB Request Reception"]
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

    #[doc = "Control Transfer Stage"]
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
}
impl ::core::default::Default for Intsts0 {
    #[inline(always)]
    fn default() -> Intsts0 {
        <crate::RegValueT<Intsts0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod intsts0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbint_SPEC;
    pub type Vbint = crate::EnumBitfieldStruct<u8, Vbint_SPEC>;
    impl Vbint {
        #[doc = "VBUS interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "VBUS interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Resm_SPEC;
    pub type Resm = crate::EnumBitfieldStruct<u8, Resm_SPEC>;
    impl Resm {
        #[doc = "Resume interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Resume interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sofr_SPEC;
    pub type Sofr = crate::EnumBitfieldStruct<u8, Sofr_SPEC>;
    impl Sofr {
        #[doc = "SOF interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "SOF interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dvst_SPEC;
    pub type Dvst = crate::EnumBitfieldStruct<u8, Dvst_SPEC>;
    impl Dvst {
        #[doc = "Device state transition interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Device state transition interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctrt_SPEC;
    pub type Ctrt = crate::EnumBitfieldStruct<u8, Ctrt_SPEC>;
    impl Ctrt {
        #[doc = "Control transfer stage transition interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Control transfer stage transition interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bemp_SPEC;
    pub type Bemp = crate::EnumBitfieldStruct<u8, Bemp_SPEC>;
    impl Bemp {
        #[doc = "BEMP interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "BEMP interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nrdy_SPEC;
    pub type Nrdy = crate::EnumBitfieldStruct<u8, Nrdy_SPEC>;
    impl Nrdy {
        #[doc = "NRDY interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "NRDY interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brdy_SPEC;
    pub type Brdy = crate::EnumBitfieldStruct<u8, Brdy_SPEC>;
    impl Brdy {
        #[doc = "BRDY interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "BRDY interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbsts_SPEC;
    pub type Vbsts = crate::EnumBitfieldStruct<u8, Vbsts_SPEC>;
    impl Vbsts {
        #[doc = "USB_VBUS pin is low."]
        pub const _0: Self = Self::new(0);

        #[doc = "USB_VBUS pin is high."]
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
    pub struct Valid_SPEC;
    pub type Valid = crate::EnumBitfieldStruct<u8, Valid_SPEC>;
    impl Valid {
        #[doc = "Setup packet is not received"]
        pub const _0: Self = Self::new(0);

        #[doc = "Setup packet is received"]
        pub const _1: Self = Self::new(1);
    }
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
    #[doc = "Overcurrent Input Change Interrupt Status"]
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

    #[doc = "USB Bus Change Interrupt Status"]
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

    #[doc = "USB Disconnection Detection Interrupt Status"]
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

    #[doc = "ATTCH Interrupt Status"]
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

    #[doc = "EOF Error Detection Interrupt Status"]
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

    #[doc = "Setup Transaction Error Interrupt Status"]
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

    #[doc = "Setup Transaction Normal Response Interrupt Status"]
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

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7, 1, 0, u8, u8, Intsts1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7,1,0,u8,u8,Intsts1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "PDDET0 Detection Interrupt Status"]
    #[inline(always)]
    pub fn pddetint0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        intsts1::Pddetint0,
        intsts1::Pddetint0,
        Intsts1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            intsts1::Pddetint0,
            intsts1::Pddetint0,
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
    pub struct Ovrcr_SPEC;
    pub type Ovrcr = crate::EnumBitfieldStruct<u8, Ovrcr_SPEC>;
    impl Ovrcr {
        #[doc = "OVRCR interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "OVRCR interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bchg_SPEC;
    pub type Bchg = crate::EnumBitfieldStruct<u8, Bchg_SPEC>;
    impl Bchg {
        #[doc = "BCHG interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "BCHG interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtch_SPEC;
    pub type Dtch = crate::EnumBitfieldStruct<u8, Dtch_SPEC>;
    impl Dtch {
        #[doc = "DTCH interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "DTCH interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Attch_SPEC;
    pub type Attch = crate::EnumBitfieldStruct<u8, Attch_SPEC>;
    impl Attch {
        #[doc = "ATTCH interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "ATTCH interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eoferr_SPEC;
    pub type Eoferr = crate::EnumBitfieldStruct<u8, Eoferr_SPEC>;
    impl Eoferr {
        #[doc = "EOFERR interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "EOFERR interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sign_SPEC;
    pub type Sign = crate::EnumBitfieldStruct<u8, Sign_SPEC>;
    impl Sign {
        #[doc = "SIGN interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "SIGN interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sack_SPEC;
    pub type Sack = crate::EnumBitfieldStruct<u8, Sack_SPEC>;
    impl Sack {
        #[doc = "SACK interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "SACK interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pddetint0_SPEC;
    pub type Pddetint0 = crate::EnumBitfieldStruct<u8, Pddetint0_SPEC>;
    impl Pddetint0 {
        #[doc = "PDDET0 detection interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "PDDET0 detection interrupts are generated."]
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
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<10, 0x3f, 1, 0, u8, u8, Brdysts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3f,1,0,u8,u8,Brdysts_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "BRDY Interrupt Status for PIPE9"]
    #[inline(always)]
    pub fn pipe9brdy(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        brdysts::Pipe9Brdy,
        brdysts::Pipe9Brdy,
        Brdysts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            brdysts::Pipe9Brdy,
            brdysts::Pipe9Brdy,
            Brdysts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BRDY Interrupt Status for PIPE8"]
    #[inline(always)]
    pub fn pipe8brdy(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        brdysts::Pipe8Brdy,
        brdysts::Pipe8Brdy,
        Brdysts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            brdysts::Pipe8Brdy,
            brdysts::Pipe8Brdy,
            Brdysts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BRDY Interrupt Status for PIPE7"]
    #[inline(always)]
    pub fn pipe7brdy(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        brdysts::Pipe7Brdy,
        brdysts::Pipe7Brdy,
        Brdysts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            brdysts::Pipe7Brdy,
            brdysts::Pipe7Brdy,
            Brdysts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BRDY Interrupt Status for PIPE6"]
    #[inline(always)]
    pub fn pipe6brdy(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        brdysts::Pipe6Brdy,
        brdysts::Pipe6Brdy,
        Brdysts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            brdysts::Pipe6Brdy,
            brdysts::Pipe6Brdy,
            Brdysts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BRDY Interrupt Status for PIPE5"]
    #[inline(always)]
    pub fn pipe5brdy(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        brdysts::Pipe5Brdy,
        brdysts::Pipe5Brdy,
        Brdysts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            brdysts::Pipe5Brdy,
            brdysts::Pipe5Brdy,
            Brdysts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BRDY Interrupt Status for PIPE4"]
    #[inline(always)]
    pub fn pipe4brdy(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        brdysts::Pipe4Brdy,
        brdysts::Pipe4Brdy,
        Brdysts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            brdysts::Pipe4Brdy,
            brdysts::Pipe4Brdy,
            Brdysts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BRDY Interrupt Status for PIPE3"]
    #[inline(always)]
    pub fn pipe3brdy(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        brdysts::Pipe3Brdy,
        brdysts::Pipe3Brdy,
        Brdysts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            brdysts::Pipe3Brdy,
            brdysts::Pipe3Brdy,
            Brdysts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BRDY Interrupt Status for PIPE2"]
    #[inline(always)]
    pub fn pipe2brdy(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        brdysts::Pipe2Brdy,
        brdysts::Pipe2Brdy,
        Brdysts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            brdysts::Pipe2Brdy,
            brdysts::Pipe2Brdy,
            Brdysts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BRDY Interrupt Status for PIPE1"]
    #[inline(always)]
    pub fn pipe1brdy(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        brdysts::Pipe1Brdy,
        brdysts::Pipe1Brdy,
        Brdysts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            brdysts::Pipe1Brdy,
            brdysts::Pipe1Brdy,
            Brdysts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BRDY Interrupt Status for PIPE0"]
    #[inline(always)]
    pub fn pipe0brdy(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        brdysts::Pipe0Brdy,
        brdysts::Pipe0Brdy,
        Brdysts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            brdysts::Pipe0Brdy,
            brdysts::Pipe0Brdy,
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
    pub struct Pipe9Brdy_SPEC;
    pub type Pipe9Brdy = crate::EnumBitfieldStruct<u8, Pipe9Brdy_SPEC>;
    impl Pipe9Brdy {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe8Brdy_SPEC;
    pub type Pipe8Brdy = crate::EnumBitfieldStruct<u8, Pipe8Brdy_SPEC>;
    impl Pipe8Brdy {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe7Brdy_SPEC;
    pub type Pipe7Brdy = crate::EnumBitfieldStruct<u8, Pipe7Brdy_SPEC>;
    impl Pipe7Brdy {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe6Brdy_SPEC;
    pub type Pipe6Brdy = crate::EnumBitfieldStruct<u8, Pipe6Brdy_SPEC>;
    impl Pipe6Brdy {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe5Brdy_SPEC;
    pub type Pipe5Brdy = crate::EnumBitfieldStruct<u8, Pipe5Brdy_SPEC>;
    impl Pipe5Brdy {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe4Brdy_SPEC;
    pub type Pipe4Brdy = crate::EnumBitfieldStruct<u8, Pipe4Brdy_SPEC>;
    impl Pipe4Brdy {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe3Brdy_SPEC;
    pub type Pipe3Brdy = crate::EnumBitfieldStruct<u8, Pipe3Brdy_SPEC>;
    impl Pipe3Brdy {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe2Brdy_SPEC;
    pub type Pipe2Brdy = crate::EnumBitfieldStruct<u8, Pipe2Brdy_SPEC>;
    impl Pipe2Brdy {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe1Brdy_SPEC;
    pub type Pipe1Brdy = crate::EnumBitfieldStruct<u8, Pipe1Brdy_SPEC>;
    impl Pipe1Brdy {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe0Brdy_SPEC;
    pub type Pipe0Brdy = crate::EnumBitfieldStruct<u8, Pipe0Brdy_SPEC>;
    impl Pipe0Brdy {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
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
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<10, 0x3f, 1, 0, u8, u8, Nrdysts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3f,1,0,u8,u8,Nrdysts_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "NRDY Interrupt Status for PIPE9"]
    #[inline(always)]
    pub fn pipe9nrdy(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        nrdysts::Pipe9Nrdy,
        nrdysts::Pipe9Nrdy,
        Nrdysts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            nrdysts::Pipe9Nrdy,
            nrdysts::Pipe9Nrdy,
            Nrdysts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NRDY Interrupt Status for PIPE8"]
    #[inline(always)]
    pub fn pipe8nrdy(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        nrdysts::Pipe8Nrdy,
        nrdysts::Pipe8Nrdy,
        Nrdysts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            nrdysts::Pipe8Nrdy,
            nrdysts::Pipe8Nrdy,
            Nrdysts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NRDY Interrupt Status for PIPE7"]
    #[inline(always)]
    pub fn pipe7nrdy(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        nrdysts::Pipe7Nrdy,
        nrdysts::Pipe7Nrdy,
        Nrdysts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            nrdysts::Pipe7Nrdy,
            nrdysts::Pipe7Nrdy,
            Nrdysts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NRDY Interrupt Status for PIPE6"]
    #[inline(always)]
    pub fn pipe6nrdy(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        nrdysts::Pipe6Nrdy,
        nrdysts::Pipe6Nrdy,
        Nrdysts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            nrdysts::Pipe6Nrdy,
            nrdysts::Pipe6Nrdy,
            Nrdysts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NRDY Interrupt Status for PIPE5"]
    #[inline(always)]
    pub fn pipe5nrdy(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        nrdysts::Pipe5Nrdy,
        nrdysts::Pipe5Nrdy,
        Nrdysts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            nrdysts::Pipe5Nrdy,
            nrdysts::Pipe5Nrdy,
            Nrdysts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NRDY Interrupt Status for PIPE4"]
    #[inline(always)]
    pub fn pipe4nrdy(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        nrdysts::Pipe4Nrdy,
        nrdysts::Pipe4Nrdy,
        Nrdysts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            nrdysts::Pipe4Nrdy,
            nrdysts::Pipe4Nrdy,
            Nrdysts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NRDY Interrupt Status for PIPE3"]
    #[inline(always)]
    pub fn pipe3nrdy(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        nrdysts::Pipe3Nrdy,
        nrdysts::Pipe3Nrdy,
        Nrdysts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            nrdysts::Pipe3Nrdy,
            nrdysts::Pipe3Nrdy,
            Nrdysts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NRDY Interrupt Status for PIPE2"]
    #[inline(always)]
    pub fn pipe2nrdy(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        nrdysts::Pipe2Nrdy,
        nrdysts::Pipe2Nrdy,
        Nrdysts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            nrdysts::Pipe2Nrdy,
            nrdysts::Pipe2Nrdy,
            Nrdysts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NRDY Interrupt Status for PIPE1"]
    #[inline(always)]
    pub fn pipe1nrdy(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        nrdysts::Pipe1Nrdy,
        nrdysts::Pipe1Nrdy,
        Nrdysts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            nrdysts::Pipe1Nrdy,
            nrdysts::Pipe1Nrdy,
            Nrdysts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "NRDY Interrupt Status for PIPE0"]
    #[inline(always)]
    pub fn pipe0nrdy(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        nrdysts::Pipe0Nrdy,
        nrdysts::Pipe0Nrdy,
        Nrdysts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            nrdysts::Pipe0Nrdy,
            nrdysts::Pipe0Nrdy,
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
    pub struct Pipe9Nrdy_SPEC;
    pub type Pipe9Nrdy = crate::EnumBitfieldStruct<u8, Pipe9Nrdy_SPEC>;
    impl Pipe9Nrdy {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe8Nrdy_SPEC;
    pub type Pipe8Nrdy = crate::EnumBitfieldStruct<u8, Pipe8Nrdy_SPEC>;
    impl Pipe8Nrdy {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe7Nrdy_SPEC;
    pub type Pipe7Nrdy = crate::EnumBitfieldStruct<u8, Pipe7Nrdy_SPEC>;
    impl Pipe7Nrdy {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe6Nrdy_SPEC;
    pub type Pipe6Nrdy = crate::EnumBitfieldStruct<u8, Pipe6Nrdy_SPEC>;
    impl Pipe6Nrdy {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe5Nrdy_SPEC;
    pub type Pipe5Nrdy = crate::EnumBitfieldStruct<u8, Pipe5Nrdy_SPEC>;
    impl Pipe5Nrdy {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe4Nrdy_SPEC;
    pub type Pipe4Nrdy = crate::EnumBitfieldStruct<u8, Pipe4Nrdy_SPEC>;
    impl Pipe4Nrdy {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe3Nrdy_SPEC;
    pub type Pipe3Nrdy = crate::EnumBitfieldStruct<u8, Pipe3Nrdy_SPEC>;
    impl Pipe3Nrdy {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe2Nrdy_SPEC;
    pub type Pipe2Nrdy = crate::EnumBitfieldStruct<u8, Pipe2Nrdy_SPEC>;
    impl Pipe2Nrdy {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe1Nrdy_SPEC;
    pub type Pipe1Nrdy = crate::EnumBitfieldStruct<u8, Pipe1Nrdy_SPEC>;
    impl Pipe1Nrdy {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe0Nrdy_SPEC;
    pub type Pipe0Nrdy = crate::EnumBitfieldStruct<u8, Pipe0Nrdy_SPEC>;
    impl Pipe0Nrdy {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
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
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<10, 0x3f, 1, 0, u8, u8, Bempsts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3f,1,0,u8,u8,Bempsts_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "BEMP Interrupt Status for PIPE9"]
    #[inline(always)]
    pub fn pipe9bemp(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        bempsts::Pipe9Bemp,
        bempsts::Pipe9Bemp,
        Bempsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            bempsts::Pipe9Bemp,
            bempsts::Pipe9Bemp,
            Bempsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BEMP Interrupt Status for PIPE8"]
    #[inline(always)]
    pub fn pipe8bemp(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        bempsts::Pipe8Bemp,
        bempsts::Pipe8Bemp,
        Bempsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            bempsts::Pipe8Bemp,
            bempsts::Pipe8Bemp,
            Bempsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BEMP Interrupt Status for PIPE7"]
    #[inline(always)]
    pub fn pipe7bemp(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        bempsts::Pipe7Bemp,
        bempsts::Pipe7Bemp,
        Bempsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            bempsts::Pipe7Bemp,
            bempsts::Pipe7Bemp,
            Bempsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BEMP Interrupt Status for PIPE6"]
    #[inline(always)]
    pub fn pipe6bemp(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        bempsts::Pipe6Bemp,
        bempsts::Pipe6Bemp,
        Bempsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            bempsts::Pipe6Bemp,
            bempsts::Pipe6Bemp,
            Bempsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BEMP Interrupt Status for PIPE5"]
    #[inline(always)]
    pub fn pipe5bemp(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        bempsts::Pipe5Bemp,
        bempsts::Pipe5Bemp,
        Bempsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            bempsts::Pipe5Bemp,
            bempsts::Pipe5Bemp,
            Bempsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BEMP Interrupt Status for PIPE4"]
    #[inline(always)]
    pub fn pipe4bemp(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        bempsts::Pipe4Bemp,
        bempsts::Pipe4Bemp,
        Bempsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            bempsts::Pipe4Bemp,
            bempsts::Pipe4Bemp,
            Bempsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BEMP Interrupt Status for PIPE3"]
    #[inline(always)]
    pub fn pipe3bemp(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        bempsts::Pipe3Bemp,
        bempsts::Pipe3Bemp,
        Bempsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            bempsts::Pipe3Bemp,
            bempsts::Pipe3Bemp,
            Bempsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BEMP Interrupt Status for PIPE2"]
    #[inline(always)]
    pub fn pipe2bemp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        bempsts::Pipe2Bemp,
        bempsts::Pipe2Bemp,
        Bempsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            bempsts::Pipe2Bemp,
            bempsts::Pipe2Bemp,
            Bempsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BEMP Interrupt Status for PIPE1"]
    #[inline(always)]
    pub fn pipe1bemp(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        bempsts::Pipe1Bemp,
        bempsts::Pipe1Bemp,
        Bempsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            bempsts::Pipe1Bemp,
            bempsts::Pipe1Bemp,
            Bempsts_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "BEMP Interrupt Status for PIPE0"]
    #[inline(always)]
    pub fn pipe0bemp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bempsts::Pipe0Bemp,
        bempsts::Pipe0Bemp,
        Bempsts_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bempsts::Pipe0Bemp,
            bempsts::Pipe0Bemp,
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
    pub struct Pipe9Bemp_SPEC;
    pub type Pipe9Bemp = crate::EnumBitfieldStruct<u8, Pipe9Bemp_SPEC>;
    impl Pipe9Bemp {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe8Bemp_SPEC;
    pub type Pipe8Bemp = crate::EnumBitfieldStruct<u8, Pipe8Bemp_SPEC>;
    impl Pipe8Bemp {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe7Bemp_SPEC;
    pub type Pipe7Bemp = crate::EnumBitfieldStruct<u8, Pipe7Bemp_SPEC>;
    impl Pipe7Bemp {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe6Bemp_SPEC;
    pub type Pipe6Bemp = crate::EnumBitfieldStruct<u8, Pipe6Bemp_SPEC>;
    impl Pipe6Bemp {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe5Bemp_SPEC;
    pub type Pipe5Bemp = crate::EnumBitfieldStruct<u8, Pipe5Bemp_SPEC>;
    impl Pipe5Bemp {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe4Bemp_SPEC;
    pub type Pipe4Bemp = crate::EnumBitfieldStruct<u8, Pipe4Bemp_SPEC>;
    impl Pipe4Bemp {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe3Bemp_SPEC;
    pub type Pipe3Bemp = crate::EnumBitfieldStruct<u8, Pipe3Bemp_SPEC>;
    impl Pipe3Bemp {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe2Bemp_SPEC;
    pub type Pipe2Bemp = crate::EnumBitfieldStruct<u8, Pipe2Bemp_SPEC>;
    impl Pipe2Bemp {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe1Bemp_SPEC;
    pub type Pipe1Bemp = crate::EnumBitfieldStruct<u8, Pipe1Bemp_SPEC>;
    impl Pipe1Bemp {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pipe0Bemp_SPEC;
    pub type Pipe0Bemp = crate::EnumBitfieldStruct<u8, Pipe0Bemp_SPEC>;
    impl Pipe0Bemp {
        #[doc = "Interrupts are not generated."]
        pub const _0: Self = Self::new(0);

        #[doc = "Interrupts are generated."]
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
    #[doc = "Overrun/Underrun Detection Status"]
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

    #[doc = "Receive Data Error"]
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

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, u8, Frmnum_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x7,1,0,u8,u8,Frmnum_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Frame NumberLatest frame number"]
    #[inline(always)]
    pub fn frnm(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, u16, Frmnum_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16,u16,Frmnum_SPEC,crate::common::R>::from_register(self,0)
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
    pub struct Ovrn_SPEC;
    pub type Ovrn = crate::EnumBitfieldStruct<u8, Ovrn_SPEC>;
    impl Ovrn {
        #[doc = "No error"]
        pub const _0: Self = Self::new(0);

        #[doc = "An error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crce_SPEC;
    pub type Crce = crate::EnumBitfieldStruct<u8, Crce_SPEC>;
    impl Crce {
        #[doc = "No error"]
        pub const _0: Self = Self::new(0);

        #[doc = "An error occurred"]
        pub const _1: Self = Self::new(1);
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
    #[doc = "RequestThese bits store the USB request bRequest value."]
    #[inline(always)]
    pub fn brequest(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Usbreq_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Usbreq_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Request TypeThese bits store the USB request bmRequestType value."]
    #[inline(always)]
    pub fn bmrequesttype(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Usbreq_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Usbreq_SPEC,crate::common::RW>::from_register(self,0)
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
    #[doc = "ValueThese bits store the USB request Value value."]
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
    #[doc = "IndexThese bits store the USB request wIndex value."]
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
    #[doc = "LengthThese bits store the USB request wLength value."]
    #[inline(always)]
    pub fn wlength(
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
    #[doc = "Pipe Disabled at End of Transfer"]
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

    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Dcpcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Dcpcfg_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Shtnak_SPEC;
    pub type Shtnak = crate::EnumBitfieldStruct<u8, Shtnak_SPEC>;
    impl Shtnak {
        #[doc = "Pipe continued at the end of transfer"]
        pub const _0: Self = Self::new(0);

        #[doc = "Pipe disabled at the end of transfer"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dir_SPEC;
    pub type Dir = crate::EnumBitfieldStruct<u8, Dir_SPEC>;
    impl Dir {
        #[doc = "Data receiving direction"]
        pub const _0: Self = Self::new(0);

        #[doc = "Data transmitting direction"]
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

    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<7, 0x1f, 1, 0, u8, u8, Dcpmaxp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1f,1,0,u8,u8,Dcpmaxp_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Maximum Packet SizeThese bits set the maximum amount of data (maximum packet size) in payloads for the DCP."]
    #[inline(always)]
    pub fn mxps(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7f,
        1,
        0,
        dcpmaxp::Mxps,
        dcpmaxp::Mxps,
        Dcpmaxp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7f,
            1,
            0,
            dcpmaxp::Mxps,
            dcpmaxp::Mxps,
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
        #[doc = "Address 0000"]
        pub const _0000: Self = Self::new(0);

        #[doc = "Address 0001"]
        pub const _0001: Self = Self::new(1);

        #[doc = "Address 0010"]
        pub const _0010: Self = Self::new(2);

        #[doc = "Address 0011"]
        pub const _0011: Self = Self::new(3);

        #[doc = "Address 0100"]
        pub const _0100: Self = Self::new(4);

        #[doc = "Address 0101"]
        pub const _0101: Self = Self::new(5);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mxps_SPEC;
    pub type Mxps = crate::EnumBitfieldStruct<u8, Mxps_SPEC>;
    impl Mxps {
        #[doc = "8 bytes"]
        pub const _0_X_08: Self = Self::new(8);

        #[doc = "16 bytes"]
        pub const _0_X_10: Self = Self::new(16);

        #[doc = "24 bytes"]
        pub const _0_X_18: Self = Self::new(24);

        #[doc = "32 bytes"]
        pub const _0_X_20: Self = Self::new(32);

        #[doc = "40 bytes"]
        pub const _0_X_28: Self = Self::new(40);

        #[doc = "48 bytes"]
        pub const _0_X_30: Self = Self::new(48);

        #[doc = "56 bytes"]
        pub const _0_X_38: Self = Self::new(56);

        #[doc = "64 bytes"]
        pub const _0_X_40: Self = Self::new(64);

        #[doc = "72 bytes"]
        pub const _0_X_48: Self = Self::new(72);

        #[doc = "80 bytes"]
        pub const _0_X_50: Self = Self::new(80);

        #[doc = "88 bytes"]
        pub const _0_X_58: Self = Self::new(88);

        #[doc = "96 bytes"]
        pub const _0_X_60: Self = Self::new(96);

        #[doc = "104 bytes"]
        pub const _0_X_68: Self = Self::new(104);

        #[doc = "112 bytes"]
        pub const _0_X_70: Self = Self::new(112);

        #[doc = "120 bytes"]
        pub const _0_X_78: Self = Self::new(120);
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
    #[doc = "Buffer Status"]
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

    #[doc = "Setup Token Transmission"]
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
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            dcpctr::Sureqclr,
            dcpctr::Sureqclr,
            Dcpctr_SPEC,
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
        dcpctr::Sqclr,
        dcpctr::Sqclr,
        Dcpctr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            dcpctr::Sqclr,
            dcpctr::Sqclr,
            Dcpctr_SPEC,
            crate::common::RW,
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
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            dcpctr::Sqset,
            dcpctr::Sqset,
            Dcpctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Sequence Toggle Bit Monitor"]
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

    #[doc = "Pipe Busy"]
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

    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, Dcpctr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,Dcpctr_SPEC,crate::common::RW>::from_register(self,0)
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
}
impl ::core::default::Default for Dcpctr {
    #[inline(always)]
    fn default() -> Dcpctr {
        <crate::RegValueT<Dcpctr_SPEC> as RegisterValue<_>>::new(64)
    }
}
pub mod dcpctr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsts_SPEC;
    pub type Bsts = crate::EnumBitfieldStruct<u8, Bsts_SPEC>;
    impl Bsts {
        #[doc = "Buffer access is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Buffer access is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sureq_SPEC;
    pub type Sureq = crate::EnumBitfieldStruct<u8, Sureq_SPEC>;
    impl Sureq {
        #[doc = "Invalid"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmits the setup packet."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sureqclr_SPEC;
    pub type Sureqclr = crate::EnumBitfieldStruct<u8, Sureqclr_SPEC>;
    impl Sureqclr {
        #[doc = "Invalid"]
        pub const _0: Self = Self::new(0);

        #[doc = "Clears the SUREQ bit to 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sqclr_SPEC;
    pub type Sqclr = crate::EnumBitfieldStruct<u8, Sqclr_SPEC>;
    impl Sqclr {
        #[doc = "Invalid"]
        pub const _0: Self = Self::new(0);

        #[doc = "Specifies DATA0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sqset_SPEC;
    pub type Sqset = crate::EnumBitfieldStruct<u8, Sqset_SPEC>;
    impl Sqset {
        #[doc = "Invalid"]
        pub const _0: Self = Self::new(0);

        #[doc = "Specifies DATA1."]
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
    pub struct Pbusy_SPEC;
    pub type Pbusy = crate::EnumBitfieldStruct<u8, Pbusy_SPEC>;
    impl Pbusy {
        #[doc = "DCP is not used for the transaction."]
        pub const _0: Self = Self::new(0);

        #[doc = "DCP is used for the transaction."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccpl_SPEC;
    pub type Ccpl = crate::EnumBitfieldStruct<u8, Ccpl_SPEC>;
    impl Ccpl {
        #[doc = "Invalid"]
        pub const _0: Self = Self::new(0);

        #[doc = "Completion of control transfer is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pid_SPEC;
    pub type Pid = crate::EnumBitfieldStruct<u8, Pid_SPEC>;
    impl Pid {
        #[doc = "NAK response"]
        pub const _00: Self = Self::new(0);

        #[doc = "BUF response (depending on the buffer state)"]
        pub const _01: Self = Self::new(1);

        #[doc = "STALL response"]
        pub const _10: Self = Self::new(2);

        #[doc = "STALL response"]
        pub const _11: Self = Self::new(3);
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
    #[doc = "These bits are read as 000000000000. The write value should be 000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfff, 1, 0, u16, u16, Pipesel_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xfff,1,0,u16,u16,Pipesel_SPEC,crate::common::RW>::from_register(self,0)
    }

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
        pub const _0000: Self = Self::new(0);

        #[doc = "PIPE1"]
        pub const _0001: Self = Self::new(1);

        #[doc = "PIPE2"]
        pub const _0010: Self = Self::new(2);

        #[doc = "PIPE3"]
        pub const _0011: Self = Self::new(3);

        #[doc = "PIPE4"]
        pub const _0100: Self = Self::new(4);

        #[doc = "PIPE5"]
        pub const _0101: Self = Self::new(5);

        #[doc = "PIPE6"]
        pub const _0110: Self = Self::new(6);

        #[doc = "PIPE7"]
        pub const _0111: Self = Self::new(7);

        #[doc = "PIPE8"]
        pub const _1000: Self = Self::new(8);

        #[doc = "PIPE9"]
        pub const _1001: Self = Self::new(9);
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

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Pipecfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Pipecfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
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

    #[doc = "Endpoint NumberThese bits specify the endpoint number for the selected pipe.Setting 0000b means unused pipe."]
    #[inline(always)]
    pub fn epnum(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Pipecfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Pipecfg_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Type_SPEC;
    pub type Type = crate::EnumBitfieldStruct<u8, Type_SPEC>;
    impl Type {
        #[doc = "Pipe not used"]
        pub const _00: Self = Self::new(0);

        #[doc = "Bulk transfer(PIPE1 and PIPE5) /Setting prohibited(PIPE6 to PIPE9)"]
        pub const _01: Self = Self::new(1);

        #[doc = "Setting prohibited(PIPE1 and PIPE5) /Interrupt transfer(PIPE6 to PIPE9)"]
        pub const _10: Self = Self::new(2);

        #[doc = "Isochronous transfer(PIPE1 and PIPE2) /Setting prohibited(PIPE3 to PIPE9)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfre_SPEC;
    pub type Bfre = crate::EnumBitfieldStruct<u8, Bfre_SPEC>;
    impl Bfre {
        #[doc = "BRDY interrupt upon transmitting or receiving data"]
        pub const _0: Self = Self::new(0);

        #[doc = "BRDY interrupt upon completion of reading data"]
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
    pub struct Shtnak_SPEC;
    pub type Shtnak = crate::EnumBitfieldStruct<u8, Shtnak_SPEC>;
    impl Shtnak {
        #[doc = "Continue pipe operation after transfer ends"]
        pub const _0: Self = Self::new(0);

        #[doc = "Disable pipe operation after transfer ends."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dir_SPEC;
    pub type Dir = crate::EnumBitfieldStruct<u8, Dir_SPEC>;
    impl Dir {
        #[doc = "Receiving direction"]
        pub const _0: Self = Self::new(0);

        #[doc = "Transmitting direction"]
        pub const _1: Self = Self::new(1);
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
    #[doc = "Device Select"]
    #[inline(always)]
    pub fn devsel(
        self,
    ) -> crate::common::RegisterField<
        12,
        0xf,
        1,
        0,
        pipemaxp::Devsel,
        pipemaxp::Devsel,
        Pipemaxp_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0xf,
            1,
            0,
            pipemaxp::Devsel,
            pipemaxp::Devsel,
            Pipemaxp_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, u8, Pipemaxp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7,1,0,u8,u8,Pipemaxp_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Maximum Packet SizePIPE1 and PIPE2:  1 byte (001h) to 256 bytes (100h)PIPE3 to PIPE5:   8 bytes (008h), 16 bytes (010h),  32 bytes (020h), 64 bytes (040h)   (Bits \\[8:7\\] and \\[2:0\\] are not provided.)PIPE6 to PIPE9:  1 byte (001h) to 64 bytes (040h)  (Bits \\[8:7\\] are not provided.)"]
    #[inline(always)]
    pub fn mxps(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Pipemaxp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Pipemaxp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pipemaxp {
    #[inline(always)]
    fn default() -> Pipemaxp {
        <crate::RegValueT<Pipemaxp_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pipemaxp {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Devsel_SPEC;
    pub type Devsel = crate::EnumBitfieldStruct<u8, Devsel_SPEC>;
    impl Devsel {
        #[doc = "Address 0000"]
        pub const _0000: Self = Self::new(0);

        #[doc = "Address 0001"]
        pub const _0001: Self = Self::new(1);

        #[doc = "Address 0010"]
        pub const _0010: Self = Self::new(2);

        #[doc = "Address 0011"]
        pub const _0011: Self = Self::new(3);

        #[doc = "Address 0100"]
        pub const _0100: Self = Self::new(4);

        #[doc = "Address 0101"]
        pub const _0101: Self = Self::new(5);
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

    #[doc = "These bits are read as 000000000. The write value should be 000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1ff, 1, 0, u16, u16, Pipeperi_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1ff,1,0,u16,u16,Pipeperi_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Interval Error Detection IntervalSpecifies the interval error detection timing for the selected pipe in terms of frames, which is expressed as nth power of 2."]
    #[inline(always)]
    pub fn iitv(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, Pipeperi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,Pipeperi_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "The buffer is not flushed."]
        pub const _0: Self = Self::new(0);

        #[doc = "The buffer is flushed."]
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
    #[doc = "Buffer Status"]
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
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pipectr::Sqset,
            pipectr::Sqset,
            Pipectr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "Sequence Toggle Bit Confirmation"]
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

    #[doc = "Pipe Busy"]
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

    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x7, 1, 0, u8, u8, Pipectr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x7,1,0,u8,u8,Pipectr_SPEC,crate::common::RW>::from_register(self,0)
    }

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
}
impl ::core::default::Default for Pipectr {
    #[inline(always)]
    fn default() -> Pipectr {
        <crate::RegValueT<Pipectr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pipectr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsts_SPEC;
    pub type Bsts = crate::EnumBitfieldStruct<u8, Bsts_SPEC>;
    impl Bsts {
        #[doc = "Buffer access is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Buffer access is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aclrm_SPEC;
    pub type Aclrm = crate::EnumBitfieldStruct<u8, Aclrm_SPEC>;
    impl Aclrm {
        #[doc = "Auto buffer clear mode is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Auto buffer clear mode is enabled (all buffers are initialized)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sqclr_SPEC;
    pub type Sqclr = crate::EnumBitfieldStruct<u8, Sqclr_SPEC>;
    impl Sqclr {
        #[doc = "Invalid"]
        pub const _0: Self = Self::new(0);

        #[doc = "Specifies DATA0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sqset_SPEC;
    pub type Sqset = crate::EnumBitfieldStruct<u8, Sqset_SPEC>;
    impl Sqset {
        #[doc = "Invalid"]
        pub const _0: Self = Self::new(0);

        #[doc = "Specifies DATA1."]
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
    pub struct Pbusy_SPEC;
    pub type Pbusy = crate::EnumBitfieldStruct<u8, Pbusy_SPEC>;
    impl Pbusy {
        #[doc = "Pipe n not in use for the transaction"]
        pub const _0: Self = Self::new(0);

        #[doc = "Pipe n in use for the transaction."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pid_SPEC;
    pub type Pid = crate::EnumBitfieldStruct<u8, Pid_SPEC>;
    impl Pid {
        #[doc = "NAK response"]
        pub const _00: Self = Self::new(0);

        #[doc = "BUF response (depending on the buffer state)"]
        pub const _01: Self = Self::new(1);

        #[doc = "STALL response"]
        pub const _10: Self = Self::new(2);

        #[doc = "STALL response"]
        pub const _11: Self = Self::new(3);
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

    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Pipetre_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Pipetre_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Trenb_SPEC;
    pub type Trenb = crate::EnumBitfieldStruct<u8, Trenb_SPEC>;
    impl Trenb {
        #[doc = "Transaction counter is disabled."]
        pub const _0: Self = Self::new(0);

        #[doc = "Transaction counter is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trclr_SPEC;
    pub type Trclr = crate::EnumBitfieldStruct<u8, Trclr_SPEC>;
    impl Trclr {
        #[doc = "Invalid"]
        pub const _0: Self = Self::new(0);

        #[doc = "The current counter value is cleared."]
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

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Devadd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Devadd_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "DEVADDn is not used"]
        pub const _00: Self = Self::new(0);

        #[doc = "Low speed"]
        pub const _01: Self = Self::new(1);

        #[doc = "Full speed"]
        pub const _10: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbmc_SPEC;
impl crate::sealed::RegSpec for Usbmc_SPEC {
    type DataType = u16;
}

#[doc = "USB Module Control Register"]
pub type Usbmc = crate::RegValueT<Usbmc_SPEC>;

impl Usbmc {
    #[doc = "USB Regulator On/Off Control"]
    #[inline(always)]
    pub fn vdcen(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        usbmc::Vdcen,
        usbmc::Vdcen,
        Usbmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            usbmc::Vdcen,
            usbmc::Vdcen,
            Usbmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 1. The write value should be 1."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Usbmc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Usbmc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "USB Reference Power Supply Circuit On/Off Control"]
    #[inline(always)]
    pub fn vddusbe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        usbmc::Vddusbe,
        usbmc::Vddusbe,
        Usbmc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            usbmc::Vddusbe,
            usbmc::Vddusbe,
            Usbmc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Usbmc {
    #[inline(always)]
    fn default() -> Usbmc {
        <crate::RegValueT<Usbmc_SPEC> as RegisterValue<_>>::new(2)
    }
}
pub mod usbmc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vdcen_SPEC;
    pub type Vdcen = crate::EnumBitfieldStruct<u8, Vdcen_SPEC>;
    impl Vdcen {
        #[doc = "USB regulator off"]
        pub const _0: Self = Self::new(0);

        #[doc = "USB regulator on"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vddusbe_SPEC;
    pub type Vddusbe = crate::EnumBitfieldStruct<u8, Vddusbe_SPEC>;
    impl Vddusbe {
        #[doc = "USB reference power supply circuit off"]
        pub const _0: Self = Self::new(0);

        #[doc = "USB reference power supply circuit on"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbbcctrl0_SPEC;
impl crate::sealed::RegSpec for Usbbcctrl0_SPEC {
    type DataType = u16;
}

#[doc = "BC Control Register 0"]
pub type Usbbcctrl0 = crate::RegValueT<Usbbcctrl0_SPEC>;

impl Usbbcctrl0 {
    #[doc = "D+ Pin 0.6 V Input Detection Status"]
    #[inline(always)]
    pub fn pddetsts0(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        usbbcctrl0::Pddetsts0,
        usbbcctrl0::Pddetsts0,
        Usbbcctrl0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            usbbcctrl0::Pddetsts0,
            usbbcctrl0::Pddetsts0,
            Usbbcctrl0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "D- Pin 0.6 V Input Detection Status"]
    #[inline(always)]
    pub fn chgdetsts0(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        usbbcctrl0::Chgdetsts0,
        usbbcctrl0::Chgdetsts0,
        Usbbcctrl0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            usbbcctrl0::Chgdetsts0,
            usbbcctrl0::Chgdetsts0,
            Usbbcctrl0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "BC (Battery Charger) Function Ch0 General Enable Control"]
    #[inline(always)]
    pub fn batchge0(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        usbbcctrl0::Batchge0,
        usbbcctrl0::Batchge0,
        Usbbcctrl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            usbbcctrl0::Batchge0,
            usbbcctrl0::Batchge0,
            Usbbcctrl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Usbbcctrl0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Usbbcctrl0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "D- Pin VDMSRC (0.6 V) Output Control"]
    #[inline(always)]
    pub fn vdmsrce0(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        usbbcctrl0::Vdmsrce0,
        usbbcctrl0::Vdmsrce0,
        Usbbcctrl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            usbbcctrl0::Vdmsrce0,
            usbbcctrl0::Vdmsrce0,
            Usbbcctrl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "D+ Pin 0.6 V Input Detection (Comparator and Sink) Control"]
    #[inline(always)]
    pub fn idpsinke0(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        usbbcctrl0::Idpsinke0,
        usbbcctrl0::Idpsinke0,
        Usbbcctrl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            usbbcctrl0::Idpsinke0,
            usbbcctrl0::Idpsinke0,
            Usbbcctrl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "D+ Pin VDPSRC (0.6 V) Output Control"]
    #[inline(always)]
    pub fn vdpsrce0(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        usbbcctrl0::Vdpsrce0,
        usbbcctrl0::Vdpsrce0,
        Usbbcctrl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            usbbcctrl0::Vdpsrce0,
            usbbcctrl0::Vdpsrce0,
            Usbbcctrl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "D- Pin 0.6 V Input Detection (Comparator and Sink) Control"]
    #[inline(always)]
    pub fn idmsinke0(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        usbbcctrl0::Idmsinke0,
        usbbcctrl0::Idmsinke0,
        Usbbcctrl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            usbbcctrl0::Idmsinke0,
            usbbcctrl0::Idmsinke0,
            Usbbcctrl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "D+ Pin IDPSRC Output Control"]
    #[inline(always)]
    pub fn idpsrce0(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        usbbcctrl0::Idpsrce0,
        usbbcctrl0::Idpsrce0,
        Usbbcctrl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            usbbcctrl0::Idpsrce0,
            usbbcctrl0::Idpsrce0,
            Usbbcctrl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "D- Pin Pull-Down Control"]
    #[inline(always)]
    pub fn rpdme0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        usbbcctrl0::Rpdme0,
        usbbcctrl0::Rpdme0,
        Usbbcctrl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            usbbcctrl0::Rpdme0,
            usbbcctrl0::Rpdme0,
            Usbbcctrl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Usbbcctrl0 {
    #[inline(always)]
    fn default() -> Usbbcctrl0 {
        <crate::RegValueT<Usbbcctrl0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod usbbcctrl0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pddetsts0_SPEC;
    pub type Pddetsts0 = crate::EnumBitfieldStruct<u8, Pddetsts0_SPEC>;
    impl Pddetsts0 {
        #[doc = "Not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chgdetsts0_SPEC;
    pub type Chgdetsts0 = crate::EnumBitfieldStruct<u8, Chgdetsts0_SPEC>;
    impl Chgdetsts0 {
        #[doc = "Not detected"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Batchge0_SPEC;
    pub type Batchge0 = crate::EnumBitfieldStruct<u8, Batchge0_SPEC>;
    impl Batchge0 {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vdmsrce0_SPEC;
    pub type Vdmsrce0 = crate::EnumBitfieldStruct<u8, Vdmsrce0_SPEC>;
    impl Vdmsrce0 {
        #[doc = "Stop"]
        pub const _0: Self = Self::new(0);

        #[doc = "0.6V output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idpsinke0_SPEC;
    pub type Idpsinke0 = crate::EnumBitfieldStruct<u8, Idpsinke0_SPEC>;
    impl Idpsinke0 {
        #[doc = "Detection off"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection on ( Comparator and sink current on )"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vdpsrce0_SPEC;
    pub type Vdpsrce0 = crate::EnumBitfieldStruct<u8, Vdpsrce0_SPEC>;
    impl Vdpsrce0 {
        #[doc = "Stop"]
        pub const _0: Self = Self::new(0);

        #[doc = "0.6V output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idmsinke0_SPEC;
    pub type Idmsinke0 = crate::EnumBitfieldStruct<u8, Idmsinke0_SPEC>;
    impl Idmsinke0 {
        #[doc = "Detection off"]
        pub const _0: Self = Self::new(0);

        #[doc = "Detection on ( Comparator and sink current on )"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idpsrce0_SPEC;
    pub type Idpsrce0 = crate::EnumBitfieldStruct<u8, Idpsrce0_SPEC>;
    impl Idpsrce0 {
        #[doc = "Stop"]
        pub const _0: Self = Self::new(0);

        #[doc = "10uA output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rpdme0_SPEC;
    pub type Rpdme0 = crate::EnumBitfieldStruct<u8, Rpdme0_SPEC>;
    impl Rpdme0 {
        #[doc = "Pull-down off"]
        pub const _0: Self = Self::new(0);

        #[doc = "Pull-down on"]
        pub const _1: Self = Self::new(1);
    }
}
