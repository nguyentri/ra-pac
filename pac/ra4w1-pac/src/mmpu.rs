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
// Generated from SVD 1.0, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:50:19 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Bus Master MPU"]
unsafe impl ::core::marker::Send for super::Mmpu {}
unsafe impl ::core::marker::Sync for super::Mmpu {}
impl super::Mmpu {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Bus Master MPU Control Register A"]
    #[inline(always)]
    pub const fn mmpuctla(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuctla_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuctla_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Group A Region %s Access Control Register"]
    #[inline(always)]
    pub const fn mmpuaca(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x200usize))
        }
    }
    #[inline(always)]
    pub const fn mmpuaca0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x200usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x210usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca2(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x220usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca3(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x230usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca4(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x240usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca5(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x250usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca6(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x260usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca7(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x270usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca8(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x280usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca9(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x290usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca10(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2a0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca11(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca12(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2c0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca13(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2d0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca14(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2e0usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuaca15(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuaca_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuaca_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f0usize),
            )
        }
    }

    #[doc = "Group A Region %s Start Address Register"]
    #[inline(always)]
    pub const fn mmpusa(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x204usize))
        }
    }
    #[inline(always)]
    pub const fn mmpusa0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x204usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x214usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa2(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x224usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa3(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x234usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa4(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x244usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa5(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x254usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa6(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x264usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa7(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x274usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa8(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x284usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa9(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x294usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa10(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2a4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa11(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa12(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2c4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa13(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2d4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa14(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2e4usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpusa15(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f4usize),
            )
        }
    }

    #[doc = "Group A Region %s End Address Register"]
    #[inline(always)]
    pub const fn mmpuea(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW>,
        16,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x208usize))
        }
    }
    #[inline(always)]
    pub const fn mmpuea0(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x208usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea1(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x218usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea2(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x228usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea3(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x238usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea4(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x248usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea5(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x258usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea6(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x268usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea7(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x278usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea8(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x288usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea9(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x298usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea10(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2a8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea11(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2b8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea12(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2c8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea13(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2d8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea14(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2e8usize),
            )
        }
    }
    #[inline(always)]
    pub const fn mmpuea15(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuea_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuea_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0x2f8usize),
            )
        }
    }

    #[doc = "Group A Protection of Register"]
    #[inline(always)]
    pub const fn mmpupta(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpupta_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpupta_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(258usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuctla_SPEC;
impl crate::sealed::RegSpec for Mmpuctla_SPEC {
    type DataType = u16;
}

#[doc = "Bus Master MPU Control Register A"]
pub type Mmpuctla = crate::RegValueT<Mmpuctla_SPEC>;

impl Mmpuctla {
    #[doc = "Key CodeThese bits are used to enable or disable writing of the OAD and ENABLE bit."]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Mmpuctla_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Mmpuctla_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Mmpuctla_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Mmpuctla_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Operation after detection"]
    #[inline(always)]
    pub fn oad(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mmpuctla::Oad,
        mmpuctla::Oad,
        Mmpuctla_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mmpuctla::Oad,
            mmpuctla::Oad,
            Mmpuctla_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Master Group enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuctla::Enable,
        mmpuctla::Enable,
        Mmpuctla_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuctla::Enable,
            mmpuctla::Enable,
            Mmpuctla_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpuctla {
    #[inline(always)]
    fn default() -> Mmpuctla {
        <crate::RegValueT<Mmpuctla_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuctla {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oad_SPEC;
    pub type Oad = crate::EnumBitfieldStruct<u8, Oad_SPEC>;
    impl Oad {
        #[doc = "Non-maskable interrupt."]
        pub const _0: Self = Self::new(0);

        #[doc = "Internal reset."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Master Group A disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Master Group A enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuaca_SPEC;
impl crate::sealed::RegSpec for Mmpuaca_SPEC {
    type DataType = u16;
}

#[doc = "Group A Region %s Access Control Register"]
pub type Mmpuaca = crate::RegValueT<Mmpuaca_SPEC>;

impl Mmpuaca {
    #[doc = "These bits are read as 0000000000000. The write value should be 0000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1fff, 1, 0, u16, u16, Mmpuaca_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1fff,1,0,u16,u16,Mmpuaca_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Write protection"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mmpuaca::Wp,
        mmpuaca::Wp,
        Mmpuaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mmpuaca::Wp,
            mmpuaca::Wp,
            Mmpuaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Read protection"]
    #[inline(always)]
    pub fn rp(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mmpuaca::Rp,
        mmpuaca::Rp,
        Mmpuaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mmpuaca::Rp,
            mmpuaca::Rp,
            Mmpuaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Region enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuaca::Enable,
        mmpuaca::Enable,
        Mmpuaca_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuaca::Enable,
            mmpuaca::Enable,
            Mmpuaca_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpuaca {
    #[inline(always)]
    fn default() -> Mmpuaca {
        <crate::RegValueT<Mmpuaca_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuaca {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        #[doc = "Write permission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Write protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rp_SPEC;
    pub type Rp = crate::EnumBitfieldStruct<u8, Rp_SPEC>;
    impl Rp {
        #[doc = "Read permission"]
        pub const _0: Self = Self::new(0);

        #[doc = "Read protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Group m Region n unit is disabled"]
        pub const _0: Self = Self::new(0);

        #[doc = "Group m Region n unit is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusa_SPEC;
impl crate::sealed::RegSpec for Mmpusa_SPEC {
    type DataType = u32;
}

#[doc = "Group A Region %s Start Address Register"]
pub type Mmpusa = crate::RegValueT<Mmpusa_SPEC>;

impl Mmpusa {
    #[doc = "Address where the region starts, for use in region determination.NOTE: The low-order 2 bits are fixed to 0."]
    #[inline(always)]
    pub fn mmpusa(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mmpusa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mmpusa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpusa {
    #[inline(always)]
    fn default() -> Mmpusa {
        <crate::RegValueT<Mmpusa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuea_SPEC;
impl crate::sealed::RegSpec for Mmpuea_SPEC {
    type DataType = u32;
}

#[doc = "Group A Region %s End Address Register"]
pub type Mmpuea = crate::RegValueT<Mmpuea_SPEC>;

impl Mmpuea {
    #[doc = "Region end address registerAddress where the region end, for use in region determination.NOTE: The low-order 2 bits are fixed to 1."]
    #[inline(always)]
    pub fn mmpuea(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Mmpuea_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Mmpuea_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuea {
    #[inline(always)]
    fn default() -> Mmpuea {
        <crate::RegValueT<Mmpuea_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpupta_SPEC;
impl crate::sealed::RegSpec for Mmpupta_SPEC {
    type DataType = u16;
}

#[doc = "Group A Protection of Register"]
pub type Mmpupta = crate::RegValueT<Mmpupta_SPEC>;

impl Mmpupta {
    #[doc = "Write Keyword The data written to these bits are not stored."]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        mmpupta::Key,
        mmpupta::Key,
        Mmpupta_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            mmpupta::Key,
            mmpupta::Key,
            Mmpupta_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Mmpupta_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Mmpupta_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Protection of register(MMPUSAn, MMPUEAn and MMPUACAn)"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpupta::Protect,
        mmpupta::Protect,
        Mmpupta_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpupta::Protect,
            mmpupta::Protect,
            Mmpupta_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mmpupta {
    #[inline(always)]
    fn default() -> Mmpupta {
        <crate::RegValueT<Mmpupta_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpupta {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Key_SPEC;
    pub type Key = crate::EnumBitfieldStruct<u8, Key_SPEC>;
    impl Key {
        #[doc = "Writing to the PROTECT bit is valid, when the KEY bits are written 0xA5."]
        pub const _0_X_A_5: Self = Self::new(165);

        #[doc = "Writing to the PROTECT bit is invalid."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "All Bus Master MPU Group A register writing is possible."]
        pub const _0: Self = Self::new(0);

        #[doc = "All Bus Master MPU Group A register writing is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
