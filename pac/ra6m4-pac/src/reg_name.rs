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
// Generated from SVD 1.50.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:51:51 +0000

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
  0x40000500u64 => "
      RMPU.mmpuenedmac(),
    ",
  0x40000504u64 => "
      RMPU.mmpuenptedmac(),
    ",
  0x40000508u64 => "
      RMPU.mmpurptedmac(),
    ",
  0x40000600u64 => "
      RMPU.mmpuacedmac()[0],
    ",
  0x40000610u64 => "
      RMPU.mmpuacedmac()[1],
    ",
  0x40000620u64 => "
      RMPU.mmpuacedmac()[2],
    ",
  0x40000630u64 => "
      RMPU.mmpuacedmac()[3],
    ",
  0x40000604u64 => "
      RMPU.mmpusedmac()[0],
    ",
  0x40000614u64 => "
      RMPU.mmpusedmac()[1],
    ",
  0x40000624u64 => "
      RMPU.mmpusedmac()[2],
    ",
  0x40000634u64 => "
      RMPU.mmpusedmac()[3],
    ",
  0x40000608u64 => "
      RMPU.mmpueedmac()[0],
    ",
  0x40000618u64 => "
      RMPU.mmpueedmac()[1],
    ",
  0x40000628u64 => "
      RMPU.mmpueedmac()[2],
    ",
  0x40000638u64 => "
      RMPU.mmpueedmac()[3],
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
  0x40003002u64 => "
      BUS.csmod()[0],
    ",
  0x40003012u64 => "
      BUS.csmod()[1],
    ",
  0x40003022u64 => "
      BUS.csmod()[2],
    ",
  0x40003032u64 => "
      BUS.csmod()[3],
    ",
  0x40003042u64 => "
      BUS.csmod()[4],
    ",
  0x40003052u64 => "
      BUS.csmod()[5],
    ",
  0x40003062u64 => "
      BUS.csmod()[6],
    ",
  0x40003072u64 => "
      BUS.csmod()[7],
    ",
  0x40003004u64 => "
      BUS.cswcr1()[0],
    ",
  0x40003014u64 => "
      BUS.cswcr1()[1],
    ",
  0x40003024u64 => "
      BUS.cswcr1()[2],
    ",
  0x40003034u64 => "
      BUS.cswcr1()[3],
    ",
  0x40003044u64 => "
      BUS.cswcr1()[4],
    ",
  0x40003054u64 => "
      BUS.cswcr1()[5],
    ",
  0x40003064u64 => "
      BUS.cswcr1()[6],
    ",
  0x40003074u64 => "
      BUS.cswcr1()[7],
    ",
  0x40003008u64 => "
      BUS.cswcr2()[0],
    ",
  0x40003018u64 => "
      BUS.cswcr2()[1],
    ",
  0x40003028u64 => "
      BUS.cswcr2()[2],
    ",
  0x40003038u64 => "
      BUS.cswcr2()[3],
    ",
  0x40003048u64 => "
      BUS.cswcr2()[4],
    ",
  0x40003058u64 => "
      BUS.cswcr2()[5],
    ",
  0x40003068u64 => "
      BUS.cswcr2()[6],
    ",
  0x40003078u64 => "
      BUS.cswcr2()[7],
    ",
  0x40003802u64 => "
      BUS.cscr()[0],
    ",
  0x40003812u64 => "
      BUS.cscr()[1],
    ",
  0x40003822u64 => "
      BUS.cscr()[2],
    ",
  0x40003832u64 => "
      BUS.cscr()[3],
    ",
  0x40003842u64 => "
      BUS.cscr()[4],
    ",
  0x40003852u64 => "
      BUS.cscr()[5],
    ",
  0x40003862u64 => "
      BUS.cscr()[6],
    ",
  0x40003872u64 => "
      BUS.cscr()[7],
    ",
  0x4000380au64 => "
      BUS.csrec()[0],
    ",
  0x4000381au64 => "
      BUS.csrec()[1],
    ",
  0x4000382au64 => "
      BUS.csrec()[2],
    ",
  0x4000383au64 => "
      BUS.csrec()[3],
    ",
  0x4000384au64 => "
      BUS.csrec()[4],
    ",
  0x4000385au64 => "
      BUS.csrec()[5],
    ",
  0x4000386au64 => "
      BUS.csrec()[6],
    ",
  0x4000387au64 => "
      BUS.csrec()[7],
    ",
  0x40003880u64 => "
      BUS.csrecen(),
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
  0x40004144u64 => "
      BUS.busscnteobiu(),
    ",
  0x40004148u64 => "
      BUS.busscntecbiu(),
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
  0x40004830u64 => "
      BUS.buserradd()[3],
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
  0x40004834u64 => "
      BUS.buserrrw()[3],
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
  0x40004930u64 => "
      BUS.btzferradd()[3],
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
  0x40004934u64 => "
      BUS.btzferrrw()[3],
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
  0x40004a30u64 => "
      BUS.buserrstat()[3],
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
  0x40004a38u64 => "
      BUS.buserrclr()[3],
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
  0x40005030u64 => "
      DMAC_0.dmbwr(),
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
  0x4001e030u64 => "
      SYSC.bckcr(),
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
  0x4001e052u64 => "
      SYSC.ebckocr(),
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
  0x4001e06du64 => "
      SYSC.octackdivcr(),
    ",
  0x4001e074u64 => "
      SYSC.usbckcr(),
    ",
  0x4001e075u64 => "
      SYSC.octackcr(),
    ",
  0x4001e088u64 => "
      SYSC.snzreqcr1(),
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
  0x4001e3d0u64 => "
      SYSC.bbfsar(),
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
  0x4001e41du64 => "
      SYSC.vbattmnselr(),
    ",
  0x4001e41eu64 => "
      SYSC.vbattmonr(),
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
  0x4001e4bbu64 => "
      SYSC.vbtictlr(),
    ",
  0x4001e4c0u64 => "
      SYSC.vbtber(),
    ",
  0x4001e500u64 => "
      SYSC.vbtbkr()[0],
    ",
  0x4001e501u64 => "
      SYSC.vbtbkr()[1],
    ",
  0x4001e502u64 => "
      SYSC.vbtbkr()[2],
    ",
  0x4001e503u64 => "
      SYSC.vbtbkr()[3],
    ",
  0x4001e504u64 => "
      SYSC.vbtbkr()[4],
    ",
  0x4001e505u64 => "
      SYSC.vbtbkr()[5],
    ",
  0x4001e506u64 => "
      SYSC.vbtbkr()[6],
    ",
  0x4001e507u64 => "
      SYSC.vbtbkr()[7],
    ",
  0x4001e508u64 => "
      SYSC.vbtbkr()[8],
    ",
  0x4001e509u64 => "
      SYSC.vbtbkr()[9],
    ",
  0x4001e50au64 => "
      SYSC.vbtbkr()[10],
    ",
  0x4001e50bu64 => "
      SYSC.vbtbkr()[11],
    ",
  0x4001e50cu64 => "
      SYSC.vbtbkr()[12],
    ",
  0x4001e50du64 => "
      SYSC.vbtbkr()[13],
    ",
  0x4001e50eu64 => "
      SYSC.vbtbkr()[14],
    ",
  0x4001e50fu64 => "
      SYSC.vbtbkr()[15],
    ",
  0x4001e510u64 => "
      SYSC.vbtbkr()[16],
    ",
  0x4001e511u64 => "
      SYSC.vbtbkr()[17],
    ",
  0x4001e512u64 => "
      SYSC.vbtbkr()[18],
    ",
  0x4001e513u64 => "
      SYSC.vbtbkr()[19],
    ",
  0x4001e514u64 => "
      SYSC.vbtbkr()[20],
    ",
  0x4001e515u64 => "
      SYSC.vbtbkr()[21],
    ",
  0x4001e516u64 => "
      SYSC.vbtbkr()[22],
    ",
  0x4001e517u64 => "
      SYSC.vbtbkr()[23],
    ",
  0x4001e518u64 => "
      SYSC.vbtbkr()[24],
    ",
  0x4001e519u64 => "
      SYSC.vbtbkr()[25],
    ",
  0x4001e51au64 => "
      SYSC.vbtbkr()[26],
    ",
  0x4001e51bu64 => "
      SYSC.vbtbkr()[27],
    ",
  0x4001e51cu64 => "
      SYSC.vbtbkr()[28],
    ",
  0x4001e51du64 => "
      SYSC.vbtbkr()[29],
    ",
  0x4001e51eu64 => "
      SYSC.vbtbkr()[30],
    ",
  0x4001e51fu64 => "
      SYSC.vbtbkr()[31],
    ",
  0x4001e520u64 => "
      SYSC.vbtbkr()[32],
    ",
  0x4001e521u64 => "
      SYSC.vbtbkr()[33],
    ",
  0x4001e522u64 => "
      SYSC.vbtbkr()[34],
    ",
  0x4001e523u64 => "
      SYSC.vbtbkr()[35],
    ",
  0x4001e524u64 => "
      SYSC.vbtbkr()[36],
    ",
  0x4001e525u64 => "
      SYSC.vbtbkr()[37],
    ",
  0x4001e526u64 => "
      SYSC.vbtbkr()[38],
    ",
  0x4001e527u64 => "
      SYSC.vbtbkr()[39],
    ",
  0x4001e528u64 => "
      SYSC.vbtbkr()[40],
    ",
  0x4001e529u64 => "
      SYSC.vbtbkr()[41],
    ",
  0x4001e52au64 => "
      SYSC.vbtbkr()[42],
    ",
  0x4001e52bu64 => "
      SYSC.vbtbkr()[43],
    ",
  0x4001e52cu64 => "
      SYSC.vbtbkr()[44],
    ",
  0x4001e52du64 => "
      SYSC.vbtbkr()[45],
    ",
  0x4001e52eu64 => "
      SYSC.vbtbkr()[46],
    ",
  0x4001e52fu64 => "
      SYSC.vbtbkr()[47],
    ",
  0x4001e530u64 => "
      SYSC.vbtbkr()[48],
    ",
  0x4001e531u64 => "
      SYSC.vbtbkr()[49],
    ",
  0x4001e532u64 => "
      SYSC.vbtbkr()[50],
    ",
  0x4001e533u64 => "
      SYSC.vbtbkr()[51],
    ",
  0x4001e534u64 => "
      SYSC.vbtbkr()[52],
    ",
  0x4001e535u64 => "
      SYSC.vbtbkr()[53],
    ",
  0x4001e536u64 => "
      SYSC.vbtbkr()[54],
    ",
  0x4001e537u64 => "
      SYSC.vbtbkr()[55],
    ",
  0x4001e538u64 => "
      SYSC.vbtbkr()[56],
    ",
  0x4001e539u64 => "
      SYSC.vbtbkr()[57],
    ",
  0x4001e53au64 => "
      SYSC.vbtbkr()[58],
    ",
  0x4001e53bu64 => "
      SYSC.vbtbkr()[59],
    ",
  0x4001e53cu64 => "
      SYSC.vbtbkr()[60],
    ",
  0x4001e53du64 => "
      SYSC.vbtbkr()[61],
    ",
  0x4001e53eu64 => "
      SYSC.vbtbkr()[62],
    ",
  0x4001e53fu64 => "
      SYSC.vbtbkr()[63],
    ",
  0x4001e540u64 => "
      SYSC.vbtbkr()[64],
    ",
  0x4001e541u64 => "
      SYSC.vbtbkr()[65],
    ",
  0x4001e542u64 => "
      SYSC.vbtbkr()[66],
    ",
  0x4001e543u64 => "
      SYSC.vbtbkr()[67],
    ",
  0x4001e544u64 => "
      SYSC.vbtbkr()[68],
    ",
  0x4001e545u64 => "
      SYSC.vbtbkr()[69],
    ",
  0x4001e546u64 => "
      SYSC.vbtbkr()[70],
    ",
  0x4001e547u64 => "
      SYSC.vbtbkr()[71],
    ",
  0x4001e548u64 => "
      SYSC.vbtbkr()[72],
    ",
  0x4001e549u64 => "
      SYSC.vbtbkr()[73],
    ",
  0x4001e54au64 => "
      SYSC.vbtbkr()[74],
    ",
  0x4001e54bu64 => "
      SYSC.vbtbkr()[75],
    ",
  0x4001e54cu64 => "
      SYSC.vbtbkr()[76],
    ",
  0x4001e54du64 => "
      SYSC.vbtbkr()[77],
    ",
  0x4001e54eu64 => "
      SYSC.vbtbkr()[78],
    ",
  0x4001e54fu64 => "
      SYSC.vbtbkr()[79],
    ",
  0x4001e550u64 => "
      SYSC.vbtbkr()[80],
    ",
  0x4001e551u64 => "
      SYSC.vbtbkr()[81],
    ",
  0x4001e552u64 => "
      SYSC.vbtbkr()[82],
    ",
  0x4001e553u64 => "
      SYSC.vbtbkr()[83],
    ",
  0x4001e554u64 => "
      SYSC.vbtbkr()[84],
    ",
  0x4001e555u64 => "
      SYSC.vbtbkr()[85],
    ",
  0x4001e556u64 => "
      SYSC.vbtbkr()[86],
    ",
  0x4001e557u64 => "
      SYSC.vbtbkr()[87],
    ",
  0x4001e558u64 => "
      SYSC.vbtbkr()[88],
    ",
  0x4001e559u64 => "
      SYSC.vbtbkr()[89],
    ",
  0x4001e55au64 => "
      SYSC.vbtbkr()[90],
    ",
  0x4001e55bu64 => "
      SYSC.vbtbkr()[91],
    ",
  0x4001e55cu64 => "
      SYSC.vbtbkr()[92],
    ",
  0x4001e55du64 => "
      SYSC.vbtbkr()[93],
    ",
  0x4001e55eu64 => "
      SYSC.vbtbkr()[94],
    ",
  0x4001e55fu64 => "
      SYSC.vbtbkr()[95],
    ",
  0x4001e560u64 => "
      SYSC.vbtbkr()[96],
    ",
  0x4001e561u64 => "
      SYSC.vbtbkr()[97],
    ",
  0x4001e562u64 => "
      SYSC.vbtbkr()[98],
    ",
  0x4001e563u64 => "
      SYSC.vbtbkr()[99],
    ",
  0x4001e564u64 => "
      SYSC.vbtbkr()[100],
    ",
  0x4001e565u64 => "
      SYSC.vbtbkr()[101],
    ",
  0x4001e566u64 => "
      SYSC.vbtbkr()[102],
    ",
  0x4001e567u64 => "
      SYSC.vbtbkr()[103],
    ",
  0x4001e568u64 => "
      SYSC.vbtbkr()[104],
    ",
  0x4001e569u64 => "
      SYSC.vbtbkr()[105],
    ",
  0x4001e56au64 => "
      SYSC.vbtbkr()[106],
    ",
  0x4001e56bu64 => "
      SYSC.vbtbkr()[107],
    ",
  0x4001e56cu64 => "
      SYSC.vbtbkr()[108],
    ",
  0x4001e56du64 => "
      SYSC.vbtbkr()[109],
    ",
  0x4001e56eu64 => "
      SYSC.vbtbkr()[110],
    ",
  0x4001e56fu64 => "
      SYSC.vbtbkr()[111],
    ",
  0x4001e570u64 => "
      SYSC.vbtbkr()[112],
    ",
  0x4001e571u64 => "
      SYSC.vbtbkr()[113],
    ",
  0x4001e572u64 => "
      SYSC.vbtbkr()[114],
    ",
  0x4001e573u64 => "
      SYSC.vbtbkr()[115],
    ",
  0x4001e574u64 => "
      SYSC.vbtbkr()[116],
    ",
  0x4001e575u64 => "
      SYSC.vbtbkr()[117],
    ",
  0x4001e576u64 => "
      SYSC.vbtbkr()[118],
    ",
  0x4001e577u64 => "
      SYSC.vbtbkr()[119],
    ",
  0x4001e578u64 => "
      SYSC.vbtbkr()[120],
    ",
  0x4001e579u64 => "
      SYSC.vbtbkr()[121],
    ",
  0x4001e57au64 => "
      SYSC.vbtbkr()[122],
    ",
  0x4001e57bu64 => "
      SYSC.vbtbkr()[123],
    ",
  0x4001e57cu64 => "
      SYSC.vbtbkr()[124],
    ",
  0x4001e57du64 => "
      SYSC.vbtbkr()[125],
    ",
  0x4001e57eu64 => "
      SYSC.vbtbkr()[126],
    ",
  0x4001e57fu64 => "
      SYSC.vbtbkr()[127],
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
  0x40080800u64 => "
      PFS.p00pfs()[0],
    ",
  0x40080804u64 => "
      PFS.p00pfs()[1],
    ",
  0x40080808u64 => "
      PFS.p00pfs()[2],
    ",
  0x4008080cu64 => "
      PFS.p00pfs()[3],
    ",
  0x40080810u64 => "
      PFS.p00pfs()[4],
    ",
  0x40080814u64 => "
      PFS.p00pfs()[5],
    ",
  0x40080818u64 => "
      PFS.p00pfs()[6],
    ",
  0x4008081cu64 => "
      PFS.p00pfs()[7],
    ",
  0x40080802u64 => "
      PFS.p00pfs_ha()[0],
    ",
  0x40080806u64 => "
      PFS.p00pfs_ha()[1],
    ",
  0x4008080au64 => "
      PFS.p00pfs_ha()[2],
    ",
  0x4008080eu64 => "
      PFS.p00pfs_ha()[3],
    ",
  0x40080812u64 => "
      PFS.p00pfs_ha()[4],
    ",
  0x40080816u64 => "
      PFS.p00pfs_ha()[5],
    ",
  0x4008081au64 => "
      PFS.p00pfs_ha()[6],
    ",
  0x4008081eu64 => "
      PFS.p00pfs_ha()[7],
    ",
  0x40080803u64 => "
      PFS.p00pfs_by()[0],
    ",
  0x40080807u64 => "
      PFS.p00pfs_by()[1],
    ",
  0x4008080bu64 => "
      PFS.p00pfs_by()[2],
    ",
  0x4008080fu64 => "
      PFS.p00pfs_by()[3],
    ",
  0x40080813u64 => "
      PFS.p00pfs_by()[4],
    ",
  0x40080817u64 => "
      PFS.p00pfs_by()[5],
    ",
  0x4008081bu64 => "
      PFS.p00pfs_by()[6],
    ",
  0x4008081fu64 => "
      PFS.p00pfs_by()[7],
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
  0x40080824u64 => "
      PFS.p009pfs(),
    ",
  0x40080826u64 => "
      PFS.p009pfs_ha(),
    ",
  0x40080827u64 => "
      PFS.p009pfs_by(),
    ",
  0x40080838u64 => "
      PFS.p0pfs()[0],
    ",
  0x4008083cu64 => "
      PFS.p0pfs()[1],
    ",
  0x4008083au64 => "
      PFS.p0pfs_ha()[0],
    ",
  0x4008083eu64 => "
      PFS.p0pfs_ha()[1],
    ",
  0x4008083bu64 => "
      PFS.p0pfs_by()[0],
    ",
  0x4008083fu64 => "
      PFS.p0pfs_by()[1],
    ",
  0x40080840u64 => "
      PFS.p10pfs()[0],
    ",
  0x40080844u64 => "
      PFS.p10pfs()[1],
    ",
  0x40080848u64 => "
      PFS.p10pfs()[2],
    ",
  0x4008084cu64 => "
      PFS.p10pfs()[3],
    ",
  0x40080850u64 => "
      PFS.p10pfs()[4],
    ",
  0x40080854u64 => "
      PFS.p10pfs()[5],
    ",
  0x40080858u64 => "
      PFS.p10pfs()[6],
    ",
  0x4008085cu64 => "
      PFS.p10pfs()[7],
    ",
  0x40080860u64 => "
      PFS.p10pfs()[8],
    ",
  0x40080864u64 => "
      PFS.p10pfs()[9],
    ",
  0x40080842u64 => "
      PFS.p10pfs_ha()[0],
    ",
  0x40080846u64 => "
      PFS.p10pfs_ha()[1],
    ",
  0x4008084au64 => "
      PFS.p10pfs_ha()[2],
    ",
  0x4008084eu64 => "
      PFS.p10pfs_ha()[3],
    ",
  0x40080852u64 => "
      PFS.p10pfs_ha()[4],
    ",
  0x40080856u64 => "
      PFS.p10pfs_ha()[5],
    ",
  0x4008085au64 => "
      PFS.p10pfs_ha()[6],
    ",
  0x4008085eu64 => "
      PFS.p10pfs_ha()[7],
    ",
  0x40080862u64 => "
      PFS.p10pfs_ha()[8],
    ",
  0x40080866u64 => "
      PFS.p10pfs_ha()[9],
    ",
  0x40080843u64 => "
      PFS.p10pfs_by()[0],
    ",
  0x40080847u64 => "
      PFS.p10pfs_by()[1],
    ",
  0x4008084bu64 => "
      PFS.p10pfs_by()[2],
    ",
  0x4008084fu64 => "
      PFS.p10pfs_by()[3],
    ",
  0x40080853u64 => "
      PFS.p10pfs_by()[4],
    ",
  0x40080857u64 => "
      PFS.p10pfs_by()[5],
    ",
  0x4008085bu64 => "
      PFS.p10pfs_by()[6],
    ",
  0x4008085fu64 => "
      PFS.p10pfs_by()[7],
    ",
  0x40080863u64 => "
      PFS.p10pfs_by()[8],
    ",
  0x40080867u64 => "
      PFS.p10pfs_by()[9],
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
  0x40080878u64 => "
      PFS.p1pfs()[4],
    ",
  0x4008087cu64 => "
      PFS.p1pfs()[5],
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
  0x4008087au64 => "
      PFS.p1pfs_ha()[4],
    ",
  0x4008087eu64 => "
      PFS.p1pfs_ha()[5],
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
  0x4008087bu64 => "
      PFS.p1pfs_by()[4],
    ",
  0x4008087fu64 => "
      PFS.p1pfs_by()[5],
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
  0x40080888u64 => "
      PFS.p20pfs()[0],
    ",
  0x4008088cu64 => "
      PFS.p20pfs()[1],
    ",
  0x40080890u64 => "
      PFS.p20pfs()[2],
    ",
  0x40080894u64 => "
      PFS.p20pfs()[3],
    ",
  0x40080898u64 => "
      PFS.p20pfs()[4],
    ",
  0x4008089cu64 => "
      PFS.p20pfs()[5],
    ",
  0x400808a0u64 => "
      PFS.p20pfs()[6],
    ",
  0x400808a4u64 => "
      PFS.p20pfs()[7],
    ",
  0x4008088au64 => "
      PFS.p20pfs_ha()[0],
    ",
  0x4008088eu64 => "
      PFS.p20pfs_ha()[1],
    ",
  0x40080892u64 => "
      PFS.p20pfs_ha()[2],
    ",
  0x40080896u64 => "
      PFS.p20pfs_ha()[3],
    ",
  0x4008089au64 => "
      PFS.p20pfs_ha()[4],
    ",
  0x4008089eu64 => "
      PFS.p20pfs_ha()[5],
    ",
  0x400808a2u64 => "
      PFS.p20pfs_ha()[6],
    ",
  0x400808a6u64 => "
      PFS.p20pfs_ha()[7],
    ",
  0x4008088bu64 => "
      PFS.p20pfs_by()[0],
    ",
  0x4008088fu64 => "
      PFS.p20pfs_by()[1],
    ",
  0x40080893u64 => "
      PFS.p20pfs_by()[2],
    ",
  0x40080897u64 => "
      PFS.p20pfs_by()[3],
    ",
  0x4008089bu64 => "
      PFS.p20pfs_by()[4],
    ",
  0x4008089fu64 => "
      PFS.p20pfs_by()[5],
    ",
  0x400808a3u64 => "
      PFS.p20pfs_by()[6],
    ",
  0x400808a7u64 => "
      PFS.p20pfs_by()[7],
    ",
  0x400808a8u64 => "
      PFS.p2pfs()[0],
    ",
  0x400808acu64 => "
      PFS.p2pfs()[1],
    ",
  0x400808b0u64 => "
      PFS.p2pfs()[2],
    ",
  0x400808b4u64 => "
      PFS.p2pfs()[3],
    ",
  0x400808b8u64 => "
      PFS.p2pfs()[4],
    ",
  0x400808aau64 => "
      PFS.p2pfs_ha()[0],
    ",
  0x400808aeu64 => "
      PFS.p2pfs_ha()[1],
    ",
  0x400808b2u64 => "
      PFS.p2pfs_ha()[2],
    ",
  0x400808b6u64 => "
      PFS.p2pfs_ha()[3],
    ",
  0x400808bau64 => "
      PFS.p2pfs_ha()[4],
    ",
  0x400808abu64 => "
      PFS.p2pfs_by()[0],
    ",
  0x400808afu64 => "
      PFS.p2pfs_by()[1],
    ",
  0x400808b3u64 => "
      PFS.p2pfs_by()[2],
    ",
  0x400808b7u64 => "
      PFS.p2pfs_by()[3],
    ",
  0x400808bbu64 => "
      PFS.p2pfs_by()[4],
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
  0x400808d4u64 => "
      PFS.p30pfs()[4],
    ",
  0x400808d8u64 => "
      PFS.p30pfs()[5],
    ",
  0x400808dcu64 => "
      PFS.p30pfs()[6],
    ",
  0x400808e0u64 => "
      PFS.p30pfs()[7],
    ",
  0x400808e4u64 => "
      PFS.p30pfs()[8],
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
  0x400808d6u64 => "
      PFS.p30pfs_ha()[4],
    ",
  0x400808dau64 => "
      PFS.p30pfs_ha()[5],
    ",
  0x400808deu64 => "
      PFS.p30pfs_ha()[6],
    ",
  0x400808e2u64 => "
      PFS.p30pfs_ha()[7],
    ",
  0x400808e6u64 => "
      PFS.p30pfs_ha()[8],
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
  0x400808d7u64 => "
      PFS.p30pfs_by()[4],
    ",
  0x400808dbu64 => "
      PFS.p30pfs_by()[5],
    ",
  0x400808dfu64 => "
      PFS.p30pfs_by()[6],
    ",
  0x400808e3u64 => "
      PFS.p30pfs_by()[7],
    ",
  0x400808e7u64 => "
      PFS.p30pfs_by()[8],
    ",
  0x400808e8u64 => "
      PFS.p3pfs()[0],
    ",
  0x400808ecu64 => "
      PFS.p3pfs()[1],
    ",
  0x400808f0u64 => "
      PFS.p3pfs()[2],
    ",
  0x400808f4u64 => "
      PFS.p3pfs()[3],
    ",
  0x400808eau64 => "
      PFS.p3pfs_ha()[0],
    ",
  0x400808eeu64 => "
      PFS.p3pfs_ha()[1],
    ",
  0x400808f2u64 => "
      PFS.p3pfs_ha()[2],
    ",
  0x400808f6u64 => "
      PFS.p3pfs_ha()[3],
    ",
  0x400808ebu64 => "
      PFS.p3pfs_by()[0],
    ",
  0x400808efu64 => "
      PFS.p3pfs_by()[1],
    ",
  0x400808f3u64 => "
      PFS.p3pfs_by()[2],
    ",
  0x400808f7u64 => "
      PFS.p3pfs_by()[3],
    ",
  0x40080900u64 => "
      PFS.p40pfs()[0],
    ",
  0x40080904u64 => "
      PFS.p40pfs()[1],
    ",
  0x40080908u64 => "
      PFS.p40pfs()[2],
    ",
  0x4008090cu64 => "
      PFS.p40pfs()[3],
    ",
  0x40080910u64 => "
      PFS.p40pfs()[4],
    ",
  0x40080914u64 => "
      PFS.p40pfs()[5],
    ",
  0x40080918u64 => "
      PFS.p40pfs()[6],
    ",
  0x4008091cu64 => "
      PFS.p40pfs()[7],
    ",
  0x40080920u64 => "
      PFS.p40pfs()[8],
    ",
  0x40080924u64 => "
      PFS.p40pfs()[9],
    ",
  0x40080902u64 => "
      PFS.p40pfs_ha()[0],
    ",
  0x40080906u64 => "
      PFS.p40pfs_ha()[1],
    ",
  0x4008090au64 => "
      PFS.p40pfs_ha()[2],
    ",
  0x4008090eu64 => "
      PFS.p40pfs_ha()[3],
    ",
  0x40080912u64 => "
      PFS.p40pfs_ha()[4],
    ",
  0x40080916u64 => "
      PFS.p40pfs_ha()[5],
    ",
  0x4008091au64 => "
      PFS.p40pfs_ha()[6],
    ",
  0x4008091eu64 => "
      PFS.p40pfs_ha()[7],
    ",
  0x40080922u64 => "
      PFS.p40pfs_ha()[8],
    ",
  0x40080926u64 => "
      PFS.p40pfs_ha()[9],
    ",
  0x40080903u64 => "
      PFS.p40pfs_by()[0],
    ",
  0x40080907u64 => "
      PFS.p40pfs_by()[1],
    ",
  0x4008090bu64 => "
      PFS.p40pfs_by()[2],
    ",
  0x4008090fu64 => "
      PFS.p40pfs_by()[3],
    ",
  0x40080913u64 => "
      PFS.p40pfs_by()[4],
    ",
  0x40080917u64 => "
      PFS.p40pfs_by()[5],
    ",
  0x4008091bu64 => "
      PFS.p40pfs_by()[6],
    ",
  0x4008091fu64 => "
      PFS.p40pfs_by()[7],
    ",
  0x40080923u64 => "
      PFS.p40pfs_by()[8],
    ",
  0x40080927u64 => "
      PFS.p40pfs_by()[9],
    ",
  0x40080928u64 => "
      PFS.p4pfs()[0],
    ",
  0x4008092cu64 => "
      PFS.p4pfs()[1],
    ",
  0x40080930u64 => "
      PFS.p4pfs()[2],
    ",
  0x40080934u64 => "
      PFS.p4pfs()[3],
    ",
  0x40080938u64 => "
      PFS.p4pfs()[4],
    ",
  0x4008093cu64 => "
      PFS.p4pfs()[5],
    ",
  0x4008092au64 => "
      PFS.p4pfs_ha()[0],
    ",
  0x4008092eu64 => "
      PFS.p4pfs_ha()[1],
    ",
  0x40080932u64 => "
      PFS.p4pfs_ha()[2],
    ",
  0x40080936u64 => "
      PFS.p4pfs_ha()[3],
    ",
  0x4008093au64 => "
      PFS.p4pfs_ha()[4],
    ",
  0x4008093eu64 => "
      PFS.p4pfs_ha()[5],
    ",
  0x4008092bu64 => "
      PFS.p4pfs_by()[0],
    ",
  0x4008092fu64 => "
      PFS.p4pfs_by()[1],
    ",
  0x40080933u64 => "
      PFS.p4pfs_by()[2],
    ",
  0x40080937u64 => "
      PFS.p4pfs_by()[3],
    ",
  0x4008093bu64 => "
      PFS.p4pfs_by()[4],
    ",
  0x4008093fu64 => "
      PFS.p4pfs_by()[5],
    ",
  0x40080940u64 => "
      PFS.p50pfs()[0],
    ",
  0x40080944u64 => "
      PFS.p50pfs()[1],
    ",
  0x40080948u64 => "
      PFS.p50pfs()[2],
    ",
  0x4008094cu64 => "
      PFS.p50pfs()[3],
    ",
  0x40080950u64 => "
      PFS.p50pfs()[4],
    ",
  0x40080954u64 => "
      PFS.p50pfs()[5],
    ",
  0x40080958u64 => "
      PFS.p50pfs()[6],
    ",
  0x4008095cu64 => "
      PFS.p50pfs()[7],
    ",
  0x40080942u64 => "
      PFS.p50pfs_ha()[0],
    ",
  0x40080946u64 => "
      PFS.p50pfs_ha()[1],
    ",
  0x4008094au64 => "
      PFS.p50pfs_ha()[2],
    ",
  0x4008094eu64 => "
      PFS.p50pfs_ha()[3],
    ",
  0x40080952u64 => "
      PFS.p50pfs_ha()[4],
    ",
  0x40080956u64 => "
      PFS.p50pfs_ha()[5],
    ",
  0x4008095au64 => "
      PFS.p50pfs_ha()[6],
    ",
  0x4008095eu64 => "
      PFS.p50pfs_ha()[7],
    ",
  0x40080943u64 => "
      PFS.p50pfs_by()[0],
    ",
  0x40080947u64 => "
      PFS.p50pfs_by()[1],
    ",
  0x4008094bu64 => "
      PFS.p50pfs_by()[2],
    ",
  0x4008094fu64 => "
      PFS.p50pfs_by()[3],
    ",
  0x40080953u64 => "
      PFS.p50pfs_by()[4],
    ",
  0x40080957u64 => "
      PFS.p50pfs_by()[5],
    ",
  0x4008095bu64 => "
      PFS.p50pfs_by()[6],
    ",
  0x4008095fu64 => "
      PFS.p50pfs_by()[7],
    ",
  0x4008096cu64 => "
      PFS.p5pfs()[0],
    ",
  0x40080970u64 => "
      PFS.p5pfs()[1],
    ",
  0x4008096eu64 => "
      PFS.p5pfs_ha()[0],
    ",
  0x40080972u64 => "
      PFS.p5pfs_ha()[1],
    ",
  0x4008096fu64 => "
      PFS.p5pfs_by()[0],
    ",
  0x40080973u64 => "
      PFS.p5pfs_by()[1],
    ",
  0x40080988u64 => "
      PFS.p60pfs()[2],
    ",
  0x4008098cu64 => "
      PFS.p60pfs()[3],
    ",
  0x40080990u64 => "
      PFS.p60pfs()[4],
    ",
  0x40080994u64 => "
      PFS.p60pfs()[5],
    ",
  0x4008098au64 => "
      PFS.p60pfs_ha()[2],
    ",
  0x4008098eu64 => "
      PFS.p60pfs_ha()[3],
    ",
  0x40080992u64 => "
      PFS.p60pfs_ha()[4],
    ",
  0x40080996u64 => "
      PFS.p60pfs_ha()[5],
    ",
  0x4008098bu64 => "
      PFS.p60pfs_by()[2],
    ",
  0x4008098fu64 => "
      PFS.p60pfs_by()[3],
    ",
  0x40080993u64 => "
      PFS.p60pfs_by()[4],
    ",
  0x40080997u64 => "
      PFS.p60pfs_by()[5],
    ",
  0x400809a0u64 => "
      PFS.p60pfs()[0],
    ",
  0x400809a4u64 => "
      PFS.p60pfs()[1],
    ",
  0x400809a2u64 => "
      PFS.p60pfs_ha()[0],
    ",
  0x400809a6u64 => "
      PFS.p60pfs_ha()[1],
    ",
  0x400809a3u64 => "
      PFS.p60pfs_by()[0],
    ",
  0x400809a7u64 => "
      PFS.p60pfs_by()[1],
    ",
  0x400809a8u64 => "
      PFS.p6pfs()[0],
    ",
  0x400809acu64 => "
      PFS.p6pfs()[1],
    ",
  0x400809b0u64 => "
      PFS.p6pfs()[2],
    ",
  0x400809b4u64 => "
      PFS.p6pfs()[3],
    ",
  0x400809b8u64 => "
      PFS.p6pfs()[4],
    ",
  0x400809aau64 => "
      PFS.p6pfs_ha()[0],
    ",
  0x400809aeu64 => "
      PFS.p6pfs_ha()[1],
    ",
  0x400809b2u64 => "
      PFS.p6pfs_ha()[2],
    ",
  0x400809b6u64 => "
      PFS.p6pfs_ha()[3],
    ",
  0x400809bau64 => "
      PFS.p6pfs_ha()[4],
    ",
  0x400809abu64 => "
      PFS.p6pfs_by()[0],
    ",
  0x400809afu64 => "
      PFS.p6pfs_by()[1],
    ",
  0x400809b3u64 => "
      PFS.p6pfs_by()[2],
    ",
  0x400809b7u64 => "
      PFS.p6pfs_by()[3],
    ",
  0x400809bbu64 => "
      PFS.p6pfs_by()[4],
    ",
  0x400809c8u64 => "
      PFS.p70pfs()[2],
    ",
  0x400809ccu64 => "
      PFS.p70pfs()[3],
    ",
  0x400809d0u64 => "
      PFS.p70pfs()[4],
    ",
  0x400809d4u64 => "
      PFS.p70pfs()[5],
    ",
  0x400809cau64 => "
      PFS.p70pfs_ha()[2],
    ",
  0x400809ceu64 => "
      PFS.p70pfs_ha()[3],
    ",
  0x400809d2u64 => "
      PFS.p70pfs_ha()[4],
    ",
  0x400809d6u64 => "
      PFS.p70pfs_ha()[5],
    ",
  0x400809cbu64 => "
      PFS.p70pfs_by()[2],
    ",
  0x400809cfu64 => "
      PFS.p70pfs_by()[3],
    ",
  0x400809d3u64 => "
      PFS.p70pfs_by()[4],
    ",
  0x400809d7u64 => "
      PFS.p70pfs_by()[5],
    ",
  0x400809e0u64 => "
      PFS.p70pfs()[0],
    ",
  0x400809e4u64 => "
      PFS.p70pfs()[1],
    ",
  0x400809e2u64 => "
      PFS.p70pfs_ha()[0],
    ",
  0x400809e6u64 => "
      PFS.p70pfs_ha()[1],
    ",
  0x400809e3u64 => "
      PFS.p70pfs_by()[0],
    ",
  0x400809e7u64 => "
      PFS.p70pfs_by()[1],
    ",
  0x400809e8u64 => "
      PFS.p7pfs()[0],
    ",
  0x400809ecu64 => "
      PFS.p7pfs()[1],
    ",
  0x400809f0u64 => "
      PFS.p7pfs()[2],
    ",
  0x400809f4u64 => "
      PFS.p7pfs()[3],
    ",
  0x400809eau64 => "
      PFS.p7pfs_ha()[0],
    ",
  0x400809eeu64 => "
      PFS.p7pfs_ha()[1],
    ",
  0x400809f2u64 => "
      PFS.p7pfs_ha()[2],
    ",
  0x400809f6u64 => "
      PFS.p7pfs_ha()[3],
    ",
  0x400809ebu64 => "
      PFS.p7pfs_by()[0],
    ",
  0x400809efu64 => "
      PFS.p7pfs_by()[1],
    ",
  0x400809f3u64 => "
      PFS.p7pfs_by()[2],
    ",
  0x400809f7u64 => "
      PFS.p7pfs_by()[3],
    ",
  0x40080a00u64 => "
      PFS.p80pfs()[0],
    ",
  0x40080a04u64 => "
      PFS.p80pfs()[1],
    ",
  0x40080a02u64 => "
      PFS.p80pfs_ha()[0],
    ",
  0x40080a06u64 => "
      PFS.p80pfs_ha()[1],
    ",
  0x40080a03u64 => "
      PFS.p80pfs_by()[0],
    ",
  0x40080a07u64 => "
      PFS.p80pfs_by()[1],
    ",
  0x40080d00u64 => "
      PFS.pfenet(),
    ",
  0x40080d03u64 => "
      PFS.pwpr(),
    ",
  0x40080d05u64 => "
      PFS.pwprs(),
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
  0x40080d1cu64 => "
      PFS.psar()[6],
    ",
  0x40080d1eu64 => "
      PFS.psar()[7],
    ",
  0x40080d20u64 => "
      PFS.psar()[8],
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
  0x40082010u64 => "
      ELC.elsr()[0],
    ",
  0x40082014u64 => "
      ELC.elsr()[1],
    ",
  0x40082018u64 => "
      ELC.elsr()[2],
    ",
  0x4008201cu64 => "
      ELC.elsr()[3],
    ",
  0x40082020u64 => "
      ELC.elsr()[4],
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
  0x40082038u64 => "
      ELC.elsr()[10],
    ",
  0x4008203cu64 => "
      ELC.elsr()[11],
    ",
  0x40082040u64 => "
      ELC.elsr()[12],
    ",
  0x40082044u64 => "
      ELC.elsr()[13],
    ",
  0x40082048u64 => "
      ELC.elsr()[14],
    ",
  0x4008204cu64 => "
      ELC.elsr()[15],
    ",
  0x40082050u64 => "
      ELC.elsr()[16],
    ",
  0x40082054u64 => "
      ELC.elsr()[17],
    ",
  0x40082058u64 => "
      ELC.elsr()[18],
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
  0x400900b0u64 => "
      USBFS.bcctrl1(),
    ",
  0x400900b4u64 => "
      USBFS.bcctrl2(),
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
  0x400900f4u64 => "
      USBFS.physectrl(),
    ",
  0x40090400u64 => "
      USBFS.dpusr0r(),
    ",
  0x40090404u64 => "
      USBFS.dpusr1r(),
    ",
  0x40092000u64 => "
      SDHI_0.sd_cmd(),
    ",
  0x40092008u64 => "
      SDHI_0.sd_arg(),
    ",
  0x4009200cu64 => "
      SDHI_0.sd_arg1(),
    ",
  0x40092010u64 => "
      SDHI_0.sd_stop(),
    ",
  0x40092014u64 => "
      SDHI_0.sd_seccnt(),
    ",
  0x40092018u64 => "
      SDHI_0.sd_rsp10(),
    ",
  0x4009201cu64 => "
      SDHI_0.sd_rsp1(),
    ",
  0x40092020u64 => "
      SDHI_0.sd_rsp32(),
    ",
  0x40092024u64 => "
      SDHI_0.sd_rsp3(),
    ",
  0x40092028u64 => "
      SDHI_0.sd_rsp54(),
    ",
  0x4009202cu64 => "
      SDHI_0.sd_rsp5(),
    ",
  0x40092030u64 => "
      SDHI_0.sd_rsp76(),
    ",
  0x40092034u64 => "
      SDHI_0.sd_rsp7(),
    ",
  0x40092038u64 => "
      SDHI_0.sd_info1(),
    ",
  0x4009203cu64 => "
      SDHI_0.sd_info2(),
    ",
  0x40092040u64 => "
      SDHI_0.sd_info1_mask(),
    ",
  0x40092044u64 => "
      SDHI_0.sd_info2_mask(),
    ",
  0x40092048u64 => "
      SDHI_0.sd_clk_ctrl(),
    ",
  0x4009204cu64 => "
      SDHI_0.sd_size(),
    ",
  0x40092050u64 => "
      SDHI_0.sd_option(),
    ",
  0x40092058u64 => "
      SDHI_0.sd_err_sts1(),
    ",
  0x4009205cu64 => "
      SDHI_0.sd_err_sts2(),
    ",
  0x40092060u64 => "
      SDHI_0.sd_buf0(),
    ",
  0x40092068u64 => "
      SDHI_0.sdio_mode(),
    ",
  0x4009206cu64 => "
      SDHI_0.sdio_info1(),
    ",
  0x40092070u64 => "
      SDHI_0.sdio_info1_mask(),
    ",
  0x400921b0u64 => "
      SDHI_0.sd_dmaen(),
    ",
  0x400921c0u64 => "
      SDHI_0.soft_rst(),
    ",
  0x400921ccu64 => "
      SDHI_0.sdif_mode(),
    ",
  0x400921e0u64 => "
      SDHI_0.ext_swap(),
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
  0x400a6000u64 => "
      OSPI.dcr(),
    ",
  0x400a6004u64 => "
      OSPI.dar(),
    ",
  0x400a6008u64 => "
      OSPI.dcsr(),
    ",
  0x400a600cu64 => "
      OSPI.dsr0(),
    ",
  0x400a6010u64 => "
      OSPI.dsr1(),
    ",
  0x400a6014u64 => "
      OSPI.mdtr(),
    ",
  0x400a6018u64 => "
      OSPI.actr(),
    ",
  0x400a601cu64 => "
      OSPI.acar0(),
    ",
  0x400a6020u64 => "
      OSPI.acar1(),
    ",
  0x400a6034u64 => "
      OSPI.drcstr(),
    ",
  0x400a6038u64 => "
      OSPI.dwcstr(),
    ",
  0x400a603cu64 => "
      OSPI.dcstr(),
    ",
  0x400a6040u64 => "
      OSPI.cdsr(),
    ",
  0x400a6044u64 => "
      OSPI.mdlr(),
    ",
  0x400a6048u64 => "
      OSPI.mrwcr0(),
    ",
  0x400a604cu64 => "
      OSPI.mrwcr1(),
    ",
  0x400a6050u64 => "
      OSPI.mrwcsr(),
    ",
  0x400a6054u64 => "
      OSPI.esr(),
    ",
  0x400a6058u64 => "
      OSPI.cwndr(),
    ",
  0x400a605cu64 => "
      OSPI.cwdr(),
    ",
  0x400a6060u64 => "
      OSPI.crr(),
    ",
  0x400a6064u64 => "
      OSPI.acsr(),
    ",
  0x400a607cu64 => "
      OSPI.dcsmxr(),
    ",
  0x400a6080u64 => "
      OSPI.dwsctsr(),
    ",
  0x400a8200u64 => "
      CAN_0.mb_id()[0],
    ",
  0x400a8210u64 => "
      CAN_0.mb_id()[1],
    ",
  0x400a8220u64 => "
      CAN_0.mb_id()[2],
    ",
  0x400a8230u64 => "
      CAN_0.mb_id()[3],
    ",
  0x400a8240u64 => "
      CAN_0.mb_id()[4],
    ",
  0x400a8250u64 => "
      CAN_0.mb_id()[5],
    ",
  0x400a8260u64 => "
      CAN_0.mb_id()[6],
    ",
  0x400a8270u64 => "
      CAN_0.mb_id()[7],
    ",
  0x400a8280u64 => "
      CAN_0.mb_id()[8],
    ",
  0x400a8290u64 => "
      CAN_0.mb_id()[9],
    ",
  0x400a82a0u64 => "
      CAN_0.mb_id()[10],
    ",
  0x400a82b0u64 => "
      CAN_0.mb_id()[11],
    ",
  0x400a82c0u64 => "
      CAN_0.mb_id()[12],
    ",
  0x400a82d0u64 => "
      CAN_0.mb_id()[13],
    ",
  0x400a82e0u64 => "
      CAN_0.mb_id()[14],
    ",
  0x400a82f0u64 => "
      CAN_0.mb_id()[15],
    ",
  0x400a8300u64 => "
      CAN_0.mb_id()[16],
    ",
  0x400a8310u64 => "
      CAN_0.mb_id()[17],
    ",
  0x400a8320u64 => "
      CAN_0.mb_id()[18],
    ",
  0x400a8330u64 => "
      CAN_0.mb_id()[19],
    ",
  0x400a8340u64 => "
      CAN_0.mb_id()[20],
    ",
  0x400a8350u64 => "
      CAN_0.mb_id()[21],
    ",
  0x400a8360u64 => "
      CAN_0.mb_id()[22],
    ",
  0x400a8370u64 => "
      CAN_0.mb_id()[23],
    ",
  0x400a8380u64 => "
      CAN_0.mb_id()[24],
    ",
  0x400a8390u64 => "
      CAN_0.mb_id()[25],
    ",
  0x400a83a0u64 => "
      CAN_0.mb_id()[26],
    ",
  0x400a83b0u64 => "
      CAN_0.mb_id()[27],
    ",
  0x400a83c0u64 => "
      CAN_0.mb_id()[28],
    ",
  0x400a83d0u64 => "
      CAN_0.mb_id()[29],
    ",
  0x400a83e0u64 => "
      CAN_0.mb_id()[30],
    ",
  0x400a83f0u64 => "
      CAN_0.mb_id()[31],
    ",
  0x400a8204u64 => "
      CAN_0.mb_dl()[0],
    ",
  0x400a8214u64 => "
      CAN_0.mb_dl()[1],
    ",
  0x400a8224u64 => "
      CAN_0.mb_dl()[2],
    ",
  0x400a8234u64 => "
      CAN_0.mb_dl()[3],
    ",
  0x400a8244u64 => "
      CAN_0.mb_dl()[4],
    ",
  0x400a8254u64 => "
      CAN_0.mb_dl()[5],
    ",
  0x400a8264u64 => "
      CAN_0.mb_dl()[6],
    ",
  0x400a8274u64 => "
      CAN_0.mb_dl()[7],
    ",
  0x400a8284u64 => "
      CAN_0.mb_dl()[8],
    ",
  0x400a8294u64 => "
      CAN_0.mb_dl()[9],
    ",
  0x400a82a4u64 => "
      CAN_0.mb_dl()[10],
    ",
  0x400a82b4u64 => "
      CAN_0.mb_dl()[11],
    ",
  0x400a82c4u64 => "
      CAN_0.mb_dl()[12],
    ",
  0x400a82d4u64 => "
      CAN_0.mb_dl()[13],
    ",
  0x400a82e4u64 => "
      CAN_0.mb_dl()[14],
    ",
  0x400a82f4u64 => "
      CAN_0.mb_dl()[15],
    ",
  0x400a8304u64 => "
      CAN_0.mb_dl()[16],
    ",
  0x400a8314u64 => "
      CAN_0.mb_dl()[17],
    ",
  0x400a8324u64 => "
      CAN_0.mb_dl()[18],
    ",
  0x400a8334u64 => "
      CAN_0.mb_dl()[19],
    ",
  0x400a8344u64 => "
      CAN_0.mb_dl()[20],
    ",
  0x400a8354u64 => "
      CAN_0.mb_dl()[21],
    ",
  0x400a8364u64 => "
      CAN_0.mb_dl()[22],
    ",
  0x400a8374u64 => "
      CAN_0.mb_dl()[23],
    ",
  0x400a8384u64 => "
      CAN_0.mb_dl()[24],
    ",
  0x400a8394u64 => "
      CAN_0.mb_dl()[25],
    ",
  0x400a83a4u64 => "
      CAN_0.mb_dl()[26],
    ",
  0x400a83b4u64 => "
      CAN_0.mb_dl()[27],
    ",
  0x400a83c4u64 => "
      CAN_0.mb_dl()[28],
    ",
  0x400a83d4u64 => "
      CAN_0.mb_dl()[29],
    ",
  0x400a83e4u64 => "
      CAN_0.mb_dl()[30],
    ",
  0x400a83f4u64 => "
      CAN_0.mb_dl()[31],
    ",
  0x400a8206u64 => "
      CAN_0.mb_d0()[0],
    ",
  0x400a8216u64 => "
      CAN_0.mb_d0()[1],
    ",
  0x400a8226u64 => "
      CAN_0.mb_d0()[2],
    ",
  0x400a8236u64 => "
      CAN_0.mb_d0()[3],
    ",
  0x400a8246u64 => "
      CAN_0.mb_d0()[4],
    ",
  0x400a8256u64 => "
      CAN_0.mb_d0()[5],
    ",
  0x400a8266u64 => "
      CAN_0.mb_d0()[6],
    ",
  0x400a8276u64 => "
      CAN_0.mb_d0()[7],
    ",
  0x400a8286u64 => "
      CAN_0.mb_d0()[8],
    ",
  0x400a8296u64 => "
      CAN_0.mb_d0()[9],
    ",
  0x400a82a6u64 => "
      CAN_0.mb_d0()[10],
    ",
  0x400a82b6u64 => "
      CAN_0.mb_d0()[11],
    ",
  0x400a82c6u64 => "
      CAN_0.mb_d0()[12],
    ",
  0x400a82d6u64 => "
      CAN_0.mb_d0()[13],
    ",
  0x400a82e6u64 => "
      CAN_0.mb_d0()[14],
    ",
  0x400a82f6u64 => "
      CAN_0.mb_d0()[15],
    ",
  0x400a8306u64 => "
      CAN_0.mb_d0()[16],
    ",
  0x400a8316u64 => "
      CAN_0.mb_d0()[17],
    ",
  0x400a8326u64 => "
      CAN_0.mb_d0()[18],
    ",
  0x400a8336u64 => "
      CAN_0.mb_d0()[19],
    ",
  0x400a8346u64 => "
      CAN_0.mb_d0()[20],
    ",
  0x400a8356u64 => "
      CAN_0.mb_d0()[21],
    ",
  0x400a8366u64 => "
      CAN_0.mb_d0()[22],
    ",
  0x400a8376u64 => "
      CAN_0.mb_d0()[23],
    ",
  0x400a8386u64 => "
      CAN_0.mb_d0()[24],
    ",
  0x400a8396u64 => "
      CAN_0.mb_d0()[25],
    ",
  0x400a83a6u64 => "
      CAN_0.mb_d0()[26],
    ",
  0x400a83b6u64 => "
      CAN_0.mb_d0()[27],
    ",
  0x400a83c6u64 => "
      CAN_0.mb_d0()[28],
    ",
  0x400a83d6u64 => "
      CAN_0.mb_d0()[29],
    ",
  0x400a83e6u64 => "
      CAN_0.mb_d0()[30],
    ",
  0x400a83f6u64 => "
      CAN_0.mb_d0()[31],
    ",
  0x400a8207u64 => "
      CAN_0.mb_d1()[0],
    ",
  0x400a8217u64 => "
      CAN_0.mb_d1()[1],
    ",
  0x400a8227u64 => "
      CAN_0.mb_d1()[2],
    ",
  0x400a8237u64 => "
      CAN_0.mb_d1()[3],
    ",
  0x400a8247u64 => "
      CAN_0.mb_d1()[4],
    ",
  0x400a8257u64 => "
      CAN_0.mb_d1()[5],
    ",
  0x400a8267u64 => "
      CAN_0.mb_d1()[6],
    ",
  0x400a8277u64 => "
      CAN_0.mb_d1()[7],
    ",
  0x400a8287u64 => "
      CAN_0.mb_d1()[8],
    ",
  0x400a8297u64 => "
      CAN_0.mb_d1()[9],
    ",
  0x400a82a7u64 => "
      CAN_0.mb_d1()[10],
    ",
  0x400a82b7u64 => "
      CAN_0.mb_d1()[11],
    ",
  0x400a82c7u64 => "
      CAN_0.mb_d1()[12],
    ",
  0x400a82d7u64 => "
      CAN_0.mb_d1()[13],
    ",
  0x400a82e7u64 => "
      CAN_0.mb_d1()[14],
    ",
  0x400a82f7u64 => "
      CAN_0.mb_d1()[15],
    ",
  0x400a8307u64 => "
      CAN_0.mb_d1()[16],
    ",
  0x400a8317u64 => "
      CAN_0.mb_d1()[17],
    ",
  0x400a8327u64 => "
      CAN_0.mb_d1()[18],
    ",
  0x400a8337u64 => "
      CAN_0.mb_d1()[19],
    ",
  0x400a8347u64 => "
      CAN_0.mb_d1()[20],
    ",
  0x400a8357u64 => "
      CAN_0.mb_d1()[21],
    ",
  0x400a8367u64 => "
      CAN_0.mb_d1()[22],
    ",
  0x400a8377u64 => "
      CAN_0.mb_d1()[23],
    ",
  0x400a8387u64 => "
      CAN_0.mb_d1()[24],
    ",
  0x400a8397u64 => "
      CAN_0.mb_d1()[25],
    ",
  0x400a83a7u64 => "
      CAN_0.mb_d1()[26],
    ",
  0x400a83b7u64 => "
      CAN_0.mb_d1()[27],
    ",
  0x400a83c7u64 => "
      CAN_0.mb_d1()[28],
    ",
  0x400a83d7u64 => "
      CAN_0.mb_d1()[29],
    ",
  0x400a83e7u64 => "
      CAN_0.mb_d1()[30],
    ",
  0x400a83f7u64 => "
      CAN_0.mb_d1()[31],
    ",
  0x400a8208u64 => "
      CAN_0.mb_d2()[0],
    ",
  0x400a8218u64 => "
      CAN_0.mb_d2()[1],
    ",
  0x400a8228u64 => "
      CAN_0.mb_d2()[2],
    ",
  0x400a8238u64 => "
      CAN_0.mb_d2()[3],
    ",
  0x400a8248u64 => "
      CAN_0.mb_d2()[4],
    ",
  0x400a8258u64 => "
      CAN_0.mb_d2()[5],
    ",
  0x400a8268u64 => "
      CAN_0.mb_d2()[6],
    ",
  0x400a8278u64 => "
      CAN_0.mb_d2()[7],
    ",
  0x400a8288u64 => "
      CAN_0.mb_d2()[8],
    ",
  0x400a8298u64 => "
      CAN_0.mb_d2()[9],
    ",
  0x400a82a8u64 => "
      CAN_0.mb_d2()[10],
    ",
  0x400a82b8u64 => "
      CAN_0.mb_d2()[11],
    ",
  0x400a82c8u64 => "
      CAN_0.mb_d2()[12],
    ",
  0x400a82d8u64 => "
      CAN_0.mb_d2()[13],
    ",
  0x400a82e8u64 => "
      CAN_0.mb_d2()[14],
    ",
  0x400a82f8u64 => "
      CAN_0.mb_d2()[15],
    ",
  0x400a8308u64 => "
      CAN_0.mb_d2()[16],
    ",
  0x400a8318u64 => "
      CAN_0.mb_d2()[17],
    ",
  0x400a8328u64 => "
      CAN_0.mb_d2()[18],
    ",
  0x400a8338u64 => "
      CAN_0.mb_d2()[19],
    ",
  0x400a8348u64 => "
      CAN_0.mb_d2()[20],
    ",
  0x400a8358u64 => "
      CAN_0.mb_d2()[21],
    ",
  0x400a8368u64 => "
      CAN_0.mb_d2()[22],
    ",
  0x400a8378u64 => "
      CAN_0.mb_d2()[23],
    ",
  0x400a8388u64 => "
      CAN_0.mb_d2()[24],
    ",
  0x400a8398u64 => "
      CAN_0.mb_d2()[25],
    ",
  0x400a83a8u64 => "
      CAN_0.mb_d2()[26],
    ",
  0x400a83b8u64 => "
      CAN_0.mb_d2()[27],
    ",
  0x400a83c8u64 => "
      CAN_0.mb_d2()[28],
    ",
  0x400a83d8u64 => "
      CAN_0.mb_d2()[29],
    ",
  0x400a83e8u64 => "
      CAN_0.mb_d2()[30],
    ",
  0x400a83f8u64 => "
      CAN_0.mb_d2()[31],
    ",
  0x400a8209u64 => "
      CAN_0.mb_d3()[0],
    ",
  0x400a8219u64 => "
      CAN_0.mb_d3()[1],
    ",
  0x400a8229u64 => "
      CAN_0.mb_d3()[2],
    ",
  0x400a8239u64 => "
      CAN_0.mb_d3()[3],
    ",
  0x400a8249u64 => "
      CAN_0.mb_d3()[4],
    ",
  0x400a8259u64 => "
      CAN_0.mb_d3()[5],
    ",
  0x400a8269u64 => "
      CAN_0.mb_d3()[6],
    ",
  0x400a8279u64 => "
      CAN_0.mb_d3()[7],
    ",
  0x400a8289u64 => "
      CAN_0.mb_d3()[8],
    ",
  0x400a8299u64 => "
      CAN_0.mb_d3()[9],
    ",
  0x400a82a9u64 => "
      CAN_0.mb_d3()[10],
    ",
  0x400a82b9u64 => "
      CAN_0.mb_d3()[11],
    ",
  0x400a82c9u64 => "
      CAN_0.mb_d3()[12],
    ",
  0x400a82d9u64 => "
      CAN_0.mb_d3()[13],
    ",
  0x400a82e9u64 => "
      CAN_0.mb_d3()[14],
    ",
  0x400a82f9u64 => "
      CAN_0.mb_d3()[15],
    ",
  0x400a8309u64 => "
      CAN_0.mb_d3()[16],
    ",
  0x400a8319u64 => "
      CAN_0.mb_d3()[17],
    ",
  0x400a8329u64 => "
      CAN_0.mb_d3()[18],
    ",
  0x400a8339u64 => "
      CAN_0.mb_d3()[19],
    ",
  0x400a8349u64 => "
      CAN_0.mb_d3()[20],
    ",
  0x400a8359u64 => "
      CAN_0.mb_d3()[21],
    ",
  0x400a8369u64 => "
      CAN_0.mb_d3()[22],
    ",
  0x400a8379u64 => "
      CAN_0.mb_d3()[23],
    ",
  0x400a8389u64 => "
      CAN_0.mb_d3()[24],
    ",
  0x400a8399u64 => "
      CAN_0.mb_d3()[25],
    ",
  0x400a83a9u64 => "
      CAN_0.mb_d3()[26],
    ",
  0x400a83b9u64 => "
      CAN_0.mb_d3()[27],
    ",
  0x400a83c9u64 => "
      CAN_0.mb_d3()[28],
    ",
  0x400a83d9u64 => "
      CAN_0.mb_d3()[29],
    ",
  0x400a83e9u64 => "
      CAN_0.mb_d3()[30],
    ",
  0x400a83f9u64 => "
      CAN_0.mb_d3()[31],
    ",
  0x400a820au64 => "
      CAN_0.mb_d4()[0],
    ",
  0x400a821au64 => "
      CAN_0.mb_d4()[1],
    ",
  0x400a822au64 => "
      CAN_0.mb_d4()[2],
    ",
  0x400a823au64 => "
      CAN_0.mb_d4()[3],
    ",
  0x400a824au64 => "
      CAN_0.mb_d4()[4],
    ",
  0x400a825au64 => "
      CAN_0.mb_d4()[5],
    ",
  0x400a826au64 => "
      CAN_0.mb_d4()[6],
    ",
  0x400a827au64 => "
      CAN_0.mb_d4()[7],
    ",
  0x400a828au64 => "
      CAN_0.mb_d4()[8],
    ",
  0x400a829au64 => "
      CAN_0.mb_d4()[9],
    ",
  0x400a82aau64 => "
      CAN_0.mb_d4()[10],
    ",
  0x400a82bau64 => "
      CAN_0.mb_d4()[11],
    ",
  0x400a82cau64 => "
      CAN_0.mb_d4()[12],
    ",
  0x400a82dau64 => "
      CAN_0.mb_d4()[13],
    ",
  0x400a82eau64 => "
      CAN_0.mb_d4()[14],
    ",
  0x400a82fau64 => "
      CAN_0.mb_d4()[15],
    ",
  0x400a830au64 => "
      CAN_0.mb_d4()[16],
    ",
  0x400a831au64 => "
      CAN_0.mb_d4()[17],
    ",
  0x400a832au64 => "
      CAN_0.mb_d4()[18],
    ",
  0x400a833au64 => "
      CAN_0.mb_d4()[19],
    ",
  0x400a834au64 => "
      CAN_0.mb_d4()[20],
    ",
  0x400a835au64 => "
      CAN_0.mb_d4()[21],
    ",
  0x400a836au64 => "
      CAN_0.mb_d4()[22],
    ",
  0x400a837au64 => "
      CAN_0.mb_d4()[23],
    ",
  0x400a838au64 => "
      CAN_0.mb_d4()[24],
    ",
  0x400a839au64 => "
      CAN_0.mb_d4()[25],
    ",
  0x400a83aau64 => "
      CAN_0.mb_d4()[26],
    ",
  0x400a83bau64 => "
      CAN_0.mb_d4()[27],
    ",
  0x400a83cau64 => "
      CAN_0.mb_d4()[28],
    ",
  0x400a83dau64 => "
      CAN_0.mb_d4()[29],
    ",
  0x400a83eau64 => "
      CAN_0.mb_d4()[30],
    ",
  0x400a83fau64 => "
      CAN_0.mb_d4()[31],
    ",
  0x400a820bu64 => "
      CAN_0.mb_d5()[0],
    ",
  0x400a821bu64 => "
      CAN_0.mb_d5()[1],
    ",
  0x400a822bu64 => "
      CAN_0.mb_d5()[2],
    ",
  0x400a823bu64 => "
      CAN_0.mb_d5()[3],
    ",
  0x400a824bu64 => "
      CAN_0.mb_d5()[4],
    ",
  0x400a825bu64 => "
      CAN_0.mb_d5()[5],
    ",
  0x400a826bu64 => "
      CAN_0.mb_d5()[6],
    ",
  0x400a827bu64 => "
      CAN_0.mb_d5()[7],
    ",
  0x400a828bu64 => "
      CAN_0.mb_d5()[8],
    ",
  0x400a829bu64 => "
      CAN_0.mb_d5()[9],
    ",
  0x400a82abu64 => "
      CAN_0.mb_d5()[10],
    ",
  0x400a82bbu64 => "
      CAN_0.mb_d5()[11],
    ",
  0x400a82cbu64 => "
      CAN_0.mb_d5()[12],
    ",
  0x400a82dbu64 => "
      CAN_0.mb_d5()[13],
    ",
  0x400a82ebu64 => "
      CAN_0.mb_d5()[14],
    ",
  0x400a82fbu64 => "
      CAN_0.mb_d5()[15],
    ",
  0x400a830bu64 => "
      CAN_0.mb_d5()[16],
    ",
  0x400a831bu64 => "
      CAN_0.mb_d5()[17],
    ",
  0x400a832bu64 => "
      CAN_0.mb_d5()[18],
    ",
  0x400a833bu64 => "
      CAN_0.mb_d5()[19],
    ",
  0x400a834bu64 => "
      CAN_0.mb_d5()[20],
    ",
  0x400a835bu64 => "
      CAN_0.mb_d5()[21],
    ",
  0x400a836bu64 => "
      CAN_0.mb_d5()[22],
    ",
  0x400a837bu64 => "
      CAN_0.mb_d5()[23],
    ",
  0x400a838bu64 => "
      CAN_0.mb_d5()[24],
    ",
  0x400a839bu64 => "
      CAN_0.mb_d5()[25],
    ",
  0x400a83abu64 => "
      CAN_0.mb_d5()[26],
    ",
  0x400a83bbu64 => "
      CAN_0.mb_d5()[27],
    ",
  0x400a83cbu64 => "
      CAN_0.mb_d5()[28],
    ",
  0x400a83dbu64 => "
      CAN_0.mb_d5()[29],
    ",
  0x400a83ebu64 => "
      CAN_0.mb_d5()[30],
    ",
  0x400a83fbu64 => "
      CAN_0.mb_d5()[31],
    ",
  0x400a820cu64 => "
      CAN_0.mb_d6()[0],
    ",
  0x400a821cu64 => "
      CAN_0.mb_d6()[1],
    ",
  0x400a822cu64 => "
      CAN_0.mb_d6()[2],
    ",
  0x400a823cu64 => "
      CAN_0.mb_d6()[3],
    ",
  0x400a824cu64 => "
      CAN_0.mb_d6()[4],
    ",
  0x400a825cu64 => "
      CAN_0.mb_d6()[5],
    ",
  0x400a826cu64 => "
      CAN_0.mb_d6()[6],
    ",
  0x400a827cu64 => "
      CAN_0.mb_d6()[7],
    ",
  0x400a828cu64 => "
      CAN_0.mb_d6()[8],
    ",
  0x400a829cu64 => "
      CAN_0.mb_d6()[9],
    ",
  0x400a82acu64 => "
      CAN_0.mb_d6()[10],
    ",
  0x400a82bcu64 => "
      CAN_0.mb_d6()[11],
    ",
  0x400a82ccu64 => "
      CAN_0.mb_d6()[12],
    ",
  0x400a82dcu64 => "
      CAN_0.mb_d6()[13],
    ",
  0x400a82ecu64 => "
      CAN_0.mb_d6()[14],
    ",
  0x400a82fcu64 => "
      CAN_0.mb_d6()[15],
    ",
  0x400a830cu64 => "
      CAN_0.mb_d6()[16],
    ",
  0x400a831cu64 => "
      CAN_0.mb_d6()[17],
    ",
  0x400a832cu64 => "
      CAN_0.mb_d6()[18],
    ",
  0x400a833cu64 => "
      CAN_0.mb_d6()[19],
    ",
  0x400a834cu64 => "
      CAN_0.mb_d6()[20],
    ",
  0x400a835cu64 => "
      CAN_0.mb_d6()[21],
    ",
  0x400a836cu64 => "
      CAN_0.mb_d6()[22],
    ",
  0x400a837cu64 => "
      CAN_0.mb_d6()[23],
    ",
  0x400a838cu64 => "
      CAN_0.mb_d6()[24],
    ",
  0x400a839cu64 => "
      CAN_0.mb_d6()[25],
    ",
  0x400a83acu64 => "
      CAN_0.mb_d6()[26],
    ",
  0x400a83bcu64 => "
      CAN_0.mb_d6()[27],
    ",
  0x400a83ccu64 => "
      CAN_0.mb_d6()[28],
    ",
  0x400a83dcu64 => "
      CAN_0.mb_d6()[29],
    ",
  0x400a83ecu64 => "
      CAN_0.mb_d6()[30],
    ",
  0x400a83fcu64 => "
      CAN_0.mb_d6()[31],
    ",
  0x400a820du64 => "
      CAN_0.mb_d7()[0],
    ",
  0x400a821du64 => "
      CAN_0.mb_d7()[1],
    ",
  0x400a822du64 => "
      CAN_0.mb_d7()[2],
    ",
  0x400a823du64 => "
      CAN_0.mb_d7()[3],
    ",
  0x400a824du64 => "
      CAN_0.mb_d7()[4],
    ",
  0x400a825du64 => "
      CAN_0.mb_d7()[5],
    ",
  0x400a826du64 => "
      CAN_0.mb_d7()[6],
    ",
  0x400a827du64 => "
      CAN_0.mb_d7()[7],
    ",
  0x400a828du64 => "
      CAN_0.mb_d7()[8],
    ",
  0x400a829du64 => "
      CAN_0.mb_d7()[9],
    ",
  0x400a82adu64 => "
      CAN_0.mb_d7()[10],
    ",
  0x400a82bdu64 => "
      CAN_0.mb_d7()[11],
    ",
  0x400a82cdu64 => "
      CAN_0.mb_d7()[12],
    ",
  0x400a82ddu64 => "
      CAN_0.mb_d7()[13],
    ",
  0x400a82edu64 => "
      CAN_0.mb_d7()[14],
    ",
  0x400a82fdu64 => "
      CAN_0.mb_d7()[15],
    ",
  0x400a830du64 => "
      CAN_0.mb_d7()[16],
    ",
  0x400a831du64 => "
      CAN_0.mb_d7()[17],
    ",
  0x400a832du64 => "
      CAN_0.mb_d7()[18],
    ",
  0x400a833du64 => "
      CAN_0.mb_d7()[19],
    ",
  0x400a834du64 => "
      CAN_0.mb_d7()[20],
    ",
  0x400a835du64 => "
      CAN_0.mb_d7()[21],
    ",
  0x400a836du64 => "
      CAN_0.mb_d7()[22],
    ",
  0x400a837du64 => "
      CAN_0.mb_d7()[23],
    ",
  0x400a838du64 => "
      CAN_0.mb_d7()[24],
    ",
  0x400a839du64 => "
      CAN_0.mb_d7()[25],
    ",
  0x400a83adu64 => "
      CAN_0.mb_d7()[26],
    ",
  0x400a83bdu64 => "
      CAN_0.mb_d7()[27],
    ",
  0x400a83cdu64 => "
      CAN_0.mb_d7()[28],
    ",
  0x400a83ddu64 => "
      CAN_0.mb_d7()[29],
    ",
  0x400a83edu64 => "
      CAN_0.mb_d7()[30],
    ",
  0x400a83fdu64 => "
      CAN_0.mb_d7()[31],
    ",
  0x400a820eu64 => "
      CAN_0.mb_ts()[0],
    ",
  0x400a821eu64 => "
      CAN_0.mb_ts()[1],
    ",
  0x400a822eu64 => "
      CAN_0.mb_ts()[2],
    ",
  0x400a823eu64 => "
      CAN_0.mb_ts()[3],
    ",
  0x400a824eu64 => "
      CAN_0.mb_ts()[4],
    ",
  0x400a825eu64 => "
      CAN_0.mb_ts()[5],
    ",
  0x400a826eu64 => "
      CAN_0.mb_ts()[6],
    ",
  0x400a827eu64 => "
      CAN_0.mb_ts()[7],
    ",
  0x400a828eu64 => "
      CAN_0.mb_ts()[8],
    ",
  0x400a829eu64 => "
      CAN_0.mb_ts()[9],
    ",
  0x400a82aeu64 => "
      CAN_0.mb_ts()[10],
    ",
  0x400a82beu64 => "
      CAN_0.mb_ts()[11],
    ",
  0x400a82ceu64 => "
      CAN_0.mb_ts()[12],
    ",
  0x400a82deu64 => "
      CAN_0.mb_ts()[13],
    ",
  0x400a82eeu64 => "
      CAN_0.mb_ts()[14],
    ",
  0x400a82feu64 => "
      CAN_0.mb_ts()[15],
    ",
  0x400a830eu64 => "
      CAN_0.mb_ts()[16],
    ",
  0x400a831eu64 => "
      CAN_0.mb_ts()[17],
    ",
  0x400a832eu64 => "
      CAN_0.mb_ts()[18],
    ",
  0x400a833eu64 => "
      CAN_0.mb_ts()[19],
    ",
  0x400a834eu64 => "
      CAN_0.mb_ts()[20],
    ",
  0x400a835eu64 => "
      CAN_0.mb_ts()[21],
    ",
  0x400a836eu64 => "
      CAN_0.mb_ts()[22],
    ",
  0x400a837eu64 => "
      CAN_0.mb_ts()[23],
    ",
  0x400a838eu64 => "
      CAN_0.mb_ts()[24],
    ",
  0x400a839eu64 => "
      CAN_0.mb_ts()[25],
    ",
  0x400a83aeu64 => "
      CAN_0.mb_ts()[26],
    ",
  0x400a83beu64 => "
      CAN_0.mb_ts()[27],
    ",
  0x400a83ceu64 => "
      CAN_0.mb_ts()[28],
    ",
  0x400a83deu64 => "
      CAN_0.mb_ts()[29],
    ",
  0x400a83eeu64 => "
      CAN_0.mb_ts()[30],
    ",
  0x400a83feu64 => "
      CAN_0.mb_ts()[31],
    ",
  0x400a8400u64 => "
      CAN_0.mkr()[0],
    ",
  0x400a8404u64 => "
      CAN_0.mkr()[1],
    ",
  0x400a8408u64 => "
      CAN_0.mkr()[2],
    ",
  0x400a840cu64 => "
      CAN_0.mkr()[3],
    ",
  0x400a8410u64 => "
      CAN_0.mkr()[4],
    ",
  0x400a8414u64 => "
      CAN_0.mkr()[5],
    ",
  0x400a8418u64 => "
      CAN_0.mkr()[6],
    ",
  0x400a841cu64 => "
      CAN_0.mkr()[7],
    ",
  0x400a8420u64 => "
      CAN_0.fidcr()[0],
    ",
  0x400a8424u64 => "
      CAN_0.fidcr()[1],
    ",
  0x400a8428u64 => "
      CAN_0.mkivlr(),
    ",
  0x400a842cu64 => "
      CAN_0.mier(),
      CAN_0.mier_fifo(),
    ",
  0x400a8820u64 => "
      CAN_0.mctl_rx()[0],
      CAN_0.mctl_tx()[0],
    ",
  0x400a8821u64 => "
      CAN_0.mctl_rx()[1],
      CAN_0.mctl_tx()[1],
    ",
  0x400a8822u64 => "
      CAN_0.mctl_rx()[2],
      CAN_0.mctl_tx()[2],
    ",
  0x400a8823u64 => "
      CAN_0.mctl_rx()[3],
      CAN_0.mctl_tx()[3],
    ",
  0x400a8824u64 => "
      CAN_0.mctl_rx()[4],
      CAN_0.mctl_tx()[4],
    ",
  0x400a8825u64 => "
      CAN_0.mctl_rx()[5],
      CAN_0.mctl_tx()[5],
    ",
  0x400a8826u64 => "
      CAN_0.mctl_rx()[6],
      CAN_0.mctl_tx()[6],
    ",
  0x400a8827u64 => "
      CAN_0.mctl_rx()[7],
      CAN_0.mctl_tx()[7],
    ",
  0x400a8828u64 => "
      CAN_0.mctl_rx()[8],
      CAN_0.mctl_tx()[8],
    ",
  0x400a8829u64 => "
      CAN_0.mctl_rx()[9],
      CAN_0.mctl_tx()[9],
    ",
  0x400a882au64 => "
      CAN_0.mctl_rx()[10],
      CAN_0.mctl_tx()[10],
    ",
  0x400a882bu64 => "
      CAN_0.mctl_rx()[11],
      CAN_0.mctl_tx()[11],
    ",
  0x400a882cu64 => "
      CAN_0.mctl_rx()[12],
      CAN_0.mctl_tx()[12],
    ",
  0x400a882du64 => "
      CAN_0.mctl_rx()[13],
      CAN_0.mctl_tx()[13],
    ",
  0x400a882eu64 => "
      CAN_0.mctl_rx()[14],
      CAN_0.mctl_tx()[14],
    ",
  0x400a882fu64 => "
      CAN_0.mctl_rx()[15],
      CAN_0.mctl_tx()[15],
    ",
  0x400a8830u64 => "
      CAN_0.mctl_rx()[16],
      CAN_0.mctl_tx()[16],
    ",
  0x400a8831u64 => "
      CAN_0.mctl_rx()[17],
      CAN_0.mctl_tx()[17],
    ",
  0x400a8832u64 => "
      CAN_0.mctl_rx()[18],
      CAN_0.mctl_tx()[18],
    ",
  0x400a8833u64 => "
      CAN_0.mctl_rx()[19],
      CAN_0.mctl_tx()[19],
    ",
  0x400a8834u64 => "
      CAN_0.mctl_rx()[20],
      CAN_0.mctl_tx()[20],
    ",
  0x400a8835u64 => "
      CAN_0.mctl_rx()[21],
      CAN_0.mctl_tx()[21],
    ",
  0x400a8836u64 => "
      CAN_0.mctl_rx()[22],
      CAN_0.mctl_tx()[22],
    ",
  0x400a8837u64 => "
      CAN_0.mctl_rx()[23],
      CAN_0.mctl_tx()[23],
    ",
  0x400a8838u64 => "
      CAN_0.mctl_rx()[24],
      CAN_0.mctl_tx()[24],
    ",
  0x400a8839u64 => "
      CAN_0.mctl_rx()[25],
      CAN_0.mctl_tx()[25],
    ",
  0x400a883au64 => "
      CAN_0.mctl_rx()[26],
      CAN_0.mctl_tx()[26],
    ",
  0x400a883bu64 => "
      CAN_0.mctl_rx()[27],
      CAN_0.mctl_tx()[27],
    ",
  0x400a883cu64 => "
      CAN_0.mctl_rx()[28],
      CAN_0.mctl_tx()[28],
    ",
  0x400a883du64 => "
      CAN_0.mctl_rx()[29],
      CAN_0.mctl_tx()[29],
    ",
  0x400a883eu64 => "
      CAN_0.mctl_rx()[30],
      CAN_0.mctl_tx()[30],
    ",
  0x400a883fu64 => "
      CAN_0.mctl_rx()[31],
      CAN_0.mctl_tx()[31],
    ",
  0x400a8840u64 => "
      CAN_0.ctlr(),
    ",
  0x400a8842u64 => "
      CAN_0.str(),
    ",
  0x400a8844u64 => "
      CAN_0.bcr(),
    ",
  0x400a8848u64 => "
      CAN_0.rfcr(),
    ",
  0x400a8849u64 => "
      CAN_0.rfpcr(),
    ",
  0x400a884au64 => "
      CAN_0.tfcr(),
    ",
  0x400a884bu64 => "
      CAN_0.tfpcr(),
    ",
  0x400a884cu64 => "
      CAN_0.eier(),
    ",
  0x400a884du64 => "
      CAN_0.eifr(),
    ",
  0x400a884eu64 => "
      CAN_0.recr(),
    ",
  0x400a884fu64 => "
      CAN_0.tecr(),
    ",
  0x400a8850u64 => "
      CAN_0.ecsr(),
    ",
  0x400a8851u64 => "
      CAN_0.cssr(),
    ",
  0x400a8852u64 => "
      CAN_0.mssr(),
    ",
  0x400a8853u64 => "
      CAN_0.msmr(),
    ",
  0x400a8854u64 => "
      CAN_0.tsr(),
    ",
  0x400a8856u64 => "
      CAN_0.afsr(),
    ",
  0x400a8858u64 => "
      CAN_0.tcr(),
    ",
  0x400d0000u64 => "
      CTSU.ctsucr0(),
    ",
  0x400d0001u64 => "
      CTSU.ctsucr1(),
    ",
  0x400d0002u64 => "
      CTSU.ctsusdprs(),
    ",
  0x400d0003u64 => "
      CTSU.ctsusst(),
    ",
  0x400d0004u64 => "
      CTSU.ctsumch0(),
    ",
  0x400d0005u64 => "
      CTSU.ctsumch1(),
    ",
  0x400d0006u64 => "
      CTSU.ctsuchac0(),
    ",
  0x400d0007u64 => "
      CTSU.ctsuchac1(),
    ",
  0x400d0008u64 => "
      CTSU.ctsuchac2(),
    ",
  0x400d000bu64 => "
      CTSU.ctsuchtrc0(),
    ",
  0x400d000cu64 => "
      CTSU.ctsuchtrc1(),
    ",
  0x400d000du64 => "
      CTSU.ctsuchtrc2(),
    ",
  0x400d0010u64 => "
      CTSU.ctsudclkc(),
    ",
  0x400d0011u64 => "
      CTSU.ctsust(),
    ",
  0x400d0012u64 => "
      CTSU.ctsussc(),
    ",
  0x400d0014u64 => "
      CTSU.ctsuso0(),
    ",
  0x400d0016u64 => "
      CTSU.ctsuso1(),
    ",
  0x400d0018u64 => "
      CTSU.ctsusc(),
    ",
  0x400d001au64 => "
      CTSU.ctsurc(),
    ",
  0x400d001cu64 => "
      CTSU.ctsuerrs(),
    ",
  0x400d0020u64 => "
      CTSU.ctsutrmr(),
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
      AGT_0.agt(),
    ",
  0x400e8002u64 => "
      AGT_0.agtcma(),
    ",
  0x400e8004u64 => "
      AGT_0.agtcmb(),
    ",
  0x400e8008u64 => "
      AGT_0.agtcr(),
    ",
  0x400e8009u64 => "
      AGT_0.agtmr1(),
    ",
  0x400e800au64 => "
      AGT_0.agtmr2(),
    ",
  0x400e800cu64 => "
      AGT_0.agtioc(),
    ",
  0x400e800du64 => "
      AGT_0.agtisr(),
    ",
  0x400e800eu64 => "
      AGT_0.agtcmsr(),
    ",
  0x400e800fu64 => "
      AGT_0.agtiosel(),
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
  0x40114000u64 => "
      EDMAC_0.edmr(),
    ",
  0x40114008u64 => "
      EDMAC_0.edtrr(),
    ",
  0x40114010u64 => "
      EDMAC_0.edrrr(),
    ",
  0x40114018u64 => "
      EDMAC_0.tdlar(),
    ",
  0x40114020u64 => "
      EDMAC_0.rdlar(),
    ",
  0x40114028u64 => "
      EDMAC_0.eesr(),
    ",
  0x40114030u64 => "
      EDMAC_0.eesipr(),
    ",
  0x40114038u64 => "
      EDMAC_0.trscer(),
    ",
  0x40114040u64 => "
      EDMAC_0.rmfcr(),
    ",
  0x40114048u64 => "
      EDMAC_0.tftr(),
    ",
  0x40114050u64 => "
      EDMAC_0.fdr(),
    ",
  0x40114058u64 => "
      EDMAC_0.rmcr(),
    ",
  0x40114064u64 => "
      EDMAC_0.tfucr(),
    ",
  0x40114068u64 => "
      EDMAC_0.rfocr(),
    ",
  0x4011406cu64 => "
      EDMAC_0.iosr(),
    ",
  0x40114070u64 => "
      EDMAC_0.fcftr(),
    ",
  0x40114078u64 => "
      EDMAC_0.rpadir(),
    ",
  0x4011407cu64 => "
      EDMAC_0.trimd(),
    ",
  0x401140c8u64 => "
      EDMAC_0.rbwar(),
    ",
  0x401140ccu64 => "
      EDMAC_0.rdfar(),
    ",
  0x401140d4u64 => "
      EDMAC_0.tbrar(),
    ",
  0x401140d8u64 => "
      EDMAC_0.tdfar(),
    ",
  0x40114100u64 => "
      ETHERC_0.ecmr(),
    ",
  0x40114108u64 => "
      ETHERC_0.rflr(),
    ",
  0x40114110u64 => "
      ETHERC_0.ecsr(),
    ",
  0x40114118u64 => "
      ETHERC_0.ecsipr(),
    ",
  0x40114120u64 => "
      ETHERC_0.pir(),
    ",
  0x40114128u64 => "
      ETHERC_0.psr(),
    ",
  0x40114140u64 => "
      ETHERC_0.rdmlr(),
    ",
  0x40114150u64 => "
      ETHERC_0.ipgr(),
    ",
  0x40114154u64 => "
      ETHERC_0.apr(),
    ",
  0x40114158u64 => "
      ETHERC_0.mpr(),
    ",
  0x40114160u64 => "
      ETHERC_0.rfcf(),
    ",
  0x40114164u64 => "
      ETHERC_0.tpauser(),
    ",
  0x40114168u64 => "
      ETHERC_0.tpausecr(),
    ",
  0x4011416cu64 => "
      ETHERC_0.bcfrr(),
    ",
  0x401141c0u64 => "
      ETHERC_0.mahr(),
    ",
  0x401141c8u64 => "
      ETHERC_0.malr(),
    ",
  0x401141d0u64 => "
      ETHERC_0.trocr(),
    ",
  0x401141d4u64 => "
      ETHERC_0.cdcr(),
    ",
  0x401141d8u64 => "
      ETHERC_0.lccr(),
    ",
  0x401141dcu64 => "
      ETHERC_0.cndcr(),
    ",
  0x401141e4u64 => "
      ETHERC_0.cefcr(),
    ",
  0x401141e8u64 => "
      ETHERC_0.frecr(),
    ",
  0x401141ecu64 => "
      ETHERC_0.tsfrcr(),
    ",
  0x401141f0u64 => "
      ETHERC_0.tlfrcr(),
    ",
  0x401141f4u64 => "
      ETHERC_0.rfcr(),
    ",
  0x401141f8u64 => "
      ETHERC_0.mafcr(),
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
  0x40169400u64 => "
      GPT_164.gtwp(),
    ",
  0x40169404u64 => "
      GPT_164.gtstr(),
    ",
  0x40169408u64 => "
      GPT_164.gtstp(),
    ",
  0x4016940cu64 => "
      GPT_164.gtclr(),
    ",
  0x40169410u64 => "
      GPT_164.gtssr(),
    ",
  0x40169414u64 => "
      GPT_164.gtpsr(),
    ",
  0x40169418u64 => "
      GPT_164.gtcsr(),
    ",
  0x4016941cu64 => "
      GPT_164.gtupsr(),
    ",
  0x40169420u64 => "
      GPT_164.gtdnsr(),
    ",
  0x40169424u64 => "
      GPT_164.gticasr(),
    ",
  0x40169428u64 => "
      GPT_164.gticbsr(),
    ",
  0x4016942cu64 => "
      GPT_164.gtcr(),
    ",
  0x40169430u64 => "
      GPT_164.gtuddtyc(),
    ",
  0x40169434u64 => "
      GPT_164.gtior(),
    ",
  0x40169438u64 => "
      GPT_164.gtintad(),
    ",
  0x4016943cu64 => "
      GPT_164.gtst(),
    ",
  0x40169440u64 => "
      GPT_164.gtber(),
    ",
  0x40169448u64 => "
      GPT_164.gtcnt(),
    ",
  0x4016944cu64 => "
      GPT_164.gtccra(),
    ",
  0x40169450u64 => "
      GPT_164.gtccrb(),
    ",
  0x40169454u64 => "
      GPT_164.gtccrc(),
    ",
  0x40169458u64 => "
      GPT_164.gtccre(),
    ",
  0x4016945cu64 => "
      GPT_164.gtccrd(),
    ",
  0x40169460u64 => "
      GPT_164.gtccrf(),
    ",
  0x40169464u64 => "
      GPT_164.gtpr(),
    ",
  0x40169468u64 => "
      GPT_164.gtpbr(),
    ",
  0x40169488u64 => "
      GPT_164.gtdtcr(),
    ",
  0x4016948cu64 => "
      GPT_164.gtdvu(),
    ",
  0x401694b8u64 => "
      GPT_164.gticlf(),
    ",
  0x401694bcu64 => "
      GPT_164.gtpc(),
    ",
  0x401694d0u64 => "
      GPT_164.gtsecsr(),
    ",
  0x401694d4u64 => "
      GPT_164.gtsecr(),
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
  0x40170024u64 => "
      ADC_120.addr()[2],
    ",
  0x40170026u64 => "
      ADC_120.addr()[3],
    ",
  0x40170028u64 => "
      ADC_120.addr()[4],
    ",
  0x4017002au64 => "
      ADC_120.addr()[5],
    ",
  0x4017002cu64 => "
      ADC_120.addr()[6],
    ",
  0x4017002eu64 => "
      ADC_120.addr()[7],
    ",
  0x40170030u64 => "
      ADC_120.addr()[8],
    ",
  0x40170032u64 => "
      ADC_120.addr()[9],
    ",
  0x40170038u64 => "
      ADC_120.addr()[0],
    ",
  0x4017003au64 => "
      ADC_120.addr()[1],
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
  0x401700e7u64 => "
      ADC_120.adsstr()[7],
    ",
  0x401700e8u64 => "
      ADC_120.adsstr()[8],
    ",
  0x401700e9u64 => "
      ADC_120.adsstr()[9],
    ",
  0x401700ecu64 => "
      ADC_120.adsstr()[0],
    ",
  0x401700edu64 => "
      ADC_120.adsstr()[1],
    ",
  0x40170200u64 => "
      ADC_121.adcsr(),
    ",
  0x40170206u64 => "
      ADC_121.adansa1(),
    ",
  0x40170208u64 => "
      ADC_121.adads0(),
    ",
  0x4017020au64 => "
      ADC_121.adads1(),
    ",
  0x4017020cu64 => "
      ADC_121.adadc(),
    ",
  0x4017020eu64 => "
      ADC_121.adcer(),
    ",
  0x40170210u64 => "
      ADC_121.adstrgr(),
    ",
  0x40170212u64 => "
      ADC_121.adexicr(),
    ",
  0x40170214u64 => "
      ADC_121.adansb0(),
    ",
  0x40170216u64 => "
      ADC_121.adansb1(),
    ",
  0x40170218u64 => "
      ADC_121.addbldr(),
    ",
  0x4017021au64 => "
      ADC_121.adtsdr(),
    ",
  0x4017021cu64 => "
      ADC_121.adocdr(),
    ",
  0x4017021eu64 => "
      ADC_121.adrd(),
    ",
  0x40170240u64 => "
      ADC_121.addr()[0],
    ",
  0x40170242u64 => "
      ADC_121.addr()[1],
    ",
  0x40170244u64 => "
      ADC_121.addr()[2],
    ",
  0x40170246u64 => "
      ADC_121.addr()[3],
    ",
  0x40170248u64 => "
      ADC_121.addr()[4],
    ",
  0x4017024au64 => "
      ADC_121.addr()[5],
    ",
  0x4017024cu64 => "
      ADC_121.addr()[6],
    ",
  0x4017027au64 => "
      ADC_121.addiscr(),
    ",
  0x40170280u64 => "
      ADC_121.adgspcr(),
    ",
  0x40170284u64 => "
      ADC_121.addbldra(),
    ",
  0x40170286u64 => "
      ADC_121.addbldrb(),
    ",
  0x4017028cu64 => "
      ADC_121.adwinmon(),
    ",
  0x40170290u64 => "
      ADC_121.adcmpcr(),
    ",
  0x40170292u64 => "
      ADC_121.adcmpanser(),
    ",
  0x40170293u64 => "
      ADC_121.adcmpler(),
    ",
  0x40170294u64 => "
      ADC_121.adcmpansr0(),
    ",
  0x40170296u64 => "
      ADC_121.adcmpansr1(),
    ",
  0x40170298u64 => "
      ADC_121.adcmplr0(),
    ",
  0x4017029au64 => "
      ADC_121.adcmplr1(),
    ",
  0x4017029cu64 => "
      ADC_121.adcmpdr()[0],
    ",
  0x4017029eu64 => "
      ADC_121.adcmpdr()[1],
    ",
  0x401702a0u64 => "
      ADC_121.adcmpsr0(),
    ",
  0x401702a2u64 => "
      ADC_121.adcmpsr1(),
    ",
  0x401702a4u64 => "
      ADC_121.adcmpser(),
    ",
  0x401702a6u64 => "
      ADC_121.adcmpbnsr(),
    ",
  0x401702a8u64 => "
      ADC_121.adwinllb(),
    ",
  0x401702aau64 => "
      ADC_121.adwinulb(),
    ",
  0x401702acu64 => "
      ADC_121.adcmpbsr(),
    ",
  0x401702b0u64 => "
      ADC_121.adbuf()[0],
    ",
  0x401702b2u64 => "
      ADC_121.adbuf()[1],
    ",
  0x401702b4u64 => "
      ADC_121.adbuf()[2],
    ",
  0x401702b6u64 => "
      ADC_121.adbuf()[3],
    ",
  0x401702b8u64 => "
      ADC_121.adbuf()[4],
    ",
  0x401702bau64 => "
      ADC_121.adbuf()[5],
    ",
  0x401702bcu64 => "
      ADC_121.adbuf()[6],
    ",
  0x401702beu64 => "
      ADC_121.adbuf()[7],
    ",
  0x401702c0u64 => "
      ADC_121.adbuf()[8],
    ",
  0x401702c2u64 => "
      ADC_121.adbuf()[9],
    ",
  0x401702c4u64 => "
      ADC_121.adbuf()[10],
    ",
  0x401702c6u64 => "
      ADC_121.adbuf()[11],
    ",
  0x401702c8u64 => "
      ADC_121.adbuf()[12],
    ",
  0x401702cau64 => "
      ADC_121.adbuf()[13],
    ",
  0x401702ccu64 => "
      ADC_121.adbuf()[14],
    ",
  0x401702ceu64 => "
      ADC_121.adbuf()[15],
    ",
  0x401702d0u64 => "
      ADC_121.adbufen(),
    ",
  0x401702d2u64 => "
      ADC_121.adbufptr(),
    ",
  0x401702e0u64 => "
      ADC_121.adsstr()[0],
    ",
  0x401702e1u64 => "
      ADC_121.adsstr()[1],
    ",
  0x401702e2u64 => "
      ADC_121.adsstr()[2],
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
