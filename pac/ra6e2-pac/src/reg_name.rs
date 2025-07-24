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
// Generated from SVD 1.30.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:50:47 +0000

//! Contains perfect hash function that maps form raw addresses to
//! a string containing the names of all registers that point to an address.
//!
//! When using tracing feature to record accesses to registers, the exact
//! API path, though which a specific address was accessed gets lost.
//! This poses a problem when recorded register accesses contain accesses
//! to unexpected registers. [`reg_name_from_addr`] can be used to make
//! logs of raw register accesses more readable to humans by providing a list
//! of names of registers that alias a specific physical address.
//!
use phf::phf_map;

/// Get a &str name of a register given it's address.
pub fn reg_name_from_addr(addr: u64) -> Option<&'static &'static str> {
    REGISTER_NAMES.get(&addr)
}

static REGISTER_NAMES: phf::Map<u64, &'static str> = phf_map! {
  0x40000000u64 => "
      RMPU.mmpuoad(),
    ",
  0x40000004u64 => "
      RMPU.mmpuoadpt(),
    ",
  0x40000100u64 => "
      RMPU.mmpuendmac(),
    ",
  0x40000104u64 => "
      RMPU.mmpuenptdmac(),
    ",
  0x40000108u64 => "
      RMPU.mmpurptdmac(),
    ",
  0x4000010cu64 => "
      RMPU.mmpurptdmac_sec(),
    ",
  0x40000200u64 => "
      RMPU.mmpuacdmac()[0],
    ",
  0x40000210u64 => "
      RMPU.mmpuacdmac()[1],
    ",
  0x40000220u64 => "
      RMPU.mmpuacdmac()[2],
    ",
  0x40000230u64 => "
      RMPU.mmpuacdmac()[3],
    ",
  0x40000240u64 => "
      RMPU.mmpuacdmac()[4],
    ",
  0x40000250u64 => "
      RMPU.mmpuacdmac()[5],
    ",
  0x40000260u64 => "
      RMPU.mmpuacdmac()[6],
    ",
  0x40000270u64 => "
      RMPU.mmpuacdmac()[7],
    ",
  0x40000204u64 => "
      RMPU.mmpusdmac()[0],
    ",
  0x40000214u64 => "
      RMPU.mmpusdmac()[1],
    ",
  0x40000224u64 => "
      RMPU.mmpusdmac()[2],
    ",
  0x40000234u64 => "
      RMPU.mmpusdmac()[3],
    ",
  0x40000244u64 => "
      RMPU.mmpusdmac()[4],
    ",
  0x40000254u64 => "
      RMPU.mmpusdmac()[5],
    ",
  0x40000264u64 => "
      RMPU.mmpusdmac()[6],
    ",
  0x40000274u64 => "
      RMPU.mmpusdmac()[7],
    ",
  0x40000208u64 => "
      RMPU.mmpuedmac()[0],
    ",
  0x40000218u64 => "
      RMPU.mmpuedmac()[1],
    ",
  0x40000228u64 => "
      RMPU.mmpuedmac()[2],
    ",
  0x40000238u64 => "
      RMPU.mmpuedmac()[3],
    ",
  0x40000248u64 => "
      RMPU.mmpuedmac()[4],
    ",
  0x40000258u64 => "
      RMPU.mmpuedmac()[5],
    ",
  0x40000268u64 => "
      RMPU.mmpuedmac()[6],
    ",
  0x40000278u64 => "
      RMPU.mmpuedmac()[7],
    ",
  0x40000e00u64 => "
      TZF.tzfoad(),
    ",
  0x40000e04u64 => "
      TZF.tzfpt(),
    ",
  0x40002000u64 => "
      SRAM.parioad(),
    ",
  0x40002004u64 => "
      SRAM.sramprcr(),
    ",
  0x40002008u64 => "
      SRAM.sramwtsc(),
    ",
  0x4000200cu64 => "
      SRAM.sramprcr2(),
    ",
  0x400020c0u64 => "
      SRAM.eccmode(),
    ",
  0x400020c1u64 => "
      SRAM.ecc2sts(),
    ",
  0x400020c2u64 => "
      SRAM.ecc1stsen(),
    ",
  0x400020c3u64 => "
      SRAM.ecc1sts(),
    ",
  0x400020c4u64 => "
      SRAM.eccprcr(),
    ",
  0x400020d0u64 => "
      SRAM.eccprcr2(),
    ",
  0x400020d4u64 => "
      SRAM.eccetst(),
    ",
  0x400020d8u64 => "
      SRAM.eccoad(),
    ",
  0x40004100u64 => "
      BUS.busscntfhbiu(),
    ",
  0x40004104u64 => "
      BUS.busscntflbiu(),
    ",
  0x40004110u64 => "
      BUS.busscnts0biu(),
    ",
  0x40004120u64 => "
      BUS.busscntpsbiu(),
    ",
  0x40004130u64 => "
      BUS.busscntplbiu(),
    ",
  0x40004134u64 => "
      BUS.busscntphbiu(),
    ",
  0x40004140u64 => "
      BUS.busscnteqbiu(),
    ",
  0x40004800u64 => "
      BUS.buserradd()[0],
    ",
  0x40004810u64 => "
      BUS.buserradd()[1],
    ",
  0x40004820u64 => "
      BUS.buserradd()[2],
    ",
  0x40004804u64 => "
      BUS.buserrrw()[0],
    ",
  0x40004814u64 => "
      BUS.buserrrw()[1],
    ",
  0x40004824u64 => "
      BUS.buserrrw()[2],
    ",
  0x40004900u64 => "
      BUS.btzferradd()[0],
    ",
  0x40004910u64 => "
      BUS.btzferradd()[1],
    ",
  0x40004920u64 => "
      BUS.btzferradd()[2],
    ",
  0x40004904u64 => "
      BUS.btzferrrw()[0],
    ",
  0x40004914u64 => "
      BUS.btzferrrw()[1],
    ",
  0x40004924u64 => "
      BUS.btzferrrw()[2],
    ",
  0x40004a00u64 => "
      BUS.buserrstat()[0],
    ",
  0x40004a10u64 => "
      BUS.buserrstat()[1],
    ",
  0x40004a20u64 => "
      BUS.buserrstat()[2],
    ",
  0x40004a08u64 => "
      BUS.buserrclr()[0],
    ",
  0x40004a18u64 => "
      BUS.buserrclr()[1],
    ",
  0x40004a28u64 => "
      BUS.buserrclr()[2],
    ",
  0x40004a24u64 => "
      BUS.dmacdtcerrstat(),
    ",
  0x40004a2cu64 => "
      BUS.dmacdtcerrclr(),
    ",
  0x40005000u64 => "
      DMAC_0.dmsar(),
    ",
  0x40005004u64 => "
      DMAC_0.dmdar(),
    ",
  0x40005008u64 => "
      DMAC_0.dmcra(),
    ",
  0x4000500cu64 => "
      DMAC_0.dmcrb(),
    ",
  0x40005010u64 => "
      DMAC_0.dmtmd(),
    ",
  0x40005013u64 => "
      DMAC_0.dmint(),
    ",
  0x40005014u64 => "
      DMAC_0.dmamd(),
    ",
  0x40005018u64 => "
      DMAC_0.dmofr(),
    ",
  0x4000501cu64 => "
      DMAC_0.dmcnt(),
    ",
  0x4000501du64 => "
      DMAC_0.dmreq(),
    ",
  0x4000501eu64 => "
      DMAC_0.dmsts(),
    ",
  0x40005020u64 => "
      DMAC_0.dmsrr(),
    ",
  0x40005024u64 => "
      DMAC_0.dmdrr(),
    ",
  0x40005028u64 => "
      DMAC_0.dmsbs(),
    ",
  0x4000502cu64 => "
      DMAC_0.dmdbs(),
    ",
  0x40005200u64 => "
      DMA.dmast(),
    ",
  0x40005240u64 => "
      DMA.dmechr(),
    ",
  0x40005400u64 => "
      DTC.dtccr(),
    ",
  0x40005404u64 => "
      DTC.dtcvbr(),
    ",
  0x4000540cu64 => "
      DTC.dtcst(),
    ",
  0x4000540eu64 => "
      DTC.dtcsts(),
    ",
  0x40005410u64 => "
      DTC.dtccr_sec(),
    ",
  0x40005414u64 => "
      DTC.dtcvbr_sec(),
    ",
  0x40005420u64 => "
      DTC.dtevr(),
    ",
  0x40006000u64 => "
      ICU.irqcr()[0],
    ",
  0x40006001u64 => "
      ICU.irqcr()[1],
    ",
  0x40006002u64 => "
      ICU.irqcr()[2],
    ",
  0x40006003u64 => "
      ICU.irqcr()[3],
    ",
  0x40006004u64 => "
      ICU.irqcr()[4],
    ",
  0x40006005u64 => "
      ICU.irqcr()[5],
    ",
  0x40006006u64 => "
      ICU.irqcr()[6],
    ",
  0x40006007u64 => "
      ICU.irqcr()[7],
    ",
  0x40006008u64 => "
      ICU.irqcr()[8],
    ",
  0x40006009u64 => "
      ICU.irqcr()[9],
    ",
  0x4000600au64 => "
      ICU.irqcr()[10],
    ",
  0x4000600bu64 => "
      ICU.irqcr()[11],
    ",
  0x4000600cu64 => "
      ICU.irqcr()[12],
    ",
  0x4000600du64 => "
      ICU.irqcr()[13],
    ",
  0x4000600eu64 => "
      ICU.irqcr()[14],
    ",
  0x40006100u64 => "
      ICU.nmicr(),
    ",
  0x40006120u64 => "
      ICU.nmier(),
    ",
  0x40006130u64 => "
      ICU.nmiclr(),
    ",
  0x40006140u64 => "
      ICU.nmisr(),
    ",
  0x400061a0u64 => "
      ICU.wupen0(),
    ",
  0x400061a4u64 => "
      ICU.wupen1(),
    ",
  0x40006200u64 => "
      ICU.selsr0(),
    ",
  0x40006280u64 => "
      ICU.delsr()[0],
    ",
  0x40006284u64 => "
      ICU.delsr()[1],
    ",
  0x40006288u64 => "
      ICU.delsr()[2],
    ",
  0x4000628cu64 => "
      ICU.delsr()[3],
    ",
  0x40006290u64 => "
      ICU.delsr()[4],
    ",
  0x40006294u64 => "
      ICU.delsr()[5],
    ",
  0x40006298u64 => "
      ICU.delsr()[6],
    ",
  0x4000629cu64 => "
      ICU.delsr()[7],
    ",
  0x40006300u64 => "
      ICU.ielsr()[0],
    ",
  0x40006304u64 => "
      ICU.ielsr()[1],
    ",
  0x40006308u64 => "
      ICU.ielsr()[2],
    ",
  0x4000630cu64 => "
      ICU.ielsr()[3],
    ",
  0x40006310u64 => "
      ICU.ielsr()[4],
    ",
  0x40006314u64 => "
      ICU.ielsr()[5],
    ",
  0x40006318u64 => "
      ICU.ielsr()[6],
    ",
  0x4000631cu64 => "
      ICU.ielsr()[7],
    ",
  0x40006320u64 => "
      ICU.ielsr()[8],
    ",
  0x40006324u64 => "
      ICU.ielsr()[9],
    ",
  0x40006328u64 => "
      ICU.ielsr()[10],
    ",
  0x4000632cu64 => "
      ICU.ielsr()[11],
    ",
  0x40006330u64 => "
      ICU.ielsr()[12],
    ",
  0x40006334u64 => "
      ICU.ielsr()[13],
    ",
  0x40006338u64 => "
      ICU.ielsr()[14],
    ",
  0x4000633cu64 => "
      ICU.ielsr()[15],
    ",
  0x40006340u64 => "
      ICU.ielsr()[16],
    ",
  0x40006344u64 => "
      ICU.ielsr()[17],
    ",
  0x40006348u64 => "
      ICU.ielsr()[18],
    ",
  0x4000634cu64 => "
      ICU.ielsr()[19],
    ",
  0x40006350u64 => "
      ICU.ielsr()[20],
    ",
  0x40006354u64 => "
      ICU.ielsr()[21],
    ",
  0x40006358u64 => "
      ICU.ielsr()[22],
    ",
  0x4000635cu64 => "
      ICU.ielsr()[23],
    ",
  0x40006360u64 => "
      ICU.ielsr()[24],
    ",
  0x40006364u64 => "
      ICU.ielsr()[25],
    ",
  0x40006368u64 => "
      ICU.ielsr()[26],
    ",
  0x4000636cu64 => "
      ICU.ielsr()[27],
    ",
  0x40006370u64 => "
      ICU.ielsr()[28],
    ",
  0x40006374u64 => "
      ICU.ielsr()[29],
    ",
  0x40006378u64 => "
      ICU.ielsr()[30],
    ",
  0x4000637cu64 => "
      ICU.ielsr()[31],
    ",
  0x40006380u64 => "
      ICU.ielsr()[32],
    ",
  0x40006384u64 => "
      ICU.ielsr()[33],
    ",
  0x40006388u64 => "
      ICU.ielsr()[34],
    ",
  0x4000638cu64 => "
      ICU.ielsr()[35],
    ",
  0x40006390u64 => "
      ICU.ielsr()[36],
    ",
  0x40006394u64 => "
      ICU.ielsr()[37],
    ",
  0x40006398u64 => "
      ICU.ielsr()[38],
    ",
  0x4000639cu64 => "
      ICU.ielsr()[39],
    ",
  0x400063a0u64 => "
      ICU.ielsr()[40],
    ",
  0x400063a4u64 => "
      ICU.ielsr()[41],
    ",
  0x400063a8u64 => "
      ICU.ielsr()[42],
    ",
  0x400063acu64 => "
      ICU.ielsr()[43],
    ",
  0x400063b0u64 => "
      ICU.ielsr()[44],
    ",
  0x400063b4u64 => "
      ICU.ielsr()[45],
    ",
  0x400063b8u64 => "
      ICU.ielsr()[46],
    ",
  0x400063bcu64 => "
      ICU.ielsr()[47],
    ",
  0x400063c0u64 => "
      ICU.ielsr()[48],
    ",
  0x400063c4u64 => "
      ICU.ielsr()[49],
    ",
  0x400063c8u64 => "
      ICU.ielsr()[50],
    ",
  0x400063ccu64 => "
      ICU.ielsr()[51],
    ",
  0x400063d0u64 => "
      ICU.ielsr()[52],
    ",
  0x400063d4u64 => "
      ICU.ielsr()[53],
    ",
  0x400063d8u64 => "
      ICU.ielsr()[54],
    ",
  0x400063dcu64 => "
      ICU.ielsr()[55],
    ",
  0x400063e0u64 => "
      ICU.ielsr()[56],
    ",
  0x400063e4u64 => "
      ICU.ielsr()[57],
    ",
  0x400063e8u64 => "
      ICU.ielsr()[58],
    ",
  0x400063ecu64 => "
      ICU.ielsr()[59],
    ",
  0x400063f0u64 => "
      ICU.ielsr()[60],
    ",
  0x400063f4u64 => "
      ICU.ielsr()[61],
    ",
  0x400063f8u64 => "
      ICU.ielsr()[62],
    ",
  0x400063fcu64 => "
      ICU.ielsr()[63],
    ",
  0x40006400u64 => "
      ICU.ielsr()[64],
    ",
  0x40006404u64 => "
      ICU.ielsr()[65],
    ",
  0x40006408u64 => "
      ICU.ielsr()[66],
    ",
  0x4000640cu64 => "
      ICU.ielsr()[67],
    ",
  0x40006410u64 => "
      ICU.ielsr()[68],
    ",
  0x40006414u64 => "
      ICU.ielsr()[69],
    ",
  0x40006418u64 => "
      ICU.ielsr()[70],
    ",
  0x4000641cu64 => "
      ICU.ielsr()[71],
    ",
  0x40006420u64 => "
      ICU.ielsr()[72],
    ",
  0x40006424u64 => "
      ICU.ielsr()[73],
    ",
  0x40006428u64 => "
      ICU.ielsr()[74],
    ",
  0x4000642cu64 => "
      ICU.ielsr()[75],
    ",
  0x40006430u64 => "
      ICU.ielsr()[76],
    ",
  0x40006434u64 => "
      ICU.ielsr()[77],
    ",
  0x40006438u64 => "
      ICU.ielsr()[78],
    ",
  0x4000643cu64 => "
      ICU.ielsr()[79],
    ",
  0x40006440u64 => "
      ICU.ielsr()[80],
    ",
  0x40006444u64 => "
      ICU.ielsr()[81],
    ",
  0x40006448u64 => "
      ICU.ielsr()[82],
    ",
  0x4000644cu64 => "
      ICU.ielsr()[83],
    ",
  0x40006450u64 => "
      ICU.ielsr()[84],
    ",
  0x40006454u64 => "
      ICU.ielsr()[85],
    ",
  0x40006458u64 => "
      ICU.ielsr()[86],
    ",
  0x4000645cu64 => "
      ICU.ielsr()[87],
    ",
  0x40006460u64 => "
      ICU.ielsr()[88],
    ",
  0x40006464u64 => "
      ICU.ielsr()[89],
    ",
  0x40006468u64 => "
      ICU.ielsr()[90],
    ",
  0x4000646cu64 => "
      ICU.ielsr()[91],
    ",
  0x40006470u64 => "
      ICU.ielsr()[92],
    ",
  0x40006474u64 => "
      ICU.ielsr()[93],
    ",
  0x40006478u64 => "
      ICU.ielsr()[94],
    ",
  0x4000647cu64 => "
      ICU.ielsr()[95],
    ",
  0x40007000u64 => "
      CACHE.ccactl(),
    ",
  0x40007004u64 => "
      CACHE.ccafct(),
    ",
  0x40007008u64 => "
      CACHE.ccalcf(),
    ",
  0x40007040u64 => "
      CACHE.scactl(),
    ",
  0x40007044u64 => "
      CACHE.scafct(),
    ",
  0x40007048u64 => "
      CACHE.scalcf(),
    ",
  0x40007200u64 => "
      CACHE.capoad(),
    ",
  0x40007204u64 => "
      CACHE.caprcr(),
    ",
  0x40008000u64 => "
      CPSCU.csar(),
    ",
  0x40008010u64 => "
      CPSCU.sramsar(),
    ",
  0x40008014u64 => "
      CPSCU.stbramsar(),
    ",
  0x40008030u64 => "
      CPSCU.dtcsar(),
    ",
  0x40008034u64 => "
      CPSCU.dmacsar(),
    ",
  0x40008040u64 => "
      CPSCU.icusara(),
    ",
  0x40008044u64 => "
      CPSCU.icusarb(),
    ",
  0x40008048u64 => "
      CPSCU.icusarc(),
    ",
  0x4000804cu64 => "
      CPSCU.icusard(),
    ",
  0x40008054u64 => "
      CPSCU.icusarf(),
    ",
  0x40008070u64 => "
      CPSCU.icusarg(),
    ",
  0x40008074u64 => "
      CPSCU.icusarh(),
    ",
  0x40008078u64 => "
      CPSCU.icusari(),
    ",
  0x40008100u64 => "
      CPSCU.bussara(),
    ",
  0x40008104u64 => "
      CPSCU.bussarb(),
    ",
  0x40008130u64 => "
      CPSCU.mmpusara(),
    ",
  0x40008134u64 => "
      CPSCU.mmpusarb(),
    ",
  0x40008180u64 => "
      CPSCU.tzfsar(),
    ",
  0x400081b0u64 => "
      CPSCU.cpudsar(),
    ",
  0x4001b000u64 => "
      DBG.dbgstr(),
    ",
  0x4001b010u64 => "
      DBG.dbgstopcr(),
    ",
  0x4001c100u64 => "
      FCACHE.fcachee(),
    ",
  0x4001c104u64 => "
      FCACHE.fcacheiv(),
    ",
  0x4001c11cu64 => "
      FCACHE.flwt(),
    ",
  0x4001c140u64 => "
      FCACHE.fsar(),
    ",
  0x4001e00cu64 => "
      SYSC.sbycr(),
    ",
  0x4001e020u64 => "
      SYSC.sckdivcr(),
    ",
  0x4001e026u64 => "
      SYSC.sckscr(),
    ",
  0x4001e028u64 => "
      SYSC.pllccr(),
    ",
  0x4001e02au64 => "
      SYSC.pllcr(),
    ",
  0x4001e032u64 => "
      SYSC.mosccr(),
    ",
  0x4001e036u64 => "
      SYSC.hococr(),
    ",
  0x4001e037u64 => "
      SYSC.hococr2(),
    ",
  0x4001e038u64 => "
      SYSC.mococr(),
    ",
  0x4001e039u64 => "
      SYSC.fllcr1(),
    ",
  0x4001e03au64 => "
      SYSC.fllcr2(),
    ",
  0x4001e03cu64 => "
      SYSC.oscsf(),
    ",
  0x4001e03eu64 => "
      SYSC.ckocr(),
    ",
  0x4001e040u64 => "
      SYSC.ostdcr(),
    ",
  0x4001e041u64 => "
      SYSC.ostdsr(),
    ",
  0x4001e061u64 => "
      SYSC.mocoutcr(),
    ",
  0x4001e062u64 => "
      SYSC.hocoutcr(),
    ",
  0x4001e06cu64 => "
      SYSC.usbckdivcr(),
    ",
  0x4001e06eu64 => "
      SYSC.canfdckdivcr(),
    ",
  0x4001e070u64 => "
      SYSC.cecckdivcr(),
    ",
  0x4001e071u64 => "
      SYSC.i3cckdivcr(),
    ",
  0x4001e074u64 => "
      SYSC.usbckcr(),
    ",
  0x4001e076u64 => "
      SYSC.canfdckcr(),
    ",
  0x4001e078u64 => "
      SYSC.cecckcr(),
    ",
  0x4001e079u64 => "
      SYSC.i3cckcr(),
    ",
  0x4001e092u64 => "
      SYSC.snzcr(),
    ",
  0x4001e094u64 => "
      SYSC.snzedcr0(),
    ",
  0x4001e098u64 => "
      SYSC.snzreqcr0(),
    ",
  0x4001e0a0u64 => "
      SYSC.opccr(),
    ",
  0x4001e0a2u64 => "
      SYSC.moscwtcr(),
    ",
  0x4001e0aau64 => "
      SYSC.sopccr(),
    ",
  0x4001e0c0u64 => "
      SYSC.rstsr1(),
    ",
  0x4001e0e0u64 => "
      SYSC.lvd1cr1(),
    ",
  0x4001e0e1u64 => "
      SYSC.lvd1sr(),
    ",
  0x4001e0e2u64 => "
      SYSC.lvd2cr1(),
    ",
  0x4001e0e3u64 => "
      SYSC.lvd2sr(),
    ",
  0x4001e3c0u64 => "
      SYSC.cgfsar(),
    ",
  0x4001e3c4u64 => "
      SYSC.rstsar(),
    ",
  0x4001e3c8u64 => "
      SYSC.lpmsar(),
    ",
  0x4001e3ccu64 => "
      SYSC.lvdsar(),
    ",
  0x4001e3e0u64 => "
      SYSC.dpfsar(),
    ",
  0x4001e3feu64 => "
      SYSC.prcr(),
    ",
  0x4001e400u64 => "
      SYSC.dpsbycr(),
    ",
  0x4001e401u64 => "
      SYSC.dpswcr(),
    ",
  0x4001e402u64 => "
      SYSC.dpsier0(),
    ",
  0x4001e403u64 => "
      SYSC.dpsier1(),
    ",
  0x4001e404u64 => "
      SYSC.dpsier2(),
    ",
  0x4001e405u64 => "
      SYSC.dpsier3(),
    ",
  0x4001e406u64 => "
      SYSC.dpsifr0(),
    ",
  0x4001e407u64 => "
      SYSC.dpsifr1(),
    ",
  0x4001e408u64 => "
      SYSC.dpsifr2(),
    ",
  0x4001e409u64 => "
      SYSC.dpsifr3(),
    ",
  0x4001e40au64 => "
      SYSC.dpsiegr0(),
    ",
  0x4001e40bu64 => "
      SYSC.dpsiegr1(),
    ",
  0x4001e40cu64 => "
      SYSC.dpsiegr2(),
    ",
  0x4001e40eu64 => "
      SYSC.syocdcr(),
    ",
  0x4001e410u64 => "
      SYSC.rstsr0(),
    ",
  0x4001e411u64 => "
      SYSC.rstsr2(),
    ",
  0x4001e413u64 => "
      SYSC.momcr(),
    ",
  0x4001e416u64 => "
      SYSC.fwepror(),
    ",
  0x4001e417u64 => "
      SYSC.lvd1cmpcr(),
    ",
  0x4001e418u64 => "
      SYSC.lvd2cmpcr(),
    ",
  0x4001e41au64 => "
      SYSC.lvd1cr0(),
    ",
  0x4001e41bu64 => "
      SYSC.lvd2cr0(),
    ",
  0x4001e480u64 => "
      SYSC.sosccr(),
    ",
  0x4001e481u64 => "
      SYSC.somcr(),
    ",
  0x4001e490u64 => "
      SYSC.lococr(),
    ",
  0x4001e492u64 => "
      SYSC.locoutcr(),
    ",
  0x40080000u64 => "
      PORT_0.pcntr1(),
      PORT_0.podr(),
    ",
  0x40080002u64 => "
      PORT_0.pdr(),
    ",
  0x40080004u64 => "
      PORT_0.pcntr2(),
    ",
  0x40080006u64 => "
      PORT_0.pidr(),
    ",
  0x40080008u64 => "
      PORT_0.pcntr3(),
      PORT_0.porr(),
    ",
  0x4008000au64 => "
      PORT_0.posr(),
    ",
  0x40080020u64 => "
      PORT_1.pcntr1(),
      PORT_1.podr(),
    ",
  0x40080022u64 => "
      PORT_1.pdr(),
    ",
  0x40080024u64 => "
      PORT_1.pcntr2(),
      PORT_1.eidr(),
    ",
  0x40080026u64 => "
      PORT_1.pidr(),
    ",
  0x40080028u64 => "
      PORT_1.pcntr3(),
      PORT_1.porr(),
    ",
  0x4008002au64 => "
      PORT_1.posr(),
    ",
  0x4008002cu64 => "
      PORT_1.pcntr4(),
      PORT_1.eorr(),
    ",
  0x4008002eu64 => "
      PORT_1.eosr(),
    ",
  0x4008080cu64 => "
      PFS.p00pfs()[3],
    ",
  0x4008080eu64 => "
      PFS.p00pfs_ha()[3],
    ",
  0x4008080fu64 => "
      PFS.p00pfs_by()[3],
    ",
  0x40080810u64 => "
      PFS.p00pfs()[0],
    ",
  0x40080814u64 => "
      PFS.p00pfs()[1],
    ",
  0x40080818u64 => "
      PFS.p00pfs()[2],
    ",
  0x40080812u64 => "
      PFS.p00pfs_ha()[0],
    ",
  0x40080816u64 => "
      PFS.p00pfs_ha()[1],
    ",
  0x4008081au64 => "
      PFS.p00pfs_ha()[2],
    ",
  0x40080813u64 => "
      PFS.p00pfs_by()[0],
    ",
  0x40080817u64 => "
      PFS.p00pfs_by()[1],
    ",
  0x4008081bu64 => "
      PFS.p00pfs_by()[2],
    ",
  0x40080820u64 => "
      PFS.p008pfs(),
    ",
  0x40080822u64 => "
      PFS.p008pfs_ha(),
    ",
  0x40080823u64 => "
      PFS.p008pfs_by(),
    ",
  0x40080834u64 => "
      PFS.p0pfs()[0],
    ",
  0x40080838u64 => "
      PFS.p0pfs()[1],
    ",
  0x4008083cu64 => "
      PFS.p0pfs()[2],
    ",
  0x40080836u64 => "
      PFS.p0pfs_ha()[0],
    ",
  0x4008083au64 => "
      PFS.p0pfs_ha()[1],
    ",
  0x4008083eu64 => "
      PFS.p0pfs_ha()[2],
    ",
  0x40080837u64 => "
      PFS.p0pfs_by()[0],
    ",
  0x4008083bu64 => "
      PFS.p0pfs_by()[1],
    ",
  0x4008083fu64 => "
      PFS.p0pfs_by()[2],
    ",
  0x40080848u64 => "
      PFS.p10pfs()[0],
    ",
  0x4008084cu64 => "
      PFS.p10pfs()[1],
    ",
  0x40080850u64 => "
      PFS.p10pfs()[2],
    ",
  0x40080854u64 => "
      PFS.p10pfs()[3],
    ",
  0x40080858u64 => "
      PFS.p10pfs()[4],
    ",
  0x4008085cu64 => "
      PFS.p10pfs()[5],
    ",
  0x40080860u64 => "
      PFS.p10pfs()[6],
    ",
  0x40080864u64 => "
      PFS.p10pfs()[7],
    ",
  0x4008084au64 => "
      PFS.p10pfs_ha()[0],
    ",
  0x4008084eu64 => "
      PFS.p10pfs_ha()[1],
    ",
  0x40080852u64 => "
      PFS.p10pfs_ha()[2],
    ",
  0x40080856u64 => "
      PFS.p10pfs_ha()[3],
    ",
  0x4008085au64 => "
      PFS.p10pfs_ha()[4],
    ",
  0x4008085eu64 => "
      PFS.p10pfs_ha()[5],
    ",
  0x40080862u64 => "
      PFS.p10pfs_ha()[6],
    ",
  0x40080866u64 => "
      PFS.p10pfs_ha()[7],
    ",
  0x4008084bu64 => "
      PFS.p10pfs_by()[0],
    ",
  0x4008084fu64 => "
      PFS.p10pfs_by()[1],
    ",
  0x40080853u64 => "
      PFS.p10pfs_by()[2],
    ",
  0x40080857u64 => "
      PFS.p10pfs_by()[3],
    ",
  0x4008085bu64 => "
      PFS.p10pfs_by()[4],
    ",
  0x4008085fu64 => "
      PFS.p10pfs_by()[5],
    ",
  0x40080863u64 => "
      PFS.p10pfs_by()[6],
    ",
  0x40080867u64 => "
      PFS.p10pfs_by()[7],
    ",
  0x40080868u64 => "
      PFS.p1pfs()[0],
    ",
  0x4008086cu64 => "
      PFS.p1pfs()[1],
    ",
  0x40080870u64 => "
      PFS.p1pfs()[2],
    ",
  0x40080874u64 => "
      PFS.p1pfs()[3],
    ",
  0x4008086au64 => "
      PFS.p1pfs_ha()[0],
    ",
  0x4008086eu64 => "
      PFS.p1pfs_ha()[1],
    ",
  0x40080872u64 => "
      PFS.p1pfs_ha()[2],
    ",
  0x40080876u64 => "
      PFS.p1pfs_ha()[3],
    ",
  0x4008086bu64 => "
      PFS.p1pfs_by()[0],
    ",
  0x4008086fu64 => "
      PFS.p1pfs_by()[1],
    ",
  0x40080873u64 => "
      PFS.p1pfs_by()[2],
    ",
  0x40080877u64 => "
      PFS.p1pfs_by()[3],
    ",
  0x40080880u64 => "
      PFS.p200pfs(),
    ",
  0x40080882u64 => "
      PFS.p200pfs_ha(),
    ",
  0x40080883u64 => "
      PFS.p200pfs_by(),
    ",
  0x40080884u64 => "
      PFS.p201pfs(),
    ",
  0x40080886u64 => "
      PFS.p201pfs_ha(),
    ",
  0x40080887u64 => "
      PFS.p201pfs_by(),
    ",
  0x40080894u64 => "
      PFS.p20pfs()[0],
    ",
  0x40080898u64 => "
      PFS.p20pfs()[1],
    ",
  0x4008089cu64 => "
      PFS.p20pfs()[2],
    ",
  0x400808a0u64 => "
      PFS.p20pfs()[3],
    ",
  0x40080896u64 => "
      PFS.p20pfs_ha()[0],
    ",
  0x4008089au64 => "
      PFS.p20pfs_ha()[1],
    ",
  0x4008089eu64 => "
      PFS.p20pfs_ha()[2],
    ",
  0x400808a2u64 => "
      PFS.p20pfs_ha()[3],
    ",
  0x40080897u64 => "
      PFS.p20pfs_by()[0],
    ",
  0x4008089bu64 => "
      PFS.p20pfs_by()[1],
    ",
  0x4008089fu64 => "
      PFS.p20pfs_by()[2],
    ",
  0x400808a3u64 => "
      PFS.p20pfs_by()[3],
    ",
  0x400808b0u64 => "
      PFS.p2pfs()[0],
    ",
  0x400808b4u64 => "
      PFS.p2pfs()[1],
    ",
  0x400808b2u64 => "
      PFS.p2pfs_ha()[0],
    ",
  0x400808b6u64 => "
      PFS.p2pfs_ha()[1],
    ",
  0x400808b3u64 => "
      PFS.p2pfs_by()[0],
    ",
  0x400808b7u64 => "
      PFS.p2pfs_by()[1],
    ",
  0x400808c0u64 => "
      PFS.p300pfs(),
    ",
  0x400808c2u64 => "
      PFS.p300pfs_ha(),
    ",
  0x400808c3u64 => "
      PFS.p300pfs_by(),
    ",
  0x400808c4u64 => "
      PFS.p30pfs()[0],
    ",
  0x400808c8u64 => "
      PFS.p30pfs()[1],
    ",
  0x400808ccu64 => "
      PFS.p30pfs()[2],
    ",
  0x400808d0u64 => "
      PFS.p30pfs()[3],
    ",
  0x400808c6u64 => "
      PFS.p30pfs_ha()[0],
    ",
  0x400808cau64 => "
      PFS.p30pfs_ha()[1],
    ",
  0x400808ceu64 => "
      PFS.p30pfs_ha()[2],
    ",
  0x400808d2u64 => "
      PFS.p30pfs_ha()[3],
    ",
  0x400808c7u64 => "
      PFS.p30pfs_by()[0],
    ",
  0x400808cbu64 => "
      PFS.p30pfs_by()[1],
    ",
  0x400808cfu64 => "
      PFS.p30pfs_by()[2],
    ",
  0x400808d3u64 => "
      PFS.p30pfs_by()[3],
    ",
  0x4008090cu64 => "
      PFS.p40pfs()[3],
    ",
  0x4008090eu64 => "
      PFS.p40pfs_ha()[3],
    ",
  0x4008090fu64 => "
      PFS.p40pfs_by()[3],
    ",
  0x4008091cu64 => "
      PFS.p40pfs()[0],
    ",
  0x40080920u64 => "
      PFS.p40pfs()[1],
    ",
  0x40080924u64 => "
      PFS.p40pfs()[2],
    ",
  0x4008091eu64 => "
      PFS.p40pfs_ha()[0],
    ",
  0x40080922u64 => "
      PFS.p40pfs_ha()[1],
    ",
  0x40080926u64 => "
      PFS.p40pfs_ha()[2],
    ",
  0x4008091fu64 => "
      PFS.p40pfs_by()[0],
    ",
  0x40080923u64 => "
      PFS.p40pfs_by()[1],
    ",
  0x40080927u64 => "
      PFS.p40pfs_by()[2],
    ",
  0x40080928u64 => "
      PFS.p4pfs()[0],
    ",
  0x4008092cu64 => "
      PFS.p4pfs()[1],
    ",
  0x4008092au64 => "
      PFS.p4pfs_ha()[0],
    ",
  0x4008092eu64 => "
      PFS.p4pfs_ha()[1],
    ",
  0x4008092bu64 => "
      PFS.p4pfs_by()[0],
    ",
  0x4008092fu64 => "
      PFS.p4pfs_by()[1],
    ",
  0x40080940u64 => "
      PFS.p500pfs(),
    ",
  0x40080942u64 => "
      PFS.p500pfs_ha(),
    ",
  0x40080943u64 => "
      PFS.p500pfs_by(),
    ",
  0x40080a38u64 => "
      PFS.p80pfs()[0],
    ",
  0x40080a3cu64 => "
      PFS.p80pfs()[1],
    ",
  0x40080a3au64 => "
      PFS.p80pfs_ha()[0],
    ",
  0x40080a3eu64 => "
      PFS.p80pfs_ha()[1],
    ",
  0x40080a3bu64 => "
      PFS.p80pfs_by()[0],
    ",
  0x40080a3fu64 => "
      PFS.p80pfs_by()[1],
    ",
  0x40080d03u64 => "
      PFS.pwpr(),
    ",
  0x40080d05u64 => "
      PFS.pwprs(),
    ",
  0x40080d0cu64 => "
      PFS.pfi3c(),
    ",
  0x40080d10u64 => "
      PFS.psar()[0],
    ",
  0x40080d12u64 => "
      PFS.psar()[1],
    ",
  0x40080d14u64 => "
      PFS.psar()[2],
    ",
  0x40080d16u64 => "
      PFS.psar()[3],
    ",
  0x40080d18u64 => "
      PFS.psar()[4],
    ",
  0x40080d1au64 => "
      PFS.psar()[5],
    ",
  0x40080d20u64 => "
      PFS.p8sar(),
    ",
  0x40082000u64 => "
      ELC.elcr(),
    ",
  0x40082002u64 => "
      ELC.elsegr()[0],
    ",
  0x40082004u64 => "
      ELC.elsegr()[1],
    ",
  0x40082028u64 => "
      ELC.elsr()[6],
    ",
  0x4008202cu64 => "
      ELC.elsr()[7],
    ",
  0x40082030u64 => "
      ELC.elsr()[8],
    ",
  0x40082034u64 => "
      ELC.elsr()[9],
    ",
  0x40082040u64 => "
      ELC.elsr()[0],
    ",
  0x40082044u64 => "
      ELC.elsr()[1],
    ",
  0x40082048u64 => "
      ELC.elsr()[2],
    ",
  0x4008204cu64 => "
      ELC.elsr()[3],
    ",
  0x40082050u64 => "
      ELC.elsr()[4],
    ",
  0x40082054u64 => "
      ELC.elsr()[5],
    ",
  0x4008206cu64 => "
      ELC.elsr23(),
    ",
  0x40082074u64 => "
      ELC.elcsara(),
    ",
  0x40082078u64 => "
      ELC.elcsarb(),
    ",
  0x4008207cu64 => "
      ELC.elcsarc(),
    ",
  0x40083000u64 => "
      RTC.r64cnt(),
    ",
  0x40083002u64 => "
      RTC.bcnt()[0],
      RTC.rseccnt(),
    ",
  0x40083004u64 => "
      RTC.bcnt()[1],
      RTC.rmincnt(),
    ",
  0x40083006u64 => "
      RTC.bcnt()[2],
      RTC.rhrcnt(),
    ",
  0x40083008u64 => "
      RTC.bcnt()[3],
      RTC.rwkcnt(),
    ",
  0x4008300au64 => "
      RTC.rdaycnt(),
    ",
  0x4008300cu64 => "
      RTC.rmoncnt(),
    ",
  0x4008300eu64 => "
      RTC.ryrcnt(),
    ",
  0x40083010u64 => "
      RTC.bcntar()[0],
      RTC.rsecar(),
    ",
  0x40083012u64 => "
      RTC.bcntar()[1],
      RTC.rminar(),
    ",
  0x40083014u64 => "
      RTC.bcntar()[2],
      RTC.rhrar(),
    ",
  0x40083016u64 => "
      RTC.bcntar()[3],
      RTC.rwkar(),
    ",
  0x40083018u64 => "
      RTC.bcntaer()[0],
      RTC.rdayar(),
    ",
  0x4008301au64 => "
      RTC.bcntaer()[1],
      RTC.rmonar(),
    ",
  0x4008301cu64 => "
      RTC.bcnt2aer(),
      RTC.ryrar(),
    ",
  0x4008301eu64 => "
      RTC.bcnt3aer(),
      RTC.ryraren(),
    ",
  0x40083022u64 => "
      RTC.rcr1(),
    ",
  0x40083024u64 => "
      RTC.rcr2(),
      RTC.rcr2_bcnt(),
    ",
  0x40083028u64 => "
      RTC.rcr4(),
    ",
  0x4008302au64 => "
      RTC.rfrh(),
    ",
  0x4008302cu64 => "
      RTC.rfrl(),
    ",
  0x4008302eu64 => "
      RTC.radj(),
    ",
  0x40083040u64 => "
      RTC.rtccr()[0],
    ",
  0x40083042u64 => "
      RTC.rtccr()[1],
    ",
  0x40083052u64 => "
      RTC.bcnt0cp()[0],
      RTC.rseccp()[0],
    ",
  0x40083062u64 => "
      RTC.bcnt0cp()[1],
      RTC.rseccp()[1],
    ",
  0x40083054u64 => "
      RTC.bcnt1cp()[0],
      RTC.rmincp()[0],
    ",
  0x40083064u64 => "
      RTC.bcnt1cp()[1],
      RTC.rmincp()[1],
    ",
  0x40083056u64 => "
      RTC.bcnt2cp()[0],
      RTC.rhrcp()[0],
    ",
  0x40083066u64 => "
      RTC.bcnt2cp()[1],
      RTC.rhrcp()[1],
    ",
  0x4008305au64 => "
      RTC.bcnt3cp()[0],
      RTC.rdaycp()[0],
    ",
  0x4008306au64 => "
      RTC.bcnt3cp()[1],
      RTC.rdaycp()[1],
    ",
  0x4008305cu64 => "
      RTC.rmoncp()[0],
    ",
  0x4008306cu64 => "
      RTC.rmoncp()[1],
    ",
  0x40083200u64 => "
      IWDT.iwdtrr(),
    ",
  0x40083204u64 => "
      IWDT.iwdtsr(),
    ",
  0x40083400u64 => "
      WDT.wdtrr(),
    ",
  0x40083402u64 => "
      WDT.wdtcr(),
    ",
  0x40083404u64 => "
      WDT.wdtsr(),
    ",
  0x40083406u64 => "
      WDT.wdtrcr(),
    ",
  0x40083408u64 => "
      WDT.wdtcstpr(),
    ",
  0x40083600u64 => "
      CAC.cacr0(),
    ",
  0x40083601u64 => "
      CAC.cacr1(),
    ",
  0x40083602u64 => "
      CAC.cacr2(),
    ",
  0x40083603u64 => "
      CAC.caicr(),
    ",
  0x40083604u64 => "
      CAC.castr(),
    ",
  0x40083606u64 => "
      CAC.caulvr(),
    ",
  0x40083608u64 => "
      CAC.callvr(),
    ",
  0x4008360au64 => "
      CAC.cacntbr(),
    ",
  0x40084000u64 => "
      MSTP.mstpcra(),
    ",
  0x40084004u64 => "
      MSTP.mstpcrb(),
    ",
  0x40084008u64 => "
      MSTP.mstpcrc(),
    ",
  0x4008400cu64 => "
      MSTP.mstpcrd(),
    ",
  0x40084010u64 => "
      MSTP.mstpcre(),
    ",
  0x4008a000u64 => "
      POEG.poegga(),
    ",
  0x4008a100u64 => "
      POEG.poeggb(),
    ",
  0x4008a200u64 => "
      POEG.poeggc(),
    ",
  0x4008a300u64 => "
      POEG.poeggd(),
    ",
  0x40090000u64 => "
      USBFS.syscfg(),
    ",
  0x40090004u64 => "
      USBFS.syssts0(),
    ",
  0x40090008u64 => "
      USBFS.dvstctr0(),
    ",
  0x40090014u64 => "
      USBFS.cfifo(),
      USBFS.cfifol(),
    ",
  0x40090020u64 => "
      USBFS.cfifosel(),
    ",
  0x40090022u64 => "
      USBFS.cfifoctr(),
    ",
  0x40090030u64 => "
      USBFS.intenb0(),
    ",
  0x40090036u64 => "
      USBFS.brdyenb(),
    ",
  0x40090038u64 => "
      USBFS.nrdyenb(),
    ",
  0x4009003au64 => "
      USBFS.bempenb(),
    ",
  0x4009003cu64 => "
      USBFS.sofcfg(),
    ",
  0x40090040u64 => "
      USBFS.intsts0(),
    ",
  0x40090046u64 => "
      USBFS.brdysts(),
    ",
  0x40090048u64 => "
      USBFS.nrdysts(),
    ",
  0x4009004au64 => "
      USBFS.bempsts(),
    ",
  0x4009004cu64 => "
      USBFS.frmnum(),
    ",
  0x4009004eu64 => "
      USBFS.dvchgr(),
    ",
  0x40090050u64 => "
      USBFS.usbaddr(),
    ",
  0x40090054u64 => "
      USBFS.usbreq(),
    ",
  0x40090056u64 => "
      USBFS.usbval(),
    ",
  0x40090058u64 => "
      USBFS.usbindx(),
    ",
  0x4009005au64 => "
      USBFS.usbleng(),
    ",
  0x4009005cu64 => "
      USBFS.dcpcfg(),
    ",
  0x4009005eu64 => "
      USBFS.dcpmaxp(),
    ",
  0x40090060u64 => "
      USBFS.dcpctr(),
    ",
  0x40090064u64 => "
      USBFS.pipesel(),
    ",
  0x40090068u64 => "
      USBFS.pipecfg(),
    ",
  0x4009006cu64 => "
      USBFS.pipemaxp(),
    ",
  0x4009007au64 => "
      USBFS.pipectr()[0],
    ",
  0x4009007cu64 => "
      USBFS.pipectr()[1],
    ",
  0x4009009cu64 => "
      USBFS.pipetre()[0],
    ",
  0x400900a0u64 => "
      USBFS.pipetre()[1],
    ",
  0x4009009eu64 => "
      USBFS.pipetrn()[0],
    ",
  0x400900a2u64 => "
      USBFS.pipetrn()[1],
    ",
  0x400900f4u64 => "
      USBFS.physectrl(),
    ",
  0x40090400u64 => "
      USBFS.dpusr0r(),
    ",
  0x40090404u64 => "
      USBFS.dpusr1r(),
    ",
  0x4009d000u64 => "
      SSIE_0.ssicr(),
    ",
  0x4009d004u64 => "
      SSIE_0.ssisr(),
    ",
  0x4009d010u64 => "
      SSIE_0.ssifcr(),
    ",
  0x4009d014u64 => "
      SSIE_0.ssifsr(),
    ",
  0x4009d018u64 => "
      SSIE_0.ssiftdr(),
    ",
  0x4009d01cu64 => "
      SSIE_0.ssifrdr(),
    ",
  0x4009d020u64 => "
      SSIE_0.ssiofr(),
    ",
  0x4009d024u64 => "
      SSIE_0.ssiscr(),
    ",
  0x400ac000u64 => "
      CEC.cadr(),
    ",
  0x400ac002u64 => "
      CEC.cecctl1(),
    ",
  0x400ac004u64 => "
      CEC.statb(),
    ",
  0x400ac006u64 => "
      CEC.statl(),
    ",
  0x400ac008u64 => "
      CEC.lgc0l(),
    ",
  0x400ac00au64 => "
      CEC.lgc1l(),
    ",
  0x400ac00cu64 => "
      CEC.datb(),
    ",
  0x400ac00eu64 => "
      CEC.nomt(),
    ",
  0x400ac010u64 => "
      CEC.statll(),
    ",
  0x400ac012u64 => "
      CEC.statlh(),
    ",
  0x400ac014u64 => "
      CEC.statbl(),
    ",
  0x400ac016u64 => "
      CEC.statbh(),
    ",
  0x400ac018u64 => "
      CEC.lgc0ll(),
    ",
  0x400ac01au64 => "
      CEC.lgc0lh(),
    ",
  0x400ac01cu64 => "
      CEC.lgc1ll(),
    ",
  0x400ac01eu64 => "
      CEC.lgc1lh(),
    ",
  0x400ac020u64 => "
      CEC.datbl(),
    ",
  0x400ac022u64 => "
      CEC.datbh(),
    ",
  0x400ac024u64 => "
      CEC.nomp(),
    ",
  0x400ac028u64 => "
      CEC.cecexmd(),
    ",
  0x400ac02au64 => "
      CEC.cecexmon(),
    ",
  0x400ac040u64 => "
      CEC.ctxd(),
    ",
  0x400ac041u64 => "
      CEC.crxd(),
    ",
  0x400ac042u64 => "
      CEC.ceces(),
    ",
  0x400ac043u64 => "
      CEC.cecs(),
    ",
  0x400ac044u64 => "
      CEC.cecfc(),
    ",
  0x400ac045u64 => "
      CEC.cecctl0(),
    ",
  0x400b0000u64 => "
      CANFD_B.cfdc0ncfg(),
    ",
  0x400b0004u64 => "
      CANFD_B.cfdc0ctr(),
    ",
  0x400b0008u64 => "
      CANFD_B.cfdc0sts(),
    ",
  0x400b000cu64 => "
      CANFD_B.cfdc0erfl(),
    ",
  0x400b0014u64 => "
      CANFD_B.cfdgcfg(),
    ",
  0x400b0018u64 => "
      CANFD_B.cfdgctr(),
    ",
  0x400b001cu64 => "
      CANFD_B.cfdgsts(),
    ",
  0x400b0020u64 => "
      CANFD_B.cfdgerfl(),
    ",
  0x400b0024u64 => "
      CANFD_B.cfdgtsc(),
    ",
  0x400b0028u64 => "
      CANFD_B.cfdgaflectr(),
    ",
  0x400b002cu64 => "
      CANFD_B.cfdgaflcfg(),
    ",
  0x400b0030u64 => "
      CANFD_B.cfdrmnb(),
    ",
  0x400b0034u64 => "
      CANFD_B.cfdrmnd(),
    ",
  0x400b0038u64 => "
      CANFD_B.cfdrmiec(),
    ",
  0x400b003cu64 => "
      CANFD_B.cfdrfcc()[0],
    ",
  0x400b0040u64 => "
      CANFD_B.cfdrfcc()[1],
    ",
  0x400b0044u64 => "
      CANFD_B.cfdrfsts()[0],
    ",
  0x400b0048u64 => "
      CANFD_B.cfdrfsts()[1],
    ",
  0x400b004cu64 => "
      CANFD_B.cfdrfpctr()[0],
    ",
  0x400b0050u64 => "
      CANFD_B.cfdrfpctr()[1],
    ",
  0x400b0054u64 => "
      CANFD_B.cfdcfcc(),
    ",
  0x400b0058u64 => "
      CANFD_B.cfdcfsts(),
    ",
  0x400b005cu64 => "
      CANFD_B.cfdcfpctr(),
    ",
  0x400b0060u64 => "
      CANFD_B.cfdfests(),
    ",
  0x400b0064u64 => "
      CANFD_B.cfdffsts(),
    ",
  0x400b0068u64 => "
      CANFD_B.cfdfmsts(),
    ",
  0x400b006cu64 => "
      CANFD_B.cfdrfists(),
    ",
  0x400b0070u64 => "
      CANFD_B.cfdtmc()[0],
    ",
  0x400b0071u64 => "
      CANFD_B.cfdtmc()[1],
    ",
  0x400b0072u64 => "
      CANFD_B.cfdtmc()[2],
    ",
  0x400b0073u64 => "
      CANFD_B.cfdtmc()[3],
    ",
  0x400b0074u64 => "
      CANFD_B.cfdtmsts()[0],
    ",
  0x400b0075u64 => "
      CANFD_B.cfdtmsts()[1],
    ",
  0x400b0076u64 => "
      CANFD_B.cfdtmsts()[2],
    ",
  0x400b0077u64 => "
      CANFD_B.cfdtmsts()[3],
    ",
  0x400b0078u64 => "
      CANFD_B.cfdtmtrsts(),
    ",
  0x400b007cu64 => "
      CANFD_B.cfdtmtarsts(),
    ",
  0x400b0080u64 => "
      CANFD_B.cfdtmtcsts(),
    ",
  0x400b0084u64 => "
      CANFD_B.cfdtmtasts(),
    ",
  0x400b0088u64 => "
      CANFD_B.cfdtmiec(),
    ",
  0x400b008cu64 => "
      CANFD_B.cfdtxqcc(),
    ",
  0x400b0090u64 => "
      CANFD_B.cfdtxqsts(),
    ",
  0x400b0094u64 => "
      CANFD_B.cfdtxqpctr(),
    ",
  0x400b0098u64 => "
      CANFD_B.cfdthlcc(),
    ",
  0x400b009cu64 => "
      CANFD_B.cfdthlsts(),
    ",
  0x400b00a0u64 => "
      CANFD_B.cfdthlpctr(),
    ",
  0x400b00a4u64 => "
      CANFD_B.cfdgtintsts(),
    ",
  0x400b00a8u64 => "
      CANFD_B.cfdgtstcfg(),
    ",
  0x400b00acu64 => "
      CANFD_B.cfdgtstctr(),
    ",
  0x400b00b0u64 => "
      CANFD_B.cfdgfdcfg(),
    ",
  0x400b00b8u64 => "
      CANFD_B.cfdglockk(),
    ",
  0x400b00c0u64 => "
      CANFD_B.cfdgaflignent(),
    ",
  0x400b00c4u64 => "
      CANFD_B.cfdgaflignctr(),
    ",
  0x400b00c8u64 => "
      CANFD_B.cfdcdtct(),
    ",
  0x400b00ccu64 => "
      CANFD_B.cfdcdtsts(),
    ",
  0x400b00d8u64 => "
      CANFD_B.cfdgrstc(),
    ",
  0x400b0100u64 => "
      CANFD_B.cfdc0dcfg(),
    ",
  0x400b0104u64 => "
      CANFD_B.cfdc0fdcfg(),
    ",
  0x400b0108u64 => "
      CANFD_B.cfdc0fdctr(),
    ",
  0x400b010cu64 => "
      CANFD_B.cfdc0fdsts(),
    ",
  0x400b0110u64 => "
      CANFD_B.cfdc0fdcrc(),
    ",
  0x400b0120u64 => "
      CANFD_B.cfdgaflid()[0],
    ",
  0x400b0130u64 => "
      CANFD_B.cfdgaflid()[1],
    ",
  0x400b0140u64 => "
      CANFD_B.cfdgaflid()[2],
    ",
  0x400b0150u64 => "
      CANFD_B.cfdgaflid()[3],
    ",
  0x400b0160u64 => "
      CANFD_B.cfdgaflid()[4],
    ",
  0x400b0170u64 => "
      CANFD_B.cfdgaflid()[5],
    ",
  0x400b0180u64 => "
      CANFD_B.cfdgaflid()[6],
    ",
  0x400b0190u64 => "
      CANFD_B.cfdgaflid()[7],
    ",
  0x400b01a0u64 => "
      CANFD_B.cfdgaflid()[8],
    ",
  0x400b01b0u64 => "
      CANFD_B.cfdgaflid()[9],
    ",
  0x400b01c0u64 => "
      CANFD_B.cfdgaflid()[10],
    ",
  0x400b01d0u64 => "
      CANFD_B.cfdgaflid()[11],
    ",
  0x400b01e0u64 => "
      CANFD_B.cfdgaflid()[12],
    ",
  0x400b01f0u64 => "
      CANFD_B.cfdgaflid()[13],
    ",
  0x400b0200u64 => "
      CANFD_B.cfdgaflid()[14],
    ",
  0x400b0210u64 => "
      CANFD_B.cfdgaflid()[15],
    ",
  0x400b0124u64 => "
      CANFD_B.cfdgaflm()[0],
    ",
  0x400b0134u64 => "
      CANFD_B.cfdgaflm()[1],
    ",
  0x400b0144u64 => "
      CANFD_B.cfdgaflm()[2],
    ",
  0x400b0154u64 => "
      CANFD_B.cfdgaflm()[3],
    ",
  0x400b0164u64 => "
      CANFD_B.cfdgaflm()[4],
    ",
  0x400b0174u64 => "
      CANFD_B.cfdgaflm()[5],
    ",
  0x400b0184u64 => "
      CANFD_B.cfdgaflm()[6],
    ",
  0x400b0194u64 => "
      CANFD_B.cfdgaflm()[7],
    ",
  0x400b01a4u64 => "
      CANFD_B.cfdgaflm()[8],
    ",
  0x400b01b4u64 => "
      CANFD_B.cfdgaflm()[9],
    ",
  0x400b01c4u64 => "
      CANFD_B.cfdgaflm()[10],
    ",
  0x400b01d4u64 => "
      CANFD_B.cfdgaflm()[11],
    ",
  0x400b01e4u64 => "
      CANFD_B.cfdgaflm()[12],
    ",
  0x400b01f4u64 => "
      CANFD_B.cfdgaflm()[13],
    ",
  0x400b0204u64 => "
      CANFD_B.cfdgaflm()[14],
    ",
  0x400b0214u64 => "
      CANFD_B.cfdgaflm()[15],
    ",
  0x400b0128u64 => "
      CANFD_B.cfdgaflp0()[0],
    ",
  0x400b0138u64 => "
      CANFD_B.cfdgaflp0()[1],
    ",
  0x400b0148u64 => "
      CANFD_B.cfdgaflp0()[2],
    ",
  0x400b0158u64 => "
      CANFD_B.cfdgaflp0()[3],
    ",
  0x400b0168u64 => "
      CANFD_B.cfdgaflp0()[4],
    ",
  0x400b0178u64 => "
      CANFD_B.cfdgaflp0()[5],
    ",
  0x400b0188u64 => "
      CANFD_B.cfdgaflp0()[6],
    ",
  0x400b0198u64 => "
      CANFD_B.cfdgaflp0()[7],
    ",
  0x400b01a8u64 => "
      CANFD_B.cfdgaflp0()[8],
    ",
  0x400b01b8u64 => "
      CANFD_B.cfdgaflp0()[9],
    ",
  0x400b01c8u64 => "
      CANFD_B.cfdgaflp0()[10],
    ",
  0x400b01d8u64 => "
      CANFD_B.cfdgaflp0()[11],
    ",
  0x400b01e8u64 => "
      CANFD_B.cfdgaflp0()[12],
    ",
  0x400b01f8u64 => "
      CANFD_B.cfdgaflp0()[13],
    ",
  0x400b0208u64 => "
      CANFD_B.cfdgaflp0()[14],
    ",
  0x400b0218u64 => "
      CANFD_B.cfdgaflp0()[15],
    ",
  0x400b012cu64 => "
      CANFD_B.cfdgaflp1()[0],
    ",
  0x400b013cu64 => "
      CANFD_B.cfdgaflp1()[1],
    ",
  0x400b014cu64 => "
      CANFD_B.cfdgaflp1()[2],
    ",
  0x400b015cu64 => "
      CANFD_B.cfdgaflp1()[3],
    ",
  0x400b016cu64 => "
      CANFD_B.cfdgaflp1()[4],
    ",
  0x400b017cu64 => "
      CANFD_B.cfdgaflp1()[5],
    ",
  0x400b018cu64 => "
      CANFD_B.cfdgaflp1()[6],
    ",
  0x400b019cu64 => "
      CANFD_B.cfdgaflp1()[7],
    ",
  0x400b01acu64 => "
      CANFD_B.cfdgaflp1()[8],
    ",
  0x400b01bcu64 => "
      CANFD_B.cfdgaflp1()[9],
    ",
  0x400b01ccu64 => "
      CANFD_B.cfdgaflp1()[10],
    ",
  0x400b01dcu64 => "
      CANFD_B.cfdgaflp1()[11],
    ",
  0x400b01ecu64 => "
      CANFD_B.cfdgaflp1()[12],
    ",
  0x400b01fcu64 => "
      CANFD_B.cfdgaflp1()[13],
    ",
  0x400b020cu64 => "
      CANFD_B.cfdgaflp1()[14],
    ",
  0x400b021cu64 => "
      CANFD_B.cfdgaflp1()[15],
    ",
  0x400b0280u64 => "
      CANFD_B.cfdrpgacc()[0],
    ",
  0x400b0284u64 => "
      CANFD_B.cfdrpgacc()[1],
    ",
  0x400b0288u64 => "
      CANFD_B.cfdrpgacc()[2],
    ",
  0x400b028cu64 => "
      CANFD_B.cfdrpgacc()[3],
    ",
  0x400b0290u64 => "
      CANFD_B.cfdrpgacc()[4],
    ",
  0x400b0294u64 => "
      CANFD_B.cfdrpgacc()[5],
    ",
  0x400b0298u64 => "
      CANFD_B.cfdrpgacc()[6],
    ",
  0x400b029cu64 => "
      CANFD_B.cfdrpgacc()[7],
    ",
  0x400b02a0u64 => "
      CANFD_B.cfdrpgacc()[8],
    ",
  0x400b02a4u64 => "
      CANFD_B.cfdrpgacc()[9],
    ",
  0x400b02a8u64 => "
      CANFD_B.cfdrpgacc()[10],
    ",
  0x400b02acu64 => "
      CANFD_B.cfdrpgacc()[11],
    ",
  0x400b02b0u64 => "
      CANFD_B.cfdrpgacc()[12],
    ",
  0x400b02b4u64 => "
      CANFD_B.cfdrpgacc()[13],
    ",
  0x400b02b8u64 => "
      CANFD_B.cfdrpgacc()[14],
    ",
  0x400b02bcu64 => "
      CANFD_B.cfdrpgacc()[15],
    ",
  0x400b02c0u64 => "
      CANFD_B.cfdrpgacc()[16],
    ",
  0x400b02c4u64 => "
      CANFD_B.cfdrpgacc()[17],
    ",
  0x400b02c8u64 => "
      CANFD_B.cfdrpgacc()[18],
    ",
  0x400b02ccu64 => "
      CANFD_B.cfdrpgacc()[19],
    ",
  0x400b02d0u64 => "
      CANFD_B.cfdrpgacc()[20],
    ",
  0x400b02d4u64 => "
      CANFD_B.cfdrpgacc()[21],
    ",
  0x400b02d8u64 => "
      CANFD_B.cfdrpgacc()[22],
    ",
  0x400b02dcu64 => "
      CANFD_B.cfdrpgacc()[23],
    ",
  0x400b02e0u64 => "
      CANFD_B.cfdrpgacc()[24],
    ",
  0x400b02e4u64 => "
      CANFD_B.cfdrpgacc()[25],
    ",
  0x400b02e8u64 => "
      CANFD_B.cfdrpgacc()[26],
    ",
  0x400b02ecu64 => "
      CANFD_B.cfdrpgacc()[27],
    ",
  0x400b02f0u64 => "
      CANFD_B.cfdrpgacc()[28],
    ",
  0x400b02f4u64 => "
      CANFD_B.cfdrpgacc()[29],
    ",
  0x400b02f8u64 => "
      CANFD_B.cfdrpgacc()[30],
    ",
  0x400b02fcu64 => "
      CANFD_B.cfdrpgacc()[31],
    ",
  0x400b0300u64 => "
      CANFD_B.cfdrpgacc()[32],
    ",
  0x400b0304u64 => "
      CANFD_B.cfdrpgacc()[33],
    ",
  0x400b0308u64 => "
      CANFD_B.cfdrpgacc()[34],
    ",
  0x400b030cu64 => "
      CANFD_B.cfdrpgacc()[35],
    ",
  0x400b0310u64 => "
      CANFD_B.cfdrpgacc()[36],
    ",
  0x400b0314u64 => "
      CANFD_B.cfdrpgacc()[37],
    ",
  0x400b0318u64 => "
      CANFD_B.cfdrpgacc()[38],
    ",
  0x400b031cu64 => "
      CANFD_B.cfdrpgacc()[39],
    ",
  0x400b0320u64 => "
      CANFD_B.cfdrpgacc()[40],
    ",
  0x400b0324u64 => "
      CANFD_B.cfdrpgacc()[41],
    ",
  0x400b0328u64 => "
      CANFD_B.cfdrpgacc()[42],
    ",
  0x400b032cu64 => "
      CANFD_B.cfdrpgacc()[43],
    ",
  0x400b0330u64 => "
      CANFD_B.cfdrpgacc()[44],
    ",
  0x400b0334u64 => "
      CANFD_B.cfdrpgacc()[45],
    ",
  0x400b0338u64 => "
      CANFD_B.cfdrpgacc()[46],
    ",
  0x400b033cu64 => "
      CANFD_B.cfdrpgacc()[47],
    ",
  0x400b0340u64 => "
      CANFD_B.cfdrpgacc()[48],
    ",
  0x400b0344u64 => "
      CANFD_B.cfdrpgacc()[49],
    ",
  0x400b0348u64 => "
      CANFD_B.cfdrpgacc()[50],
    ",
  0x400b034cu64 => "
      CANFD_B.cfdrpgacc()[51],
    ",
  0x400b0350u64 => "
      CANFD_B.cfdrpgacc()[52],
    ",
  0x400b0354u64 => "
      CANFD_B.cfdrpgacc()[53],
    ",
  0x400b0358u64 => "
      CANFD_B.cfdrpgacc()[54],
    ",
  0x400b035cu64 => "
      CANFD_B.cfdrpgacc()[55],
    ",
  0x400b0360u64 => "
      CANFD_B.cfdrpgacc()[56],
    ",
  0x400b0364u64 => "
      CANFD_B.cfdrpgacc()[57],
    ",
  0x400b0368u64 => "
      CANFD_B.cfdrpgacc()[58],
    ",
  0x400b036cu64 => "
      CANFD_B.cfdrpgacc()[59],
    ",
  0x400b0370u64 => "
      CANFD_B.cfdrpgacc()[60],
    ",
  0x400b0374u64 => "
      CANFD_B.cfdrpgacc()[61],
    ",
  0x400b0378u64 => "
      CANFD_B.cfdrpgacc()[62],
    ",
  0x400b037cu64 => "
      CANFD_B.cfdrpgacc()[63],
    ",
  0x400b0520u64 => "
      CANFD_B.cfdrfid()[0],
    ",
  0x400b056cu64 => "
      CANFD_B.cfdrfid()[1],
    ",
  0x400b0524u64 => "
      CANFD_B.cfdrfptr()[0],
    ",
  0x400b0570u64 => "
      CANFD_B.cfdrfptr()[1],
    ",
  0x400b0528u64 => "
      CANFD_B.cfdrffdsts()[0],
    ",
  0x400b0574u64 => "
      CANFD_B.cfdrffdsts()[1],
    ",
  0x400b052cu64 => "
      CANFD_B.cfdrfdf_0()[0],
    ",
  0x400b0578u64 => "
      CANFD_B.cfdrfdf_0()[1],
    ",
  0x400b0530u64 => "
      CANFD_B.cfdrfdf_1()[0],
    ",
  0x400b057cu64 => "
      CANFD_B.cfdrfdf_1()[1],
    ",
  0x400b0534u64 => "
      CANFD_B.cfdrfdf_2()[0],
    ",
  0x400b0580u64 => "
      CANFD_B.cfdrfdf_2()[1],
    ",
  0x400b0538u64 => "
      CANFD_B.cfdrfdf_3()[0],
    ",
  0x400b0584u64 => "
      CANFD_B.cfdrfdf_3()[1],
    ",
  0x400b053cu64 => "
      CANFD_B.cfdrfdf_4()[0],
    ",
  0x400b0588u64 => "
      CANFD_B.cfdrfdf_4()[1],
    ",
  0x400b0540u64 => "
      CANFD_B.cfdrfdf_5()[0],
    ",
  0x400b058cu64 => "
      CANFD_B.cfdrfdf_5()[1],
    ",
  0x400b0544u64 => "
      CANFD_B.cfdrfdf_6()[0],
    ",
  0x400b0590u64 => "
      CANFD_B.cfdrfdf_6()[1],
    ",
  0x400b0548u64 => "
      CANFD_B.cfdrfdf_7()[0],
    ",
  0x400b0594u64 => "
      CANFD_B.cfdrfdf_7()[1],
    ",
  0x400b054cu64 => "
      CANFD_B.cfdrfdf_8()[0],
    ",
  0x400b0598u64 => "
      CANFD_B.cfdrfdf_8()[1],
    ",
  0x400b0550u64 => "
      CANFD_B.cfdrfdf_9()[0],
    ",
  0x400b059cu64 => "
      CANFD_B.cfdrfdf_9()[1],
    ",
  0x400b0554u64 => "
      CANFD_B.cfdrfdf_10()[0],
    ",
  0x400b05a0u64 => "
      CANFD_B.cfdrfdf_10()[1],
    ",
  0x400b0558u64 => "
      CANFD_B.cfdrfdf_11()[0],
    ",
  0x400b05a4u64 => "
      CANFD_B.cfdrfdf_11()[1],
    ",
  0x400b055cu64 => "
      CANFD_B.cfdrfdf_12()[0],
    ",
  0x400b05a8u64 => "
      CANFD_B.cfdrfdf_12()[1],
    ",
  0x400b0560u64 => "
      CANFD_B.cfdrfdf_13()[0],
    ",
  0x400b05acu64 => "
      CANFD_B.cfdrfdf_13()[1],
    ",
  0x400b0564u64 => "
      CANFD_B.cfdrfdf_14()[0],
    ",
  0x400b05b0u64 => "
      CANFD_B.cfdrfdf_14()[1],
    ",
  0x400b0568u64 => "
      CANFD_B.cfdrfdf_15()[0],
    ",
  0x400b05b4u64 => "
      CANFD_B.cfdrfdf_15()[1],
    ",
  0x400b05b8u64 => "
      CANFD_B.cfdcfid(),
    ",
  0x400b05bcu64 => "
      CANFD_B.cfdcfptr(),
    ",
  0x400b05c0u64 => "
      CANFD_B.cfdcffdcsts(),
    ",
  0x400b05c4u64 => "
      CANFD_B.cfdcfdf()[0],
    ",
  0x400b05c8u64 => "
      CANFD_B.cfdcfdf()[1],
    ",
  0x400b05ccu64 => "
      CANFD_B.cfdcfdf()[2],
    ",
  0x400b05d0u64 => "
      CANFD_B.cfdcfdf()[3],
    ",
  0x400b05d4u64 => "
      CANFD_B.cfdcfdf()[4],
    ",
  0x400b05d8u64 => "
      CANFD_B.cfdcfdf()[5],
    ",
  0x400b05dcu64 => "
      CANFD_B.cfdcfdf()[6],
    ",
  0x400b05e0u64 => "
      CANFD_B.cfdcfdf()[7],
    ",
  0x400b05e4u64 => "
      CANFD_B.cfdcfdf()[8],
    ",
  0x400b05e8u64 => "
      CANFD_B.cfdcfdf()[9],
    ",
  0x400b05ecu64 => "
      CANFD_B.cfdcfdf()[10],
    ",
  0x400b05f0u64 => "
      CANFD_B.cfdcfdf()[11],
    ",
  0x400b05f4u64 => "
      CANFD_B.cfdcfdf()[12],
    ",
  0x400b05f8u64 => "
      CANFD_B.cfdcfdf()[13],
    ",
  0x400b05fcu64 => "
      CANFD_B.cfdcfdf()[14],
    ",
  0x400b0600u64 => "
      CANFD_B.cfdcfdf()[15],
    ",
  0x400b0604u64 => "
      CANFD_B.cfdtmid()[0],
    ",
  0x400b0650u64 => "
      CANFD_B.cfdtmid()[1],
    ",
  0x400b069cu64 => "
      CANFD_B.cfdtmid()[2],
    ",
  0x400b06e8u64 => "
      CANFD_B.cfdtmid()[3],
    ",
  0x400b0608u64 => "
      CANFD_B.cfdtmptr()[0],
    ",
  0x400b0654u64 => "
      CANFD_B.cfdtmptr()[1],
    ",
  0x400b06a0u64 => "
      CANFD_B.cfdtmptr()[2],
    ",
  0x400b06ecu64 => "
      CANFD_B.cfdtmptr()[3],
    ",
  0x400b060cu64 => "
      CANFD_B.cfdtmfdctr()[0],
    ",
  0x400b0658u64 => "
      CANFD_B.cfdtmfdctr()[1],
    ",
  0x400b06a4u64 => "
      CANFD_B.cfdtmfdctr()[2],
    ",
  0x400b06f0u64 => "
      CANFD_B.cfdtmfdctr()[3],
    ",
  0x400b0610u64 => "
      CANFD_B.cfdtmdf_0()[0],
    ",
  0x400b065cu64 => "
      CANFD_B.cfdtmdf_0()[1],
    ",
  0x400b06a8u64 => "
      CANFD_B.cfdtmdf_0()[2],
    ",
  0x400b06f4u64 => "
      CANFD_B.cfdtmdf_0()[3],
    ",
  0x400b0614u64 => "
      CANFD_B.cfdtmdf_1()[0],
    ",
  0x400b0660u64 => "
      CANFD_B.cfdtmdf_1()[1],
    ",
  0x400b06acu64 => "
      CANFD_B.cfdtmdf_1()[2],
    ",
  0x400b06f8u64 => "
      CANFD_B.cfdtmdf_1()[3],
    ",
  0x400b0618u64 => "
      CANFD_B.cfdtmdf_2()[0],
    ",
  0x400b0664u64 => "
      CANFD_B.cfdtmdf_2()[1],
    ",
  0x400b06b0u64 => "
      CANFD_B.cfdtmdf_2()[2],
    ",
  0x400b06fcu64 => "
      CANFD_B.cfdtmdf_2()[3],
    ",
  0x400b061cu64 => "
      CANFD_B.cfdtmdf_3()[0],
    ",
  0x400b0668u64 => "
      CANFD_B.cfdtmdf_3()[1],
    ",
  0x400b06b4u64 => "
      CANFD_B.cfdtmdf_3()[2],
    ",
  0x400b0700u64 => "
      CANFD_B.cfdtmdf_3()[3],
    ",
  0x400b0620u64 => "
      CANFD_B.cfdtmdf_4()[0],
    ",
  0x400b066cu64 => "
      CANFD_B.cfdtmdf_4()[1],
    ",
  0x400b06b8u64 => "
      CANFD_B.cfdtmdf_4()[2],
    ",
  0x400b0704u64 => "
      CANFD_B.cfdtmdf_4()[3],
    ",
  0x400b0624u64 => "
      CANFD_B.cfdtmdf_5()[0],
    ",
  0x400b0670u64 => "
      CANFD_B.cfdtmdf_5()[1],
    ",
  0x400b06bcu64 => "
      CANFD_B.cfdtmdf_5()[2],
    ",
  0x400b0708u64 => "
      CANFD_B.cfdtmdf_5()[3],
    ",
  0x400b0628u64 => "
      CANFD_B.cfdtmdf_6()[0],
    ",
  0x400b0674u64 => "
      CANFD_B.cfdtmdf_6()[1],
    ",
  0x400b06c0u64 => "
      CANFD_B.cfdtmdf_6()[2],
    ",
  0x400b070cu64 => "
      CANFD_B.cfdtmdf_6()[3],
    ",
  0x400b062cu64 => "
      CANFD_B.cfdtmdf_7()[0],
    ",
  0x400b0678u64 => "
      CANFD_B.cfdtmdf_7()[1],
    ",
  0x400b06c4u64 => "
      CANFD_B.cfdtmdf_7()[2],
    ",
  0x400b0710u64 => "
      CANFD_B.cfdtmdf_7()[3],
    ",
  0x400b0630u64 => "
      CANFD_B.cfdtmdf_8()[0],
    ",
  0x400b067cu64 => "
      CANFD_B.cfdtmdf_8()[1],
    ",
  0x400b06c8u64 => "
      CANFD_B.cfdtmdf_8()[2],
    ",
  0x400b0714u64 => "
      CANFD_B.cfdtmdf_8()[3],
    ",
  0x400b0634u64 => "
      CANFD_B.cfdtmdf_9()[0],
    ",
  0x400b0680u64 => "
      CANFD_B.cfdtmdf_9()[1],
    ",
  0x400b06ccu64 => "
      CANFD_B.cfdtmdf_9()[2],
    ",
  0x400b0718u64 => "
      CANFD_B.cfdtmdf_9()[3],
    ",
  0x400b0638u64 => "
      CANFD_B.cfdtmdf_10()[0],
    ",
  0x400b0684u64 => "
      CANFD_B.cfdtmdf_10()[1],
    ",
  0x400b06d0u64 => "
      CANFD_B.cfdtmdf_10()[2],
    ",
  0x400b071cu64 => "
      CANFD_B.cfdtmdf_10()[3],
    ",
  0x400b063cu64 => "
      CANFD_B.cfdtmdf_11()[0],
    ",
  0x400b0688u64 => "
      CANFD_B.cfdtmdf_11()[1],
    ",
  0x400b06d4u64 => "
      CANFD_B.cfdtmdf_11()[2],
    ",
  0x400b0720u64 => "
      CANFD_B.cfdtmdf_11()[3],
    ",
  0x400b0640u64 => "
      CANFD_B.cfdtmdf_12()[0],
    ",
  0x400b068cu64 => "
      CANFD_B.cfdtmdf_12()[1],
    ",
  0x400b06d8u64 => "
      CANFD_B.cfdtmdf_12()[2],
    ",
  0x400b0724u64 => "
      CANFD_B.cfdtmdf_12()[3],
    ",
  0x400b0644u64 => "
      CANFD_B.cfdtmdf_13()[0],
    ",
  0x400b0690u64 => "
      CANFD_B.cfdtmdf_13()[1],
    ",
  0x400b06dcu64 => "
      CANFD_B.cfdtmdf_13()[2],
    ",
  0x400b0728u64 => "
      CANFD_B.cfdtmdf_13()[3],
    ",
  0x400b0648u64 => "
      CANFD_B.cfdtmdf_14()[0],
    ",
  0x400b0694u64 => "
      CANFD_B.cfdtmdf_14()[1],
    ",
  0x400b06e0u64 => "
      CANFD_B.cfdtmdf_14()[2],
    ",
  0x400b072cu64 => "
      CANFD_B.cfdtmdf_14()[3],
    ",
  0x400b064cu64 => "
      CANFD_B.cfdtmdf_15()[0],
    ",
  0x400b0698u64 => "
      CANFD_B.cfdtmdf_15()[1],
    ",
  0x400b06e4u64 => "
      CANFD_B.cfdtmdf_15()[2],
    ",
  0x400b0730u64 => "
      CANFD_B.cfdtmdf_15()[3],
    ",
  0x400b0740u64 => "
      CANFD_B.cfdthlacc0(),
    ",
  0x400b0744u64 => "
      CANFD_B.cfdthlacc1(),
    ",
  0x400b1520u64 => "
      CANFD_B.cfdrmid()[0],
    ",
  0x400b156cu64 => "
      CANFD_B.cfdrmid()[1],
    ",
  0x400b15b8u64 => "
      CANFD_B.cfdrmid()[2],
    ",
  0x400b1604u64 => "
      CANFD_B.cfdrmid()[3],
    ",
  0x400b1650u64 => "
      CANFD_B.cfdrmid()[4],
    ",
  0x400b169cu64 => "
      CANFD_B.cfdrmid()[5],
    ",
  0x400b16e8u64 => "
      CANFD_B.cfdrmid()[6],
    ",
  0x400b1734u64 => "
      CANFD_B.cfdrmid()[7],
    ",
  0x400b1524u64 => "
      CANFD_B.cfdrmptr()[0],
    ",
  0x400b1570u64 => "
      CANFD_B.cfdrmptr()[1],
    ",
  0x400b15bcu64 => "
      CANFD_B.cfdrmptr()[2],
    ",
  0x400b1608u64 => "
      CANFD_B.cfdrmptr()[3],
    ",
  0x400b1654u64 => "
      CANFD_B.cfdrmptr()[4],
    ",
  0x400b16a0u64 => "
      CANFD_B.cfdrmptr()[5],
    ",
  0x400b16ecu64 => "
      CANFD_B.cfdrmptr()[6],
    ",
  0x400b1738u64 => "
      CANFD_B.cfdrmptr()[7],
    ",
  0x400b1528u64 => "
      CANFD_B.cfdrmfdsts()[0],
    ",
  0x400b1574u64 => "
      CANFD_B.cfdrmfdsts()[1],
    ",
  0x400b15c0u64 => "
      CANFD_B.cfdrmfdsts()[2],
    ",
  0x400b160cu64 => "
      CANFD_B.cfdrmfdsts()[3],
    ",
  0x400b1658u64 => "
      CANFD_B.cfdrmfdsts()[4],
    ",
  0x400b16a4u64 => "
      CANFD_B.cfdrmfdsts()[5],
    ",
  0x400b16f0u64 => "
      CANFD_B.cfdrmfdsts()[6],
    ",
  0x400b173cu64 => "
      CANFD_B.cfdrmfdsts()[7],
    ",
  0x400b152cu64 => "
      CANFD_B.cfdrmdf_0()[0],
    ",
  0x400b1578u64 => "
      CANFD_B.cfdrmdf_0()[1],
    ",
  0x400b15c4u64 => "
      CANFD_B.cfdrmdf_0()[2],
    ",
  0x400b1610u64 => "
      CANFD_B.cfdrmdf_0()[3],
    ",
  0x400b165cu64 => "
      CANFD_B.cfdrmdf_0()[4],
    ",
  0x400b16a8u64 => "
      CANFD_B.cfdrmdf_0()[5],
    ",
  0x400b16f4u64 => "
      CANFD_B.cfdrmdf_0()[6],
    ",
  0x400b1740u64 => "
      CANFD_B.cfdrmdf_0()[7],
    ",
  0x400b1530u64 => "
      CANFD_B.cfdrmdf_1()[0],
    ",
  0x400b157cu64 => "
      CANFD_B.cfdrmdf_1()[1],
    ",
  0x400b15c8u64 => "
      CANFD_B.cfdrmdf_1()[2],
    ",
  0x400b1614u64 => "
      CANFD_B.cfdrmdf_1()[3],
    ",
  0x400b1660u64 => "
      CANFD_B.cfdrmdf_1()[4],
    ",
  0x400b16acu64 => "
      CANFD_B.cfdrmdf_1()[5],
    ",
  0x400b16f8u64 => "
      CANFD_B.cfdrmdf_1()[6],
    ",
  0x400b1744u64 => "
      CANFD_B.cfdrmdf_1()[7],
    ",
  0x400b1534u64 => "
      CANFD_B.cfdrmdf_2()[0],
    ",
  0x400b1580u64 => "
      CANFD_B.cfdrmdf_2()[1],
    ",
  0x400b15ccu64 => "
      CANFD_B.cfdrmdf_2()[2],
    ",
  0x400b1618u64 => "
      CANFD_B.cfdrmdf_2()[3],
    ",
  0x400b1664u64 => "
      CANFD_B.cfdrmdf_2()[4],
    ",
  0x400b16b0u64 => "
      CANFD_B.cfdrmdf_2()[5],
    ",
  0x400b16fcu64 => "
      CANFD_B.cfdrmdf_2()[6],
    ",
  0x400b1748u64 => "
      CANFD_B.cfdrmdf_2()[7],
    ",
  0x400b1538u64 => "
      CANFD_B.cfdrmdf_3()[0],
    ",
  0x400b1584u64 => "
      CANFD_B.cfdrmdf_3()[1],
    ",
  0x400b15d0u64 => "
      CANFD_B.cfdrmdf_3()[2],
    ",
  0x400b161cu64 => "
      CANFD_B.cfdrmdf_3()[3],
    ",
  0x400b1668u64 => "
      CANFD_B.cfdrmdf_3()[4],
    ",
  0x400b16b4u64 => "
      CANFD_B.cfdrmdf_3()[5],
    ",
  0x400b1700u64 => "
      CANFD_B.cfdrmdf_3()[6],
    ",
  0x400b174cu64 => "
      CANFD_B.cfdrmdf_3()[7],
    ",
  0x400b153cu64 => "
      CANFD_B.cfdrmdf_4()[0],
    ",
  0x400b1588u64 => "
      CANFD_B.cfdrmdf_4()[1],
    ",
  0x400b15d4u64 => "
      CANFD_B.cfdrmdf_4()[2],
    ",
  0x400b1620u64 => "
      CANFD_B.cfdrmdf_4()[3],
    ",
  0x400b166cu64 => "
      CANFD_B.cfdrmdf_4()[4],
    ",
  0x400b16b8u64 => "
      CANFD_B.cfdrmdf_4()[5],
    ",
  0x400b1704u64 => "
      CANFD_B.cfdrmdf_4()[6],
    ",
  0x400b1750u64 => "
      CANFD_B.cfdrmdf_4()[7],
    ",
  0x400b1540u64 => "
      CANFD_B.cfdrmdf_5()[0],
    ",
  0x400b158cu64 => "
      CANFD_B.cfdrmdf_5()[1],
    ",
  0x400b15d8u64 => "
      CANFD_B.cfdrmdf_5()[2],
    ",
  0x400b1624u64 => "
      CANFD_B.cfdrmdf_5()[3],
    ",
  0x400b1670u64 => "
      CANFD_B.cfdrmdf_5()[4],
    ",
  0x400b16bcu64 => "
      CANFD_B.cfdrmdf_5()[5],
    ",
  0x400b1708u64 => "
      CANFD_B.cfdrmdf_5()[6],
    ",
  0x400b1754u64 => "
      CANFD_B.cfdrmdf_5()[7],
    ",
  0x400b1544u64 => "
      CANFD_B.cfdrmdf_6()[0],
    ",
  0x400b1590u64 => "
      CANFD_B.cfdrmdf_6()[1],
    ",
  0x400b15dcu64 => "
      CANFD_B.cfdrmdf_6()[2],
    ",
  0x400b1628u64 => "
      CANFD_B.cfdrmdf_6()[3],
    ",
  0x400b1674u64 => "
      CANFD_B.cfdrmdf_6()[4],
    ",
  0x400b16c0u64 => "
      CANFD_B.cfdrmdf_6()[5],
    ",
  0x400b170cu64 => "
      CANFD_B.cfdrmdf_6()[6],
    ",
  0x400b1758u64 => "
      CANFD_B.cfdrmdf_6()[7],
    ",
  0x400b1548u64 => "
      CANFD_B.cfdrmdf_7()[0],
    ",
  0x400b1594u64 => "
      CANFD_B.cfdrmdf_7()[1],
    ",
  0x400b15e0u64 => "
      CANFD_B.cfdrmdf_7()[2],
    ",
  0x400b162cu64 => "
      CANFD_B.cfdrmdf_7()[3],
    ",
  0x400b1678u64 => "
      CANFD_B.cfdrmdf_7()[4],
    ",
  0x400b16c4u64 => "
      CANFD_B.cfdrmdf_7()[5],
    ",
  0x400b1710u64 => "
      CANFD_B.cfdrmdf_7()[6],
    ",
  0x400b175cu64 => "
      CANFD_B.cfdrmdf_7()[7],
    ",
  0x400b154cu64 => "
      CANFD_B.cfdrmdf_8()[0],
    ",
  0x400b1598u64 => "
      CANFD_B.cfdrmdf_8()[1],
    ",
  0x400b15e4u64 => "
      CANFD_B.cfdrmdf_8()[2],
    ",
  0x400b1630u64 => "
      CANFD_B.cfdrmdf_8()[3],
    ",
  0x400b167cu64 => "
      CANFD_B.cfdrmdf_8()[4],
    ",
  0x400b16c8u64 => "
      CANFD_B.cfdrmdf_8()[5],
    ",
  0x400b1714u64 => "
      CANFD_B.cfdrmdf_8()[6],
    ",
  0x400b1760u64 => "
      CANFD_B.cfdrmdf_8()[7],
    ",
  0x400b1550u64 => "
      CANFD_B.cfdrmdf_9()[0],
    ",
  0x400b159cu64 => "
      CANFD_B.cfdrmdf_9()[1],
    ",
  0x400b15e8u64 => "
      CANFD_B.cfdrmdf_9()[2],
    ",
  0x400b1634u64 => "
      CANFD_B.cfdrmdf_9()[3],
    ",
  0x400b1680u64 => "
      CANFD_B.cfdrmdf_9()[4],
    ",
  0x400b16ccu64 => "
      CANFD_B.cfdrmdf_9()[5],
    ",
  0x400b1718u64 => "
      CANFD_B.cfdrmdf_9()[6],
    ",
  0x400b1764u64 => "
      CANFD_B.cfdrmdf_9()[7],
    ",
  0x400b1554u64 => "
      CANFD_B.cfdrmdf_10()[0],
    ",
  0x400b15a0u64 => "
      CANFD_B.cfdrmdf_10()[1],
    ",
  0x400b15ecu64 => "
      CANFD_B.cfdrmdf_10()[2],
    ",
  0x400b1638u64 => "
      CANFD_B.cfdrmdf_10()[3],
    ",
  0x400b1684u64 => "
      CANFD_B.cfdrmdf_10()[4],
    ",
  0x400b16d0u64 => "
      CANFD_B.cfdrmdf_10()[5],
    ",
  0x400b171cu64 => "
      CANFD_B.cfdrmdf_10()[6],
    ",
  0x400b1768u64 => "
      CANFD_B.cfdrmdf_10()[7],
    ",
  0x400b1558u64 => "
      CANFD_B.cfdrmdf_11()[0],
    ",
  0x400b15a4u64 => "
      CANFD_B.cfdrmdf_11()[1],
    ",
  0x400b15f0u64 => "
      CANFD_B.cfdrmdf_11()[2],
    ",
  0x400b163cu64 => "
      CANFD_B.cfdrmdf_11()[3],
    ",
  0x400b1688u64 => "
      CANFD_B.cfdrmdf_11()[4],
    ",
  0x400b16d4u64 => "
      CANFD_B.cfdrmdf_11()[5],
    ",
  0x400b1720u64 => "
      CANFD_B.cfdrmdf_11()[6],
    ",
  0x400b176cu64 => "
      CANFD_B.cfdrmdf_11()[7],
    ",
  0x400b155cu64 => "
      CANFD_B.cfdrmdf_12()[0],
    ",
  0x400b15a8u64 => "
      CANFD_B.cfdrmdf_12()[1],
    ",
  0x400b15f4u64 => "
      CANFD_B.cfdrmdf_12()[2],
    ",
  0x400b1640u64 => "
      CANFD_B.cfdrmdf_12()[3],
    ",
  0x400b168cu64 => "
      CANFD_B.cfdrmdf_12()[4],
    ",
  0x400b16d8u64 => "
      CANFD_B.cfdrmdf_12()[5],
    ",
  0x400b1724u64 => "
      CANFD_B.cfdrmdf_12()[6],
    ",
  0x400b1770u64 => "
      CANFD_B.cfdrmdf_12()[7],
    ",
  0x400b1560u64 => "
      CANFD_B.cfdrmdf_13()[0],
    ",
  0x400b15acu64 => "
      CANFD_B.cfdrmdf_13()[1],
    ",
  0x400b15f8u64 => "
      CANFD_B.cfdrmdf_13()[2],
    ",
  0x400b1644u64 => "
      CANFD_B.cfdrmdf_13()[3],
    ",
  0x400b1690u64 => "
      CANFD_B.cfdrmdf_13()[4],
    ",
  0x400b16dcu64 => "
      CANFD_B.cfdrmdf_13()[5],
    ",
  0x400b1728u64 => "
      CANFD_B.cfdrmdf_13()[6],
    ",
  0x400b1774u64 => "
      CANFD_B.cfdrmdf_13()[7],
    ",
  0x400b1564u64 => "
      CANFD_B.cfdrmdf_14()[0],
    ",
  0x400b15b0u64 => "
      CANFD_B.cfdrmdf_14()[1],
    ",
  0x400b15fcu64 => "
      CANFD_B.cfdrmdf_14()[2],
    ",
  0x400b1648u64 => "
      CANFD_B.cfdrmdf_14()[3],
    ",
  0x400b1694u64 => "
      CANFD_B.cfdrmdf_14()[4],
    ",
  0x400b16e0u64 => "
      CANFD_B.cfdrmdf_14()[5],
    ",
  0x400b172cu64 => "
      CANFD_B.cfdrmdf_14()[6],
    ",
  0x400b1778u64 => "
      CANFD_B.cfdrmdf_14()[7],
    ",
  0x400b1568u64 => "
      CANFD_B.cfdrmdf_15()[0],
    ",
  0x400b15b4u64 => "
      CANFD_B.cfdrmdf_15()[1],
    ",
  0x400b1600u64 => "
      CANFD_B.cfdrmdf_15()[2],
    ",
  0x400b164cu64 => "
      CANFD_B.cfdrmdf_15()[3],
    ",
  0x400b1698u64 => "
      CANFD_B.cfdrmdf_15()[4],
    ",
  0x400b16e4u64 => "
      CANFD_B.cfdrmdf_15()[5],
    ",
  0x400b1730u64 => "
      CANFD_B.cfdrmdf_15()[6],
    ",
  0x400b177cu64 => "
      CANFD_B.cfdrmdf_15()[7],
    ",
  0x400e0004u64 => "
      PSCU.psarb(),
    ",
  0x400e0008u64 => "
      PSCU.psarc(),
    ",
  0x400e000cu64 => "
      PSCU.psard(),
    ",
  0x400e0010u64 => "
      PSCU.psare(),
    ",
  0x400e0014u64 => "
      PSCU.mssar(),
    ",
  0x400e0018u64 => "
      PSCU.cfsamona(),
    ",
  0x400e001cu64 => "
      PSCU.cfsamonb(),
    ",
  0x400e0020u64 => "
      PSCU.dfsamon(),
    ",
  0x400e0024u64 => "
      PSCU.ssamona(),
    ",
  0x400e0028u64 => "
      PSCU.ssamonb(),
    ",
  0x400e8000u64 => "
      AGTW_0.agt(),
    ",
  0x400e8004u64 => "
      AGTW_0.agtcma(),
    ",
  0x400e8008u64 => "
      AGTW_0.agtcmb(),
    ",
  0x400e800cu64 => "
      AGTW_0.agtcr(),
    ",
  0x400e800du64 => "
      AGTW_0.agtmr1(),
    ",
  0x400e800eu64 => "
      AGTW_0.agtmr2(),
    ",
  0x400e8010u64 => "
      AGTW_0.agtioc(),
    ",
  0x400e8011u64 => "
      AGTW_0.agtisr(),
    ",
  0x400e8012u64 => "
      AGTW_0.agtcmsr(),
    ",
  0x400e8013u64 => "
      AGTW_0.agtiosel(),
    ",
  0x400f3000u64 => "
      TSN.tscr(),
    ",
  0x40108000u64 => "
      CRC.crccr0(),
    ",
  0x40108004u64 => "
      CRC.crcdir(),
      CRC.crcdir_by(),
    ",
  0x40108008u64 => "
      CRC.crcdor(),
      CRC.crcdor_ha(),
      CRC.crcdor_by(),
    ",
  0x40109000u64 => "
      DOC.docr(),
    ",
  0x40109002u64 => "
      DOC.dodir(),
    ",
  0x40109004u64 => "
      DOC.dodsr(),
    ",
  0x40118000u64 => "
      SCI_0.smr(),
      SCI_0.smr_smci(),
    ",
  0x40118001u64 => "
      SCI_0.brr(),
    ",
  0x40118002u64 => "
      SCI_0.scr(),
      SCI_0.scr_smci(),
    ",
  0x40118003u64 => "
      SCI_0.tdr(),
    ",
  0x40118004u64 => "
      SCI_0.ssr(),
      SCI_0.ssr_fifo(),
      SCI_0.ssr_manc(),
      SCI_0.ssr_smci(),
    ",
  0x40118005u64 => "
      SCI_0.rdr(),
    ",
  0x40118006u64 => "
      SCI_0.scmr(),
    ",
  0x40118007u64 => "
      SCI_0.semr(),
    ",
  0x40118008u64 => "
      SCI_0.snfr(),
    ",
  0x40118009u64 => "
      SCI_0.simr1(),
    ",
  0x4011800au64 => "
      SCI_0.simr2(),
    ",
  0x4011800bu64 => "
      SCI_0.simr3(),
    ",
  0x4011800cu64 => "
      SCI_0.sisr(),
    ",
  0x4011800du64 => "
      SCI_0.spmr(),
    ",
  0x4011800eu64 => "
      SCI_0.ftdrhl(),
      SCI_0.tdrhl(),
      SCI_0.tdrhl_man(),
      SCI_0.ftdrh(),
    ",
  0x4011800fu64 => "
      SCI_0.ftdrl(),
    ",
  0x40118010u64 => "
      SCI_0.frdrhl(),
      SCI_0.rdrhl(),
      SCI_0.rdrhl_man(),
      SCI_0.frdrh(),
    ",
  0x40118011u64 => "
      SCI_0.frdrl(),
    ",
  0x40118012u64 => "
      SCI_0.mddr(),
    ",
  0x40118013u64 => "
      SCI_0.dccr(),
    ",
  0x40118014u64 => "
      SCI_0.fcr(),
    ",
  0x40118016u64 => "
      SCI_0.fdr(),
    ",
  0x40118018u64 => "
      SCI_0.lsr(),
    ",
  0x4011801au64 => "
      SCI_0.cdr(),
    ",
  0x4011801cu64 => "
      SCI_0.sptr(),
    ",
  0x4011801du64 => "
      SCI_0.actr(),
    ",
  0x40118020u64 => "
      SCI_0.mmr(),
    ",
  0x40118022u64 => "
      SCI_0.tmpr(),
    ",
  0x40118023u64 => "
      SCI_0.rmpr(),
    ",
  0x40118024u64 => "
      SCI_0.mesr(),
    ",
  0x40118025u64 => "
      SCI_0.mecr(),
    ",
  0x4011a000u64 => "
      SPI_0.spcr(),
    ",
  0x4011a001u64 => "
      SPI_0.sslp(),
    ",
  0x4011a002u64 => "
      SPI_0.sppcr(),
    ",
  0x4011a003u64 => "
      SPI_0.spsr(),
    ",
  0x4011a004u64 => "
      SPI_0.spdr(),
      SPI_0.spdr_ha(),
      SPI_0.spdr_by(),
    ",
  0x4011a008u64 => "
      SPI_0.spscr(),
    ",
  0x4011a009u64 => "
      SPI_0.spssr(),
    ",
  0x4011a00au64 => "
      SPI_0.spbr(),
    ",
  0x4011a00bu64 => "
      SPI_0.spdcr(),
    ",
  0x4011a00cu64 => "
      SPI_0.spckd(),
    ",
  0x4011a00du64 => "
      SPI_0.sslnd(),
    ",
  0x4011a00eu64 => "
      SPI_0.spnd(),
    ",
  0x4011a00fu64 => "
      SPI_0.spcr2(),
    ",
  0x4011a010u64 => "
      SPI_0.spcmd()[0],
    ",
  0x4011a012u64 => "
      SPI_0.spcmd()[1],
    ",
  0x4011a014u64 => "
      SPI_0.spcmd()[2],
    ",
  0x4011a016u64 => "
      SPI_0.spcmd()[3],
    ",
  0x4011a018u64 => "
      SPI_0.spcmd()[4],
    ",
  0x4011a01au64 => "
      SPI_0.spcmd()[5],
    ",
  0x4011a01cu64 => "
      SPI_0.spcmd()[6],
    ",
  0x4011a01eu64 => "
      SPI_0.spcmd()[7],
    ",
  0x4011a020u64 => "
      SPI_0.spdcr2(),
    ",
  0x4011a021u64 => "
      SPI_0.spcr3(),
    ",
  0x4011f000u64 => "
      I_3_C.prts(),
    ",
  0x4011f010u64 => "
      I_3_C.cectl(),
    ",
  0x4011f014u64 => "
      I_3_C.bctl(),
    ",
  0x4011f018u64 => "
      I_3_C.msdvad(),
    ",
  0x4011f020u64 => "
      I_3_C.rstctl(),
    ",
  0x4011f024u64 => "
      I_3_C.prsst(),
    ",
  0x4011f030u64 => "
      I_3_C.inst(),
    ",
  0x4011f034u64 => "
      I_3_C.inste(),
    ",
  0x4011f038u64 => "
      I_3_C.inie(),
    ",
  0x4011f03cu64 => "
      I_3_C.instfc(),
    ",
  0x4011f044u64 => "
      I_3_C.dvct(),
    ",
  0x4011f058u64 => "
      I_3_C.ibinctl(),
    ",
  0x4011f060u64 => "
      I_3_C.bfctl(),
    ",
  0x4011f064u64 => "
      I_3_C.svctl(),
    ",
  0x4011f070u64 => "
      I_3_C.refckctl(),
    ",
  0x4011f074u64 => "
      I_3_C.stdbr(),
    ",
  0x4011f078u64 => "
      I_3_C.extbr(),
    ",
  0x4011f07cu64 => "
      I_3_C.bfrecdt(),
    ",
  0x4011f080u64 => "
      I_3_C.bavlcdt(),
    ",
  0x4011f084u64 => "
      I_3_C.bidlcdt(),
    ",
  0x4011f088u64 => "
      I_3_C.outctl(),
    ",
  0x4011f08cu64 => "
      I_3_C.inctl(),
    ",
  0x4011f090u64 => "
      I_3_C.tmoctl(),
    ",
  0x4011f098u64 => "
      I_3_C.wuctl(),
    ",
  0x4011f0a0u64 => "
      I_3_C.ackctl(),
    ",
  0x4011f0a4u64 => "
      I_3_C.scstrctl(),
    ",
  0x4011f0b0u64 => "
      I_3_C.scstlctl(),
    ",
  0x4011f0c0u64 => "
      I_3_C.svtdlg0(),
    ",
  0x4011f120u64 => "
      I_3_C.stctl(),
    ",
  0x4011f124u64 => "
      I_3_C.atctl(),
    ",
  0x4011f128u64 => "
      I_3_C.attrg(),
    ",
  0x4011f12cu64 => "
      I_3_C.atccnte(),
    ",
  0x4011f140u64 => "
      I_3_C.cndctl(),
    ",
  0x4011f150u64 => "
      I_3_C.ncmdqp(),
    ",
  0x4011f154u64 => "
      I_3_C.nrspqp(),
    ",
  0x4011f158u64 => "
      I_3_C.ntdtbp0(),
      I_3_C.ntdtbp0_by(),
    ",
  0x4011f17cu64 => "
      I_3_C.nibiqp(),
    ",
  0x4011f180u64 => "
      I_3_C.nrsqp(),
    ",
  0x4011f184u64 => "
      I_3_C.hcmdqp(),
    ",
  0x4011f188u64 => "
      I_3_C.hrspqp(),
    ",
  0x4011f18cu64 => "
      I_3_C.htdtbp(),
    ",
  0x4011f190u64 => "
      I_3_C.nqthctl(),
    ",
  0x4011f194u64 => "
      I_3_C.ntbthctl0(),
    ",
  0x4011f1c0u64 => "
      I_3_C.nrqthctl(),
    ",
  0x4011f1c4u64 => "
      I_3_C.hqthctl(),
    ",
  0x4011f1c8u64 => "
      I_3_C.htbthctl(),
    ",
  0x4011f1d0u64 => "
      I_3_C.bst(),
    ",
  0x4011f1d4u64 => "
      I_3_C.bste(),
    ",
  0x4011f1d8u64 => "
      I_3_C.bie(),
    ",
  0x4011f1dcu64 => "
      I_3_C.bstfc(),
    ",
  0x4011f1e0u64 => "
      I_3_C.ntst(),
    ",
  0x4011f1e4u64 => "
      I_3_C.ntste(),
    ",
  0x4011f1e8u64 => "
      I_3_C.ntie(),
    ",
  0x4011f1ecu64 => "
      I_3_C.ntstfc(),
    ",
  0x4011f200u64 => "
      I_3_C.htst(),
    ",
  0x4011f204u64 => "
      I_3_C.htste(),
    ",
  0x4011f208u64 => "
      I_3_C.htie(),
    ",
  0x4011f20cu64 => "
      I_3_C.htstfc(),
    ",
  0x4011f210u64 => "
      I_3_C.bcst(),
    ",
  0x4011f214u64 => "
      I_3_C.svst(),
    ",
  0x4011f218u64 => "
      I_3_C.wust(),
    ",
  0x4011f21cu64 => "
      I_3_C.mrccpt(),
    ",
  0x4011f224u64 => "
      I_3_C.datbas()[0],
    ",
  0x4011f22cu64 => "
      I_3_C.datbas()[1],
    ",
  0x4011f234u64 => "
      I_3_C.datbas()[2],
    ",
  0x4011f23cu64 => "
      I_3_C.datbas()[3],
    ",
  0x4011f244u64 => "
      I_3_C.datbas()[4],
    ",
  0x4011f24cu64 => "
      I_3_C.datbas()[5],
    ",
  0x4011f254u64 => "
      I_3_C.datbas()[6],
    ",
  0x4011f25cu64 => "
      I_3_C.datbas()[7],
    ",
  0x4011f2a0u64 => "
      I_3_C.exdatbas(),
    ",
  0x4011f2b0u64 => "
      I_3_C.sdatbas0(),
      I_3_C.sdatbas1(),
      I_3_C.sdatbas2(),
    ",
  0x4011f2d0u64 => "
      I_3_C.msdct()[0],
    ",
  0x4011f2d4u64 => "
      I_3_C.msdct()[1],
    ",
  0x4011f2d8u64 => "
      I_3_C.msdct()[2],
    ",
  0x4011f2dcu64 => "
      I_3_C.msdct()[3],
    ",
  0x4011f2e0u64 => "
      I_3_C.msdct()[4],
    ",
  0x4011f2e4u64 => "
      I_3_C.msdct()[5],
    ",
  0x4011f2e8u64 => "
      I_3_C.msdct()[6],
    ",
  0x4011f2ecu64 => "
      I_3_C.msdct()[7],
    ",
  0x4011f320u64 => "
      I_3_C.svdct(),
    ",
  0x4011f324u64 => "
      I_3_C.sdctpidl(),
    ",
  0x4011f328u64 => "
      I_3_C.sdctpidh(),
    ",
  0x4011f330u64 => "
      I_3_C.svdvad()[0],
    ",
  0x4011f334u64 => "
      I_3_C.svdvad()[1],
    ",
  0x4011f338u64 => "
      I_3_C.svdvad()[2],
    ",
  0x4011f350u64 => "
      I_3_C.csecmd(),
    ",
  0x4011f354u64 => "
      I_3_C.ceactst(),
    ",
  0x4011f358u64 => "
      I_3_C.cmwlg(),
    ",
  0x4011f35cu64 => "
      I_3_C.cmrlg(),
    ",
  0x4011f360u64 => "
      I_3_C.cetstmd(),
    ",
  0x4011f364u64 => "
      I_3_C.cgdvst(),
    ",
  0x4011f368u64 => "
      I_3_C.cmdspw(),
    ",
  0x4011f36cu64 => "
      I_3_C.cmdspr(),
    ",
  0x4011f370u64 => "
      I_3_C.cmdspt(),
    ",
  0x4011f374u64 => "
      I_3_C.cetsm(),
    ",
  0x4011f378u64 => "
      I_3_C.cetss(),
    ",
  0x4011f380u64 => "
      I_3_C.bitcnt(),
    ",
  0x4011f394u64 => "
      I_3_C.nqstlv(),
    ",
  0x4011f398u64 => "
      I_3_C.ndbstlv0(),
    ",
  0x4011f3c0u64 => "
      I_3_C.nrsqstlv(),
    ",
  0x4011f3c4u64 => "
      I_3_C.hqstlv(),
    ",
  0x4011f3c8u64 => "
      I_3_C.hdbstlv(),
    ",
  0x4011f3ccu64 => "
      I_3_C.prstdbg(),
    ",
  0x4011f3d0u64 => "
      I_3_C.mserrcnt(),
    ",
  0x4011f3e0u64 => "
      I_3_C.sc1cpt(),
    ",
  0x4011f3e4u64 => "
      I_3_C.sc2cpt(),
    ",
  0x4012f200u64 => "
      ECCMB.ec710ctl(),
    ",
  0x4012f204u64 => "
      ECCMB.ec710tmc(),
    ",
  0x4012f20cu64 => "
      ECCMB.ec710ted(),
    ",
  0x4012f210u64 => "
      ECCMB.ec710ead0(),
    ",
  0x40169000u64 => "
      GPT_16_E_0.gtwp(),
    ",
  0x40169004u64 => "
      GPT_16_E_0.gtstr(),
    ",
  0x40169008u64 => "
      GPT_16_E_0.gtstp(),
    ",
  0x4016900cu64 => "
      GPT_16_E_0.gtclr(),
    ",
  0x40169010u64 => "
      GPT_16_E_0.gtssr(),
    ",
  0x40169014u64 => "
      GPT_16_E_0.gtpsr(),
    ",
  0x40169018u64 => "
      GPT_16_E_0.gtcsr(),
    ",
  0x4016901cu64 => "
      GPT_16_E_0.gtupsr(),
    ",
  0x40169020u64 => "
      GPT_16_E_0.gtdnsr(),
    ",
  0x40169024u64 => "
      GPT_16_E_0.gticasr(),
    ",
  0x40169028u64 => "
      GPT_16_E_0.gticbsr(),
    ",
  0x4016902cu64 => "
      GPT_16_E_0.gtcr(),
    ",
  0x40169030u64 => "
      GPT_16_E_0.gtuddtyc(),
    ",
  0x40169034u64 => "
      GPT_16_E_0.gtior(),
    ",
  0x40169038u64 => "
      GPT_16_E_0.gtintad(),
    ",
  0x4016903cu64 => "
      GPT_16_E_0.gtst(),
    ",
  0x40169040u64 => "
      GPT_16_E_0.gtber(),
    ",
  0x40169044u64 => "
      GPT_16_E_0.gtitc(),
    ",
  0x40169048u64 => "
      GPT_16_E_0.gtcnt(),
    ",
  0x4016904cu64 => "
      GPT_16_E_0.gtccra(),
    ",
  0x40169050u64 => "
      GPT_16_E_0.gtccrb(),
    ",
  0x40169054u64 => "
      GPT_16_E_0.gtccrc(),
    ",
  0x40169058u64 => "
      GPT_16_E_0.gtccre(),
    ",
  0x4016905cu64 => "
      GPT_16_E_0.gtccrd(),
    ",
  0x40169060u64 => "
      GPT_16_E_0.gtccrf(),
    ",
  0x40169064u64 => "
      GPT_16_E_0.gtpr(),
    ",
  0x40169068u64 => "
      GPT_16_E_0.gtpbr(),
    ",
  0x4016906cu64 => "
      GPT_16_E_0.gtpdbr(),
    ",
  0x40169070u64 => "
      GPT_16_E_0.gtadtra(),
    ",
  0x40169074u64 => "
      GPT_16_E_0.gtadtbra(),
    ",
  0x40169078u64 => "
      GPT_16_E_0.gtadtdbra(),
    ",
  0x4016907cu64 => "
      GPT_16_E_0.gtadtrb(),
    ",
  0x40169080u64 => "
      GPT_16_E_0.gtadtbrb(),
    ",
  0x40169084u64 => "
      GPT_16_E_0.gtadtdbrb(),
    ",
  0x40169088u64 => "
      GPT_16_E_0.gtdtcr(),
    ",
  0x4016908cu64 => "
      GPT_16_E_0.gtdvu(),
    ",
  0x40169090u64 => "
      GPT_16_E_0.gtdvd(),
    ",
  0x40169094u64 => "
      GPT_16_E_0.gtdbu(),
    ",
  0x40169098u64 => "
      GPT_16_E_0.gtdbd(),
    ",
  0x4016909cu64 => "
      GPT_16_E_0.gtsos(),
    ",
  0x401690a0u64 => "
      GPT_16_E_0.gtsotr(),
    ",
  0x401690a4u64 => "
      GPT_16_E_0.gtadsmr(),
    ",
  0x401690b8u64 => "
      GPT_16_E_0.gticlf(),
    ",
  0x401690bcu64 => "
      GPT_16_E_0.gtpc(),
    ",
  0x401690d0u64 => "
      GPT_16_E_0.gtsecsr(),
    ",
  0x401690d4u64 => "
      GPT_16_E_0.gtsecr(),
    ",
  0x40169a00u64 => "
      GPT_OPS.opscr(),
    ",
  0x40170000u64 => "
      ADC_120.adcsr(),
    ",
  0x40170004u64 => "
      ADC_120.adansa0(),
    ",
  0x40170006u64 => "
      ADC_120.adansa1(),
    ",
  0x40170008u64 => "
      ADC_120.adads0(),
    ",
  0x4017000au64 => "
      ADC_120.adads1(),
    ",
  0x4017000cu64 => "
      ADC_120.adadc(),
    ",
  0x4017000eu64 => "
      ADC_120.adcer(),
    ",
  0x40170010u64 => "
      ADC_120.adstrgr(),
    ",
  0x40170012u64 => "
      ADC_120.adexicr(),
    ",
  0x40170014u64 => "
      ADC_120.adansb0(),
    ",
  0x40170016u64 => "
      ADC_120.adansb1(),
    ",
  0x40170018u64 => "
      ADC_120.addbldr(),
    ",
  0x4017001au64 => "
      ADC_120.adtsdr(),
    ",
  0x4017001cu64 => "
      ADC_120.adocdr(),
    ",
  0x4017001eu64 => "
      ADC_120.adrd(),
    ",
  0x4017002eu64 => "
      ADC_120.addr()[3],
    ",
  0x40170030u64 => "
      ADC_120.addr()[4],
    ",
  0x40170036u64 => "
      ADC_120.addr()[0],
    ",
  0x40170038u64 => "
      ADC_120.addr()[1],
    ",
  0x4017003au64 => "
      ADC_120.addr()[2],
    ",
  0x40170040u64 => "
      ADC_120.addr16(),
    ",
  0x4017007au64 => "
      ADC_120.addiscr(),
    ",
  0x40170080u64 => "
      ADC_120.adgspcr(),
    ",
  0x40170084u64 => "
      ADC_120.addbldra(),
    ",
  0x40170086u64 => "
      ADC_120.addbldrb(),
    ",
  0x4017008cu64 => "
      ADC_120.adwinmon(),
    ",
  0x40170090u64 => "
      ADC_120.adcmpcr(),
    ",
  0x40170092u64 => "
      ADC_120.adcmpanser(),
    ",
  0x40170093u64 => "
      ADC_120.adcmpler(),
    ",
  0x40170094u64 => "
      ADC_120.adcmpansr0(),
    ",
  0x40170096u64 => "
      ADC_120.adcmpansr1(),
    ",
  0x40170098u64 => "
      ADC_120.adcmplr0(),
    ",
  0x4017009au64 => "
      ADC_120.adcmplr1(),
    ",
  0x4017009cu64 => "
      ADC_120.adcmpdr()[0],
    ",
  0x4017009eu64 => "
      ADC_120.adcmpdr()[1],
    ",
  0x401700a0u64 => "
      ADC_120.adcmpsr0(),
    ",
  0x401700a2u64 => "
      ADC_120.adcmpsr1(),
    ",
  0x401700a4u64 => "
      ADC_120.adcmpser(),
    ",
  0x401700a6u64 => "
      ADC_120.adcmpbnsr(),
    ",
  0x401700a8u64 => "
      ADC_120.adwinllb(),
    ",
  0x401700aau64 => "
      ADC_120.adwinulb(),
    ",
  0x401700acu64 => "
      ADC_120.adcmpbsr(),
    ",
  0x401700b0u64 => "
      ADC_120.adbuf()[0],
    ",
  0x401700b2u64 => "
      ADC_120.adbuf()[1],
    ",
  0x401700b4u64 => "
      ADC_120.adbuf()[2],
    ",
  0x401700b6u64 => "
      ADC_120.adbuf()[3],
    ",
  0x401700b8u64 => "
      ADC_120.adbuf()[4],
    ",
  0x401700bau64 => "
      ADC_120.adbuf()[5],
    ",
  0x401700bcu64 => "
      ADC_120.adbuf()[6],
    ",
  0x401700beu64 => "
      ADC_120.adbuf()[7],
    ",
  0x401700c0u64 => "
      ADC_120.adbuf()[8],
    ",
  0x401700c2u64 => "
      ADC_120.adbuf()[9],
    ",
  0x401700c4u64 => "
      ADC_120.adbuf()[10],
    ",
  0x401700c6u64 => "
      ADC_120.adbuf()[11],
    ",
  0x401700c8u64 => "
      ADC_120.adbuf()[12],
    ",
  0x401700cau64 => "
      ADC_120.adbuf()[13],
    ",
  0x401700ccu64 => "
      ADC_120.adbuf()[14],
    ",
  0x401700ceu64 => "
      ADC_120.adbuf()[15],
    ",
  0x401700d0u64 => "
      ADC_120.adbufen(),
    ",
  0x401700d2u64 => "
      ADC_120.adbufptr(),
    ",
  0x401700ddu64 => "
      ADC_120.adsstrl(),
    ",
  0x401700deu64 => "
      ADC_120.adsstrt(),
    ",
  0x401700dfu64 => "
      ADC_120.adsstro(),
    ",
  0x401700e7u64 => "
      ADC_120.adsstr()[3],
    ",
  0x401700e8u64 => "
      ADC_120.adsstr()[4],
    ",
  0x401700ebu64 => "
      ADC_120.adsstr()[0],
    ",
  0x401700ecu64 => "
      ADC_120.adsstr()[1],
    ",
  0x401700edu64 => "
      ADC_120.adsstr()[2],
    ",
  0x40171000u64 => "
      DAC_12.dadr()[0],
    ",
  0x40171002u64 => "
      DAC_12.dadr()[1],
    ",
  0x40171004u64 => "
      DAC_12.dacr(),
    ",
  0x40171005u64 => "
      DAC_12.dadpr(),
    ",
  0x40171006u64 => "
      DAC_12.daadscr(),
    ",
  0x40171008u64 => "
      DAC_12.daampcr(),
    ",
  0x4017101cu64 => "
      DAC_12.daaswcr(),
    ",
  0x401720c0u64 => "
      DAC_12.daadusr(),
    ",
  0x407fb17cu64 => "
      TSD.tscdr(),
    ",
  0x407fc040u64 => "
      FLAD.fckmhz(),
    ",
  0x407fe010u64 => "
      FACI.fastat(),
    ",
  0x407fe014u64 => "
      FACI.faeint(),
    ",
  0x407fe018u64 => "
      FACI.frdyie(),
    ",
  0x407fe030u64 => "
      FACI.fsaddr(),
    ",
  0x407fe034u64 => "
      FACI.feaddr(),
    ",
  0x407fe044u64 => "
      FACI.fmeprot(),
    ",
  0x407fe07cu64 => "
      FACI.fbprot1(),
    ",
  0x407fe080u64 => "
      FACI.fstatr(),
    ",
  0x407fe084u64 => "
      FACI.fentryr(),
    ",
  0x407fe08cu64 => "
      FACI.fsuinitr(),
    ",
  0x407fe0a0u64 => "
      FACI.fcmdr(),
    ",
  0x407fe0d0u64 => "
      FACI.fbccnt(),
    ",
  0x407fe0d4u64 => "
      FACI.fbcstat(),
    ",
  0x407fe0d8u64 => "
      FACI.fpsaddr(),
    ",
  0x407fe0dcu64 => "
      FACI.fsuasmon(),
    ",
  0x407fe0e0u64 => "
      FACI.fcpsr(),
    ",
  0x407fe0e4u64 => "
      FACI.fpckar(),
    ",
  0x407fe0e8u64 => "
      FACI.fsuacr(),
    ",
  0x64000000u64 => "
      QSPI.sfmsmd(),
    ",
  0x64000004u64 => "
      QSPI.sfmssc(),
    ",
  0x64000008u64 => "
      QSPI.sfmskc(),
    ",
  0x6400000cu64 => "
      QSPI.sfmsst(),
    ",
  0x64000010u64 => "
      QSPI.sfmcom(),
    ",
  0x64000014u64 => "
      QSPI.sfmcmd(),
    ",
  0x64000018u64 => "
      QSPI.sfmcst(),
    ",
  0x64000020u64 => "
      QSPI.sfmsic(),
    ",
  0x64000024u64 => "
      QSPI.sfmsac(),
    ",
  0x64000028u64 => "
      QSPI.sfmsdc(),
    ",
  0x64000030u64 => "
      QSPI.sfmspc(),
    ",
  0x64000034u64 => "
      QSPI.sfmpmd(),
    ",
  0x64000804u64 => "
      QSPI.sfmcnt1(),
    ",
};
