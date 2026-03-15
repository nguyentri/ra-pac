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
// Generated from SVD 1.40.00, with svd2pac 0.6.1 on Sun, 15 Mar 2026 06:37:20 +0000

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
  0x4000600fu64 => "
      ICU.irqcr()[15],
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
  0x40008050u64 => "
      CPSCU.icusare(),
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
  0x4001e03cu64 => "
      SYSC.oscsf(),
    ",
  0x4001e03eu64 => "
      SYSC.ckocr(),
    ",
  0x4001e03fu64 => "
      SYSC.trckcr(),
    ",
  0x4001e040u64 => "
      SYSC.ostdcr(),
    ",
  0x4001e041u64 => "
      SYSC.ostdsr(),
    ",
  0x4001e048u64 => "
      SYSC.pll2ccr(),
    ",
  0x4001e04au64 => "
      SYSC.pll2cr(),
    ",
  0x4001e061u64 => "
      SYSC.mocoutcr(),
    ",
  0x4001e062u64 => "
      SYSC.hocoutcr(),
    ",
  0x4001e06du64 => "
      SYSC.scispickdivcr(),
    ",
  0x4001e06eu64 => "
      SYSC.canfdckdivcr(),
    ",
  0x4001e06fu64 => "
      SYSC.gptckdivcr(),
    ",
  0x4001e070u64 => "
      SYSC.iicckdivcr(),
    ",
  0x4001e075u64 => "
      SYSC.scispickcr(),
    ",
  0x4001e076u64 => "
      SYSC.canfdckcr(),
    ",
  0x4001e077u64 => "
      SYSC.gptckcr(),
    ",
  0x4001e078u64 => "
      SYSC.iicckcr(),
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
  0x4001e406u64 => "
      SYSC.dpsifr0(),
    ",
  0x4001e407u64 => "
      SYSC.dpsifr1(),
    ",
  0x4001e408u64 => "
      SYSC.dpsifr2(),
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
  0x4001e490u64 => "
      SYSC.lococr(),
    ",
  0x4001e492u64 => "
      SYSC.locoutcr(),
    ",
  0x4001f000u64 => "
      PORT_0.pcntr1(),
      PORT_0.pdr(),
    ",
  0x4001f002u64 => "
      PORT_0.podr(),
    ",
  0x4001f004u64 => "
      PORT_0.pcntr2(),
      PORT_0.pidr(),
    ",
  0x4001f008u64 => "
      PORT_0.pcntr3(),
      PORT_0.posr(),
    ",
  0x4001f00au64 => "
      PORT_0.porr(),
    ",
  0x4001f040u64 => "
      PORT_2.pcntr1(),
      PORT_2.pdr(),
    ",
  0x4001f042u64 => "
      PORT_2.podr(),
    ",
  0x4001f044u64 => "
      PORT_2.pcntr2(),
      PORT_2.pidr(),
    ",
  0x4001f048u64 => "
      PORT_2.pcntr3(),
      PORT_2.posr(),
    ",
  0x4001f04au64 => "
      PORT_2.porr(),
    ",
  0x4001f140u64 => "
      PORTA.pcntr1(),
      PORTA.pdr(),
    ",
  0x4001f142u64 => "
      PORTA.podr(),
    ",
  0x4001f144u64 => "
      PORTA.pcntr2(),
      PORTA.pidr(),
    ",
  0x4001f148u64 => "
      PORTA.pcntr3(),
      PORTA.posr(),
    ",
  0x4001f14au64 => "
      PORTA.porr(),
    ",
  0x4001f160u64 => "
      PORTB.pcntr1(),
      PORTB.pdr(),
    ",
  0x4001f162u64 => "
      PORTB.podr(),
    ",
  0x4001f164u64 => "
      PORTB.pcntr2(),
      PORTB.pidr(),
    ",
  0x4001f166u64 => "
      PORTB.eidr(),
    ",
  0x4001f168u64 => "
      PORTB.pcntr3(),
      PORTB.posr(),
    ",
  0x4001f16au64 => "
      PORTB.porr(),
    ",
  0x4001f16cu64 => "
      PORTB.pcntr4(),
      PORTB.eosr(),
    ",
  0x4001f16eu64 => "
      PORTB.eorr(),
    ",
  0x4001f180u64 => "
      PORTC.pcntr1(),
      PORTC.pdr(),
    ",
  0x4001f182u64 => "
      PORTC.podr(),
    ",
  0x4001f184u64 => "
      PORTC.pcntr2(),
      PORTC.pidr(),
    ",
  0x4001f186u64 => "
      PORTC.eidr(),
    ",
  0x4001f188u64 => "
      PORTC.pcntr3(),
      PORTC.posr(),
    ",
  0x4001f18au64 => "
      PORTC.porr(),
    ",
  0x4001f18cu64 => "
      PORTC.pcntr4(),
      PORTC.eosr(),
    ",
  0x4001f18eu64 => "
      PORTC.eorr(),
    ",
  0x4001f1a0u64 => "
      PORTD.pcntr1(),
      PORTD.pdr(),
    ",
  0x4001f1a2u64 => "
      PORTD.podr(),
    ",
  0x4001f1a4u64 => "
      PORTD.pcntr2(),
      PORTD.pidr(),
    ",
  0x4001f1a6u64 => "
      PORTD.eidr(),
    ",
  0x4001f1a8u64 => "
      PORTD.pcntr3(),
      PORTD.posr(),
    ",
  0x4001f1aau64 => "
      PORTD.porr(),
    ",
  0x4001f1acu64 => "
      PORTD.pcntr4(),
      PORTD.eosr(),
    ",
  0x4001f1aeu64 => "
      PORTD.eorr(),
    ",
  0x4001f1c0u64 => "
      PORTE.pcntr1(),
      PORTE.pdr(),
    ",
  0x4001f1c2u64 => "
      PORTE.podr(),
    ",
  0x4001f1c4u64 => "
      PORTE.pcntr2(),
      PORTE.pidr(),
    ",
  0x4001f1c6u64 => "
      PORTE.eidr(),
    ",
  0x4001f1c8u64 => "
      PORTE.pcntr3(),
      PORTE.posr(),
    ",
  0x4001f1cau64 => "
      PORTE.porr(),
    ",
  0x4001f1ccu64 => "
      PORTE.pcntr4(),
      PORTE.eosr(),
    ",
  0x4001f1ceu64 => "
      PORTE.eorr(),
    ",
  0x4001f800u64 => "
      PFS_B.p00pfs()[0],
      PFS_B.p00pfs_ha()[0],
      PFS_B.p00pfs_by()[0],
    ",
  0x4001f804u64 => "
      PFS_B.p00pfs()[1],
      PFS_B.p00pfs_ha()[1],
      PFS_B.p00pfs_by()[1],
    ",
  0x4001f808u64 => "
      PFS_B.p002pfs(),
      PFS_B.p002pfs_ha(),
      PFS_B.p002pfs_by(),
    ",
  0x4001f884u64 => "
      PFS_B.p201pfs(),
      PFS_B.p201pfs_ha(),
      PFS_B.p201pfs_by(),
    ",
  0x4001fa90u64 => "
      PFS_B.pa0pfs()[4],
      PFS_B.pa0pfs_ha()[4],
      PFS_B.pa0pfs_by()[4],
    ",
  0x4001fa94u64 => "
      PFS_B.pa0pfs()[5],
      PFS_B.pa0pfs_ha()[5],
      PFS_B.pa0pfs_by()[5],
    ",
  0x4001fa98u64 => "
      PFS_B.pa0pfs()[0],
      PFS_B.pa0pfs_ha()[0],
      PFS_B.pa0pfs_by()[0],
    ",
  0x4001fa9cu64 => "
      PFS_B.pa0pfs()[1],
      PFS_B.pa0pfs_ha()[1],
      PFS_B.pa0pfs_by()[1],
    ",
  0x4001faa0u64 => "
      PFS_B.pa0pfs()[2],
      PFS_B.pa0pfs_ha()[2],
      PFS_B.pa0pfs_by()[2],
    ",
  0x4001faa4u64 => "
      PFS_B.pa0pfs()[3],
      PFS_B.pa0pfs_ha()[3],
      PFS_B.pa0pfs_by()[3],
    ",
  0x4001fab0u64 => "
      PFS_B.pa1pfs()[2],
      PFS_B.pa1pfs_ha()[2],
      PFS_B.pa1pfs_by()[2],
    ",
  0x4001fab4u64 => "
      PFS_B.pa13pfs(),
      PFS_B.pa13pfs_ha(),
      PFS_B.pa13pfs_by(),
    ",
  0x4001fab8u64 => "
      PFS_B.pa1pfs()[0],
      PFS_B.pa1pfs_ha()[0],
      PFS_B.pa1pfs_by()[0],
    ",
  0x4001fabcu64 => "
      PFS_B.pa1pfs()[1],
      PFS_B.pa1pfs_ha()[1],
      PFS_B.pa1pfs_by()[1],
    ",
  0x4001fac8u64 => "
      PFS_B.pb02pfs(),
      PFS_B.pb02pfs_ha(),
      PFS_B.pb02pfs_by(),
    ",
  0x4001faccu64 => "
      PFS_B.pb03pfs(),
      PFS_B.pb03pfs_ha(),
      PFS_B.pb03pfs_by(),
    ",
  0x4001fad0u64 => "
      PFS_B.pb0pfs()[0],
      PFS_B.pb0pfs_ha()[0],
      PFS_B.pb0pfs_by()[0],
    ",
  0x4001fad4u64 => "
      PFS_B.pb0pfs()[1],
      PFS_B.pb0pfs_ha()[1],
      PFS_B.pb0pfs_by()[1],
    ",
  0x4001fad8u64 => "
      PFS_B.pb0pfs()[2],
      PFS_B.pb0pfs_ha()[2],
      PFS_B.pb0pfs_by()[2],
    ",
  0x4001fadcu64 => "
      PFS_B.pb0pfs()[3],
      PFS_B.pb0pfs_ha()[3],
      PFS_B.pb0pfs_by()[3],
    ",
  0x4001fae0u64 => "
      PFS_B.pb0pfs()[4],
      PFS_B.pb0pfs_ha()[4],
      PFS_B.pb0pfs_by()[4],
    ",
  0x4001fae4u64 => "
      PFS_B.pb0pfs()[5],
      PFS_B.pb0pfs_ha()[5],
      PFS_B.pb0pfs_by()[5],
    ",
  0x4001fae8u64 => "
      PFS_B.pb10pfs(),
      PFS_B.pb10pfs_ha(),
      PFS_B.pb10pfs_by(),
    ",
  0x4001faf0u64 => "
      PFS_B.pb1pfs()[0],
      PFS_B.pb1pfs_ha()[0],
      PFS_B.pb1pfs_by()[0],
    ",
  0x4001faf4u64 => "
      PFS_B.pb1pfs()[1],
      PFS_B.pb1pfs_ha()[1],
      PFS_B.pb1pfs_by()[1],
    ",
  0x4001faf8u64 => "
      PFS_B.pb1pfs()[2],
      PFS_B.pb1pfs_ha()[2],
      PFS_B.pb1pfs_by()[2],
    ",
  0x4001fafcu64 => "
      PFS_B.pb1pfs()[3],
      PFS_B.pb1pfs_ha()[3],
      PFS_B.pb1pfs_by()[3],
    ",
  0x4001fb00u64 => "
      PFS_B.pc0pfs()[0],
      PFS_B.pc0pfs_ha()[0],
      PFS_B.pc0pfs_by()[0],
    ",
  0x4001fb04u64 => "
      PFS_B.pc0pfs()[1],
      PFS_B.pc0pfs_ha()[1],
      PFS_B.pc0pfs_by()[1],
    ",
  0x4001fb08u64 => "
      PFS_B.pc0pfs()[2],
      PFS_B.pc0pfs_ha()[2],
      PFS_B.pc0pfs_by()[2],
    ",
  0x4001fb0cu64 => "
      PFS_B.pc0pfs()[3],
      PFS_B.pc0pfs_ha()[3],
      PFS_B.pc0pfs_by()[3],
    ",
  0x4001fb10u64 => "
      PFS_B.pc0pfs()[4],
      PFS_B.pc0pfs_ha()[4],
      PFS_B.pc0pfs_by()[4],
    ",
  0x4001fb14u64 => "
      PFS_B.pc0pfs()[5],
      PFS_B.pc0pfs_ha()[5],
      PFS_B.pc0pfs_by()[5],
    ",
  0x4001fb18u64 => "
      PFS_B.pc0pfs()[6],
      PFS_B.pc0pfs_ha()[6],
      PFS_B.pc0pfs_by()[6],
    ",
  0x4001fb1cu64 => "
      PFS_B.pc0pfs()[7],
      PFS_B.pc0pfs_ha()[7],
      PFS_B.pc0pfs_by()[7],
    ",
  0x4001fb20u64 => "
      PFS_B.pc0pfs()[8],
      PFS_B.pc0pfs_ha()[8],
      PFS_B.pc0pfs_by()[8],
    ",
  0x4001fb24u64 => "
      PFS_B.pc0pfs()[9],
      PFS_B.pc0pfs_ha()[9],
      PFS_B.pc0pfs_by()[9],
    ",
  0x4001fb28u64 => "
      PFS_B.pc1pfs()[0],
      PFS_B.pc1pfs_ha()[0],
      PFS_B.pc1pfs_by()[0],
    ",
  0x4001fb2cu64 => "
      PFS_B.pc1pfs()[1],
      PFS_B.pc1pfs_ha()[1],
      PFS_B.pc1pfs_by()[1],
    ",
  0x4001fb30u64 => "
      PFS_B.pc1pfs()[2],
      PFS_B.pc1pfs_ha()[2],
      PFS_B.pc1pfs_by()[2],
    ",
  0x4001fb34u64 => "
      PFS_B.pc1pfs()[3],
      PFS_B.pc1pfs_ha()[3],
      PFS_B.pc1pfs_by()[3],
    ",
  0x4001fb38u64 => "
      PFS_B.pc1pfs()[4],
      PFS_B.pc1pfs_ha()[4],
      PFS_B.pc1pfs_by()[4],
    ",
  0x4001fb3cu64 => "
      PFS_B.pc1pfs()[5],
      PFS_B.pc1pfs_ha()[5],
      PFS_B.pc1pfs_by()[5],
    ",
  0x4001fb40u64 => "
      PFS_B.pd0pfs()[0],
      PFS_B.pd0pfs_ha()[0],
      PFS_B.pd0pfs_by()[0],
    ",
  0x4001fb44u64 => "
      PFS_B.pd0pfs()[1],
      PFS_B.pd0pfs_ha()[1],
      PFS_B.pd0pfs_by()[1],
    ",
  0x4001fb48u64 => "
      PFS_B.pd0pfs()[2],
      PFS_B.pd0pfs_ha()[2],
      PFS_B.pd0pfs_by()[2],
    ",
  0x4001fb4cu64 => "
      PFS_B.pd0pfs()[3],
      PFS_B.pd0pfs_ha()[3],
      PFS_B.pd0pfs_by()[3],
    ",
  0x4001fb50u64 => "
      PFS_B.pd0pfs()[4],
      PFS_B.pd0pfs_ha()[4],
      PFS_B.pd0pfs_by()[4],
    ",
  0x4001fb54u64 => "
      PFS_B.pd0pfs()[5],
      PFS_B.pd0pfs_ha()[5],
      PFS_B.pd0pfs_by()[5],
    ",
  0x4001fb58u64 => "
      PFS_B.pd0pfs()[6],
      PFS_B.pd0pfs_ha()[6],
      PFS_B.pd0pfs_by()[6],
    ",
  0x4001fb5cu64 => "
      PFS_B.pd0pfs()[7],
      PFS_B.pd0pfs_ha()[7],
      PFS_B.pd0pfs_by()[7],
    ",
  0x4001fb60u64 => "
      PFS_B.pd0pfs()[8],
      PFS_B.pd0pfs_ha()[8],
      PFS_B.pd0pfs_by()[8],
    ",
  0x4001fb64u64 => "
      PFS_B.pd0pfs()[9],
      PFS_B.pd0pfs_ha()[9],
      PFS_B.pd0pfs_by()[9],
    ",
  0x4001fb68u64 => "
      PFS_B.pd1pfs()[0],
      PFS_B.pd1pfs_ha()[0],
      PFS_B.pd1pfs_by()[0],
    ",
  0x4001fb6cu64 => "
      PFS_B.pd1pfs()[1],
      PFS_B.pd1pfs_ha()[1],
      PFS_B.pd1pfs_by()[1],
    ",
  0x4001fb70u64 => "
      PFS_B.pd1pfs()[2],
      PFS_B.pd1pfs_ha()[2],
      PFS_B.pd1pfs_by()[2],
    ",
  0x4001fb74u64 => "
      PFS_B.pd1pfs()[3],
      PFS_B.pd1pfs_ha()[3],
      PFS_B.pd1pfs_by()[3],
    ",
  0x4001fb78u64 => "
      PFS_B.pd1pfs()[4],
      PFS_B.pd1pfs_ha()[4],
      PFS_B.pd1pfs_by()[4],
    ",
  0x4001fb7cu64 => "
      PFS_B.pd1pfs()[5],
      PFS_B.pd1pfs_ha()[5],
      PFS_B.pd1pfs_by()[5],
    ",
  0x4001fb88u64 => "
      PFS_B.pe0pfs()[2],
      PFS_B.pe0pfs_ha()[2],
      PFS_B.pe0pfs_by()[2],
    ",
  0x4001fb8cu64 => "
      PFS_B.pe0pfs()[3],
      PFS_B.pe0pfs_ha()[3],
      PFS_B.pe0pfs_by()[3],
    ",
  0x4001fb90u64 => "
      PFS_B.pe0pfs()[4],
      PFS_B.pe0pfs_ha()[4],
      PFS_B.pe0pfs_by()[4],
    ",
  0x4001fb94u64 => "
      PFS_B.pe0pfs()[5],
      PFS_B.pe0pfs_ha()[5],
      PFS_B.pe0pfs_by()[5],
    ",
  0x4001fb98u64 => "
      PFS_B.pe0pfs()[6],
      PFS_B.pe0pfs_ha()[6],
      PFS_B.pe0pfs_by()[6],
    ",
  0x4001fba0u64 => "
      PFS_B.pe0pfs()[0],
      PFS_B.pe0pfs_ha()[0],
      PFS_B.pe0pfs_by()[0],
    ",
  0x4001fba4u64 => "
      PFS_B.pe0pfs()[1],
      PFS_B.pe0pfs_ha()[1],
      PFS_B.pe0pfs_by()[1],
    ",
  0x4001fba8u64 => "
      PFS_B.pe1pfs()[0],
      PFS_B.pe1pfs_ha()[0],
      PFS_B.pe1pfs_by()[0],
    ",
  0x4001fbacu64 => "
      PFS_B.pe1pfs()[1],
      PFS_B.pe1pfs_ha()[1],
      PFS_B.pe1pfs_by()[1],
    ",
  0x4001fbb0u64 => "
      PFS_B.pe1pfs()[2],
      PFS_B.pe1pfs_ha()[2],
      PFS_B.pe1pfs_by()[2],
    ",
  0x4001fbb4u64 => "
      PFS_B.pe1pfs()[3],
      PFS_B.pe1pfs_ha()[3],
      PFS_B.pe1pfs_by()[3],
    ",
  0x4001fbb8u64 => "
      PFS_B.pe1pfs()[4],
      PFS_B.pe1pfs_ha()[4],
      PFS_B.pe1pfs_by()[4],
    ",
  0x4001fbbcu64 => "
      PFS_B.pe1pfs()[5],
      PFS_B.pe1pfs_ha()[5],
      PFS_B.pe1pfs_by()[5],
    ",
  0x4001fd0cu64 => "
      PFS_B.pwpr(),
    ",
  0x4001fd14u64 => "
      PFS_B.pwprs(),
    ",
  0x4001fd30u64 => "
      PFS_B.p0sar(),
    ",
  0x4001fd38u64 => "
      PFS_B.p2sar(),
    ",
  0x4001fd58u64 => "
      PFS_B.pasar(),
    ",
  0x4001fd5cu64 => "
      PFS_B.pbsar(),
    ",
  0x4001fd60u64 => "
      PFS_B.pcsar(),
    ",
  0x4001fd64u64 => "
      PFS_B.pdsar(),
    ",
  0x4001fd68u64 => "
      PFS_B.pesar(),
    ",
  0x40020000u64 => "
      IIRFA.iircprcs(),
    ",
  0x40020004u64 => "
      IIRFA.iircprcff(),
    ",
  0x40020008u64 => "
      IIRFA.iirordyf(),
    ",
  0x4002000cu64 => "
      IIRFA.iircerrf(),
    ",
  0x40020010u64 => "
      IIRFA.iiropcnt(),
    ",
  0x40020020u64 => "
      IIRFA.iirecccnt(),
    ",
  0x40020028u64 => "
      IIRFA.iireccint(),
    ",
  0x40020030u64 => "
      IIRFA.iireccef(),
    ",
  0x40020034u64 => "
      IIRFA.iireccefclr(),
    ",
  0x40020038u64 => "
      IIRFA.iireseadr(),
    ",
  0x4002003cu64 => "
      IIRFA.iiredeadr(),
    ",
  0x40020100u64 => "
      IIRFA.iirchinp()[0],
    ",
  0x40020110u64 => "
      IIRFA.iirchinp()[1],
    ",
  0x40020120u64 => "
      IIRFA.iirchinp()[2],
    ",
  0x40020130u64 => "
      IIRFA.iirchinp()[3],
    ",
  0x40020140u64 => "
      IIRFA.iirchinp()[4],
    ",
  0x40020150u64 => "
      IIRFA.iirchinp()[5],
    ",
  0x40020160u64 => "
      IIRFA.iirchinp()[6],
    ",
  0x40020170u64 => "
      IIRFA.iirchinp()[7],
    ",
  0x40020180u64 => "
      IIRFA.iirchinp()[8],
    ",
  0x40020190u64 => "
      IIRFA.iirchinp()[9],
    ",
  0x400201a0u64 => "
      IIRFA.iirchinp()[10],
    ",
  0x400201b0u64 => "
      IIRFA.iirchinp()[11],
    ",
  0x400201c0u64 => "
      IIRFA.iirchinp()[12],
    ",
  0x400201d0u64 => "
      IIRFA.iirchinp()[13],
    ",
  0x400201e0u64 => "
      IIRFA.iirchinp()[14],
    ",
  0x400201f0u64 => "
      IIRFA.iirchinp()[15],
    ",
  0x40020104u64 => "
      IIRFA.iirchout()[0],
    ",
  0x40020114u64 => "
      IIRFA.iirchout()[1],
    ",
  0x40020124u64 => "
      IIRFA.iirchout()[2],
    ",
  0x40020134u64 => "
      IIRFA.iirchout()[3],
    ",
  0x40020144u64 => "
      IIRFA.iirchout()[4],
    ",
  0x40020154u64 => "
      IIRFA.iirchout()[5],
    ",
  0x40020164u64 => "
      IIRFA.iirchout()[6],
    ",
  0x40020174u64 => "
      IIRFA.iirchout()[7],
    ",
  0x40020184u64 => "
      IIRFA.iirchout()[8],
    ",
  0x40020194u64 => "
      IIRFA.iirchout()[9],
    ",
  0x400201a4u64 => "
      IIRFA.iirchout()[10],
    ",
  0x400201b4u64 => "
      IIRFA.iirchout()[11],
    ",
  0x400201c4u64 => "
      IIRFA.iirchout()[12],
    ",
  0x400201d4u64 => "
      IIRFA.iirchout()[13],
    ",
  0x400201e4u64 => "
      IIRFA.iirchout()[14],
    ",
  0x400201f4u64 => "
      IIRFA.iirchout()[15],
    ",
  0x40020108u64 => "
      IIRFA.iirchcnt()[0],
    ",
  0x40020118u64 => "
      IIRFA.iirchcnt()[1],
    ",
  0x40020128u64 => "
      IIRFA.iirchcnt()[2],
    ",
  0x40020138u64 => "
      IIRFA.iirchcnt()[3],
    ",
  0x40020148u64 => "
      IIRFA.iirchcnt()[4],
    ",
  0x40020158u64 => "
      IIRFA.iirchcnt()[5],
    ",
  0x40020168u64 => "
      IIRFA.iirchcnt()[6],
    ",
  0x40020178u64 => "
      IIRFA.iirchcnt()[7],
    ",
  0x40020188u64 => "
      IIRFA.iirchcnt()[8],
    ",
  0x40020198u64 => "
      IIRFA.iirchcnt()[9],
    ",
  0x400201a8u64 => "
      IIRFA.iirchcnt()[10],
    ",
  0x400201b8u64 => "
      IIRFA.iirchcnt()[11],
    ",
  0x400201c8u64 => "
      IIRFA.iirchcnt()[12],
    ",
  0x400201d8u64 => "
      IIRFA.iirchcnt()[13],
    ",
  0x400201e8u64 => "
      IIRFA.iirchcnt()[14],
    ",
  0x400201f8u64 => "
      IIRFA.iirchcnt()[15],
    ",
  0x4002010cu64 => "
      IIRFA.iirchint()[0],
    ",
  0x4002011cu64 => "
      IIRFA.iirchint()[1],
    ",
  0x4002012cu64 => "
      IIRFA.iirchint()[2],
    ",
  0x4002013cu64 => "
      IIRFA.iirchint()[3],
    ",
  0x4002014cu64 => "
      IIRFA.iirchint()[4],
    ",
  0x4002015cu64 => "
      IIRFA.iirchint()[5],
    ",
  0x4002016cu64 => "
      IIRFA.iirchint()[6],
    ",
  0x4002017cu64 => "
      IIRFA.iirchint()[7],
    ",
  0x4002018cu64 => "
      IIRFA.iirchint()[8],
    ",
  0x4002019cu64 => "
      IIRFA.iirchint()[9],
    ",
  0x400201acu64 => "
      IIRFA.iirchint()[10],
    ",
  0x400201bcu64 => "
      IIRFA.iirchint()[11],
    ",
  0x400201ccu64 => "
      IIRFA.iirchint()[12],
    ",
  0x400201dcu64 => "
      IIRFA.iirchint()[13],
    ",
  0x400201ecu64 => "
      IIRFA.iirchint()[14],
    ",
  0x400201fcu64 => "
      IIRFA.iirchint()[15],
    ",
  0x4002010du64 => "
      IIRFA.iirchsts()[0],
    ",
  0x4002011du64 => "
      IIRFA.iirchsts()[1],
    ",
  0x4002012du64 => "
      IIRFA.iirchsts()[2],
    ",
  0x4002013du64 => "
      IIRFA.iirchsts()[3],
    ",
  0x4002014du64 => "
      IIRFA.iirchsts()[4],
    ",
  0x4002015du64 => "
      IIRFA.iirchsts()[5],
    ",
  0x4002016du64 => "
      IIRFA.iirchsts()[6],
    ",
  0x4002017du64 => "
      IIRFA.iirchsts()[7],
    ",
  0x4002018du64 => "
      IIRFA.iirchsts()[8],
    ",
  0x4002019du64 => "
      IIRFA.iirchsts()[9],
    ",
  0x400201adu64 => "
      IIRFA.iirchsts()[10],
    ",
  0x400201bdu64 => "
      IIRFA.iirchsts()[11],
    ",
  0x400201cdu64 => "
      IIRFA.iirchsts()[12],
    ",
  0x400201ddu64 => "
      IIRFA.iirchsts()[13],
    ",
  0x400201edu64 => "
      IIRFA.iirchsts()[14],
    ",
  0x400201fdu64 => "
      IIRFA.iirchsts()[15],
    ",
  0x4002010eu64 => "
      IIRFA.iirchfclr()[0],
    ",
  0x4002011eu64 => "
      IIRFA.iirchfclr()[1],
    ",
  0x4002012eu64 => "
      IIRFA.iirchfclr()[2],
    ",
  0x4002013eu64 => "
      IIRFA.iirchfclr()[3],
    ",
  0x4002014eu64 => "
      IIRFA.iirchfclr()[4],
    ",
  0x4002015eu64 => "
      IIRFA.iirchfclr()[5],
    ",
  0x4002016eu64 => "
      IIRFA.iirchfclr()[6],
    ",
  0x4002017eu64 => "
      IIRFA.iirchfclr()[7],
    ",
  0x4002018eu64 => "
      IIRFA.iirchfclr()[8],
    ",
  0x4002019eu64 => "
      IIRFA.iirchfclr()[9],
    ",
  0x400201aeu64 => "
      IIRFA.iirchfclr()[10],
    ",
  0x400201beu64 => "
      IIRFA.iirchfclr()[11],
    ",
  0x400201ceu64 => "
      IIRFA.iirchfclr()[12],
    ",
  0x400201deu64 => "
      IIRFA.iirchfclr()[13],
    ",
  0x400201eeu64 => "
      IIRFA.iirchfclr()[14],
    ",
  0x400201feu64 => "
      IIRFA.iirchfclr()[15],
    ",
  0x40020400u64 => "
      IIRFA.iirstgb0()[0],
    ",
  0x40020420u64 => "
      IIRFA.iirstgb0()[1],
    ",
  0x40020440u64 => "
      IIRFA.iirstgb0()[2],
    ",
  0x40020460u64 => "
      IIRFA.iirstgb0()[3],
    ",
  0x40020480u64 => "
      IIRFA.iirstgb0()[4],
    ",
  0x400204a0u64 => "
      IIRFA.iirstgb0()[5],
    ",
  0x400204c0u64 => "
      IIRFA.iirstgb0()[6],
    ",
  0x400204e0u64 => "
      IIRFA.iirstgb0()[7],
    ",
  0x40020500u64 => "
      IIRFA.iirstgb0()[8],
    ",
  0x40020520u64 => "
      IIRFA.iirstgb0()[9],
    ",
  0x40020540u64 => "
      IIRFA.iirstgb0()[10],
    ",
  0x40020560u64 => "
      IIRFA.iirstgb0()[11],
    ",
  0x40020580u64 => "
      IIRFA.iirstgb0()[12],
    ",
  0x400205a0u64 => "
      IIRFA.iirstgb0()[13],
    ",
  0x400205c0u64 => "
      IIRFA.iirstgb0()[14],
    ",
  0x400205e0u64 => "
      IIRFA.iirstgb0()[15],
    ",
  0x40020600u64 => "
      IIRFA.iirstgb0()[16],
    ",
  0x40020620u64 => "
      IIRFA.iirstgb0()[17],
    ",
  0x40020640u64 => "
      IIRFA.iirstgb0()[18],
    ",
  0x40020660u64 => "
      IIRFA.iirstgb0()[19],
    ",
  0x40020680u64 => "
      IIRFA.iirstgb0()[20],
    ",
  0x400206a0u64 => "
      IIRFA.iirstgb0()[21],
    ",
  0x400206c0u64 => "
      IIRFA.iirstgb0()[22],
    ",
  0x400206e0u64 => "
      IIRFA.iirstgb0()[23],
    ",
  0x40020700u64 => "
      IIRFA.iirstgb0()[24],
    ",
  0x40020720u64 => "
      IIRFA.iirstgb0()[25],
    ",
  0x40020740u64 => "
      IIRFA.iirstgb0()[26],
    ",
  0x40020760u64 => "
      IIRFA.iirstgb0()[27],
    ",
  0x40020780u64 => "
      IIRFA.iirstgb0()[28],
    ",
  0x400207a0u64 => "
      IIRFA.iirstgb0()[29],
    ",
  0x400207c0u64 => "
      IIRFA.iirstgb0()[30],
    ",
  0x400207e0u64 => "
      IIRFA.iirstgb0()[31],
    ",
  0x40020404u64 => "
      IIRFA.iirstgb1()[0],
    ",
  0x40020424u64 => "
      IIRFA.iirstgb1()[1],
    ",
  0x40020444u64 => "
      IIRFA.iirstgb1()[2],
    ",
  0x40020464u64 => "
      IIRFA.iirstgb1()[3],
    ",
  0x40020484u64 => "
      IIRFA.iirstgb1()[4],
    ",
  0x400204a4u64 => "
      IIRFA.iirstgb1()[5],
    ",
  0x400204c4u64 => "
      IIRFA.iirstgb1()[6],
    ",
  0x400204e4u64 => "
      IIRFA.iirstgb1()[7],
    ",
  0x40020504u64 => "
      IIRFA.iirstgb1()[8],
    ",
  0x40020524u64 => "
      IIRFA.iirstgb1()[9],
    ",
  0x40020544u64 => "
      IIRFA.iirstgb1()[10],
    ",
  0x40020564u64 => "
      IIRFA.iirstgb1()[11],
    ",
  0x40020584u64 => "
      IIRFA.iirstgb1()[12],
    ",
  0x400205a4u64 => "
      IIRFA.iirstgb1()[13],
    ",
  0x400205c4u64 => "
      IIRFA.iirstgb1()[14],
    ",
  0x400205e4u64 => "
      IIRFA.iirstgb1()[15],
    ",
  0x40020604u64 => "
      IIRFA.iirstgb1()[16],
    ",
  0x40020624u64 => "
      IIRFA.iirstgb1()[17],
    ",
  0x40020644u64 => "
      IIRFA.iirstgb1()[18],
    ",
  0x40020664u64 => "
      IIRFA.iirstgb1()[19],
    ",
  0x40020684u64 => "
      IIRFA.iirstgb1()[20],
    ",
  0x400206a4u64 => "
      IIRFA.iirstgb1()[21],
    ",
  0x400206c4u64 => "
      IIRFA.iirstgb1()[22],
    ",
  0x400206e4u64 => "
      IIRFA.iirstgb1()[23],
    ",
  0x40020704u64 => "
      IIRFA.iirstgb1()[24],
    ",
  0x40020724u64 => "
      IIRFA.iirstgb1()[25],
    ",
  0x40020744u64 => "
      IIRFA.iirstgb1()[26],
    ",
  0x40020764u64 => "
      IIRFA.iirstgb1()[27],
    ",
  0x40020784u64 => "
      IIRFA.iirstgb1()[28],
    ",
  0x400207a4u64 => "
      IIRFA.iirstgb1()[29],
    ",
  0x400207c4u64 => "
      IIRFA.iirstgb1()[30],
    ",
  0x400207e4u64 => "
      IIRFA.iirstgb1()[31],
    ",
  0x40020408u64 => "
      IIRFA.iirstgb2()[0],
    ",
  0x40020428u64 => "
      IIRFA.iirstgb2()[1],
    ",
  0x40020448u64 => "
      IIRFA.iirstgb2()[2],
    ",
  0x40020468u64 => "
      IIRFA.iirstgb2()[3],
    ",
  0x40020488u64 => "
      IIRFA.iirstgb2()[4],
    ",
  0x400204a8u64 => "
      IIRFA.iirstgb2()[5],
    ",
  0x400204c8u64 => "
      IIRFA.iirstgb2()[6],
    ",
  0x400204e8u64 => "
      IIRFA.iirstgb2()[7],
    ",
  0x40020508u64 => "
      IIRFA.iirstgb2()[8],
    ",
  0x40020528u64 => "
      IIRFA.iirstgb2()[9],
    ",
  0x40020548u64 => "
      IIRFA.iirstgb2()[10],
    ",
  0x40020568u64 => "
      IIRFA.iirstgb2()[11],
    ",
  0x40020588u64 => "
      IIRFA.iirstgb2()[12],
    ",
  0x400205a8u64 => "
      IIRFA.iirstgb2()[13],
    ",
  0x400205c8u64 => "
      IIRFA.iirstgb2()[14],
    ",
  0x400205e8u64 => "
      IIRFA.iirstgb2()[15],
    ",
  0x40020608u64 => "
      IIRFA.iirstgb2()[16],
    ",
  0x40020628u64 => "
      IIRFA.iirstgb2()[17],
    ",
  0x40020648u64 => "
      IIRFA.iirstgb2()[18],
    ",
  0x40020668u64 => "
      IIRFA.iirstgb2()[19],
    ",
  0x40020688u64 => "
      IIRFA.iirstgb2()[20],
    ",
  0x400206a8u64 => "
      IIRFA.iirstgb2()[21],
    ",
  0x400206c8u64 => "
      IIRFA.iirstgb2()[22],
    ",
  0x400206e8u64 => "
      IIRFA.iirstgb2()[23],
    ",
  0x40020708u64 => "
      IIRFA.iirstgb2()[24],
    ",
  0x40020728u64 => "
      IIRFA.iirstgb2()[25],
    ",
  0x40020748u64 => "
      IIRFA.iirstgb2()[26],
    ",
  0x40020768u64 => "
      IIRFA.iirstgb2()[27],
    ",
  0x40020788u64 => "
      IIRFA.iirstgb2()[28],
    ",
  0x400207a8u64 => "
      IIRFA.iirstgb2()[29],
    ",
  0x400207c8u64 => "
      IIRFA.iirstgb2()[30],
    ",
  0x400207e8u64 => "
      IIRFA.iirstgb2()[31],
    ",
  0x4002040cu64 => "
      IIRFA.iirstga1()[0],
    ",
  0x4002042cu64 => "
      IIRFA.iirstga1()[1],
    ",
  0x4002044cu64 => "
      IIRFA.iirstga1()[2],
    ",
  0x4002046cu64 => "
      IIRFA.iirstga1()[3],
    ",
  0x4002048cu64 => "
      IIRFA.iirstga1()[4],
    ",
  0x400204acu64 => "
      IIRFA.iirstga1()[5],
    ",
  0x400204ccu64 => "
      IIRFA.iirstga1()[6],
    ",
  0x400204ecu64 => "
      IIRFA.iirstga1()[7],
    ",
  0x4002050cu64 => "
      IIRFA.iirstga1()[8],
    ",
  0x4002052cu64 => "
      IIRFA.iirstga1()[9],
    ",
  0x4002054cu64 => "
      IIRFA.iirstga1()[10],
    ",
  0x4002056cu64 => "
      IIRFA.iirstga1()[11],
    ",
  0x4002058cu64 => "
      IIRFA.iirstga1()[12],
    ",
  0x400205acu64 => "
      IIRFA.iirstga1()[13],
    ",
  0x400205ccu64 => "
      IIRFA.iirstga1()[14],
    ",
  0x400205ecu64 => "
      IIRFA.iirstga1()[15],
    ",
  0x4002060cu64 => "
      IIRFA.iirstga1()[16],
    ",
  0x4002062cu64 => "
      IIRFA.iirstga1()[17],
    ",
  0x4002064cu64 => "
      IIRFA.iirstga1()[18],
    ",
  0x4002066cu64 => "
      IIRFA.iirstga1()[19],
    ",
  0x4002068cu64 => "
      IIRFA.iirstga1()[20],
    ",
  0x400206acu64 => "
      IIRFA.iirstga1()[21],
    ",
  0x400206ccu64 => "
      IIRFA.iirstga1()[22],
    ",
  0x400206ecu64 => "
      IIRFA.iirstga1()[23],
    ",
  0x4002070cu64 => "
      IIRFA.iirstga1()[24],
    ",
  0x4002072cu64 => "
      IIRFA.iirstga1()[25],
    ",
  0x4002074cu64 => "
      IIRFA.iirstga1()[26],
    ",
  0x4002076cu64 => "
      IIRFA.iirstga1()[27],
    ",
  0x4002078cu64 => "
      IIRFA.iirstga1()[28],
    ",
  0x400207acu64 => "
      IIRFA.iirstga1()[29],
    ",
  0x400207ccu64 => "
      IIRFA.iirstga1()[30],
    ",
  0x400207ecu64 => "
      IIRFA.iirstga1()[31],
    ",
  0x40020410u64 => "
      IIRFA.iirstga2()[0],
    ",
  0x40020430u64 => "
      IIRFA.iirstga2()[1],
    ",
  0x40020450u64 => "
      IIRFA.iirstga2()[2],
    ",
  0x40020470u64 => "
      IIRFA.iirstga2()[3],
    ",
  0x40020490u64 => "
      IIRFA.iirstga2()[4],
    ",
  0x400204b0u64 => "
      IIRFA.iirstga2()[5],
    ",
  0x400204d0u64 => "
      IIRFA.iirstga2()[6],
    ",
  0x400204f0u64 => "
      IIRFA.iirstga2()[7],
    ",
  0x40020510u64 => "
      IIRFA.iirstga2()[8],
    ",
  0x40020530u64 => "
      IIRFA.iirstga2()[9],
    ",
  0x40020550u64 => "
      IIRFA.iirstga2()[10],
    ",
  0x40020570u64 => "
      IIRFA.iirstga2()[11],
    ",
  0x40020590u64 => "
      IIRFA.iirstga2()[12],
    ",
  0x400205b0u64 => "
      IIRFA.iirstga2()[13],
    ",
  0x400205d0u64 => "
      IIRFA.iirstga2()[14],
    ",
  0x400205f0u64 => "
      IIRFA.iirstga2()[15],
    ",
  0x40020610u64 => "
      IIRFA.iirstga2()[16],
    ",
  0x40020630u64 => "
      IIRFA.iirstga2()[17],
    ",
  0x40020650u64 => "
      IIRFA.iirstga2()[18],
    ",
  0x40020670u64 => "
      IIRFA.iirstga2()[19],
    ",
  0x40020690u64 => "
      IIRFA.iirstga2()[20],
    ",
  0x400206b0u64 => "
      IIRFA.iirstga2()[21],
    ",
  0x400206d0u64 => "
      IIRFA.iirstga2()[22],
    ",
  0x400206f0u64 => "
      IIRFA.iirstga2()[23],
    ",
  0x40020710u64 => "
      IIRFA.iirstga2()[24],
    ",
  0x40020730u64 => "
      IIRFA.iirstga2()[25],
    ",
  0x40020750u64 => "
      IIRFA.iirstga2()[26],
    ",
  0x40020770u64 => "
      IIRFA.iirstga2()[27],
    ",
  0x40020790u64 => "
      IIRFA.iirstga2()[28],
    ",
  0x400207b0u64 => "
      IIRFA.iirstga2()[29],
    ",
  0x400207d0u64 => "
      IIRFA.iirstga2()[30],
    ",
  0x400207f0u64 => "
      IIRFA.iirstga2()[31],
    ",
  0x40020414u64 => "
      IIRFA.iirstgd0()[0],
    ",
  0x40020434u64 => "
      IIRFA.iirstgd0()[1],
    ",
  0x40020454u64 => "
      IIRFA.iirstgd0()[2],
    ",
  0x40020474u64 => "
      IIRFA.iirstgd0()[3],
    ",
  0x40020494u64 => "
      IIRFA.iirstgd0()[4],
    ",
  0x400204b4u64 => "
      IIRFA.iirstgd0()[5],
    ",
  0x400204d4u64 => "
      IIRFA.iirstgd0()[6],
    ",
  0x400204f4u64 => "
      IIRFA.iirstgd0()[7],
    ",
  0x40020514u64 => "
      IIRFA.iirstgd0()[8],
    ",
  0x40020534u64 => "
      IIRFA.iirstgd0()[9],
    ",
  0x40020554u64 => "
      IIRFA.iirstgd0()[10],
    ",
  0x40020574u64 => "
      IIRFA.iirstgd0()[11],
    ",
  0x40020594u64 => "
      IIRFA.iirstgd0()[12],
    ",
  0x400205b4u64 => "
      IIRFA.iirstgd0()[13],
    ",
  0x400205d4u64 => "
      IIRFA.iirstgd0()[14],
    ",
  0x400205f4u64 => "
      IIRFA.iirstgd0()[15],
    ",
  0x40020614u64 => "
      IIRFA.iirstgd0()[16],
    ",
  0x40020634u64 => "
      IIRFA.iirstgd0()[17],
    ",
  0x40020654u64 => "
      IIRFA.iirstgd0()[18],
    ",
  0x40020674u64 => "
      IIRFA.iirstgd0()[19],
    ",
  0x40020694u64 => "
      IIRFA.iirstgd0()[20],
    ",
  0x400206b4u64 => "
      IIRFA.iirstgd0()[21],
    ",
  0x400206d4u64 => "
      IIRFA.iirstgd0()[22],
    ",
  0x400206f4u64 => "
      IIRFA.iirstgd0()[23],
    ",
  0x40020714u64 => "
      IIRFA.iirstgd0()[24],
    ",
  0x40020734u64 => "
      IIRFA.iirstgd0()[25],
    ",
  0x40020754u64 => "
      IIRFA.iirstgd0()[26],
    ",
  0x40020774u64 => "
      IIRFA.iirstgd0()[27],
    ",
  0x40020794u64 => "
      IIRFA.iirstgd0()[28],
    ",
  0x400207b4u64 => "
      IIRFA.iirstgd0()[29],
    ",
  0x400207d4u64 => "
      IIRFA.iirstgd0()[30],
    ",
  0x400207f4u64 => "
      IIRFA.iirstgd0()[31],
    ",
  0x40020418u64 => "
      IIRFA.iirstgd1()[0],
    ",
  0x40020438u64 => "
      IIRFA.iirstgd1()[1],
    ",
  0x40020458u64 => "
      IIRFA.iirstgd1()[2],
    ",
  0x40020478u64 => "
      IIRFA.iirstgd1()[3],
    ",
  0x40020498u64 => "
      IIRFA.iirstgd1()[4],
    ",
  0x400204b8u64 => "
      IIRFA.iirstgd1()[5],
    ",
  0x400204d8u64 => "
      IIRFA.iirstgd1()[6],
    ",
  0x400204f8u64 => "
      IIRFA.iirstgd1()[7],
    ",
  0x40020518u64 => "
      IIRFA.iirstgd1()[8],
    ",
  0x40020538u64 => "
      IIRFA.iirstgd1()[9],
    ",
  0x40020558u64 => "
      IIRFA.iirstgd1()[10],
    ",
  0x40020578u64 => "
      IIRFA.iirstgd1()[11],
    ",
  0x40020598u64 => "
      IIRFA.iirstgd1()[12],
    ",
  0x400205b8u64 => "
      IIRFA.iirstgd1()[13],
    ",
  0x400205d8u64 => "
      IIRFA.iirstgd1()[14],
    ",
  0x400205f8u64 => "
      IIRFA.iirstgd1()[15],
    ",
  0x40020618u64 => "
      IIRFA.iirstgd1()[16],
    ",
  0x40020638u64 => "
      IIRFA.iirstgd1()[17],
    ",
  0x40020658u64 => "
      IIRFA.iirstgd1()[18],
    ",
  0x40020678u64 => "
      IIRFA.iirstgd1()[19],
    ",
  0x40020698u64 => "
      IIRFA.iirstgd1()[20],
    ",
  0x400206b8u64 => "
      IIRFA.iirstgd1()[21],
    ",
  0x400206d8u64 => "
      IIRFA.iirstgd1()[22],
    ",
  0x400206f8u64 => "
      IIRFA.iirstgd1()[23],
    ",
  0x40020718u64 => "
      IIRFA.iirstgd1()[24],
    ",
  0x40020738u64 => "
      IIRFA.iirstgd1()[25],
    ",
  0x40020758u64 => "
      IIRFA.iirstgd1()[26],
    ",
  0x40020778u64 => "
      IIRFA.iirstgd1()[27],
    ",
  0x40020798u64 => "
      IIRFA.iirstgd1()[28],
    ",
  0x400207b8u64 => "
      IIRFA.iirstgd1()[29],
    ",
  0x400207d8u64 => "
      IIRFA.iirstgd1()[30],
    ",
  0x400207f8u64 => "
      IIRFA.iirstgd1()[31],
    ",
  0x40021008u64 => "
      TFU.trgsts(),
    ",
  0x40021010u64 => "
      TFU.scdt0(),
    ",
  0x40021014u64 => "
      TFU.scdt1(),
    ",
  0x40021018u64 => "
      TFU.atdt0(),
    ",
  0x4002101cu64 => "
      TFU.atdt1(),
    ",
  0x40082000u64 => "
      ELC_B.elcr(),
    ",
  0x40082004u64 => "
      ELC_B.elsegr()[0],
    ",
  0x40082008u64 => "
      ELC_B.elsegr()[1],
    ",
  0x40082038u64 => "
      ELC_B.elsr()[6],
    ",
  0x4008203cu64 => "
      ELC_B.elsr()[7],
    ",
  0x40082074u64 => "
      ELC_B.elsr()[2],
    ",
  0x40082078u64 => "
      ELC_B.elsr()[3],
    ",
  0x4008207cu64 => "
      ELC_B.elsr()[4],
    ",
  0x40082080u64 => "
      ELC_B.elsr()[5],
    ",
  0x40082090u64 => "
      ELC_B.elsr()[0],
    ",
  0x40082094u64 => "
      ELC_B.elsr()[1],
    ",
  0x400820e0u64 => "
      ELC_B.elcsara(),
    ",
  0x400820e4u64 => "
      ELC_B.elcsarb(),
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
  0x40085000u64 => "
      KINT.krctl(),
    ",
  0x40085004u64 => "
      KINT.krf(),
    ",
  0x40085008u64 => "
      KINT.krm(),
    ",
  0x4008a000u64 => "
      POEG.poegga(),
    ",
  0x4008a040u64 => "
      POEG.gtoncwpa(),
    ",
  0x4008a044u64 => "
      POEG.gtonccra(),
    ",
  0x4008a100u64 => "
      POEG.poeggb(),
    ",
  0x4008a140u64 => "
      POEG.gtoncwpb(),
    ",
  0x4008a144u64 => "
      POEG.gtonccrb(),
    ",
  0x4008a200u64 => "
      POEG.poeggc(),
    ",
  0x4008a240u64 => "
      POEG.gtoncwpc(),
    ",
  0x4008a244u64 => "
      POEG.gtonccrc(),
    ",
  0x4008a300u64 => "
      POEG.poeggd(),
    ",
  0x4008a340u64 => "
      POEG.gtoncwpd(),
    ",
  0x4008a344u64 => "
      POEG.gtonccrd(),
    ",
  0x400b0000u64 => "
      CANFD.cfdc0ncfg(),
    ",
  0x400b0004u64 => "
      CANFD.cfdc0ctr(),
    ",
  0x400b0008u64 => "
      CANFD.cfdc0sts(),
    ",
  0x400b000cu64 => "
      CANFD.cfdc0erfl(),
    ",
  0x400b0014u64 => "
      CANFD.cfdgcfg(),
    ",
  0x400b0018u64 => "
      CANFD.cfdgctr(),
    ",
  0x400b001cu64 => "
      CANFD.cfdgsts(),
    ",
  0x400b0020u64 => "
      CANFD.cfdgerfl(),
    ",
  0x400b0024u64 => "
      CANFD.cfdgtsc(),
    ",
  0x400b0028u64 => "
      CANFD.cfdgaflectr(),
    ",
  0x400b002cu64 => "
      CANFD.cfdgaflcfg(),
    ",
  0x400b0030u64 => "
      CANFD.cfdrmnb(),
    ",
  0x400b0034u64 => "
      CANFD.cfdrmnd(),
    ",
  0x400b0038u64 => "
      CANFD.cfdrmiec(),
    ",
  0x400b003cu64 => "
      CANFD.cfdrfcc()[0],
    ",
  0x400b0040u64 => "
      CANFD.cfdrfcc()[1],
    ",
  0x400b0044u64 => "
      CANFD.cfdrfsts()[0],
    ",
  0x400b0048u64 => "
      CANFD.cfdrfsts()[1],
    ",
  0x400b004cu64 => "
      CANFD.cfdrfpctr()[0],
    ",
  0x400b0050u64 => "
      CANFD.cfdrfpctr()[1],
    ",
  0x400b0054u64 => "
      CANFD.cfdcfcc(),
    ",
  0x400b0058u64 => "
      CANFD.cfdcfsts(),
    ",
  0x400b005cu64 => "
      CANFD.cfdcfpctr(),
    ",
  0x400b0060u64 => "
      CANFD.cfdfests(),
    ",
  0x400b0064u64 => "
      CANFD.cfdffsts(),
    ",
  0x400b0068u64 => "
      CANFD.cfdfmsts(),
    ",
  0x400b006cu64 => "
      CANFD.cfdrfists(),
    ",
  0x400b0070u64 => "
      CANFD.cfdtmc()[0],
    ",
  0x400b0071u64 => "
      CANFD.cfdtmc()[1],
    ",
  0x400b0072u64 => "
      CANFD.cfdtmc()[2],
    ",
  0x400b0073u64 => "
      CANFD.cfdtmc()[3],
    ",
  0x400b0074u64 => "
      CANFD.cfdtmsts()[0],
    ",
  0x400b0075u64 => "
      CANFD.cfdtmsts()[1],
    ",
  0x400b0076u64 => "
      CANFD.cfdtmsts()[2],
    ",
  0x400b0077u64 => "
      CANFD.cfdtmsts()[3],
    ",
  0x400b0078u64 => "
      CANFD.cfdtmtrsts(),
    ",
  0x400b007cu64 => "
      CANFD.cfdtmtarsts(),
    ",
  0x400b0080u64 => "
      CANFD.cfdtmtcsts(),
    ",
  0x400b0084u64 => "
      CANFD.cfdtmtasts(),
    ",
  0x400b0088u64 => "
      CANFD.cfdtmiec(),
    ",
  0x400b008cu64 => "
      CANFD.cfdtxqcc(),
    ",
  0x400b0090u64 => "
      CANFD.cfdtxqsts(),
    ",
  0x400b0094u64 => "
      CANFD.cfdtxqpctr(),
    ",
  0x400b0098u64 => "
      CANFD.cfdthlcc(),
    ",
  0x400b009cu64 => "
      CANFD.cfdthlsts(),
    ",
  0x400b00a0u64 => "
      CANFD.cfdthlpctr(),
    ",
  0x400b00a4u64 => "
      CANFD.cfdgtintsts(),
    ",
  0x400b00a8u64 => "
      CANFD.cfdgtstcfg(),
    ",
  0x400b00acu64 => "
      CANFD.cfdgtstctr(),
    ",
  0x400b00b0u64 => "
      CANFD.cfdgfdcfg(),
    ",
  0x400b00b8u64 => "
      CANFD.cfdglockk(),
    ",
  0x400b00c0u64 => "
      CANFD.cfdgaflignent(),
    ",
  0x400b00c4u64 => "
      CANFD.cfdgaflignctr(),
    ",
  0x400b00c8u64 => "
      CANFD.cfdcdtct(),
    ",
  0x400b00ccu64 => "
      CANFD.cfdcdtsts(),
    ",
  0x400b00d8u64 => "
      CANFD.cfdgrstc(),
    ",
  0x400b0100u64 => "
      CANFD.cfdc0dcfg(),
    ",
  0x400b0104u64 => "
      CANFD.cfdc0fdcfg(),
    ",
  0x400b0108u64 => "
      CANFD.cfdc0fdctr(),
    ",
  0x400b010cu64 => "
      CANFD.cfdc0fdsts(),
    ",
  0x400b0110u64 => "
      CANFD.cfdc0fdcrc(),
    ",
  0x400b0120u64 => "
      CANFD.cfdgaflid()[0],
    ",
  0x400b0130u64 => "
      CANFD.cfdgaflid()[1],
    ",
  0x400b0140u64 => "
      CANFD.cfdgaflid()[2],
    ",
  0x400b0150u64 => "
      CANFD.cfdgaflid()[3],
    ",
  0x400b0160u64 => "
      CANFD.cfdgaflid()[4],
    ",
  0x400b0170u64 => "
      CANFD.cfdgaflid()[5],
    ",
  0x400b0180u64 => "
      CANFD.cfdgaflid()[6],
    ",
  0x400b0190u64 => "
      CANFD.cfdgaflid()[7],
    ",
  0x400b01a0u64 => "
      CANFD.cfdgaflid()[8],
    ",
  0x400b01b0u64 => "
      CANFD.cfdgaflid()[9],
    ",
  0x400b01c0u64 => "
      CANFD.cfdgaflid()[10],
    ",
  0x400b01d0u64 => "
      CANFD.cfdgaflid()[11],
    ",
  0x400b01e0u64 => "
      CANFD.cfdgaflid()[12],
    ",
  0x400b01f0u64 => "
      CANFD.cfdgaflid()[13],
    ",
  0x400b0200u64 => "
      CANFD.cfdgaflid()[14],
    ",
  0x400b0210u64 => "
      CANFD.cfdgaflid()[15],
    ",
  0x400b0124u64 => "
      CANFD.cfdgaflm()[0],
    ",
  0x400b0134u64 => "
      CANFD.cfdgaflm()[1],
    ",
  0x400b0144u64 => "
      CANFD.cfdgaflm()[2],
    ",
  0x400b0154u64 => "
      CANFD.cfdgaflm()[3],
    ",
  0x400b0164u64 => "
      CANFD.cfdgaflm()[4],
    ",
  0x400b0174u64 => "
      CANFD.cfdgaflm()[5],
    ",
  0x400b0184u64 => "
      CANFD.cfdgaflm()[6],
    ",
  0x400b0194u64 => "
      CANFD.cfdgaflm()[7],
    ",
  0x400b01a4u64 => "
      CANFD.cfdgaflm()[8],
    ",
  0x400b01b4u64 => "
      CANFD.cfdgaflm()[9],
    ",
  0x400b01c4u64 => "
      CANFD.cfdgaflm()[10],
    ",
  0x400b01d4u64 => "
      CANFD.cfdgaflm()[11],
    ",
  0x400b01e4u64 => "
      CANFD.cfdgaflm()[12],
    ",
  0x400b01f4u64 => "
      CANFD.cfdgaflm()[13],
    ",
  0x400b0204u64 => "
      CANFD.cfdgaflm()[14],
    ",
  0x400b0214u64 => "
      CANFD.cfdgaflm()[15],
    ",
  0x400b0128u64 => "
      CANFD.cfdgaflp0()[0],
    ",
  0x400b0138u64 => "
      CANFD.cfdgaflp0()[1],
    ",
  0x400b0148u64 => "
      CANFD.cfdgaflp0()[2],
    ",
  0x400b0158u64 => "
      CANFD.cfdgaflp0()[3],
    ",
  0x400b0168u64 => "
      CANFD.cfdgaflp0()[4],
    ",
  0x400b0178u64 => "
      CANFD.cfdgaflp0()[5],
    ",
  0x400b0188u64 => "
      CANFD.cfdgaflp0()[6],
    ",
  0x400b0198u64 => "
      CANFD.cfdgaflp0()[7],
    ",
  0x400b01a8u64 => "
      CANFD.cfdgaflp0()[8],
    ",
  0x400b01b8u64 => "
      CANFD.cfdgaflp0()[9],
    ",
  0x400b01c8u64 => "
      CANFD.cfdgaflp0()[10],
    ",
  0x400b01d8u64 => "
      CANFD.cfdgaflp0()[11],
    ",
  0x400b01e8u64 => "
      CANFD.cfdgaflp0()[12],
    ",
  0x400b01f8u64 => "
      CANFD.cfdgaflp0()[13],
    ",
  0x400b0208u64 => "
      CANFD.cfdgaflp0()[14],
    ",
  0x400b0218u64 => "
      CANFD.cfdgaflp0()[15],
    ",
  0x400b012cu64 => "
      CANFD.cfdgaflp1()[0],
    ",
  0x400b013cu64 => "
      CANFD.cfdgaflp1()[1],
    ",
  0x400b014cu64 => "
      CANFD.cfdgaflp1()[2],
    ",
  0x400b015cu64 => "
      CANFD.cfdgaflp1()[3],
    ",
  0x400b016cu64 => "
      CANFD.cfdgaflp1()[4],
    ",
  0x400b017cu64 => "
      CANFD.cfdgaflp1()[5],
    ",
  0x400b018cu64 => "
      CANFD.cfdgaflp1()[6],
    ",
  0x400b019cu64 => "
      CANFD.cfdgaflp1()[7],
    ",
  0x400b01acu64 => "
      CANFD.cfdgaflp1()[8],
    ",
  0x400b01bcu64 => "
      CANFD.cfdgaflp1()[9],
    ",
  0x400b01ccu64 => "
      CANFD.cfdgaflp1()[10],
    ",
  0x400b01dcu64 => "
      CANFD.cfdgaflp1()[11],
    ",
  0x400b01ecu64 => "
      CANFD.cfdgaflp1()[12],
    ",
  0x400b01fcu64 => "
      CANFD.cfdgaflp1()[13],
    ",
  0x400b020cu64 => "
      CANFD.cfdgaflp1()[14],
    ",
  0x400b021cu64 => "
      CANFD.cfdgaflp1()[15],
    ",
  0x400b0280u64 => "
      CANFD.cfdrpgacc()[0],
    ",
  0x400b0284u64 => "
      CANFD.cfdrpgacc()[1],
    ",
  0x400b0288u64 => "
      CANFD.cfdrpgacc()[2],
    ",
  0x400b028cu64 => "
      CANFD.cfdrpgacc()[3],
    ",
  0x400b0290u64 => "
      CANFD.cfdrpgacc()[4],
    ",
  0x400b0294u64 => "
      CANFD.cfdrpgacc()[5],
    ",
  0x400b0298u64 => "
      CANFD.cfdrpgacc()[6],
    ",
  0x400b029cu64 => "
      CANFD.cfdrpgacc()[7],
    ",
  0x400b02a0u64 => "
      CANFD.cfdrpgacc()[8],
    ",
  0x400b02a4u64 => "
      CANFD.cfdrpgacc()[9],
    ",
  0x400b02a8u64 => "
      CANFD.cfdrpgacc()[10],
    ",
  0x400b02acu64 => "
      CANFD.cfdrpgacc()[11],
    ",
  0x400b02b0u64 => "
      CANFD.cfdrpgacc()[12],
    ",
  0x400b02b4u64 => "
      CANFD.cfdrpgacc()[13],
    ",
  0x400b02b8u64 => "
      CANFD.cfdrpgacc()[14],
    ",
  0x400b02bcu64 => "
      CANFD.cfdrpgacc()[15],
    ",
  0x400b02c0u64 => "
      CANFD.cfdrpgacc()[16],
    ",
  0x400b02c4u64 => "
      CANFD.cfdrpgacc()[17],
    ",
  0x400b02c8u64 => "
      CANFD.cfdrpgacc()[18],
    ",
  0x400b02ccu64 => "
      CANFD.cfdrpgacc()[19],
    ",
  0x400b02d0u64 => "
      CANFD.cfdrpgacc()[20],
    ",
  0x400b02d4u64 => "
      CANFD.cfdrpgacc()[21],
    ",
  0x400b02d8u64 => "
      CANFD.cfdrpgacc()[22],
    ",
  0x400b02dcu64 => "
      CANFD.cfdrpgacc()[23],
    ",
  0x400b02e0u64 => "
      CANFD.cfdrpgacc()[24],
    ",
  0x400b02e4u64 => "
      CANFD.cfdrpgacc()[25],
    ",
  0x400b02e8u64 => "
      CANFD.cfdrpgacc()[26],
    ",
  0x400b02ecu64 => "
      CANFD.cfdrpgacc()[27],
    ",
  0x400b02f0u64 => "
      CANFD.cfdrpgacc()[28],
    ",
  0x400b02f4u64 => "
      CANFD.cfdrpgacc()[29],
    ",
  0x400b02f8u64 => "
      CANFD.cfdrpgacc()[30],
    ",
  0x400b02fcu64 => "
      CANFD.cfdrpgacc()[31],
    ",
  0x400b0300u64 => "
      CANFD.cfdrpgacc()[32],
    ",
  0x400b0304u64 => "
      CANFD.cfdrpgacc()[33],
    ",
  0x400b0308u64 => "
      CANFD.cfdrpgacc()[34],
    ",
  0x400b030cu64 => "
      CANFD.cfdrpgacc()[35],
    ",
  0x400b0310u64 => "
      CANFD.cfdrpgacc()[36],
    ",
  0x400b0314u64 => "
      CANFD.cfdrpgacc()[37],
    ",
  0x400b0318u64 => "
      CANFD.cfdrpgacc()[38],
    ",
  0x400b031cu64 => "
      CANFD.cfdrpgacc()[39],
    ",
  0x400b0320u64 => "
      CANFD.cfdrpgacc()[40],
    ",
  0x400b0324u64 => "
      CANFD.cfdrpgacc()[41],
    ",
  0x400b0328u64 => "
      CANFD.cfdrpgacc()[42],
    ",
  0x400b032cu64 => "
      CANFD.cfdrpgacc()[43],
    ",
  0x400b0330u64 => "
      CANFD.cfdrpgacc()[44],
    ",
  0x400b0334u64 => "
      CANFD.cfdrpgacc()[45],
    ",
  0x400b0338u64 => "
      CANFD.cfdrpgacc()[46],
    ",
  0x400b033cu64 => "
      CANFD.cfdrpgacc()[47],
    ",
  0x400b0340u64 => "
      CANFD.cfdrpgacc()[48],
    ",
  0x400b0344u64 => "
      CANFD.cfdrpgacc()[49],
    ",
  0x400b0348u64 => "
      CANFD.cfdrpgacc()[50],
    ",
  0x400b034cu64 => "
      CANFD.cfdrpgacc()[51],
    ",
  0x400b0350u64 => "
      CANFD.cfdrpgacc()[52],
    ",
  0x400b0354u64 => "
      CANFD.cfdrpgacc()[53],
    ",
  0x400b0358u64 => "
      CANFD.cfdrpgacc()[54],
    ",
  0x400b035cu64 => "
      CANFD.cfdrpgacc()[55],
    ",
  0x400b0360u64 => "
      CANFD.cfdrpgacc()[56],
    ",
  0x400b0364u64 => "
      CANFD.cfdrpgacc()[57],
    ",
  0x400b0368u64 => "
      CANFD.cfdrpgacc()[58],
    ",
  0x400b036cu64 => "
      CANFD.cfdrpgacc()[59],
    ",
  0x400b0370u64 => "
      CANFD.cfdrpgacc()[60],
    ",
  0x400b0374u64 => "
      CANFD.cfdrpgacc()[61],
    ",
  0x400b0378u64 => "
      CANFD.cfdrpgacc()[62],
    ",
  0x400b037cu64 => "
      CANFD.cfdrpgacc()[63],
    ",
  0x400b0520u64 => "
      CANFD.cfdrfid()[0],
    ",
  0x400b056cu64 => "
      CANFD.cfdrfid()[1],
    ",
  0x400b0524u64 => "
      CANFD.cfdrfptr()[0],
    ",
  0x400b0570u64 => "
      CANFD.cfdrfptr()[1],
    ",
  0x400b0528u64 => "
      CANFD.cfdrffdsts()[0],
    ",
  0x400b0574u64 => "
      CANFD.cfdrffdsts()[1],
    ",
  0x400b052cu64 => "
      CANFD.cfdrfdf_0()[0],
    ",
  0x400b0578u64 => "
      CANFD.cfdrfdf_0()[1],
    ",
  0x400b0530u64 => "
      CANFD.cfdrfdf_1()[0],
    ",
  0x400b057cu64 => "
      CANFD.cfdrfdf_1()[1],
    ",
  0x400b0534u64 => "
      CANFD.cfdrfdf_2()[0],
    ",
  0x400b0580u64 => "
      CANFD.cfdrfdf_2()[1],
    ",
  0x400b0538u64 => "
      CANFD.cfdrfdf_3()[0],
    ",
  0x400b0584u64 => "
      CANFD.cfdrfdf_3()[1],
    ",
  0x400b053cu64 => "
      CANFD.cfdrfdf_4()[0],
    ",
  0x400b0588u64 => "
      CANFD.cfdrfdf_4()[1],
    ",
  0x400b0540u64 => "
      CANFD.cfdrfdf_5()[0],
    ",
  0x400b058cu64 => "
      CANFD.cfdrfdf_5()[1],
    ",
  0x400b0544u64 => "
      CANFD.cfdrfdf_6()[0],
    ",
  0x400b0590u64 => "
      CANFD.cfdrfdf_6()[1],
    ",
  0x400b0548u64 => "
      CANFD.cfdrfdf_7()[0],
    ",
  0x400b0594u64 => "
      CANFD.cfdrfdf_7()[1],
    ",
  0x400b054cu64 => "
      CANFD.cfdrfdf_8()[0],
    ",
  0x400b0598u64 => "
      CANFD.cfdrfdf_8()[1],
    ",
  0x400b0550u64 => "
      CANFD.cfdrfdf_9()[0],
    ",
  0x400b059cu64 => "
      CANFD.cfdrfdf_9()[1],
    ",
  0x400b0554u64 => "
      CANFD.cfdrfdf_10()[0],
    ",
  0x400b05a0u64 => "
      CANFD.cfdrfdf_10()[1],
    ",
  0x400b0558u64 => "
      CANFD.cfdrfdf_11()[0],
    ",
  0x400b05a4u64 => "
      CANFD.cfdrfdf_11()[1],
    ",
  0x400b055cu64 => "
      CANFD.cfdrfdf_12()[0],
    ",
  0x400b05a8u64 => "
      CANFD.cfdrfdf_12()[1],
    ",
  0x400b0560u64 => "
      CANFD.cfdrfdf_13()[0],
    ",
  0x400b05acu64 => "
      CANFD.cfdrfdf_13()[1],
    ",
  0x400b0564u64 => "
      CANFD.cfdrfdf_14()[0],
    ",
  0x400b05b0u64 => "
      CANFD.cfdrfdf_14()[1],
    ",
  0x400b0568u64 => "
      CANFD.cfdrfdf_15()[0],
    ",
  0x400b05b4u64 => "
      CANFD.cfdrfdf_15()[1],
    ",
  0x400b05b8u64 => "
      CANFD.cfdcfid(),
    ",
  0x400b05bcu64 => "
      CANFD.cfdcfptr(),
    ",
  0x400b05c0u64 => "
      CANFD.cfdcffdcsts(),
    ",
  0x400b05c4u64 => "
      CANFD.cfdcfdf()[0],
    ",
  0x400b05c8u64 => "
      CANFD.cfdcfdf()[1],
    ",
  0x400b05ccu64 => "
      CANFD.cfdcfdf()[2],
    ",
  0x400b05d0u64 => "
      CANFD.cfdcfdf()[3],
    ",
  0x400b05d4u64 => "
      CANFD.cfdcfdf()[4],
    ",
  0x400b05d8u64 => "
      CANFD.cfdcfdf()[5],
    ",
  0x400b05dcu64 => "
      CANFD.cfdcfdf()[6],
    ",
  0x400b05e0u64 => "
      CANFD.cfdcfdf()[7],
    ",
  0x400b05e4u64 => "
      CANFD.cfdcfdf()[8],
    ",
  0x400b05e8u64 => "
      CANFD.cfdcfdf()[9],
    ",
  0x400b05ecu64 => "
      CANFD.cfdcfdf()[10],
    ",
  0x400b05f0u64 => "
      CANFD.cfdcfdf()[11],
    ",
  0x400b05f4u64 => "
      CANFD.cfdcfdf()[12],
    ",
  0x400b05f8u64 => "
      CANFD.cfdcfdf()[13],
    ",
  0x400b05fcu64 => "
      CANFD.cfdcfdf()[14],
    ",
  0x400b0600u64 => "
      CANFD.cfdcfdf()[15],
    ",
  0x400b0604u64 => "
      CANFD.cfdtmid()[0],
    ",
  0x400b0650u64 => "
      CANFD.cfdtmid()[1],
    ",
  0x400b069cu64 => "
      CANFD.cfdtmid()[2],
    ",
  0x400b06e8u64 => "
      CANFD.cfdtmid()[3],
    ",
  0x400b0608u64 => "
      CANFD.cfdtmptr()[0],
    ",
  0x400b0654u64 => "
      CANFD.cfdtmptr()[1],
    ",
  0x400b06a0u64 => "
      CANFD.cfdtmptr()[2],
    ",
  0x400b06ecu64 => "
      CANFD.cfdtmptr()[3],
    ",
  0x400b060cu64 => "
      CANFD.cfdtmfdctr()[0],
    ",
  0x400b0658u64 => "
      CANFD.cfdtmfdctr()[1],
    ",
  0x400b06a4u64 => "
      CANFD.cfdtmfdctr()[2],
    ",
  0x400b06f0u64 => "
      CANFD.cfdtmfdctr()[3],
    ",
  0x400b0610u64 => "
      CANFD.cfdtmdf_0()[0],
    ",
  0x400b065cu64 => "
      CANFD.cfdtmdf_0()[1],
    ",
  0x400b06a8u64 => "
      CANFD.cfdtmdf_0()[2],
    ",
  0x400b06f4u64 => "
      CANFD.cfdtmdf_0()[3],
    ",
  0x400b0614u64 => "
      CANFD.cfdtmdf_1()[0],
    ",
  0x400b0660u64 => "
      CANFD.cfdtmdf_1()[1],
    ",
  0x400b06acu64 => "
      CANFD.cfdtmdf_1()[2],
    ",
  0x400b06f8u64 => "
      CANFD.cfdtmdf_1()[3],
    ",
  0x400b0618u64 => "
      CANFD.cfdtmdf_2()[0],
    ",
  0x400b0664u64 => "
      CANFD.cfdtmdf_2()[1],
    ",
  0x400b06b0u64 => "
      CANFD.cfdtmdf_2()[2],
    ",
  0x400b06fcu64 => "
      CANFD.cfdtmdf_2()[3],
    ",
  0x400b061cu64 => "
      CANFD.cfdtmdf_3()[0],
    ",
  0x400b0668u64 => "
      CANFD.cfdtmdf_3()[1],
    ",
  0x400b06b4u64 => "
      CANFD.cfdtmdf_3()[2],
    ",
  0x400b0700u64 => "
      CANFD.cfdtmdf_3()[3],
    ",
  0x400b0620u64 => "
      CANFD.cfdtmdf_4()[0],
    ",
  0x400b066cu64 => "
      CANFD.cfdtmdf_4()[1],
    ",
  0x400b06b8u64 => "
      CANFD.cfdtmdf_4()[2],
    ",
  0x400b0704u64 => "
      CANFD.cfdtmdf_4()[3],
    ",
  0x400b0624u64 => "
      CANFD.cfdtmdf_5()[0],
    ",
  0x400b0670u64 => "
      CANFD.cfdtmdf_5()[1],
    ",
  0x400b06bcu64 => "
      CANFD.cfdtmdf_5()[2],
    ",
  0x400b0708u64 => "
      CANFD.cfdtmdf_5()[3],
    ",
  0x400b0628u64 => "
      CANFD.cfdtmdf_6()[0],
    ",
  0x400b0674u64 => "
      CANFD.cfdtmdf_6()[1],
    ",
  0x400b06c0u64 => "
      CANFD.cfdtmdf_6()[2],
    ",
  0x400b070cu64 => "
      CANFD.cfdtmdf_6()[3],
    ",
  0x400b062cu64 => "
      CANFD.cfdtmdf_7()[0],
    ",
  0x400b0678u64 => "
      CANFD.cfdtmdf_7()[1],
    ",
  0x400b06c4u64 => "
      CANFD.cfdtmdf_7()[2],
    ",
  0x400b0710u64 => "
      CANFD.cfdtmdf_7()[3],
    ",
  0x400b0630u64 => "
      CANFD.cfdtmdf_8()[0],
    ",
  0x400b067cu64 => "
      CANFD.cfdtmdf_8()[1],
    ",
  0x400b06c8u64 => "
      CANFD.cfdtmdf_8()[2],
    ",
  0x400b0714u64 => "
      CANFD.cfdtmdf_8()[3],
    ",
  0x400b0634u64 => "
      CANFD.cfdtmdf_9()[0],
    ",
  0x400b0680u64 => "
      CANFD.cfdtmdf_9()[1],
    ",
  0x400b06ccu64 => "
      CANFD.cfdtmdf_9()[2],
    ",
  0x400b0718u64 => "
      CANFD.cfdtmdf_9()[3],
    ",
  0x400b0638u64 => "
      CANFD.cfdtmdf_10()[0],
    ",
  0x400b0684u64 => "
      CANFD.cfdtmdf_10()[1],
    ",
  0x400b06d0u64 => "
      CANFD.cfdtmdf_10()[2],
    ",
  0x400b071cu64 => "
      CANFD.cfdtmdf_10()[3],
    ",
  0x400b063cu64 => "
      CANFD.cfdtmdf_11()[0],
    ",
  0x400b0688u64 => "
      CANFD.cfdtmdf_11()[1],
    ",
  0x400b06d4u64 => "
      CANFD.cfdtmdf_11()[2],
    ",
  0x400b0720u64 => "
      CANFD.cfdtmdf_11()[3],
    ",
  0x400b0640u64 => "
      CANFD.cfdtmdf_12()[0],
    ",
  0x400b068cu64 => "
      CANFD.cfdtmdf_12()[1],
    ",
  0x400b06d8u64 => "
      CANFD.cfdtmdf_12()[2],
    ",
  0x400b0724u64 => "
      CANFD.cfdtmdf_12()[3],
    ",
  0x400b0644u64 => "
      CANFD.cfdtmdf_13()[0],
    ",
  0x400b0690u64 => "
      CANFD.cfdtmdf_13()[1],
    ",
  0x400b06dcu64 => "
      CANFD.cfdtmdf_13()[2],
    ",
  0x400b0728u64 => "
      CANFD.cfdtmdf_13()[3],
    ",
  0x400b0648u64 => "
      CANFD.cfdtmdf_14()[0],
    ",
  0x400b0694u64 => "
      CANFD.cfdtmdf_14()[1],
    ",
  0x400b06e0u64 => "
      CANFD.cfdtmdf_14()[2],
    ",
  0x400b072cu64 => "
      CANFD.cfdtmdf_14()[3],
    ",
  0x400b064cu64 => "
      CANFD.cfdtmdf_15()[0],
    ",
  0x400b0698u64 => "
      CANFD.cfdtmdf_15()[1],
    ",
  0x400b06e4u64 => "
      CANFD.cfdtmdf_15()[2],
    ",
  0x400b0730u64 => "
      CANFD.cfdtmdf_15()[3],
    ",
  0x400b0740u64 => "
      CANFD.cfdthlacc0(),
    ",
  0x400b0744u64 => "
      CANFD.cfdthlacc1(),
    ",
  0x400b1520u64 => "
      CANFD.cfdrmid()[0],
    ",
  0x400b156cu64 => "
      CANFD.cfdrmid()[1],
    ",
  0x400b15b8u64 => "
      CANFD.cfdrmid()[2],
    ",
  0x400b1604u64 => "
      CANFD.cfdrmid()[3],
    ",
  0x400b1650u64 => "
      CANFD.cfdrmid()[4],
    ",
  0x400b169cu64 => "
      CANFD.cfdrmid()[5],
    ",
  0x400b16e8u64 => "
      CANFD.cfdrmid()[6],
    ",
  0x400b1734u64 => "
      CANFD.cfdrmid()[7],
    ",
  0x400b1524u64 => "
      CANFD.cfdrmptr()[0],
    ",
  0x400b1570u64 => "
      CANFD.cfdrmptr()[1],
    ",
  0x400b15bcu64 => "
      CANFD.cfdrmptr()[2],
    ",
  0x400b1608u64 => "
      CANFD.cfdrmptr()[3],
    ",
  0x400b1654u64 => "
      CANFD.cfdrmptr()[4],
    ",
  0x400b16a0u64 => "
      CANFD.cfdrmptr()[5],
    ",
  0x400b16ecu64 => "
      CANFD.cfdrmptr()[6],
    ",
  0x400b1738u64 => "
      CANFD.cfdrmptr()[7],
    ",
  0x400b1528u64 => "
      CANFD.cfdrmfdsts()[0],
    ",
  0x400b1574u64 => "
      CANFD.cfdrmfdsts()[1],
    ",
  0x400b15c0u64 => "
      CANFD.cfdrmfdsts()[2],
    ",
  0x400b160cu64 => "
      CANFD.cfdrmfdsts()[3],
    ",
  0x400b1658u64 => "
      CANFD.cfdrmfdsts()[4],
    ",
  0x400b16a4u64 => "
      CANFD.cfdrmfdsts()[5],
    ",
  0x400b16f0u64 => "
      CANFD.cfdrmfdsts()[6],
    ",
  0x400b173cu64 => "
      CANFD.cfdrmfdsts()[7],
    ",
  0x400b152cu64 => "
      CANFD.cfdrmdf_0()[0],
    ",
  0x400b1578u64 => "
      CANFD.cfdrmdf_0()[1],
    ",
  0x400b15c4u64 => "
      CANFD.cfdrmdf_0()[2],
    ",
  0x400b1610u64 => "
      CANFD.cfdrmdf_0()[3],
    ",
  0x400b165cu64 => "
      CANFD.cfdrmdf_0()[4],
    ",
  0x400b16a8u64 => "
      CANFD.cfdrmdf_0()[5],
    ",
  0x400b16f4u64 => "
      CANFD.cfdrmdf_0()[6],
    ",
  0x400b1740u64 => "
      CANFD.cfdrmdf_0()[7],
    ",
  0x400b1530u64 => "
      CANFD.cfdrmdf_1()[0],
    ",
  0x400b157cu64 => "
      CANFD.cfdrmdf_1()[1],
    ",
  0x400b15c8u64 => "
      CANFD.cfdrmdf_1()[2],
    ",
  0x400b1614u64 => "
      CANFD.cfdrmdf_1()[3],
    ",
  0x400b1660u64 => "
      CANFD.cfdrmdf_1()[4],
    ",
  0x400b16acu64 => "
      CANFD.cfdrmdf_1()[5],
    ",
  0x400b16f8u64 => "
      CANFD.cfdrmdf_1()[6],
    ",
  0x400b1744u64 => "
      CANFD.cfdrmdf_1()[7],
    ",
  0x400b1534u64 => "
      CANFD.cfdrmdf_2()[0],
    ",
  0x400b1580u64 => "
      CANFD.cfdrmdf_2()[1],
    ",
  0x400b15ccu64 => "
      CANFD.cfdrmdf_2()[2],
    ",
  0x400b1618u64 => "
      CANFD.cfdrmdf_2()[3],
    ",
  0x400b1664u64 => "
      CANFD.cfdrmdf_2()[4],
    ",
  0x400b16b0u64 => "
      CANFD.cfdrmdf_2()[5],
    ",
  0x400b16fcu64 => "
      CANFD.cfdrmdf_2()[6],
    ",
  0x400b1748u64 => "
      CANFD.cfdrmdf_2()[7],
    ",
  0x400b1538u64 => "
      CANFD.cfdrmdf_3()[0],
    ",
  0x400b1584u64 => "
      CANFD.cfdrmdf_3()[1],
    ",
  0x400b15d0u64 => "
      CANFD.cfdrmdf_3()[2],
    ",
  0x400b161cu64 => "
      CANFD.cfdrmdf_3()[3],
    ",
  0x400b1668u64 => "
      CANFD.cfdrmdf_3()[4],
    ",
  0x400b16b4u64 => "
      CANFD.cfdrmdf_3()[5],
    ",
  0x400b1700u64 => "
      CANFD.cfdrmdf_3()[6],
    ",
  0x400b174cu64 => "
      CANFD.cfdrmdf_3()[7],
    ",
  0x400b153cu64 => "
      CANFD.cfdrmdf_4()[0],
    ",
  0x400b1588u64 => "
      CANFD.cfdrmdf_4()[1],
    ",
  0x400b15d4u64 => "
      CANFD.cfdrmdf_4()[2],
    ",
  0x400b1620u64 => "
      CANFD.cfdrmdf_4()[3],
    ",
  0x400b166cu64 => "
      CANFD.cfdrmdf_4()[4],
    ",
  0x400b16b8u64 => "
      CANFD.cfdrmdf_4()[5],
    ",
  0x400b1704u64 => "
      CANFD.cfdrmdf_4()[6],
    ",
  0x400b1750u64 => "
      CANFD.cfdrmdf_4()[7],
    ",
  0x400b1540u64 => "
      CANFD.cfdrmdf_5()[0],
    ",
  0x400b158cu64 => "
      CANFD.cfdrmdf_5()[1],
    ",
  0x400b15d8u64 => "
      CANFD.cfdrmdf_5()[2],
    ",
  0x400b1624u64 => "
      CANFD.cfdrmdf_5()[3],
    ",
  0x400b1670u64 => "
      CANFD.cfdrmdf_5()[4],
    ",
  0x400b16bcu64 => "
      CANFD.cfdrmdf_5()[5],
    ",
  0x400b1708u64 => "
      CANFD.cfdrmdf_5()[6],
    ",
  0x400b1754u64 => "
      CANFD.cfdrmdf_5()[7],
    ",
  0x400b1544u64 => "
      CANFD.cfdrmdf_6()[0],
    ",
  0x400b1590u64 => "
      CANFD.cfdrmdf_6()[1],
    ",
  0x400b15dcu64 => "
      CANFD.cfdrmdf_6()[2],
    ",
  0x400b1628u64 => "
      CANFD.cfdrmdf_6()[3],
    ",
  0x400b1674u64 => "
      CANFD.cfdrmdf_6()[4],
    ",
  0x400b16c0u64 => "
      CANFD.cfdrmdf_6()[5],
    ",
  0x400b170cu64 => "
      CANFD.cfdrmdf_6()[6],
    ",
  0x400b1758u64 => "
      CANFD.cfdrmdf_6()[7],
    ",
  0x400b1548u64 => "
      CANFD.cfdrmdf_7()[0],
    ",
  0x400b1594u64 => "
      CANFD.cfdrmdf_7()[1],
    ",
  0x400b15e0u64 => "
      CANFD.cfdrmdf_7()[2],
    ",
  0x400b162cu64 => "
      CANFD.cfdrmdf_7()[3],
    ",
  0x400b1678u64 => "
      CANFD.cfdrmdf_7()[4],
    ",
  0x400b16c4u64 => "
      CANFD.cfdrmdf_7()[5],
    ",
  0x400b1710u64 => "
      CANFD.cfdrmdf_7()[6],
    ",
  0x400b175cu64 => "
      CANFD.cfdrmdf_7()[7],
    ",
  0x400b154cu64 => "
      CANFD.cfdrmdf_8()[0],
    ",
  0x400b1598u64 => "
      CANFD.cfdrmdf_8()[1],
    ",
  0x400b15e4u64 => "
      CANFD.cfdrmdf_8()[2],
    ",
  0x400b1630u64 => "
      CANFD.cfdrmdf_8()[3],
    ",
  0x400b167cu64 => "
      CANFD.cfdrmdf_8()[4],
    ",
  0x400b16c8u64 => "
      CANFD.cfdrmdf_8()[5],
    ",
  0x400b1714u64 => "
      CANFD.cfdrmdf_8()[6],
    ",
  0x400b1760u64 => "
      CANFD.cfdrmdf_8()[7],
    ",
  0x400b1550u64 => "
      CANFD.cfdrmdf_9()[0],
    ",
  0x400b159cu64 => "
      CANFD.cfdrmdf_9()[1],
    ",
  0x400b15e8u64 => "
      CANFD.cfdrmdf_9()[2],
    ",
  0x400b1634u64 => "
      CANFD.cfdrmdf_9()[3],
    ",
  0x400b1680u64 => "
      CANFD.cfdrmdf_9()[4],
    ",
  0x400b16ccu64 => "
      CANFD.cfdrmdf_9()[5],
    ",
  0x400b1718u64 => "
      CANFD.cfdrmdf_9()[6],
    ",
  0x400b1764u64 => "
      CANFD.cfdrmdf_9()[7],
    ",
  0x400b1554u64 => "
      CANFD.cfdrmdf_10()[0],
    ",
  0x400b15a0u64 => "
      CANFD.cfdrmdf_10()[1],
    ",
  0x400b15ecu64 => "
      CANFD.cfdrmdf_10()[2],
    ",
  0x400b1638u64 => "
      CANFD.cfdrmdf_10()[3],
    ",
  0x400b1684u64 => "
      CANFD.cfdrmdf_10()[4],
    ",
  0x400b16d0u64 => "
      CANFD.cfdrmdf_10()[5],
    ",
  0x400b171cu64 => "
      CANFD.cfdrmdf_10()[6],
    ",
  0x400b1768u64 => "
      CANFD.cfdrmdf_10()[7],
    ",
  0x400b1558u64 => "
      CANFD.cfdrmdf_11()[0],
    ",
  0x400b15a4u64 => "
      CANFD.cfdrmdf_11()[1],
    ",
  0x400b15f0u64 => "
      CANFD.cfdrmdf_11()[2],
    ",
  0x400b163cu64 => "
      CANFD.cfdrmdf_11()[3],
    ",
  0x400b1688u64 => "
      CANFD.cfdrmdf_11()[4],
    ",
  0x400b16d4u64 => "
      CANFD.cfdrmdf_11()[5],
    ",
  0x400b1720u64 => "
      CANFD.cfdrmdf_11()[6],
    ",
  0x400b176cu64 => "
      CANFD.cfdrmdf_11()[7],
    ",
  0x400b155cu64 => "
      CANFD.cfdrmdf_12()[0],
    ",
  0x400b15a8u64 => "
      CANFD.cfdrmdf_12()[1],
    ",
  0x400b15f4u64 => "
      CANFD.cfdrmdf_12()[2],
    ",
  0x400b1640u64 => "
      CANFD.cfdrmdf_12()[3],
    ",
  0x400b168cu64 => "
      CANFD.cfdrmdf_12()[4],
    ",
  0x400b16d8u64 => "
      CANFD.cfdrmdf_12()[5],
    ",
  0x400b1724u64 => "
      CANFD.cfdrmdf_12()[6],
    ",
  0x400b1770u64 => "
      CANFD.cfdrmdf_12()[7],
    ",
  0x400b1560u64 => "
      CANFD.cfdrmdf_13()[0],
    ",
  0x400b15acu64 => "
      CANFD.cfdrmdf_13()[1],
    ",
  0x400b15f8u64 => "
      CANFD.cfdrmdf_13()[2],
    ",
  0x400b1644u64 => "
      CANFD.cfdrmdf_13()[3],
    ",
  0x400b1690u64 => "
      CANFD.cfdrmdf_13()[4],
    ",
  0x400b16dcu64 => "
      CANFD.cfdrmdf_13()[5],
    ",
  0x400b1728u64 => "
      CANFD.cfdrmdf_13()[6],
    ",
  0x400b1774u64 => "
      CANFD.cfdrmdf_13()[7],
    ",
  0x400b1564u64 => "
      CANFD.cfdrmdf_14()[0],
    ",
  0x400b15b0u64 => "
      CANFD.cfdrmdf_14()[1],
    ",
  0x400b15fcu64 => "
      CANFD.cfdrmdf_14()[2],
    ",
  0x400b1648u64 => "
      CANFD.cfdrmdf_14()[3],
    ",
  0x400b1694u64 => "
      CANFD.cfdrmdf_14()[4],
    ",
  0x400b16e0u64 => "
      CANFD.cfdrmdf_14()[5],
    ",
  0x400b172cu64 => "
      CANFD.cfdrmdf_14()[6],
    ",
  0x400b1778u64 => "
      CANFD.cfdrmdf_14()[7],
    ",
  0x400b1568u64 => "
      CANFD.cfdrmdf_15()[0],
    ",
  0x400b15b4u64 => "
      CANFD.cfdrmdf_15()[1],
    ",
  0x400b1600u64 => "
      CANFD.cfdrmdf_15()[2],
    ",
  0x400b164cu64 => "
      CANFD.cfdrmdf_15()[3],
    ",
  0x400b1698u64 => "
      CANFD.cfdrmdf_15()[4],
    ",
  0x400b16e4u64 => "
      CANFD.cfdrmdf_15()[5],
    ",
  0x400b1730u64 => "
      CANFD.cfdrmdf_15()[6],
    ",
  0x400b177cu64 => "
      CANFD.cfdrmdf_15()[7],
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
  0x400e002cu64 => "
      PSCU.dlmmon(),
    ",
  0x400e8000u64 => "
      AGTW_B_0.agt(),
    ",
  0x400e8004u64 => "
      AGTW_B_0.agtcma(),
    ",
  0x400e8008u64 => "
      AGTW_B_0.agtcmb(),
    ",
  0x400e800cu64 => "
      AGTW_B_0.agtcr(),
    ",
  0x400e800du64 => "
      AGTW_B_0.agtmr1(),
    ",
  0x400e800eu64 => "
      AGTW_B_0.agtmr2(),
    ",
  0x400e8010u64 => "
      AGTW_B_0.agtioc(),
    ",
  0x400e8011u64 => "
      AGTW_B_0.agtisr(),
    ",
  0x400e8012u64 => "
      AGTW_B_0.agtcmsr(),
    ",
  0x400e8013u64 => "
      AGTW_B_0.agtiosel(),
    ",
  0x400f3000u64 => "
      TSN.tscr(),
    ",
  0x400f4000u64 => "
      ACMPHS_0.cmpctl(),
    ",
  0x400f4004u64 => "
      ACMPHS_0.cmpsel0(),
    ",
  0x400f4008u64 => "
      ACMPHS_0.cmpsel1(),
    ",
  0x400f400cu64 => "
      ACMPHS_0.cmpmon(),
    ",
  0x400f4010u64 => "
      ACMPHS_0.cpioc(),
    ",
  0x40108000u64 => "
      CRC.crccr0(),
    ",
  0x40108001u64 => "
      CRC.crccr1(),
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
  0x4010800cu64 => "
      CRC.crcsar(),
    ",
  0x40109000u64 => "
      DOC_B.docr(),
    ",
  0x40109004u64 => "
      DOC_B.dosr(),
    ",
  0x40109008u64 => "
      DOC_B.doscr(),
    ",
  0x4010900cu64 => "
      DOC_B.dodir(),
    ",
  0x40109010u64 => "
      DOC_B.dodsr0(),
    ",
  0x40109014u64 => "
      DOC_B.dodsr1(),
    ",
  0x40118000u64 => "
      SCI_B_0.rdr(),
      SCI_B_0.rdr_by(),
    ",
  0x40118004u64 => "
      SCI_B_0.tdr(),
      SCI_B_0.tdrll(),
    ",
  0x40118005u64 => "
      SCI_B_0.tdrlh(),
    ",
  0x40118008u64 => "
      SCI_B_0.ccr0(),
    ",
  0x4011800cu64 => "
      SCI_B_0.ccr1(),
    ",
  0x40118010u64 => "
      SCI_B_0.ccr2(),
    ",
  0x40118014u64 => "
      SCI_B_0.ccr3(),
    ",
  0x40118018u64 => "
      SCI_B_0.ccr4(),
    ",
  0x4011801cu64 => "
      SCI_B_0.cesr(),
    ",
  0x40118020u64 => "
      SCI_B_0.icr(),
    ",
  0x40118024u64 => "
      SCI_B_0.fcr(),
    ",
  0x4011802cu64 => "
      SCI_B_0.mcr(),
    ",
  0x40118030u64 => "
      SCI_B_0.dcr(),
    ",
  0x40118034u64 => "
      SCI_B_0.xcr0(),
    ",
  0x40118038u64 => "
      SCI_B_0.xcr1(),
    ",
  0x4011803cu64 => "
      SCI_B_0.xcr2(),
    ",
  0x40118048u64 => "
      SCI_B_0.csr(),
    ",
  0x4011804cu64 => "
      SCI_B_0.isr(),
    ",
  0x40118050u64 => "
      SCI_B_0.frsr(),
    ",
  0x40118054u64 => "
      SCI_B_0.ftsr(),
    ",
  0x40118058u64 => "
      SCI_B_0.msr(),
    ",
  0x4011805cu64 => "
      SCI_B_0.xsr0(),
    ",
  0x40118060u64 => "
      SCI_B_0.xsr1(),
    ",
  0x40118068u64 => "
      SCI_B_0.cfclr(),
    ",
  0x4011806cu64 => "
      SCI_B_0.icfclr(),
    ",
  0x40118070u64 => "
      SCI_B_0.ffclr(),
    ",
  0x40118074u64 => "
      SCI_B_0.mfclr(),
    ",
  0x40118078u64 => "
      SCI_B_0.xfclr(),
    ",
  0x4011a000u64 => "
      SPI_B_0.spdr(),
    ",
  0x4011a004u64 => "
      SPI_B_0.spdecr(),
    ",
  0x4011a008u64 => "
      SPI_B_0.spcr(),
    ",
  0x4011a00cu64 => "
      SPI_B_0.spcr2(),
    ",
  0x4011a010u64 => "
      SPI_B_0.spcr3(),
    ",
  0x4011a014u64 => "
      SPI_B_0.spcmd()[0],
    ",
  0x4011a018u64 => "
      SPI_B_0.spcmd()[1],
    ",
  0x4011a01cu64 => "
      SPI_B_0.spcmd()[2],
    ",
  0x4011a020u64 => "
      SPI_B_0.spcmd()[3],
    ",
  0x4011a024u64 => "
      SPI_B_0.spcmd()[4],
    ",
  0x4011a028u64 => "
      SPI_B_0.spcmd()[5],
    ",
  0x4011a02cu64 => "
      SPI_B_0.spcmd()[6],
    ",
  0x4011a030u64 => "
      SPI_B_0.spcmd()[7],
    ",
  0x4011a040u64 => "
      SPI_B_0.spdcr(),
    ",
  0x4011a044u64 => "
      SPI_B_0.spdcr2(),
    ",
  0x4011a050u64 => "
      SPI_B_0.spsr(),
    ",
  0x4011a058u64 => "
      SPI_B_0.sptfsr(),
    ",
  0x4011a05cu64 => "
      SPI_B_0.sprfsr(),
    ",
  0x4011a060u64 => "
      SPI_B_0.sppsr(),
    ",
  0x4011a068u64 => "
      SPI_B_0.spsrc(),
    ",
  0x4011a06cu64 => "
      SPI_B_0.spfcr(),
    ",
  0x4011f014u64 => "
      IIC_B_0.bctl(),
    ",
  0x4011f020u64 => "
      IIC_B_0.rstctl(),
    ",
  0x4011f024u64 => "
      IIC_B_0.prsst(),
    ",
  0x4011f060u64 => "
      IIC_B_0.bfctl(),
    ",
  0x4011f064u64 => "
      IIC_B_0.svctl(),
    ",
  0x4011f070u64 => "
      IIC_B_0.refckctl(),
    ",
  0x4011f074u64 => "
      IIC_B_0.stdbr(),
    ",
  0x4011f078u64 => "
      IIC_B_0.extbr(),
    ",
  0x4011f07cu64 => "
      IIC_B_0.bfrecdt(),
    ",
  0x4011f088u64 => "
      IIC_B_0.outctl(),
    ",
  0x4011f08cu64 => "
      IIC_B_0.inctl(),
    ",
  0x4011f090u64 => "
      IIC_B_0.tmoctl(),
    ",
  0x4011f0a0u64 => "
      IIC_B_0.ackctl(),
    ",
  0x4011f0a4u64 => "
      IIC_B_0.scstrctl(),
    ",
  0x4011f140u64 => "
      IIC_B_0.cndctl(),
    ",
  0x4011f158u64 => "
      IIC_B_0.ntdtbp0(),
      IIC_B_0.ntdtbp0_by(),
    ",
  0x4011f1d0u64 => "
      IIC_B_0.bst(),
    ",
  0x4011f1d4u64 => "
      IIC_B_0.bste(),
    ",
  0x4011f1d8u64 => "
      IIC_B_0.bie(),
    ",
  0x4011f1dcu64 => "
      IIC_B_0.bstfc(),
    ",
  0x4011f1e0u64 => "
      IIC_B_0.ntst(),
    ",
  0x4011f1e4u64 => "
      IIC_B_0.ntste(),
    ",
  0x4011f1e8u64 => "
      IIC_B_0.ntie(),
    ",
  0x4011f1ecu64 => "
      IIC_B_0.ntstfc(),
    ",
  0x4011f210u64 => "
      IIC_B_0.bcst(),
    ",
  0x4011f214u64 => "
      IIC_B_0.svst(),
    ",
  0x4011f2b0u64 => "
      IIC_B_0.sdatbas()[0],
    ",
  0x4011f2b4u64 => "
      IIC_B_0.sdatbas()[1],
    ",
  0x4011f2b8u64 => "
      IIC_B_0.sdatbas()[2],
    ",
  0x4011f330u64 => "
      IIC_B_0.svdvad()[0],
    ",
  0x4011f334u64 => "
      IIC_B_0.svdvad()[1],
    ",
  0x4011f338u64 => "
      IIC_B_0.svdvad()[2],
    ",
  0x4011f380u64 => "
      IIC_B_0.bitcnt(),
    ",
  0x4011f3ccu64 => "
      IIC_B_0.prstdbg(),
    ",
  0x4011f098u64 => "
      IIC_0_WU_B.wuctl(),
    ",
  0x4011f218u64 => "
      IIC_0_WU_B.wust(),
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
      GPT_320.gtwp(),
    ",
  0x40169004u64 => "
      GPT_320.gtstr(),
    ",
  0x40169008u64 => "
      GPT_320.gtstp(),
    ",
  0x4016900cu64 => "
      GPT_320.gtclr(),
    ",
  0x40169010u64 => "
      GPT_320.gtssr(),
    ",
  0x40169014u64 => "
      GPT_320.gtpsr(),
    ",
  0x40169018u64 => "
      GPT_320.gtcsr(),
    ",
  0x4016901cu64 => "
      GPT_320.gtupsr(),
    ",
  0x40169020u64 => "
      GPT_320.gtdnsr(),
    ",
  0x40169024u64 => "
      GPT_320.gticasr(),
    ",
  0x40169028u64 => "
      GPT_320.gticbsr(),
    ",
  0x4016902cu64 => "
      GPT_320.gtcr(),
    ",
  0x40169030u64 => "
      GPT_320.gtuddtyc(),
    ",
  0x40169034u64 => "
      GPT_320.gtior(),
    ",
  0x40169038u64 => "
      GPT_320.gtintad(),
    ",
  0x4016903cu64 => "
      GPT_320.gtst(),
    ",
  0x40169040u64 => "
      GPT_320.gtber(),
    ",
  0x40169044u64 => "
      GPT_320.gtitc(),
    ",
  0x40169048u64 => "
      GPT_320.gtcnt(),
    ",
  0x4016904cu64 => "
      GPT_320.gtccra(),
    ",
  0x40169050u64 => "
      GPT_320.gtccrb(),
    ",
  0x40169054u64 => "
      GPT_320.gtccrc(),
    ",
  0x40169058u64 => "
      GPT_320.gtccre(),
    ",
  0x4016905cu64 => "
      GPT_320.gtccrd(),
    ",
  0x40169060u64 => "
      GPT_320.gtccrf(),
    ",
  0x40169064u64 => "
      GPT_320.gtpr(),
    ",
  0x40169068u64 => "
      GPT_320.gtpbr(),
    ",
  0x4016906cu64 => "
      GPT_320.gtpdbr(),
    ",
  0x40169070u64 => "
      GPT_320.gtadtra(),
    ",
  0x40169074u64 => "
      GPT_320.gtadtbra(),
    ",
  0x40169078u64 => "
      GPT_320.gtadtdbra(),
    ",
  0x4016907cu64 => "
      GPT_320.gtadtrb(),
    ",
  0x40169080u64 => "
      GPT_320.gtadtbrb(),
    ",
  0x40169084u64 => "
      GPT_320.gtadtdbrb(),
    ",
  0x40169088u64 => "
      GPT_320.gtdtcr(),
    ",
  0x4016908cu64 => "
      GPT_320.gtdvu(),
    ",
  0x40169090u64 => "
      GPT_320.gtdvd(),
    ",
  0x40169094u64 => "
      GPT_320.gtdbu(),
    ",
  0x40169098u64 => "
      GPT_320.gtdbd(),
    ",
  0x4016909cu64 => "
      GPT_320.gtsos(),
    ",
  0x401690a0u64 => "
      GPT_320.gtsotr(),
    ",
  0x401690a4u64 => "
      GPT_320.gtadsmr(),
    ",
  0x401690a8u64 => "
      GPT_320.gteitc(),
    ",
  0x401690acu64 => "
      GPT_320.gteitli1(),
    ",
  0x401690b0u64 => "
      GPT_320.gteitli2(),
    ",
  0x401690b4u64 => "
      GPT_320.gteitlb(),
    ",
  0x401690b8u64 => "
      GPT_320.gticlf(),
    ",
  0x401690bcu64 => "
      GPT_320.gtpc(),
    ",
  0x401690d0u64 => "
      GPT_320.gtsecsr(),
    ",
  0x401690d4u64 => "
      GPT_320.gtsecr(),
    ",
  0x401690e0u64 => "
      GPT_320.gtber2(),
    ",
  0x401690e4u64 => "
      GPT_320.gtolbr(),
    ",
  0x401690ecu64 => "
      GPT_320.gticcr(),
    ",
  0x40169400u64 => "
      GPT_324.gtwp(),
    ",
  0x40169404u64 => "
      GPT_324.gtstr(),
    ",
  0x40169408u64 => "
      GPT_324.gtstp(),
    ",
  0x4016940cu64 => "
      GPT_324.gtclr(),
    ",
  0x40169410u64 => "
      GPT_324.gtssr(),
    ",
  0x40169414u64 => "
      GPT_324.gtpsr(),
    ",
  0x40169418u64 => "
      GPT_324.gtcsr(),
    ",
  0x40169424u64 => "
      GPT_324.gticasr(),
    ",
  0x40169428u64 => "
      GPT_324.gticbsr(),
    ",
  0x4016942cu64 => "
      GPT_324.gtcr(),
    ",
  0x40169430u64 => "
      GPT_324.gtuddtyc(),
    ",
  0x40169434u64 => "
      GPT_324.gtior(),
    ",
  0x40169438u64 => "
      GPT_324.gtintad(),
    ",
  0x4016943cu64 => "
      GPT_324.gtst(),
    ",
  0x40169440u64 => "
      GPT_324.gtber(),
    ",
  0x40169444u64 => "
      GPT_324.gtitc(),
    ",
  0x40169448u64 => "
      GPT_324.gtcnt(),
    ",
  0x4016944cu64 => "
      GPT_324.gtccra(),
    ",
  0x40169450u64 => "
      GPT_324.gtccrb(),
    ",
  0x40169454u64 => "
      GPT_324.gtccrc(),
    ",
  0x40169458u64 => "
      GPT_324.gtccre(),
    ",
  0x4016945cu64 => "
      GPT_324.gtccrd(),
    ",
  0x40169460u64 => "
      GPT_324.gtccrf(),
    ",
  0x40169464u64 => "
      GPT_324.gtpr(),
    ",
  0x40169468u64 => "
      GPT_324.gtpbr(),
    ",
  0x4016946cu64 => "
      GPT_324.gtpdbr(),
    ",
  0x40169470u64 => "
      GPT_324.gtadtra(),
    ",
  0x40169474u64 => "
      GPT_324.gtadtbra(),
    ",
  0x40169478u64 => "
      GPT_324.gtadtdbra(),
    ",
  0x4016947cu64 => "
      GPT_324.gtadtrb(),
    ",
  0x40169480u64 => "
      GPT_324.gtadtbrb(),
    ",
  0x40169484u64 => "
      GPT_324.gtadtdbrb(),
    ",
  0x40169488u64 => "
      GPT_324.gtdtcr(),
    ",
  0x4016948cu64 => "
      GPT_324.gtdvu(),
    ",
  0x40169490u64 => "
      GPT_324.gtdvd(),
    ",
  0x40169494u64 => "
      GPT_324.gtdbu(),
    ",
  0x40169498u64 => "
      GPT_324.gtdbd(),
    ",
  0x4016949cu64 => "
      GPT_324.gtsos(),
    ",
  0x401694a0u64 => "
      GPT_324.gtsotr(),
    ",
  0x401694a4u64 => "
      GPT_324.gtadsmr(),
    ",
  0x401694a8u64 => "
      GPT_324.gteitc(),
    ",
  0x401694acu64 => "
      GPT_324.gteitli1(),
    ",
  0x401694b0u64 => "
      GPT_324.gteitli2(),
    ",
  0x401694b4u64 => "
      GPT_324.gteitlb(),
    ",
  0x401694b8u64 => "
      GPT_324.gticlf(),
    ",
  0x401694c0u64 => "
      GPT_324.gtadcmsc(),
    ",
  0x401694c4u64 => "
      GPT_324.gtadcmss(),
    ",
  0x401694d0u64 => "
      GPT_324.gtsecsr(),
    ",
  0x401694d4u64 => "
      GPT_324.gtsecr(),
    ",
  0x401694e0u64 => "
      GPT_324.gtber2(),
    ",
  0x401694e4u64 => "
      GPT_324.gtolbr(),
    ",
  0x401694ecu64 => "
      GPT_324.gticcr(),
    ",
  0x40169a00u64 => "
      GPT_OPS.opscr(),
    ",
  0x40169b00u64 => "
      GPT_GTCLK.gtclkcr(),
    ",
  0x4016a000u64 => "
      PDG.gtdlycr(),
    ",
  0x4016a002u64 => "
      PDG.gtdlycr2(),
    ",
  0x4016a018u64 => "
      PDG.gtdlyra()[0],
    ",
  0x4016a01cu64 => "
      PDG.gtdlyra()[1],
    ",
  0x4016a020u64 => "
      PDG.gtdlyra()[2],
    ",
  0x4016a024u64 => "
      PDG.gtdlyra()[3],
    ",
  0x4016a01au64 => "
      PDG.gtdlyrb()[0],
    ",
  0x4016a01eu64 => "
      PDG.gtdlyrb()[1],
    ",
  0x4016a022u64 => "
      PDG.gtdlyrb()[2],
    ",
  0x4016a026u64 => "
      PDG.gtdlyrb()[3],
    ",
  0x4016a028u64 => "
      PDG.gtdlyfa()[0],
    ",
  0x4016a02cu64 => "
      PDG.gtdlyfa()[1],
    ",
  0x4016a030u64 => "
      PDG.gtdlyfa()[2],
    ",
  0x4016a034u64 => "
      PDG.gtdlyfa()[3],
    ",
  0x4016a02au64 => "
      PDG.gtdlyfb()[0],
    ",
  0x4016a02eu64 => "
      PDG.gtdlyfb()[1],
    ",
  0x4016a032u64 => "
      PDG.gtdlyfb()[2],
    ",
  0x4016a036u64 => "
      PDG.gtdlyfb()[3],
    ",
  0x40170000u64 => "
      ADC_B.adclkenr(),
    ",
  0x40170004u64 => "
      ADC_B.adclksr(),
    ",
  0x40170008u64 => "
      ADC_B.adclkcr(),
    ",
  0x4017000cu64 => "
      ADC_B.adsycr(),
    ",
  0x40170020u64 => "
      ADC_B.aderintcr(),
    ",
  0x40170024u64 => "
      ADC_B.adovfintcr(),
    ",
  0x40170028u64 => "
      ADC_B.adcalintcr(),
    ",
  0x40170040u64 => "
      ADC_B.admdr(),
    ",
  0x40170044u64 => "
      ADC_B.adgspcr(),
    ",
  0x40170048u64 => "
      ADC_B.adsger(),
    ",
  0x4017004cu64 => "
      ADC_B.adsgcr0(),
    ",
  0x40170050u64 => "
      ADC_B.adsgcr1(),
    ",
  0x40170054u64 => "
      ADC_B.adsgcr2(),
    ",
  0x4017005cu64 => "
      ADC_B.adintcr(),
    ",
  0x401700c0u64 => "
      ADC_B.adtrgext()[0],
    ",
  0x401700d0u64 => "
      ADC_B.adtrgext()[1],
    ",
  0x401700e0u64 => "
      ADC_B.adtrgext()[2],
    ",
  0x401700f0u64 => "
      ADC_B.adtrgext()[3],
    ",
  0x40170100u64 => "
      ADC_B.adtrgext()[4],
    ",
  0x40170110u64 => "
      ADC_B.adtrgext()[5],
    ",
  0x40170120u64 => "
      ADC_B.adtrgext()[6],
    ",
  0x40170130u64 => "
      ADC_B.adtrgext()[7],
    ",
  0x40170140u64 => "
      ADC_B.adtrgext()[8],
    ",
  0x401700c4u64 => "
      ADC_B.adtrgelc()[0],
    ",
  0x401700d4u64 => "
      ADC_B.adtrgelc()[1],
    ",
  0x401700e4u64 => "
      ADC_B.adtrgelc()[2],
    ",
  0x401700f4u64 => "
      ADC_B.adtrgelc()[3],
    ",
  0x40170104u64 => "
      ADC_B.adtrgelc()[4],
    ",
  0x40170114u64 => "
      ADC_B.adtrgelc()[5],
    ",
  0x40170124u64 => "
      ADC_B.adtrgelc()[6],
    ",
  0x40170134u64 => "
      ADC_B.adtrgelc()[7],
    ",
  0x40170144u64 => "
      ADC_B.adtrgelc()[8],
    ",
  0x401700c8u64 => "
      ADC_B.adtrggpt()[0],
    ",
  0x401700d8u64 => "
      ADC_B.adtrggpt()[1],
    ",
  0x401700e8u64 => "
      ADC_B.adtrggpt()[2],
    ",
  0x401700f8u64 => "
      ADC_B.adtrggpt()[3],
    ",
  0x40170108u64 => "
      ADC_B.adtrggpt()[4],
    ",
  0x40170118u64 => "
      ADC_B.adtrggpt()[5],
    ",
  0x40170128u64 => "
      ADC_B.adtrggpt()[6],
    ",
  0x40170138u64 => "
      ADC_B.adtrggpt()[7],
    ",
  0x40170148u64 => "
      ADC_B.adtrggpt()[8],
    ",
  0x401701c0u64 => "
      ADC_B.adtrgdlr0(),
    ",
  0x401701c4u64 => "
      ADC_B.adtrgdlr1(),
    ",
  0x401701c8u64 => "
      ADC_B.adtrgdlr2(),
    ",
  0x401701ccu64 => "
      ADC_B.adtrgdlr3(),
    ",
  0x401701d0u64 => "
      ADC_B.adtrgdlr4(),
    ",
  0x40170200u64 => "
      ADC_B.adsgdcr()[0],
    ",
  0x40170204u64 => "
      ADC_B.adsgdcr()[1],
    ",
  0x40170208u64 => "
      ADC_B.adsgdcr()[2],
    ",
  0x4017020cu64 => "
      ADC_B.adsgdcr()[3],
    ",
  0x40170210u64 => "
      ADC_B.adsgdcr()[4],
    ",
  0x40170214u64 => "
      ADC_B.adsgdcr()[5],
    ",
  0x40170218u64 => "
      ADC_B.adsgdcr()[6],
    ",
  0x4017021cu64 => "
      ADC_B.adsgdcr()[7],
    ",
  0x40170220u64 => "
      ADC_B.adsgdcr()[8],
    ",
  0x40170240u64 => "
      ADC_B.adsstr0(),
    ",
  0x40170244u64 => "
      ADC_B.adsstr1(),
    ",
  0x40170248u64 => "
      ADC_B.adsstr2(),
    ",
  0x4017024cu64 => "
      ADC_B.adsstr3(),
    ",
  0x40170250u64 => "
      ADC_B.adsstr4(),
    ",
  0x40170254u64 => "
      ADC_B.adsstr5(),
    ",
  0x40170258u64 => "
      ADC_B.adsstr6(),
    ",
  0x4017025cu64 => "
      ADC_B.adsstr7(),
    ",
  0x40170260u64 => "
      ADC_B.adcnvstr(),
    ",
  0x40170264u64 => "
      ADC_B.adcalstcr(),
    ",
  0x40170280u64 => "
      ADC_B.adshcr0(),
    ",
  0x40170288u64 => "
      ADC_B.adshstr0(),
    ",
  0x4017028cu64 => "
      ADC_B.adshcr1(),
    ",
  0x40170294u64 => "
      ADC_B.adshstr1(),
    ",
  0x401702b0u64 => "
      ADC_B.adcalshcr(),
    ",
  0x401702c0u64 => "
      ADC_B.adpgacr()[0],
    ",
  0x401702c4u64 => "
      ADC_B.adpgacr()[1],
    ",
  0x401702c8u64 => "
      ADC_B.adpgacr()[2],
    ",
  0x401702ccu64 => "
      ADC_B.adpgacr()[3],
    ",
  0x40170300u64 => "
      ADC_B.adpgamoncr(),
    ",
  0x40170320u64 => "
      ADC_B.adrefcr(),
    ",
  0x40170340u64 => "
      ADC_B.addfsr()[0],
    ",
  0x40170344u64 => "
      ADC_B.addfsr()[1],
    ",
  0x40170360u64 => "
      ADC_B.aduoftr()[0],
    ",
  0x40170364u64 => "
      ADC_B.aduoftr()[1],
    ",
  0x40170368u64 => "
      ADC_B.aduoftr()[2],
    ",
  0x4017036cu64 => "
      ADC_B.aduoftr()[3],
    ",
  0x40170370u64 => "
      ADC_B.aduoftr()[4],
    ",
  0x40170374u64 => "
      ADC_B.aduoftr()[5],
    ",
  0x40170378u64 => "
      ADC_B.aduoftr()[6],
    ",
  0x4017037cu64 => "
      ADC_B.aduoftr()[7],
    ",
  0x40170380u64 => "
      ADC_B.adugtr()[0],
    ",
  0x40170384u64 => "
      ADC_B.adugtr()[1],
    ",
  0x40170388u64 => "
      ADC_B.adugtr()[2],
    ",
  0x4017038cu64 => "
      ADC_B.adugtr()[3],
    ",
  0x40170390u64 => "
      ADC_B.adugtr()[4],
    ",
  0x40170394u64 => "
      ADC_B.adugtr()[5],
    ",
  0x40170398u64 => "
      ADC_B.adugtr()[6],
    ",
  0x4017039cu64 => "
      ADC_B.adugtr()[7],
    ",
  0x401703a0u64 => "
      ADC_B.adlimintcr(),
    ",
  0x401703a4u64 => "
      ADC_B.adlimtr()[0],
    ",
  0x401703a8u64 => "
      ADC_B.adlimtr()[1],
    ",
  0x401703acu64 => "
      ADC_B.adlimtr()[2],
    ",
  0x401703b0u64 => "
      ADC_B.adlimtr()[3],
    ",
  0x401703b4u64 => "
      ADC_B.adlimtr()[4],
    ",
  0x401703b8u64 => "
      ADC_B.adlimtr()[5],
    ",
  0x401703bcu64 => "
      ADC_B.adlimtr()[6],
    ",
  0x401703c0u64 => "
      ADC_B.adlimtr()[7],
    ",
  0x40170400u64 => "
      ADC_B.adcmpenr(),
    ",
  0x40170404u64 => "
      ADC_B.adcmpintcr(),
    ",
  0x40170408u64 => "
      ADC_B.adccmpcr()[0],
    ",
  0x4017040cu64 => "
      ADC_B.adccmpcr()[1],
    ",
  0x40170448u64 => "
      ADC_B.adcmpmdr0(),
    ",
  0x4017044cu64 => "
      ADC_B.adcmpmdr1(),
    ",
  0x40170458u64 => "
      ADC_B.adcmptbr()[0],
    ",
  0x4017045cu64 => "
      ADC_B.adcmptbr()[1],
    ",
  0x40170460u64 => "
      ADC_B.adcmptbr()[2],
    ",
  0x40170464u64 => "
      ADC_B.adcmptbr()[3],
    ",
  0x40170468u64 => "
      ADC_B.adcmptbr()[4],
    ",
  0x4017046cu64 => "
      ADC_B.adcmptbr()[5],
    ",
  0x40170470u64 => "
      ADC_B.adcmptbr()[6],
    ",
  0x40170474u64 => "
      ADC_B.adcmptbr()[7],
    ",
  0x401704c0u64 => "
      ADC_B.adfifocr(),
    ",
  0x401704c4u64 => "
      ADC_B.adfifointcr(),
    ",
  0x401704c8u64 => "
      ADC_B.adfifointlr0(),
    ",
  0x401704ccu64 => "
      ADC_B.adfifointlr1(),
    ",
  0x401704d0u64 => "
      ADC_B.adfifointlr2(),
    ",
  0x401704d4u64 => "
      ADC_B.adfifointlr3(),
    ",
  0x401704d8u64 => "
      ADC_B.adfifointlr4(),
    ",
  0x40170600u64 => "
      ADC_B.adchcr()[0],
    ",
  0x40170610u64 => "
      ADC_B.adchcr()[1],
    ",
  0x40170620u64 => "
      ADC_B.adchcr()[2],
    ",
  0x40170630u64 => "
      ADC_B.adchcr()[3],
    ",
  0x40170640u64 => "
      ADC_B.adchcr()[4],
    ",
  0x40170650u64 => "
      ADC_B.adchcr()[5],
    ",
  0x40170660u64 => "
      ADC_B.adchcr()[6],
    ",
  0x40170670u64 => "
      ADC_B.adchcr()[7],
    ",
  0x40170680u64 => "
      ADC_B.adchcr()[8],
    ",
  0x40170690u64 => "
      ADC_B.adchcr()[9],
    ",
  0x401706a0u64 => "
      ADC_B.adchcr()[10],
    ",
  0x401706b0u64 => "
      ADC_B.adchcr()[11],
    ",
  0x401706c0u64 => "
      ADC_B.adchcr()[12],
    ",
  0x401706d0u64 => "
      ADC_B.adchcr()[13],
    ",
  0x401706e0u64 => "
      ADC_B.adchcr()[14],
    ",
  0x401706f0u64 => "
      ADC_B.adchcr()[15],
    ",
  0x40170700u64 => "
      ADC_B.adchcr()[16],
    ",
  0x40170710u64 => "
      ADC_B.adchcr()[17],
    ",
  0x40170720u64 => "
      ADC_B.adchcr()[18],
    ",
  0x40170730u64 => "
      ADC_B.adchcr()[19],
    ",
  0x40170740u64 => "
      ADC_B.adchcr()[20],
    ",
  0x40170750u64 => "
      ADC_B.adchcr()[21],
    ",
  0x40170760u64 => "
      ADC_B.adchcr()[22],
    ",
  0x40170770u64 => "
      ADC_B.adchcr()[23],
    ",
  0x40170780u64 => "
      ADC_B.adchcr()[24],
    ",
  0x40170790u64 => "
      ADC_B.adchcr()[25],
    ",
  0x401707a0u64 => "
      ADC_B.adchcr()[26],
    ",
  0x401707b0u64 => "
      ADC_B.adchcr()[27],
    ",
  0x401707c0u64 => "
      ADC_B.adchcr()[28],
    ",
  0x401707d0u64 => "
      ADC_B.adchcr()[29],
    ",
  0x401707e0u64 => "
      ADC_B.adchcr()[30],
    ",
  0x401707f0u64 => "
      ADC_B.adchcr()[31],
    ",
  0x40170800u64 => "
      ADC_B.adchcr()[32],
    ",
  0x40170810u64 => "
      ADC_B.adchcr()[33],
    ",
  0x40170820u64 => "
      ADC_B.adchcr()[34],
    ",
  0x40170830u64 => "
      ADC_B.adchcr()[35],
    ",
  0x40170840u64 => "
      ADC_B.adchcr()[36],
    ",
  0x40170604u64 => "
      ADC_B.addopcra()[0],
    ",
  0x40170614u64 => "
      ADC_B.addopcra()[1],
    ",
  0x40170624u64 => "
      ADC_B.addopcra()[2],
    ",
  0x40170634u64 => "
      ADC_B.addopcra()[3],
    ",
  0x40170644u64 => "
      ADC_B.addopcra()[4],
    ",
  0x40170654u64 => "
      ADC_B.addopcra()[5],
    ",
  0x40170664u64 => "
      ADC_B.addopcra()[6],
    ",
  0x40170674u64 => "
      ADC_B.addopcra()[7],
    ",
  0x40170684u64 => "
      ADC_B.addopcra()[8],
    ",
  0x40170694u64 => "
      ADC_B.addopcra()[9],
    ",
  0x401706a4u64 => "
      ADC_B.addopcra()[10],
    ",
  0x401706b4u64 => "
      ADC_B.addopcra()[11],
    ",
  0x401706c4u64 => "
      ADC_B.addopcra()[12],
    ",
  0x401706d4u64 => "
      ADC_B.addopcra()[13],
    ",
  0x401706e4u64 => "
      ADC_B.addopcra()[14],
    ",
  0x401706f4u64 => "
      ADC_B.addopcra()[15],
    ",
  0x40170704u64 => "
      ADC_B.addopcra()[16],
    ",
  0x40170714u64 => "
      ADC_B.addopcra()[17],
    ",
  0x40170724u64 => "
      ADC_B.addopcra()[18],
    ",
  0x40170734u64 => "
      ADC_B.addopcra()[19],
    ",
  0x40170744u64 => "
      ADC_B.addopcra()[20],
    ",
  0x40170754u64 => "
      ADC_B.addopcra()[21],
    ",
  0x40170764u64 => "
      ADC_B.addopcra()[22],
    ",
  0x40170774u64 => "
      ADC_B.addopcra()[23],
    ",
  0x40170784u64 => "
      ADC_B.addopcra()[24],
    ",
  0x40170794u64 => "
      ADC_B.addopcra()[25],
    ",
  0x401707a4u64 => "
      ADC_B.addopcra()[26],
    ",
  0x401707b4u64 => "
      ADC_B.addopcra()[27],
    ",
  0x401707c4u64 => "
      ADC_B.addopcra()[28],
    ",
  0x401707d4u64 => "
      ADC_B.addopcra()[29],
    ",
  0x401707e4u64 => "
      ADC_B.addopcra()[30],
    ",
  0x401707f4u64 => "
      ADC_B.addopcra()[31],
    ",
  0x40170804u64 => "
      ADC_B.addopcra()[32],
    ",
  0x40170814u64 => "
      ADC_B.addopcra()[33],
    ",
  0x40170824u64 => "
      ADC_B.addopcra()[34],
    ",
  0x40170834u64 => "
      ADC_B.addopcra()[35],
    ",
  0x40170844u64 => "
      ADC_B.addopcra()[36],
    ",
  0x40170608u64 => "
      ADC_B.addopcrb()[0],
    ",
  0x40170618u64 => "
      ADC_B.addopcrb()[1],
    ",
  0x40170628u64 => "
      ADC_B.addopcrb()[2],
    ",
  0x40170638u64 => "
      ADC_B.addopcrb()[3],
    ",
  0x40170648u64 => "
      ADC_B.addopcrb()[4],
    ",
  0x40170658u64 => "
      ADC_B.addopcrb()[5],
    ",
  0x40170668u64 => "
      ADC_B.addopcrb()[6],
    ",
  0x40170678u64 => "
      ADC_B.addopcrb()[7],
    ",
  0x40170688u64 => "
      ADC_B.addopcrb()[8],
    ",
  0x40170698u64 => "
      ADC_B.addopcrb()[9],
    ",
  0x401706a8u64 => "
      ADC_B.addopcrb()[10],
    ",
  0x401706b8u64 => "
      ADC_B.addopcrb()[11],
    ",
  0x401706c8u64 => "
      ADC_B.addopcrb()[12],
    ",
  0x401706d8u64 => "
      ADC_B.addopcrb()[13],
    ",
  0x401706e8u64 => "
      ADC_B.addopcrb()[14],
    ",
  0x401706f8u64 => "
      ADC_B.addopcrb()[15],
    ",
  0x40170708u64 => "
      ADC_B.addopcrb()[16],
    ",
  0x40170718u64 => "
      ADC_B.addopcrb()[17],
    ",
  0x40170728u64 => "
      ADC_B.addopcrb()[18],
    ",
  0x40170738u64 => "
      ADC_B.addopcrb()[19],
    ",
  0x40170748u64 => "
      ADC_B.addopcrb()[20],
    ",
  0x40170758u64 => "
      ADC_B.addopcrb()[21],
    ",
  0x40170768u64 => "
      ADC_B.addopcrb()[22],
    ",
  0x40170778u64 => "
      ADC_B.addopcrb()[23],
    ",
  0x40170788u64 => "
      ADC_B.addopcrb()[24],
    ",
  0x40170798u64 => "
      ADC_B.addopcrb()[25],
    ",
  0x401707a8u64 => "
      ADC_B.addopcrb()[26],
    ",
  0x401707b8u64 => "
      ADC_B.addopcrb()[27],
    ",
  0x401707c8u64 => "
      ADC_B.addopcrb()[28],
    ",
  0x401707d8u64 => "
      ADC_B.addopcrb()[29],
    ",
  0x401707e8u64 => "
      ADC_B.addopcrb()[30],
    ",
  0x401707f8u64 => "
      ADC_B.addopcrb()[31],
    ",
  0x40170808u64 => "
      ADC_B.addopcrb()[32],
    ",
  0x40170818u64 => "
      ADC_B.addopcrb()[33],
    ",
  0x40170828u64 => "
      ADC_B.addopcrb()[34],
    ",
  0x40170838u64 => "
      ADC_B.addopcrb()[35],
    ",
  0x40170848u64 => "
      ADC_B.addopcrb()[36],
    ",
  0x4017060cu64 => "
      ADC_B.addopcrc()[0],
    ",
  0x4017061cu64 => "
      ADC_B.addopcrc()[1],
    ",
  0x4017062cu64 => "
      ADC_B.addopcrc()[2],
    ",
  0x4017063cu64 => "
      ADC_B.addopcrc()[3],
    ",
  0x4017064cu64 => "
      ADC_B.addopcrc()[4],
    ",
  0x4017065cu64 => "
      ADC_B.addopcrc()[5],
    ",
  0x4017066cu64 => "
      ADC_B.addopcrc()[6],
    ",
  0x4017067cu64 => "
      ADC_B.addopcrc()[7],
    ",
  0x4017068cu64 => "
      ADC_B.addopcrc()[8],
    ",
  0x4017069cu64 => "
      ADC_B.addopcrc()[9],
    ",
  0x401706acu64 => "
      ADC_B.addopcrc()[10],
    ",
  0x401706bcu64 => "
      ADC_B.addopcrc()[11],
    ",
  0x401706ccu64 => "
      ADC_B.addopcrc()[12],
    ",
  0x401706dcu64 => "
      ADC_B.addopcrc()[13],
    ",
  0x401706ecu64 => "
      ADC_B.addopcrc()[14],
    ",
  0x401706fcu64 => "
      ADC_B.addopcrc()[15],
    ",
  0x4017070cu64 => "
      ADC_B.addopcrc()[16],
    ",
  0x4017071cu64 => "
      ADC_B.addopcrc()[17],
    ",
  0x4017072cu64 => "
      ADC_B.addopcrc()[18],
    ",
  0x4017073cu64 => "
      ADC_B.addopcrc()[19],
    ",
  0x4017074cu64 => "
      ADC_B.addopcrc()[20],
    ",
  0x4017075cu64 => "
      ADC_B.addopcrc()[21],
    ",
  0x4017076cu64 => "
      ADC_B.addopcrc()[22],
    ",
  0x4017077cu64 => "
      ADC_B.addopcrc()[23],
    ",
  0x4017078cu64 => "
      ADC_B.addopcrc()[24],
    ",
  0x4017079cu64 => "
      ADC_B.addopcrc()[25],
    ",
  0x401707acu64 => "
      ADC_B.addopcrc()[26],
    ",
  0x401707bcu64 => "
      ADC_B.addopcrc()[27],
    ",
  0x401707ccu64 => "
      ADC_B.addopcrc()[28],
    ",
  0x401707dcu64 => "
      ADC_B.addopcrc()[29],
    ",
  0x401707ecu64 => "
      ADC_B.addopcrc()[30],
    ",
  0x401707fcu64 => "
      ADC_B.addopcrc()[31],
    ",
  0x4017080cu64 => "
      ADC_B.addopcrc()[32],
    ",
  0x4017081cu64 => "
      ADC_B.addopcrc()[33],
    ",
  0x4017082cu64 => "
      ADC_B.addopcrc()[34],
    ",
  0x4017083cu64 => "
      ADC_B.addopcrc()[35],
    ",
  0x4017084cu64 => "
      ADC_B.addopcrc()[36],
    ",
  0x40170c00u64 => "
      ADC_B.adcalstr(),
    ",
  0x40170c08u64 => "
      ADC_B.adtrgenr(),
    ",
  0x40170c10u64 => "
      ADC_B.adsystr(),
    ",
  0x40170c20u64 => "
      ADC_B.adstr()[0],
    ",
  0x40170c24u64 => "
      ADC_B.adstr()[1],
    ",
  0x40170c28u64 => "
      ADC_B.adstr()[2],
    ",
  0x40170c2cu64 => "
      ADC_B.adstr()[3],
    ",
  0x40170c30u64 => "
      ADC_B.adstr()[4],
    ",
  0x40170c34u64 => "
      ADC_B.adstr()[5],
    ",
  0x40170c38u64 => "
      ADC_B.adstr()[6],
    ",
  0x40170c3cu64 => "
      ADC_B.adstr()[7],
    ",
  0x40170c40u64 => "
      ADC_B.adstr()[8],
    ",
  0x40170c60u64 => "
      ADC_B.adstopr(),
    ",
  0x40170c80u64 => "
      ADC_B.adsr(),
    ",
  0x40170c84u64 => "
      ADC_B.adgrsr(),
    ",
  0x40170c88u64 => "
      ADC_B.adersr(),
    ",
  0x40170c8cu64 => "
      ADC_B.aderscr(),
    ",
  0x40170c98u64 => "
      ADC_B.adcalendsr(),
    ",
  0x40170c9cu64 => "
      ADC_B.adcalendscr(),
    ",
  0x40170ca0u64 => "
      ADC_B.adovfersr(),
    ",
  0x40170ca4u64 => "
      ADC_B.adovfchsr0(),
    ",
  0x40170cb0u64 => "
      ADC_B.adovfexsr(),
    ",
  0x40170cb4u64 => "
      ADC_B.adovferscr(),
    ",
  0x40170cb8u64 => "
      ADC_B.adovfchscr0(),
    ",
  0x40170cc4u64 => "
      ADC_B.adovfexscr(),
    ",
  0x40170cd0u64 => "
      ADC_B.adfifosr0(),
    ",
  0x40170cd4u64 => "
      ADC_B.adfifosr1(),
    ",
  0x40170cd8u64 => "
      ADC_B.adfifosr2(),
    ",
  0x40170cdcu64 => "
      ADC_B.adfifosr3(),
    ",
  0x40170ce0u64 => "
      ADC_B.adfifosr4(),
    ",
  0x40170cf0u64 => "
      ADC_B.adfifodcr(),
    ",
  0x40170cf4u64 => "
      ADC_B.adfifoersr(),
    ",
  0x40170cf8u64 => "
      ADC_B.adfifoerscr(),
    ",
  0x40170d00u64 => "
      ADC_B.adcmptbsr(),
    ",
  0x40170d04u64 => "
      ADC_B.adcmptbscr(),
    ",
  0x40170d08u64 => "
      ADC_B.adcmpchsr0(),
    ",
  0x40170d14u64 => "
      ADC_B.adcmpexsr(),
    ",
  0x40170d18u64 => "
      ADC_B.adcmpchscr0(),
    ",
  0x40170d24u64 => "
      ADC_B.adcmpexscr(),
    ",
  0x40170d28u64 => "
      ADC_B.adlimgrsr(),
    ",
  0x40170d2cu64 => "
      ADC_B.adlimchsr0(),
    ",
  0x40170d38u64 => "
      ADC_B.adlimexsr(),
    ",
  0x40170d3cu64 => "
      ADC_B.adlimgrscr(),
    ",
  0x40170d40u64 => "
      ADC_B.adlimchscr0(),
    ",
  0x40170d4cu64 => "
      ADC_B.adlimexscr(),
    ",
  0x40170d50u64 => "
      ADC_B.adscanendsr(),
    ",
  0x40170d54u64 => "
      ADC_B.adscanendscr(),
    ",
  0x40171000u64 => "
      ADC_B.addr()[0],
    ",
  0x40171004u64 => "
      ADC_B.addr()[1],
    ",
  0x40171008u64 => "
      ADC_B.addr()[2],
    ",
  0x4017100cu64 => "
      ADC_B.addr()[3],
    ",
  0x40171010u64 => "
      ADC_B.addr()[4],
    ",
  0x40171014u64 => "
      ADC_B.addr()[5],
    ",
  0x40171018u64 => "
      ADC_B.addr()[6],
    ",
  0x4017101cu64 => "
      ADC_B.addr()[7],
    ",
  0x40171020u64 => "
      ADC_B.addr()[8],
    ",
  0x40171024u64 => "
      ADC_B.addr()[9],
    ",
  0x40171028u64 => "
      ADC_B.addr()[10],
    ",
  0x4017102cu64 => "
      ADC_B.addr()[11],
    ",
  0x40171030u64 => "
      ADC_B.addr()[12],
    ",
  0x40171034u64 => "
      ADC_B.addr()[13],
    ",
  0x40171038u64 => "
      ADC_B.addr()[14],
    ",
  0x4017103cu64 => "
      ADC_B.addr()[15],
    ",
  0x40171040u64 => "
      ADC_B.addr()[16],
    ",
  0x40171044u64 => "
      ADC_B.addr()[17],
    ",
  0x40171048u64 => "
      ADC_B.addr()[18],
    ",
  0x4017104cu64 => "
      ADC_B.addr()[19],
    ",
  0x40171050u64 => "
      ADC_B.addr()[20],
    ",
  0x40171054u64 => "
      ADC_B.addr()[21],
    ",
  0x40171058u64 => "
      ADC_B.addr()[22],
    ",
  0x4017105cu64 => "
      ADC_B.addr()[23],
    ",
  0x40171060u64 => "
      ADC_B.addr()[24],
    ",
  0x40171064u64 => "
      ADC_B.addr()[25],
    ",
  0x40171068u64 => "
      ADC_B.addr()[26],
    ",
  0x4017106cu64 => "
      ADC_B.addr()[27],
    ",
  0x40171070u64 => "
      ADC_B.addr()[28],
    ",
  0x40171194u64 => "
      ADC_B.adexdr()[0],
    ",
  0x40171198u64 => "
      ADC_B.adexdr()[1],
    ",
  0x4017119cu64 => "
      ADC_B.adexdr()[2],
    ",
  0x401711a0u64 => "
      ADC_B.adexdr()[3],
    ",
  0x40171200u64 => "
      ADC_B.adfifodr()[0],
    ",
  0x40171204u64 => "
      ADC_B.adfifodr()[1],
    ",
  0x40171208u64 => "
      ADC_B.adfifodr()[2],
    ",
  0x4017120cu64 => "
      ADC_B.adfifodr()[3],
    ",
  0x40171210u64 => "
      ADC_B.adfifodr()[4],
    ",
  0x40171214u64 => "
      ADC_B.adfifodr()[5],
    ",
  0x40171218u64 => "
      ADC_B.adfifodr()[6],
    ",
  0x4017121cu64 => "
      ADC_B.adfifodr()[7],
    ",
  0x40171220u64 => "
      ADC_B.adfifodr()[8],
    ",
  0x40172000u64 => "
      DAC_120.dadr()[0],
    ",
  0x40172002u64 => "
      DAC_120.dadr()[1],
    ",
  0x40172004u64 => "
      DAC_120.dacr(),
    ",
  0x40172005u64 => "
      DAC_120.dadpr(),
    ",
  0x40172008u64 => "
      DAC_120.daampcr(),
    ",
  0x4017201cu64 => "
      DAC_120.daaswcr(),
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
  0x407fe078u64 => "
      FACI.fbprot0(),
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
};
