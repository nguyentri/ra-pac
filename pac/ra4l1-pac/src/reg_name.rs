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
// Generated from SVD 1.20.01, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:48:57 +0000

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
  0x400061a4u64 => "
      ICU.wupen1(),
    ",
  0x400061a8u64 => "
      ICU.wupen2(),
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
  0x40008058u64 => "
      CPSCU.icusarm(),
    ",
  0x40008070u64 => "
      CPSCU.icusarg(),
    ",
  0x40008074u64 => "
      CPSCU.icusarh(),
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
  0x4001e031u64 => "
      SYSC.memwait(),
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
  0x4001e040u64 => "
      SYSC.ostdcr(),
    ",
  0x4001e041u64 => "
      SYSC.ostdsr(),
    ",
  0x4001e042u64 => "
      SYSC.sostdcr(),
    ",
  0x4001e050u64 => "
      SYSC.slcdsckcr(),
    ",
  0x4001e061u64 => "
      SYSC.mocoutcr(),
    ",
  0x4001e062u64 => "
      SYSC.hocoutcr(),
    ",
  0x4001e06eu64 => "
      SYSC.canfdckdivcr(),
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
  0x4001e079u64 => "
      SYSC.i3cckcr(),
    ",
  0x4001e092u64 => "
      SYSC.snzcr(),
    ",
  0x4001e094u64 => "
      SYSC.snzedcr0(),
    ",
  0x4001e095u64 => "
      SYSC.snzedcr1(),
    ",
  0x4001e098u64 => "
      SYSC.snzreqcr0(),
    ",
  0x4001e09fu64 => "
      SYSC.psmcr(),
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
  0x4001e0acu64 => "
      SYSC.ldocr(),
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
  0x4001e3feu64 => "
      SYSC.prcr(),
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
  0x4001e482u64 => "
      SYSC.somrg(),
    ",
  0x4001e490u64 => "
      SYSC.lococr(),
    ",
  0x4001e492u64 => "
      SYSC.locoutcr(),
    ",
  0x4001f000u64 => "
      PORT_0.pcntr1(),
      PORT_0.podr(),
    ",
  0x4001f002u64 => "
      PORT_0.pdr(),
    ",
  0x4001f004u64 => "
      PORT_0.pcntr2(),
    ",
  0x4001f006u64 => "
      PORT_0.pidr(),
    ",
  0x4001f008u64 => "
      PORT_0.pcntr3(),
      PORT_0.porr(),
    ",
  0x4001f00au64 => "
      PORT_0.posr(),
    ",
  0x4001f020u64 => "
      PORT_1.pcntr1(),
      PORT_1.podr(),
    ",
  0x4001f022u64 => "
      PORT_1.pdr(),
    ",
  0x4001f024u64 => "
      PORT_1.pcntr2(),
      PORT_1.eidr(),
    ",
  0x4001f026u64 => "
      PORT_1.pidr(),
    ",
  0x4001f028u64 => "
      PORT_1.pcntr3(),
      PORT_1.porr(),
    ",
  0x4001f02au64 => "
      PORT_1.posr(),
    ",
  0x4001f02cu64 => "
      PORT_1.pcntr4(),
      PORT_1.eorr(),
    ",
  0x4001f02eu64 => "
      PORT_1.eosr(),
    ",
  0x4001f80cu64 => "
      PFS.p00pfs()[3],
    ",
  0x4001f80eu64 => "
      PFS.p00pfs_ha()[3],
    ",
  0x4001f80fu64 => "
      PFS.p00pfs_by()[3],
    ",
  0x4001f810u64 => "
      PFS.p00pfs()[0],
    ",
  0x4001f814u64 => "
      PFS.p00pfs()[1],
    ",
  0x4001f818u64 => "
      PFS.p00pfs()[2],
    ",
  0x4001f812u64 => "
      PFS.p00pfs_ha()[0],
    ",
  0x4001f816u64 => "
      PFS.p00pfs_ha()[1],
    ",
  0x4001f81au64 => "
      PFS.p00pfs_ha()[2],
    ",
  0x4001f813u64 => "
      PFS.p00pfs_by()[0],
    ",
  0x4001f817u64 => "
      PFS.p00pfs_by()[1],
    ",
  0x4001f81bu64 => "
      PFS.p00pfs_by()[2],
    ",
  0x4001f820u64 => "
      PFS.p008pfs(),
    ",
  0x4001f822u64 => "
      PFS.p008pfs_ha(),
    ",
  0x4001f823u64 => "
      PFS.p008pfs_by(),
    ",
  0x4001f834u64 => "
      PFS.p0pfs()[0],
    ",
  0x4001f838u64 => "
      PFS.p0pfs()[1],
    ",
  0x4001f83cu64 => "
      PFS.p0pfs()[2],
    ",
  0x4001f836u64 => "
      PFS.p0pfs_ha()[0],
    ",
  0x4001f83au64 => "
      PFS.p0pfs_ha()[1],
    ",
  0x4001f83eu64 => "
      PFS.p0pfs_ha()[2],
    ",
  0x4001f837u64 => "
      PFS.p0pfs_by()[0],
    ",
  0x4001f83bu64 => "
      PFS.p0pfs_by()[1],
    ",
  0x4001f83fu64 => "
      PFS.p0pfs_by()[2],
    ",
  0x4001f848u64 => "
      PFS.p10pfs()[0],
    ",
  0x4001f84cu64 => "
      PFS.p10pfs()[1],
    ",
  0x4001f850u64 => "
      PFS.p10pfs()[2],
    ",
  0x4001f854u64 => "
      PFS.p10pfs()[3],
    ",
  0x4001f858u64 => "
      PFS.p10pfs()[4],
    ",
  0x4001f85cu64 => "
      PFS.p10pfs()[5],
    ",
  0x4001f860u64 => "
      PFS.p10pfs()[6],
    ",
  0x4001f864u64 => "
      PFS.p10pfs()[7],
    ",
  0x4001f84au64 => "
      PFS.p10pfs_ha()[0],
    ",
  0x4001f84eu64 => "
      PFS.p10pfs_ha()[1],
    ",
  0x4001f852u64 => "
      PFS.p10pfs_ha()[2],
    ",
  0x4001f856u64 => "
      PFS.p10pfs_ha()[3],
    ",
  0x4001f85au64 => "
      PFS.p10pfs_ha()[4],
    ",
  0x4001f85eu64 => "
      PFS.p10pfs_ha()[5],
    ",
  0x4001f862u64 => "
      PFS.p10pfs_ha()[6],
    ",
  0x4001f866u64 => "
      PFS.p10pfs_ha()[7],
    ",
  0x4001f84bu64 => "
      PFS.p10pfs_by()[0],
    ",
  0x4001f84fu64 => "
      PFS.p10pfs_by()[1],
    ",
  0x4001f853u64 => "
      PFS.p10pfs_by()[2],
    ",
  0x4001f857u64 => "
      PFS.p10pfs_by()[3],
    ",
  0x4001f85bu64 => "
      PFS.p10pfs_by()[4],
    ",
  0x4001f85fu64 => "
      PFS.p10pfs_by()[5],
    ",
  0x4001f863u64 => "
      PFS.p10pfs_by()[6],
    ",
  0x4001f867u64 => "
      PFS.p10pfs_by()[7],
    ",
  0x4001f868u64 => "
      PFS.p1pfs()[0],
    ",
  0x4001f86cu64 => "
      PFS.p1pfs()[1],
    ",
  0x4001f870u64 => "
      PFS.p1pfs()[2],
    ",
  0x4001f874u64 => "
      PFS.p1pfs()[3],
    ",
  0x4001f86au64 => "
      PFS.p1pfs_ha()[0],
    ",
  0x4001f86eu64 => "
      PFS.p1pfs_ha()[1],
    ",
  0x4001f872u64 => "
      PFS.p1pfs_ha()[2],
    ",
  0x4001f876u64 => "
      PFS.p1pfs_ha()[3],
    ",
  0x4001f86bu64 => "
      PFS.p1pfs_by()[0],
    ",
  0x4001f86fu64 => "
      PFS.p1pfs_by()[1],
    ",
  0x4001f873u64 => "
      PFS.p1pfs_by()[2],
    ",
  0x4001f877u64 => "
      PFS.p1pfs_by()[3],
    ",
  0x4001f880u64 => "
      PFS.p200pfs(),
    ",
  0x4001f882u64 => "
      PFS.p200pfs_ha(),
    ",
  0x4001f883u64 => "
      PFS.p200pfs_by(),
    ",
  0x4001f884u64 => "
      PFS.p201pfs(),
    ",
  0x4001f886u64 => "
      PFS.p201pfs_ha(),
    ",
  0x4001f887u64 => "
      PFS.p201pfs_by(),
    ",
  0x4001f894u64 => "
      PFS.p20pfs()[0],
    ",
  0x4001f898u64 => "
      PFS.p20pfs()[1],
    ",
  0x4001f89cu64 => "
      PFS.p20pfs()[2],
    ",
  0x4001f8a0u64 => "
      PFS.p20pfs()[3],
    ",
  0x4001f896u64 => "
      PFS.p20pfs_ha()[0],
    ",
  0x4001f89au64 => "
      PFS.p20pfs_ha()[1],
    ",
  0x4001f89eu64 => "
      PFS.p20pfs_ha()[2],
    ",
  0x4001f8a2u64 => "
      PFS.p20pfs_ha()[3],
    ",
  0x4001f897u64 => "
      PFS.p20pfs_by()[0],
    ",
  0x4001f89bu64 => "
      PFS.p20pfs_by()[1],
    ",
  0x4001f89fu64 => "
      PFS.p20pfs_by()[2],
    ",
  0x4001f8a3u64 => "
      PFS.p20pfs_by()[3],
    ",
  0x4001f8b0u64 => "
      PFS.p2pfs()[0],
    ",
  0x4001f8b4u64 => "
      PFS.p2pfs()[1],
    ",
  0x4001f8b2u64 => "
      PFS.p2pfs_ha()[0],
    ",
  0x4001f8b6u64 => "
      PFS.p2pfs_ha()[1],
    ",
  0x4001f8b3u64 => "
      PFS.p2pfs_by()[0],
    ",
  0x4001f8b7u64 => "
      PFS.p2pfs_by()[1],
    ",
  0x4001f8c0u64 => "
      PFS.p300pfs(),
    ",
  0x4001f8c2u64 => "
      PFS.p300pfs_ha(),
    ",
  0x4001f8c3u64 => "
      PFS.p300pfs_by(),
    ",
  0x4001f8c4u64 => "
      PFS.p30pfs()[0],
    ",
  0x4001f8c8u64 => "
      PFS.p30pfs()[1],
    ",
  0x4001f8ccu64 => "
      PFS.p30pfs()[2],
    ",
  0x4001f8d0u64 => "
      PFS.p30pfs()[3],
    ",
  0x4001f8c6u64 => "
      PFS.p30pfs_ha()[0],
    ",
  0x4001f8cau64 => "
      PFS.p30pfs_ha()[1],
    ",
  0x4001f8ceu64 => "
      PFS.p30pfs_ha()[2],
    ",
  0x4001f8d2u64 => "
      PFS.p30pfs_ha()[3],
    ",
  0x4001f8c7u64 => "
      PFS.p30pfs_by()[0],
    ",
  0x4001f8cbu64 => "
      PFS.p30pfs_by()[1],
    ",
  0x4001f8cfu64 => "
      PFS.p30pfs_by()[2],
    ",
  0x4001f8d3u64 => "
      PFS.p30pfs_by()[3],
    ",
  0x4001f90cu64 => "
      PFS.p40pfs()[3],
    ",
  0x4001f90eu64 => "
      PFS.p40pfs_ha()[3],
    ",
  0x4001f90fu64 => "
      PFS.p40pfs_by()[3],
    ",
  0x4001f91cu64 => "
      PFS.p40pfs()[0],
    ",
  0x4001f920u64 => "
      PFS.p40pfs()[1],
    ",
  0x4001f924u64 => "
      PFS.p40pfs()[2],
    ",
  0x4001f91eu64 => "
      PFS.p40pfs_ha()[0],
    ",
  0x4001f922u64 => "
      PFS.p40pfs_ha()[1],
    ",
  0x4001f926u64 => "
      PFS.p40pfs_ha()[2],
    ",
  0x4001f91fu64 => "
      PFS.p40pfs_by()[0],
    ",
  0x4001f923u64 => "
      PFS.p40pfs_by()[1],
    ",
  0x4001f927u64 => "
      PFS.p40pfs_by()[2],
    ",
  0x4001f928u64 => "
      PFS.p4pfs()[0],
    ",
  0x4001f92cu64 => "
      PFS.p4pfs()[1],
    ",
  0x4001f92au64 => "
      PFS.p4pfs_ha()[0],
    ",
  0x4001f92eu64 => "
      PFS.p4pfs_ha()[1],
    ",
  0x4001f92bu64 => "
      PFS.p4pfs_by()[0],
    ",
  0x4001f92fu64 => "
      PFS.p4pfs_by()[1],
    ",
  0x4001f940u64 => "
      PFS.p500pfs(),
    ",
  0x4001f942u64 => "
      PFS.p500pfs_ha(),
    ",
  0x4001f943u64 => "
      PFS.p500pfs_by(),
    ",
  0x4001fa38u64 => "
      PFS.p8pfs()[0],
    ",
  0x4001fa3cu64 => "
      PFS.p8pfs()[1],
    ",
  0x4001fa3au64 => "
      PFS.p8pfs_ha()[0],
    ",
  0x4001fa3eu64 => "
      PFS.p8pfs_ha()[1],
    ",
  0x4001fa3bu64 => "
      PFS.p8pfs_by()[0],
    ",
  0x4001fa3fu64 => "
      PFS.p8pfs_by()[1],
    ",
  0x4001fd03u64 => "
      PFS.pwpr(),
    ",
  0x4001fd05u64 => "
      PFS.pwprs(),
    ",
  0x4001fd0cu64 => "
      PFS.prwcntr(),
    ",
  0x4001fd10u64 => "
      PFS.psar()[0],
    ",
  0x4001fd12u64 => "
      PFS.psar()[1],
    ",
  0x4001fd14u64 => "
      PFS.psar()[2],
    ",
  0x4001fd16u64 => "
      PFS.psar()[3],
    ",
  0x4001fd18u64 => "
      PFS.psar()[4],
    ",
  0x4001fd1au64 => "
      PFS.psar()[5],
    ",
  0x4001fd1cu64 => "
      PFS.psar()[6],
    ",
  0x4001fd1eu64 => "
      PFS.psar()[7],
    ",
  0x4001fd20u64 => "
      PFS.psar()[8],
    ",
  0x4001fd40u64 => "
      PFS.psar(),
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
  0x40082024u64 => "
      ELC.elsr()[5],
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
  0x40082048u64 => "
      ELC.elsr()[0],
    ",
  0x4008204cu64 => "
      ELC.elsr()[1],
    ",
  0x40082050u64 => "
      ELC.elsr()[2],
    ",
  0x40082054u64 => "
      ELC.elsr()[3],
    ",
  0x40082058u64 => "
      ELC.elsr()[4],
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
  0x40083044u64 => "
      RTC.rtccr()[2],
    ",
  0x40083052u64 => "
      RTC.bcnt0cp()[0],
      RTC.rseccp()[0],
    ",
  0x40083062u64 => "
      RTC.bcnt0cp()[1],
      RTC.rseccp()[1],
    ",
  0x40083072u64 => "
      RTC.bcnt0cp()[2],
      RTC.rseccp()[2],
    ",
  0x40083054u64 => "
      RTC.bcnt1cp()[0],
      RTC.rmincp()[0],
    ",
  0x40083064u64 => "
      RTC.bcnt1cp()[1],
      RTC.rmincp()[1],
    ",
  0x40083074u64 => "
      RTC.bcnt1cp()[2],
      RTC.rmincp()[2],
    ",
  0x40083056u64 => "
      RTC.bcnt2cp()[0],
      RTC.rhrcp()[0],
    ",
  0x40083066u64 => "
      RTC.bcnt2cp()[1],
      RTC.rhrcp()[1],
    ",
  0x40083076u64 => "
      RTC.bcnt2cp()[2],
      RTC.rhrcp()[2],
    ",
  0x4008305au64 => "
      RTC.bcnt3cp()[0],
      RTC.rdaycp()[0],
    ",
  0x4008306au64 => "
      RTC.bcnt3cp()[1],
      RTC.rdaycp()[1],
    ",
  0x4008307au64 => "
      RTC.bcnt3cp()[2],
      RTC.rdaycp()[2],
    ",
  0x4008305cu64 => "
      RTC.rmoncp()[0],
    ",
  0x4008306cu64 => "
      RTC.rmoncp()[1],
    ",
  0x4008307cu64 => "
      RTC.rmoncp()[2],
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
  0x40090018u64 => "
      USBFS.dfifo()[0],
      USBFS.dfifol()[0],
    ",
  0x4009001cu64 => "
      USBFS.dfifo()[1],
      USBFS.dfifol()[1],
    ",
  0x40090020u64 => "
      USBFS.cfifosel(),
    ",
  0x40090022u64 => "
      USBFS.cfifoctr(),
    ",
  0x40090028u64 => "
      USBFS.dfifosel()[0],
    ",
  0x4009002cu64 => "
      USBFS.dfifosel()[1],
    ",
  0x4009002au64 => "
      USBFS.dfifoctr()[0],
    ",
  0x4009002eu64 => "
      USBFS.dfifoctr()[1],
    ",
  0x40090030u64 => "
      USBFS.intenb0(),
    ",
  0x40090032u64 => "
      USBFS.intenb1(),
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
  0x40090042u64 => "
      USBFS.intsts1(),
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
  0x4009006eu64 => "
      USBFS.pipeperi(),
    ",
  0x40090078u64 => "
      USBFS.pipectr()[4],
    ",
  0x4009007au64 => "
      USBFS.pipectr()[0],
    ",
  0x4009007cu64 => "
      USBFS.pipectr()[1],
    ",
  0x4009007eu64 => "
      USBFS.pipectr()[2],
    ",
  0x40090080u64 => "
      USBFS.pipectr()[3],
    ",
  0x40090090u64 => "
      USBFS.pipetre()[0],
    ",
  0x40090094u64 => "
      USBFS.pipetre()[1],
    ",
  0x40090098u64 => "
      USBFS.pipetre()[2],
    ",
  0x4009009cu64 => "
      USBFS.pipetre()[3],
    ",
  0x400900a0u64 => "
      USBFS.pipetre()[4],
    ",
  0x40090092u64 => "
      USBFS.pipetrn()[0],
    ",
  0x40090096u64 => "
      USBFS.pipetrn()[1],
    ",
  0x4009009au64 => "
      USBFS.pipetrn()[2],
    ",
  0x4009009eu64 => "
      USBFS.pipetrn()[3],
    ",
  0x400900a2u64 => "
      USBFS.pipetrn()[4],
    ",
  0x400900d0u64 => "
      USBFS.devadd()[0],
    ",
  0x400900d2u64 => "
      USBFS.devadd()[1],
    ",
  0x400900d4u64 => "
      USBFS.devadd()[2],
    ",
  0x400900d6u64 => "
      USBFS.devadd()[3],
    ",
  0x400900d8u64 => "
      USBFS.devadd()[4],
    ",
  0x400900dau64 => "
      USBFS.devadd()[5],
    ",
  0x40097000u64 => "
      UARTA.txba()[0],
    ",
  0x40097008u64 => "
      UARTA.txba()[1],
    ",
  0x40097001u64 => "
      UARTA.rxba()[0],
    ",
  0x40097009u64 => "
      UARTA.rxba()[1],
    ",
  0x40097002u64 => "
      UARTA.asima0()[0],
    ",
  0x4009700au64 => "
      UARTA.asima0()[1],
    ",
  0x40097003u64 => "
      UARTA.asima1()[0],
    ",
  0x4009700bu64 => "
      UARTA.asima1()[1],
    ",
  0x40097004u64 => "
      UARTA.brgca()[0],
    ",
  0x4009700cu64 => "
      UARTA.brgca()[1],
    ",
  0x40097005u64 => "
      UARTA.asisa()[0],
    ",
  0x4009700du64 => "
      UARTA.asisa()[1],
    ",
  0x40097006u64 => "
      UARTA.ascta()[0],
    ",
  0x4009700eu64 => "
      UARTA.ascta()[1],
    ",
  0x40097010u64 => "
      UARTA.uta0ck(),
    ",
  0x40097011u64 => "
      UARTA.uta1ck(),
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
  0x4009f000u64 => "
      IIC_0.iccr1(),
    ",
  0x4009f001u64 => "
      IIC_0.iccr2(),
    ",
  0x4009f002u64 => "
      IIC_0.icmr1(),
    ",
  0x4009f003u64 => "
      IIC_0.icmr2(),
    ",
  0x4009f004u64 => "
      IIC_0.icmr3(),
    ",
  0x4009f005u64 => "
      IIC_0.icfer(),
    ",
  0x4009f006u64 => "
      IIC_0.icser(),
    ",
  0x4009f007u64 => "
      IIC_0.icier(),
    ",
  0x4009f008u64 => "
      IIC_0.icsr1(),
    ",
  0x4009f009u64 => "
      IIC_0.icsr2(),
    ",
  0x4009f00au64 => "
      IIC_0.sarl()[0],
    ",
  0x4009f00cu64 => "
      IIC_0.sarl()[1],
    ",
  0x4009f00eu64 => "
      IIC_0.sarl()[2],
    ",
  0x4009f00bu64 => "
      IIC_0.saru()[0],
    ",
  0x4009f00du64 => "
      IIC_0.saru()[1],
    ",
  0x4009f00fu64 => "
      IIC_0.saru()[2],
    ",
  0x4009f010u64 => "
      IIC_0.icbrl(),
    ",
  0x4009f011u64 => "
      IIC_0.icbrh(),
    ",
  0x4009f012u64 => "
      IIC_0.icdrt(),
    ",
  0x4009f013u64 => "
      IIC_0.icdrr(),
    ",
  0x4009f016u64 => "
      IIC_0_WU.icwur(),
    ",
  0x4009f017u64 => "
      IIC_0_WU.icwur2(),
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
  0x400b0d20u64 => "
      CANFD.cfdrmid()[0],
    ",
  0x400b0d6cu64 => "
      CANFD.cfdrmid()[1],
    ",
  0x400b0db8u64 => "
      CANFD.cfdrmid()[2],
    ",
  0x400b0e04u64 => "
      CANFD.cfdrmid()[3],
    ",
  0x400b0e50u64 => "
      CANFD.cfdrmid()[4],
    ",
  0x400b0e9cu64 => "
      CANFD.cfdrmid()[5],
    ",
  0x400b0ee8u64 => "
      CANFD.cfdrmid()[6],
    ",
  0x400b0f34u64 => "
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
  0x400d0000u64 => "
      CTSU.ctsucra(),
      CTSU.ctsucral(),
      CTSU.ctsucr0(),
    ",
  0x400d0001u64 => "
      CTSU.ctsucr1(),
    ",
  0x400d0002u64 => "
      CTSU.ctsucr2(),
    ",
  0x400d0003u64 => "
      CTSU.ctsucr3(),
    ",
  0x400d0004u64 => "
      CTSU.ctsucrb(),
      CTSU.ctsucrbl(),
      CTSU.ctsusdprs(),
    ",
  0x400d0005u64 => "
      CTSU.ctsusst(),
    ",
  0x400d0006u64 => "
      CTSU.ctsucrbh(),
    ",
  0x400d0007u64 => "
      CTSU.ctsudclkc(),
    ",
  0x400d0008u64 => "
      CTSU.ctsumch(),
      CTSU.ctsumchl(),
      CTSU.ctsumch0(),
    ",
  0x400d0009u64 => "
      CTSU.ctsumch1(),
    ",
  0x400d000au64 => "
      CTSU.ctsumchh(),
      CTSU.ctsumfaf(),
    ",
  0x400d000cu64 => "
      CTSU.ctsuchaca(),
      CTSU.ctsuchacal(),
      CTSU.ctsuchac0(),
    ",
  0x400d000du64 => "
      CTSU.ctsuchac1(),
    ",
  0x400d0014u64 => "
      CTSU.ctsuchtrca(),
      CTSU.ctsuchtrcal(),
      CTSU.ctsuchtrc0(),
    ",
  0x400d0015u64 => "
      CTSU.ctsuchtrc1(),
    ",
  0x400d001cu64 => "
      CTSU.ctsusr(),
      CTSU.ctsusrl(),
      CTSU.ctsusr0(),
    ",
  0x400d001du64 => "
      CTSU.ctsust(),
    ",
  0x400d001eu64 => "
      CTSU.ctsusrh(),
      CTSU.ctsusr2(),
    ",
  0x400d0020u64 => "
      CTSU.ctsuso(),
      CTSU.ctsuso0(),
    ",
  0x400d0022u64 => "
      CTSU.ctsuso1(),
    ",
  0x400d0024u64 => "
      CTSU.ctsuscnt(),
      CTSU.ctsusc(),
    ",
  0x400d0028u64 => "
      CTSU.ctsucalib(),
      CTSU.ctsudbgr0(),
    ",
  0x400d002au64 => "
      CTSU.ctsudbgr1(),
    ",
  0x400d002cu64 => "
      CTSU.ctsusuclka(),
      CTSU.ctsusuclk0(),
    ",
  0x400d002eu64 => "
      CTSU.ctsusuclk1(),
    ",
  0x400d0030u64 => "
      CTSU.ctsusuclkb(),
      CTSU.ctsusuclk2(),
    ",
  0x400d0032u64 => "
      CTSU.ctsusuclk3(),
    ",
  0x400d0040u64 => "
      CTSU.ctsuopt(),
      CTSU.ctsuoptl(),
      CTSU.ac(),
    ",
  0x400d0041u64 => "
      CTSU.aj(),
    ",
  0x400d0042u64 => "
      CTSU.ctsuopth(),
      CTSU.acsb(),
    ",
  0x400d0044u64 => "
      CTSU.ctsuscntact(),
      CTSU.ctsuscntactl(),
    ",
  0x400d0046u64 => "
      CTSU.ctsuscntacth(),
    ",
  0x400d004cu64 => "
      CTSU.ctsumact1(),
      CTSU.ctsumact1l(),
    ",
  0x400d004eu64 => "
      CTSU.ctsumact1h(),
    ",
  0x400d0050u64 => "
      CTSU.ctsumact2(),
      CTSU.ctsumact2l(),
    ",
  0x400d0052u64 => "
      CTSU.ctsumact2h(),
    ",
  0x400d0054u64 => "
      CTSU.ctsumact3(),
      CTSU.ctsumact3l(),
    ",
  0x400d0056u64 => "
      CTSU.ctsumact3h(),
    ",
  0x400d0058u64 => "
      CTSU.ctsuajcr(),
      CTSU.ctsuajcrl(),
      CTSU.ajcr0(),
    ",
  0x400d0059u64 => "
      CTSU.ajcr1(),
    ",
  0x400d005au64 => "
      CTSU.ctsuajcrh(),
      CTSU.ajcr2(),
    ",
  0x400d005bu64 => "
      CTSU.ajcr3(),
    ",
  0x400d005cu64 => "
      CTSU.ctsuajthr(),
      CTSU.ctsuajthrl(),
    ",
  0x400d005eu64 => "
      CTSU.ctsuajthrh(),
    ",
  0x400d0060u64 => "
      CTSU.ctsuajmmar(),
      CTSU.ctsuajmmarl(),
    ",
  0x400d0062u64 => "
      CTSU.ctsuajmmarh(),
    ",
  0x400d0064u64 => "
      CTSU.ctsuajblact(),
      CTSU.ctsuajblactl(),
    ",
  0x400d0066u64 => "
      CTSU.ctsuajblacth(),
    ",
  0x400d0068u64 => "
      CTSU.ctsuajblar(),
      CTSU.ctsuajblarl(),
    ",
  0x400d006au64 => "
      CTSU.ctsuajblarh(),
    ",
  0x400d006cu64 => "
      CTSU.ctsuajrr(),
      CTSU.ctsuajrrl(),
      CTSU.ctsuajrr0(),
    ",
  0x400d006du64 => "
      CTSU.ctsuajrr1(),
    ",
  0x400d4000u64 => "
      SLCDC.lcdm0(),
    ",
  0x400d4001u64 => "
      SLCDC.lcdm1(),
    ",
  0x400d4002u64 => "
      SLCDC.lcdc0(),
    ",
  0x400d4003u64 => "
      SLCDC.vlcd(),
    ",
  0x400d4100u64 => "
      SLCDC.seg()[0],
    ",
  0x400d4101u64 => "
      SLCDC.seg()[1],
    ",
  0x400d4102u64 => "
      SLCDC.seg()[2],
    ",
  0x400d4103u64 => "
      SLCDC.seg()[3],
    ",
  0x400d4104u64 => "
      SLCDC.seg()[4],
    ",
  0x400d4105u64 => "
      SLCDC.seg()[5],
    ",
  0x400d4106u64 => "
      SLCDC.seg()[6],
    ",
  0x400d4107u64 => "
      SLCDC.seg()[7],
    ",
  0x400d4108u64 => "
      SLCDC.seg()[8],
    ",
  0x400d4109u64 => "
      SLCDC.seg()[9],
    ",
  0x400d410au64 => "
      SLCDC.seg()[10],
    ",
  0x400d410bu64 => "
      SLCDC.seg()[11],
    ",
  0x400d410cu64 => "
      SLCDC.seg()[12],
    ",
  0x400d410du64 => "
      SLCDC.seg()[13],
    ",
  0x400d410eu64 => "
      SLCDC.seg()[14],
    ",
  0x400d410fu64 => "
      SLCDC.seg()[15],
    ",
  0x400d4110u64 => "
      SLCDC.seg()[16],
    ",
  0x400d4111u64 => "
      SLCDC.seg()[17],
    ",
  0x400d4112u64 => "
      SLCDC.seg()[18],
    ",
  0x400d4113u64 => "
      SLCDC.seg()[19],
    ",
  0x400d4114u64 => "
      SLCDC.seg()[20],
    ",
  0x400d4115u64 => "
      SLCDC.seg()[21],
    ",
  0x400d4116u64 => "
      SLCDC.seg()[22],
    ",
  0x400d4117u64 => "
      SLCDC.seg()[23],
    ",
  0x400d4118u64 => "
      SLCDC.seg()[24],
    ",
  0x400d4119u64 => "
      SLCDC.seg()[25],
    ",
  0x400d411au64 => "
      SLCDC.seg()[26],
    ",
  0x400d411bu64 => "
      SLCDC.seg()[27],
    ",
  0x400d411cu64 => "
      SLCDC.seg()[28],
    ",
  0x400d411du64 => "
      SLCDC.seg()[29],
    ",
  0x400d411eu64 => "
      SLCDC.seg()[30],
    ",
  0x400d411fu64 => "
      SLCDC.seg()[31],
    ",
  0x400d4120u64 => "
      SLCDC.seg()[32],
    ",
  0x400d4121u64 => "
      SLCDC.seg()[33],
    ",
  0x400d4122u64 => "
      SLCDC.seg()[34],
    ",
  0x400d4123u64 => "
      SLCDC.seg()[35],
    ",
  0x400d4124u64 => "
      SLCDC.seg()[36],
    ",
  0x400d4125u64 => "
      SLCDC.seg()[37],
    ",
  0x400d4126u64 => "
      SLCDC.seg()[38],
    ",
  0x400d4127u64 => "
      SLCDC.seg()[39],
    ",
  0x400d4128u64 => "
      SLCDC.seg()[40],
    ",
  0x400d4129u64 => "
      SLCDC.seg()[41],
    ",
  0x400d412au64 => "
      SLCDC.seg()[42],
    ",
  0x400d412bu64 => "
      SLCDC.seg()[43],
    ",
  0x400d412cu64 => "
      SLCDC.seg()[44],
    ",
  0x400d412du64 => "
      SLCDC.seg()[45],
    ",
  0x400d412eu64 => "
      SLCDC.seg()[46],
    ",
  0x400d412fu64 => "
      SLCDC.seg()[47],
    ",
  0x400d4130u64 => "
      SLCDC.seg()[48],
    ",
  0x400d4131u64 => "
      SLCDC.seg()[49],
    ",
  0x400d4132u64 => "
      SLCDC.seg()[50],
    ",
  0x400d4133u64 => "
      SLCDC.seg()[51],
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
  0x400f4000u64 => "
      ACMPLP.compmdr(),
    ",
  0x400f4001u64 => "
      ACMPLP.compfir(),
    ",
  0x400f4002u64 => "
      ACMPLP.compocr(),
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
      SCI_0.ftdrh(),
    ",
  0x4011800fu64 => "
      SCI_0.ftdrl(),
    ",
  0x40118010u64 => "
      SCI_0.frdrhl(),
      SCI_0.rdrhl(),
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
  0x40118100u64 => "
      SCI_1.smr(),
      SCI_1.smr_smci(),
    ",
  0x40118101u64 => "
      SCI_1.brr(),
    ",
  0x40118102u64 => "
      SCI_1.scr(),
      SCI_1.scr_smci(),
    ",
  0x40118103u64 => "
      SCI_1.tdr(),
    ",
  0x40118104u64 => "
      SCI_1.ssr(),
      SCI_1.ssr_smci(),
    ",
  0x40118105u64 => "
      SCI_1.rdr(),
    ",
  0x40118106u64 => "
      SCI_1.scmr(),
    ",
  0x40118107u64 => "
      SCI_1.semr(),
    ",
  0x40118108u64 => "
      SCI_1.snfr(),
    ",
  0x40118109u64 => "
      SCI_1.simr1(),
    ",
  0x4011810au64 => "
      SCI_1.simr2(),
    ",
  0x4011810bu64 => "
      SCI_1.simr3(),
    ",
  0x4011810cu64 => "
      SCI_1.sisr(),
    ",
  0x4011810du64 => "
      SCI_1.spmr(),
    ",
  0x4011810eu64 => "
      SCI_1.tdrhl(),
    ",
  0x40118110u64 => "
      SCI_1.rdrhl(),
    ",
  0x40118112u64 => "
      SCI_1.mddr(),
    ",
  0x40118120u64 => "
      SCI_1.esmer(),
    ",
  0x40118121u64 => "
      SCI_1.cr0(),
    ",
  0x40118122u64 => "
      SCI_1.cr1(),
    ",
  0x40118123u64 => "
      SCI_1.cr2(),
    ",
  0x40118124u64 => "
      SCI_1.cr3(),
    ",
  0x40118125u64 => "
      SCI_1.pcr(),
    ",
  0x40118126u64 => "
      SCI_1.icr(),
    ",
  0x40118127u64 => "
      SCI_1.str(),
    ",
  0x40118128u64 => "
      SCI_1.stcr(),
    ",
  0x40118129u64 => "
      SCI_1.cf0dr(),
    ",
  0x4011812au64 => "
      SCI_1.cf0cr(),
    ",
  0x4011812bu64 => "
      SCI_1.cf0rr(),
    ",
  0x4011812cu64 => "
      SCI_1.pcf1dr(),
    ",
  0x4011812du64 => "
      SCI_1.scf1dr(),
    ",
  0x4011812eu64 => "
      SCI_1.cf1cr(),
    ",
  0x4011812fu64 => "
      SCI_1.cf1rr(),
    ",
  0x40118130u64 => "
      SCI_1.tcr(),
    ",
  0x40118131u64 => "
      SCI_1.tmr(),
    ",
  0x40118132u64 => "
      SCI_1.tpre(),
    ",
  0x40118133u64 => "
      SCI_1.tcnt(),
    ",
  0x40118300u64 => "
      SCI_3.smr(),
      SCI_3.smr_smci(),
    ",
  0x40118301u64 => "
      SCI_3.brr(),
    ",
  0x40118302u64 => "
      SCI_3.scr(),
      SCI_3.scr_smci(),
    ",
  0x40118303u64 => "
      SCI_3.tdr(),
    ",
  0x40118304u64 => "
      SCI_3.ssr(),
      SCI_3.ssr_fifo(),
      SCI_3.ssr_manc(),
      SCI_3.ssr_smci(),
    ",
  0x40118305u64 => "
      SCI_3.rdr(),
    ",
  0x40118306u64 => "
      SCI_3.scmr(),
    ",
  0x40118307u64 => "
      SCI_3.semr(),
    ",
  0x40118308u64 => "
      SCI_3.snfr(),
    ",
  0x40118309u64 => "
      SCI_3.simr1(),
    ",
  0x4011830au64 => "
      SCI_3.simr2(),
    ",
  0x4011830bu64 => "
      SCI_3.simr3(),
    ",
  0x4011830cu64 => "
      SCI_3.sisr(),
    ",
  0x4011830du64 => "
      SCI_3.spmr(),
    ",
  0x4011830eu64 => "
      SCI_3.ftdrhl(),
      SCI_3.tdrhl(),
      SCI_3.tdrhl_man(),
      SCI_3.ftdrh(),
    ",
  0x4011830fu64 => "
      SCI_3.ftdrl(),
    ",
  0x40118310u64 => "
      SCI_3.frdrhl(),
      SCI_3.rdrhl(),
      SCI_3.rdrhl_man(),
      SCI_3.frdrh(),
    ",
  0x40118311u64 => "
      SCI_3.frdrl(),
    ",
  0x40118312u64 => "
      SCI_3.mddr(),
    ",
  0x40118313u64 => "
      SCI_3.dccr(),
    ",
  0x40118314u64 => "
      SCI_3.fcr(),
    ",
  0x40118316u64 => "
      SCI_3.fdr(),
    ",
  0x40118318u64 => "
      SCI_3.lsr(),
    ",
  0x4011831au64 => "
      SCI_3.cdr(),
    ",
  0x4011831cu64 => "
      SCI_3.sptr(),
    ",
  0x4011831du64 => "
      SCI_3.actr(),
    ",
  0x40118320u64 => "
      SCI_3.mmr(),
    ",
  0x40118322u64 => "
      SCI_3.tmpr(),
    ",
  0x40118323u64 => "
      SCI_3.rmpr(),
    ",
  0x40118324u64 => "
      SCI_3.mesr(),
    ",
  0x40118325u64 => "
      SCI_3.mecr(),
    ",
  0x40118f00u64 => "
      SCI_5.ircr(),
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
  0x4011f190u64 => "
      I_3_C.nqthctl(),
    ",
  0x4011f194u64 => "
      I_3_C.ntbthctl0(),
    ",
  0x4011f1c0u64 => "
      I_3_C.nrqthctl(),
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
  0x4011f210u64 => "
      I_3_C.bcst(),
    ",
  0x4011f214u64 => "
      I_3_C.svst(),
    ",
  0x4011f218u64 => "
      I_3_C.wust(),
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
  0x4011f2a0u64 => "
      I_3_C.exdatbas(),
    ",
  0x4011f2b0u64 => "
      I_3_C.sdatbas0(),
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
      I_3_C.svdvad0(),
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
  0x4011f37cu64 => "
      I_3_C.cghdrcap(),
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
  0x4011f3ccu64 => "
      I_3_C.prstdbg(),
    ",
  0x4011f3d0u64 => "
      I_3_C.mserrcnt(),
    ",
  0x4012f000u64 => "
      ECCMB.ec710ctl(),
    ",
  0x4012f004u64 => "
      ECCMB.ec710tmc(),
    ",
  0x4012f00cu64 => "
      ECCMB.ec710ted(),
    ",
  0x4012f010u64 => "
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
  0x40169088u64 => "
      GPT_320.gtdtcr(),
    ",
  0x4016908cu64 => "
      GPT_320.gtdvu(),
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
  0x40169200u64 => "
      GPT_162.gtwp(),
    ",
  0x40169204u64 => "
      GPT_162.gtstr(),
    ",
  0x40169208u64 => "
      GPT_162.gtstp(),
    ",
  0x4016920cu64 => "
      GPT_162.gtclr(),
    ",
  0x40169210u64 => "
      GPT_162.gtssr(),
    ",
  0x40169214u64 => "
      GPT_162.gtpsr(),
    ",
  0x40169218u64 => "
      GPT_162.gtcsr(),
    ",
  0x4016921cu64 => "
      GPT_162.gtupsr(),
    ",
  0x40169220u64 => "
      GPT_162.gtdnsr(),
    ",
  0x40169224u64 => "
      GPT_162.gticasr(),
    ",
  0x40169228u64 => "
      GPT_162.gticbsr(),
    ",
  0x4016922cu64 => "
      GPT_162.gtcr(),
    ",
  0x40169230u64 => "
      GPT_162.gtuddtyc(),
    ",
  0x40169234u64 => "
      GPT_162.gtior(),
    ",
  0x40169238u64 => "
      GPT_162.gtintad(),
    ",
  0x4016923cu64 => "
      GPT_162.gtst(),
    ",
  0x40169240u64 => "
      GPT_162.gtber(),
    ",
  0x40169248u64 => "
      GPT_162.gtcnt(),
    ",
  0x4016924cu64 => "
      GPT_162.gtccra(),
    ",
  0x40169250u64 => "
      GPT_162.gtccrb(),
    ",
  0x40169254u64 => "
      GPT_162.gtccrc(),
    ",
  0x40169258u64 => "
      GPT_162.gtccre(),
    ",
  0x4016925cu64 => "
      GPT_162.gtccrd(),
    ",
  0x40169260u64 => "
      GPT_162.gtccrf(),
    ",
  0x40169264u64 => "
      GPT_162.gtpr(),
    ",
  0x40169268u64 => "
      GPT_162.gtpbr(),
    ",
  0x40169288u64 => "
      GPT_162.gtdtcr(),
    ",
  0x4016928cu64 => "
      GPT_162.gtdvu(),
    ",
  0x401692b8u64 => "
      GPT_162.gticlf(),
    ",
  0x401692bcu64 => "
      GPT_162.gtpc(),
    ",
  0x401692d0u64 => "
      GPT_162.gtsecsr(),
    ",
  0x401692d4u64 => "
      GPT_162.gtsecr(),
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
  0x40170040u64 => "
      ADC_120.adctdr(),
    ",
  0x40170042u64 => "
      ADC_120.addr()[0],
    ",
  0x40170044u64 => "
      ADC_120.addr()[1],
    ",
  0x40170046u64 => "
      ADC_120.addr()[2],
    ",
  0x40170048u64 => "
      ADC_120.addr()[3],
    ",
  0x4017004au64 => "
      ADC_120.addr()[4],
    ",
  0x4017004cu64 => "
      ADC_120.addr()[5],
    ",
  0x4017004eu64 => "
      ADC_120.addr()[6],
    ",
  0x40170050u64 => "
      ADC_120.addr()[7],
    ",
  0x40170052u64 => "
      ADC_120.addr()[8],
    ",
  0x4017007au64 => "
      ADC_120.addiscr(),
    ",
  0x4017007eu64 => "
      ADC_120.adacsr(),
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
  0x4017008au64 => "
      ADC_120.adhvrefcnt(),
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
  0x401700ddu64 => "
      ADC_120.adsstrl(),
    ",
  0x401700deu64 => "
      ADC_120.adsstrt(),
    ",
  0x401700dfu64 => "
      ADC_120.adsstro(),
    ",
  0x401700e0u64 => "
      ADC_120.adsstr()[0],
    ",
  0x401700e1u64 => "
      ADC_120.adsstr()[1],
    ",
  0x401700e2u64 => "
      ADC_120.adsstr()[2],
    ",
  0x401700e3u64 => "
      ADC_120.adsstr()[3],
    ",
  0x401700e4u64 => "
      ADC_120.adsstr()[4],
    ",
  0x401700e5u64 => "
      ADC_120.adsstr()[5],
    ",
  0x401700e6u64 => "
      ADC_120.adsstr()[6],
    ",
  0x40171000u64 => "
      DAC_12.dadr0(),
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
  0x40171007u64 => "
      DAC_12.davrefcr(),
    ",
  0x407ec090u64 => "
      FLCN.dflctl(),
    ",
  0x407ec228u64 => "
      FLCN.tscdr(),
    ",
  0x407effc4u64 => "
      FLCN.fldwaitr(),
    ",
  0x407effc8u64 => "
      FLCN.pfber(),
    ",
  0x407ec3a4u64 => "
      CIBC.ctsutrima(),
    ",
  0x407ec3a8u64 => "
      CIBC.ctsutrimb(),
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
      FACI.fbcaddr(),
    ",
  0x407fe0dcu64 => "
      FACI.fsuasmon(),
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
  0x80000400u64 => "
      CPU_OCD.mcustat(),
    ",
  0x80000410u64 => "
      CPU_OCD.mcuctrl(),
    ",
};
