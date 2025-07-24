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
#[doc = r"MIPI_CSI0 Register area"]
unsafe impl ::core::marker::Send for super::MipiCsi0 {}
unsafe impl ::core::marker::Sync for super::MipiCsi0 {}
impl super::MipiCsi0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Module Configuration Register"]
    #[inline(always)]
    pub const fn mcg(&self) -> &'static crate::common::Reg<self::Mcg_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mcg_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Module Control Register 0"]
    #[inline(always)]
    pub const fn mct0(&self) -> &'static crate::common::Reg<self::Mct0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mct0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Module Control Register 2"]
    #[inline(always)]
    pub const fn mct2(&self) -> &'static crate::common::Reg<self::Mct2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mct2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Module Control Register 3"]
    #[inline(always)]
    pub const fn mct3(&self) -> &'static crate::common::Reg<self::Mct3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mct3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Reset Control Register"]
    #[inline(always)]
    pub const fn rtct(&self) -> &'static crate::common::Reg<self::Rtct_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Rtct_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "Reset Status Register"]
    #[inline(always)]
    pub const fn rtst(&self) -> &'static crate::common::Reg<self::Rtst_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rtst_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "EPD Option Control Register"]
    #[inline(always)]
    pub const fn epct(&self) -> &'static crate::common::Reg<self::Epct_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Epct_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "EPD Misc Option Control Register"]
    #[inline(always)]
    pub const fn emct(&self) -> &'static crate::common::Reg<self::Emct_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Emct_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "Module Interrupt Status Register"]
    #[inline(always)]
    pub const fn mist(&self) -> &'static crate::common::Reg<self::Mist_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mist_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "Receive Data Type Enable Low Register"]
    #[inline(always)]
    pub const fn dtel(&self) -> &'static crate::common::Reg<self::Dtel_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dtel_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "Receive Data Type Enable High Register"]
    #[inline(always)]
    pub const fn dteh(&self) -> &'static crate::common::Reg<self::Dteh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dteh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "Receive Status Register"]
    #[inline(always)]
    pub const fn rxst(&self) -> &'static crate::common::Reg<self::Rxst_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rxst_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[doc = "Receive Status Clear Register"]
    #[inline(always)]
    pub const fn rxsc(&self) -> &'static crate::common::Reg<self::Rxsc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Rxsc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[doc = "Receive Interrupt Enable Register"]
    #[inline(always)]
    pub const fn rxie(&self) -> &'static crate::common::Reg<self::Rxie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rxie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[doc = "Data Lane (N) Status Register"]
    #[inline(always)]
    pub const fn dlst(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Dlst_SPEC, crate::common::R>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x80usize))
        }
    }
    #[inline(always)]
    pub const fn dlst0(&self) -> &'static crate::common::Reg<self::Dlst_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Dlst_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x80usize),
            )
        }
    }
    #[inline(always)]
    pub const fn dlst1(&self) -> &'static crate::common::Reg<self::Dlst_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Dlst_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x90usize),
            )
        }
    }

    #[doc = "Data Lane (N) Status Clear Register"]
    #[inline(always)]
    pub const fn dlsc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Dlsc_SPEC, crate::common::W>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x84usize))
        }
    }
    #[inline(always)]
    pub const fn dlsc0(&self) -> &'static crate::common::Reg<self::Dlsc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Dlsc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x84usize),
            )
        }
    }
    #[inline(always)]
    pub const fn dlsc1(&self) -> &'static crate::common::Reg<self::Dlsc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Dlsc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x94usize),
            )
        }
    }

    #[doc = "Data Lane (N) Interrupt Enable Register"]
    #[inline(always)]
    pub const fn dlie(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Dlie_SPEC, crate::common::RW>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x88usize))
        }
    }
    #[inline(always)]
    pub const fn dlie0(&self) -> &'static crate::common::Reg<self::Dlie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dlie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x88usize),
            )
        }
    }
    #[inline(always)]
    pub const fn dlie1(&self) -> &'static crate::common::Reg<self::Dlie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dlie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x98usize),
            )
        }
    }

    #[doc = "Virtual Channel (M) Status Register"]
    #[inline(always)]
    pub const fn vcst(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Vcst_SPEC, crate::common::R>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x100usize))
        }
    }
    #[inline(always)]
    pub const fn vcst0(&self) -> &'static crate::common::Reg<self::Vcst_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Vcst_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x100usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcst1(&self) -> &'static crate::common::Reg<self::Vcst_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Vcst_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x110usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcst2(&self) -> &'static crate::common::Reg<self::Vcst_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Vcst_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x120usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcst3(&self) -> &'static crate::common::Reg<self::Vcst_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Vcst_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x130usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcst4(&self) -> &'static crate::common::Reg<self::Vcst_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Vcst_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x140usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcst5(&self) -> &'static crate::common::Reg<self::Vcst_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Vcst_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x150usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcst6(&self) -> &'static crate::common::Reg<self::Vcst_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Vcst_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x160usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcst7(&self) -> &'static crate::common::Reg<self::Vcst_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Vcst_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x170usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcst8(&self) -> &'static crate::common::Reg<self::Vcst_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Vcst_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x180usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcst9(&self) -> &'static crate::common::Reg<self::Vcst_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Vcst_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x190usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcst10(&self) -> &'static crate::common::Reg<self::Vcst_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Vcst_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcst11(&self) -> &'static crate::common::Reg<self::Vcst_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Vcst_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcst12(&self) -> &'static crate::common::Reg<self::Vcst_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Vcst_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcst13(&self) -> &'static crate::common::Reg<self::Vcst_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Vcst_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcst14(&self) -> &'static crate::common::Reg<self::Vcst_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Vcst_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcst15(&self) -> &'static crate::common::Reg<self::Vcst_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Vcst_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0x1f0usize),
            )
        }
    }

    #[doc = "Virtual Channel (M) Status Clear Register"]
    #[inline(always)]
    pub const fn vcsc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Vcsc_SPEC, crate::common::W>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x104usize))
        }
    }
    #[inline(always)]
    pub const fn vcsc0(&self) -> &'static crate::common::Reg<self::Vcsc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Vcsc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x104usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcsc1(&self) -> &'static crate::common::Reg<self::Vcsc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Vcsc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x114usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcsc2(&self) -> &'static crate::common::Reg<self::Vcsc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Vcsc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x124usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcsc3(&self) -> &'static crate::common::Reg<self::Vcsc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Vcsc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x134usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcsc4(&self) -> &'static crate::common::Reg<self::Vcsc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Vcsc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x144usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcsc5(&self) -> &'static crate::common::Reg<self::Vcsc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Vcsc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x154usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcsc6(&self) -> &'static crate::common::Reg<self::Vcsc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Vcsc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x164usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcsc7(&self) -> &'static crate::common::Reg<self::Vcsc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Vcsc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x174usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcsc8(&self) -> &'static crate::common::Reg<self::Vcsc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Vcsc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x184usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcsc9(&self) -> &'static crate::common::Reg<self::Vcsc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Vcsc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x194usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcsc10(&self) -> &'static crate::common::Reg<self::Vcsc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Vcsc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcsc11(&self) -> &'static crate::common::Reg<self::Vcsc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Vcsc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x1b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcsc12(&self) -> &'static crate::common::Reg<self::Vcsc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Vcsc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcsc13(&self) -> &'static crate::common::Reg<self::Vcsc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Vcsc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x1d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcsc14(&self) -> &'static crate::common::Reg<self::Vcsc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Vcsc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x1e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcsc15(&self) -> &'static crate::common::Reg<self::Vcsc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Vcsc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(0x1f4usize),
            )
        }
    }

    #[doc = "Virtual Channel (M) Interrupt Enable Register"]
    #[inline(always)]
    pub const fn vcie(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Vcie_SPEC, crate::common::RW>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x108usize))
        }
    }
    #[inline(always)]
    pub const fn vcie0(&self) -> &'static crate::common::Reg<self::Vcie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vcie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x108usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcie1(&self) -> &'static crate::common::Reg<self::Vcie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vcie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x118usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcie2(&self) -> &'static crate::common::Reg<self::Vcie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vcie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x128usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcie3(&self) -> &'static crate::common::Reg<self::Vcie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vcie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x138usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcie4(&self) -> &'static crate::common::Reg<self::Vcie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vcie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x148usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcie5(&self) -> &'static crate::common::Reg<self::Vcie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vcie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x158usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcie6(&self) -> &'static crate::common::Reg<self::Vcie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vcie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x168usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcie7(&self) -> &'static crate::common::Reg<self::Vcie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vcie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x178usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcie8(&self) -> &'static crate::common::Reg<self::Vcie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vcie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x188usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcie9(&self) -> &'static crate::common::Reg<self::Vcie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vcie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x198usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcie10(&self) -> &'static crate::common::Reg<self::Vcie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vcie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcie11(&self) -> &'static crate::common::Reg<self::Vcie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vcie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcie12(&self) -> &'static crate::common::Reg<self::Vcie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vcie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcie13(&self) -> &'static crate::common::Reg<self::Vcie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vcie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcie14(&self) -> &'static crate::common::Reg<self::Vcie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vcie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn vcie15(&self) -> &'static crate::common::Reg<self::Vcie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vcie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x1f8usize),
            )
        }
    }

    #[doc = "Power Management Status Register"]
    #[inline(always)]
    pub const fn pmst(&self) -> &'static crate::common::Reg<self::Pmst_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Pmst_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(512usize),
            )
        }
    }

    #[doc = "Power Management Status Clear Register"]
    #[inline(always)]
    pub const fn pmsc(&self) -> &'static crate::common::Reg<self::Pmsc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Pmsc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(516usize),
            )
        }
    }

    #[doc = "Power Management Interrupt Enable Register"]
    #[inline(always)]
    pub const fn pmie(&self) -> &'static crate::common::Reg<self::Pmie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pmie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(520usize),
            )
        }
    }

    #[doc = "Generic Short Packet Control Register"]
    #[inline(always)]
    pub const fn gsct(&self) -> &'static crate::common::Reg<self::Gsct_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gsct_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(640usize),
            )
        }
    }

    #[doc = "Generic Short Packet Status Register"]
    #[inline(always)]
    pub const fn gsst(&self) -> &'static crate::common::Reg<self::Gsst_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gsst_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(644usize),
            )
        }
    }

    #[doc = "Generic Short Packet Status Clear Register"]
    #[inline(always)]
    pub const fn gssc(&self) -> &'static crate::common::Reg<self::Gssc_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Gssc_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(648usize),
            )
        }
    }

    #[doc = "Generic Short Packet Interrupt Enable Register"]
    #[inline(always)]
    pub const fn gsie(&self) -> &'static crate::common::Reg<self::Gsie_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gsie_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(652usize),
            )
        }
    }

    #[doc = "Generic Short Packet Register"]
    #[inline(always)]
    pub const fn gsht(&self) -> &'static crate::common::Reg<self::Gsht_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Gsht_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(656usize),
            )
        }
    }

    #[doc = "Generic Short Packet Information Update Register"]
    #[inline(always)]
    pub const fn gsiu(&self) -> &'static crate::common::Reg<self::Gsiu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gsiu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(660usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcg_SPEC;
impl crate::sealed::RegSpec for Mcg_SPEC {
    type DataType = u32;
}

#[doc = "Module Configuration Register"]
pub type Mcg = crate::RegValueT<Mcg_SPEC>;

impl Mcg {
    #[doc = "Version of this IP"]
    #[inline(always)]
    pub fn ver(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Mcg_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Mcg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Number of Supported Data Lanes"]
    #[inline(always)]
    pub fn sdln(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, mcg::Sdln, mcg::Sdln, Mcg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xf,1,0,mcg::Sdln,mcg::Sdln,Mcg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Number of Generic Short Packet FIFO"]
    #[inline(always)]
    pub fn gsnm(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Mcg_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Mcg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mcg {
    #[inline(always)]
    fn default() -> Mcg {
        <crate::RegValueT<Mcg_SPEC> as RegisterValue<_>>::new(1049088)
    }
}
pub mod mcg {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdln_SPEC;
    pub type Sdln = crate::EnumBitfieldStruct<u8, Sdln_SPEC>;
    impl Sdln {
        #[doc = "Operable with 2 lanes or 1 lane"]
        pub const _0_X_2: Self = Self::new(2);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mct0_SPEC;
impl crate::sealed::RegSpec for Mct0_SPEC {
    type DataType = u32;
}

#[doc = "Module Control Register 0"]
pub type Mct0 = crate::RegValueT<Mct0_SPEC>;

impl Mct0 {
    #[doc = "Number of Valid Data Lanes"]
    #[inline(always)]
    pub fn vdln(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        mct0::Vdln,
        mct0::Vdln,
        Mct0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            mct0::Vdln,
            mct0::Vdln,
            Mct0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Zero Length Long Packet Output Mode"]
    #[inline(always)]
    pub fn zlmd(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        mct0::Zlmd,
        mct0::Zlmd,
        Mct0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            mct0::Zlmd,
            mct0::Zlmd,
            Mct0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ErrframeData Notification Mode"]
    #[inline(always)]
    pub fn edmd(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        mct0::Edmd,
        mct0::Edmd,
        Mct0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            mct0::Edmd,
            mct0::Edmd,
            Mct0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Reserved Packet Reception Mode"]
    #[inline(always)]
    pub fn rvmd(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        mct0::Rvmd,
        mct0::Rvmd,
        Mct0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            mct0::Rvmd,
            mct0::Rvmd,
            Mct0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Generic CSI-2 Rule Mode"]
    #[inline(always)]
    pub fn grmd(self) -> crate::common::RegisterFieldBool<20, 1, 0, Mct0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Mct0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "ECC Check CSI-2 Ver 1.3 Mode"]
    #[inline(always)]
    pub fn eccv13(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        mct0::Eccv13,
        mct0::Eccv13,
        Mct0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            mct0::Eccv13,
            mct0::Eccv13,
            Mct0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "LFSR Enable Mode"]
    #[inline(always)]
    pub fn lfsren(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        mct0::Lfsren,
        mct0::Lfsren,
        Mct0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            mct0::Lfsren,
            mct0::Lfsren,
            Mct0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mct0 {
    #[inline(always)]
    fn default() -> Mct0 {
        <crate::RegValueT<Mct0_SPEC> as RegisterValue<_>>::new(33554434)
    }
}
pub mod mct0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vdln_SPEC;
    pub type Vdln = crate::EnumBitfieldStruct<u8, Vdln_SPEC>;
    impl Vdln {
        #[doc = "Operation with one lane"]
        pub const _0_X_1: Self = Self::new(1);

        #[doc = "Operation with two lanes"]
        pub const _0_X_2: Self = Self::new(2);

        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Zlmd_SPEC;
    pub type Zlmd = crate::EnumBitfieldStruct<u8, Zlmd_SPEC>;
    impl Zlmd {
        #[doc = "Output"]
        pub const _0: Self = Self::new(0);

        #[doc = "No output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Edmd_SPEC;
    pub type Edmd = crate::EnumBitfieldStruct<u8, Edmd_SPEC>;
    impl Edmd {
        #[doc = "Does not notify ErrFrameData"]
        pub const _0: Self = Self::new(0);

        #[doc = "Notifies ErrFrameData"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rvmd_SPEC;
    pub type Rvmd = crate::EnumBitfieldStruct<u8, Rvmd_SPEC>;
    impl Rvmd {
        #[doc = "Discards the data (notifies an ErrID)"]
        pub const _0: Self = Self::new(0);

        #[doc = "Receives data as a long packet (outputs data to Video Pixel interface when the corresponding bit in DTEH is 1, or discards the data and notifies an ErrID when the corresponding bit in DTEH is 0)."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eccv13_SPEC;
    pub type Eccv13 = crate::EnumBitfieldStruct<u8, Eccv13_SPEC>;
    impl Eccv13 {
        #[doc = "26 bits to be checked for ECC"]
        pub const _0: Self = Self::new(0);

        #[doc = "24 bits to be checked for ECC"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lfsren_SPEC;
    pub type Lfsren = crate::EnumBitfieldStruct<u8, Lfsren_SPEC>;
    impl Lfsren {
        #[doc = "Disables descrambling"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables descrambling"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mct2_SPEC;
impl crate::sealed::RegSpec for Mct2_SPEC {
    type DataType = u32;
}

#[doc = "Module Control Register 2"]
pub type Mct2 = crate::RegValueT<Mct2_SPEC>;

impl Mct2 {
    #[doc = "Frequency clock rate to determine packet reception end (internal setting parameter)"]
    #[inline(always)]
    pub fn frrclk(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Mct2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Mct2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Frequency clock rate to adjust data lane skew (internal setting parameter)"]
    #[inline(always)]
    pub fn frrskw(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, u16, Mct2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1ff,1,0,u16,u16,Mct2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mct2 {
    #[inline(always)]
    fn default() -> Mct2 {
        <crate::RegValueT<Mct2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mct3_SPEC;
impl crate::sealed::RegSpec for Mct3_SPEC {
    type DataType = u32;
}

#[doc = "Module Control Register 3"]
pub type Mct3 = crate::RegValueT<Mct3_SPEC>;

impl Mct3 {
    #[doc = "RX (reception) Enable"]
    #[inline(always)]
    pub fn rxen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mct3::Rxen,
        mct3::Rxen,
        Mct3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mct3::Rxen,
            mct3::Rxen,
            Mct3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mct3 {
    #[inline(always)]
    fn default() -> Mct3 {
        <crate::RegValueT<Mct3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mct3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxen_SPEC;
    pub type Rxen = crate::EnumBitfieldStruct<u8, Rxen_SPEC>;
    impl Rxen {
        #[doc = "Disables reception"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables reception"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtct_SPEC;
impl crate::sealed::RegSpec for Rtct_SPEC {
    type DataType = u32;
}

#[doc = "Reset Control Register"]
pub type Rtct = crate::RegValueT<Rtct_SPEC>;

impl Rtct {
    #[doc = "Video Pixel Interface Software Reset"]
    #[inline(always)]
    pub fn vsrst(self) -> crate::common::RegisterFieldBool<0, 1, 0, Rtct_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Rtct_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Rtct {
    #[inline(always)]
    fn default() -> Rtct {
        <crate::RegValueT<Rtct_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtst_SPEC;
impl crate::sealed::RegSpec for Rtst_SPEC {
    type DataType = u32;
}

#[doc = "Reset Status Register"]
pub type Rtst = crate::RegValueT<Rtst_SPEC>;

impl Rtst {
    #[doc = "Video Pixel Interface Software Reset Status"]
    #[inline(always)]
    pub fn vsrsts(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rtst::Vsrsts,
        rtst::Vsrsts,
        Rtst_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rtst::Vsrsts,
            rtst::Vsrsts,
            Rtst_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rtst {
    #[inline(always)]
    fn default() -> Rtst {
        <crate::RegValueT<Rtst_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rtst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vsrsts_SPEC;
    pub type Vsrsts = crate::EnumBitfieldStruct<u8, Vsrsts_SPEC>;
    impl Vsrsts {
        #[doc = "Does not indicate status during a reset"]
        pub const _0: Self = Self::new(0);

        #[doc = "Indicates status during a reset"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Epct_SPEC;
impl crate::sealed::RegSpec for Epct_SPEC {
    type DataType = u32;
}

#[doc = "EPD Option Control Register"]
pub type Epct = crate::RegValueT<Epct_SPEC>;

impl Epct {
    #[doc = "Long Packet Spacers"]
    #[inline(always)]
    pub fn slp(
        self,
    ) -> crate::common::RegisterField<0, 0x7fff, 1, 0, u16, u16, Epct_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7fff,1,0,u16,u16,Epct_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "EPD Option Select"]
    #[inline(always)]
    pub fn epdop(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        epct::Epdop,
        epct::Epdop,
        Epct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            epct::Epdop,
            epct::Epdop,
            Epct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "EPD Short Packet Spacers"]
    #[inline(always)]
    pub fn ssp(
        self,
    ) -> crate::common::RegisterField<16, 0x7fff, 1, 0, u16, u16, Epct_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7fff,1,0,u16,u16,Epct_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable EPD Operation"]
    #[inline(always)]
    pub fn epden(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        epct::Epden,
        epct::Epden,
        Epct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            epct::Epden,
            epct::Epden,
            Epct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Epct {
    #[inline(always)]
    fn default() -> Epct {
        <crate::RegValueT<Epct_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod epct {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Epdop_SPEC;
    pub type Epdop = crate::EnumBitfieldStruct<u8, Epdop_SPEC>;
    impl Epdop {
        #[doc = "D-PHY EPD Option 1 (0 cannot be set when EPDEN = 1)"]
        pub const _0: Self = Self::new(0);

        #[doc = "D-PHY EPD Option 2"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Epden_SPEC;
    pub type Epden = crate::EnumBitfieldStruct<u8, Epden_SPEC>;
    impl Epden {
        #[doc = "Disables the EPD"]
        pub const _0: Self = Self::new(0);

        #[doc = "Enables the EPD"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emct_SPEC;
impl crate::sealed::RegSpec for Emct_SPEC {
    type DataType = u32;
}

#[doc = "EPD Misc Option Control Register"]
pub type Emct = crate::RegValueT<Emct_SPEC>;

impl Emct {
    #[doc = "Enable Variable Length Spacer Insertions"]
    #[inline(always)]
    pub fn vlsien(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        emct::Vlsien,
        emct::Vlsien,
        Emct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            emct::Vlsien,
            emct::Vlsien,
            Emct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Enable EOTP"]
    #[inline(always)]
    pub fn eotpen(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        emct::Eotpen,
        emct::Eotpen,
        Emct_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            emct::Eotpen,
            emct::Eotpen,
            Emct_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Emct {
    #[inline(always)]
    fn default() -> Emct {
        <crate::RegValueT<Emct_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod emct {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vlsien_SPEC;
    pub type Vlsien = crate::EnumBitfieldStruct<u8, Vlsien_SPEC>;
    impl Vlsien {
        #[doc = "Variable length spacer invalid (spacer number is fixed length)"]
        pub const _00: Self = Self::new(0);

        #[doc = "Variable length spacer valid (spacer number is 1 × n/lane)"]
        pub const _01: Self = Self::new(1);

        #[doc = "Variable length spacer valid (spacer number is 2 × n/lane)"]
        pub const _10: Self = Self::new(2);

        #[doc = "Variable length spacer valid (spacer number is 4 × n/lane)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eotpen_SPEC;
    pub type Eotpen = crate::EnumBitfieldStruct<u8, Eotpen_SPEC>;
    impl Eotpen {
        #[doc = "No EOTP"]
        pub const _0: Self = Self::new(0);

        #[doc = "With EOTP"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mist_SPEC;
impl crate::sealed::RegSpec for Mist_SPEC {
    type DataType = u32;
}

#[doc = "Module Interrupt Status Register"]
pub type Mist = crate::RegValueT<Mist_SPEC>;

impl Mist {
    #[doc = "Interrupt status of data lane 0"]
    #[inline(always)]
    pub fn dl0s(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mist_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mist_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of data lane 1"]
    #[inline(always)]
    pub fn dl1s(self) -> crate::common::RegisterFieldBool<1, 1, 0, Mist_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mist_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of power management"]
    #[inline(always)]
    pub fn pms(self) -> crate::common::RegisterFieldBool<8, 1, 0, Mist_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Mist_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of generic short packet"]
    #[inline(always)]
    pub fn gsts(self) -> crate::common::RegisterFieldBool<9, 1, 0, Mist_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Mist_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of RX (reception)"]
    #[inline(always)]
    pub fn rxs(self) -> crate::common::RegisterFieldBool<10, 1, 0, Mist_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Mist_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of virtual channel 0"]
    #[inline(always)]
    pub fn vc0s(self) -> crate::common::RegisterFieldBool<16, 1, 0, Mist_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Mist_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of virtual channel 1"]
    #[inline(always)]
    pub fn vc1s(self) -> crate::common::RegisterFieldBool<17, 1, 0, Mist_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Mist_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of virtual channel 2"]
    #[inline(always)]
    pub fn vc2s(self) -> crate::common::RegisterFieldBool<18, 1, 0, Mist_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Mist_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of virtual channel 3"]
    #[inline(always)]
    pub fn vc3s(self) -> crate::common::RegisterFieldBool<19, 1, 0, Mist_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Mist_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of virtual channel 4"]
    #[inline(always)]
    pub fn vc4s(self) -> crate::common::RegisterFieldBool<20, 1, 0, Mist_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Mist_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of virtual channel 5"]
    #[inline(always)]
    pub fn vc5s(self) -> crate::common::RegisterFieldBool<21, 1, 0, Mist_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Mist_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of virtual channel 6"]
    #[inline(always)]
    pub fn vc6s(self) -> crate::common::RegisterFieldBool<22, 1, 0, Mist_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Mist_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of virtual channel 7"]
    #[inline(always)]
    pub fn vc7s(self) -> crate::common::RegisterFieldBool<23, 1, 0, Mist_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Mist_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of virtual channel 8"]
    #[inline(always)]
    pub fn vc8s(self) -> crate::common::RegisterFieldBool<24, 1, 0, Mist_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Mist_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of virtual channel 9"]
    #[inline(always)]
    pub fn vc9s(self) -> crate::common::RegisterFieldBool<25, 1, 0, Mist_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Mist_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of virtual channel 10"]
    #[inline(always)]
    pub fn vc10s(self) -> crate::common::RegisterFieldBool<26, 1, 0, Mist_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Mist_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of virtual channel 11"]
    #[inline(always)]
    pub fn vc11s(self) -> crate::common::RegisterFieldBool<27, 1, 0, Mist_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Mist_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of virtual channel 12"]
    #[inline(always)]
    pub fn vc12s(self) -> crate::common::RegisterFieldBool<28, 1, 0, Mist_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Mist_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of virtual channel 13"]
    #[inline(always)]
    pub fn vc13s(self) -> crate::common::RegisterFieldBool<29, 1, 0, Mist_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Mist_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of virtual channel 14"]
    #[inline(always)]
    pub fn vc14s(self) -> crate::common::RegisterFieldBool<30, 1, 0, Mist_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Mist_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Interrupt status of virtual channel 15"]
    #[inline(always)]
    pub fn vc15s(self) -> crate::common::RegisterFieldBool<31, 1, 0, Mist_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Mist_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Mist {
    #[inline(always)]
    fn default() -> Mist {
        <crate::RegValueT<Mist_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtel_SPEC;
impl crate::sealed::RegSpec for Dtel_SPEC {
    type DataType = u32;
}

#[doc = "Receive Data Type Enable Low Register"]
pub type Dtel = crate::RegValueT<Dtel_SPEC>;

impl Dtel {
    #[doc = "Data Type Enable (DT = 0x00 to 0x1F)"]
    #[inline(always)]
    pub fn dten(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        dtel::Dten,
        dtel::Dten,
        Dtel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            dtel::Dten,
            dtel::Dten,
            Dtel_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dtel {
    #[inline(always)]
    fn default() -> Dtel {
        <crate::RegValueT<Dtel_SPEC> as RegisterValue<_>>::new(15)
    }
}
pub mod dtel {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dten_SPEC;
    pub type Dten = crate::EnumBitfieldStruct<u8, Dten_SPEC>;
    impl Dten {
        #[doc = "Does not receive data"]
        pub const _0: Self = Self::new(0);

        #[doc = "Receives data"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dteh_SPEC;
impl crate::sealed::RegSpec for Dteh_SPEC {
    type DataType = u32;
}

#[doc = "Receive Data Type Enable High Register"]
pub type Dteh = crate::RegValueT<Dteh_SPEC>;

impl Dteh {
    #[doc = "Data Type Enable (DT = 0x20 to 0x3F)"]
    #[inline(always)]
    pub fn dten(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        dteh::Dten,
        dteh::Dten,
        Dteh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            dteh::Dten,
            dteh::Dten,
            Dteh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dteh {
    #[inline(always)]
    fn default() -> Dteh {
        <crate::RegValueT<Dteh_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dteh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dten_SPEC;
    pub type Dten = crate::EnumBitfieldStruct<u8, Dten_SPEC>;
    impl Dten {
        #[doc = "Does not receive data"]
        pub const _0: Self = Self::new(0);

        #[doc = "Receives data"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxst_SPEC;
impl crate::sealed::RegSpec for Rxst_SPEC {
    type DataType = u32;
}

#[doc = "Receive Status Register"]
pub type Rxst = crate::RegValueT<Rxst_SPEC>;

impl Rxst {
    #[doc = "Frame of virtual channel 0 active"]
    #[inline(always)]
    pub fn frm0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Rxst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Rxst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame of virtual channel 1 active"]
    #[inline(always)]
    pub fn frm1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Rxst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Rxst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame of virtual channel 2 active"]
    #[inline(always)]
    pub fn frm2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Rxst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Rxst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame of virtual channel 3 active"]
    #[inline(always)]
    pub fn frm3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Rxst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Rxst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame of virtual channel 4 active"]
    #[inline(always)]
    pub fn frm4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Rxst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Rxst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame of virtual channel 5 active"]
    #[inline(always)]
    pub fn frm5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Rxst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Rxst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame of virtual channel 6 active"]
    #[inline(always)]
    pub fn frm6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Rxst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Rxst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame of virtual channel 7 active"]
    #[inline(always)]
    pub fn frm7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Rxst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Rxst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame of virtual channel 8 active"]
    #[inline(always)]
    pub fn frm8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Rxst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Rxst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "FRaMe of virtual channel 9 active"]
    #[inline(always)]
    pub fn frm9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Rxst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Rxst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame of virtual channel 10 active"]
    #[inline(always)]
    pub fn frm10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Rxst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Rxst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame of virtual channel 11 active"]
    #[inline(always)]
    pub fn frm11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Rxst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Rxst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame of virtual channel 12 active"]
    #[inline(always)]
    pub fn frm12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Rxst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Rxst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame of virtual channel 13 active"]
    #[inline(always)]
    pub fn frm13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Rxst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Rxst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame of virtual channel 14 active"]
    #[inline(always)]
    pub fn frm14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Rxst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Rxst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame of virtual channel 15 active"]
    #[inline(always)]
    pub fn frm15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Rxst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Rxst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Rx (Reception) active status"]
    #[inline(always)]
    pub fn ract(self) -> crate::common::RegisterFieldBool<16, 1, 0, Rxst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Rxst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "RX (Reception) Active Detect"]
    #[inline(always)]
    pub fn ractdet(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Rxst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Rxst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Rxst {
    #[inline(always)]
    fn default() -> Rxst {
        <crate::RegValueT<Rxst_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxsc_SPEC;
impl crate::sealed::RegSpec for Rxsc_SPEC {
    type DataType = u32;
}

#[doc = "Receive Status Clear Register"]
pub type Rxsc = crate::RegValueT<Rxsc_SPEC>;

impl Rxsc {
    #[doc = "RX (Reception) Active Detect Status Clear"]
    #[inline(always)]
    pub fn ractdetc(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Rxsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, Rxsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Rxsc {
    #[inline(always)]
    fn default() -> Rxsc {
        <crate::RegValueT<Rxsc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxie_SPEC;
impl crate::sealed::RegSpec for Rxie_SPEC {
    type DataType = u32;
}

#[doc = "Receive Interrupt Enable Register"]
pub type Rxie = crate::RegValueT<Rxie_SPEC>;

impl Rxie {
    #[doc = "RX (Reception) Active Detect Interrupt Enable"]
    #[inline(always)]
    pub fn ractdete(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        rxie::Ractdete,
        rxie::Ractdete,
        Rxie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            rxie::Ractdete,
            rxie::Ractdete,
            Rxie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rxie {
    #[inline(always)]
    fn default() -> Rxie {
        <crate::RegValueT<Rxie_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rxie {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ractdete_SPEC;
    pub type Ractdete = crate::EnumBitfieldStruct<u8, Ractdete_SPEC>;
    impl Ractdete {
        #[doc = "Does not assert csi2_int_rx when RXST.RACTDET = 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_rx when RXST.RACTDET = 1"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dlst_SPEC;
impl crate::sealed::RegSpec for Dlst_SPEC {
    type DataType = u32;
}

#[doc = "Data Lane (N) Status Register"]
pub type Dlst = crate::RegValueT<Dlst_SPEC>;

impl Dlst {
    #[doc = "ErrSotHs detect on data lane (N) status"]
    #[inline(always)]
    pub fn esh(self) -> crate::common::RegisterFieldBool<0, 1, 0, Dlst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Dlst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "ErrSotSynchs detect on data lane (N) status"]
    #[inline(always)]
    pub fn ess(self) -> crate::common::RegisterFieldBool<1, 1, 0, Dlst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Dlst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "ErrConTrol detect on data lane (N) status"]
    #[inline(always)]
    pub fn ect(self) -> crate::common::RegisterFieldBool<2, 1, 0, Dlst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Dlst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "ErrESc detect on data lane (N) status"]
    #[inline(always)]
    pub fn ees(self) -> crate::common::RegisterFieldBool<3, 1, 0, Dlst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Dlst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Exit from ULps detect on data lane (N) status"]
    #[inline(always)]
    pub fn eul(self) -> crate::common::RegisterFieldBool<16, 1, 0, Dlst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Dlst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Entry to ULPS detect on data lane (N) status"]
    #[inline(always)]
    pub fn rul(self) -> crate::common::RegisterFieldBool<17, 1, 0, Dlst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Dlst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "RxULPsesc of data lane (N) status"]
    #[inline(always)]
    pub fn ulp(self) -> crate::common::RegisterFieldBool<24, 1, 0, Dlst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Dlst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Dlst {
    #[inline(always)]
    fn default() -> Dlst {
        <crate::RegValueT<Dlst_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dlsc_SPEC;
impl crate::sealed::RegSpec for Dlsc_SPEC {
    type DataType = u32;
}

#[doc = "Data Lane (N) Status Clear Register"]
pub type Dlsc = crate::RegValueT<Dlsc_SPEC>;

impl Dlsc {
    #[doc = "ErrSotHs detect on data lane (N) status Clear"]
    #[inline(always)]
    pub fn eshc(self) -> crate::common::RegisterFieldBool<0, 1, 0, Dlsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Dlsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "ErrSotSynchs detect on data lane (N) status Clear"]
    #[inline(always)]
    pub fn essc(self) -> crate::common::RegisterFieldBool<1, 1, 0, Dlsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Dlsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "ErrConTrol detect on data lane (N) status Clear"]
    #[inline(always)]
    pub fn ectc(self) -> crate::common::RegisterFieldBool<2, 1, 0, Dlsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Dlsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "ErrESc detect on data lane (N) status Clear"]
    #[inline(always)]
    pub fn eesc(self) -> crate::common::RegisterFieldBool<3, 1, 0, Dlsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Dlsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Exit from ULps detect on data lane (N) status Clear"]
    #[inline(always)]
    pub fn eulc(self) -> crate::common::RegisterFieldBool<16, 1, 0, Dlsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Dlsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Entry to ULps detect on data lane (N) status Clear"]
    #[inline(always)]
    pub fn rulc(self) -> crate::common::RegisterFieldBool<17, 1, 0, Dlsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, Dlsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Dlsc {
    #[inline(always)]
    fn default() -> Dlsc {
        <crate::RegValueT<Dlsc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dlie_SPEC;
impl crate::sealed::RegSpec for Dlie_SPEC {
    type DataType = u32;
}

#[doc = "Data Lane (N) Interrupt Enable Register"]
pub type Dlie = crate::RegValueT<Dlie_SPEC>;

impl Dlie {
    #[doc = "ErrSotHs detect on data lane (N) interrupt Enable"]
    #[inline(always)]
    pub fn eshe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dlie::Eshe,
        dlie::Eshe,
        Dlie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dlie::Eshe,
            dlie::Eshe,
            Dlie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ErrSotSynchs detect on data lane (N) interrupt Enable"]
    #[inline(always)]
    pub fn esse(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        dlie::Esse,
        dlie::Esse,
        Dlie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            dlie::Esse,
            dlie::Esse,
            Dlie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ErrConTrol detect on data lane (N) interrupt Enable"]
    #[inline(always)]
    pub fn ecte(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        dlie::Ecte,
        dlie::Ecte,
        Dlie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            dlie::Ecte,
            dlie::Ecte,
            Dlie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ErrESc detect on data lane (N) interrupt Enable"]
    #[inline(always)]
    pub fn eese(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        dlie::Eese,
        dlie::Eese,
        Dlie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            dlie::Eese,
            dlie::Eese,
            Dlie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Exit to ULps detect on data lane (N) interrupt Enable"]
    #[inline(always)]
    pub fn eule(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        dlie::Eule,
        dlie::Eule,
        Dlie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            dlie::Eule,
            dlie::Eule,
            Dlie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Entry to ULps detect on data lane (N) interrupt Enable"]
    #[inline(always)]
    pub fn rule(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        dlie::Rule,
        dlie::Rule,
        Dlie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            dlie::Rule,
            dlie::Rule,
            Dlie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Dlie {
    #[inline(always)]
    fn default() -> Dlie {
        <crate::RegValueT<Dlie_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dlie {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eshe_SPEC;
    pub type Eshe = crate::EnumBitfieldStruct<u8, Eshe_SPEC>;
    impl Eshe {
        #[doc = "Does not assert csi2_int_dl when DLST(N).ESH = 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_dl when DLST(N).ESH = 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Esse_SPEC;
    pub type Esse = crate::EnumBitfieldStruct<u8, Esse_SPEC>;
    impl Esse {
        #[doc = "Does not assert csi2_int_dl when DLST(N).ESS = 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_dl when DLST(N).ESS = 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecte_SPEC;
    pub type Ecte = crate::EnumBitfieldStruct<u8, Ecte_SPEC>;
    impl Ecte {
        #[doc = "Does not assert csi2_int_dl when DLST(N).ECT = 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_dl when DLST(N).ECT = 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eese_SPEC;
    pub type Eese = crate::EnumBitfieldStruct<u8, Eese_SPEC>;
    impl Eese {
        #[doc = "Does not assert csi2_int_dl when DLST(N).EES = 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_dl when DLST(N).EES = 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eule_SPEC;
    pub type Eule = crate::EnumBitfieldStruct<u8, Eule_SPEC>;
    impl Eule {
        #[doc = "Does not assert csi2_int_dl when DLST(N).EUL = 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_dl when DLST(N).EUL = 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rule_SPEC;
    pub type Rule = crate::EnumBitfieldStruct<u8, Rule_SPEC>;
    impl Rule {
        #[doc = "Does not assert csi2_int_dl when DLST(N).RUL = 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_dl when DLST(N).RUL = 1"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vcst_SPEC;
impl crate::sealed::RegSpec for Vcst_SPEC {
    type DataType = u32;
}

#[doc = "Virtual Channel (M) Status Register"]
pub type Vcst = crate::RegValueT<Vcst_SPEC>;

impl Vcst {
    #[doc = "MaLFormed packet with virtual channel (M) detect status"]
    #[inline(always)]
    pub fn mlf(self) -> crate::common::RegisterFieldBool<0, 1, 0, Vcst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Vcst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "ECc 2-bit (Double) error packet with virtual channel (M) Detect status"]
    #[inline(always)]
    pub fn ecd(self) -> crate::common::RegisterFieldBool<1, 1, 0, Vcst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Vcst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "CRC error packet with virtual channel (M) detect status"]
    #[inline(always)]
    pub fn crc(self) -> crate::common::RegisterFieldBool<2, 1, 0, Vcst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Vcst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "ErrID packet with virtual channel (M) detect status"]
    #[inline(always)]
    pub fn ide(self) -> crate::common::RegisterFieldBool<3, 1, 0, Vcst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Vcst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Word Count Error packet with virtual channel (M) detect status"]
    #[inline(always)]
    pub fn wce(self) -> crate::common::RegisterFieldBool<4, 1, 0, Vcst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Vcst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "ECc 1-bit error (Corrected) packet with virtual channel (M) detect status"]
    #[inline(always)]
    pub fn ecc(self) -> crate::common::RegisterFieldBool<5, 1, 0, Vcst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Vcst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "ECc No-error packet with virtual channel (M) detect status"]
    #[inline(always)]
    pub fn ecn(self) -> crate::common::RegisterFieldBool<6, 1, 0, Vcst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Vcst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "errFRameSync of virtual channel (M) detect status"]
    #[inline(always)]
    pub fn frs(self) -> crate::common::RegisterFieldBool<8, 1, 0, Vcst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Vcst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "errFRameData of virtual channel (M) detect status"]
    #[inline(always)]
    pub fn frd(self) -> crate::common::RegisterFieldBool<9, 1, 0, Vcst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Vcst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "generic short packet with virtual channel (M) discard by fifo OVerFlow status"]
    #[inline(always)]
    pub fn ovf(self) -> crate::common::RegisterFieldBool<16, 1, 0, Vcst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Vcst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame Start packet with virtual channel (M) Reception status"]
    #[inline(always)]
    pub fn fsr(self) -> crate::common::RegisterFieldBool<24, 1, 0, Vcst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Vcst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame End packet with virtual channel (M) Reception status"]
    #[inline(always)]
    pub fn fer(self) -> crate::common::RegisterFieldBool<25, 1, 0, Vcst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Vcst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Line Start packet with virtual channel (M) Reception status"]
    #[inline(always)]
    pub fn lsr(self) -> crate::common::RegisterFieldBool<26, 1, 0, Vcst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Vcst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Line End packet with virtual channel (M) Reception status"]
    #[inline(always)]
    pub fn ler(self) -> crate::common::RegisterFieldBool<27, 1, 0, Vcst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Vcst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "EoTp packet with virtual channel (M) Reception status"]
    #[inline(always)]
    pub fn etr(self) -> crate::common::RegisterFieldBool<28, 1, 0, Vcst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Vcst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Vcst {
    #[inline(always)]
    fn default() -> Vcst {
        <crate::RegValueT<Vcst_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vcsc_SPEC;
impl crate::sealed::RegSpec for Vcsc_SPEC {
    type DataType = u32;
}

#[doc = "Virtual Channel (M) Status Clear Register"]
pub type Vcsc = crate::RegValueT<Vcsc_SPEC>;

impl Vcsc {
    #[doc = "MaLFormed packet with virtual channel (M) detect status Clear"]
    #[inline(always)]
    pub fn mlfc(self) -> crate::common::RegisterFieldBool<0, 1, 0, Vcsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Vcsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "ECc 2-bit (Double) error packet with virtual channel (M) Detect status Clear"]
    #[inline(always)]
    pub fn ecdc(self) -> crate::common::RegisterFieldBool<1, 1, 0, Vcsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Vcsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "CRC error packet with virtual channel (M) detect status Clear"]
    #[inline(always)]
    pub fn crcc(self) -> crate::common::RegisterFieldBool<2, 1, 0, Vcsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Vcsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "ErrID packet with virtual channel (M) detect status Clear"]
    #[inline(always)]
    pub fn idec(self) -> crate::common::RegisterFieldBool<3, 1, 0, Vcsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Vcsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Word Count Error packet with virtual channel (M) detect status Clear"]
    #[inline(always)]
    pub fn wcec(self) -> crate::common::RegisterFieldBool<4, 1, 0, Vcsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Vcsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "ECc 1-bit error (Corrected) packet with virtual channel (M) detect status Clear"]
    #[inline(always)]
    pub fn eccc(self) -> crate::common::RegisterFieldBool<5, 1, 0, Vcsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Vcsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "ECc No-error packet with virtual channel (M) detect status Clear"]
    #[inline(always)]
    pub fn ecnc(self) -> crate::common::RegisterFieldBool<6, 1, 0, Vcsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, Vcsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "errFRameSync of virtual channel (M) detect status Clear"]
    #[inline(always)]
    pub fn frsc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Vcsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Vcsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "errFRameData of virtual channel (M) detect status Clear"]
    #[inline(always)]
    pub fn frdc(self) -> crate::common::RegisterFieldBool<9, 1, 0, Vcsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, Vcsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "MaLFormed packet with any virtual channels detect status Clear"]
    #[inline(always)]
    pub fn amlfc(self) -> crate::common::RegisterFieldBool<14, 1, 0, Vcsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, Vcsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "ECc 2-bit (Double) error packet with any virtual channels Detect status Clear"]
    #[inline(always)]
    pub fn aecdc(self) -> crate::common::RegisterFieldBool<15, 1, 0, Vcsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, Vcsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Generic short packet with virtual channel (M) discard by FIFO Overflow Status Clear"]
    #[inline(always)]
    pub fn ovfc(self) -> crate::common::RegisterFieldBool<16, 1, 0, Vcsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Vcsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame Start packet with virtual channel (M) Reception status Clear"]
    #[inline(always)]
    pub fn fsrc(self) -> crate::common::RegisterFieldBool<24, 1, 0, Vcsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, Vcsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Frame End packet with virtual channel (M) Reception status Clear"]
    #[inline(always)]
    pub fn ferc(self) -> crate::common::RegisterFieldBool<25, 1, 0, Vcsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, Vcsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Line Start packet with virtual channel (M) Reception status Clear"]
    #[inline(always)]
    pub fn lsrc(self) -> crate::common::RegisterFieldBool<26, 1, 0, Vcsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, Vcsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Line End packet with virtual channel (M) Reception status Clear"]
    #[inline(always)]
    pub fn lerc(self) -> crate::common::RegisterFieldBool<27, 1, 0, Vcsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, Vcsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "EoTp packet with virtual channel (M) Reception status Clear"]
    #[inline(always)]
    pub fn etrc(self) -> crate::common::RegisterFieldBool<28, 1, 0, Vcsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, Vcsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Vcsc {
    #[inline(always)]
    fn default() -> Vcsc {
        <crate::RegValueT<Vcsc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vcie_SPEC;
impl crate::sealed::RegSpec for Vcie_SPEC {
    type DataType = u32;
}

#[doc = "Virtual Channel (M) Interrupt Enable Register"]
pub type Vcie = crate::RegValueT<Vcie_SPEC>;

impl Vcie {
    #[doc = "MaLFormed packet with virtual channel (M) detect interrupt Enable"]
    #[inline(always)]
    pub fn mlfe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        vcie::Mlfe,
        vcie::Mlfe,
        Vcie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            vcie::Mlfe,
            vcie::Mlfe,
            Vcie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ECc 2-bit (Double) error packet with virtual channel (M) Detect interrupt Enable"]
    #[inline(always)]
    pub fn ecde(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        vcie::Ecde,
        vcie::Ecde,
        Vcie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            vcie::Ecde,
            vcie::Ecde,
            Vcie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "CRC error packet with virtual channel (M) detect interrupt Enable"]
    #[inline(always)]
    pub fn crce(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        vcie::Crce,
        vcie::Crce,
        Vcie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            vcie::Crce,
            vcie::Crce,
            Vcie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ErrID packet with virtual channel (M) detect interrupt Enable"]
    #[inline(always)]
    pub fn idee(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        vcie::Idee,
        vcie::Idee,
        Vcie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            vcie::Idee,
            vcie::Idee,
            Vcie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Word Count Error packet with virtual channel (M) detect interrupt Enable"]
    #[inline(always)]
    pub fn wcee(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        vcie::Wcee,
        vcie::Wcee,
        Vcie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            vcie::Wcee,
            vcie::Wcee,
            Vcie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ECc 1-bit error (Corrected) packet with virtual channel (M) detect interrupt Enable"]
    #[inline(always)]
    pub fn ecce(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        vcie::Ecce,
        vcie::Ecce,
        Vcie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            vcie::Ecce,
            vcie::Ecce,
            Vcie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "ECc No-error packet with virtual channel (M) detect interrupt Enable"]
    #[inline(always)]
    pub fn ecne(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        vcie::Ecne,
        vcie::Ecne,
        Vcie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            vcie::Ecne,
            vcie::Ecne,
            Vcie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "errFRameSync of virtual channel (M) detect interrupt Enable"]
    #[inline(always)]
    pub fn frse(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        vcie::Frse,
        vcie::Frse,
        Vcie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            vcie::Frse,
            vcie::Frse,
            Vcie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "errFRameData of virtual channel (M) detect interrupt Enable"]
    #[inline(always)]
    pub fn frde(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        vcie::Frde,
        vcie::Frde,
        Vcie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            vcie::Frde,
            vcie::Frde,
            Vcie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Generic short packet with virtual channel (M) discarded by FIFO Overflow interrupt Enable"]
    #[inline(always)]
    pub fn ovfe(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        vcie::Ovfe,
        vcie::Ovfe,
        Vcie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            vcie::Ovfe,
            vcie::Ovfe,
            Vcie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Frame Start packet with virtual channel (M) Reception interrupt Enable"]
    #[inline(always)]
    pub fn fsre(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        vcie::Fsre,
        vcie::Fsre,
        Vcie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            vcie::Fsre,
            vcie::Fsre,
            Vcie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Frame End packet with virtual channel (M) Reception interrupt Enable"]
    #[inline(always)]
    pub fn fere(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        vcie::Fere,
        vcie::Fere,
        Vcie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            vcie::Fere,
            vcie::Fere,
            Vcie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Line Start packet with virtual channel (M) Reception interrupt Enable"]
    #[inline(always)]
    pub fn lsre(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        vcie::Lsre,
        vcie::Lsre,
        Vcie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            vcie::Lsre,
            vcie::Lsre,
            Vcie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Line End packet with virtual channel (M) Reception interrupt Enable"]
    #[inline(always)]
    pub fn lere(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        vcie::Lere,
        vcie::Lere,
        Vcie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            vcie::Lere,
            vcie::Lere,
            Vcie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "EoTp packet with virtual channel (M) Reception interrupt Enable"]
    #[inline(always)]
    pub fn etre(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        vcie::Etre,
        vcie::Etre,
        Vcie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            vcie::Etre,
            vcie::Etre,
            Vcie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Vcie {
    #[inline(always)]
    fn default() -> Vcie {
        <crate::RegValueT<Vcie_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vcie {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mlfe_SPEC;
    pub type Mlfe = crate::EnumBitfieldStruct<u8, Mlfe_SPEC>;
    impl Mlfe {
        #[doc = "Does not assert csi2_int_vc when VCST(M).MLF = 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_vc when VCST(M).MLF = 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecde_SPEC;
    pub type Ecde = crate::EnumBitfieldStruct<u8, Ecde_SPEC>;
    impl Ecde {
        #[doc = "Does not assert csi2_int_vc when VCST(M).ECD = 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_vc when VCST(M).ECD = 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crce_SPEC;
    pub type Crce = crate::EnumBitfieldStruct<u8, Crce_SPEC>;
    impl Crce {
        #[doc = "Does not assert csi2_int_vc when VCST(M).CRC = 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_vc when VCST(M).CRC = 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idee_SPEC;
    pub type Idee = crate::EnumBitfieldStruct<u8, Idee_SPEC>;
    impl Idee {
        #[doc = "Does not assert csi2_int_vc when VCST(M).IDE = 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_vc when VCST(M).IDE = 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wcee_SPEC;
    pub type Wcee = crate::EnumBitfieldStruct<u8, Wcee_SPEC>;
    impl Wcee {
        #[doc = "Does not assert csi2_int_vc when VCST(M).WCE = 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_vc when VCST(M).WCE = 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecce_SPEC;
    pub type Ecce = crate::EnumBitfieldStruct<u8, Ecce_SPEC>;
    impl Ecce {
        #[doc = "Does not assert csi2_int_vc when VCST(M).ECC = 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_vc when VCST(M).ECC = 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecne_SPEC;
    pub type Ecne = crate::EnumBitfieldStruct<u8, Ecne_SPEC>;
    impl Ecne {
        #[doc = "Does not assert csi2_int_vc when VCST(M).ECN = 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_vc when VCST(M).ECN = 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Frse_SPEC;
    pub type Frse = crate::EnumBitfieldStruct<u8, Frse_SPEC>;
    impl Frse {
        #[doc = "Does not assert csi2_int_vc when VCST(M).FRS = 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_vc when VCST(M).FRS = 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Frde_SPEC;
    pub type Frde = crate::EnumBitfieldStruct<u8, Frde_SPEC>;
    impl Frde {
        #[doc = "Does not assert csi2_int_vc when VCST(M).FRD = 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_vc when VCST(M).FRD = 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ovfe_SPEC;
    pub type Ovfe = crate::EnumBitfieldStruct<u8, Ovfe_SPEC>;
    impl Ovfe {
        #[doc = "Does not assert csi2_int_vc when VCST(M).OVF = 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_vc when VCST(M).OVF = 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsre_SPEC;
    pub type Fsre = crate::EnumBitfieldStruct<u8, Fsre_SPEC>;
    impl Fsre {
        #[doc = "Does not assert csi2_int_vc when VCST(M).FSR = 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_vc when VCST(M).FSR = 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fere_SPEC;
    pub type Fere = crate::EnumBitfieldStruct<u8, Fere_SPEC>;
    impl Fere {
        #[doc = "Does not assert csi2_int_vc when VCST(M).FER = 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_vc when VCST(M).FER = 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lsre_SPEC;
    pub type Lsre = crate::EnumBitfieldStruct<u8, Lsre_SPEC>;
    impl Lsre {
        #[doc = "Does not assert csi2_int_vc when VCST(M).LSR = 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_vc when VCST(M).LSR = 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lere_SPEC;
    pub type Lere = crate::EnumBitfieldStruct<u8, Lere_SPEC>;
    impl Lere {
        #[doc = "Does not assert csi2_int_vc when VCST(M).LER = 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_vc when VCST(M).LER = 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Etre_SPEC;
    pub type Etre = crate::EnumBitfieldStruct<u8, Etre_SPEC>;
    impl Etre {
        #[doc = "Does not assert csi2_int_vc when VCST(M).ETR = 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_vc when VCST(M).ETR = 1"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmst_SPEC;
impl crate::sealed::RegSpec for Pmst_SPEC {
    type DataType = u32;
}

#[doc = "Power Management Status Register"]
pub type Pmst = crate::RegValueT<Pmst_SPEC>;

impl Pmst {
    #[doc = "eXit from Stop state detect on all valid Data lanes status"]
    #[inline(always)]
    pub fn dsx(self) -> crate::common::RegisterFieldBool<0, 1, 0, Pmst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Pmst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "eNtry to Stop state detect on all valid Data lanes status"]
    #[inline(always)]
    pub fn dsn(self) -> crate::common::RegisterFieldBool<1, 1, 0, Pmst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Pmst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "eXit from Stop state detect on Clock lane status"]
    #[inline(always)]
    pub fn csx(self) -> crate::common::RegisterFieldBool<2, 1, 0, Pmst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Pmst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "eNtry to Stop state detect on Clock lane status"]
    #[inline(always)]
    pub fn csn(self) -> crate::common::RegisterFieldBool<3, 1, 0, Pmst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Pmst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "eXit from Ulps detect on all valid Data lanes status"]
    #[inline(always)]
    pub fn dux(self) -> crate::common::RegisterFieldBool<4, 1, 0, Pmst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Pmst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "eNtry to Ulps detect on all valid Data lanes status"]
    #[inline(always)]
    pub fn dun(self) -> crate::common::RegisterFieldBool<5, 1, 0, Pmst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Pmst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "eXit from Ulps detect on Clock lane status"]
    #[inline(always)]
    pub fn cux(self) -> crate::common::RegisterFieldBool<6, 1, 0, Pmst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Pmst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "eNtry to Ulps detect on Clock lane status"]
    #[inline(always)]
    pub fn cun(self) -> crate::common::RegisterFieldBool<7, 1, 0, Pmst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Pmst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Stop State of Clock Lane status"]
    #[inline(always)]
    pub fn clss(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        pmst::Clss,
        pmst::Clss,
        Pmst_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pmst::Clss,
            pmst::Clss,
            Pmst_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "rxULpsclknot (inverted) of Clock Lane status"]
    #[inline(always)]
    pub fn clul(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        pmst::Clul,
        pmst::Clul,
        Pmst_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pmst::Clul,
            pmst::Clul,
            Pmst_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Stop State of Data Lanes status"]
    #[inline(always)]
    pub fn dlss(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        pmst::Dlss,
        pmst::Dlss,
        Pmst_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            pmst::Dlss,
            pmst::Dlss,
            Pmst_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "rxULpsesc of Data Lanes status"]
    #[inline(always)]
    pub fn dlul(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        pmst::Dlul,
        pmst::Dlul,
        Pmst_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            pmst::Dlul,
            pmst::Dlul,
            Pmst_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pmst {
    #[inline(always)]
    fn default() -> Pmst {
        <crate::RegValueT<Pmst_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pmst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clss_SPEC;
    pub type Clss = crate::EnumBitfieldStruct<u8, Clss_SPEC>;
    impl Clss {
        #[doc = "Not in the stop state"]
        pub const _0: Self = Self::new(0);

        #[doc = "In the stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clul_SPEC;
    pub type Clul = crate::EnumBitfieldStruct<u8, Clul_SPEC>;
    impl Clul {
        #[doc = "Not in ULPS"]
        pub const _0: Self = Self::new(0);

        #[doc = "In ULPS"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlss_SPEC;
    pub type Dlss = crate::EnumBitfieldStruct<u8, Dlss_SPEC>;
    impl Dlss {
        #[doc = "Stop state of lane 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "Stop state of lane 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlul_SPEC;
    pub type Dlul = crate::EnumBitfieldStruct<u8, Dlul_SPEC>;
    impl Dlul {
        #[doc = "RxUlpsEsc of lane 0"]
        pub const _0: Self = Self::new(0);

        #[doc = "RxUlpsEsc of lane 1"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmsc_SPEC;
impl crate::sealed::RegSpec for Pmsc_SPEC {
    type DataType = u32;
}

#[doc = "Power Management Status Clear Register"]
pub type Pmsc = crate::RegValueT<Pmsc_SPEC>;

impl Pmsc {
    #[doc = "eXit from Stop state detect on all valid Data lanes status Clear"]
    #[inline(always)]
    pub fn dsxc(self) -> crate::common::RegisterFieldBool<0, 1, 0, Pmsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Pmsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "eNtry to Stop state detect on all valid Data lanes status Clear"]
    #[inline(always)]
    pub fn dsnc(self) -> crate::common::RegisterFieldBool<1, 1, 0, Pmsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Pmsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "eXit from Stop state detect on Clock lane status Clear"]
    #[inline(always)]
    pub fn csxc(self) -> crate::common::RegisterFieldBool<2, 1, 0, Pmsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Pmsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "eNtry to Stop state detect on Clock lane status Clear"]
    #[inline(always)]
    pub fn csnc(self) -> crate::common::RegisterFieldBool<3, 1, 0, Pmsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Pmsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "eXit from Ulps detect on all valid Data lanes status Clear"]
    #[inline(always)]
    pub fn duxc(self) -> crate::common::RegisterFieldBool<4, 1, 0, Pmsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Pmsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "eNtry to Ulps detect on all valid Data lanes status Clear"]
    #[inline(always)]
    pub fn dunc(self) -> crate::common::RegisterFieldBool<5, 1, 0, Pmsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Pmsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "eXit from Ulps detect on Clock lane status Clear"]
    #[inline(always)]
    pub fn cuxc(self) -> crate::common::RegisterFieldBool<6, 1, 0, Pmsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, Pmsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "eNtry to Ulps detect on Clock lane status Clear"]
    #[inline(always)]
    pub fn cunc(self) -> crate::common::RegisterFieldBool<7, 1, 0, Pmsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, Pmsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Pmsc {
    #[inline(always)]
    fn default() -> Pmsc {
        <crate::RegValueT<Pmsc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmie_SPEC;
impl crate::sealed::RegSpec for Pmie_SPEC {
    type DataType = u32;
}

#[doc = "Power Management Interrupt Enable Register"]
pub type Pmie = crate::RegValueT<Pmie_SPEC>;

impl Pmie {
    #[doc = "eXit from Stop state detect on all valid Data lanes interrupt Enable"]
    #[inline(always)]
    pub fn dsxe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pmie::Dsxe,
        pmie::Dsxe,
        Pmie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pmie::Dsxe,
            pmie::Dsxe,
            Pmie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "eNtry to Stop state detect on all valid Data lanes interrupt Enable"]
    #[inline(always)]
    pub fn dsne(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pmie::Dsne,
        pmie::Dsne,
        Pmie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pmie::Dsne,
            pmie::Dsne,
            Pmie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "eXit from Stop state detect on Clock lane interrupt Enable"]
    #[inline(always)]
    pub fn csxe(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pmie::Csxe,
        pmie::Csxe,
        Pmie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pmie::Csxe,
            pmie::Csxe,
            Pmie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "eNtry to Stop state detect on Clock lane interrupt Enable"]
    #[inline(always)]
    pub fn csne(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        pmie::Csne,
        pmie::Csne,
        Pmie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            pmie::Csne,
            pmie::Csne,
            Pmie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "eXit from Ulps detect on all valid Data lanes interrupt Enable"]
    #[inline(always)]
    pub fn duxe(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        pmie::Duxe,
        pmie::Duxe,
        Pmie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pmie::Duxe,
            pmie::Duxe,
            Pmie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "eNtry to Ulps detect on all valid Data lanes interrupt Enable"]
    #[inline(always)]
    pub fn dune(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        pmie::Dune,
        pmie::Dune,
        Pmie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            pmie::Dune,
            pmie::Dune,
            Pmie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "eXit from Ulps detect on Clock lane interrupt Enable"]
    #[inline(always)]
    pub fn cuxe(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pmie::Cuxe,
        pmie::Cuxe,
        Pmie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pmie::Cuxe,
            pmie::Cuxe,
            Pmie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "eNtry to Ulps detect on Clock lane interrupt Enable"]
    #[inline(always)]
    pub fn cune(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pmie::Cune,
        pmie::Cune,
        Pmie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pmie::Cune,
            pmie::Cune,
            Pmie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pmie {
    #[inline(always)]
    fn default() -> Pmie {
        <crate::RegValueT<Pmie_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pmie {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dsxe_SPEC;
    pub type Dsxe = crate::EnumBitfieldStruct<u8, Dsxe_SPEC>;
    impl Dsxe {
        #[doc = "Does not assert csi2_int_pm when PMST.DSX = 1."]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_pm when PMST.DSX = 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dsne_SPEC;
    pub type Dsne = crate::EnumBitfieldStruct<u8, Dsne_SPEC>;
    impl Dsne {
        #[doc = "Does not assert csi2_int_pm when PMST.DSN = 1."]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_pm when PMST.DSN = 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csxe_SPEC;
    pub type Csxe = crate::EnumBitfieldStruct<u8, Csxe_SPEC>;
    impl Csxe {
        #[doc = "Does not assert csi2_int_pm when PMST.CSX = 1."]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_pm when PMST.CSX = 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csne_SPEC;
    pub type Csne = crate::EnumBitfieldStruct<u8, Csne_SPEC>;
    impl Csne {
        #[doc = "Does not assert csi2_int_pm when PMST.CSN = 1."]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_pm when PMST.CSN = 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Duxe_SPEC;
    pub type Duxe = crate::EnumBitfieldStruct<u8, Duxe_SPEC>;
    impl Duxe {
        #[doc = "Does not assert csi2_int_pm when PMST.DUX = 1."]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_pm when PMST.DUX = 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dune_SPEC;
    pub type Dune = crate::EnumBitfieldStruct<u8, Dune_SPEC>;
    impl Dune {
        #[doc = "Does not assert csi2_int_pm when PMST.DUN = 1."]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_pm when PMST.DUN = 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cuxe_SPEC;
    pub type Cuxe = crate::EnumBitfieldStruct<u8, Cuxe_SPEC>;
    impl Cuxe {
        #[doc = "Does not assert csi2_int_pm when PMST.CUX = 1."]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_pm when PMST.CUX = 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cune_SPEC;
    pub type Cune = crate::EnumBitfieldStruct<u8, Cune_SPEC>;
    impl Cune {
        #[doc = "Does not assert csi2_int_pm when PMST.CUN = 1."]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_pm when PMST.CUN = 1."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gsct_SPEC;
impl crate::sealed::RegSpec for Gsct_SPEC {
    type DataType = u32;
}

#[doc = "Generic Short Packet Control Register"]
pub type Gsct = crate::RegValueT<Gsct_SPEC>;

impl Gsct {
    #[doc = "Stored generic short packet THreshold"]
    #[inline(always)]
    pub fn shth(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, Gsct_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,Gsct_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Generic short packet store in FIFO"]
    #[inline(always)]
    pub fn gfif(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gsct_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gsct_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Gsct {
    #[inline(always)]
    fn default() -> Gsct {
        <crate::RegValueT<Gsct_SPEC> as RegisterValue<_>>::new(65536)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gsst_SPEC;
impl crate::sealed::RegSpec for Gsst_SPEC {
    type DataType = u32;
}

#[doc = "Generic Short Packet Status Register"]
pub type Gsst = crate::RegValueT<Gsst_SPEC>;

impl Gsst {
    #[doc = "Generic short packet FIFO Not Empty"]
    #[inline(always)]
    pub fn gne(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gsst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gsst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "More than THreshold Generic short packets existed in FIFO"]
    #[inline(always)]
    pub fn gth(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gsst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gsst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Generic short packet fifo OVerflow status"]
    #[inline(always)]
    pub fn gov(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gsst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gsst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "NUMber of stored generic short Packets in FIFO"]
    #[inline(always)]
    pub fn pnum(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Gsst_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Gsst_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Generic short packet FIFO Clear status"]
    #[inline(always)]
    pub fn gcd(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gsst_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gsst_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[doc = "Generic short packet SToRe DiSable"]
    #[inline(always)]
    pub fn strds(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        gsst::Strds,
        gsst::Strds,
        Gsst_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gsst::Strds,
            gsst::Strds,
            Gsst_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gsst {
    #[inline(always)]
    fn default() -> Gsst {
        <crate::RegValueT<Gsst_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gsst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Strds_SPEC;
    pub type Strds = crate::EnumBitfieldStruct<u8, Strds_SPEC>;
    impl Strds {
        #[doc = "Can be stored in the generic short packet FIFO"]
        pub const _0: Self = Self::new(0);

        #[doc = "Cannot be stored in the generic short packet FIFO"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gssc_SPEC;
impl crate::sealed::RegSpec for Gssc_SPEC {
    type DataType = u32;
}

#[doc = "Generic Short Packet Status Clear Register"]
pub type Gssc = crate::RegValueT<Gssc_SPEC>;

impl Gssc {
    #[doc = "Generic short packet FIFO OVerflow status Clear"]
    #[inline(always)]
    pub fn govc(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gssc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gssc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Gssc {
    #[inline(always)]
    fn default() -> Gssc {
        <crate::RegValueT<Gssc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gsie_SPEC;
impl crate::sealed::RegSpec for Gsie_SPEC {
    type DataType = u32;
}

#[doc = "Generic Short Packet Interrupt Enable Register"]
pub type Gsie = crate::RegValueT<Gsie_SPEC>;

impl Gsie {
    #[doc = "Generic short packet FIFO Not Empty interrupt Enable"]
    #[inline(always)]
    pub fn gnee(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gsie::Gnee,
        gsie::Gnee,
        Gsie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gsie::Gnee,
            gsie::Gnee,
            Gsie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "More than THreshold Generic short packets existed in FIFO interrupt Enable"]
    #[inline(always)]
    pub fn gthe(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gsie::Gthe,
        gsie::Gthe,
        Gsie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gsie::Gthe,
            gsie::Gthe,
            Gsie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Generic short packet FIFO OVerflow interrupt Enable"]
    #[inline(always)]
    pub fn gove(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gsie::Gove,
        gsie::Gove,
        Gsie_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gsie::Gove,
            gsie::Gove,
            Gsie_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gsie {
    #[inline(always)]
    fn default() -> Gsie {
        <crate::RegValueT<Gsie_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gsie {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gnee_SPEC;
    pub type Gnee = crate::EnumBitfieldStruct<u8, Gnee_SPEC>;
    impl Gnee {
        #[doc = "Does not assert csi2_int_gst when GSST.GNE = 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_gst when GSST.GNE = 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gthe_SPEC;
    pub type Gthe = crate::EnumBitfieldStruct<u8, Gthe_SPEC>;
    impl Gthe {
        #[doc = "Does not assert csi2_int_gst when GSST.GTH = 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_gst when GSST.GTH = 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gove_SPEC;
    pub type Gove = crate::EnumBitfieldStruct<u8, Gove_SPEC>;
    impl Gove {
        #[doc = "Does not assert csi2_int_gst when GSST.GOV = 1"]
        pub const _0: Self = Self::new(0);

        #[doc = "Asserts csi2_int_gst when GSST.GOV = 1"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gsht_SPEC;
impl crate::sealed::RegSpec for Gsht_SPEC {
    type DataType = u32;
}

#[doc = "Generic Short Packet Register"]
pub type Gsht = crate::RegValueT<Gsht_SPEC>;

impl Gsht {
    #[doc = "Stored Packet DaTa"]
    #[inline(always)]
    pub fn spdt(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Gsht_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Gsht_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Stored packet Data TYPe"]
    #[inline(always)]
    pub fn dtyp(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, u8, Gsht_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3f,1,0,u8,u8,Gsht_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Stored Packet Virtual Channel"]
    #[inline(always)]
    pub fn spvc(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, u8, Gsht_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xf,1,0,u8,u8,Gsht_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Gsht {
    #[inline(always)]
    fn default() -> Gsht {
        <crate::RegValueT<Gsht_SPEC> as RegisterValue<_>>::new(524288)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gsiu_SPEC;
impl crate::sealed::RegSpec for Gsiu_SPEC {
    type DataType = u32;
}

#[doc = "Generic Short Packet Information Update Register"]
pub type Gsiu = crate::RegValueT<Gsiu_SPEC>;

impl Gsiu {
    #[doc = "Generic short packet FIFO update (INCrement internal pointer)"]
    #[inline(always)]
    pub fn finc(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gsiu_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gsiu_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[doc = "Generic short packet FIFO CLeaR"]
    #[inline(always)]
    pub fn gfclr(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gsiu_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gsiu_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[doc = "Generic short packet FIFO ENable"]
    #[inline(always)]
    pub fn gfen(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gsiu_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gsiu_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Gsiu {
    #[inline(always)]
    fn default() -> Gsiu {
        <crate::RegValueT<Gsiu_SPEC> as RegisterValue<_>>::new(0)
    }
}
