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
// Generated from SVD 1.00.01, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:53:56 +0000

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
  0x40000d00u64 => "
      RMPU.mmpuenceu(),
    ",
  0x40000d04u64 => "
      RMPU.mmpuenptceu(),
    ",
  0x40000d08u64 => "
      RMPU.mmpurptceu(),
    ",
  0x40000e00u64 => "
      RMPU.mmpuacceu()[0],
    ",
  0x40000e10u64 => "
      RMPU.mmpuacceu()[1],
    ",
  0x40000e04u64 => "
      RMPU.mmpusceu()[0],
    ",
  0x40000e14u64 => "
      RMPU.mmpusceu()[1],
    ",
  0x40000e08u64 => "
      RMPU.mmpueceu()[0],
    ",
  0x40000e18u64 => "
      RMPU.mmpueceu()[1],
    ",
  0x40002000u64 => "
      SRAM.sramprcr_s(),
    ",
  0x40002008u64 => "
      SRAM.sramwtsc(),
    ",
  0x40002014u64 => "
      SRAM.sramcr1(),
    ",
  0x40002040u64 => "
      SRAM.sramesr(),
    ",
  0x40002048u64 => "
      SRAM.sramesclr(),
    ",
  0x40002058u64 => "
      SRAM.sramear2(),
    ",
  0x40002110u64 => "
      SRAM.stbramcr(),
    ",
  0x40002150u64 => "
      SRAM.stbramear(),
    ",
  0x40003802u64 => "
      BUS.cs0cr(),
    ",
  0x40003812u64 => "
      BUS.cscr()[0],
    ",
  0x40003822u64 => "
      BUS.cscr()[1],
    ",
  0x40003832u64 => "
      BUS.cscr()[2],
    ",
  0x40003842u64 => "
      BUS.cscr()[3],
    ",
  0x40003852u64 => "
      BUS.cscr()[4],
    ",
  0x40003862u64 => "
      BUS.cscr()[5],
    ",
  0x40003872u64 => "
      BUS.cscr()[6],
    ",
  0x40004000u64 => "
      BUS.busoad(),
    ",
  0x40004004u64 => "
      BUS.busoadpt(),
    ",
  0x40004010u64 => "
      BUS.msaoad(),
    ",
  0x40004014u64 => "
      BUS.msapt(),
    ",
  0x40004200u64 => "
      BUS.bussabt1fhbi(),
    ",
  0x40004210u64 => "
      BUS.bussabt0flbi(),
    ",
  0x40004220u64 => "
      BUS.bussabt1s1bi(),
    ",
  0x40004248u64 => "
      BUS.bussabt0stbysbi(),
    ",
  0x40004258u64 => "
      BUS.bussabt0eobi(),
    ",
  0x40004260u64 => "
      BUS.bussabt0pbbi(),
    ",
  0x40004268u64 => "
      BUS.bussabt0pabi(),
    ",
  0x40004270u64 => "
      BUS.bussabt0pibi(),
    ",
  0x40004278u64 => "
      BUS.bussabt0psbi(),
    ",
  0x40004300u64 => "
      BUS.busdivbyp(),
    ",
  0x40004804u64 => "
      BUS.buserrrw()[0],
    ",
  0x40004814u64 => "
      BUS.buserrrw()[1],
    ",
  0x40004830u64 => "
      BUS.buserradd()[0],
    ",
  0x40004840u64 => "
      BUS.buserradd()[1],
    ",
  0x40004900u64 => "
      BUS.bmsaerradd()[0],
    ",
  0x40004910u64 => "
      BUS.bmsaerradd()[1],
    ",
  0x40004904u64 => "
      BUS.bmsaerrrw()[0],
    ",
  0x40004914u64 => "
      BUS.bmsaerrrw()[1],
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
  0x40004a40u64 => "
      BUS.buserrstat()[4],
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
  0x40004a48u64 => "
      BUS.buserrclr()[4],
    ",
  0x40004a90u64 => "
      BUS.buserrstat()[0],
    ",
  0x40004a98u64 => "
      BUS.buserrclr()[0],
    ",
  0x40004b00u64 => "
      BUS.mbwerrstat(),
    ",
  0x40004b08u64 => "
      BUS.mbwerrclr(),
    ",
  0x40004b20u64 => "
      BUS.sbwerrstat(),
    ",
  0x40004b28u64 => "
      BUS.sbwerrclr(),
    ",
  0x40006000u64 => "
      ICU_COMMON.irqcr()[0],
    ",
  0x40006001u64 => "
      ICU_COMMON.irqcr()[1],
    ",
  0x40006002u64 => "
      ICU_COMMON.irqcr()[2],
    ",
  0x40006003u64 => "
      ICU_COMMON.irqcr()[3],
    ",
  0x40006004u64 => "
      ICU_COMMON.irqcr()[4],
    ",
  0x40006005u64 => "
      ICU_COMMON.irqcr()[5],
    ",
  0x40006006u64 => "
      ICU_COMMON.irqcr()[6],
    ",
  0x40006007u64 => "
      ICU_COMMON.irqcr()[7],
    ",
  0x40006008u64 => "
      ICU_COMMON.irqcr()[8],
    ",
  0x40006009u64 => "
      ICU_COMMON.irqcr()[9],
    ",
  0x4000600au64 => "
      ICU_COMMON.irqcr()[10],
    ",
  0x4000600bu64 => "
      ICU_COMMON.irqcr()[11],
    ",
  0x4000600cu64 => "
      ICU_COMMON.irqcr()[12],
    ",
  0x4000600du64 => "
      ICU_COMMON.irqcr()[13],
    ",
  0x4000600eu64 => "
      ICU_COMMON.irqcr()[14],
    ",
  0x4000600fu64 => "
      ICU_COMMON.irqcr()[15],
    ",
  0x40006010u64 => "
      ICU_COMMON.nmicr(),
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
  0x40008050u64 => "
      CPSCU.icusare(),
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
  0x40008170u64 => "
      CPSCU.cpusar(),
    ",
  0x40008180u64 => "
      CPSCU.debugsar(),
    ",
  0x400081a0u64 => "
      CPSCU.dmacchsar(),
    ",
  0x400081f0u64 => "
      CPSCU.dmacchpar(),
    ",
  0x40008404u64 => "
      CPSCU.sramsabar1(),
    ",
  0x40008420u64 => "
      CPSCU.stbramsabar(),
    ",
  0x40008494u64 => "
      CPSCU.stbrampabar_s(),
    ",
  0x40008600u64 => "
      CPSCU.tevtrcr(),
    ",
  0x4000a000u64 => "
      DMAC_00.dmsar(),
    ",
  0x4000a004u64 => "
      DMAC_00.dmdar(),
    ",
  0x4000a008u64 => "
      DMAC_00.dmcra(),
    ",
  0x4000a00cu64 => "
      DMAC_00.dmcrb(),
    ",
  0x4000a010u64 => "
      DMAC_00.dmtmd(),
    ",
  0x4000a013u64 => "
      DMAC_00.dmint(),
    ",
  0x4000a014u64 => "
      DMAC_00.dmamd(),
    ",
  0x4000a018u64 => "
      DMAC_00.dmofr(),
    ",
  0x4000a01cu64 => "
      DMAC_00.dmcnt(),
    ",
  0x4000a01du64 => "
      DMAC_00.dmreq(),
    ",
  0x4000a01eu64 => "
      DMAC_00.dmsts(),
    ",
  0x4000a020u64 => "
      DMAC_00.dmsrr(),
    ",
  0x4000a024u64 => "
      DMAC_00.dmdrr(),
    ",
  0x4000a028u64 => "
      DMAC_00.dmsbs(),
    ",
  0x4000a02cu64 => "
      DMAC_00.dmdbs(),
    ",
  0x4000a030u64 => "
      DMAC_00.dmbwr(),
    ",
  0x4000a800u64 => "
      DMA_0.dmast(),
    ",
  0x4000a840u64 => "
      DMA_0.dmechr(),
    ",
  0x4000a880u64 => "
      DMA_0.delsr()[0],
    ",
  0x4000a884u64 => "
      DMA_0.delsr()[1],
    ",
  0x4000a888u64 => "
      DMA_0.delsr()[2],
    ",
  0x4000a88cu64 => "
      DMA_0.delsr()[3],
    ",
  0x4000a890u64 => "
      DMA_0.delsr()[4],
    ",
  0x4000a894u64 => "
      DMA_0.delsr()[5],
    ",
  0x4000a898u64 => "
      DMA_0.delsr()[6],
    ",
  0x4000a89cu64 => "
      DMA_0.delsr()[7],
    ",
  0x4000ac0cu64 => "
      DTC_0.dtcst(),
    ",
  0x4000ac0eu64 => "
      DTC_0.dtcsts(),
    ",
  0x4000ac10u64 => "
      DTC_0.dtccr_sec(),
    ",
  0x4000ac14u64 => "
      DTC_0.dtcvbr_sec(),
    ",
  0x4000ac20u64 => "
      DTC_0.dtevr(),
    ",
  0x4000c100u64 => "
      ICU.nmier(),
    ",
  0x4000c110u64 => "
      ICU.nmiclr(),
    ",
  0x4000c120u64 => "
      ICU.nmisr(),
    ",
  0x4000c1a0u64 => "
      ICU.wupen0(),
    ",
  0x4000c1a4u64 => "
      ICU.wupen1(),
    ",
  0x4000c300u64 => "
      ICU.ielsr()[0],
    ",
  0x4000c304u64 => "
      ICU.ielsr()[1],
    ",
  0x4000c308u64 => "
      ICU.ielsr()[2],
    ",
  0x4000c30cu64 => "
      ICU.ielsr()[3],
    ",
  0x4000c310u64 => "
      ICU.ielsr()[4],
    ",
  0x4000c314u64 => "
      ICU.ielsr()[5],
    ",
  0x4000c318u64 => "
      ICU.ielsr()[6],
    ",
  0x4000c31cu64 => "
      ICU.ielsr()[7],
    ",
  0x4000c320u64 => "
      ICU.ielsr()[8],
    ",
  0x4000c324u64 => "
      ICU.ielsr()[9],
    ",
  0x4000c328u64 => "
      ICU.ielsr()[10],
    ",
  0x4000c32cu64 => "
      ICU.ielsr()[11],
    ",
  0x4000c330u64 => "
      ICU.ielsr()[12],
    ",
  0x4000c334u64 => "
      ICU.ielsr()[13],
    ",
  0x4000c338u64 => "
      ICU.ielsr()[14],
    ",
  0x4000c33cu64 => "
      ICU.ielsr()[15],
    ",
  0x4000c340u64 => "
      ICU.ielsr()[16],
    ",
  0x4000c344u64 => "
      ICU.ielsr()[17],
    ",
  0x4000c348u64 => "
      ICU.ielsr()[18],
    ",
  0x4000c34cu64 => "
      ICU.ielsr()[19],
    ",
  0x4000c350u64 => "
      ICU.ielsr()[20],
    ",
  0x4000c354u64 => "
      ICU.ielsr()[21],
    ",
  0x4000c358u64 => "
      ICU.ielsr()[22],
    ",
  0x4000c35cu64 => "
      ICU.ielsr()[23],
    ",
  0x4000c360u64 => "
      ICU.ielsr()[24],
    ",
  0x4000c364u64 => "
      ICU.ielsr()[25],
    ",
  0x4000c368u64 => "
      ICU.ielsr()[26],
    ",
  0x4000c36cu64 => "
      ICU.ielsr()[27],
    ",
  0x4000c370u64 => "
      ICU.ielsr()[28],
    ",
  0x4000c374u64 => "
      ICU.ielsr()[29],
    ",
  0x4000c378u64 => "
      ICU.ielsr()[30],
    ",
  0x4000c37cu64 => "
      ICU.ielsr()[31],
    ",
  0x4000c380u64 => "
      ICU.ielsr()[32],
    ",
  0x4000c384u64 => "
      ICU.ielsr()[33],
    ",
  0x4000c388u64 => "
      ICU.ielsr()[34],
    ",
  0x4000c38cu64 => "
      ICU.ielsr()[35],
    ",
  0x4000c390u64 => "
      ICU.ielsr()[36],
    ",
  0x4000c394u64 => "
      ICU.ielsr()[37],
    ",
  0x4000c398u64 => "
      ICU.ielsr()[38],
    ",
  0x4000c39cu64 => "
      ICU.ielsr()[39],
    ",
  0x4000c3a0u64 => "
      ICU.ielsr()[40],
    ",
  0x4000c3a4u64 => "
      ICU.ielsr()[41],
    ",
  0x4000c3a8u64 => "
      ICU.ielsr()[42],
    ",
  0x4000c3acu64 => "
      ICU.ielsr()[43],
    ",
  0x4000c3b0u64 => "
      ICU.ielsr()[44],
    ",
  0x4000c3b4u64 => "
      ICU.ielsr()[45],
    ",
  0x4000c3b8u64 => "
      ICU.ielsr()[46],
    ",
  0x4000c3bcu64 => "
      ICU.ielsr()[47],
    ",
  0x4000c3c0u64 => "
      ICU.ielsr()[48],
    ",
  0x4000c3c4u64 => "
      ICU.ielsr()[49],
    ",
  0x4000c3c8u64 => "
      ICU.ielsr()[50],
    ",
  0x4000c3ccu64 => "
      ICU.ielsr()[51],
    ",
  0x4000c3d0u64 => "
      ICU.ielsr()[52],
    ",
  0x4000c3d4u64 => "
      ICU.ielsr()[53],
    ",
  0x4000c3d8u64 => "
      ICU.ielsr()[54],
    ",
  0x4000c3dcu64 => "
      ICU.ielsr()[55],
    ",
  0x4000c3e0u64 => "
      ICU.ielsr()[56],
    ",
  0x4000c3e4u64 => "
      ICU.ielsr()[57],
    ",
  0x4000c3e8u64 => "
      ICU.ielsr()[58],
    ",
  0x4000c3ecu64 => "
      ICU.ielsr()[59],
    ",
  0x4000c3f0u64 => "
      ICU.ielsr()[60],
    ",
  0x4000c3f4u64 => "
      ICU.ielsr()[61],
    ",
  0x4000c3f8u64 => "
      ICU.ielsr()[62],
    ",
  0x4000c3fcu64 => "
      ICU.ielsr()[63],
    ",
  0x4000c400u64 => "
      ICU.ielsr()[64],
    ",
  0x4000c404u64 => "
      ICU.ielsr()[65],
    ",
  0x4000c408u64 => "
      ICU.ielsr()[66],
    ",
  0x4000c40cu64 => "
      ICU.ielsr()[67],
    ",
  0x4000c410u64 => "
      ICU.ielsr()[68],
    ",
  0x4000c414u64 => "
      ICU.ielsr()[69],
    ",
  0x4000c418u64 => "
      ICU.ielsr()[70],
    ",
  0x4000c41cu64 => "
      ICU.ielsr()[71],
    ",
  0x4000c420u64 => "
      ICU.ielsr()[72],
    ",
  0x4000c424u64 => "
      ICU.ielsr()[73],
    ",
  0x4000c428u64 => "
      ICU.ielsr()[74],
    ",
  0x4000c42cu64 => "
      ICU.ielsr()[75],
    ",
  0x4000c430u64 => "
      ICU.ielsr()[76],
    ",
  0x4000c434u64 => "
      ICU.ielsr()[77],
    ",
  0x4000c438u64 => "
      ICU.ielsr()[78],
    ",
  0x4000c43cu64 => "
      ICU.ielsr()[79],
    ",
  0x4000c440u64 => "
      ICU.ielsr()[80],
    ",
  0x4000c444u64 => "
      ICU.ielsr()[81],
    ",
  0x4000c448u64 => "
      ICU.ielsr()[82],
    ",
  0x4000c44cu64 => "
      ICU.ielsr()[83],
    ",
  0x4000c450u64 => "
      ICU.ielsr()[84],
    ",
  0x4000c454u64 => "
      ICU.ielsr()[85],
    ",
  0x4000c458u64 => "
      ICU.ielsr()[86],
    ",
  0x4000c45cu64 => "
      ICU.ielsr()[87],
    ",
  0x4000c460u64 => "
      ICU.ielsr()[88],
    ",
  0x4000c464u64 => "
      ICU.ielsr()[89],
    ",
  0x4000c468u64 => "
      ICU.ielsr()[90],
    ",
  0x4000c46cu64 => "
      ICU.ielsr()[91],
    ",
  0x4000c470u64 => "
      ICU.ielsr()[92],
    ",
  0x4000c474u64 => "
      ICU.ielsr()[93],
    ",
  0x4000c478u64 => "
      ICU.ielsr()[94],
    ",
  0x4000c47cu64 => "
      ICU.ielsr()[95],
    ",
  0x4000f030u64 => "
      CPU_CTRL.cpulckupcr(),
    ",
  0x4000f400u64 => "
      CPU_CTRL.cpulockcr(),
    ",
  0x4000f840u64 => "
      CPU_CTRL.cpucrpt(),
    ",
  0x40011004u64 => "
      OCD_CPU.mcuctrl(),
    ",
  0x40011100u64 => "
      OCD_CPU.jbmdr(),
    ",
  0x40011120u64 => "
      OCD_CPU.jbrdr(),
    ",
  0x40011130u64 => "
      OCD_CPU.jbtdr(),
    ",
  0x40011140u64 => "
      OCD_CPU.jbstr(),
    ",
  0x40011150u64 => "
      OCD_CPU.jbicr(),
    ",
  0x4001b000u64 => "
      CPU_DBG.dbgstr(),
    ",
  0x4001b010u64 => "
      CPU_DBG.dbgstopcr(),
    ",
  0x4001b020u64 => "
      CPU_DBG.dbgauth0(),
    ",
  0x4001b024u64 => "
      CPU_DBG.dbgauth1(),
    ",
  0x4001b030u64 => "
      CPU_DBG.trportcr(),
    ",
  0x4001b040u64 => "
      CPU_DBG.cachedbgcr(),
    ",
  0x4001b300u64 => "
      CPU_DBG.dbgmocoen(),
    ",
  0x4001b310u64 => "
      CPU_DBG.dbgfclksel(),
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
  0x4001e020u64 => "
      SYSC.sckdivcr(),
    ",
  0x4001e024u64 => "
      SYSC.sckdivcr2(),
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
  0x4001e03fu64 => "
      SYSC.trckcr(),
    ",
  0x4001e040u64 => "
      SYSC.ostdcr(),
    ",
  0x4001e041u64 => "
      SYSC.ostdsr(),
    ",
  0x4001e043u64 => "
      SYSC.oscmonr(),
    ",
  0x4001e048u64 => "
      SYSC.pll2ccr(),
    ",
  0x4001e04au64 => "
      SYSC.pll2cr(),
    ",
  0x4001e04cu64 => "
      SYSC.pllccr2(),
    ",
  0x4001e04eu64 => "
      SYSC.pll2ccr2(),
    ",
  0x4001e054u64 => "
      SYSC.scickdivcr(),
    ",
  0x4001e055u64 => "
      SYSC.scickcr(),
    ",
  0x4001e056u64 => "
      SYSC.spickdivcr(),
    ",
  0x4001e057u64 => "
      SYSC.spickcr(),
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
  0x4001e06eu64 => "
      SYSC.canfdckdivcr(),
    ",
  0x4001e074u64 => "
      SYSC.usbckcr(),
    ",
  0x4001e075u64 => "
      SYSC.octackcr(),
    ",
  0x4001e076u64 => "
      SYSC.canfdckcr(),
    ",
  0x4001e07cu64 => "
      SYSC.moscscr(),
    ",
  0x4001e07du64 => "
      SYSC.hocoscr(),
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
  0x4001e0ccu64 => "
      SYSC.syraccr(),
    ",
  0x4001e0e0u64 => "
      SYSC.pvdcr1()[0],
    ",
  0x4001e0e2u64 => "
      SYSC.pvdcr1()[1],
    ",
  0x4001e0e1u64 => "
      SYSC.pvdsr()[0],
    ",
  0x4001e0e3u64 => "
      SYSC.pvdsr()[1],
    ",
  0x4001e140u64 => "
      SYSC.pdramscr0(),
    ",
  0x4001e142u64 => "
      SYSC.pdramscr1(),
    ",
  0x4001e3b0u64 => "
      SYSC.vbrsabar(),
    ",
  0x4001e3b4u64 => "
      SYSC.vbrpabars(),
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
      SYSC.pvdsar(),
    ",
  0x4001e3d0u64 => "
      SYSC.bbfsar(),
    ",
  0x4001e3e0u64 => "
      SYSC.dpfsar(),
    ",
  0x4001e3e4u64 => "
      SYSC.rscsar(),
    ",
  0x4001e3fau64 => "
      SYSC.prcr_s(),
    ",
  0x4001e400u64 => "
      SYSC.lococr(),
    ",
  0x4001e402u64 => "
      SYSC.locoutcr(),
    ",
  0x4001ea00u64 => "
      SYSC.dpsbycr(),
    ",
  0x4001ea04u64 => "
      SYSC.dpswcr(),
    ",
  0x4001ea08u64 => "
      SYSC.dpsier0(),
    ",
  0x4001ea0cu64 => "
      SYSC.dpsier1(),
    ",
  0x4001ea10u64 => "
      SYSC.dpsier2(),
    ",
  0x4001ea14u64 => "
      SYSC.dpsier3(),
    ",
  0x4001ea18u64 => "
      SYSC.dpsifr0(),
    ",
  0x4001ea1cu64 => "
      SYSC.dpsifr1(),
    ",
  0x4001ea20u64 => "
      SYSC.dpsifr2(),
    ",
  0x4001ea24u64 => "
      SYSC.dpsifr3(),
    ",
  0x4001ea28u64 => "
      SYSC.dpsiegr0(),
    ",
  0x4001ea2cu64 => "
      SYSC.dpsiegr1(),
    ",
  0x4001ea30u64 => "
      SYSC.dpsiegr2(),
    ",
  0x4001ea38u64 => "
      SYSC.syocdcr(),
    ",
  0x4001ea40u64 => "
      SYSC.rstsr0(),
    ",
  0x4001ea44u64 => "
      SYSC.rstsr2(),
    ",
  0x4001ea50u64 => "
      SYSC.momcr(),
    ",
  0x4001ea54u64 => "
      SYSC.fwepror(),
    ",
  0x4001ea58u64 => "
      SYSC.pvdcmpcr()[0],
    ",
  0x4001ea5cu64 => "
      SYSC.pvdcmpcr()[1],
    ",
  0x4001ea70u64 => "
      SYSC.pvdcr0()[0],
    ",
  0x4001ea74u64 => "
      SYSC.pvdcr0()[1],
    ",
  0x4001ea84u64 => "
      SYSC.vbattmnselr(),
    ",
  0x4001ea88u64 => "
      SYSC.vbtbpcr1(),
    ",
  0x4001ea90u64 => "
      SYSC.lpscr(),
    ",
  0x4001ea98u64 => "
      SYSC.sscr1(),
    ",
  0x4001eab0u64 => "
      SYSC.lvocr(),
    ",
  0x4001ead0u64 => "
      SYSC.syrstmsk0(),
    ",
  0x4001ead8u64 => "
      SYSC.syrstmsk2(),
    ",
  0x4001eb04u64 => "
      SYSC.pll1ldocr(),
    ",
  0x4001eb08u64 => "
      SYSC.pll2ldocr(),
    ",
  0x4001eb0cu64 => "
      SYSC.hocoldocr(),
    ",
  0x4001eb20u64 => "
      SYSC.pvdfcr()[0],
    ",
  0x4001eb24u64 => "
      SYSC.pvdfcr()[1],
    ",
  0x4001ec00u64 => "
      SYSC.sosccr(),
    ",
  0x4001ec01u64 => "
      SYSC.somcr(),
    ",
  0x4001ec40u64 => "
      SYSC.vbtber(),
    ",
  0x4001ec45u64 => "
      SYSC.vbtbpcr2(),
    ",
  0x4001ec46u64 => "
      SYSC.vbtbpsr(),
    ",
  0x4001ec48u64 => "
      SYSC.vbtadsr(),
    ",
  0x4001ec49u64 => "
      SYSC.vbtadcr1(),
    ",
  0x4001ec4au64 => "
      SYSC.vbtadcr2(),
    ",
  0x4001ec4cu64 => "
      SYSC.vbtictlr(),
    ",
  0x4001ec4du64 => "
      SYSC.vbtictlr2(),
    ",
  0x4001ec4eu64 => "
      SYSC.vbtimonr(),
    ",
  0x4001ed00u64 => "
      SYSC.vbtbkr()[0],
    ",
  0x4001ed01u64 => "
      SYSC.vbtbkr()[1],
    ",
  0x4001ed02u64 => "
      SYSC.vbtbkr()[2],
    ",
  0x4001ed03u64 => "
      SYSC.vbtbkr()[3],
    ",
  0x4001ed04u64 => "
      SYSC.vbtbkr()[4],
    ",
  0x4001ed05u64 => "
      SYSC.vbtbkr()[5],
    ",
  0x4001ed06u64 => "
      SYSC.vbtbkr()[6],
    ",
  0x4001ed07u64 => "
      SYSC.vbtbkr()[7],
    ",
  0x4001ed08u64 => "
      SYSC.vbtbkr()[8],
    ",
  0x4001ed09u64 => "
      SYSC.vbtbkr()[9],
    ",
  0x4001ed0au64 => "
      SYSC.vbtbkr()[10],
    ",
  0x4001ed0bu64 => "
      SYSC.vbtbkr()[11],
    ",
  0x4001ed0cu64 => "
      SYSC.vbtbkr()[12],
    ",
  0x4001ed0du64 => "
      SYSC.vbtbkr()[13],
    ",
  0x4001ed0eu64 => "
      SYSC.vbtbkr()[14],
    ",
  0x4001ed0fu64 => "
      SYSC.vbtbkr()[15],
    ",
  0x4001ed10u64 => "
      SYSC.vbtbkr()[16],
    ",
  0x4001ed11u64 => "
      SYSC.vbtbkr()[17],
    ",
  0x4001ed12u64 => "
      SYSC.vbtbkr()[18],
    ",
  0x4001ed13u64 => "
      SYSC.vbtbkr()[19],
    ",
  0x4001ed14u64 => "
      SYSC.vbtbkr()[20],
    ",
  0x4001ed15u64 => "
      SYSC.vbtbkr()[21],
    ",
  0x4001ed16u64 => "
      SYSC.vbtbkr()[22],
    ",
  0x4001ed17u64 => "
      SYSC.vbtbkr()[23],
    ",
  0x4001ed18u64 => "
      SYSC.vbtbkr()[24],
    ",
  0x4001ed19u64 => "
      SYSC.vbtbkr()[25],
    ",
  0x4001ed1au64 => "
      SYSC.vbtbkr()[26],
    ",
  0x4001ed1bu64 => "
      SYSC.vbtbkr()[27],
    ",
  0x4001ed1cu64 => "
      SYSC.vbtbkr()[28],
    ",
  0x4001ed1du64 => "
      SYSC.vbtbkr()[29],
    ",
  0x4001ed1eu64 => "
      SYSC.vbtbkr()[30],
    ",
  0x4001ed1fu64 => "
      SYSC.vbtbkr()[31],
    ",
  0x4001ed20u64 => "
      SYSC.vbtbkr()[32],
    ",
  0x4001ed21u64 => "
      SYSC.vbtbkr()[33],
    ",
  0x4001ed22u64 => "
      SYSC.vbtbkr()[34],
    ",
  0x4001ed23u64 => "
      SYSC.vbtbkr()[35],
    ",
  0x4001ed24u64 => "
      SYSC.vbtbkr()[36],
    ",
  0x4001ed25u64 => "
      SYSC.vbtbkr()[37],
    ",
  0x4001ed26u64 => "
      SYSC.vbtbkr()[38],
    ",
  0x4001ed27u64 => "
      SYSC.vbtbkr()[39],
    ",
  0x4001ed28u64 => "
      SYSC.vbtbkr()[40],
    ",
  0x4001ed29u64 => "
      SYSC.vbtbkr()[41],
    ",
  0x4001ed2au64 => "
      SYSC.vbtbkr()[42],
    ",
  0x4001ed2bu64 => "
      SYSC.vbtbkr()[43],
    ",
  0x4001ed2cu64 => "
      SYSC.vbtbkr()[44],
    ",
  0x4001ed2du64 => "
      SYSC.vbtbkr()[45],
    ",
  0x4001ed2eu64 => "
      SYSC.vbtbkr()[46],
    ",
  0x4001ed2fu64 => "
      SYSC.vbtbkr()[47],
    ",
  0x4001ed30u64 => "
      SYSC.vbtbkr()[48],
    ",
  0x4001ed31u64 => "
      SYSC.vbtbkr()[49],
    ",
  0x4001ed32u64 => "
      SYSC.vbtbkr()[50],
    ",
  0x4001ed33u64 => "
      SYSC.vbtbkr()[51],
    ",
  0x4001ed34u64 => "
      SYSC.vbtbkr()[52],
    ",
  0x4001ed35u64 => "
      SYSC.vbtbkr()[53],
    ",
  0x4001ed36u64 => "
      SYSC.vbtbkr()[54],
    ",
  0x4001ed37u64 => "
      SYSC.vbtbkr()[55],
    ",
  0x4001ed38u64 => "
      SYSC.vbtbkr()[56],
    ",
  0x4001ed39u64 => "
      SYSC.vbtbkr()[57],
    ",
  0x4001ed3au64 => "
      SYSC.vbtbkr()[58],
    ",
  0x4001ed3bu64 => "
      SYSC.vbtbkr()[59],
    ",
  0x4001ed3cu64 => "
      SYSC.vbtbkr()[60],
    ",
  0x4001ed3du64 => "
      SYSC.vbtbkr()[61],
    ",
  0x4001ed3eu64 => "
      SYSC.vbtbkr()[62],
    ",
  0x4001ed3fu64 => "
      SYSC.vbtbkr()[63],
    ",
  0x4001ed40u64 => "
      SYSC.vbtbkr()[64],
    ",
  0x4001ed41u64 => "
      SYSC.vbtbkr()[65],
    ",
  0x4001ed42u64 => "
      SYSC.vbtbkr()[66],
    ",
  0x4001ed43u64 => "
      SYSC.vbtbkr()[67],
    ",
  0x4001ed44u64 => "
      SYSC.vbtbkr()[68],
    ",
  0x4001ed45u64 => "
      SYSC.vbtbkr()[69],
    ",
  0x4001ed46u64 => "
      SYSC.vbtbkr()[70],
    ",
  0x4001ed47u64 => "
      SYSC.vbtbkr()[71],
    ",
  0x4001ed48u64 => "
      SYSC.vbtbkr()[72],
    ",
  0x4001ed49u64 => "
      SYSC.vbtbkr()[73],
    ",
  0x4001ed4au64 => "
      SYSC.vbtbkr()[74],
    ",
  0x4001ed4bu64 => "
      SYSC.vbtbkr()[75],
    ",
  0x4001ed4cu64 => "
      SYSC.vbtbkr()[76],
    ",
  0x4001ed4du64 => "
      SYSC.vbtbkr()[77],
    ",
  0x4001ed4eu64 => "
      SYSC.vbtbkr()[78],
    ",
  0x4001ed4fu64 => "
      SYSC.vbtbkr()[79],
    ",
  0x4001ed50u64 => "
      SYSC.vbtbkr()[80],
    ",
  0x4001ed51u64 => "
      SYSC.vbtbkr()[81],
    ",
  0x4001ed52u64 => "
      SYSC.vbtbkr()[82],
    ",
  0x4001ed53u64 => "
      SYSC.vbtbkr()[83],
    ",
  0x4001ed54u64 => "
      SYSC.vbtbkr()[84],
    ",
  0x4001ed55u64 => "
      SYSC.vbtbkr()[85],
    ",
  0x4001ed56u64 => "
      SYSC.vbtbkr()[86],
    ",
  0x4001ed57u64 => "
      SYSC.vbtbkr()[87],
    ",
  0x4001ed58u64 => "
      SYSC.vbtbkr()[88],
    ",
  0x4001ed59u64 => "
      SYSC.vbtbkr()[89],
    ",
  0x4001ed5au64 => "
      SYSC.vbtbkr()[90],
    ",
  0x4001ed5bu64 => "
      SYSC.vbtbkr()[91],
    ",
  0x4001ed5cu64 => "
      SYSC.vbtbkr()[92],
    ",
  0x4001ed5du64 => "
      SYSC.vbtbkr()[93],
    ",
  0x4001ed5eu64 => "
      SYSC.vbtbkr()[94],
    ",
  0x4001ed5fu64 => "
      SYSC.vbtbkr()[95],
    ",
  0x4001ed60u64 => "
      SYSC.vbtbkr()[96],
    ",
  0x4001ed61u64 => "
      SYSC.vbtbkr()[97],
    ",
  0x4001ed62u64 => "
      SYSC.vbtbkr()[98],
    ",
  0x4001ed63u64 => "
      SYSC.vbtbkr()[99],
    ",
  0x4001ed64u64 => "
      SYSC.vbtbkr()[100],
    ",
  0x4001ed65u64 => "
      SYSC.vbtbkr()[101],
    ",
  0x4001ed66u64 => "
      SYSC.vbtbkr()[102],
    ",
  0x4001ed67u64 => "
      SYSC.vbtbkr()[103],
    ",
  0x4001ed68u64 => "
      SYSC.vbtbkr()[104],
    ",
  0x4001ed69u64 => "
      SYSC.vbtbkr()[105],
    ",
  0x4001ed6au64 => "
      SYSC.vbtbkr()[106],
    ",
  0x4001ed6bu64 => "
      SYSC.vbtbkr()[107],
    ",
  0x4001ed6cu64 => "
      SYSC.vbtbkr()[108],
    ",
  0x4001ed6du64 => "
      SYSC.vbtbkr()[109],
    ",
  0x4001ed6eu64 => "
      SYSC.vbtbkr()[110],
    ",
  0x4001ed6fu64 => "
      SYSC.vbtbkr()[111],
    ",
  0x4001ed70u64 => "
      SYSC.vbtbkr()[112],
    ",
  0x4001ed71u64 => "
      SYSC.vbtbkr()[113],
    ",
  0x4001ed72u64 => "
      SYSC.vbtbkr()[114],
    ",
  0x4001ed73u64 => "
      SYSC.vbtbkr()[115],
    ",
  0x4001ed74u64 => "
      SYSC.vbtbkr()[116],
    ",
  0x4001ed75u64 => "
      SYSC.vbtbkr()[117],
    ",
  0x4001ed76u64 => "
      SYSC.vbtbkr()[118],
    ",
  0x4001ed77u64 => "
      SYSC.vbtbkr()[119],
    ",
  0x4001ed78u64 => "
      SYSC.vbtbkr()[120],
    ",
  0x4001ed79u64 => "
      SYSC.vbtbkr()[121],
    ",
  0x4001ed7au64 => "
      SYSC.vbtbkr()[122],
    ",
  0x4001ed7bu64 => "
      SYSC.vbtbkr()[123],
    ",
  0x4001ed7cu64 => "
      SYSC.vbtbkr()[124],
    ",
  0x4001ed7du64 => "
      SYSC.vbtbkr()[125],
    ",
  0x4001ed7eu64 => "
      SYSC.vbtbkr()[126],
    ",
  0x4001ed7fu64 => "
      SYSC.vbtbkr()[127],
    ",
  0x4011b17cu64 => "
      TSD.tscdr(),
    ",
  0x4011c040u64 => "
      FLAD.fckmhz(),
    ",
  0x4011e010u64 => "
      FACI.fastat(),
    ",
  0x4011e014u64 => "
      FACI.faeint(),
    ",
  0x4011e018u64 => "
      FACI.frdyie(),
    ",
  0x4011e030u64 => "
      FACI.fsaddr(),
    ",
  0x4011e034u64 => "
      FACI.feaddr(),
    ",
  0x4011e044u64 => "
      FACI.fmeprot(),
    ",
  0x4011e048u64 => "
      FACI.fcntselr(),
    ",
  0x4011e04cu64 => "
      FACI.fcntdatar()[0],
    ",
  0x4011e050u64 => "
      FACI.fcntdatar()[1],
    ",
  0x4011e078u64 => "
      FACI.fbprot0(),
    ",
  0x4011e07cu64 => "
      FACI.fbprot1(),
    ",
  0x4011e080u64 => "
      FACI.fstatr(),
    ",
  0x4011e084u64 => "
      FACI.fentryr(),
    ",
  0x4011e08cu64 => "
      FACI.fsuinitr(),
    ",
  0x4011e0a0u64 => "
      FACI.fcmdr(),
    ",
  0x4011e0d0u64 => "
      FACI.fbccnt(),
    ",
  0x4011e0d4u64 => "
      FACI.fbcstat(),
    ",
  0x4011e0d8u64 => "
      FACI.fpsaddr(),
    ",
  0x4011e0dcu64 => "
      FACI.fsuasmon(),
    ",
  0x4011e0e0u64 => "
      FACI.fcpsr(),
    ",
  0x4011e0e4u64 => "
      FACI.fpckar(),
    ",
  0x4011e0e8u64 => "
      FACI.fsuacr(),
    ",
  0x40201000u64 => "
      ELC.elcr(),
    ",
  0x40201004u64 => "
      ELC.elsegr()[0],
    ",
  0x40201008u64 => "
      ELC.elsegr()[1],
    ",
  0x40201020u64 => "
      ELC.elsr()[0],
    ",
  0x40201024u64 => "
      ELC.elsr()[1],
    ",
  0x40201028u64 => "
      ELC.elsr()[2],
    ",
  0x4020102cu64 => "
      ELC.elsr()[3],
    ",
  0x40201030u64 => "
      ELC.elsr()[4],
    ",
  0x40201034u64 => "
      ELC.elsr()[5],
    ",
  0x40201038u64 => "
      ELC.elsr()[6],
    ",
  0x4020103cu64 => "
      ELC.elsr()[7],
    ",
  0x40201040u64 => "
      ELC.elsr()[8],
    ",
  0x40201044u64 => "
      ELC.elsr()[9],
    ",
  0x40201048u64 => "
      ELC.elsr()[10],
    ",
  0x4020104cu64 => "
      ELC.elsr()[11],
    ",
  0x40201050u64 => "
      ELC.elsr()[12],
    ",
  0x40201054u64 => "
      ELC.elsr()[13],
    ",
  0x40201058u64 => "
      ELC.elsr()[14],
    ",
  0x4020105cu64 => "
      ELC.elsr()[15],
    ",
  0x40201060u64 => "
      ELC.elsr()[16],
    ",
  0x40201064u64 => "
      ELC.elsr()[17],
    ",
  0x40201098u64 => "
      ELC.elsr30(),
    ",
  0x402010e0u64 => "
      ELC.elcsara(),
    ",
  0x402010e4u64 => "
      ELC.elcsarb(),
    ",
  0x402010f0u64 => "
      ELC.elcpara(),
    ",
  0x402010f4u64 => "
      ELC.elcparb(),
    ",
  0x40202000u64 => "
      RTC.r64cnt(),
    ",
  0x40202002u64 => "
      RTC.bcnt()[0],
      RTC.rseccnt(),
    ",
  0x40202004u64 => "
      RTC.bcnt()[1],
      RTC.rmincnt(),
    ",
  0x40202006u64 => "
      RTC.bcnt()[2],
      RTC.rhrcnt(),
    ",
  0x40202008u64 => "
      RTC.bcnt()[3],
      RTC.rwkcnt(),
    ",
  0x4020200au64 => "
      RTC.rdaycnt(),
    ",
  0x4020200cu64 => "
      RTC.rmoncnt(),
    ",
  0x4020200eu64 => "
      RTC.ryrcnt(),
    ",
  0x40202010u64 => "
      RTC.bcntar()[0],
      RTC.rsecar(),
    ",
  0x40202012u64 => "
      RTC.bcntar()[1],
      RTC.rminar(),
    ",
  0x40202014u64 => "
      RTC.bcntar()[2],
      RTC.rhrar(),
    ",
  0x40202016u64 => "
      RTC.bcntar()[3],
      RTC.rwkar(),
    ",
  0x40202018u64 => "
      RTC.bcntaer()[0],
      RTC.rdayar(),
    ",
  0x4020201au64 => "
      RTC.bcntaer()[1],
      RTC.rmonar(),
    ",
  0x4020201cu64 => "
      RTC.bcntaer()[2],
      RTC.ryrar(),
    ",
  0x4020201eu64 => "
      RTC.bcntaer()[3],
      RTC.ryraren(),
    ",
  0x40202022u64 => "
      RTC.rcr1(),
    ",
  0x40202024u64 => "
      RTC.rcr2(),
      RTC.rcr2_bcnt(),
    ",
  0x40202028u64 => "
      RTC.rcr4(),
    ",
  0x4020202au64 => "
      RTC.rfrh(),
    ",
  0x4020202cu64 => "
      RTC.rfrl(),
    ",
  0x4020202eu64 => "
      RTC.radj(),
    ",
  0x40202040u64 => "
      RTC.rtccr()[0],
    ",
  0x40202042u64 => "
      RTC.rtccr()[1],
    ",
  0x40202044u64 => "
      RTC.rtccr()[2],
    ",
  0x40202052u64 => "
      RTC.bcnt0cp()[0],
      RTC.rseccp()[0],
    ",
  0x40202062u64 => "
      RTC.bcnt0cp()[1],
      RTC.rseccp()[1],
    ",
  0x40202072u64 => "
      RTC.bcnt0cp()[2],
      RTC.rseccp()[2],
    ",
  0x40202054u64 => "
      RTC.bcnt1cp()[0],
      RTC.rmincp()[0],
    ",
  0x40202064u64 => "
      RTC.bcnt1cp()[1],
      RTC.rmincp()[1],
    ",
  0x40202074u64 => "
      RTC.bcnt1cp()[2],
      RTC.rmincp()[2],
    ",
  0x40202056u64 => "
      RTC.bcnt2cp()[0],
      RTC.rhrcp()[0],
    ",
  0x40202066u64 => "
      RTC.bcnt2cp()[1],
      RTC.rhrcp()[1],
    ",
  0x40202076u64 => "
      RTC.bcnt2cp()[2],
      RTC.rhrcp()[2],
    ",
  0x4020205au64 => "
      RTC.bcnt3cp()[0],
      RTC.rdaycp()[0],
    ",
  0x4020206au64 => "
      RTC.bcnt3cp()[1],
      RTC.rdaycp()[1],
    ",
  0x4020207au64 => "
      RTC.bcnt3cp()[2],
      RTC.rdaycp()[2],
    ",
  0x4020205cu64 => "
      RTC.rmoncp()[0],
    ",
  0x4020206cu64 => "
      RTC.rmoncp()[1],
    ",
  0x4020207cu64 => "
      RTC.rmoncp()[2],
    ",
  0x40202200u64 => "
      IWDT.iwdtrr(),
    ",
  0x40202202u64 => "
      IWDT.iwdtcr(),
    ",
  0x40202204u64 => "
      IWDT.iwdtsr(),
    ",
  0x40202206u64 => "
      IWDT.iwdtrcr(),
    ",
  0x40202208u64 => "
      IWDT.iwdtcstpr(),
    ",
  0x40202400u64 => "
      CAC.cacr0(),
    ",
  0x40202401u64 => "
      CAC.cacr1(),
    ",
  0x40202402u64 => "
      CAC.cacr2(),
    ",
  0x40202403u64 => "
      CAC.caicr(),
    ",
  0x40202404u64 => "
      CAC.castr(),
    ",
  0x40202406u64 => "
      CAC.caulvr(),
    ",
  0x40202408u64 => "
      CAC.callvr(),
    ",
  0x4020240au64 => "
      CAC.cacntbr(),
    ",
  0x40202600u64 => "
      WDT_0.wdtrr(),
    ",
  0x40202602u64 => "
      WDT_0.wdtcr(),
    ",
  0x40202604u64 => "
      WDT_0.wdtsr(),
    ",
  0x40202606u64 => "
      WDT_0.wdtrcr(),
    ",
  0x40202608u64 => "
      WDT_0.wdtcstpr(),
    ",
  0x40203000u64 => "
      MSTP.mstpcra(),
    ",
  0x40203004u64 => "
      MSTP.mstpcrb(),
    ",
  0x40203008u64 => "
      MSTP.mstpcrc(),
    ",
  0x4020300cu64 => "
      MSTP.mstpcrd(),
    ",
  0x40203010u64 => "
      MSTP.mstpcre(),
    ",
  0x40204004u64 => "
      PSCU.psarb(),
    ",
  0x40204008u64 => "
      PSCU.psarc(),
    ",
  0x4020400cu64 => "
      PSCU.psard(),
    ",
  0x40204010u64 => "
      PSCU.psare(),
    ",
  0x40204014u64 => "
      PSCU.mssar(),
    ",
  0x4020401cu64 => "
      PSCU.pparb(),
    ",
  0x40204020u64 => "
      PSCU.pparc(),
    ",
  0x40204024u64 => "
      PSCU.ppard(),
    ",
  0x40204028u64 => "
      PSCU.ppare(),
    ",
  0x4020402cu64 => "
      PSCU.mspar(),
    ",
  0x40204030u64 => "
      PSCU.cfsamona(),
    ",
  0x40204034u64 => "
      PSCU.dfsamon(),
    ",
  0x40204038u64 => "
      PSCU.dlmmon(),
    ",
  0x40212000u64 => "
      POEG.poegga(),
    ",
  0x40212100u64 => "
      POEG.poeggb(),
    ",
  0x40212200u64 => "
      POEG.poeggc(),
    ",
  0x40212300u64 => "
      POEG.poeggd(),
    ",
  0x40220000u64 => "
      ULPT_0.ulptcnt(),
    ",
  0x40220004u64 => "
      ULPT_0.ulptcma(),
    ",
  0x40220008u64 => "
      ULPT_0.ulptcmb(),
    ",
  0x4022000cu64 => "
      ULPT_0.ulptcr(),
    ",
  0x4022000du64 => "
      ULPT_0.ulptmr1(),
    ",
  0x4022000eu64 => "
      ULPT_0.ulptmr2(),
    ",
  0x4022000fu64 => "
      ULPT_0.ulptmr3(),
    ",
  0x40220010u64 => "
      ULPT_0.ulptioc(),
    ",
  0x40220011u64 => "
      ULPT_0.ulptisr(),
    ",
  0x40220012u64 => "
      ULPT_0.ulptcmsr(),
    ",
  0x40221000u64 => "
      AGT_0.agt(),
    ",
  0x40221002u64 => "
      AGT_0.agtcma(),
    ",
  0x40221004u64 => "
      AGT_0.agtcmb(),
    ",
  0x40221008u64 => "
      AGT_0.agtcr(),
    ",
  0x40221009u64 => "
      AGT_0.agtmr1(),
    ",
  0x4022100au64 => "
      AGT_0.agtmr2(),
    ",
  0x4022100cu64 => "
      AGT_0.agtioc(),
    ",
  0x4022100du64 => "
      AGT_0.agtisr(),
    ",
  0x4022100eu64 => "
      AGT_0.agtcmsr(),
    ",
  0x4022100fu64 => "
      AGT_0.agtiosel(),
    ",
  0x40235000u64 => "
      TSN.tscr(),
    ",
  0x40236000u64 => "
      ACMPHS_0.cmpctl(),
    ",
  0x40236004u64 => "
      ACMPHS_0.cmpsel0(),
    ",
  0x40236008u64 => "
      ACMPHS_0.cmpsel1(),
    ",
  0x4023600cu64 => "
      ACMPHS_0.cmpmon(),
    ",
  0x40236010u64 => "
      ACMPHS_0.cpioc(),
    ",
  0x40236040u64 => "
      ACMPHS_0.cpintctl(),
    ",
  0x40236044u64 => "
      ACMPHS_0.cpmskctl(),
    ",
  0x40250000u64 => "
      USBFS.syscfg(),
    ",
  0x40250004u64 => "
      USBFS.syssts0(),
    ",
  0x40250008u64 => "
      USBFS.dvstctr0(),
    ",
  0x40250014u64 => "
      USBFS.cfifo(),
      USBFS.cfifol(),
    ",
  0x40250018u64 => "
      USBFS.dfifo()[0],
      USBFS.dfifol()[0],
    ",
  0x4025001cu64 => "
      USBFS.dfifo()[1],
      USBFS.dfifol()[1],
    ",
  0x40250020u64 => "
      USBFS.cfifosel(),
    ",
  0x40250022u64 => "
      USBFS.cfifoctr(),
    ",
  0x40250028u64 => "
      USBFS.dfifosel()[0],
    ",
  0x4025002cu64 => "
      USBFS.dfifosel()[1],
    ",
  0x4025002au64 => "
      USBFS.dfifoctr()[0],
    ",
  0x4025002eu64 => "
      USBFS.dfifoctr()[1],
    ",
  0x40250030u64 => "
      USBFS.intenb0(),
    ",
  0x40250032u64 => "
      USBFS.intenb1(),
    ",
  0x40250036u64 => "
      USBFS.brdyenb(),
    ",
  0x40250038u64 => "
      USBFS.nrdyenb(),
    ",
  0x4025003au64 => "
      USBFS.bempenb(),
    ",
  0x4025003cu64 => "
      USBFS.sofcfg(),
    ",
  0x40250040u64 => "
      USBFS.intsts0(),
    ",
  0x40250042u64 => "
      USBFS.intsts1(),
    ",
  0x40250046u64 => "
      USBFS.brdysts(),
    ",
  0x40250048u64 => "
      USBFS.nrdysts(),
    ",
  0x4025004au64 => "
      USBFS.bempsts(),
    ",
  0x4025004cu64 => "
      USBFS.frmnum(),
    ",
  0x4025004eu64 => "
      USBFS.dvchgr(),
    ",
  0x40250050u64 => "
      USBFS.usbaddr(),
    ",
  0x40250054u64 => "
      USBFS.usbreq(),
    ",
  0x40250056u64 => "
      USBFS.usbval(),
    ",
  0x40250058u64 => "
      USBFS.usbindx(),
    ",
  0x4025005au64 => "
      USBFS.usbleng(),
    ",
  0x4025005cu64 => "
      USBFS.dcpcfg(),
    ",
  0x4025005eu64 => "
      USBFS.dcpmaxp(),
    ",
  0x40250060u64 => "
      USBFS.dcpctr(),
    ",
  0x40250064u64 => "
      USBFS.pipesel(),
    ",
  0x40250068u64 => "
      USBFS.pipecfg(),
    ",
  0x4025006cu64 => "
      USBFS.pipemaxp(),
    ",
  0x4025006eu64 => "
      USBFS.pipeperi(),
    ",
  0x40250078u64 => "
      USBFS.pipectr()[4],
    ",
  0x4025007au64 => "
      USBFS.pipectr()[0],
    ",
  0x4025007cu64 => "
      USBFS.pipectr()[1],
    ",
  0x4025007eu64 => "
      USBFS.pipectr()[2],
    ",
  0x40250080u64 => "
      USBFS.pipectr()[3],
    ",
  0x40250090u64 => "
      USBFS.pipetre()[0],
    ",
  0x40250094u64 => "
      USBFS.pipetre()[1],
    ",
  0x40250098u64 => "
      USBFS.pipetre()[2],
    ",
  0x4025009cu64 => "
      USBFS.pipetre()[3],
    ",
  0x402500a0u64 => "
      USBFS.pipetre()[4],
    ",
  0x40250092u64 => "
      USBFS.pipetrn()[0],
    ",
  0x40250096u64 => "
      USBFS.pipetrn()[1],
    ",
  0x4025009au64 => "
      USBFS.pipetrn()[2],
    ",
  0x4025009eu64 => "
      USBFS.pipetrn()[3],
    ",
  0x402500a2u64 => "
      USBFS.pipetrn()[4],
    ",
  0x402500d0u64 => "
      USBFS.devadd()[0],
    ",
  0x402500d2u64 => "
      USBFS.devadd()[1],
    ",
  0x402500d4u64 => "
      USBFS.devadd()[2],
    ",
  0x402500d6u64 => "
      USBFS.devadd()[3],
    ",
  0x402500d8u64 => "
      USBFS.devadd()[4],
    ",
  0x402500dau64 => "
      USBFS.devadd()[5],
    ",
  0x40250400u64 => "
      USBFS.dpusr0r(),
    ",
  0x40250404u64 => "
      USBFS.dpusr1r(),
    ",
  0x4025d000u64 => "
      SSIE_0.ssicr(),
    ",
  0x4025d004u64 => "
      SSIE_0.ssisr(),
    ",
  0x4025d010u64 => "
      SSIE_0.ssifcr(),
    ",
  0x4025d014u64 => "
      SSIE_0.ssifsr(),
    ",
  0x4025d018u64 => "
      SSIE_0.ssiftdr(),
    ",
  0x4025d01cu64 => "
      SSIE_0.ssifrdr(),
    ",
  0x4025d020u64 => "
      SSIE_0.ssiofr(),
    ",
  0x4025d024u64 => "
      SSIE_0.ssiscr(),
    ",
  0x4025e000u64 => "
      IIC_0.iccr1(),
    ",
  0x4025e001u64 => "
      IIC_0.iccr2(),
    ",
  0x4025e002u64 => "
      IIC_0.icmr1(),
    ",
  0x4025e003u64 => "
      IIC_0.icmr2(),
    ",
  0x4025e004u64 => "
      IIC_0.icmr3(),
    ",
  0x4025e005u64 => "
      IIC_0.icfer(),
    ",
  0x4025e006u64 => "
      IIC_0.icser(),
    ",
  0x4025e007u64 => "
      IIC_0.icier(),
    ",
  0x4025e008u64 => "
      IIC_0.icsr1(),
    ",
  0x4025e009u64 => "
      IIC_0.icsr2(),
    ",
  0x4025e00au64 => "
      IIC_0.sarl()[0],
    ",
  0x4025e00cu64 => "
      IIC_0.sarl()[1],
    ",
  0x4025e00eu64 => "
      IIC_0.sarl()[2],
    ",
  0x4025e00bu64 => "
      IIC_0.saru()[0],
    ",
  0x4025e00du64 => "
      IIC_0.saru()[1],
    ",
  0x4025e00fu64 => "
      IIC_0.saru()[2],
    ",
  0x4025e010u64 => "
      IIC_0.icbrl(),
    ",
  0x4025e011u64 => "
      IIC_0.icbrh(),
    ",
  0x4025e012u64 => "
      IIC_0.icdrt(),
    ",
  0x4025e013u64 => "
      IIC_0.icdrr(),
    ",
  0x4025e016u64 => "
      IIC_0_WU.icwur(),
    ",
  0x4025e017u64 => "
      IIC_0_WU.icwur2(),
    ",
  0x40268000u64 => "
      OSPI_0_B.wrapcfg(),
    ",
  0x40268004u64 => "
      OSPI_0_B.comcfg(),
    ",
  0x40268008u64 => "
      OSPI_0_B.bmcfgch()[0],
    ",
  0x4026800cu64 => "
      OSPI_0_B.bmcfgch()[1],
    ",
  0x40268010u64 => "
      OSPI_0_B.cmcfg0cs()[0],
    ",
  0x40268020u64 => "
      OSPI_0_B.cmcfg0cs()[1],
    ",
  0x40268014u64 => "
      OSPI_0_B.cmcfg1cs()[0],
    ",
  0x40268024u64 => "
      OSPI_0_B.cmcfg1cs()[1],
    ",
  0x40268018u64 => "
      OSPI_0_B.cmcfg2cs()[0],
    ",
  0x40268028u64 => "
      OSPI_0_B.cmcfg2cs()[1],
    ",
  0x40268050u64 => "
      OSPI_0_B.liocfgcs()[0],
    ",
  0x40268054u64 => "
      OSPI_0_B.liocfgcs()[1],
    ",
  0x40268060u64 => "
      OSPI_0_B.bmctl0(),
    ",
  0x40268064u64 => "
      OSPI_0_B.bmctl1(),
    ",
  0x40268068u64 => "
      OSPI_0_B.cmctlch()[0],
    ",
  0x4026806cu64 => "
      OSPI_0_B.cmctlch()[1],
    ",
  0x40268070u64 => "
      OSPI_0_B.cdctl0(),
    ",
  0x40268074u64 => "
      OSPI_0_B.cdctl1(),
    ",
  0x40268078u64 => "
      OSPI_0_B.cdctl2(),
    ",
  0x40268080u64 => "
      OSPI_0_B.cdtbuf()[0],
    ",
  0x40268090u64 => "
      OSPI_0_B.cdtbuf()[1],
    ",
  0x402680a0u64 => "
      OSPI_0_B.cdtbuf()[2],
    ",
  0x402680b0u64 => "
      OSPI_0_B.cdtbuf()[3],
    ",
  0x40268084u64 => "
      OSPI_0_B.cdabuf()[0],
    ",
  0x40268094u64 => "
      OSPI_0_B.cdabuf()[1],
    ",
  0x402680a4u64 => "
      OSPI_0_B.cdabuf()[2],
    ",
  0x402680b4u64 => "
      OSPI_0_B.cdabuf()[3],
    ",
  0x40268088u64 => "
      OSPI_0_B.cdd0buf()[0],
    ",
  0x40268098u64 => "
      OSPI_0_B.cdd0buf()[1],
    ",
  0x402680a8u64 => "
      OSPI_0_B.cdd0buf()[2],
    ",
  0x402680b8u64 => "
      OSPI_0_B.cdd0buf()[3],
    ",
  0x4026808cu64 => "
      OSPI_0_B.cdd1buf()[0],
    ",
  0x4026809cu64 => "
      OSPI_0_B.cdd1buf()[1],
    ",
  0x402680acu64 => "
      OSPI_0_B.cdd1buf()[2],
    ",
  0x402680bcu64 => "
      OSPI_0_B.cdd1buf()[3],
    ",
  0x40268100u64 => "
      OSPI_0_B.lpctl0(),
    ",
  0x40268104u64 => "
      OSPI_0_B.lpctl1(),
    ",
  0x40268108u64 => "
      OSPI_0_B.lioctl(),
    ",
  0x40268130u64 => "
      OSPI_0_B.ccctl0cs()[0],
    ",
  0x40268150u64 => "
      OSPI_0_B.ccctl0cs()[1],
    ",
  0x40268134u64 => "
      OSPI_0_B.ccctl1cs()[0],
    ",
  0x40268154u64 => "
      OSPI_0_B.ccctl1cs()[1],
    ",
  0x40268138u64 => "
      OSPI_0_B.ccctl2cs()[0],
    ",
  0x40268158u64 => "
      OSPI_0_B.ccctl2cs()[1],
    ",
  0x4026813cu64 => "
      OSPI_0_B.ccctl3cs()[0],
    ",
  0x4026815cu64 => "
      OSPI_0_B.ccctl3cs()[1],
    ",
  0x40268140u64 => "
      OSPI_0_B.ccctl4cs()[0],
    ",
  0x40268160u64 => "
      OSPI_0_B.ccctl4cs()[1],
    ",
  0x40268144u64 => "
      OSPI_0_B.ccctl5cs()[0],
    ",
  0x40268164u64 => "
      OSPI_0_B.ccctl5cs()[1],
    ",
  0x40268148u64 => "
      OSPI_0_B.ccctl6cs()[0],
    ",
  0x40268168u64 => "
      OSPI_0_B.ccctl6cs()[1],
    ",
  0x4026814cu64 => "
      OSPI_0_B.ccctl7cs()[0],
    ",
  0x4026816cu64 => "
      OSPI_0_B.ccctl7cs()[1],
    ",
  0x40268184u64 => "
      OSPI_0_B.comstt(),
    ",
  0x40268188u64 => "
      OSPI_0_B.casttcs()[0],
    ",
  0x4026818cu64 => "
      OSPI_0_B.casttcs()[1],
    ",
  0x40268190u64 => "
      OSPI_0_B.ints(),
    ",
  0x40268194u64 => "
      OSPI_0_B.intc(),
    ",
  0x40268198u64 => "
      OSPI_0_B.inte(),
    ",
  0x40310000u64 => "
      CRC.crccr0(),
    ",
  0x40310001u64 => "
      CRC.crccr1(),
    ",
  0x40310004u64 => "
      CRC.crcdir(),
      CRC.crcdir_by(),
    ",
  0x40310008u64 => "
      CRC.crcdor(),
      CRC.crcdor_ha(),
      CRC.crcdor_by(),
    ",
  0x4031000cu64 => "
      CRC.crcsar(),
    ",
  0x40311000u64 => "
      DOC_B.docr(),
    ",
  0x40311004u64 => "
      DOC_B.dosr(),
    ",
  0x40311008u64 => "
      DOC_B.doscr(),
    ",
  0x4031100cu64 => "
      DOC_B.dodir(),
    ",
  0x40311010u64 => "
      DOC_B.dodsr0(),
    ",
  0x40311014u64 => "
      DOC_B.dodsr1(),
    ",
  0x40322000u64 => "
      GPT_320.gtwp(),
    ",
  0x40322004u64 => "
      GPT_320.gtstr(),
    ",
  0x40322008u64 => "
      GPT_320.gtstp(),
    ",
  0x4032200cu64 => "
      GPT_320.gtclr(),
    ",
  0x40322010u64 => "
      GPT_320.gtssr(),
    ",
  0x40322014u64 => "
      GPT_320.gtpsr(),
    ",
  0x40322018u64 => "
      GPT_320.gtcsr(),
    ",
  0x4032201cu64 => "
      GPT_320.gtupsr(),
    ",
  0x40322020u64 => "
      GPT_320.gtdnsr(),
    ",
  0x40322024u64 => "
      GPT_320.gticasr(),
    ",
  0x40322028u64 => "
      GPT_320.gticbsr(),
    ",
  0x4032202cu64 => "
      GPT_320.gtcr(),
    ",
  0x40322030u64 => "
      GPT_320.gtuddtyc(),
    ",
  0x40322034u64 => "
      GPT_320.gtior(),
    ",
  0x40322038u64 => "
      GPT_320.gtintad(),
    ",
  0x4032203cu64 => "
      GPT_320.gtst(),
    ",
  0x40322040u64 => "
      GPT_320.gtber(),
    ",
  0x40322048u64 => "
      GPT_320.gtcnt(),
    ",
  0x4032204cu64 => "
      GPT_320.gtccra(),
    ",
  0x40322050u64 => "
      GPT_320.gtccrb(),
    ",
  0x40322054u64 => "
      GPT_320.gtccrc(),
    ",
  0x40322058u64 => "
      GPT_320.gtccre(),
    ",
  0x4032205cu64 => "
      GPT_320.gtccrd(),
    ",
  0x40322060u64 => "
      GPT_320.gtccrf(),
    ",
  0x40322064u64 => "
      GPT_320.gtpr(),
    ",
  0x40322068u64 => "
      GPT_320.gtpbr(),
    ",
  0x40322070u64 => "
      GPT_320.gtadtra(),
    ",
  0x40322074u64 => "
      GPT_320.gtadtbra(),
    ",
  0x40322078u64 => "
      GPT_320.gtadtdbra(),
    ",
  0x4032207cu64 => "
      GPT_320.gtadtrb(),
    ",
  0x40322080u64 => "
      GPT_320.gtadtbrb(),
    ",
  0x40322084u64 => "
      GPT_320.gtadtdbrb(),
    ",
  0x40322088u64 => "
      GPT_320.gtdtcr(),
    ",
  0x4032208cu64 => "
      GPT_320.gtdvu(),
    ",
  0x403220a4u64 => "
      GPT_320.gtadsmr(),
    ",
  0x403220b8u64 => "
      GPT_320.gticlf(),
    ",
  0x403220bcu64 => "
      GPT_320.gtpc(),
    ",
  0x403220d0u64 => "
      GPT_320.gtsecsr(),
    ",
  0x403220d4u64 => "
      GPT_320.gtsecr(),
    ",
  0x40322804u64 => "
      GPT_168.gtstr(),
    ",
  0x40322808u64 => "
      GPT_168.gtstp(),
    ",
  0x4032280cu64 => "
      GPT_168.gtclr(),
    ",
  0x40322810u64 => "
      GPT_168.gtssr(),
    ",
  0x40322814u64 => "
      GPT_168.gtpsr(),
    ",
  0x40322818u64 => "
      GPT_168.gtcsr(),
    ",
  0x4032281cu64 => "
      GPT_168.gtupsr(),
    ",
  0x40322820u64 => "
      GPT_168.gtdnsr(),
    ",
  0x40322824u64 => "
      GPT_168.gticasr(),
    ",
  0x40322828u64 => "
      GPT_168.gticbsr(),
    ",
  0x4032282cu64 => "
      GPT_168.gtcr(),
    ",
  0x40322838u64 => "
      GPT_168.gtintad(),
    ",
  0x40322870u64 => "
      GPT_168.gtadtra(),
    ",
  0x40322874u64 => "
      GPT_168.gtadtbra(),
    ",
  0x40322878u64 => "
      GPT_168.gtadtdbra(),
    ",
  0x4032287cu64 => "
      GPT_168.gtadtrb(),
    ",
  0x40322880u64 => "
      GPT_168.gtadtbrb(),
    ",
  0x40322884u64 => "
      GPT_168.gtadtdbrb(),
    ",
  0x403228a4u64 => "
      GPT_168.gtadsmr(),
    ",
  0x403228b8u64 => "
      GPT_168.gticlf(),
    ",
  0x40322a00u64 => "
      GPT_1610.gtwp(),
    ",
  0x40322a30u64 => "
      GPT_1610.gtuddtyc(),
    ",
  0x40322a34u64 => "
      GPT_1610.gtior(),
    ",
  0x40322a3cu64 => "
      GPT_1610.gtst(),
    ",
  0x40322a40u64 => "
      GPT_1610.gtber(),
    ",
  0x40322a48u64 => "
      GPT_1610.gtcnt(),
    ",
  0x40322a4cu64 => "
      GPT_1610.gtccra(),
    ",
  0x40322a50u64 => "
      GPT_1610.gtccrb(),
    ",
  0x40322a54u64 => "
      GPT_1610.gtccrc(),
    ",
  0x40322a58u64 => "
      GPT_1610.gtccre(),
    ",
  0x40322a5cu64 => "
      GPT_1610.gtccrd(),
    ",
  0x40322a60u64 => "
      GPT_1610.gtccrf(),
    ",
  0x40322a64u64 => "
      GPT_1610.gtpr(),
    ",
  0x40322a68u64 => "
      GPT_1610.gtpbr(),
    ",
  0x40322a88u64 => "
      GPT_1610.gtdtcr(),
    ",
  0x40322a8cu64 => "
      GPT_1610.gtdvu(),
    ",
  0x40322abcu64 => "
      GPT_1610.gtpc(),
    ",
  0x40322ad0u64 => "
      GPT_1610.gtsecsr(),
    ",
  0x40322ad4u64 => "
      GPT_1610.gtsecr(),
    ",
  0x40332000u64 => "
      ADC_120.adcsr(),
    ",
  0x40332004u64 => "
      ADC_120.adansa0(),
    ",
  0x40332006u64 => "
      ADC_120.adansa1(),
    ",
  0x40332008u64 => "
      ADC_120.adads0(),
    ",
  0x4033200au64 => "
      ADC_120.adads1(),
    ",
  0x4033200cu64 => "
      ADC_120.adadc(),
    ",
  0x4033200eu64 => "
      ADC_120.adcer(),
    ",
  0x40332010u64 => "
      ADC_120.adstrgr(),
    ",
  0x40332012u64 => "
      ADC_120.adexicr(),
    ",
  0x40332014u64 => "
      ADC_120.adansb0(),
    ",
  0x40332016u64 => "
      ADC_120.adansb1(),
    ",
  0x40332018u64 => "
      ADC_120.addbldr(),
    ",
  0x4033201au64 => "
      ADC_120.adtsdr(),
    ",
  0x4033201cu64 => "
      ADC_120.adocdr(),
    ",
  0x4033201eu64 => "
      ADC_120.adrd(),
    ",
  0x40332028u64 => "
      ADC_120.addr()[4],
    ",
  0x4033202au64 => "
      ADC_120.addr()[5],
    ",
  0x4033202cu64 => "
      ADC_120.addr()[6],
    ",
  0x4033202eu64 => "
      ADC_120.addr()[7],
    ",
  0x40332030u64 => "
      ADC_120.addr()[8],
    ",
  0x40332020u64 => "
      ADC_120.addr()[0],
    ",
  0x40332022u64 => "
      ADC_120.addr()[1],
    ",
  0x40332024u64 => "
      ADC_120.addr()[2],
    ",
  0x40332026u64 => "
      ADC_120.addr()[3],
      ADC_120.advmdr(),
    ",
  0x40332066u64 => "
      ADC_120.adshcr(),
    ",
  0x4033207au64 => "
      ADC_120.addiscr(),
    ",
  0x4033207cu64 => "
      ADC_120.adshmsr(),
    ",
  0x40332080u64 => "
      ADC_120.adgspcr(),
    ",
  0x40332084u64 => "
      ADC_120.addbldra(),
    ",
  0x40332086u64 => "
      ADC_120.addbldrb(),
    ",
  0x4033208cu64 => "
      ADC_120.adwinmon(),
    ",
  0x40332090u64 => "
      ADC_120.adcmpcr(),
    ",
  0x40332092u64 => "
      ADC_120.adcmpanser(),
    ",
  0x40332093u64 => "
      ADC_120.adcmpler(),
    ",
  0x40332094u64 => "
      ADC_120.adcmpansr0(),
    ",
  0x40332098u64 => "
      ADC_120.adcmplr0(),
    ",
  0x4033209cu64 => "
      ADC_120.adcmpdr()[0],
    ",
  0x4033209eu64 => "
      ADC_120.adcmpdr()[1],
    ",
  0x403320a0u64 => "
      ADC_120.adcmpsr0(),
    ",
  0x403320a4u64 => "
      ADC_120.adcmpser(),
    ",
  0x403320a6u64 => "
      ADC_120.adcmpbnsr(),
    ",
  0x403320a8u64 => "
      ADC_120.adwinllb(),
    ",
  0x403320aau64 => "
      ADC_120.adwinulb(),
    ",
  0x403320acu64 => "
      ADC_120.adcmpbsr(),
    ",
  0x403320b0u64 => "
      ADC_120.adbuf()[0],
    ",
  0x403320b2u64 => "
      ADC_120.adbuf()[1],
    ",
  0x403320b4u64 => "
      ADC_120.adbuf()[2],
    ",
  0x403320b6u64 => "
      ADC_120.adbuf()[3],
    ",
  0x403320b8u64 => "
      ADC_120.adbuf()[4],
    ",
  0x403320bau64 => "
      ADC_120.adbuf()[5],
    ",
  0x403320bcu64 => "
      ADC_120.adbuf()[6],
    ",
  0x403320beu64 => "
      ADC_120.adbuf()[7],
    ",
  0x403320c0u64 => "
      ADC_120.adbuf()[8],
    ",
  0x403320c2u64 => "
      ADC_120.adbuf()[9],
    ",
  0x403320c4u64 => "
      ADC_120.adbuf()[10],
    ",
  0x403320c6u64 => "
      ADC_120.adbuf()[11],
    ",
  0x403320c8u64 => "
      ADC_120.adbuf()[12],
    ",
  0x403320cau64 => "
      ADC_120.adbuf()[13],
    ",
  0x403320ccu64 => "
      ADC_120.adbuf()[14],
    ",
  0x403320ceu64 => "
      ADC_120.adbuf()[15],
    ",
  0x403320d0u64 => "
      ADC_120.adbufen(),
    ",
  0x403320d2u64 => "
      ADC_120.adbufptr(),
    ",
  0x403320ddu64 => "
      ADC_120.adsstrl(),
    ",
  0x403320deu64 => "
      ADC_120.adsstrt(),
    ",
  0x403320dfu64 => "
      ADC_120.adsstro(),
    ",
  0x403320e4u64 => "
      ADC_120.adsstr()[4],
    ",
  0x403320e5u64 => "
      ADC_120.adsstr()[5],
    ",
  0x403320e6u64 => "
      ADC_120.adsstr()[6],
    ",
  0x403320e7u64 => "
      ADC_120.adsstr()[7],
    ",
  0x403320e8u64 => "
      ADC_120.adsstr()[8],
    ",
  0x403320e3u64 => "
      ADC_120.adsstrv(),
    ",
  0x403320ecu64 => "
      ADC_120.adsstr()[0],
    ",
  0x403320edu64 => "
      ADC_120.adsstr()[1],
    ",
  0x403320eeu64 => "
      ADC_120.adsstr()[2],
    ",
  0x403320efu64 => "
      ADC_120.adsstr()[3],
    ",
  0x40332200u64 => "
      ADC_121.adcsr(),
    ",
  0x40332204u64 => "
      ADC_121.adansa0(),
    ",
  0x40332206u64 => "
      ADC_121.adansa1(),
    ",
  0x40332208u64 => "
      ADC_121.adads0(),
    ",
  0x4033220au64 => "
      ADC_121.adads1(),
    ",
  0x4033220cu64 => "
      ADC_121.adadc(),
    ",
  0x4033220eu64 => "
      ADC_121.adcer(),
    ",
  0x40332210u64 => "
      ADC_121.adstrgr(),
    ",
  0x40332212u64 => "
      ADC_121.adexicr(),
    ",
  0x40332214u64 => "
      ADC_121.adansb0(),
    ",
  0x40332216u64 => "
      ADC_121.adansb1(),
    ",
  0x40332218u64 => "
      ADC_121.addbldr(),
    ",
  0x4033221au64 => "
      ADC_121.adtsdr(),
    ",
  0x4033221cu64 => "
      ADC_121.adocdr(),
    ",
  0x4033221eu64 => "
      ADC_121.adrd(),
    ",
  0x40332220u64 => "
      ADC_121.addr()[0],
    ",
  0x40332222u64 => "
      ADC_121.addr()[1],
    ",
  0x40332224u64 => "
      ADC_121.addr()[2],
    ",
  0x40332226u64 => "
      ADC_121.addr()[3],
      ADC_121.advmdr(),
    ",
  0x40332228u64 => "
      ADC_121.addr()[4],
    ",
  0x4033222au64 => "
      ADC_121.addr()[5],
    ",
  0x4033222cu64 => "
      ADC_121.addr()[6],
    ",
  0x40332266u64 => "
      ADC_121.adshcr(),
    ",
  0x4033227au64 => "
      ADC_121.addiscr(),
    ",
  0x4033227cu64 => "
      ADC_121.adshmsr(),
    ",
  0x40332280u64 => "
      ADC_121.adgspcr(),
    ",
  0x40332284u64 => "
      ADC_121.addbldra(),
    ",
  0x40332286u64 => "
      ADC_121.addbldrb(),
    ",
  0x4033228cu64 => "
      ADC_121.adwinmon(),
    ",
  0x40332290u64 => "
      ADC_121.adcmpcr(),
    ",
  0x40332292u64 => "
      ADC_121.adcmpanser(),
    ",
  0x40332293u64 => "
      ADC_121.adcmpler(),
    ",
  0x40332294u64 => "
      ADC_121.adcmpansr0(),
    ",
  0x40332298u64 => "
      ADC_121.adcmplr0(),
    ",
  0x4033229cu64 => "
      ADC_121.adcmpdr()[0],
    ",
  0x4033229eu64 => "
      ADC_121.adcmpdr()[1],
    ",
  0x403322a0u64 => "
      ADC_121.adcmpsr0(),
    ",
  0x403322a4u64 => "
      ADC_121.adcmpser(),
    ",
  0x403322a6u64 => "
      ADC_121.adcmpbnsr(),
    ",
  0x403322a8u64 => "
      ADC_121.adwinllb(),
    ",
  0x403322aau64 => "
      ADC_121.adwinulb(),
    ",
  0x403322acu64 => "
      ADC_121.adcmpbsr(),
    ",
  0x403322b0u64 => "
      ADC_121.adbuf()[0],
    ",
  0x403322b2u64 => "
      ADC_121.adbuf()[1],
    ",
  0x403322b4u64 => "
      ADC_121.adbuf()[2],
    ",
  0x403322b6u64 => "
      ADC_121.adbuf()[3],
    ",
  0x403322b8u64 => "
      ADC_121.adbuf()[4],
    ",
  0x403322bau64 => "
      ADC_121.adbuf()[5],
    ",
  0x403322bcu64 => "
      ADC_121.adbuf()[6],
    ",
  0x403322beu64 => "
      ADC_121.adbuf()[7],
    ",
  0x403322c0u64 => "
      ADC_121.adbuf()[8],
    ",
  0x403322c2u64 => "
      ADC_121.adbuf()[9],
    ",
  0x403322c4u64 => "
      ADC_121.adbuf()[10],
    ",
  0x403322c6u64 => "
      ADC_121.adbuf()[11],
    ",
  0x403322c8u64 => "
      ADC_121.adbuf()[12],
    ",
  0x403322cau64 => "
      ADC_121.adbuf()[13],
    ",
  0x403322ccu64 => "
      ADC_121.adbuf()[14],
    ",
  0x403322ceu64 => "
      ADC_121.adbuf()[15],
    ",
  0x403322d0u64 => "
      ADC_121.adbufen(),
    ",
  0x403322d2u64 => "
      ADC_121.adbufptr(),
    ",
  0x403322ddu64 => "
      ADC_121.adsstrl(),
    ",
  0x403322deu64 => "
      ADC_121.adsstrt(),
    ",
  0x403322dfu64 => "
      ADC_121.adsstro(),
    ",
  0x403322e0u64 => "
      ADC_121.adsstr()[0],
    ",
  0x403322e1u64 => "
      ADC_121.adsstr()[1],
    ",
  0x403322e2u64 => "
      ADC_121.adsstr()[2],
    ",
  0x403322e3u64 => "
      ADC_121.adsstr()[3],
      ADC_121.adsstrv(),
    ",
  0x403322e4u64 => "
      ADC_121.adsstr()[4],
    ",
  0x403322e5u64 => "
      ADC_121.adsstr()[5],
    ",
  0x403322e6u64 => "
      ADC_121.adsstr()[6],
    ",
  0x40333000u64 => "
      DAC_12.dadr()[0],
    ",
  0x40333004u64 => "
      DAC_12.dacr(),
    ",
  0x40333005u64 => "
      DAC_12.dadpr(),
    ",
  0x40333006u64 => "
      DAC_12.daadscr(),
    ",
  0x40333008u64 => "
      DAC_12.daampcr(),
    ",
  0x4033301cu64 => "
      DAC_12.daaswcr(),
    ",
  0x403340c0u64 => "
      DAC_12.daadusr(),
    ",
  0x40348000u64 => "
      CEU.capsr(),
    ",
  0x40348004u64 => "
      CEU.capcr(),
    ",
  0x40348008u64 => "
      CEU.camcr(),
    ",
  0x4034800cu64 => "
      CEU.cmcyr(),
    ",
  0x40348010u64 => "
      CEU.camor(),
    ",
  0x40348014u64 => "
      CEU.capwr(),
    ",
  0x40348018u64 => "
      CEU.caifr(),
    ",
  0x40348028u64 => "
      CEU.crcntr(),
    ",
  0x4034802cu64 => "
      CEU.crcmpr(),
    ",
  0x40348030u64 => "
      CEU.cflcr(),
    ",
  0x40348034u64 => "
      CEU.cfszr(),
    ",
  0x40348038u64 => "
      CEU.cdwdr(),
    ",
  0x4034803cu64 => "
      CEU.cdayr(),
    ",
  0x40348040u64 => "
      CEU.cdacr(),
    ",
  0x40348044u64 => "
      CEU.cdbyr(),
    ",
  0x40348048u64 => "
      CEU.cdbcr(),
    ",
  0x4034804cu64 => "
      CEU.cbdsr(),
    ",
  0x4034805cu64 => "
      CEU.cfwcr(),
    ",
  0x40348060u64 => "
      CEU.clfcr(),
    ",
  0x40348064u64 => "
      CEU.cdocr(),
    ",
  0x40348070u64 => "
      CEU.ceier(),
    ",
  0x40348074u64 => "
      CEU.cetcr(),
    ",
  0x4034807cu64 => "
      CEU.cstsr(),
    ",
  0x40348084u64 => "
      CEU.cdssr(),
    ",
  0x40348090u64 => "
      CEU.cdayr2(),
    ",
  0x40348094u64 => "
      CEU.cdacr2(),
    ",
  0x40348098u64 => "
      CEU.cdbyr2(),
    ",
  0x4034809cu64 => "
      CEU.cdbcr2(),
    ",
  0x403480a0u64 => "
      CEU.cbwer(),
    ",
  0x40349010u64 => "
      CEU.camor_b(),
    ",
  0x40349014u64 => "
      CEU.capwr_b(),
    ",
  0x40349030u64 => "
      CEU.cflcr_b(),
    ",
  0x40349034u64 => "
      CEU.cfszr_b(),
    ",
  0x40349038u64 => "
      CEU.cdwdr_b(),
    ",
  0x4034903cu64 => "
      CEU.cdayr_b(),
    ",
  0x40349040u64 => "
      CEU.cdacr_b(),
    ",
  0x40349044u64 => "
      CEU.cdbyr_b(),
    ",
  0x40349048u64 => "
      CEU.cdbcr_b(),
    ",
  0x4034904cu64 => "
      CEU.cbdsr_b(),
    ",
  0x40349060u64 => "
      CEU.clfcr_b(),
    ",
  0x40349064u64 => "
      CEU.cdocr_b(),
    ",
  0x40349090u64 => "
      CEU.cdayr2_b(),
    ",
  0x40349094u64 => "
      CEU.cdacr2_b(),
    ",
  0x40349098u64 => "
      CEU.cdbyr2_b(),
    ",
  0x4034909cu64 => "
      CEU.cdbcr2_b(),
    ",
  0x4034a010u64 => "
      CEU.camor_m(),
    ",
  0x4034a014u64 => "
      CEU.capwr_m(),
    ",
  0x4034a030u64 => "
      CEU.cflcr_m(),
    ",
  0x4034a034u64 => "
      CEU.cfszr_m(),
    ",
  0x4034a038u64 => "
      CEU.cdwdr_m(),
    ",
  0x4034a03cu64 => "
      CEU.cdayr_m(),
    ",
  0x4034a040u64 => "
      CEU.cdacr_m(),
    ",
  0x4034a044u64 => "
      CEU.cdbyr_m(),
    ",
  0x4034a048u64 => "
      CEU.cdbcr_m(),
    ",
  0x4034a04cu64 => "
      CEU.cbdsr_m(),
    ",
  0x4034a060u64 => "
      CEU.clfcr_m(),
    ",
  0x4034a064u64 => "
      CEU.cdocr_m(),
    ",
  0x4034a090u64 => "
      CEU.cdayr2_m(),
    ",
  0x4034a094u64 => "
      CEU.cdacr2_m(),
    ",
  0x4034a098u64 => "
      CEU.cdbyr2_m(),
    ",
  0x4034a09cu64 => "
      CEU.cdbcr2_m(),
    ",
  0x40354000u64 => "
      EDMAC_0.edmr(),
    ",
  0x40354008u64 => "
      EDMAC_0.edtrr(),
    ",
  0x40354010u64 => "
      EDMAC_0.edrrr(),
    ",
  0x40354018u64 => "
      EDMAC_0.tdlar(),
    ",
  0x40354020u64 => "
      EDMAC_0.rdlar(),
    ",
  0x40354028u64 => "
      EDMAC_0.eesr(),
    ",
  0x40354030u64 => "
      EDMAC_0.eesipr(),
    ",
  0x40354038u64 => "
      EDMAC_0.trscer(),
    ",
  0x40354040u64 => "
      EDMAC_0.rmfcr(),
    ",
  0x40354048u64 => "
      EDMAC_0.tftr(),
    ",
  0x40354050u64 => "
      EDMAC_0.fdr(),
    ",
  0x40354058u64 => "
      EDMAC_0.rmcr(),
    ",
  0x40354064u64 => "
      EDMAC_0.tfucr(),
    ",
  0x40354068u64 => "
      EDMAC_0.rfocr(),
    ",
  0x4035406cu64 => "
      EDMAC_0.iosr(),
    ",
  0x40354070u64 => "
      EDMAC_0.fcftr(),
    ",
  0x40354078u64 => "
      EDMAC_0.rpadir(),
    ",
  0x4035407cu64 => "
      EDMAC_0.trimd(),
    ",
  0x403540c8u64 => "
      EDMAC_0.rbwar(),
    ",
  0x403540ccu64 => "
      EDMAC_0.rdfar(),
    ",
  0x403540d4u64 => "
      EDMAC_0.tbrar(),
    ",
  0x403540d8u64 => "
      EDMAC_0.tdfar(),
    ",
  0x40354100u64 => "
      ETHERC_0.ecmr(),
    ",
  0x40354108u64 => "
      ETHERC_0.rflr(),
    ",
  0x40354110u64 => "
      ETHERC_0.ecsr(),
    ",
  0x40354118u64 => "
      ETHERC_0.ecsipr(),
    ",
  0x40354120u64 => "
      ETHERC_0.pir(),
    ",
  0x40354128u64 => "
      ETHERC_0.psr(),
    ",
  0x40354140u64 => "
      ETHERC_0.rdmlr(),
    ",
  0x40354150u64 => "
      ETHERC_0.ipgr(),
    ",
  0x40354154u64 => "
      ETHERC_0.apr(),
    ",
  0x40354158u64 => "
      ETHERC_0.mpr(),
    ",
  0x40354160u64 => "
      ETHERC_0.rfcf(),
    ",
  0x40354164u64 => "
      ETHERC_0.tpauser(),
    ",
  0x40354168u64 => "
      ETHERC_0.tpausecr(),
    ",
  0x4035416cu64 => "
      ETHERC_0.bcfrr(),
    ",
  0x403541c0u64 => "
      ETHERC_0.mahr(),
    ",
  0x403541c8u64 => "
      ETHERC_0.malr(),
    ",
  0x403541d0u64 => "
      ETHERC_0.trocr(),
    ",
  0x403541d4u64 => "
      ETHERC_0.cdcr(),
    ",
  0x403541d8u64 => "
      ETHERC_0.lccr(),
    ",
  0x403541dcu64 => "
      ETHERC_0.cndcr(),
    ",
  0x403541e4u64 => "
      ETHERC_0.cefcr(),
    ",
  0x403541e8u64 => "
      ETHERC_0.frecr(),
    ",
  0x403541ecu64 => "
      ETHERC_0.tsfrcr(),
    ",
  0x403541f0u64 => "
      ETHERC_0.tlfrcr(),
    ",
  0x403541f4u64 => "
      ETHERC_0.rfcr(),
    ",
  0x403541f8u64 => "
      ETHERC_0.mafcr(),
    ",
  0x40358000u64 => "
      SCI_0_B.rdr(),
      SCI_0_B.rdr_by(),
    ",
  0x40358004u64 => "
      SCI_0_B.tdr(),
      SCI_0_B.tdrll(),
    ",
  0x40358005u64 => "
      SCI_0_B.tdrlh(),
    ",
  0x40358008u64 => "
      SCI_0_B.ccr0(),
    ",
  0x4035800cu64 => "
      SCI_0_B.ccr1(),
    ",
  0x40358010u64 => "
      SCI_0_B.ccr2(),
    ",
  0x40358014u64 => "
      SCI_0_B.ccr3(),
    ",
  0x40358018u64 => "
      SCI_0_B.ccr4(),
    ",
  0x4035801cu64 => "
      SCI_0_B.cesr(),
    ",
  0x40358020u64 => "
      SCI_0_B.icr(),
    ",
  0x40358024u64 => "
      SCI_0_B.fcr(),
    ",
  0x4035802cu64 => "
      SCI_0_B.mcr(),
    ",
  0x40358030u64 => "
      SCI_0_B.dcr(),
    ",
  0x40358034u64 => "
      SCI_0_B.xcr0(),
    ",
  0x40358038u64 => "
      SCI_0_B.xcr1(),
    ",
  0x4035803cu64 => "
      SCI_0_B.xcr2(),
    ",
  0x40358048u64 => "
      SCI_0_B.csr(),
    ",
  0x4035804cu64 => "
      SCI_0_B.isr(),
    ",
  0x40358050u64 => "
      SCI_0_B.frsr(),
    ",
  0x40358054u64 => "
      SCI_0_B.ftsr(),
    ",
  0x40358058u64 => "
      SCI_0_B.msr(),
    ",
  0x4035805cu64 => "
      SCI_0_B.xsr0(),
    ",
  0x40358060u64 => "
      SCI_0_B.xsr1(),
    ",
  0x40358068u64 => "
      SCI_0_B.cfclr(),
    ",
  0x4035806cu64 => "
      SCI_0_B.icfclr(),
    ",
  0x40358070u64 => "
      SCI_0_B.ffclr(),
    ",
  0x40358074u64 => "
      SCI_0_B.mfclr(),
    ",
  0x40358078u64 => "
      SCI_0_B.xfclr(),
    ",
  0x4035c000u64 => "
      SPI_0_B.spdr(),
    ",
  0x4035c004u64 => "
      SPI_0_B.spdecr(),
    ",
  0x4035c008u64 => "
      SPI_0_B.spcr(),
    ",
  0x4035c00cu64 => "
      SPI_0_B.spcr2(),
    ",
  0x4035c010u64 => "
      SPI_0_B.spcr3(),
    ",
  0x4035c014u64 => "
      SPI_0_B.spcmd()[0],
    ",
  0x4035c018u64 => "
      SPI_0_B.spcmd()[1],
    ",
  0x4035c01cu64 => "
      SPI_0_B.spcmd()[2],
    ",
  0x4035c020u64 => "
      SPI_0_B.spcmd()[3],
    ",
  0x4035c024u64 => "
      SPI_0_B.spcmd()[4],
    ",
  0x4035c028u64 => "
      SPI_0_B.spcmd()[5],
    ",
  0x4035c02cu64 => "
      SPI_0_B.spcmd()[6],
    ",
  0x4035c030u64 => "
      SPI_0_B.spcmd()[7],
    ",
  0x4035c040u64 => "
      SPI_0_B.spdcr(),
    ",
  0x4035c044u64 => "
      SPI_0_B.spdcr2(),
    ",
  0x4035c050u64 => "
      SPI_0_B.spsr(),
    ",
  0x4035c058u64 => "
      SPI_0_B.sptfsr(),
    ",
  0x4035c05cu64 => "
      SPI_0_B.sprfsr(),
    ",
  0x4035c060u64 => "
      SPI_0_B.sppsr(),
    ",
  0x4035c068u64 => "
      SPI_0_B.spsrc(),
    ",
  0x4035c06cu64 => "
      SPI_0_B.spfcr(),
    ",
  0x4036f200u64 => "
      ECCMB_0.ec710ctl(),
    ",
  0x4036f204u64 => "
      ECCMB_0.ec710tmc(),
    ",
  0x4036f20cu64 => "
      ECCMB_0.ec710ted(),
    ",
  0x4036f210u64 => "
      ECCMB_0.ec710ead0(),
    ",
  0x40380000u64 => "
      CANFD_0.cfdc0ncfg(),
    ",
  0x40380004u64 => "
      CANFD_0.cfdc0ctr(),
    ",
  0x40380008u64 => "
      CANFD_0.cfdc0sts(),
    ",
  0x4038000cu64 => "
      CANFD_0.cfdc0erfl(),
    ",
  0x40380014u64 => "
      CANFD_0.cfdgcfg(),
    ",
  0x40380018u64 => "
      CANFD_0.cfdgctr(),
    ",
  0x4038001cu64 => "
      CANFD_0.cfdgsts(),
    ",
  0x40380020u64 => "
      CANFD_0.cfdgerfl(),
    ",
  0x40380024u64 => "
      CANFD_0.cfdgtsc(),
    ",
  0x40380028u64 => "
      CANFD_0.cfdgaflectr(),
    ",
  0x4038002cu64 => "
      CANFD_0.cfdgaflcfg(),
    ",
  0x40380030u64 => "
      CANFD_0.cfdrmnb(),
    ",
  0x40380034u64 => "
      CANFD_0.cfdrmnd(),
    ",
  0x40380038u64 => "
      CANFD_0.cfdrmiec(),
    ",
  0x4038003cu64 => "
      CANFD_0.cfdrfcc()[0],
    ",
  0x40380040u64 => "
      CANFD_0.cfdrfcc()[1],
    ",
  0x40380044u64 => "
      CANFD_0.cfdrfsts()[0],
    ",
  0x40380048u64 => "
      CANFD_0.cfdrfsts()[1],
    ",
  0x4038004cu64 => "
      CANFD_0.cfdrfpctr()[0],
    ",
  0x40380050u64 => "
      CANFD_0.cfdrfpctr()[1],
    ",
  0x40380054u64 => "
      CANFD_0.cfdcfcc(),
    ",
  0x40380058u64 => "
      CANFD_0.cfdcfsts(),
    ",
  0x4038005cu64 => "
      CANFD_0.cfdcfpctr(),
    ",
  0x40380060u64 => "
      CANFD_0.cfdfests(),
    ",
  0x40380064u64 => "
      CANFD_0.cfdffsts(),
    ",
  0x40380068u64 => "
      CANFD_0.cfdfmsts(),
    ",
  0x4038006cu64 => "
      CANFD_0.cfdrfists(),
    ",
  0x40380070u64 => "
      CANFD_0.cfdtmc()[0],
    ",
  0x40380071u64 => "
      CANFD_0.cfdtmc()[1],
    ",
  0x40380072u64 => "
      CANFD_0.cfdtmc()[2],
    ",
  0x40380073u64 => "
      CANFD_0.cfdtmc()[3],
    ",
  0x40380074u64 => "
      CANFD_0.cfdtmsts()[0],
    ",
  0x40380075u64 => "
      CANFD_0.cfdtmsts()[1],
    ",
  0x40380076u64 => "
      CANFD_0.cfdtmsts()[2],
    ",
  0x40380077u64 => "
      CANFD_0.cfdtmsts()[3],
    ",
  0x40380078u64 => "
      CANFD_0.cfdtmtrsts(),
    ",
  0x4038007cu64 => "
      CANFD_0.cfdtmtarsts(),
    ",
  0x40380080u64 => "
      CANFD_0.cfdtmtcsts(),
    ",
  0x40380084u64 => "
      CANFD_0.cfdtmtasts(),
    ",
  0x40380088u64 => "
      CANFD_0.cfdtmiec(),
    ",
  0x4038008cu64 => "
      CANFD_0.cfdtxqcc(),
    ",
  0x40380090u64 => "
      CANFD_0.cfdtxqsts(),
    ",
  0x40380094u64 => "
      CANFD_0.cfdtxqpctr(),
    ",
  0x40380098u64 => "
      CANFD_0.cfdthlcc(),
    ",
  0x4038009cu64 => "
      CANFD_0.cfdthlsts(),
    ",
  0x403800a0u64 => "
      CANFD_0.cfdthlpctr(),
    ",
  0x403800a4u64 => "
      CANFD_0.cfdgtintsts(),
    ",
  0x403800a8u64 => "
      CANFD_0.cfdgtstcfg(),
    ",
  0x403800acu64 => "
      CANFD_0.cfdgtstctr(),
    ",
  0x403800b0u64 => "
      CANFD_0.cfdgfdcfg(),
    ",
  0x403800b8u64 => "
      CANFD_0.cfdglockk(),
    ",
  0x403800c0u64 => "
      CANFD_0.cfdgaflignent(),
    ",
  0x403800c4u64 => "
      CANFD_0.cfdgaflignctr(),
    ",
  0x403800c8u64 => "
      CANFD_0.cfdcdtct(),
    ",
  0x403800ccu64 => "
      CANFD_0.cfdcdtsts(),
    ",
  0x403800d8u64 => "
      CANFD_0.cfdgrstc(),
    ",
  0x40380100u64 => "
      CANFD_0.cfdc0dcfg(),
    ",
  0x40380104u64 => "
      CANFD_0.cfdc0fdcfg(),
    ",
  0x40380108u64 => "
      CANFD_0.cfdc0fdctr(),
    ",
  0x4038010cu64 => "
      CANFD_0.cfdc0fdsts(),
    ",
  0x40380110u64 => "
      CANFD_0.cfdc0fdcrc(),
    ",
  0x40380120u64 => "
      CANFD_0.cfdgaflid()[0],
    ",
  0x40380130u64 => "
      CANFD_0.cfdgaflid()[1],
    ",
  0x40380140u64 => "
      CANFD_0.cfdgaflid()[2],
    ",
  0x40380150u64 => "
      CANFD_0.cfdgaflid()[3],
    ",
  0x40380160u64 => "
      CANFD_0.cfdgaflid()[4],
    ",
  0x40380170u64 => "
      CANFD_0.cfdgaflid()[5],
    ",
  0x40380180u64 => "
      CANFD_0.cfdgaflid()[6],
    ",
  0x40380190u64 => "
      CANFD_0.cfdgaflid()[7],
    ",
  0x403801a0u64 => "
      CANFD_0.cfdgaflid()[8],
    ",
  0x403801b0u64 => "
      CANFD_0.cfdgaflid()[9],
    ",
  0x403801c0u64 => "
      CANFD_0.cfdgaflid()[10],
    ",
  0x403801d0u64 => "
      CANFD_0.cfdgaflid()[11],
    ",
  0x403801e0u64 => "
      CANFD_0.cfdgaflid()[12],
    ",
  0x403801f0u64 => "
      CANFD_0.cfdgaflid()[13],
    ",
  0x40380200u64 => "
      CANFD_0.cfdgaflid()[14],
    ",
  0x40380210u64 => "
      CANFD_0.cfdgaflid()[15],
    ",
  0x40380124u64 => "
      CANFD_0.cfdgaflm()[0],
    ",
  0x40380134u64 => "
      CANFD_0.cfdgaflm()[1],
    ",
  0x40380144u64 => "
      CANFD_0.cfdgaflm()[2],
    ",
  0x40380154u64 => "
      CANFD_0.cfdgaflm()[3],
    ",
  0x40380164u64 => "
      CANFD_0.cfdgaflm()[4],
    ",
  0x40380174u64 => "
      CANFD_0.cfdgaflm()[5],
    ",
  0x40380184u64 => "
      CANFD_0.cfdgaflm()[6],
    ",
  0x40380194u64 => "
      CANFD_0.cfdgaflm()[7],
    ",
  0x403801a4u64 => "
      CANFD_0.cfdgaflm()[8],
    ",
  0x403801b4u64 => "
      CANFD_0.cfdgaflm()[9],
    ",
  0x403801c4u64 => "
      CANFD_0.cfdgaflm()[10],
    ",
  0x403801d4u64 => "
      CANFD_0.cfdgaflm()[11],
    ",
  0x403801e4u64 => "
      CANFD_0.cfdgaflm()[12],
    ",
  0x403801f4u64 => "
      CANFD_0.cfdgaflm()[13],
    ",
  0x40380204u64 => "
      CANFD_0.cfdgaflm()[14],
    ",
  0x40380214u64 => "
      CANFD_0.cfdgaflm()[15],
    ",
  0x40380128u64 => "
      CANFD_0.cfdgaflp0()[0],
    ",
  0x40380138u64 => "
      CANFD_0.cfdgaflp0()[1],
    ",
  0x40380148u64 => "
      CANFD_0.cfdgaflp0()[2],
    ",
  0x40380158u64 => "
      CANFD_0.cfdgaflp0()[3],
    ",
  0x40380168u64 => "
      CANFD_0.cfdgaflp0()[4],
    ",
  0x40380178u64 => "
      CANFD_0.cfdgaflp0()[5],
    ",
  0x40380188u64 => "
      CANFD_0.cfdgaflp0()[6],
    ",
  0x40380198u64 => "
      CANFD_0.cfdgaflp0()[7],
    ",
  0x403801a8u64 => "
      CANFD_0.cfdgaflp0()[8],
    ",
  0x403801b8u64 => "
      CANFD_0.cfdgaflp0()[9],
    ",
  0x403801c8u64 => "
      CANFD_0.cfdgaflp0()[10],
    ",
  0x403801d8u64 => "
      CANFD_0.cfdgaflp0()[11],
    ",
  0x403801e8u64 => "
      CANFD_0.cfdgaflp0()[12],
    ",
  0x403801f8u64 => "
      CANFD_0.cfdgaflp0()[13],
    ",
  0x40380208u64 => "
      CANFD_0.cfdgaflp0()[14],
    ",
  0x40380218u64 => "
      CANFD_0.cfdgaflp0()[15],
    ",
  0x4038012cu64 => "
      CANFD_0.cfdgaflp1()[0],
    ",
  0x4038013cu64 => "
      CANFD_0.cfdgaflp1()[1],
    ",
  0x4038014cu64 => "
      CANFD_0.cfdgaflp1()[2],
    ",
  0x4038015cu64 => "
      CANFD_0.cfdgaflp1()[3],
    ",
  0x4038016cu64 => "
      CANFD_0.cfdgaflp1()[4],
    ",
  0x4038017cu64 => "
      CANFD_0.cfdgaflp1()[5],
    ",
  0x4038018cu64 => "
      CANFD_0.cfdgaflp1()[6],
    ",
  0x4038019cu64 => "
      CANFD_0.cfdgaflp1()[7],
    ",
  0x403801acu64 => "
      CANFD_0.cfdgaflp1()[8],
    ",
  0x403801bcu64 => "
      CANFD_0.cfdgaflp1()[9],
    ",
  0x403801ccu64 => "
      CANFD_0.cfdgaflp1()[10],
    ",
  0x403801dcu64 => "
      CANFD_0.cfdgaflp1()[11],
    ",
  0x403801ecu64 => "
      CANFD_0.cfdgaflp1()[12],
    ",
  0x403801fcu64 => "
      CANFD_0.cfdgaflp1()[13],
    ",
  0x4038020cu64 => "
      CANFD_0.cfdgaflp1()[14],
    ",
  0x4038021cu64 => "
      CANFD_0.cfdgaflp1()[15],
    ",
  0x40380280u64 => "
      CANFD_0.cfdrpgacc()[0],
    ",
  0x40380284u64 => "
      CANFD_0.cfdrpgacc()[1],
    ",
  0x40380288u64 => "
      CANFD_0.cfdrpgacc()[2],
    ",
  0x4038028cu64 => "
      CANFD_0.cfdrpgacc()[3],
    ",
  0x40380290u64 => "
      CANFD_0.cfdrpgacc()[4],
    ",
  0x40380294u64 => "
      CANFD_0.cfdrpgacc()[5],
    ",
  0x40380298u64 => "
      CANFD_0.cfdrpgacc()[6],
    ",
  0x4038029cu64 => "
      CANFD_0.cfdrpgacc()[7],
    ",
  0x403802a0u64 => "
      CANFD_0.cfdrpgacc()[8],
    ",
  0x403802a4u64 => "
      CANFD_0.cfdrpgacc()[9],
    ",
  0x403802a8u64 => "
      CANFD_0.cfdrpgacc()[10],
    ",
  0x403802acu64 => "
      CANFD_0.cfdrpgacc()[11],
    ",
  0x403802b0u64 => "
      CANFD_0.cfdrpgacc()[12],
    ",
  0x403802b4u64 => "
      CANFD_0.cfdrpgacc()[13],
    ",
  0x403802b8u64 => "
      CANFD_0.cfdrpgacc()[14],
    ",
  0x403802bcu64 => "
      CANFD_0.cfdrpgacc()[15],
    ",
  0x403802c0u64 => "
      CANFD_0.cfdrpgacc()[16],
    ",
  0x403802c4u64 => "
      CANFD_0.cfdrpgacc()[17],
    ",
  0x403802c8u64 => "
      CANFD_0.cfdrpgacc()[18],
    ",
  0x403802ccu64 => "
      CANFD_0.cfdrpgacc()[19],
    ",
  0x403802d0u64 => "
      CANFD_0.cfdrpgacc()[20],
    ",
  0x403802d4u64 => "
      CANFD_0.cfdrpgacc()[21],
    ",
  0x403802d8u64 => "
      CANFD_0.cfdrpgacc()[22],
    ",
  0x403802dcu64 => "
      CANFD_0.cfdrpgacc()[23],
    ",
  0x403802e0u64 => "
      CANFD_0.cfdrpgacc()[24],
    ",
  0x403802e4u64 => "
      CANFD_0.cfdrpgacc()[25],
    ",
  0x403802e8u64 => "
      CANFD_0.cfdrpgacc()[26],
    ",
  0x403802ecu64 => "
      CANFD_0.cfdrpgacc()[27],
    ",
  0x403802f0u64 => "
      CANFD_0.cfdrpgacc()[28],
    ",
  0x403802f4u64 => "
      CANFD_0.cfdrpgacc()[29],
    ",
  0x403802f8u64 => "
      CANFD_0.cfdrpgacc()[30],
    ",
  0x403802fcu64 => "
      CANFD_0.cfdrpgacc()[31],
    ",
  0x40380300u64 => "
      CANFD_0.cfdrpgacc()[32],
    ",
  0x40380304u64 => "
      CANFD_0.cfdrpgacc()[33],
    ",
  0x40380308u64 => "
      CANFD_0.cfdrpgacc()[34],
    ",
  0x4038030cu64 => "
      CANFD_0.cfdrpgacc()[35],
    ",
  0x40380310u64 => "
      CANFD_0.cfdrpgacc()[36],
    ",
  0x40380314u64 => "
      CANFD_0.cfdrpgacc()[37],
    ",
  0x40380318u64 => "
      CANFD_0.cfdrpgacc()[38],
    ",
  0x4038031cu64 => "
      CANFD_0.cfdrpgacc()[39],
    ",
  0x40380320u64 => "
      CANFD_0.cfdrpgacc()[40],
    ",
  0x40380324u64 => "
      CANFD_0.cfdrpgacc()[41],
    ",
  0x40380328u64 => "
      CANFD_0.cfdrpgacc()[42],
    ",
  0x4038032cu64 => "
      CANFD_0.cfdrpgacc()[43],
    ",
  0x40380330u64 => "
      CANFD_0.cfdrpgacc()[44],
    ",
  0x40380334u64 => "
      CANFD_0.cfdrpgacc()[45],
    ",
  0x40380338u64 => "
      CANFD_0.cfdrpgacc()[46],
    ",
  0x4038033cu64 => "
      CANFD_0.cfdrpgacc()[47],
    ",
  0x40380340u64 => "
      CANFD_0.cfdrpgacc()[48],
    ",
  0x40380344u64 => "
      CANFD_0.cfdrpgacc()[49],
    ",
  0x40380348u64 => "
      CANFD_0.cfdrpgacc()[50],
    ",
  0x4038034cu64 => "
      CANFD_0.cfdrpgacc()[51],
    ",
  0x40380350u64 => "
      CANFD_0.cfdrpgacc()[52],
    ",
  0x40380354u64 => "
      CANFD_0.cfdrpgacc()[53],
    ",
  0x40380358u64 => "
      CANFD_0.cfdrpgacc()[54],
    ",
  0x4038035cu64 => "
      CANFD_0.cfdrpgacc()[55],
    ",
  0x40380360u64 => "
      CANFD_0.cfdrpgacc()[56],
    ",
  0x40380364u64 => "
      CANFD_0.cfdrpgacc()[57],
    ",
  0x40380368u64 => "
      CANFD_0.cfdrpgacc()[58],
    ",
  0x4038036cu64 => "
      CANFD_0.cfdrpgacc()[59],
    ",
  0x40380370u64 => "
      CANFD_0.cfdrpgacc()[60],
    ",
  0x40380374u64 => "
      CANFD_0.cfdrpgacc()[61],
    ",
  0x40380378u64 => "
      CANFD_0.cfdrpgacc()[62],
    ",
  0x4038037cu64 => "
      CANFD_0.cfdrpgacc()[63],
    ",
  0x40380520u64 => "
      CANFD_0.cfdrfid()[0],
    ",
  0x4038056cu64 => "
      CANFD_0.cfdrfid()[1],
    ",
  0x40380524u64 => "
      CANFD_0.cfdrfptr()[0],
    ",
  0x40380570u64 => "
      CANFD_0.cfdrfptr()[1],
    ",
  0x40380528u64 => "
      CANFD_0.cfdrffdsts()[0],
    ",
  0x40380574u64 => "
      CANFD_0.cfdrffdsts()[1],
    ",
  0x4038052cu64 => "
      CANFD_0.cfdrfdf_0()[0],
    ",
  0x40380578u64 => "
      CANFD_0.cfdrfdf_0()[1],
    ",
  0x40380530u64 => "
      CANFD_0.cfdrfdf_1()[0],
    ",
  0x4038057cu64 => "
      CANFD_0.cfdrfdf_1()[1],
    ",
  0x40380534u64 => "
      CANFD_0.cfdrfdf_2()[0],
    ",
  0x40380580u64 => "
      CANFD_0.cfdrfdf_2()[1],
    ",
  0x40380538u64 => "
      CANFD_0.cfdrfdf_3()[0],
    ",
  0x40380584u64 => "
      CANFD_0.cfdrfdf_3()[1],
    ",
  0x4038053cu64 => "
      CANFD_0.cfdrfdf_4()[0],
    ",
  0x40380588u64 => "
      CANFD_0.cfdrfdf_4()[1],
    ",
  0x40380540u64 => "
      CANFD_0.cfdrfdf_5()[0],
    ",
  0x4038058cu64 => "
      CANFD_0.cfdrfdf_5()[1],
    ",
  0x40380544u64 => "
      CANFD_0.cfdrfdf_6()[0],
    ",
  0x40380590u64 => "
      CANFD_0.cfdrfdf_6()[1],
    ",
  0x40380548u64 => "
      CANFD_0.cfdrfdf_7()[0],
    ",
  0x40380594u64 => "
      CANFD_0.cfdrfdf_7()[1],
    ",
  0x4038054cu64 => "
      CANFD_0.cfdrfdf_8()[0],
    ",
  0x40380598u64 => "
      CANFD_0.cfdrfdf_8()[1],
    ",
  0x40380550u64 => "
      CANFD_0.cfdrfdf_9()[0],
    ",
  0x4038059cu64 => "
      CANFD_0.cfdrfdf_9()[1],
    ",
  0x40380554u64 => "
      CANFD_0.cfdrfdf_10()[0],
    ",
  0x403805a0u64 => "
      CANFD_0.cfdrfdf_10()[1],
    ",
  0x40380558u64 => "
      CANFD_0.cfdrfdf_11()[0],
    ",
  0x403805a4u64 => "
      CANFD_0.cfdrfdf_11()[1],
    ",
  0x4038055cu64 => "
      CANFD_0.cfdrfdf_12()[0],
    ",
  0x403805a8u64 => "
      CANFD_0.cfdrfdf_12()[1],
    ",
  0x40380560u64 => "
      CANFD_0.cfdrfdf_13()[0],
    ",
  0x403805acu64 => "
      CANFD_0.cfdrfdf_13()[1],
    ",
  0x40380564u64 => "
      CANFD_0.cfdrfdf_14()[0],
    ",
  0x403805b0u64 => "
      CANFD_0.cfdrfdf_14()[1],
    ",
  0x40380568u64 => "
      CANFD_0.cfdrfdf_15()[0],
    ",
  0x403805b4u64 => "
      CANFD_0.cfdrfdf_15()[1],
    ",
  0x403805b8u64 => "
      CANFD_0.cfdcfid(),
    ",
  0x403805bcu64 => "
      CANFD_0.cfdcfptr(),
    ",
  0x403805c0u64 => "
      CANFD_0.cfdcffdcsts(),
    ",
  0x403805c4u64 => "
      CANFD_0.cfdcfdf()[0],
    ",
  0x403805c8u64 => "
      CANFD_0.cfdcfdf()[1],
    ",
  0x403805ccu64 => "
      CANFD_0.cfdcfdf()[2],
    ",
  0x403805d0u64 => "
      CANFD_0.cfdcfdf()[3],
    ",
  0x403805d4u64 => "
      CANFD_0.cfdcfdf()[4],
    ",
  0x403805d8u64 => "
      CANFD_0.cfdcfdf()[5],
    ",
  0x403805dcu64 => "
      CANFD_0.cfdcfdf()[6],
    ",
  0x403805e0u64 => "
      CANFD_0.cfdcfdf()[7],
    ",
  0x403805e4u64 => "
      CANFD_0.cfdcfdf()[8],
    ",
  0x403805e8u64 => "
      CANFD_0.cfdcfdf()[9],
    ",
  0x403805ecu64 => "
      CANFD_0.cfdcfdf()[10],
    ",
  0x403805f0u64 => "
      CANFD_0.cfdcfdf()[11],
    ",
  0x403805f4u64 => "
      CANFD_0.cfdcfdf()[12],
    ",
  0x403805f8u64 => "
      CANFD_0.cfdcfdf()[13],
    ",
  0x403805fcu64 => "
      CANFD_0.cfdcfdf()[14],
    ",
  0x40380600u64 => "
      CANFD_0.cfdcfdf()[15],
    ",
  0x40380604u64 => "
      CANFD_0.cfdtmid()[0],
    ",
  0x40380650u64 => "
      CANFD_0.cfdtmid()[1],
    ",
  0x4038069cu64 => "
      CANFD_0.cfdtmid()[2],
    ",
  0x403806e8u64 => "
      CANFD_0.cfdtmid()[3],
    ",
  0x40380608u64 => "
      CANFD_0.cfdtmptr()[0],
    ",
  0x40380654u64 => "
      CANFD_0.cfdtmptr()[1],
    ",
  0x403806a0u64 => "
      CANFD_0.cfdtmptr()[2],
    ",
  0x403806ecu64 => "
      CANFD_0.cfdtmptr()[3],
    ",
  0x4038060cu64 => "
      CANFD_0.cfdtmfdctr()[0],
    ",
  0x40380658u64 => "
      CANFD_0.cfdtmfdctr()[1],
    ",
  0x403806a4u64 => "
      CANFD_0.cfdtmfdctr()[2],
    ",
  0x403806f0u64 => "
      CANFD_0.cfdtmfdctr()[3],
    ",
  0x40380610u64 => "
      CANFD_0.cfdtmdf_0()[0],
    ",
  0x4038065cu64 => "
      CANFD_0.cfdtmdf_0()[1],
    ",
  0x403806a8u64 => "
      CANFD_0.cfdtmdf_0()[2],
    ",
  0x403806f4u64 => "
      CANFD_0.cfdtmdf_0()[3],
    ",
  0x40380614u64 => "
      CANFD_0.cfdtmdf_1()[0],
    ",
  0x40380660u64 => "
      CANFD_0.cfdtmdf_1()[1],
    ",
  0x403806acu64 => "
      CANFD_0.cfdtmdf_1()[2],
    ",
  0x403806f8u64 => "
      CANFD_0.cfdtmdf_1()[3],
    ",
  0x40380618u64 => "
      CANFD_0.cfdtmdf_2()[0],
    ",
  0x40380664u64 => "
      CANFD_0.cfdtmdf_2()[1],
    ",
  0x403806b0u64 => "
      CANFD_0.cfdtmdf_2()[2],
    ",
  0x403806fcu64 => "
      CANFD_0.cfdtmdf_2()[3],
    ",
  0x4038061cu64 => "
      CANFD_0.cfdtmdf_3()[0],
    ",
  0x40380668u64 => "
      CANFD_0.cfdtmdf_3()[1],
    ",
  0x403806b4u64 => "
      CANFD_0.cfdtmdf_3()[2],
    ",
  0x40380700u64 => "
      CANFD_0.cfdtmdf_3()[3],
    ",
  0x40380620u64 => "
      CANFD_0.cfdtmdf_4()[0],
    ",
  0x4038066cu64 => "
      CANFD_0.cfdtmdf_4()[1],
    ",
  0x403806b8u64 => "
      CANFD_0.cfdtmdf_4()[2],
    ",
  0x40380704u64 => "
      CANFD_0.cfdtmdf_4()[3],
    ",
  0x40380624u64 => "
      CANFD_0.cfdtmdf_5()[0],
    ",
  0x40380670u64 => "
      CANFD_0.cfdtmdf_5()[1],
    ",
  0x403806bcu64 => "
      CANFD_0.cfdtmdf_5()[2],
    ",
  0x40380708u64 => "
      CANFD_0.cfdtmdf_5()[3],
    ",
  0x40380628u64 => "
      CANFD_0.cfdtmdf_6()[0],
    ",
  0x40380674u64 => "
      CANFD_0.cfdtmdf_6()[1],
    ",
  0x403806c0u64 => "
      CANFD_0.cfdtmdf_6()[2],
    ",
  0x4038070cu64 => "
      CANFD_0.cfdtmdf_6()[3],
    ",
  0x4038062cu64 => "
      CANFD_0.cfdtmdf_7()[0],
    ",
  0x40380678u64 => "
      CANFD_0.cfdtmdf_7()[1],
    ",
  0x403806c4u64 => "
      CANFD_0.cfdtmdf_7()[2],
    ",
  0x40380710u64 => "
      CANFD_0.cfdtmdf_7()[3],
    ",
  0x40380630u64 => "
      CANFD_0.cfdtmdf_8()[0],
    ",
  0x4038067cu64 => "
      CANFD_0.cfdtmdf_8()[1],
    ",
  0x403806c8u64 => "
      CANFD_0.cfdtmdf_8()[2],
    ",
  0x40380714u64 => "
      CANFD_0.cfdtmdf_8()[3],
    ",
  0x40380634u64 => "
      CANFD_0.cfdtmdf_9()[0],
    ",
  0x40380680u64 => "
      CANFD_0.cfdtmdf_9()[1],
    ",
  0x403806ccu64 => "
      CANFD_0.cfdtmdf_9()[2],
    ",
  0x40380718u64 => "
      CANFD_0.cfdtmdf_9()[3],
    ",
  0x40380638u64 => "
      CANFD_0.cfdtmdf_10()[0],
    ",
  0x40380684u64 => "
      CANFD_0.cfdtmdf_10()[1],
    ",
  0x403806d0u64 => "
      CANFD_0.cfdtmdf_10()[2],
    ",
  0x4038071cu64 => "
      CANFD_0.cfdtmdf_10()[3],
    ",
  0x4038063cu64 => "
      CANFD_0.cfdtmdf_11()[0],
    ",
  0x40380688u64 => "
      CANFD_0.cfdtmdf_11()[1],
    ",
  0x403806d4u64 => "
      CANFD_0.cfdtmdf_11()[2],
    ",
  0x40380720u64 => "
      CANFD_0.cfdtmdf_11()[3],
    ",
  0x40380640u64 => "
      CANFD_0.cfdtmdf_12()[0],
    ",
  0x4038068cu64 => "
      CANFD_0.cfdtmdf_12()[1],
    ",
  0x403806d8u64 => "
      CANFD_0.cfdtmdf_12()[2],
    ",
  0x40380724u64 => "
      CANFD_0.cfdtmdf_12()[3],
    ",
  0x40380644u64 => "
      CANFD_0.cfdtmdf_13()[0],
    ",
  0x40380690u64 => "
      CANFD_0.cfdtmdf_13()[1],
    ",
  0x403806dcu64 => "
      CANFD_0.cfdtmdf_13()[2],
    ",
  0x40380728u64 => "
      CANFD_0.cfdtmdf_13()[3],
    ",
  0x40380648u64 => "
      CANFD_0.cfdtmdf_14()[0],
    ",
  0x40380694u64 => "
      CANFD_0.cfdtmdf_14()[1],
    ",
  0x403806e0u64 => "
      CANFD_0.cfdtmdf_14()[2],
    ",
  0x4038072cu64 => "
      CANFD_0.cfdtmdf_14()[3],
    ",
  0x4038064cu64 => "
      CANFD_0.cfdtmdf_15()[0],
    ",
  0x40380698u64 => "
      CANFD_0.cfdtmdf_15()[1],
    ",
  0x403806e4u64 => "
      CANFD_0.cfdtmdf_15()[2],
    ",
  0x40380730u64 => "
      CANFD_0.cfdtmdf_15()[3],
    ",
  0x40380740u64 => "
      CANFD_0.cfdthlacc0(),
    ",
  0x40380744u64 => "
      CANFD_0.cfdthlacc1(),
    ",
  0x40380d20u64 => "
      CANFD_0.cfdrmid()[0],
    ",
  0x40380d6cu64 => "
      CANFD_0.cfdrmid()[1],
    ",
  0x40380db8u64 => "
      CANFD_0.cfdrmid()[2],
    ",
  0x40380e04u64 => "
      CANFD_0.cfdrmid()[3],
    ",
  0x40380e50u64 => "
      CANFD_0.cfdrmid()[4],
    ",
  0x40380e9cu64 => "
      CANFD_0.cfdrmid()[5],
    ",
  0x40380ee8u64 => "
      CANFD_0.cfdrmid()[6],
    ",
  0x40380f34u64 => "
      CANFD_0.cfdrmid()[7],
    ",
  0x40381524u64 => "
      CANFD_0.cfdrmptr()[0],
    ",
  0x40381570u64 => "
      CANFD_0.cfdrmptr()[1],
    ",
  0x403815bcu64 => "
      CANFD_0.cfdrmptr()[2],
    ",
  0x40381608u64 => "
      CANFD_0.cfdrmptr()[3],
    ",
  0x40381654u64 => "
      CANFD_0.cfdrmptr()[4],
    ",
  0x403816a0u64 => "
      CANFD_0.cfdrmptr()[5],
    ",
  0x403816ecu64 => "
      CANFD_0.cfdrmptr()[6],
    ",
  0x40381738u64 => "
      CANFD_0.cfdrmptr()[7],
    ",
  0x40381528u64 => "
      CANFD_0.cfdrmfdsts()[0],
    ",
  0x40381574u64 => "
      CANFD_0.cfdrmfdsts()[1],
    ",
  0x403815c0u64 => "
      CANFD_0.cfdrmfdsts()[2],
    ",
  0x4038160cu64 => "
      CANFD_0.cfdrmfdsts()[3],
    ",
  0x40381658u64 => "
      CANFD_0.cfdrmfdsts()[4],
    ",
  0x403816a4u64 => "
      CANFD_0.cfdrmfdsts()[5],
    ",
  0x403816f0u64 => "
      CANFD_0.cfdrmfdsts()[6],
    ",
  0x4038173cu64 => "
      CANFD_0.cfdrmfdsts()[7],
    ",
  0x4038152cu64 => "
      CANFD_0.cfdrmdf_0()[0],
    ",
  0x40381578u64 => "
      CANFD_0.cfdrmdf_0()[1],
    ",
  0x403815c4u64 => "
      CANFD_0.cfdrmdf_0()[2],
    ",
  0x40381610u64 => "
      CANFD_0.cfdrmdf_0()[3],
    ",
  0x4038165cu64 => "
      CANFD_0.cfdrmdf_0()[4],
    ",
  0x403816a8u64 => "
      CANFD_0.cfdrmdf_0()[5],
    ",
  0x403816f4u64 => "
      CANFD_0.cfdrmdf_0()[6],
    ",
  0x40381740u64 => "
      CANFD_0.cfdrmdf_0()[7],
    ",
  0x40381530u64 => "
      CANFD_0.cfdrmdf_1()[0],
    ",
  0x4038157cu64 => "
      CANFD_0.cfdrmdf_1()[1],
    ",
  0x403815c8u64 => "
      CANFD_0.cfdrmdf_1()[2],
    ",
  0x40381614u64 => "
      CANFD_0.cfdrmdf_1()[3],
    ",
  0x40381660u64 => "
      CANFD_0.cfdrmdf_1()[4],
    ",
  0x403816acu64 => "
      CANFD_0.cfdrmdf_1()[5],
    ",
  0x403816f8u64 => "
      CANFD_0.cfdrmdf_1()[6],
    ",
  0x40381744u64 => "
      CANFD_0.cfdrmdf_1()[7],
    ",
  0x40381534u64 => "
      CANFD_0.cfdrmdf_2()[0],
    ",
  0x40381580u64 => "
      CANFD_0.cfdrmdf_2()[1],
    ",
  0x403815ccu64 => "
      CANFD_0.cfdrmdf_2()[2],
    ",
  0x40381618u64 => "
      CANFD_0.cfdrmdf_2()[3],
    ",
  0x40381664u64 => "
      CANFD_0.cfdrmdf_2()[4],
    ",
  0x403816b0u64 => "
      CANFD_0.cfdrmdf_2()[5],
    ",
  0x403816fcu64 => "
      CANFD_0.cfdrmdf_2()[6],
    ",
  0x40381748u64 => "
      CANFD_0.cfdrmdf_2()[7],
    ",
  0x40381538u64 => "
      CANFD_0.cfdrmdf_3()[0],
    ",
  0x40381584u64 => "
      CANFD_0.cfdrmdf_3()[1],
    ",
  0x403815d0u64 => "
      CANFD_0.cfdrmdf_3()[2],
    ",
  0x4038161cu64 => "
      CANFD_0.cfdrmdf_3()[3],
    ",
  0x40381668u64 => "
      CANFD_0.cfdrmdf_3()[4],
    ",
  0x403816b4u64 => "
      CANFD_0.cfdrmdf_3()[5],
    ",
  0x40381700u64 => "
      CANFD_0.cfdrmdf_3()[6],
    ",
  0x4038174cu64 => "
      CANFD_0.cfdrmdf_3()[7],
    ",
  0x4038153cu64 => "
      CANFD_0.cfdrmdf_4()[0],
    ",
  0x40381588u64 => "
      CANFD_0.cfdrmdf_4()[1],
    ",
  0x403815d4u64 => "
      CANFD_0.cfdrmdf_4()[2],
    ",
  0x40381620u64 => "
      CANFD_0.cfdrmdf_4()[3],
    ",
  0x4038166cu64 => "
      CANFD_0.cfdrmdf_4()[4],
    ",
  0x403816b8u64 => "
      CANFD_0.cfdrmdf_4()[5],
    ",
  0x40381704u64 => "
      CANFD_0.cfdrmdf_4()[6],
    ",
  0x40381750u64 => "
      CANFD_0.cfdrmdf_4()[7],
    ",
  0x40381540u64 => "
      CANFD_0.cfdrmdf_5()[0],
    ",
  0x4038158cu64 => "
      CANFD_0.cfdrmdf_5()[1],
    ",
  0x403815d8u64 => "
      CANFD_0.cfdrmdf_5()[2],
    ",
  0x40381624u64 => "
      CANFD_0.cfdrmdf_5()[3],
    ",
  0x40381670u64 => "
      CANFD_0.cfdrmdf_5()[4],
    ",
  0x403816bcu64 => "
      CANFD_0.cfdrmdf_5()[5],
    ",
  0x40381708u64 => "
      CANFD_0.cfdrmdf_5()[6],
    ",
  0x40381754u64 => "
      CANFD_0.cfdrmdf_5()[7],
    ",
  0x40381544u64 => "
      CANFD_0.cfdrmdf_6()[0],
    ",
  0x40381590u64 => "
      CANFD_0.cfdrmdf_6()[1],
    ",
  0x403815dcu64 => "
      CANFD_0.cfdrmdf_6()[2],
    ",
  0x40381628u64 => "
      CANFD_0.cfdrmdf_6()[3],
    ",
  0x40381674u64 => "
      CANFD_0.cfdrmdf_6()[4],
    ",
  0x403816c0u64 => "
      CANFD_0.cfdrmdf_6()[5],
    ",
  0x4038170cu64 => "
      CANFD_0.cfdrmdf_6()[6],
    ",
  0x40381758u64 => "
      CANFD_0.cfdrmdf_6()[7],
    ",
  0x40381548u64 => "
      CANFD_0.cfdrmdf_7()[0],
    ",
  0x40381594u64 => "
      CANFD_0.cfdrmdf_7()[1],
    ",
  0x403815e0u64 => "
      CANFD_0.cfdrmdf_7()[2],
    ",
  0x4038162cu64 => "
      CANFD_0.cfdrmdf_7()[3],
    ",
  0x40381678u64 => "
      CANFD_0.cfdrmdf_7()[4],
    ",
  0x403816c4u64 => "
      CANFD_0.cfdrmdf_7()[5],
    ",
  0x40381710u64 => "
      CANFD_0.cfdrmdf_7()[6],
    ",
  0x4038175cu64 => "
      CANFD_0.cfdrmdf_7()[7],
    ",
  0x4038154cu64 => "
      CANFD_0.cfdrmdf_8()[0],
    ",
  0x40381598u64 => "
      CANFD_0.cfdrmdf_8()[1],
    ",
  0x403815e4u64 => "
      CANFD_0.cfdrmdf_8()[2],
    ",
  0x40381630u64 => "
      CANFD_0.cfdrmdf_8()[3],
    ",
  0x4038167cu64 => "
      CANFD_0.cfdrmdf_8()[4],
    ",
  0x403816c8u64 => "
      CANFD_0.cfdrmdf_8()[5],
    ",
  0x40381714u64 => "
      CANFD_0.cfdrmdf_8()[6],
    ",
  0x40381760u64 => "
      CANFD_0.cfdrmdf_8()[7],
    ",
  0x40381550u64 => "
      CANFD_0.cfdrmdf_9()[0],
    ",
  0x4038159cu64 => "
      CANFD_0.cfdrmdf_9()[1],
    ",
  0x403815e8u64 => "
      CANFD_0.cfdrmdf_9()[2],
    ",
  0x40381634u64 => "
      CANFD_0.cfdrmdf_9()[3],
    ",
  0x40381680u64 => "
      CANFD_0.cfdrmdf_9()[4],
    ",
  0x403816ccu64 => "
      CANFD_0.cfdrmdf_9()[5],
    ",
  0x40381718u64 => "
      CANFD_0.cfdrmdf_9()[6],
    ",
  0x40381764u64 => "
      CANFD_0.cfdrmdf_9()[7],
    ",
  0x40381554u64 => "
      CANFD_0.cfdrmdf_10()[0],
    ",
  0x403815a0u64 => "
      CANFD_0.cfdrmdf_10()[1],
    ",
  0x403815ecu64 => "
      CANFD_0.cfdrmdf_10()[2],
    ",
  0x40381638u64 => "
      CANFD_0.cfdrmdf_10()[3],
    ",
  0x40381684u64 => "
      CANFD_0.cfdrmdf_10()[4],
    ",
  0x403816d0u64 => "
      CANFD_0.cfdrmdf_10()[5],
    ",
  0x4038171cu64 => "
      CANFD_0.cfdrmdf_10()[6],
    ",
  0x40381768u64 => "
      CANFD_0.cfdrmdf_10()[7],
    ",
  0x40381558u64 => "
      CANFD_0.cfdrmdf_11()[0],
    ",
  0x403815a4u64 => "
      CANFD_0.cfdrmdf_11()[1],
    ",
  0x403815f0u64 => "
      CANFD_0.cfdrmdf_11()[2],
    ",
  0x4038163cu64 => "
      CANFD_0.cfdrmdf_11()[3],
    ",
  0x40381688u64 => "
      CANFD_0.cfdrmdf_11()[4],
    ",
  0x403816d4u64 => "
      CANFD_0.cfdrmdf_11()[5],
    ",
  0x40381720u64 => "
      CANFD_0.cfdrmdf_11()[6],
    ",
  0x4038176cu64 => "
      CANFD_0.cfdrmdf_11()[7],
    ",
  0x4038155cu64 => "
      CANFD_0.cfdrmdf_12()[0],
    ",
  0x403815a8u64 => "
      CANFD_0.cfdrmdf_12()[1],
    ",
  0x403815f4u64 => "
      CANFD_0.cfdrmdf_12()[2],
    ",
  0x40381640u64 => "
      CANFD_0.cfdrmdf_12()[3],
    ",
  0x4038168cu64 => "
      CANFD_0.cfdrmdf_12()[4],
    ",
  0x403816d8u64 => "
      CANFD_0.cfdrmdf_12()[5],
    ",
  0x40381724u64 => "
      CANFD_0.cfdrmdf_12()[6],
    ",
  0x40381770u64 => "
      CANFD_0.cfdrmdf_12()[7],
    ",
  0x40381560u64 => "
      CANFD_0.cfdrmdf_13()[0],
    ",
  0x403815acu64 => "
      CANFD_0.cfdrmdf_13()[1],
    ",
  0x403815f8u64 => "
      CANFD_0.cfdrmdf_13()[2],
    ",
  0x40381644u64 => "
      CANFD_0.cfdrmdf_13()[3],
    ",
  0x40381690u64 => "
      CANFD_0.cfdrmdf_13()[4],
    ",
  0x403816dcu64 => "
      CANFD_0.cfdrmdf_13()[5],
    ",
  0x40381728u64 => "
      CANFD_0.cfdrmdf_13()[6],
    ",
  0x40381774u64 => "
      CANFD_0.cfdrmdf_13()[7],
    ",
  0x40381564u64 => "
      CANFD_0.cfdrmdf_14()[0],
    ",
  0x403815b0u64 => "
      CANFD_0.cfdrmdf_14()[1],
    ",
  0x403815fcu64 => "
      CANFD_0.cfdrmdf_14()[2],
    ",
  0x40381648u64 => "
      CANFD_0.cfdrmdf_14()[3],
    ",
  0x40381694u64 => "
      CANFD_0.cfdrmdf_14()[4],
    ",
  0x403816e0u64 => "
      CANFD_0.cfdrmdf_14()[5],
    ",
  0x4038172cu64 => "
      CANFD_0.cfdrmdf_14()[6],
    ",
  0x40381778u64 => "
      CANFD_0.cfdrmdf_14()[7],
    ",
  0x40381568u64 => "
      CANFD_0.cfdrmdf_15()[0],
    ",
  0x403815b4u64 => "
      CANFD_0.cfdrmdf_15()[1],
    ",
  0x40381600u64 => "
      CANFD_0.cfdrmdf_15()[2],
    ",
  0x4038164cu64 => "
      CANFD_0.cfdrmdf_15()[3],
    ",
  0x40381698u64 => "
      CANFD_0.cfdrmdf_15()[4],
    ",
  0x403816e4u64 => "
      CANFD_0.cfdrmdf_15()[5],
    ",
  0x40381730u64 => "
      CANFD_0.cfdrmdf_15()[6],
    ",
  0x4038177cu64 => "
      CANFD_0.cfdrmdf_15()[7],
    ",
  0x40400000u64 => "
      PORT_0.pcntr1(),
      PORT_0.pdr(),
    ",
  0x40400002u64 => "
      PORT_0.podr(),
    ",
  0x40400004u64 => "
      PORT_0.pcntr2(),
      PORT_0.pidr(),
    ",
  0x40400008u64 => "
      PORT_0.pcntr3(),
      PORT_0.posr(),
    ",
  0x4040000au64 => "
      PORT_0.porr(),
    ",
  0x40400020u64 => "
      PORT_1.pcntr1(),
      PORT_1.pdr(),
    ",
  0x40400022u64 => "
      PORT_1.podr(),
    ",
  0x40400024u64 => "
      PORT_1.pcntr2(),
      PORT_1.pidr(),
    ",
  0x40400026u64 => "
      PORT_1.eidr(),
    ",
  0x40400028u64 => "
      PORT_1.pcntr3(),
      PORT_1.posr(),
    ",
  0x4040002au64 => "
      PORT_1.porr(),
    ",
  0x4040002cu64 => "
      PORT_1.pcntr4(),
      PORT_1.eosr(),
    ",
  0x4040002eu64 => "
      PORT_1.eorr(),
    ",
  0x40400140u64 => "
      PORTA.pcntr1(),
      PORTA.pdr(),
    ",
  0x40400142u64 => "
      PORTA.podr(),
    ",
  0x40400144u64 => "
      PORTA.pcntr2(),
      PORTA.pidr(),
    ",
  0x40400148u64 => "
      PORTA.pcntr3(),
      PORTA.posr(),
    ",
  0x4040014au64 => "
      PORTA.porr(),
    ",
  0x40400160u64 => "
      PORTB.pcntr1(),
      PORTB.pdr(),
    ",
  0x40400162u64 => "
      PORTB.podr(),
    ",
  0x40400164u64 => "
      PORTB.pcntr2(),
      PORTB.pidr(),
    ",
  0x40400168u64 => "
      PORTB.pcntr3(),
      PORTB.posr(),
    ",
  0x4040016au64 => "
      PORTB.porr(),
    ",
  0x40400800u64 => "
      PFS.p00pfs()[0],
      PFS.p00pfs_ha()[0],
      PFS.p00pfs_by()[0],
    ",
  0x40400804u64 => "
      PFS.p00pfs()[1],
      PFS.p00pfs_ha()[1],
      PFS.p00pfs_by()[1],
    ",
  0x40400808u64 => "
      PFS.p00pfs()[2],
      PFS.p00pfs_ha()[2],
      PFS.p00pfs_by()[2],
    ",
  0x4040080cu64 => "
      PFS.p00pfs()[3],
      PFS.p00pfs_ha()[3],
      PFS.p00pfs_by()[3],
    ",
  0x40400810u64 => "
      PFS.p00pfs()[4],
      PFS.p00pfs_ha()[4],
      PFS.p00pfs_by()[4],
    ",
  0x40400814u64 => "
      PFS.p00pfs()[5],
      PFS.p00pfs_ha()[5],
      PFS.p00pfs_by()[5],
    ",
  0x40400818u64 => "
      PFS.p00pfs()[6],
      PFS.p00pfs_ha()[6],
      PFS.p00pfs_by()[6],
    ",
  0x4040081cu64 => "
      PFS.p00pfs()[7],
      PFS.p00pfs_ha()[7],
      PFS.p00pfs_by()[7],
    ",
  0x40400820u64 => "
      PFS.p00pfs()[8],
      PFS.p00pfs_ha()[8],
      PFS.p00pfs_by()[8],
    ",
  0x40400824u64 => "
      PFS.p00pfs()[9],
      PFS.p00pfs_ha()[9],
      PFS.p00pfs_by()[9],
    ",
  0x40400838u64 => "
      PFS.p0pfs()[0],
      PFS.p0pfs_ha()[0],
      PFS.p0pfs_by()[0],
    ",
  0x4040083cu64 => "
      PFS.p0pfs()[1],
      PFS.p0pfs_ha()[1],
      PFS.p0pfs_by()[1],
    ",
  0x40400840u64 => "
      PFS.p10pfs()[0],
      PFS.p10pfs_ha()[0],
      PFS.p10pfs_by()[0],
    ",
  0x40400844u64 => "
      PFS.p10pfs()[1],
      PFS.p10pfs_ha()[1],
      PFS.p10pfs_by()[1],
    ",
  0x40400848u64 => "
      PFS.p10pfs()[2],
      PFS.p10pfs_ha()[2],
      PFS.p10pfs_by()[2],
    ",
  0x4040084cu64 => "
      PFS.p10pfs()[3],
      PFS.p10pfs_ha()[3],
      PFS.p10pfs_by()[3],
    ",
  0x40400850u64 => "
      PFS.p10pfs()[4],
      PFS.p10pfs_ha()[4],
      PFS.p10pfs_by()[4],
    ",
  0x40400854u64 => "
      PFS.p10pfs()[5],
      PFS.p10pfs_ha()[5],
      PFS.p10pfs_by()[5],
    ",
  0x40400858u64 => "
      PFS.p10pfs()[6],
      PFS.p10pfs_ha()[6],
      PFS.p10pfs_by()[6],
    ",
  0x4040085cu64 => "
      PFS.p10pfs()[7],
      PFS.p10pfs_ha()[7],
      PFS.p10pfs_by()[7],
    ",
  0x40400870u64 => "
      PFS.p1pfs()[0],
      PFS.p1pfs_ha()[0],
      PFS.p1pfs_by()[0],
    ",
  0x40400874u64 => "
      PFS.p1pfs()[1],
      PFS.p1pfs_ha()[1],
      PFS.p1pfs_by()[1],
    ",
  0x40400878u64 => "
      PFS.p1pfs()[2],
      PFS.p1pfs_ha()[2],
      PFS.p1pfs_by()[2],
    ",
  0x4040087cu64 => "
      PFS.p1pfs()[3],
      PFS.p1pfs_ha()[3],
      PFS.p1pfs_by()[3],
    ",
  0x40400880u64 => "
      PFS.p200pfs(),
      PFS.p200pfs_ha(),
      PFS.p200pfs_by(),
    ",
  0x40400884u64 => "
      PFS.p201pfs(),
      PFS.p201pfs_ha(),
      PFS.p201pfs_by(),
    ",
  0x40400888u64 => "
      PFS.p20pfs()[0],
      PFS.p20pfs_ha()[0],
      PFS.p20pfs_by()[0],
    ",
  0x4040088cu64 => "
      PFS.p20pfs()[1],
      PFS.p20pfs_ha()[1],
      PFS.p20pfs_by()[1],
    ",
  0x40400890u64 => "
      PFS.p20pfs()[2],
      PFS.p20pfs_ha()[2],
      PFS.p20pfs_by()[2],
    ",
  0x40400894u64 => "
      PFS.p20pfs()[3],
      PFS.p20pfs_ha()[3],
      PFS.p20pfs_by()[3],
    ",
  0x40400898u64 => "
      PFS.p20pfs()[4],
      PFS.p20pfs_ha()[4],
      PFS.p20pfs_by()[4],
    ",
  0x4040089cu64 => "
      PFS.p20pfs()[5],
      PFS.p20pfs_ha()[5],
      PFS.p20pfs_by()[5],
    ",
  0x404008a0u64 => "
      PFS.p208pfs(),
      PFS.p208pfs_ha(),
      PFS.p208pfs_by(),
    ",
  0x404008a4u64 => "
      PFS.p209pfs(),
      PFS.p209pfs_ha(),
      PFS.p209pfs_by(),
    ",
  0x404008a8u64 => "
      PFS.p210pfs(),
      PFS.p210pfs_ha(),
      PFS.p210pfs_by(),
    ",
  0x404008acu64 => "
      PFS.p211pfs(),
      PFS.p211pfs_ha(),
      PFS.p211pfs_by(),
    ",
  0x404008b0u64 => "
      PFS.p2pfs()[0],
      PFS.p2pfs_ha()[0],
      PFS.p2pfs_by()[0],
    ",
  0x404008b4u64 => "
      PFS.p2pfs()[1],
      PFS.p2pfs_ha()[1],
      PFS.p2pfs_by()[1],
    ",
  0x404008c0u64 => "
      PFS.p30pfs()[0],
      PFS.p30pfs_ha()[0],
      PFS.p30pfs_by()[0],
    ",
  0x404008c4u64 => "
      PFS.p30pfs()[1],
      PFS.p30pfs_ha()[1],
      PFS.p30pfs_by()[1],
    ",
  0x404008c8u64 => "
      PFS.p30pfs()[2],
      PFS.p30pfs_ha()[2],
      PFS.p30pfs_by()[2],
    ",
  0x404008ccu64 => "
      PFS.p30pfs()[3],
      PFS.p30pfs_ha()[3],
      PFS.p30pfs_by()[3],
    ",
  0x404008d0u64 => "
      PFS.p30pfs()[4],
      PFS.p30pfs_ha()[4],
      PFS.p30pfs_by()[4],
    ",
  0x404008d4u64 => "
      PFS.p30pfs()[5],
      PFS.p30pfs_ha()[5],
      PFS.p30pfs_by()[5],
    ",
  0x404008d8u64 => "
      PFS.p30pfs()[6],
      PFS.p30pfs_ha()[6],
      PFS.p30pfs_by()[6],
    ",
  0x404008dcu64 => "
      PFS.p30pfs()[7],
      PFS.p30pfs_ha()[7],
      PFS.p30pfs_by()[7],
    ",
  0x404008e0u64 => "
      PFS.p30pfs()[8],
      PFS.p30pfs_ha()[8],
      PFS.p30pfs_by()[8],
    ",
  0x404008e4u64 => "
      PFS.p30pfs()[9],
      PFS.p30pfs_ha()[9],
      PFS.p30pfs_by()[9],
    ",
  0x404008e8u64 => "
      PFS.p3pfs()[0],
      PFS.p3pfs_ha()[0],
      PFS.p3pfs_by()[0],
    ",
  0x404008ecu64 => "
      PFS.p3pfs()[1],
      PFS.p3pfs_ha()[1],
      PFS.p3pfs_by()[1],
    ",
  0x404008f0u64 => "
      PFS.p3pfs()[2],
      PFS.p3pfs_ha()[2],
      PFS.p3pfs_by()[2],
    ",
  0x404008f4u64 => "
      PFS.p3pfs()[3],
      PFS.p3pfs_ha()[3],
      PFS.p3pfs_by()[3],
    ",
  0x404008f8u64 => "
      PFS.p3pfs()[4],
      PFS.p3pfs_ha()[4],
      PFS.p3pfs_by()[4],
    ",
  0x404008fcu64 => "
      PFS.p3pfs()[5],
      PFS.p3pfs_ha()[5],
      PFS.p3pfs_by()[5],
    ",
  0x40400900u64 => "
      PFS.p40pfs()[0],
      PFS.p40pfs_ha()[0],
      PFS.p40pfs_by()[0],
    ",
  0x40400904u64 => "
      PFS.p40pfs()[1],
      PFS.p40pfs_ha()[1],
      PFS.p40pfs_by()[1],
    ",
  0x40400908u64 => "
      PFS.p40pfs()[2],
      PFS.p40pfs_ha()[2],
      PFS.p40pfs_by()[2],
    ",
  0x4040090cu64 => "
      PFS.p40pfs()[3],
      PFS.p40pfs_ha()[3],
      PFS.p40pfs_by()[3],
    ",
  0x40400910u64 => "
      PFS.p40pfs()[4],
      PFS.p40pfs_ha()[4],
      PFS.p40pfs_by()[4],
    ",
  0x40400914u64 => "
      PFS.p40pfs()[5],
      PFS.p40pfs_ha()[5],
      PFS.p40pfs_by()[5],
    ",
  0x40400918u64 => "
      PFS.p40pfs()[6],
      PFS.p40pfs_ha()[6],
      PFS.p40pfs_by()[6],
    ",
  0x4040091cu64 => "
      PFS.p40pfs()[7],
      PFS.p40pfs_ha()[7],
      PFS.p40pfs_by()[7],
    ",
  0x40400920u64 => "
      PFS.p40pfs()[8],
      PFS.p40pfs_ha()[8],
      PFS.p40pfs_by()[8],
    ",
  0x40400924u64 => "
      PFS.p40pfs()[9],
      PFS.p40pfs_ha()[9],
      PFS.p40pfs_by()[9],
    ",
  0x40400928u64 => "
      PFS.p4pfs()[0],
      PFS.p4pfs_ha()[0],
      PFS.p4pfs_by()[0],
    ",
  0x4040092cu64 => "
      PFS.p4pfs()[1],
      PFS.p4pfs_ha()[1],
      PFS.p4pfs_by()[1],
    ",
  0x40400930u64 => "
      PFS.p4pfs()[2],
      PFS.p4pfs_ha()[2],
      PFS.p4pfs_by()[2],
    ",
  0x40400934u64 => "
      PFS.p4pfs()[3],
      PFS.p4pfs_ha()[3],
      PFS.p4pfs_by()[3],
    ",
  0x40400938u64 => "
      PFS.p4pfs()[4],
      PFS.p4pfs_ha()[4],
      PFS.p4pfs_by()[4],
    ",
  0x4040093cu64 => "
      PFS.p4pfs()[5],
      PFS.p4pfs_ha()[5],
      PFS.p4pfs_by()[5],
    ",
  0x40400940u64 => "
      PFS.p50pfs()[0],
      PFS.p50pfs_ha()[0],
      PFS.p50pfs_by()[0],
    ",
  0x40400944u64 => "
      PFS.p50pfs()[1],
      PFS.p50pfs_ha()[1],
      PFS.p50pfs_by()[1],
    ",
  0x40400948u64 => "
      PFS.p50pfs()[2],
      PFS.p50pfs_ha()[2],
      PFS.p50pfs_by()[2],
    ",
  0x4040094cu64 => "
      PFS.p50pfs()[3],
      PFS.p50pfs_ha()[3],
      PFS.p50pfs_by()[3],
    ",
  0x40400950u64 => "
      PFS.p50pfs()[4],
      PFS.p50pfs_ha()[4],
      PFS.p50pfs_by()[4],
    ",
  0x40400954u64 => "
      PFS.p50pfs()[5],
      PFS.p50pfs_ha()[5],
      PFS.p50pfs_by()[5],
    ",
  0x40400958u64 => "
      PFS.p50pfs()[6],
      PFS.p50pfs_ha()[6],
      PFS.p50pfs_by()[6],
    ",
  0x4040095cu64 => "
      PFS.p50pfs()[7],
      PFS.p50pfs_ha()[7],
      PFS.p50pfs_by()[7],
    ",
  0x40400960u64 => "
      PFS.p50pfs()[8],
      PFS.p50pfs_ha()[8],
      PFS.p50pfs_by()[8],
    ",
  0x40400964u64 => "
      PFS.p50pfs()[9],
      PFS.p50pfs_ha()[9],
      PFS.p50pfs_by()[9],
    ",
  0x40400968u64 => "
      PFS.p5pfs()[0],
      PFS.p5pfs_ha()[0],
      PFS.p5pfs_by()[0],
    ",
  0x4040096cu64 => "
      PFS.p5pfs()[1],
      PFS.p5pfs_ha()[1],
      PFS.p5pfs_by()[1],
    ",
  0x40400970u64 => "
      PFS.p5pfs()[2],
      PFS.p5pfs_ha()[2],
      PFS.p5pfs_by()[2],
    ",
  0x40400974u64 => "
      PFS.p5pfs()[3],
      PFS.p5pfs_ha()[3],
      PFS.p5pfs_by()[3],
    ",
  0x40400978u64 => "
      PFS.p5pfs()[4],
      PFS.p5pfs_ha()[4],
      PFS.p5pfs_by()[4],
    ",
  0x4040097cu64 => "
      PFS.p5pfs()[5],
      PFS.p5pfs_ha()[5],
      PFS.p5pfs_by()[5],
    ",
  0x40400980u64 => "
      PFS.p60pfs()[0],
      PFS.p60pfs_ha()[0],
      PFS.p60pfs_by()[0],
    ",
  0x40400984u64 => "
      PFS.p60pfs()[1],
      PFS.p60pfs_ha()[1],
      PFS.p60pfs_by()[1],
    ",
  0x40400988u64 => "
      PFS.p60pfs()[2],
      PFS.p60pfs_ha()[2],
      PFS.p60pfs_by()[2],
    ",
  0x4040098cu64 => "
      PFS.p60pfs()[3],
      PFS.p60pfs_ha()[3],
      PFS.p60pfs_by()[3],
    ",
  0x40400990u64 => "
      PFS.p60pfs()[4],
      PFS.p60pfs_ha()[4],
      PFS.p60pfs_by()[4],
    ",
  0x40400994u64 => "
      PFS.p60pfs()[5],
      PFS.p60pfs_ha()[5],
      PFS.p60pfs_by()[5],
    ",
  0x40400998u64 => "
      PFS.p60pfs()[6],
      PFS.p60pfs_ha()[6],
      PFS.p60pfs_by()[6],
    ",
  0x4040099cu64 => "
      PFS.p60pfs()[7],
      PFS.p60pfs_ha()[7],
      PFS.p60pfs_by()[7],
    ",
  0x404009a0u64 => "
      PFS.p60pfs()[8],
      PFS.p60pfs_ha()[8],
      PFS.p60pfs_by()[8],
    ",
  0x404009a4u64 => "
      PFS.p60pfs()[9],
      PFS.p60pfs_ha()[9],
      PFS.p60pfs_by()[9],
    ",
  0x404009a8u64 => "
      PFS.p6pfs()[0],
      PFS.p6pfs_ha()[0],
      PFS.p6pfs_by()[0],
    ",
  0x404009acu64 => "
      PFS.p6pfs()[1],
      PFS.p6pfs_ha()[1],
      PFS.p6pfs_by()[1],
    ",
  0x404009b0u64 => "
      PFS.p6pfs()[2],
      PFS.p6pfs_ha()[2],
      PFS.p6pfs_by()[2],
    ",
  0x404009b4u64 => "
      PFS.p6pfs()[3],
      PFS.p6pfs_ha()[3],
      PFS.p6pfs_by()[3],
    ",
  0x404009b8u64 => "
      PFS.p6pfs()[4],
      PFS.p6pfs_ha()[4],
      PFS.p6pfs_by()[4],
    ",
  0x404009bcu64 => "
      PFS.p6pfs()[5],
      PFS.p6pfs_ha()[5],
      PFS.p6pfs_by()[5],
    ",
  0x404009c0u64 => "
      PFS.p70pfs()[0],
      PFS.p70pfs_ha()[0],
      PFS.p70pfs_by()[0],
    ",
  0x404009c4u64 => "
      PFS.p70pfs()[1],
      PFS.p70pfs_ha()[1],
      PFS.p70pfs_by()[1],
    ",
  0x404009c8u64 => "
      PFS.p70pfs()[2],
      PFS.p70pfs_ha()[2],
      PFS.p70pfs_by()[2],
    ",
  0x404009ccu64 => "
      PFS.p70pfs()[3],
      PFS.p70pfs_ha()[3],
      PFS.p70pfs_by()[3],
    ",
  0x404009d0u64 => "
      PFS.p70pfs()[4],
      PFS.p70pfs_ha()[4],
      PFS.p70pfs_by()[4],
    ",
  0x404009d4u64 => "
      PFS.p70pfs()[5],
      PFS.p70pfs_ha()[5],
      PFS.p70pfs_by()[5],
    ",
  0x404009d8u64 => "
      PFS.p70pfs()[6],
      PFS.p70pfs_ha()[6],
      PFS.p70pfs_by()[6],
    ",
  0x404009dcu64 => "
      PFS.p70pfs()[7],
      PFS.p70pfs_ha()[7],
      PFS.p70pfs_by()[7],
    ",
  0x404009e0u64 => "
      PFS.p70pfs()[8],
      PFS.p70pfs_ha()[8],
      PFS.p70pfs_by()[8],
    ",
  0x404009e4u64 => "
      PFS.p70pfs()[9],
      PFS.p70pfs_ha()[9],
      PFS.p70pfs_by()[9],
    ",
  0x404009e8u64 => "
      PFS.p7pfs()[0],
      PFS.p7pfs_ha()[0],
      PFS.p7pfs_by()[0],
    ",
  0x404009ecu64 => "
      PFS.p7pfs()[1],
      PFS.p7pfs_ha()[1],
      PFS.p7pfs_by()[1],
    ",
  0x404009f0u64 => "
      PFS.p7pfs()[2],
      PFS.p7pfs_ha()[2],
      PFS.p7pfs_by()[2],
    ",
  0x404009f4u64 => "
      PFS.p7pfs()[3],
      PFS.p7pfs_ha()[3],
      PFS.p7pfs_by()[3],
    ",
  0x404009f8u64 => "
      PFS.p7pfs()[4],
      PFS.p7pfs_ha()[4],
      PFS.p7pfs_by()[4],
    ",
  0x404009fcu64 => "
      PFS.p7pfs()[5],
      PFS.p7pfs_ha()[5],
      PFS.p7pfs_by()[5],
    ",
  0x40400a00u64 => "
      PFS.p80pfs()[0],
      PFS.p80pfs_ha()[0],
      PFS.p80pfs_by()[0],
    ",
  0x40400a04u64 => "
      PFS.p80pfs()[1],
      PFS.p80pfs_ha()[1],
      PFS.p80pfs_by()[1],
    ",
  0x40400a08u64 => "
      PFS.p80pfs()[2],
      PFS.p80pfs_ha()[2],
      PFS.p80pfs_by()[2],
    ",
  0x40400a0cu64 => "
      PFS.p80pfs()[3],
      PFS.p80pfs_ha()[3],
      PFS.p80pfs_by()[3],
    ",
  0x40400a10u64 => "
      PFS.p80pfs()[4],
      PFS.p80pfs_ha()[4],
      PFS.p80pfs_by()[4],
    ",
  0x40400a14u64 => "
      PFS.p80pfs()[5],
      PFS.p80pfs_ha()[5],
      PFS.p80pfs_by()[5],
    ",
  0x40400a18u64 => "
      PFS.p80pfs()[6],
      PFS.p80pfs_ha()[6],
      PFS.p80pfs_by()[6],
    ",
  0x40400a1cu64 => "
      PFS.p80pfs()[7],
      PFS.p80pfs_ha()[7],
      PFS.p80pfs_by()[7],
    ",
  0x40400a20u64 => "
      PFS.p80pfs()[8],
      PFS.p80pfs_ha()[8],
      PFS.p80pfs_by()[8],
    ",
  0x40400a24u64 => "
      PFS.p80pfs()[9],
      PFS.p80pfs_ha()[9],
      PFS.p80pfs_by()[9],
    ",
  0x40400a30u64 => "
      PFS.p8pfs()[2],
      PFS.p8pfs_ha()[2],
      PFS.p8pfs_by()[2],
    ",
  0x40400a34u64 => "
      PFS.p8pfs()[3],
      PFS.p8pfs_ha()[3],
      PFS.p8pfs_by()[3],
    ",
  0x40400a38u64 => "
      PFS.p8pfs()[0],
      PFS.p8pfs_ha()[0],
      PFS.p8pfs_by()[0],
    ",
  0x40400a3cu64 => "
      PFS.p8pfs()[1],
      PFS.p8pfs_ha()[1],
      PFS.p8pfs_by()[1],
    ",
  0x40400a40u64 => "
      PFS.p90pfs()[0],
      PFS.p90pfs_ha()[0],
      PFS.p90pfs_by()[0],
    ",
  0x40400a44u64 => "
      PFS.p90pfs()[1],
      PFS.p90pfs_ha()[1],
      PFS.p90pfs_by()[1],
    ",
  0x40400a48u64 => "
      PFS.p90pfs()[2],
      PFS.p90pfs_ha()[2],
      PFS.p90pfs_by()[2],
    ",
  0x40400a4cu64 => "
      PFS.p90pfs()[3],
      PFS.p90pfs_ha()[3],
      PFS.p90pfs_by()[3],
    ",
  0x40400a50u64 => "
      PFS.p90pfs()[4],
      PFS.p90pfs_ha()[4],
      PFS.p90pfs_by()[4],
    ",
  0x40400a54u64 => "
      PFS.p90pfs()[5],
      PFS.p90pfs_ha()[5],
      PFS.p90pfs_by()[5],
    ",
  0x40400a58u64 => "
      PFS.p90pfs()[6],
      PFS.p90pfs_ha()[6],
      PFS.p90pfs_by()[6],
    ",
  0x40400a5cu64 => "
      PFS.p90pfs()[7],
      PFS.p90pfs_ha()[7],
      PFS.p90pfs_by()[7],
    ",
  0x40400a60u64 => "
      PFS.p90pfs()[8],
      PFS.p90pfs_ha()[8],
      PFS.p90pfs_by()[8],
    ",
  0x40400a64u64 => "
      PFS.p90pfs()[9],
      PFS.p90pfs_ha()[9],
      PFS.p90pfs_by()[9],
    ",
  0x40400a68u64 => "
      PFS.p9pfs()[0],
      PFS.p9pfs_ha()[0],
      PFS.p9pfs_by()[0],
    ",
  0x40400a6cu64 => "
      PFS.p9pfs()[1],
      PFS.p9pfs_ha()[1],
      PFS.p9pfs_by()[1],
    ",
  0x40400a70u64 => "
      PFS.p9pfs()[2],
      PFS.p9pfs_ha()[2],
      PFS.p9pfs_by()[2],
    ",
  0x40400a74u64 => "
      PFS.p9pfs()[3],
      PFS.p9pfs_ha()[3],
      PFS.p9pfs_by()[3],
    ",
  0x40400a78u64 => "
      PFS.p9pfs()[4],
      PFS.p9pfs_ha()[4],
      PFS.p9pfs_by()[4],
    ",
  0x40400a7cu64 => "
      PFS.p9pfs()[5],
      PFS.p9pfs_ha()[5],
      PFS.p9pfs_by()[5],
    ",
  0x40400a80u64 => "
      PFS.pa0pfs()[0],
      PFS.pa0pfs_ha()[0],
      PFS.pa0pfs_by()[0],
    ",
  0x40400a84u64 => "
      PFS.pa0pfs()[1],
      PFS.pa0pfs_ha()[1],
      PFS.pa0pfs_by()[1],
    ",
  0x40400a88u64 => "
      PFS.pa0pfs()[2],
      PFS.pa0pfs_ha()[2],
      PFS.pa0pfs_by()[2],
    ",
  0x40400a8cu64 => "
      PFS.pa0pfs()[3],
      PFS.pa0pfs_ha()[3],
      PFS.pa0pfs_by()[3],
    ",
  0x40400a90u64 => "
      PFS.pa0pfs()[4],
      PFS.pa0pfs_ha()[4],
      PFS.pa0pfs_by()[4],
    ",
  0x40400a94u64 => "
      PFS.pa0pfs()[5],
      PFS.pa0pfs_ha()[5],
      PFS.pa0pfs_by()[5],
    ",
  0x40400a98u64 => "
      PFS.pa0pfs()[6],
      PFS.pa0pfs_ha()[6],
      PFS.pa0pfs_by()[6],
    ",
  0x40400a9cu64 => "
      PFS.pa0pfs()[7],
      PFS.pa0pfs_ha()[7],
      PFS.pa0pfs_by()[7],
    ",
  0x40400aa0u64 => "
      PFS.pa0pfs()[8],
      PFS.pa0pfs_ha()[8],
      PFS.pa0pfs_by()[8],
    ",
  0x40400aa4u64 => "
      PFS.pa0pfs()[9],
      PFS.pa0pfs_ha()[9],
      PFS.pa0pfs_by()[9],
    ",
  0x40400aa8u64 => "
      PFS.papfs()[0],
      PFS.papfs_ha()[0],
      PFS.papfs_by()[0],
    ",
  0x40400aacu64 => "
      PFS.papfs()[1],
      PFS.papfs_ha()[1],
      PFS.papfs_by()[1],
    ",
  0x40400ab0u64 => "
      PFS.papfs()[2],
      PFS.papfs_ha()[2],
      PFS.papfs_by()[2],
    ",
  0x40400ab4u64 => "
      PFS.papfs()[3],
      PFS.papfs_ha()[3],
      PFS.papfs_by()[3],
    ",
  0x40400ab8u64 => "
      PFS.papfs()[4],
      PFS.papfs_ha()[4],
      PFS.papfs_by()[4],
    ",
  0x40400abcu64 => "
      PFS.papfs()[5],
      PFS.papfs_ha()[5],
      PFS.papfs_by()[5],
    ",
  0x40400ac0u64 => "
      PFS.pb0pfs()[0],
      PFS.pb0pfs_ha()[0],
      PFS.pb0pfs_by()[0],
    ",
  0x40400ac4u64 => "
      PFS.pb0pfs()[1],
      PFS.pb0pfs_ha()[1],
      PFS.pb0pfs_by()[1],
    ",
  0x40400ac8u64 => "
      PFS.pb0pfs()[2],
      PFS.pb0pfs_ha()[2],
      PFS.pb0pfs_by()[2],
    ",
  0x40400accu64 => "
      PFS.pb0pfs()[3],
      PFS.pb0pfs_ha()[3],
      PFS.pb0pfs_by()[3],
    ",
  0x40400ad0u64 => "
      PFS.pb0pfs()[4],
      PFS.pb0pfs_ha()[4],
      PFS.pb0pfs_by()[4],
    ",
  0x40400ad4u64 => "
      PFS.pb0pfs()[5],
      PFS.pb0pfs_ha()[5],
      PFS.pb0pfs_by()[5],
    ",
  0x40400ad8u64 => "
      PFS.pb0pfs()[6],
      PFS.pb0pfs_ha()[6],
      PFS.pb0pfs_by()[6],
    ",
  0x40400adcu64 => "
      PFS.pb0pfs()[7],
      PFS.pb0pfs_ha()[7],
      PFS.pb0pfs_by()[7],
    ",
  0x40400d00u64 => "
      PFS.pfenet(),
    ",
  0x40400d14u64 => "
      PFS.pwpr_s(),
    ",
  0x40400d38u64 => "
      PFS.psar()[2],
    ",
  0x40400d3cu64 => "
      PFS.psar()[3],
    ",
  0x40400d40u64 => "
      PFS.psar()[4],
    ",
  0x40400d44u64 => "
      PFS.psar()[5],
    ",
  0x40400d48u64 => "
      PFS.psar()[6],
    ",
  0x40400d4cu64 => "
      PFS.psar()[7],
    ",
  0x40400d50u64 => "
      PFS.psar()[8],
    ",
  0x40400d54u64 => "
      PFS.psar()[9],
    ",
  0x40400d58u64 => "
      PFS.psar()[0],
    ",
  0x40400d5cu64 => "
      PFS.psar()[1],
    ",
  0x50000000u64 => "
      RMPU_NS.mmpuoad(),
    ",
  0x50000004u64 => "
      RMPU_NS.mmpuoadpt(),
    ",
  0x50000100u64 => "
      RMPU_NS.mmpuendmac(),
    ",
  0x50000104u64 => "
      RMPU_NS.mmpuenptdmac(),
    ",
  0x50000108u64 => "
      RMPU_NS.mmpurptdmac(),
    ",
  0x50000200u64 => "
      RMPU_NS.mmpuacdmac()[0],
    ",
  0x50000210u64 => "
      RMPU_NS.mmpuacdmac()[1],
    ",
  0x50000220u64 => "
      RMPU_NS.mmpuacdmac()[2],
    ",
  0x50000230u64 => "
      RMPU_NS.mmpuacdmac()[3],
    ",
  0x50000240u64 => "
      RMPU_NS.mmpuacdmac()[4],
    ",
  0x50000250u64 => "
      RMPU_NS.mmpuacdmac()[5],
    ",
  0x50000260u64 => "
      RMPU_NS.mmpuacdmac()[6],
    ",
  0x50000270u64 => "
      RMPU_NS.mmpuacdmac()[7],
    ",
  0x50000204u64 => "
      RMPU_NS.mmpusdmac()[0],
    ",
  0x50000214u64 => "
      RMPU_NS.mmpusdmac()[1],
    ",
  0x50000224u64 => "
      RMPU_NS.mmpusdmac()[2],
    ",
  0x50000234u64 => "
      RMPU_NS.mmpusdmac()[3],
    ",
  0x50000244u64 => "
      RMPU_NS.mmpusdmac()[4],
    ",
  0x50000254u64 => "
      RMPU_NS.mmpusdmac()[5],
    ",
  0x50000264u64 => "
      RMPU_NS.mmpusdmac()[6],
    ",
  0x50000274u64 => "
      RMPU_NS.mmpusdmac()[7],
    ",
  0x50000208u64 => "
      RMPU_NS.mmpuedmac()[0],
    ",
  0x50000218u64 => "
      RMPU_NS.mmpuedmac()[1],
    ",
  0x50000228u64 => "
      RMPU_NS.mmpuedmac()[2],
    ",
  0x50000238u64 => "
      RMPU_NS.mmpuedmac()[3],
    ",
  0x50000248u64 => "
      RMPU_NS.mmpuedmac()[4],
    ",
  0x50000258u64 => "
      RMPU_NS.mmpuedmac()[5],
    ",
  0x50000268u64 => "
      RMPU_NS.mmpuedmac()[6],
    ",
  0x50000278u64 => "
      RMPU_NS.mmpuedmac()[7],
    ",
  0x50000500u64 => "
      RMPU_NS.mmpuenedmac(),
    ",
  0x50000504u64 => "
      RMPU_NS.mmpuenptedmac(),
    ",
  0x50000508u64 => "
      RMPU_NS.mmpurptedmac(),
    ",
  0x50000600u64 => "
      RMPU_NS.mmpuacedmac()[0],
    ",
  0x50000610u64 => "
      RMPU_NS.mmpuacedmac()[1],
    ",
  0x50000620u64 => "
      RMPU_NS.mmpuacedmac()[2],
    ",
  0x50000630u64 => "
      RMPU_NS.mmpuacedmac()[3],
    ",
  0x50000604u64 => "
      RMPU_NS.mmpusedmac()[0],
    ",
  0x50000614u64 => "
      RMPU_NS.mmpusedmac()[1],
    ",
  0x50000624u64 => "
      RMPU_NS.mmpusedmac()[2],
    ",
  0x50000634u64 => "
      RMPU_NS.mmpusedmac()[3],
    ",
  0x50000608u64 => "
      RMPU_NS.mmpueedmac()[0],
    ",
  0x50000618u64 => "
      RMPU_NS.mmpueedmac()[1],
    ",
  0x50000628u64 => "
      RMPU_NS.mmpueedmac()[2],
    ",
  0x50000638u64 => "
      RMPU_NS.mmpueedmac()[3],
    ",
  0x50000d00u64 => "
      RMPU_NS.mmpuenceu(),
    ",
  0x50000d04u64 => "
      RMPU_NS.mmpuenptceu(),
    ",
  0x50000d08u64 => "
      RMPU_NS.mmpurptceu(),
    ",
  0x50000e00u64 => "
      RMPU_NS.mmpuacceu()[0],
    ",
  0x50000e10u64 => "
      RMPU_NS.mmpuacceu()[1],
    ",
  0x50000e04u64 => "
      RMPU_NS.mmpusceu()[0],
    ",
  0x50000e14u64 => "
      RMPU_NS.mmpusceu()[1],
    ",
  0x50000e08u64 => "
      RMPU_NS.mmpueceu()[0],
    ",
  0x50000e18u64 => "
      RMPU_NS.mmpueceu()[1],
    ",
  0x50002004u64 => "
      SRAM_NS.sramprcr_ns(),
    ",
  0x50002008u64 => "
      SRAM_NS.sramwtsc(),
    ",
  0x50002014u64 => "
      SRAM_NS.sramcr1(),
    ",
  0x50002040u64 => "
      SRAM_NS.sramesr(),
    ",
  0x50002048u64 => "
      SRAM_NS.sramesclr(),
    ",
  0x50002058u64 => "
      SRAM_NS.sramear2(),
    ",
  0x50002110u64 => "
      SRAM_NS.stbramcr(),
    ",
  0x50002150u64 => "
      SRAM_NS.stbramear(),
    ",
  0x50003802u64 => "
      BUS_NS.cs0cr(),
    ",
  0x50003812u64 => "
      BUS_NS.cscr()[0],
    ",
  0x50003822u64 => "
      BUS_NS.cscr()[1],
    ",
  0x50003832u64 => "
      BUS_NS.cscr()[2],
    ",
  0x50003842u64 => "
      BUS_NS.cscr()[3],
    ",
  0x50003852u64 => "
      BUS_NS.cscr()[4],
    ",
  0x50003862u64 => "
      BUS_NS.cscr()[5],
    ",
  0x50003872u64 => "
      BUS_NS.cscr()[6],
    ",
  0x50004000u64 => "
      BUS_NS.busoad(),
    ",
  0x50004004u64 => "
      BUS_NS.busoadpt(),
    ",
  0x50004200u64 => "
      BUS_NS.bussabt1fhbi(),
    ",
  0x50004210u64 => "
      BUS_NS.bussabt0flbi(),
    ",
  0x50004220u64 => "
      BUS_NS.bussabt1s1bi(),
    ",
  0x50004248u64 => "
      BUS_NS.bussabt0stbysbi(),
    ",
  0x50004258u64 => "
      BUS_NS.bussabt0eobi(),
    ",
  0x50004260u64 => "
      BUS_NS.bussabt0pbbi(),
    ",
  0x50004268u64 => "
      BUS_NS.bussabt0pabi(),
    ",
  0x50004270u64 => "
      BUS_NS.bussabt0pibi(),
    ",
  0x50004278u64 => "
      BUS_NS.bussabt0psbi(),
    ",
  0x50004300u64 => "
      BUS_NS.busdivbyp(),
    ",
  0x50004804u64 => "
      BUS_NS.buserrrw()[0],
    ",
  0x50004814u64 => "
      BUS_NS.buserrrw()[1],
    ",
  0x50004830u64 => "
      BUS_NS.buserradd()[0],
    ",
  0x50004840u64 => "
      BUS_NS.buserradd()[1],
    ",
  0x50004900u64 => "
      BUS_NS.bmsaerradd()[0],
    ",
  0x50004910u64 => "
      BUS_NS.bmsaerradd()[1],
    ",
  0x50004904u64 => "
      BUS_NS.bmsaerrrw()[0],
    ",
  0x50004914u64 => "
      BUS_NS.bmsaerrrw()[1],
    ",
  0x50004a10u64 => "
      BUS_NS.buserrstat()[1],
    ",
  0x50004a20u64 => "
      BUS_NS.buserrstat()[2],
    ",
  0x50004a30u64 => "
      BUS_NS.buserrstat()[3],
    ",
  0x50004a40u64 => "
      BUS_NS.buserrstat()[4],
    ",
  0x50004a18u64 => "
      BUS_NS.buserrclr()[1],
    ",
  0x50004a28u64 => "
      BUS_NS.buserrclr()[2],
    ",
  0x50004a38u64 => "
      BUS_NS.buserrclr()[3],
    ",
  0x50004a48u64 => "
      BUS_NS.buserrclr()[4],
    ",
  0x50004a90u64 => "
      BUS_NS.buserrstat()[0],
    ",
  0x50004a98u64 => "
      BUS_NS.buserrclr()[0],
    ",
  0x50004b00u64 => "
      BUS_NS.mbwerrstat(),
    ",
  0x50004b08u64 => "
      BUS_NS.mbwerrclr(),
    ",
  0x50004b20u64 => "
      BUS_NS.sbwerrstat(),
    ",
  0x50004b28u64 => "
      BUS_NS.sbwerrclr(),
    ",
  0x50006000u64 => "
      ICU_COMMON_NS.irqcr()[0],
    ",
  0x50006001u64 => "
      ICU_COMMON_NS.irqcr()[1],
    ",
  0x50006002u64 => "
      ICU_COMMON_NS.irqcr()[2],
    ",
  0x50006003u64 => "
      ICU_COMMON_NS.irqcr()[3],
    ",
  0x50006004u64 => "
      ICU_COMMON_NS.irqcr()[4],
    ",
  0x50006005u64 => "
      ICU_COMMON_NS.irqcr()[5],
    ",
  0x50006006u64 => "
      ICU_COMMON_NS.irqcr()[6],
    ",
  0x50006007u64 => "
      ICU_COMMON_NS.irqcr()[7],
    ",
  0x50006008u64 => "
      ICU_COMMON_NS.irqcr()[8],
    ",
  0x50006009u64 => "
      ICU_COMMON_NS.irqcr()[9],
    ",
  0x5000600au64 => "
      ICU_COMMON_NS.irqcr()[10],
    ",
  0x5000600bu64 => "
      ICU_COMMON_NS.irqcr()[11],
    ",
  0x5000600cu64 => "
      ICU_COMMON_NS.irqcr()[12],
    ",
  0x5000600du64 => "
      ICU_COMMON_NS.irqcr()[13],
    ",
  0x5000600eu64 => "
      ICU_COMMON_NS.irqcr()[14],
    ",
  0x5000600fu64 => "
      ICU_COMMON_NS.irqcr()[15],
    ",
  0x50006010u64 => "
      ICU_COMMON_NS.nmicr(),
    ",
  0x50008010u64 => "
      CPSCU_NS.sramsar(),
    ",
  0x50008030u64 => "
      CPSCU_NS.dtcsar(),
    ",
  0x50008034u64 => "
      CPSCU_NS.dmacsar(),
    ",
  0x50008040u64 => "
      CPSCU_NS.icusara(),
    ",
  0x50008044u64 => "
      CPSCU_NS.icusarb(),
    ",
  0x50008050u64 => "
      CPSCU_NS.icusare(),
    ",
  0x50008054u64 => "
      CPSCU_NS.icusarf(),
    ",
  0x50008070u64 => "
      CPSCU_NS.icusarg(),
    ",
  0x50008074u64 => "
      CPSCU_NS.icusarh(),
    ",
  0x50008078u64 => "
      CPSCU_NS.icusari(),
    ",
  0x50008100u64 => "
      CPSCU_NS.bussara(),
    ",
  0x50008104u64 => "
      CPSCU_NS.bussarb(),
    ",
  0x50008130u64 => "
      CPSCU_NS.mmpusara(),
    ",
  0x50008134u64 => "
      CPSCU_NS.mmpusarb(),
    ",
  0x50008170u64 => "
      CPSCU_NS.cpusar(),
    ",
  0x50008180u64 => "
      CPSCU_NS.debugsar(),
    ",
  0x500081a0u64 => "
      CPSCU_NS.dmacchsar(),
    ",
  0x500081f0u64 => "
      CPSCU_NS.dmacchpar(),
    ",
  0x50008404u64 => "
      CPSCU_NS.sramsabar1(),
    ",
  0x50008420u64 => "
      CPSCU_NS.stbramsabar(),
    ",
  0x50008490u64 => "
      CPSCU_NS.stbrampabar_ns(),
    ",
  0x50008600u64 => "
      CPSCU_NS.tevtrcr(),
    ",
  0x5000a000u64 => "
      DMAC_00_NS.dmsar(),
    ",
  0x5000a004u64 => "
      DMAC_00_NS.dmdar(),
    ",
  0x5000a008u64 => "
      DMAC_00_NS.dmcra(),
    ",
  0x5000a00cu64 => "
      DMAC_00_NS.dmcrb(),
    ",
  0x5000a010u64 => "
      DMAC_00_NS.dmtmd(),
    ",
  0x5000a013u64 => "
      DMAC_00_NS.dmint(),
    ",
  0x5000a014u64 => "
      DMAC_00_NS.dmamd(),
    ",
  0x5000a018u64 => "
      DMAC_00_NS.dmofr(),
    ",
  0x5000a01cu64 => "
      DMAC_00_NS.dmcnt(),
    ",
  0x5000a01du64 => "
      DMAC_00_NS.dmreq(),
    ",
  0x5000a01eu64 => "
      DMAC_00_NS.dmsts(),
    ",
  0x5000a020u64 => "
      DMAC_00_NS.dmsrr(),
    ",
  0x5000a024u64 => "
      DMAC_00_NS.dmdrr(),
    ",
  0x5000a028u64 => "
      DMAC_00_NS.dmsbs(),
    ",
  0x5000a02cu64 => "
      DMAC_00_NS.dmdbs(),
    ",
  0x5000a030u64 => "
      DMAC_00_NS.dmbwr(),
    ",
  0x5000a800u64 => "
      DMA_0_NS.dmast(),
    ",
  0x5000a840u64 => "
      DMA_0_NS.dmechr(),
    ",
  0x5000a880u64 => "
      DMA_0_NS.delsr()[0],
    ",
  0x5000a884u64 => "
      DMA_0_NS.delsr()[1],
    ",
  0x5000a888u64 => "
      DMA_0_NS.delsr()[2],
    ",
  0x5000a88cu64 => "
      DMA_0_NS.delsr()[3],
    ",
  0x5000a890u64 => "
      DMA_0_NS.delsr()[4],
    ",
  0x5000a894u64 => "
      DMA_0_NS.delsr()[5],
    ",
  0x5000a898u64 => "
      DMA_0_NS.delsr()[6],
    ",
  0x5000a89cu64 => "
      DMA_0_NS.delsr()[7],
    ",
  0x5000ac00u64 => "
      DTC_0_NS.dtccr(),
    ",
  0x5000ac04u64 => "
      DTC_0_NS.dtcvbr(),
    ",
  0x5000ac0cu64 => "
      DTC_0_NS.dtcst(),
    ",
  0x5000ac0eu64 => "
      DTC_0_NS.dtcsts(),
    ",
  0x5000ac20u64 => "
      DTC_0_NS.dtevr(),
    ",
  0x5000c100u64 => "
      ICU_NS.nmier(),
    ",
  0x5000c110u64 => "
      ICU_NS.nmiclr(),
    ",
  0x5000c120u64 => "
      ICU_NS.nmisr(),
    ",
  0x5000c1a0u64 => "
      ICU_NS.wupen0(),
    ",
  0x5000c1a4u64 => "
      ICU_NS.wupen1(),
    ",
  0x5000c300u64 => "
      ICU_NS.ielsr()[0],
    ",
  0x5000c304u64 => "
      ICU_NS.ielsr()[1],
    ",
  0x5000c308u64 => "
      ICU_NS.ielsr()[2],
    ",
  0x5000c30cu64 => "
      ICU_NS.ielsr()[3],
    ",
  0x5000c310u64 => "
      ICU_NS.ielsr()[4],
    ",
  0x5000c314u64 => "
      ICU_NS.ielsr()[5],
    ",
  0x5000c318u64 => "
      ICU_NS.ielsr()[6],
    ",
  0x5000c31cu64 => "
      ICU_NS.ielsr()[7],
    ",
  0x5000c320u64 => "
      ICU_NS.ielsr()[8],
    ",
  0x5000c324u64 => "
      ICU_NS.ielsr()[9],
    ",
  0x5000c328u64 => "
      ICU_NS.ielsr()[10],
    ",
  0x5000c32cu64 => "
      ICU_NS.ielsr()[11],
    ",
  0x5000c330u64 => "
      ICU_NS.ielsr()[12],
    ",
  0x5000c334u64 => "
      ICU_NS.ielsr()[13],
    ",
  0x5000c338u64 => "
      ICU_NS.ielsr()[14],
    ",
  0x5000c33cu64 => "
      ICU_NS.ielsr()[15],
    ",
  0x5000c340u64 => "
      ICU_NS.ielsr()[16],
    ",
  0x5000c344u64 => "
      ICU_NS.ielsr()[17],
    ",
  0x5000c348u64 => "
      ICU_NS.ielsr()[18],
    ",
  0x5000c34cu64 => "
      ICU_NS.ielsr()[19],
    ",
  0x5000c350u64 => "
      ICU_NS.ielsr()[20],
    ",
  0x5000c354u64 => "
      ICU_NS.ielsr()[21],
    ",
  0x5000c358u64 => "
      ICU_NS.ielsr()[22],
    ",
  0x5000c35cu64 => "
      ICU_NS.ielsr()[23],
    ",
  0x5000c360u64 => "
      ICU_NS.ielsr()[24],
    ",
  0x5000c364u64 => "
      ICU_NS.ielsr()[25],
    ",
  0x5000c368u64 => "
      ICU_NS.ielsr()[26],
    ",
  0x5000c36cu64 => "
      ICU_NS.ielsr()[27],
    ",
  0x5000c370u64 => "
      ICU_NS.ielsr()[28],
    ",
  0x5000c374u64 => "
      ICU_NS.ielsr()[29],
    ",
  0x5000c378u64 => "
      ICU_NS.ielsr()[30],
    ",
  0x5000c37cu64 => "
      ICU_NS.ielsr()[31],
    ",
  0x5000c380u64 => "
      ICU_NS.ielsr()[32],
    ",
  0x5000c384u64 => "
      ICU_NS.ielsr()[33],
    ",
  0x5000c388u64 => "
      ICU_NS.ielsr()[34],
    ",
  0x5000c38cu64 => "
      ICU_NS.ielsr()[35],
    ",
  0x5000c390u64 => "
      ICU_NS.ielsr()[36],
    ",
  0x5000c394u64 => "
      ICU_NS.ielsr()[37],
    ",
  0x5000c398u64 => "
      ICU_NS.ielsr()[38],
    ",
  0x5000c39cu64 => "
      ICU_NS.ielsr()[39],
    ",
  0x5000c3a0u64 => "
      ICU_NS.ielsr()[40],
    ",
  0x5000c3a4u64 => "
      ICU_NS.ielsr()[41],
    ",
  0x5000c3a8u64 => "
      ICU_NS.ielsr()[42],
    ",
  0x5000c3acu64 => "
      ICU_NS.ielsr()[43],
    ",
  0x5000c3b0u64 => "
      ICU_NS.ielsr()[44],
    ",
  0x5000c3b4u64 => "
      ICU_NS.ielsr()[45],
    ",
  0x5000c3b8u64 => "
      ICU_NS.ielsr()[46],
    ",
  0x5000c3bcu64 => "
      ICU_NS.ielsr()[47],
    ",
  0x5000c3c0u64 => "
      ICU_NS.ielsr()[48],
    ",
  0x5000c3c4u64 => "
      ICU_NS.ielsr()[49],
    ",
  0x5000c3c8u64 => "
      ICU_NS.ielsr()[50],
    ",
  0x5000c3ccu64 => "
      ICU_NS.ielsr()[51],
    ",
  0x5000c3d0u64 => "
      ICU_NS.ielsr()[52],
    ",
  0x5000c3d4u64 => "
      ICU_NS.ielsr()[53],
    ",
  0x5000c3d8u64 => "
      ICU_NS.ielsr()[54],
    ",
  0x5000c3dcu64 => "
      ICU_NS.ielsr()[55],
    ",
  0x5000c3e0u64 => "
      ICU_NS.ielsr()[56],
    ",
  0x5000c3e4u64 => "
      ICU_NS.ielsr()[57],
    ",
  0x5000c3e8u64 => "
      ICU_NS.ielsr()[58],
    ",
  0x5000c3ecu64 => "
      ICU_NS.ielsr()[59],
    ",
  0x5000c3f0u64 => "
      ICU_NS.ielsr()[60],
    ",
  0x5000c3f4u64 => "
      ICU_NS.ielsr()[61],
    ",
  0x5000c3f8u64 => "
      ICU_NS.ielsr()[62],
    ",
  0x5000c3fcu64 => "
      ICU_NS.ielsr()[63],
    ",
  0x5000c400u64 => "
      ICU_NS.ielsr()[64],
    ",
  0x5000c404u64 => "
      ICU_NS.ielsr()[65],
    ",
  0x5000c408u64 => "
      ICU_NS.ielsr()[66],
    ",
  0x5000c40cu64 => "
      ICU_NS.ielsr()[67],
    ",
  0x5000c410u64 => "
      ICU_NS.ielsr()[68],
    ",
  0x5000c414u64 => "
      ICU_NS.ielsr()[69],
    ",
  0x5000c418u64 => "
      ICU_NS.ielsr()[70],
    ",
  0x5000c41cu64 => "
      ICU_NS.ielsr()[71],
    ",
  0x5000c420u64 => "
      ICU_NS.ielsr()[72],
    ",
  0x5000c424u64 => "
      ICU_NS.ielsr()[73],
    ",
  0x5000c428u64 => "
      ICU_NS.ielsr()[74],
    ",
  0x5000c42cu64 => "
      ICU_NS.ielsr()[75],
    ",
  0x5000c430u64 => "
      ICU_NS.ielsr()[76],
    ",
  0x5000c434u64 => "
      ICU_NS.ielsr()[77],
    ",
  0x5000c438u64 => "
      ICU_NS.ielsr()[78],
    ",
  0x5000c43cu64 => "
      ICU_NS.ielsr()[79],
    ",
  0x5000c440u64 => "
      ICU_NS.ielsr()[80],
    ",
  0x5000c444u64 => "
      ICU_NS.ielsr()[81],
    ",
  0x5000c448u64 => "
      ICU_NS.ielsr()[82],
    ",
  0x5000c44cu64 => "
      ICU_NS.ielsr()[83],
    ",
  0x5000c450u64 => "
      ICU_NS.ielsr()[84],
    ",
  0x5000c454u64 => "
      ICU_NS.ielsr()[85],
    ",
  0x5000c458u64 => "
      ICU_NS.ielsr()[86],
    ",
  0x5000c45cu64 => "
      ICU_NS.ielsr()[87],
    ",
  0x5000c460u64 => "
      ICU_NS.ielsr()[88],
    ",
  0x5000c464u64 => "
      ICU_NS.ielsr()[89],
    ",
  0x5000c468u64 => "
      ICU_NS.ielsr()[90],
    ",
  0x5000c46cu64 => "
      ICU_NS.ielsr()[91],
    ",
  0x5000c470u64 => "
      ICU_NS.ielsr()[92],
    ",
  0x5000c474u64 => "
      ICU_NS.ielsr()[93],
    ",
  0x5000c478u64 => "
      ICU_NS.ielsr()[94],
    ",
  0x5000c47cu64 => "
      ICU_NS.ielsr()[95],
    ",
  0x5000f030u64 => "
      CPU_CTRL_NS.cpulckupcr(),
    ",
  0x5000f500u64 => "
      CPU_CTRL_NS.cpulockcrns(),
    ",
  0x5000f840u64 => "
      CPU_CTRL_NS.cpucrpt(),
    ",
  0x50011004u64 => "
      OCD_CPU_NS.mcuctrl(),
    ",
  0x50011100u64 => "
      OCD_CPU_NS.jbmdr(),
    ",
  0x50011120u64 => "
      OCD_CPU_NS.jbrdr(),
    ",
  0x50011130u64 => "
      OCD_CPU_NS.jbtdr(),
    ",
  0x50011140u64 => "
      OCD_CPU_NS.jbstr(),
    ",
  0x50011150u64 => "
      OCD_CPU_NS.jbicr(),
    ",
  0x5001b000u64 => "
      CPU_DBG_NS.dbgstr(),
    ",
  0x5001b010u64 => "
      CPU_DBG_NS.dbgstopcr(),
    ",
  0x5001b020u64 => "
      CPU_DBG_NS.dbgauth0(),
    ",
  0x5001b030u64 => "
      CPU_DBG_NS.trportcr(),
    ",
  0x5001b300u64 => "
      CPU_DBG_NS.dbgmocoen(),
    ",
  0x5001b310u64 => "
      CPU_DBG_NS.dbgfclksel(),
    ",
  0x5001c100u64 => "
      FCACHE_NS.fcachee(),
    ",
  0x5001c104u64 => "
      FCACHE_NS.fcacheiv(),
    ",
  0x5001c11cu64 => "
      FCACHE_NS.flwt(),
    ",
  0x5001c140u64 => "
      FCACHE_NS.fsar(),
    ",
  0x5001e020u64 => "
      SYSC_NS.sckdivcr(),
    ",
  0x5001e024u64 => "
      SYSC_NS.sckdivcr2(),
    ",
  0x5001e026u64 => "
      SYSC_NS.sckscr(),
    ",
  0x5001e028u64 => "
      SYSC_NS.pllccr(),
    ",
  0x5001e02au64 => "
      SYSC_NS.pllcr(),
    ",
  0x5001e032u64 => "
      SYSC_NS.mosccr(),
    ",
  0x5001e036u64 => "
      SYSC_NS.hococr(),
    ",
  0x5001e037u64 => "
      SYSC_NS.hococr2(),
    ",
  0x5001e038u64 => "
      SYSC_NS.mococr(),
    ",
  0x5001e039u64 => "
      SYSC_NS.fllcr1(),
    ",
  0x5001e03au64 => "
      SYSC_NS.fllcr2(),
    ",
  0x5001e03cu64 => "
      SYSC_NS.oscsf(),
    ",
  0x5001e03eu64 => "
      SYSC_NS.ckocr(),
    ",
  0x5001e03fu64 => "
      SYSC_NS.trckcr(),
    ",
  0x5001e040u64 => "
      SYSC_NS.ostdcr(),
    ",
  0x5001e041u64 => "
      SYSC_NS.ostdsr(),
    ",
  0x5001e043u64 => "
      SYSC_NS.oscmonr(),
    ",
  0x5001e048u64 => "
      SYSC_NS.pll2ccr(),
    ",
  0x5001e04au64 => "
      SYSC_NS.pll2cr(),
    ",
  0x5001e04cu64 => "
      SYSC_NS.pllccr2(),
    ",
  0x5001e04eu64 => "
      SYSC_NS.pll2ccr2(),
    ",
  0x5001e054u64 => "
      SYSC_NS.scickdivcr(),
    ",
  0x5001e055u64 => "
      SYSC_NS.scickcr(),
    ",
  0x5001e056u64 => "
      SYSC_NS.spickdivcr(),
    ",
  0x5001e057u64 => "
      SYSC_NS.spickcr(),
    ",
  0x5001e061u64 => "
      SYSC_NS.mocoutcr(),
    ",
  0x5001e062u64 => "
      SYSC_NS.hocoutcr(),
    ",
  0x5001e06cu64 => "
      SYSC_NS.usbckdivcr(),
    ",
  0x5001e06du64 => "
      SYSC_NS.octackdivcr(),
    ",
  0x5001e06eu64 => "
      SYSC_NS.canfdckdivcr(),
    ",
  0x5001e074u64 => "
      SYSC_NS.usbckcr(),
    ",
  0x5001e075u64 => "
      SYSC_NS.octackcr(),
    ",
  0x5001e076u64 => "
      SYSC_NS.canfdckcr(),
    ",
  0x5001e07cu64 => "
      SYSC_NS.moscscr(),
    ",
  0x5001e07du64 => "
      SYSC_NS.hocoscr(),
    ",
  0x5001e0a0u64 => "
      SYSC_NS.opccr(),
    ",
  0x5001e0a2u64 => "
      SYSC_NS.moscwtcr(),
    ",
  0x5001e0c0u64 => "
      SYSC_NS.rstsr1(),
    ",
  0x5001e0ccu64 => "
      SYSC_NS.syraccr(),
    ",
  0x5001e0e0u64 => "
      SYSC_NS.pvdcr1()[0],
    ",
  0x5001e0e2u64 => "
      SYSC_NS.pvdcr1()[1],
    ",
  0x5001e0e1u64 => "
      SYSC_NS.pvdsr()[0],
    ",
  0x5001e0e3u64 => "
      SYSC_NS.pvdsr()[1],
    ",
  0x5001e140u64 => "
      SYSC_NS.pdramscr0(),
    ",
  0x5001e142u64 => "
      SYSC_NS.pdramscr1(),
    ",
  0x5001e3b8u64 => "
      SYSC_NS.vbrpabarns(),
    ",
  0x5001e3c0u64 => "
      SYSC_NS.cgfsar(),
    ",
  0x5001e3c4u64 => "
      SYSC_NS.rstsar(),
    ",
  0x5001e3c8u64 => "
      SYSC_NS.lpmsar(),
    ",
  0x5001e3ccu64 => "
      SYSC_NS.pvdsar(),
    ",
  0x5001e3d0u64 => "
      SYSC_NS.bbfsar(),
    ",
  0x5001e3e0u64 => "
      SYSC_NS.dpfsar(),
    ",
  0x5001e3e4u64 => "
      SYSC_NS.rscsar(),
    ",
  0x5001e3feu64 => "
      SYSC_NS.prcr_ns(),
    ",
  0x5001e400u64 => "
      SYSC_NS.lococr(),
    ",
  0x5001e402u64 => "
      SYSC_NS.locoutcr(),
    ",
  0x5001ea00u64 => "
      SYSC_NS.dpsbycr(),
    ",
  0x5001ea04u64 => "
      SYSC_NS.dpswcr(),
    ",
  0x5001ea08u64 => "
      SYSC_NS.dpsier0(),
    ",
  0x5001ea0cu64 => "
      SYSC_NS.dpsier1(),
    ",
  0x5001ea10u64 => "
      SYSC_NS.dpsier2(),
    ",
  0x5001ea14u64 => "
      SYSC_NS.dpsier3(),
    ",
  0x5001ea18u64 => "
      SYSC_NS.dpsifr0(),
    ",
  0x5001ea1cu64 => "
      SYSC_NS.dpsifr1(),
    ",
  0x5001ea20u64 => "
      SYSC_NS.dpsifr2(),
    ",
  0x5001ea24u64 => "
      SYSC_NS.dpsifr3(),
    ",
  0x5001ea28u64 => "
      SYSC_NS.dpsiegr0(),
    ",
  0x5001ea2cu64 => "
      SYSC_NS.dpsiegr1(),
    ",
  0x5001ea30u64 => "
      SYSC_NS.dpsiegr2(),
    ",
  0x5001ea38u64 => "
      SYSC_NS.syocdcr(),
    ",
  0x5001ea40u64 => "
      SYSC_NS.rstsr0(),
    ",
  0x5001ea44u64 => "
      SYSC_NS.rstsr2(),
    ",
  0x5001ea50u64 => "
      SYSC_NS.momcr(),
    ",
  0x5001ea54u64 => "
      SYSC_NS.fwepror(),
    ",
  0x5001ea58u64 => "
      SYSC_NS.pvdcmpcr()[0],
    ",
  0x5001ea5cu64 => "
      SYSC_NS.pvdcmpcr()[1],
    ",
  0x5001ea70u64 => "
      SYSC_NS.pvdcr0()[0],
    ",
  0x5001ea74u64 => "
      SYSC_NS.pvdcr0()[1],
    ",
  0x5001ea84u64 => "
      SYSC_NS.vbattmnselr(),
    ",
  0x5001ea88u64 => "
      SYSC_NS.vbtbpcr1(),
    ",
  0x5001ea90u64 => "
      SYSC_NS.lpscr(),
    ",
  0x5001ea98u64 => "
      SYSC_NS.sscr1(),
    ",
  0x5001eab0u64 => "
      SYSC_NS.lvocr(),
    ",
  0x5001eb04u64 => "
      SYSC_NS.pll1ldocr(),
    ",
  0x5001eb08u64 => "
      SYSC_NS.pll2ldocr(),
    ",
  0x5001eb0cu64 => "
      SYSC_NS.hocoldocr(),
    ",
  0x5001eb20u64 => "
      SYSC_NS.pvdfcr()[0],
    ",
  0x5001eb24u64 => "
      SYSC_NS.pvdfcr()[1],
    ",
  0x5001ec00u64 => "
      SYSC_NS.sosccr(),
    ",
  0x5001ec01u64 => "
      SYSC_NS.somcr(),
    ",
  0x5001ec40u64 => "
      SYSC_NS.vbtber(),
    ",
  0x5001ec45u64 => "
      SYSC_NS.vbtbpcr2(),
    ",
  0x5001ec46u64 => "
      SYSC_NS.vbtbpsr(),
    ",
  0x5001ec48u64 => "
      SYSC_NS.vbtadsr(),
    ",
  0x5001ec49u64 => "
      SYSC_NS.vbtadcr1(),
    ",
  0x5001ec4au64 => "
      SYSC_NS.vbtadcr2(),
    ",
  0x5001ec4cu64 => "
      SYSC_NS.vbtictlr(),
    ",
  0x5001ec4du64 => "
      SYSC_NS.vbtictlr2(),
    ",
  0x5001ec4eu64 => "
      SYSC_NS.vbtimonr(),
    ",
  0x5001ed00u64 => "
      SYSC_NS.vbtbkr()[0],
    ",
  0x5001ed01u64 => "
      SYSC_NS.vbtbkr()[1],
    ",
  0x5001ed02u64 => "
      SYSC_NS.vbtbkr()[2],
    ",
  0x5001ed03u64 => "
      SYSC_NS.vbtbkr()[3],
    ",
  0x5001ed04u64 => "
      SYSC_NS.vbtbkr()[4],
    ",
  0x5001ed05u64 => "
      SYSC_NS.vbtbkr()[5],
    ",
  0x5001ed06u64 => "
      SYSC_NS.vbtbkr()[6],
    ",
  0x5001ed07u64 => "
      SYSC_NS.vbtbkr()[7],
    ",
  0x5001ed08u64 => "
      SYSC_NS.vbtbkr()[8],
    ",
  0x5001ed09u64 => "
      SYSC_NS.vbtbkr()[9],
    ",
  0x5001ed0au64 => "
      SYSC_NS.vbtbkr()[10],
    ",
  0x5001ed0bu64 => "
      SYSC_NS.vbtbkr()[11],
    ",
  0x5001ed0cu64 => "
      SYSC_NS.vbtbkr()[12],
    ",
  0x5001ed0du64 => "
      SYSC_NS.vbtbkr()[13],
    ",
  0x5001ed0eu64 => "
      SYSC_NS.vbtbkr()[14],
    ",
  0x5001ed0fu64 => "
      SYSC_NS.vbtbkr()[15],
    ",
  0x5001ed10u64 => "
      SYSC_NS.vbtbkr()[16],
    ",
  0x5001ed11u64 => "
      SYSC_NS.vbtbkr()[17],
    ",
  0x5001ed12u64 => "
      SYSC_NS.vbtbkr()[18],
    ",
  0x5001ed13u64 => "
      SYSC_NS.vbtbkr()[19],
    ",
  0x5001ed14u64 => "
      SYSC_NS.vbtbkr()[20],
    ",
  0x5001ed15u64 => "
      SYSC_NS.vbtbkr()[21],
    ",
  0x5001ed16u64 => "
      SYSC_NS.vbtbkr()[22],
    ",
  0x5001ed17u64 => "
      SYSC_NS.vbtbkr()[23],
    ",
  0x5001ed18u64 => "
      SYSC_NS.vbtbkr()[24],
    ",
  0x5001ed19u64 => "
      SYSC_NS.vbtbkr()[25],
    ",
  0x5001ed1au64 => "
      SYSC_NS.vbtbkr()[26],
    ",
  0x5001ed1bu64 => "
      SYSC_NS.vbtbkr()[27],
    ",
  0x5001ed1cu64 => "
      SYSC_NS.vbtbkr()[28],
    ",
  0x5001ed1du64 => "
      SYSC_NS.vbtbkr()[29],
    ",
  0x5001ed1eu64 => "
      SYSC_NS.vbtbkr()[30],
    ",
  0x5001ed1fu64 => "
      SYSC_NS.vbtbkr()[31],
    ",
  0x5001ed20u64 => "
      SYSC_NS.vbtbkr()[32],
    ",
  0x5001ed21u64 => "
      SYSC_NS.vbtbkr()[33],
    ",
  0x5001ed22u64 => "
      SYSC_NS.vbtbkr()[34],
    ",
  0x5001ed23u64 => "
      SYSC_NS.vbtbkr()[35],
    ",
  0x5001ed24u64 => "
      SYSC_NS.vbtbkr()[36],
    ",
  0x5001ed25u64 => "
      SYSC_NS.vbtbkr()[37],
    ",
  0x5001ed26u64 => "
      SYSC_NS.vbtbkr()[38],
    ",
  0x5001ed27u64 => "
      SYSC_NS.vbtbkr()[39],
    ",
  0x5001ed28u64 => "
      SYSC_NS.vbtbkr()[40],
    ",
  0x5001ed29u64 => "
      SYSC_NS.vbtbkr()[41],
    ",
  0x5001ed2au64 => "
      SYSC_NS.vbtbkr()[42],
    ",
  0x5001ed2bu64 => "
      SYSC_NS.vbtbkr()[43],
    ",
  0x5001ed2cu64 => "
      SYSC_NS.vbtbkr()[44],
    ",
  0x5001ed2du64 => "
      SYSC_NS.vbtbkr()[45],
    ",
  0x5001ed2eu64 => "
      SYSC_NS.vbtbkr()[46],
    ",
  0x5001ed2fu64 => "
      SYSC_NS.vbtbkr()[47],
    ",
  0x5001ed30u64 => "
      SYSC_NS.vbtbkr()[48],
    ",
  0x5001ed31u64 => "
      SYSC_NS.vbtbkr()[49],
    ",
  0x5001ed32u64 => "
      SYSC_NS.vbtbkr()[50],
    ",
  0x5001ed33u64 => "
      SYSC_NS.vbtbkr()[51],
    ",
  0x5001ed34u64 => "
      SYSC_NS.vbtbkr()[52],
    ",
  0x5001ed35u64 => "
      SYSC_NS.vbtbkr()[53],
    ",
  0x5001ed36u64 => "
      SYSC_NS.vbtbkr()[54],
    ",
  0x5001ed37u64 => "
      SYSC_NS.vbtbkr()[55],
    ",
  0x5001ed38u64 => "
      SYSC_NS.vbtbkr()[56],
    ",
  0x5001ed39u64 => "
      SYSC_NS.vbtbkr()[57],
    ",
  0x5001ed3au64 => "
      SYSC_NS.vbtbkr()[58],
    ",
  0x5001ed3bu64 => "
      SYSC_NS.vbtbkr()[59],
    ",
  0x5001ed3cu64 => "
      SYSC_NS.vbtbkr()[60],
    ",
  0x5001ed3du64 => "
      SYSC_NS.vbtbkr()[61],
    ",
  0x5001ed3eu64 => "
      SYSC_NS.vbtbkr()[62],
    ",
  0x5001ed3fu64 => "
      SYSC_NS.vbtbkr()[63],
    ",
  0x5001ed40u64 => "
      SYSC_NS.vbtbkr()[64],
    ",
  0x5001ed41u64 => "
      SYSC_NS.vbtbkr()[65],
    ",
  0x5001ed42u64 => "
      SYSC_NS.vbtbkr()[66],
    ",
  0x5001ed43u64 => "
      SYSC_NS.vbtbkr()[67],
    ",
  0x5001ed44u64 => "
      SYSC_NS.vbtbkr()[68],
    ",
  0x5001ed45u64 => "
      SYSC_NS.vbtbkr()[69],
    ",
  0x5001ed46u64 => "
      SYSC_NS.vbtbkr()[70],
    ",
  0x5001ed47u64 => "
      SYSC_NS.vbtbkr()[71],
    ",
  0x5001ed48u64 => "
      SYSC_NS.vbtbkr()[72],
    ",
  0x5001ed49u64 => "
      SYSC_NS.vbtbkr()[73],
    ",
  0x5001ed4au64 => "
      SYSC_NS.vbtbkr()[74],
    ",
  0x5001ed4bu64 => "
      SYSC_NS.vbtbkr()[75],
    ",
  0x5001ed4cu64 => "
      SYSC_NS.vbtbkr()[76],
    ",
  0x5001ed4du64 => "
      SYSC_NS.vbtbkr()[77],
    ",
  0x5001ed4eu64 => "
      SYSC_NS.vbtbkr()[78],
    ",
  0x5001ed4fu64 => "
      SYSC_NS.vbtbkr()[79],
    ",
  0x5001ed50u64 => "
      SYSC_NS.vbtbkr()[80],
    ",
  0x5001ed51u64 => "
      SYSC_NS.vbtbkr()[81],
    ",
  0x5001ed52u64 => "
      SYSC_NS.vbtbkr()[82],
    ",
  0x5001ed53u64 => "
      SYSC_NS.vbtbkr()[83],
    ",
  0x5001ed54u64 => "
      SYSC_NS.vbtbkr()[84],
    ",
  0x5001ed55u64 => "
      SYSC_NS.vbtbkr()[85],
    ",
  0x5001ed56u64 => "
      SYSC_NS.vbtbkr()[86],
    ",
  0x5001ed57u64 => "
      SYSC_NS.vbtbkr()[87],
    ",
  0x5001ed58u64 => "
      SYSC_NS.vbtbkr()[88],
    ",
  0x5001ed59u64 => "
      SYSC_NS.vbtbkr()[89],
    ",
  0x5001ed5au64 => "
      SYSC_NS.vbtbkr()[90],
    ",
  0x5001ed5bu64 => "
      SYSC_NS.vbtbkr()[91],
    ",
  0x5001ed5cu64 => "
      SYSC_NS.vbtbkr()[92],
    ",
  0x5001ed5du64 => "
      SYSC_NS.vbtbkr()[93],
    ",
  0x5001ed5eu64 => "
      SYSC_NS.vbtbkr()[94],
    ",
  0x5001ed5fu64 => "
      SYSC_NS.vbtbkr()[95],
    ",
  0x5001ed60u64 => "
      SYSC_NS.vbtbkr()[96],
    ",
  0x5001ed61u64 => "
      SYSC_NS.vbtbkr()[97],
    ",
  0x5001ed62u64 => "
      SYSC_NS.vbtbkr()[98],
    ",
  0x5001ed63u64 => "
      SYSC_NS.vbtbkr()[99],
    ",
  0x5001ed64u64 => "
      SYSC_NS.vbtbkr()[100],
    ",
  0x5001ed65u64 => "
      SYSC_NS.vbtbkr()[101],
    ",
  0x5001ed66u64 => "
      SYSC_NS.vbtbkr()[102],
    ",
  0x5001ed67u64 => "
      SYSC_NS.vbtbkr()[103],
    ",
  0x5001ed68u64 => "
      SYSC_NS.vbtbkr()[104],
    ",
  0x5001ed69u64 => "
      SYSC_NS.vbtbkr()[105],
    ",
  0x5001ed6au64 => "
      SYSC_NS.vbtbkr()[106],
    ",
  0x5001ed6bu64 => "
      SYSC_NS.vbtbkr()[107],
    ",
  0x5001ed6cu64 => "
      SYSC_NS.vbtbkr()[108],
    ",
  0x5001ed6du64 => "
      SYSC_NS.vbtbkr()[109],
    ",
  0x5001ed6eu64 => "
      SYSC_NS.vbtbkr()[110],
    ",
  0x5001ed6fu64 => "
      SYSC_NS.vbtbkr()[111],
    ",
  0x5001ed70u64 => "
      SYSC_NS.vbtbkr()[112],
    ",
  0x5001ed71u64 => "
      SYSC_NS.vbtbkr()[113],
    ",
  0x5001ed72u64 => "
      SYSC_NS.vbtbkr()[114],
    ",
  0x5001ed73u64 => "
      SYSC_NS.vbtbkr()[115],
    ",
  0x5001ed74u64 => "
      SYSC_NS.vbtbkr()[116],
    ",
  0x5001ed75u64 => "
      SYSC_NS.vbtbkr()[117],
    ",
  0x5001ed76u64 => "
      SYSC_NS.vbtbkr()[118],
    ",
  0x5001ed77u64 => "
      SYSC_NS.vbtbkr()[119],
    ",
  0x5001ed78u64 => "
      SYSC_NS.vbtbkr()[120],
    ",
  0x5001ed79u64 => "
      SYSC_NS.vbtbkr()[121],
    ",
  0x5001ed7au64 => "
      SYSC_NS.vbtbkr()[122],
    ",
  0x5001ed7bu64 => "
      SYSC_NS.vbtbkr()[123],
    ",
  0x5001ed7cu64 => "
      SYSC_NS.vbtbkr()[124],
    ",
  0x5001ed7du64 => "
      SYSC_NS.vbtbkr()[125],
    ",
  0x5001ed7eu64 => "
      SYSC_NS.vbtbkr()[126],
    ",
  0x5001ed7fu64 => "
      SYSC_NS.vbtbkr()[127],
    ",
  0x5011b17cu64 => "
      TSD_NS.tscdr(),
    ",
  0x5011c040u64 => "
      FLAD_NS.fckmhz(),
    ",
  0x5011e010u64 => "
      FACI_NS.fastat(),
    ",
  0x5011e014u64 => "
      FACI_NS.faeint(),
    ",
  0x5011e018u64 => "
      FACI_NS.frdyie(),
    ",
  0x5011e030u64 => "
      FACI_NS.fsaddr(),
    ",
  0x5011e034u64 => "
      FACI_NS.feaddr(),
    ",
  0x5011e078u64 => "
      FACI_NS.fbprot0(),
    ",
  0x5011e080u64 => "
      FACI_NS.fstatr(),
    ",
  0x5011e084u64 => "
      FACI_NS.fentryr(),
    ",
  0x5011e08cu64 => "
      FACI_NS.fsuinitr(),
    ",
  0x5011e0a0u64 => "
      FACI_NS.fcmdr(),
    ",
  0x5011e0d0u64 => "
      FACI_NS.fbccnt(),
    ",
  0x5011e0d4u64 => "
      FACI_NS.fbcstat(),
    ",
  0x5011e0d8u64 => "
      FACI_NS.fpsaddr(),
    ",
  0x5011e0dcu64 => "
      FACI_NS.fsuasmon(),
    ",
  0x5011e0e0u64 => "
      FACI_NS.fcpsr(),
    ",
  0x5011e0e4u64 => "
      FACI_NS.fpckar(),
    ",
  0x50201000u64 => "
      ELC_NS.elcr(),
    ",
  0x50201004u64 => "
      ELC_NS.elsegr()[0],
    ",
  0x50201008u64 => "
      ELC_NS.elsegr()[1],
    ",
  0x50201020u64 => "
      ELC_NS.elsr()[0],
    ",
  0x50201024u64 => "
      ELC_NS.elsr()[1],
    ",
  0x50201028u64 => "
      ELC_NS.elsr()[2],
    ",
  0x5020102cu64 => "
      ELC_NS.elsr()[3],
    ",
  0x50201030u64 => "
      ELC_NS.elsr()[4],
    ",
  0x50201034u64 => "
      ELC_NS.elsr()[5],
    ",
  0x50201038u64 => "
      ELC_NS.elsr()[6],
    ",
  0x5020103cu64 => "
      ELC_NS.elsr()[7],
    ",
  0x50201040u64 => "
      ELC_NS.elsr()[8],
    ",
  0x50201044u64 => "
      ELC_NS.elsr()[9],
    ",
  0x50201048u64 => "
      ELC_NS.elsr()[10],
    ",
  0x5020104cu64 => "
      ELC_NS.elsr()[11],
    ",
  0x50201050u64 => "
      ELC_NS.elsr()[12],
    ",
  0x50201054u64 => "
      ELC_NS.elsr()[13],
    ",
  0x50201058u64 => "
      ELC_NS.elsr()[14],
    ",
  0x5020105cu64 => "
      ELC_NS.elsr()[15],
    ",
  0x50201060u64 => "
      ELC_NS.elsr()[16],
    ",
  0x50201064u64 => "
      ELC_NS.elsr()[17],
    ",
  0x50201098u64 => "
      ELC_NS.elsr30(),
    ",
  0x502010e0u64 => "
      ELC_NS.elcsara(),
    ",
  0x502010e4u64 => "
      ELC_NS.elcsarb(),
    ",
  0x502010f0u64 => "
      ELC_NS.elcpara(),
    ",
  0x502010f4u64 => "
      ELC_NS.elcparb(),
    ",
  0x50202000u64 => "
      RTC_NS.r64cnt(),
    ",
  0x50202002u64 => "
      RTC_NS.bcnt()[0],
      RTC_NS.rseccnt(),
    ",
  0x50202004u64 => "
      RTC_NS.bcnt()[1],
      RTC_NS.rmincnt(),
    ",
  0x50202006u64 => "
      RTC_NS.bcnt()[2],
      RTC_NS.rhrcnt(),
    ",
  0x50202008u64 => "
      RTC_NS.bcnt()[3],
      RTC_NS.rwkcnt(),
    ",
  0x5020200au64 => "
      RTC_NS.rdaycnt(),
    ",
  0x5020200cu64 => "
      RTC_NS.rmoncnt(),
    ",
  0x5020200eu64 => "
      RTC_NS.ryrcnt(),
    ",
  0x50202010u64 => "
      RTC_NS.bcntar()[0],
      RTC_NS.rsecar(),
    ",
  0x50202012u64 => "
      RTC_NS.bcntar()[1],
      RTC_NS.rminar(),
    ",
  0x50202014u64 => "
      RTC_NS.bcntar()[2],
      RTC_NS.rhrar(),
    ",
  0x50202016u64 => "
      RTC_NS.bcntar()[3],
      RTC_NS.rwkar(),
    ",
  0x50202018u64 => "
      RTC_NS.bcntaer()[0],
      RTC_NS.rdayar(),
    ",
  0x5020201au64 => "
      RTC_NS.bcntaer()[1],
      RTC_NS.rmonar(),
    ",
  0x5020201cu64 => "
      RTC_NS.bcntaer()[2],
      RTC_NS.ryrar(),
    ",
  0x5020201eu64 => "
      RTC_NS.bcntaer()[3],
      RTC_NS.ryraren(),
    ",
  0x50202022u64 => "
      RTC_NS.rcr1(),
    ",
  0x50202024u64 => "
      RTC_NS.rcr2(),
      RTC_NS.rcr2_bcnt(),
    ",
  0x50202028u64 => "
      RTC_NS.rcr4(),
    ",
  0x5020202au64 => "
      RTC_NS.rfrh(),
    ",
  0x5020202cu64 => "
      RTC_NS.rfrl(),
    ",
  0x5020202eu64 => "
      RTC_NS.radj(),
    ",
  0x50202040u64 => "
      RTC_NS.rtccr()[0],
    ",
  0x50202042u64 => "
      RTC_NS.rtccr()[1],
    ",
  0x50202044u64 => "
      RTC_NS.rtccr()[2],
    ",
  0x50202052u64 => "
      RTC_NS.bcnt0cp()[0],
      RTC_NS.rseccp()[0],
    ",
  0x50202062u64 => "
      RTC_NS.bcnt0cp()[1],
      RTC_NS.rseccp()[1],
    ",
  0x50202072u64 => "
      RTC_NS.bcnt0cp()[2],
      RTC_NS.rseccp()[2],
    ",
  0x50202054u64 => "
      RTC_NS.bcnt1cp()[0],
      RTC_NS.rmincp()[0],
    ",
  0x50202064u64 => "
      RTC_NS.bcnt1cp()[1],
      RTC_NS.rmincp()[1],
    ",
  0x50202074u64 => "
      RTC_NS.bcnt1cp()[2],
      RTC_NS.rmincp()[2],
    ",
  0x50202056u64 => "
      RTC_NS.bcnt2cp()[0],
      RTC_NS.rhrcp()[0],
    ",
  0x50202066u64 => "
      RTC_NS.bcnt2cp()[1],
      RTC_NS.rhrcp()[1],
    ",
  0x50202076u64 => "
      RTC_NS.bcnt2cp()[2],
      RTC_NS.rhrcp()[2],
    ",
  0x5020205au64 => "
      RTC_NS.bcnt3cp()[0],
      RTC_NS.rdaycp()[0],
    ",
  0x5020206au64 => "
      RTC_NS.bcnt3cp()[1],
      RTC_NS.rdaycp()[1],
    ",
  0x5020207au64 => "
      RTC_NS.bcnt3cp()[2],
      RTC_NS.rdaycp()[2],
    ",
  0x5020205cu64 => "
      RTC_NS.rmoncp()[0],
    ",
  0x5020206cu64 => "
      RTC_NS.rmoncp()[1],
    ",
  0x5020207cu64 => "
      RTC_NS.rmoncp()[2],
    ",
  0x50202200u64 => "
      IWDT_NS.iwdtrr(),
    ",
  0x50202202u64 => "
      IWDT_NS.iwdtcr(),
    ",
  0x50202204u64 => "
      IWDT_NS.iwdtsr(),
    ",
  0x50202206u64 => "
      IWDT_NS.iwdtrcr(),
    ",
  0x50202208u64 => "
      IWDT_NS.iwdtcstpr(),
    ",
  0x50202400u64 => "
      CAC_NS.cacr0(),
    ",
  0x50202401u64 => "
      CAC_NS.cacr1(),
    ",
  0x50202402u64 => "
      CAC_NS.cacr2(),
    ",
  0x50202403u64 => "
      CAC_NS.caicr(),
    ",
  0x50202404u64 => "
      CAC_NS.castr(),
    ",
  0x50202406u64 => "
      CAC_NS.caulvr(),
    ",
  0x50202408u64 => "
      CAC_NS.callvr(),
    ",
  0x5020240au64 => "
      CAC_NS.cacntbr(),
    ",
  0x50202600u64 => "
      WDT_0_NS.wdtrr(),
    ",
  0x50202602u64 => "
      WDT_0_NS.wdtcr(),
    ",
  0x50202604u64 => "
      WDT_0_NS.wdtsr(),
    ",
  0x50202606u64 => "
      WDT_0_NS.wdtrcr(),
    ",
  0x50202608u64 => "
      WDT_0_NS.wdtcstpr(),
    ",
  0x50203000u64 => "
      MSTP_NS.mstpcra(),
    ",
  0x50203004u64 => "
      MSTP_NS.mstpcrb(),
    ",
  0x50203008u64 => "
      MSTP_NS.mstpcrc(),
    ",
  0x5020300cu64 => "
      MSTP_NS.mstpcrd(),
    ",
  0x50203010u64 => "
      MSTP_NS.mstpcre(),
    ",
  0x50204004u64 => "
      PSCU_NS.psarb(),
    ",
  0x50204008u64 => "
      PSCU_NS.psarc(),
    ",
  0x5020400cu64 => "
      PSCU_NS.psard(),
    ",
  0x50204010u64 => "
      PSCU_NS.psare(),
    ",
  0x50204014u64 => "
      PSCU_NS.mssar(),
    ",
  0x5020401cu64 => "
      PSCU_NS.pparb(),
    ",
  0x50204020u64 => "
      PSCU_NS.pparc(),
    ",
  0x50204024u64 => "
      PSCU_NS.ppard(),
    ",
  0x50204028u64 => "
      PSCU_NS.ppare(),
    ",
  0x5020402cu64 => "
      PSCU_NS.mspar(),
    ",
  0x50204030u64 => "
      PSCU_NS.cfsamona(),
    ",
  0x50204034u64 => "
      PSCU_NS.dfsamon(),
    ",
  0x50204038u64 => "
      PSCU_NS.dlmmon(),
    ",
  0x50212000u64 => "
      POEG_NS.poegga(),
    ",
  0x50212100u64 => "
      POEG_NS.poeggb(),
    ",
  0x50212200u64 => "
      POEG_NS.poeggc(),
    ",
  0x50212300u64 => "
      POEG_NS.poeggd(),
    ",
  0x50220000u64 => "
      ULPT_0_NS.ulptcnt(),
    ",
  0x50220004u64 => "
      ULPT_0_NS.ulptcma(),
    ",
  0x50220008u64 => "
      ULPT_0_NS.ulptcmb(),
    ",
  0x5022000cu64 => "
      ULPT_0_NS.ulptcr(),
    ",
  0x5022000du64 => "
      ULPT_0_NS.ulptmr1(),
    ",
  0x5022000eu64 => "
      ULPT_0_NS.ulptmr2(),
    ",
  0x5022000fu64 => "
      ULPT_0_NS.ulptmr3(),
    ",
  0x50220010u64 => "
      ULPT_0_NS.ulptioc(),
    ",
  0x50220011u64 => "
      ULPT_0_NS.ulptisr(),
    ",
  0x50220012u64 => "
      ULPT_0_NS.ulptcmsr(),
    ",
  0x50221000u64 => "
      AGT_0_NS.agt(),
    ",
  0x50221002u64 => "
      AGT_0_NS.agtcma(),
    ",
  0x50221004u64 => "
      AGT_0_NS.agtcmb(),
    ",
  0x50221008u64 => "
      AGT_0_NS.agtcr(),
    ",
  0x50221009u64 => "
      AGT_0_NS.agtmr1(),
    ",
  0x5022100au64 => "
      AGT_0_NS.agtmr2(),
    ",
  0x5022100cu64 => "
      AGT_0_NS.agtioc(),
    ",
  0x5022100du64 => "
      AGT_0_NS.agtisr(),
    ",
  0x5022100eu64 => "
      AGT_0_NS.agtcmsr(),
    ",
  0x5022100fu64 => "
      AGT_0_NS.agtiosel(),
    ",
  0x50235000u64 => "
      TSN_NS.tscr(),
    ",
  0x50236000u64 => "
      ACMPHS_0_NS.cmpctl(),
    ",
  0x50236004u64 => "
      ACMPHS_0_NS.cmpsel0(),
    ",
  0x50236008u64 => "
      ACMPHS_0_NS.cmpsel1(),
    ",
  0x5023600cu64 => "
      ACMPHS_0_NS.cmpmon(),
    ",
  0x50236010u64 => "
      ACMPHS_0_NS.cpioc(),
    ",
  0x50236040u64 => "
      ACMPHS_0_NS.cpintctl(),
    ",
  0x50236044u64 => "
      ACMPHS_0_NS.cpmskctl(),
    ",
  0x50250000u64 => "
      USBFS_NS.syscfg(),
    ",
  0x50250004u64 => "
      USBFS_NS.syssts0(),
    ",
  0x50250008u64 => "
      USBFS_NS.dvstctr0(),
    ",
  0x50250014u64 => "
      USBFS_NS.cfifo(),
      USBFS_NS.cfifol(),
    ",
  0x50250018u64 => "
      USBFS_NS.dfifo()[0],
      USBFS_NS.dfifol()[0],
    ",
  0x5025001cu64 => "
      USBFS_NS.dfifo()[1],
      USBFS_NS.dfifol()[1],
    ",
  0x50250020u64 => "
      USBFS_NS.cfifosel(),
    ",
  0x50250022u64 => "
      USBFS_NS.cfifoctr(),
    ",
  0x50250028u64 => "
      USBFS_NS.dfifosel()[0],
    ",
  0x5025002cu64 => "
      USBFS_NS.dfifosel()[1],
    ",
  0x5025002au64 => "
      USBFS_NS.dfifoctr()[0],
    ",
  0x5025002eu64 => "
      USBFS_NS.dfifoctr()[1],
    ",
  0x50250030u64 => "
      USBFS_NS.intenb0(),
    ",
  0x50250032u64 => "
      USBFS_NS.intenb1(),
    ",
  0x50250036u64 => "
      USBFS_NS.brdyenb(),
    ",
  0x50250038u64 => "
      USBFS_NS.nrdyenb(),
    ",
  0x5025003au64 => "
      USBFS_NS.bempenb(),
    ",
  0x5025003cu64 => "
      USBFS_NS.sofcfg(),
    ",
  0x50250040u64 => "
      USBFS_NS.intsts0(),
    ",
  0x50250042u64 => "
      USBFS_NS.intsts1(),
    ",
  0x50250046u64 => "
      USBFS_NS.brdysts(),
    ",
  0x50250048u64 => "
      USBFS_NS.nrdysts(),
    ",
  0x5025004au64 => "
      USBFS_NS.bempsts(),
    ",
  0x5025004cu64 => "
      USBFS_NS.frmnum(),
    ",
  0x5025004eu64 => "
      USBFS_NS.dvchgr(),
    ",
  0x50250050u64 => "
      USBFS_NS.usbaddr(),
    ",
  0x50250054u64 => "
      USBFS_NS.usbreq(),
    ",
  0x50250056u64 => "
      USBFS_NS.usbval(),
    ",
  0x50250058u64 => "
      USBFS_NS.usbindx(),
    ",
  0x5025005au64 => "
      USBFS_NS.usbleng(),
    ",
  0x5025005cu64 => "
      USBFS_NS.dcpcfg(),
    ",
  0x5025005eu64 => "
      USBFS_NS.dcpmaxp(),
    ",
  0x50250060u64 => "
      USBFS_NS.dcpctr(),
    ",
  0x50250064u64 => "
      USBFS_NS.pipesel(),
    ",
  0x50250068u64 => "
      USBFS_NS.pipecfg(),
    ",
  0x5025006cu64 => "
      USBFS_NS.pipemaxp(),
    ",
  0x5025006eu64 => "
      USBFS_NS.pipeperi(),
    ",
  0x50250078u64 => "
      USBFS_NS.pipectr()[4],
    ",
  0x5025007au64 => "
      USBFS_NS.pipectr()[0],
    ",
  0x5025007cu64 => "
      USBFS_NS.pipectr()[1],
    ",
  0x5025007eu64 => "
      USBFS_NS.pipectr()[2],
    ",
  0x50250080u64 => "
      USBFS_NS.pipectr()[3],
    ",
  0x50250090u64 => "
      USBFS_NS.pipetre()[0],
    ",
  0x50250094u64 => "
      USBFS_NS.pipetre()[1],
    ",
  0x50250098u64 => "
      USBFS_NS.pipetre()[2],
    ",
  0x5025009cu64 => "
      USBFS_NS.pipetre()[3],
    ",
  0x502500a0u64 => "
      USBFS_NS.pipetre()[4],
    ",
  0x50250092u64 => "
      USBFS_NS.pipetrn()[0],
    ",
  0x50250096u64 => "
      USBFS_NS.pipetrn()[1],
    ",
  0x5025009au64 => "
      USBFS_NS.pipetrn()[2],
    ",
  0x5025009eu64 => "
      USBFS_NS.pipetrn()[3],
    ",
  0x502500a2u64 => "
      USBFS_NS.pipetrn()[4],
    ",
  0x502500d0u64 => "
      USBFS_NS.devadd()[0],
    ",
  0x502500d2u64 => "
      USBFS_NS.devadd()[1],
    ",
  0x502500d4u64 => "
      USBFS_NS.devadd()[2],
    ",
  0x502500d6u64 => "
      USBFS_NS.devadd()[3],
    ",
  0x502500d8u64 => "
      USBFS_NS.devadd()[4],
    ",
  0x502500dau64 => "
      USBFS_NS.devadd()[5],
    ",
  0x50250400u64 => "
      USBFS_NS.dpusr0r(),
    ",
  0x50250404u64 => "
      USBFS_NS.dpusr1r(),
    ",
  0x5025d000u64 => "
      SSIE_0_NS.ssicr(),
    ",
  0x5025d004u64 => "
      SSIE_0_NS.ssisr(),
    ",
  0x5025d010u64 => "
      SSIE_0_NS.ssifcr(),
    ",
  0x5025d014u64 => "
      SSIE_0_NS.ssifsr(),
    ",
  0x5025d018u64 => "
      SSIE_0_NS.ssiftdr(),
    ",
  0x5025d01cu64 => "
      SSIE_0_NS.ssifrdr(),
    ",
  0x5025d020u64 => "
      SSIE_0_NS.ssiofr(),
    ",
  0x5025d024u64 => "
      SSIE_0_NS.ssiscr(),
    ",
  0x5025e000u64 => "
      IIC_0_NS.iccr1(),
    ",
  0x5025e001u64 => "
      IIC_0_NS.iccr2(),
    ",
  0x5025e002u64 => "
      IIC_0_NS.icmr1(),
    ",
  0x5025e003u64 => "
      IIC_0_NS.icmr2(),
    ",
  0x5025e004u64 => "
      IIC_0_NS.icmr3(),
    ",
  0x5025e005u64 => "
      IIC_0_NS.icfer(),
    ",
  0x5025e006u64 => "
      IIC_0_NS.icser(),
    ",
  0x5025e007u64 => "
      IIC_0_NS.icier(),
    ",
  0x5025e008u64 => "
      IIC_0_NS.icsr1(),
    ",
  0x5025e009u64 => "
      IIC_0_NS.icsr2(),
    ",
  0x5025e00au64 => "
      IIC_0_NS.sarl()[0],
    ",
  0x5025e00cu64 => "
      IIC_0_NS.sarl()[1],
    ",
  0x5025e00eu64 => "
      IIC_0_NS.sarl()[2],
    ",
  0x5025e00bu64 => "
      IIC_0_NS.saru()[0],
    ",
  0x5025e00du64 => "
      IIC_0_NS.saru()[1],
    ",
  0x5025e00fu64 => "
      IIC_0_NS.saru()[2],
    ",
  0x5025e010u64 => "
      IIC_0_NS.icbrl(),
    ",
  0x5025e011u64 => "
      IIC_0_NS.icbrh(),
    ",
  0x5025e012u64 => "
      IIC_0_NS.icdrt(),
    ",
  0x5025e013u64 => "
      IIC_0_NS.icdrr(),
    ",
  0x5025e016u64 => "
      IIC_0_WU_NS.icwur(),
    ",
  0x5025e017u64 => "
      IIC_0_WU_NS.icwur2(),
    ",
  0x50268000u64 => "
      OSPI_0_B_NS.wrapcfg(),
    ",
  0x50268004u64 => "
      OSPI_0_B_NS.comcfg(),
    ",
  0x50268008u64 => "
      OSPI_0_B_NS.bmcfgch()[0],
    ",
  0x5026800cu64 => "
      OSPI_0_B_NS.bmcfgch()[1],
    ",
  0x50268010u64 => "
      OSPI_0_B_NS.cmcfg0cs()[0],
    ",
  0x50268020u64 => "
      OSPI_0_B_NS.cmcfg0cs()[1],
    ",
  0x50268014u64 => "
      OSPI_0_B_NS.cmcfg1cs()[0],
    ",
  0x50268024u64 => "
      OSPI_0_B_NS.cmcfg1cs()[1],
    ",
  0x50268018u64 => "
      OSPI_0_B_NS.cmcfg2cs()[0],
    ",
  0x50268028u64 => "
      OSPI_0_B_NS.cmcfg2cs()[1],
    ",
  0x50268050u64 => "
      OSPI_0_B_NS.liocfgcs()[0],
    ",
  0x50268054u64 => "
      OSPI_0_B_NS.liocfgcs()[1],
    ",
  0x50268060u64 => "
      OSPI_0_B_NS.bmctl0(),
    ",
  0x50268064u64 => "
      OSPI_0_B_NS.bmctl1(),
    ",
  0x50268068u64 => "
      OSPI_0_B_NS.cmctlch()[0],
    ",
  0x5026806cu64 => "
      OSPI_0_B_NS.cmctlch()[1],
    ",
  0x50268070u64 => "
      OSPI_0_B_NS.cdctl0(),
    ",
  0x50268074u64 => "
      OSPI_0_B_NS.cdctl1(),
    ",
  0x50268078u64 => "
      OSPI_0_B_NS.cdctl2(),
    ",
  0x50268080u64 => "
      OSPI_0_B_NS.cdtbuf()[0],
    ",
  0x50268090u64 => "
      OSPI_0_B_NS.cdtbuf()[1],
    ",
  0x502680a0u64 => "
      OSPI_0_B_NS.cdtbuf()[2],
    ",
  0x502680b0u64 => "
      OSPI_0_B_NS.cdtbuf()[3],
    ",
  0x50268084u64 => "
      OSPI_0_B_NS.cdabuf()[0],
    ",
  0x50268094u64 => "
      OSPI_0_B_NS.cdabuf()[1],
    ",
  0x502680a4u64 => "
      OSPI_0_B_NS.cdabuf()[2],
    ",
  0x502680b4u64 => "
      OSPI_0_B_NS.cdabuf()[3],
    ",
  0x50268088u64 => "
      OSPI_0_B_NS.cdd0buf()[0],
    ",
  0x50268098u64 => "
      OSPI_0_B_NS.cdd0buf()[1],
    ",
  0x502680a8u64 => "
      OSPI_0_B_NS.cdd0buf()[2],
    ",
  0x502680b8u64 => "
      OSPI_0_B_NS.cdd0buf()[3],
    ",
  0x5026808cu64 => "
      OSPI_0_B_NS.cdd1buf()[0],
    ",
  0x5026809cu64 => "
      OSPI_0_B_NS.cdd1buf()[1],
    ",
  0x502680acu64 => "
      OSPI_0_B_NS.cdd1buf()[2],
    ",
  0x502680bcu64 => "
      OSPI_0_B_NS.cdd1buf()[3],
    ",
  0x50268100u64 => "
      OSPI_0_B_NS.lpctl0(),
    ",
  0x50268104u64 => "
      OSPI_0_B_NS.lpctl1(),
    ",
  0x50268108u64 => "
      OSPI_0_B_NS.lioctl(),
    ",
  0x50268130u64 => "
      OSPI_0_B_NS.ccctl0cs()[0],
    ",
  0x50268150u64 => "
      OSPI_0_B_NS.ccctl0cs()[1],
    ",
  0x50268134u64 => "
      OSPI_0_B_NS.ccctl1cs()[0],
    ",
  0x50268154u64 => "
      OSPI_0_B_NS.ccctl1cs()[1],
    ",
  0x50268138u64 => "
      OSPI_0_B_NS.ccctl2cs()[0],
    ",
  0x50268158u64 => "
      OSPI_0_B_NS.ccctl2cs()[1],
    ",
  0x5026813cu64 => "
      OSPI_0_B_NS.ccctl3cs()[0],
    ",
  0x5026815cu64 => "
      OSPI_0_B_NS.ccctl3cs()[1],
    ",
  0x50268140u64 => "
      OSPI_0_B_NS.ccctl4cs()[0],
    ",
  0x50268160u64 => "
      OSPI_0_B_NS.ccctl4cs()[1],
    ",
  0x50268144u64 => "
      OSPI_0_B_NS.ccctl5cs()[0],
    ",
  0x50268164u64 => "
      OSPI_0_B_NS.ccctl5cs()[1],
    ",
  0x50268148u64 => "
      OSPI_0_B_NS.ccctl6cs()[0],
    ",
  0x50268168u64 => "
      OSPI_0_B_NS.ccctl6cs()[1],
    ",
  0x5026814cu64 => "
      OSPI_0_B_NS.ccctl7cs()[0],
    ",
  0x5026816cu64 => "
      OSPI_0_B_NS.ccctl7cs()[1],
    ",
  0x50268184u64 => "
      OSPI_0_B_NS.comstt(),
    ",
  0x50268188u64 => "
      OSPI_0_B_NS.casttcs()[0],
    ",
  0x5026818cu64 => "
      OSPI_0_B_NS.casttcs()[1],
    ",
  0x50268190u64 => "
      OSPI_0_B_NS.ints(),
    ",
  0x50268194u64 => "
      OSPI_0_B_NS.intc(),
    ",
  0x50268198u64 => "
      OSPI_0_B_NS.inte(),
    ",
  0x50310000u64 => "
      CRC_NS.crccr0(),
    ",
  0x50310001u64 => "
      CRC_NS.crccr1(),
    ",
  0x50310004u64 => "
      CRC_NS.crcdir(),
      CRC_NS.crcdir_by(),
    ",
  0x50310008u64 => "
      CRC_NS.crcdor(),
      CRC_NS.crcdor_ha(),
      CRC_NS.crcdor_by(),
    ",
  0x5031000cu64 => "
      CRC_NS.crcsar(),
    ",
  0x50311000u64 => "
      DOC_B_NS.docr(),
    ",
  0x50311004u64 => "
      DOC_B_NS.dosr(),
    ",
  0x50311008u64 => "
      DOC_B_NS.doscr(),
    ",
  0x5031100cu64 => "
      DOC_B_NS.dodir(),
    ",
  0x50311010u64 => "
      DOC_B_NS.dodsr0(),
    ",
  0x50311014u64 => "
      DOC_B_NS.dodsr1(),
    ",
  0x50322000u64 => "
      GPT_320_NS.gtwp(),
    ",
  0x50322004u64 => "
      GPT_320_NS.gtstr(),
    ",
  0x50322008u64 => "
      GPT_320_NS.gtstp(),
    ",
  0x5032200cu64 => "
      GPT_320_NS.gtclr(),
    ",
  0x50322010u64 => "
      GPT_320_NS.gtssr(),
    ",
  0x50322014u64 => "
      GPT_320_NS.gtpsr(),
    ",
  0x50322018u64 => "
      GPT_320_NS.gtcsr(),
    ",
  0x5032201cu64 => "
      GPT_320_NS.gtupsr(),
    ",
  0x50322020u64 => "
      GPT_320_NS.gtdnsr(),
    ",
  0x50322024u64 => "
      GPT_320_NS.gticasr(),
    ",
  0x50322028u64 => "
      GPT_320_NS.gticbsr(),
    ",
  0x5032202cu64 => "
      GPT_320_NS.gtcr(),
    ",
  0x50322030u64 => "
      GPT_320_NS.gtuddtyc(),
    ",
  0x50322034u64 => "
      GPT_320_NS.gtior(),
    ",
  0x50322038u64 => "
      GPT_320_NS.gtintad(),
    ",
  0x5032203cu64 => "
      GPT_320_NS.gtst(),
    ",
  0x50322040u64 => "
      GPT_320_NS.gtber(),
    ",
  0x50322048u64 => "
      GPT_320_NS.gtcnt(),
    ",
  0x5032204cu64 => "
      GPT_320_NS.gtccra(),
    ",
  0x50322050u64 => "
      GPT_320_NS.gtccrb(),
    ",
  0x50322054u64 => "
      GPT_320_NS.gtccrc(),
    ",
  0x50322058u64 => "
      GPT_320_NS.gtccre(),
    ",
  0x5032205cu64 => "
      GPT_320_NS.gtccrd(),
    ",
  0x50322060u64 => "
      GPT_320_NS.gtccrf(),
    ",
  0x50322064u64 => "
      GPT_320_NS.gtpr(),
    ",
  0x50322068u64 => "
      GPT_320_NS.gtpbr(),
    ",
  0x50322070u64 => "
      GPT_320_NS.gtadtra(),
    ",
  0x50322074u64 => "
      GPT_320_NS.gtadtbra(),
    ",
  0x50322078u64 => "
      GPT_320_NS.gtadtdbra(),
    ",
  0x5032207cu64 => "
      GPT_320_NS.gtadtrb(),
    ",
  0x50322080u64 => "
      GPT_320_NS.gtadtbrb(),
    ",
  0x50322084u64 => "
      GPT_320_NS.gtadtdbrb(),
    ",
  0x50322088u64 => "
      GPT_320_NS.gtdtcr(),
    ",
  0x5032208cu64 => "
      GPT_320_NS.gtdvu(),
    ",
  0x503220a4u64 => "
      GPT_320_NS.gtadsmr(),
    ",
  0x503220b8u64 => "
      GPT_320_NS.gticlf(),
    ",
  0x503220bcu64 => "
      GPT_320_NS.gtpc(),
    ",
  0x503220d0u64 => "
      GPT_320_NS.gtsecsr(),
    ",
  0x503220d4u64 => "
      GPT_320_NS.gtsecr(),
    ",
  0x50322804u64 => "
      GPT_168_NS.gtstr(),
    ",
  0x50322808u64 => "
      GPT_168_NS.gtstp(),
    ",
  0x5032280cu64 => "
      GPT_168_NS.gtclr(),
    ",
  0x50322810u64 => "
      GPT_168_NS.gtssr(),
    ",
  0x50322814u64 => "
      GPT_168_NS.gtpsr(),
    ",
  0x50322818u64 => "
      GPT_168_NS.gtcsr(),
    ",
  0x5032281cu64 => "
      GPT_168_NS.gtupsr(),
    ",
  0x50322820u64 => "
      GPT_168_NS.gtdnsr(),
    ",
  0x50322824u64 => "
      GPT_168_NS.gticasr(),
    ",
  0x50322828u64 => "
      GPT_168_NS.gticbsr(),
    ",
  0x5032282cu64 => "
      GPT_168_NS.gtcr(),
    ",
  0x50322838u64 => "
      GPT_168_NS.gtintad(),
    ",
  0x50322870u64 => "
      GPT_168_NS.gtadtra(),
    ",
  0x50322874u64 => "
      GPT_168_NS.gtadtbra(),
    ",
  0x50322878u64 => "
      GPT_168_NS.gtadtdbra(),
    ",
  0x5032287cu64 => "
      GPT_168_NS.gtadtrb(),
    ",
  0x50322880u64 => "
      GPT_168_NS.gtadtbrb(),
    ",
  0x50322884u64 => "
      GPT_168_NS.gtadtdbrb(),
    ",
  0x503228a4u64 => "
      GPT_168_NS.gtadsmr(),
    ",
  0x503228b8u64 => "
      GPT_168_NS.gticlf(),
    ",
  0x50322a00u64 => "
      GPT_1610_NS.gtwp(),
    ",
  0x50322a30u64 => "
      GPT_1610_NS.gtuddtyc(),
    ",
  0x50322a34u64 => "
      GPT_1610_NS.gtior(),
    ",
  0x50322a3cu64 => "
      GPT_1610_NS.gtst(),
    ",
  0x50322a40u64 => "
      GPT_1610_NS.gtber(),
    ",
  0x50322a48u64 => "
      GPT_1610_NS.gtcnt(),
    ",
  0x50322a4cu64 => "
      GPT_1610_NS.gtccra(),
    ",
  0x50322a50u64 => "
      GPT_1610_NS.gtccrb(),
    ",
  0x50322a54u64 => "
      GPT_1610_NS.gtccrc(),
    ",
  0x50322a58u64 => "
      GPT_1610_NS.gtccre(),
    ",
  0x50322a5cu64 => "
      GPT_1610_NS.gtccrd(),
    ",
  0x50322a60u64 => "
      GPT_1610_NS.gtccrf(),
    ",
  0x50322a64u64 => "
      GPT_1610_NS.gtpr(),
    ",
  0x50322a68u64 => "
      GPT_1610_NS.gtpbr(),
    ",
  0x50322a88u64 => "
      GPT_1610_NS.gtdtcr(),
    ",
  0x50322a8cu64 => "
      GPT_1610_NS.gtdvu(),
    ",
  0x50322abcu64 => "
      GPT_1610_NS.gtpc(),
    ",
  0x50322ad0u64 => "
      GPT_1610_NS.gtsecsr(),
    ",
  0x50322ad4u64 => "
      GPT_1610_NS.gtsecr(),
    ",
  0x50332000u64 => "
      ADC_120_NS.adcsr(),
    ",
  0x50332004u64 => "
      ADC_120_NS.adansa0(),
    ",
  0x50332006u64 => "
      ADC_120_NS.adansa1(),
    ",
  0x50332008u64 => "
      ADC_120_NS.adads0(),
    ",
  0x5033200au64 => "
      ADC_120_NS.adads1(),
    ",
  0x5033200cu64 => "
      ADC_120_NS.adadc(),
    ",
  0x5033200eu64 => "
      ADC_120_NS.adcer(),
    ",
  0x50332010u64 => "
      ADC_120_NS.adstrgr(),
    ",
  0x50332012u64 => "
      ADC_120_NS.adexicr(),
    ",
  0x50332014u64 => "
      ADC_120_NS.adansb0(),
    ",
  0x50332016u64 => "
      ADC_120_NS.adansb1(),
    ",
  0x50332018u64 => "
      ADC_120_NS.addbldr(),
    ",
  0x5033201au64 => "
      ADC_120_NS.adtsdr(),
    ",
  0x5033201cu64 => "
      ADC_120_NS.adocdr(),
    ",
  0x5033201eu64 => "
      ADC_120_NS.adrd(),
    ",
  0x50332028u64 => "
      ADC_120_NS.addr()[4],
    ",
  0x5033202au64 => "
      ADC_120_NS.addr()[5],
    ",
  0x5033202cu64 => "
      ADC_120_NS.addr()[6],
    ",
  0x5033202eu64 => "
      ADC_120_NS.addr()[7],
    ",
  0x50332030u64 => "
      ADC_120_NS.addr()[8],
    ",
  0x50332020u64 => "
      ADC_120_NS.addr()[0],
    ",
  0x50332022u64 => "
      ADC_120_NS.addr()[1],
    ",
  0x50332024u64 => "
      ADC_120_NS.addr()[2],
    ",
  0x50332026u64 => "
      ADC_120_NS.addr()[3],
      ADC_120_NS.advmdr(),
    ",
  0x50332066u64 => "
      ADC_120_NS.adshcr(),
    ",
  0x5033207au64 => "
      ADC_120_NS.addiscr(),
    ",
  0x5033207cu64 => "
      ADC_120_NS.adshmsr(),
    ",
  0x50332080u64 => "
      ADC_120_NS.adgspcr(),
    ",
  0x50332084u64 => "
      ADC_120_NS.addbldra(),
    ",
  0x50332086u64 => "
      ADC_120_NS.addbldrb(),
    ",
  0x5033208cu64 => "
      ADC_120_NS.adwinmon(),
    ",
  0x50332090u64 => "
      ADC_120_NS.adcmpcr(),
    ",
  0x50332092u64 => "
      ADC_120_NS.adcmpanser(),
    ",
  0x50332093u64 => "
      ADC_120_NS.adcmpler(),
    ",
  0x50332094u64 => "
      ADC_120_NS.adcmpansr0(),
    ",
  0x50332098u64 => "
      ADC_120_NS.adcmplr0(),
    ",
  0x5033209cu64 => "
      ADC_120_NS.adcmpdr()[0],
    ",
  0x5033209eu64 => "
      ADC_120_NS.adcmpdr()[1],
    ",
  0x503320a0u64 => "
      ADC_120_NS.adcmpsr0(),
    ",
  0x503320a4u64 => "
      ADC_120_NS.adcmpser(),
    ",
  0x503320a6u64 => "
      ADC_120_NS.adcmpbnsr(),
    ",
  0x503320a8u64 => "
      ADC_120_NS.adwinllb(),
    ",
  0x503320aau64 => "
      ADC_120_NS.adwinulb(),
    ",
  0x503320acu64 => "
      ADC_120_NS.adcmpbsr(),
    ",
  0x503320b0u64 => "
      ADC_120_NS.adbuf()[0],
    ",
  0x503320b2u64 => "
      ADC_120_NS.adbuf()[1],
    ",
  0x503320b4u64 => "
      ADC_120_NS.adbuf()[2],
    ",
  0x503320b6u64 => "
      ADC_120_NS.adbuf()[3],
    ",
  0x503320b8u64 => "
      ADC_120_NS.adbuf()[4],
    ",
  0x503320bau64 => "
      ADC_120_NS.adbuf()[5],
    ",
  0x503320bcu64 => "
      ADC_120_NS.adbuf()[6],
    ",
  0x503320beu64 => "
      ADC_120_NS.adbuf()[7],
    ",
  0x503320c0u64 => "
      ADC_120_NS.adbuf()[8],
    ",
  0x503320c2u64 => "
      ADC_120_NS.adbuf()[9],
    ",
  0x503320c4u64 => "
      ADC_120_NS.adbuf()[10],
    ",
  0x503320c6u64 => "
      ADC_120_NS.adbuf()[11],
    ",
  0x503320c8u64 => "
      ADC_120_NS.adbuf()[12],
    ",
  0x503320cau64 => "
      ADC_120_NS.adbuf()[13],
    ",
  0x503320ccu64 => "
      ADC_120_NS.adbuf()[14],
    ",
  0x503320ceu64 => "
      ADC_120_NS.adbuf()[15],
    ",
  0x503320d0u64 => "
      ADC_120_NS.adbufen(),
    ",
  0x503320d2u64 => "
      ADC_120_NS.adbufptr(),
    ",
  0x503320ddu64 => "
      ADC_120_NS.adsstrl(),
    ",
  0x503320deu64 => "
      ADC_120_NS.adsstrt(),
    ",
  0x503320dfu64 => "
      ADC_120_NS.adsstro(),
    ",
  0x503320e4u64 => "
      ADC_120_NS.adsstr()[4],
    ",
  0x503320e5u64 => "
      ADC_120_NS.adsstr()[5],
    ",
  0x503320e6u64 => "
      ADC_120_NS.adsstr()[6],
    ",
  0x503320e7u64 => "
      ADC_120_NS.adsstr()[7],
    ",
  0x503320e8u64 => "
      ADC_120_NS.adsstr()[8],
    ",
  0x503320e3u64 => "
      ADC_120_NS.adsstrv(),
    ",
  0x503320ecu64 => "
      ADC_120_NS.adsstr()[0],
    ",
  0x503320edu64 => "
      ADC_120_NS.adsstr()[1],
    ",
  0x503320eeu64 => "
      ADC_120_NS.adsstr()[2],
    ",
  0x503320efu64 => "
      ADC_120_NS.adsstr()[3],
    ",
  0x50332200u64 => "
      ADC_121_NS.adcsr(),
    ",
  0x50332204u64 => "
      ADC_121_NS.adansa0(),
    ",
  0x50332206u64 => "
      ADC_121_NS.adansa1(),
    ",
  0x50332208u64 => "
      ADC_121_NS.adads0(),
    ",
  0x5033220au64 => "
      ADC_121_NS.adads1(),
    ",
  0x5033220cu64 => "
      ADC_121_NS.adadc(),
    ",
  0x5033220eu64 => "
      ADC_121_NS.adcer(),
    ",
  0x50332210u64 => "
      ADC_121_NS.adstrgr(),
    ",
  0x50332212u64 => "
      ADC_121_NS.adexicr(),
    ",
  0x50332214u64 => "
      ADC_121_NS.adansb0(),
    ",
  0x50332216u64 => "
      ADC_121_NS.adansb1(),
    ",
  0x50332218u64 => "
      ADC_121_NS.addbldr(),
    ",
  0x5033221au64 => "
      ADC_121_NS.adtsdr(),
    ",
  0x5033221cu64 => "
      ADC_121_NS.adocdr(),
    ",
  0x5033221eu64 => "
      ADC_121_NS.adrd(),
    ",
  0x50332220u64 => "
      ADC_121_NS.addr()[0],
    ",
  0x50332222u64 => "
      ADC_121_NS.addr()[1],
    ",
  0x50332224u64 => "
      ADC_121_NS.addr()[2],
    ",
  0x50332226u64 => "
      ADC_121_NS.addr()[3],
      ADC_121_NS.advmdr(),
    ",
  0x50332228u64 => "
      ADC_121_NS.addr()[4],
    ",
  0x5033222au64 => "
      ADC_121_NS.addr()[5],
    ",
  0x5033222cu64 => "
      ADC_121_NS.addr()[6],
    ",
  0x50332266u64 => "
      ADC_121_NS.adshcr(),
    ",
  0x5033227au64 => "
      ADC_121_NS.addiscr(),
    ",
  0x5033227cu64 => "
      ADC_121_NS.adshmsr(),
    ",
  0x50332280u64 => "
      ADC_121_NS.adgspcr(),
    ",
  0x50332284u64 => "
      ADC_121_NS.addbldra(),
    ",
  0x50332286u64 => "
      ADC_121_NS.addbldrb(),
    ",
  0x5033228cu64 => "
      ADC_121_NS.adwinmon(),
    ",
  0x50332290u64 => "
      ADC_121_NS.adcmpcr(),
    ",
  0x50332292u64 => "
      ADC_121_NS.adcmpanser(),
    ",
  0x50332293u64 => "
      ADC_121_NS.adcmpler(),
    ",
  0x50332294u64 => "
      ADC_121_NS.adcmpansr0(),
    ",
  0x50332298u64 => "
      ADC_121_NS.adcmplr0(),
    ",
  0x5033229cu64 => "
      ADC_121_NS.adcmpdr()[0],
    ",
  0x5033229eu64 => "
      ADC_121_NS.adcmpdr()[1],
    ",
  0x503322a0u64 => "
      ADC_121_NS.adcmpsr0(),
    ",
  0x503322a4u64 => "
      ADC_121_NS.adcmpser(),
    ",
  0x503322a6u64 => "
      ADC_121_NS.adcmpbnsr(),
    ",
  0x503322a8u64 => "
      ADC_121_NS.adwinllb(),
    ",
  0x503322aau64 => "
      ADC_121_NS.adwinulb(),
    ",
  0x503322acu64 => "
      ADC_121_NS.adcmpbsr(),
    ",
  0x503322b0u64 => "
      ADC_121_NS.adbuf()[0],
    ",
  0x503322b2u64 => "
      ADC_121_NS.adbuf()[1],
    ",
  0x503322b4u64 => "
      ADC_121_NS.adbuf()[2],
    ",
  0x503322b6u64 => "
      ADC_121_NS.adbuf()[3],
    ",
  0x503322b8u64 => "
      ADC_121_NS.adbuf()[4],
    ",
  0x503322bau64 => "
      ADC_121_NS.adbuf()[5],
    ",
  0x503322bcu64 => "
      ADC_121_NS.adbuf()[6],
    ",
  0x503322beu64 => "
      ADC_121_NS.adbuf()[7],
    ",
  0x503322c0u64 => "
      ADC_121_NS.adbuf()[8],
    ",
  0x503322c2u64 => "
      ADC_121_NS.adbuf()[9],
    ",
  0x503322c4u64 => "
      ADC_121_NS.adbuf()[10],
    ",
  0x503322c6u64 => "
      ADC_121_NS.adbuf()[11],
    ",
  0x503322c8u64 => "
      ADC_121_NS.adbuf()[12],
    ",
  0x503322cau64 => "
      ADC_121_NS.adbuf()[13],
    ",
  0x503322ccu64 => "
      ADC_121_NS.adbuf()[14],
    ",
  0x503322ceu64 => "
      ADC_121_NS.adbuf()[15],
    ",
  0x503322d0u64 => "
      ADC_121_NS.adbufen(),
    ",
  0x503322d2u64 => "
      ADC_121_NS.adbufptr(),
    ",
  0x503322ddu64 => "
      ADC_121_NS.adsstrl(),
    ",
  0x503322deu64 => "
      ADC_121_NS.adsstrt(),
    ",
  0x503322dfu64 => "
      ADC_121_NS.adsstro(),
    ",
  0x503322e0u64 => "
      ADC_121_NS.adsstr()[0],
    ",
  0x503322e1u64 => "
      ADC_121_NS.adsstr()[1],
    ",
  0x503322e2u64 => "
      ADC_121_NS.adsstr()[2],
    ",
  0x503322e3u64 => "
      ADC_121_NS.adsstr()[3],
      ADC_121_NS.adsstrv(),
    ",
  0x503322e4u64 => "
      ADC_121_NS.adsstr()[4],
    ",
  0x503322e5u64 => "
      ADC_121_NS.adsstr()[5],
    ",
  0x503322e6u64 => "
      ADC_121_NS.adsstr()[6],
    ",
  0x50333000u64 => "
      DAC_12_NS.dadr()[0],
    ",
  0x50333004u64 => "
      DAC_12_NS.dacr(),
    ",
  0x50333005u64 => "
      DAC_12_NS.dadpr(),
    ",
  0x50333006u64 => "
      DAC_12_NS.daadscr(),
    ",
  0x50333008u64 => "
      DAC_12_NS.daampcr(),
    ",
  0x5033301cu64 => "
      DAC_12_NS.daaswcr(),
    ",
  0x503340c0u64 => "
      DAC_12_NS.daadusr(),
    ",
  0x50348000u64 => "
      CEU_NS.capsr(),
    ",
  0x50348004u64 => "
      CEU_NS.capcr(),
    ",
  0x50348008u64 => "
      CEU_NS.camcr(),
    ",
  0x5034800cu64 => "
      CEU_NS.cmcyr(),
    ",
  0x50348010u64 => "
      CEU_NS.camor(),
    ",
  0x50348014u64 => "
      CEU_NS.capwr(),
    ",
  0x50348018u64 => "
      CEU_NS.caifr(),
    ",
  0x50348028u64 => "
      CEU_NS.crcntr(),
    ",
  0x5034802cu64 => "
      CEU_NS.crcmpr(),
    ",
  0x50348030u64 => "
      CEU_NS.cflcr(),
    ",
  0x50348034u64 => "
      CEU_NS.cfszr(),
    ",
  0x50348038u64 => "
      CEU_NS.cdwdr(),
    ",
  0x5034803cu64 => "
      CEU_NS.cdayr(),
    ",
  0x50348040u64 => "
      CEU_NS.cdacr(),
    ",
  0x50348044u64 => "
      CEU_NS.cdbyr(),
    ",
  0x50348048u64 => "
      CEU_NS.cdbcr(),
    ",
  0x5034804cu64 => "
      CEU_NS.cbdsr(),
    ",
  0x5034805cu64 => "
      CEU_NS.cfwcr(),
    ",
  0x50348060u64 => "
      CEU_NS.clfcr(),
    ",
  0x50348064u64 => "
      CEU_NS.cdocr(),
    ",
  0x50348070u64 => "
      CEU_NS.ceier(),
    ",
  0x50348074u64 => "
      CEU_NS.cetcr(),
    ",
  0x5034807cu64 => "
      CEU_NS.cstsr(),
    ",
  0x50348084u64 => "
      CEU_NS.cdssr(),
    ",
  0x50348090u64 => "
      CEU_NS.cdayr2(),
    ",
  0x50348094u64 => "
      CEU_NS.cdacr2(),
    ",
  0x50348098u64 => "
      CEU_NS.cdbyr2(),
    ",
  0x5034809cu64 => "
      CEU_NS.cdbcr2(),
    ",
  0x503480a0u64 => "
      CEU_NS.cbwer(),
    ",
  0x50349010u64 => "
      CEU_NS.camor_b(),
    ",
  0x50349014u64 => "
      CEU_NS.capwr_b(),
    ",
  0x50349030u64 => "
      CEU_NS.cflcr_b(),
    ",
  0x50349034u64 => "
      CEU_NS.cfszr_b(),
    ",
  0x50349038u64 => "
      CEU_NS.cdwdr_b(),
    ",
  0x5034903cu64 => "
      CEU_NS.cdayr_b(),
    ",
  0x50349040u64 => "
      CEU_NS.cdacr_b(),
    ",
  0x50349044u64 => "
      CEU_NS.cdbyr_b(),
    ",
  0x50349048u64 => "
      CEU_NS.cdbcr_b(),
    ",
  0x5034904cu64 => "
      CEU_NS.cbdsr_b(),
    ",
  0x50349060u64 => "
      CEU_NS.clfcr_b(),
    ",
  0x50349064u64 => "
      CEU_NS.cdocr_b(),
    ",
  0x50349090u64 => "
      CEU_NS.cdayr2_b(),
    ",
  0x50349094u64 => "
      CEU_NS.cdacr2_b(),
    ",
  0x50349098u64 => "
      CEU_NS.cdbyr2_b(),
    ",
  0x5034909cu64 => "
      CEU_NS.cdbcr2_b(),
    ",
  0x5034a010u64 => "
      CEU_NS.camor_m(),
    ",
  0x5034a014u64 => "
      CEU_NS.capwr_m(),
    ",
  0x5034a030u64 => "
      CEU_NS.cflcr_m(),
    ",
  0x5034a034u64 => "
      CEU_NS.cfszr_m(),
    ",
  0x5034a038u64 => "
      CEU_NS.cdwdr_m(),
    ",
  0x5034a03cu64 => "
      CEU_NS.cdayr_m(),
    ",
  0x5034a040u64 => "
      CEU_NS.cdacr_m(),
    ",
  0x5034a044u64 => "
      CEU_NS.cdbyr_m(),
    ",
  0x5034a048u64 => "
      CEU_NS.cdbcr_m(),
    ",
  0x5034a04cu64 => "
      CEU_NS.cbdsr_m(),
    ",
  0x5034a060u64 => "
      CEU_NS.clfcr_m(),
    ",
  0x5034a064u64 => "
      CEU_NS.cdocr_m(),
    ",
  0x5034a090u64 => "
      CEU_NS.cdayr2_m(),
    ",
  0x5034a094u64 => "
      CEU_NS.cdacr2_m(),
    ",
  0x5034a098u64 => "
      CEU_NS.cdbyr2_m(),
    ",
  0x5034a09cu64 => "
      CEU_NS.cdbcr2_m(),
    ",
  0x50354000u64 => "
      EDMAC_0_NS.edmr(),
    ",
  0x50354008u64 => "
      EDMAC_0_NS.edtrr(),
    ",
  0x50354010u64 => "
      EDMAC_0_NS.edrrr(),
    ",
  0x50354018u64 => "
      EDMAC_0_NS.tdlar(),
    ",
  0x50354020u64 => "
      EDMAC_0_NS.rdlar(),
    ",
  0x50354028u64 => "
      EDMAC_0_NS.eesr(),
    ",
  0x50354030u64 => "
      EDMAC_0_NS.eesipr(),
    ",
  0x50354038u64 => "
      EDMAC_0_NS.trscer(),
    ",
  0x50354040u64 => "
      EDMAC_0_NS.rmfcr(),
    ",
  0x50354048u64 => "
      EDMAC_0_NS.tftr(),
    ",
  0x50354050u64 => "
      EDMAC_0_NS.fdr(),
    ",
  0x50354058u64 => "
      EDMAC_0_NS.rmcr(),
    ",
  0x50354064u64 => "
      EDMAC_0_NS.tfucr(),
    ",
  0x50354068u64 => "
      EDMAC_0_NS.rfocr(),
    ",
  0x5035406cu64 => "
      EDMAC_0_NS.iosr(),
    ",
  0x50354070u64 => "
      EDMAC_0_NS.fcftr(),
    ",
  0x50354078u64 => "
      EDMAC_0_NS.rpadir(),
    ",
  0x5035407cu64 => "
      EDMAC_0_NS.trimd(),
    ",
  0x503540c8u64 => "
      EDMAC_0_NS.rbwar(),
    ",
  0x503540ccu64 => "
      EDMAC_0_NS.rdfar(),
    ",
  0x503540d4u64 => "
      EDMAC_0_NS.tbrar(),
    ",
  0x503540d8u64 => "
      EDMAC_0_NS.tdfar(),
    ",
  0x50354100u64 => "
      ETHERC_0_NS.ecmr(),
    ",
  0x50354108u64 => "
      ETHERC_0_NS.rflr(),
    ",
  0x50354110u64 => "
      ETHERC_0_NS.ecsr(),
    ",
  0x50354118u64 => "
      ETHERC_0_NS.ecsipr(),
    ",
  0x50354120u64 => "
      ETHERC_0_NS.pir(),
    ",
  0x50354128u64 => "
      ETHERC_0_NS.psr(),
    ",
  0x50354140u64 => "
      ETHERC_0_NS.rdmlr(),
    ",
  0x50354150u64 => "
      ETHERC_0_NS.ipgr(),
    ",
  0x50354154u64 => "
      ETHERC_0_NS.apr(),
    ",
  0x50354158u64 => "
      ETHERC_0_NS.mpr(),
    ",
  0x50354160u64 => "
      ETHERC_0_NS.rfcf(),
    ",
  0x50354164u64 => "
      ETHERC_0_NS.tpauser(),
    ",
  0x50354168u64 => "
      ETHERC_0_NS.tpausecr(),
    ",
  0x5035416cu64 => "
      ETHERC_0_NS.bcfrr(),
    ",
  0x503541c0u64 => "
      ETHERC_0_NS.mahr(),
    ",
  0x503541c8u64 => "
      ETHERC_0_NS.malr(),
    ",
  0x503541d0u64 => "
      ETHERC_0_NS.trocr(),
    ",
  0x503541d4u64 => "
      ETHERC_0_NS.cdcr(),
    ",
  0x503541d8u64 => "
      ETHERC_0_NS.lccr(),
    ",
  0x503541dcu64 => "
      ETHERC_0_NS.cndcr(),
    ",
  0x503541e4u64 => "
      ETHERC_0_NS.cefcr(),
    ",
  0x503541e8u64 => "
      ETHERC_0_NS.frecr(),
    ",
  0x503541ecu64 => "
      ETHERC_0_NS.tsfrcr(),
    ",
  0x503541f0u64 => "
      ETHERC_0_NS.tlfrcr(),
    ",
  0x503541f4u64 => "
      ETHERC_0_NS.rfcr(),
    ",
  0x503541f8u64 => "
      ETHERC_0_NS.mafcr(),
    ",
  0x50358000u64 => "
      SCI_0_B_NS.rdr(),
      SCI_0_B_NS.rdr_by(),
    ",
  0x50358004u64 => "
      SCI_0_B_NS.tdr(),
      SCI_0_B_NS.tdrll(),
    ",
  0x50358005u64 => "
      SCI_0_B_NS.tdrlh(),
    ",
  0x50358008u64 => "
      SCI_0_B_NS.ccr0(),
    ",
  0x5035800cu64 => "
      SCI_0_B_NS.ccr1(),
    ",
  0x50358010u64 => "
      SCI_0_B_NS.ccr2(),
    ",
  0x50358014u64 => "
      SCI_0_B_NS.ccr3(),
    ",
  0x50358018u64 => "
      SCI_0_B_NS.ccr4(),
    ",
  0x5035801cu64 => "
      SCI_0_B_NS.cesr(),
    ",
  0x50358020u64 => "
      SCI_0_B_NS.icr(),
    ",
  0x50358024u64 => "
      SCI_0_B_NS.fcr(),
    ",
  0x5035802cu64 => "
      SCI_0_B_NS.mcr(),
    ",
  0x50358030u64 => "
      SCI_0_B_NS.dcr(),
    ",
  0x50358034u64 => "
      SCI_0_B_NS.xcr0(),
    ",
  0x50358038u64 => "
      SCI_0_B_NS.xcr1(),
    ",
  0x5035803cu64 => "
      SCI_0_B_NS.xcr2(),
    ",
  0x50358048u64 => "
      SCI_0_B_NS.csr(),
    ",
  0x5035804cu64 => "
      SCI_0_B_NS.isr(),
    ",
  0x50358050u64 => "
      SCI_0_B_NS.frsr(),
    ",
  0x50358054u64 => "
      SCI_0_B_NS.ftsr(),
    ",
  0x50358058u64 => "
      SCI_0_B_NS.msr(),
    ",
  0x5035805cu64 => "
      SCI_0_B_NS.xsr0(),
    ",
  0x50358060u64 => "
      SCI_0_B_NS.xsr1(),
    ",
  0x50358068u64 => "
      SCI_0_B_NS.cfclr(),
    ",
  0x5035806cu64 => "
      SCI_0_B_NS.icfclr(),
    ",
  0x50358070u64 => "
      SCI_0_B_NS.ffclr(),
    ",
  0x50358074u64 => "
      SCI_0_B_NS.mfclr(),
    ",
  0x50358078u64 => "
      SCI_0_B_NS.xfclr(),
    ",
  0x5035c000u64 => "
      SPI_0_B_NS.spdr(),
    ",
  0x5035c004u64 => "
      SPI_0_B_NS.spdecr(),
    ",
  0x5035c008u64 => "
      SPI_0_B_NS.spcr(),
    ",
  0x5035c00cu64 => "
      SPI_0_B_NS.spcr2(),
    ",
  0x5035c010u64 => "
      SPI_0_B_NS.spcr3(),
    ",
  0x5035c014u64 => "
      SPI_0_B_NS.spcmd()[0],
    ",
  0x5035c018u64 => "
      SPI_0_B_NS.spcmd()[1],
    ",
  0x5035c01cu64 => "
      SPI_0_B_NS.spcmd()[2],
    ",
  0x5035c020u64 => "
      SPI_0_B_NS.spcmd()[3],
    ",
  0x5035c024u64 => "
      SPI_0_B_NS.spcmd()[4],
    ",
  0x5035c028u64 => "
      SPI_0_B_NS.spcmd()[5],
    ",
  0x5035c02cu64 => "
      SPI_0_B_NS.spcmd()[6],
    ",
  0x5035c030u64 => "
      SPI_0_B_NS.spcmd()[7],
    ",
  0x5035c040u64 => "
      SPI_0_B_NS.spdcr(),
    ",
  0x5035c044u64 => "
      SPI_0_B_NS.spdcr2(),
    ",
  0x5035c050u64 => "
      SPI_0_B_NS.spsr(),
    ",
  0x5035c058u64 => "
      SPI_0_B_NS.sptfsr(),
    ",
  0x5035c05cu64 => "
      SPI_0_B_NS.sprfsr(),
    ",
  0x5035c060u64 => "
      SPI_0_B_NS.sppsr(),
    ",
  0x5035c068u64 => "
      SPI_0_B_NS.spsrc(),
    ",
  0x5035c06cu64 => "
      SPI_0_B_NS.spfcr(),
    ",
  0x5036f200u64 => "
      ECCMB_0_NS.ec710ctl(),
    ",
  0x5036f204u64 => "
      ECCMB_0_NS.ec710tmc(),
    ",
  0x5036f20cu64 => "
      ECCMB_0_NS.ec710ted(),
    ",
  0x5036f210u64 => "
      ECCMB_0_NS.ec710ead0(),
    ",
  0x50380000u64 => "
      CANFD_0_NS.cfdc0ncfg(),
    ",
  0x50380004u64 => "
      CANFD_0_NS.cfdc0ctr(),
    ",
  0x50380008u64 => "
      CANFD_0_NS.cfdc0sts(),
    ",
  0x5038000cu64 => "
      CANFD_0_NS.cfdc0erfl(),
    ",
  0x50380014u64 => "
      CANFD_0_NS.cfdgcfg(),
    ",
  0x50380018u64 => "
      CANFD_0_NS.cfdgctr(),
    ",
  0x5038001cu64 => "
      CANFD_0_NS.cfdgsts(),
    ",
  0x50380020u64 => "
      CANFD_0_NS.cfdgerfl(),
    ",
  0x50380024u64 => "
      CANFD_0_NS.cfdgtsc(),
    ",
  0x50380028u64 => "
      CANFD_0_NS.cfdgaflectr(),
    ",
  0x5038002cu64 => "
      CANFD_0_NS.cfdgaflcfg(),
    ",
  0x50380030u64 => "
      CANFD_0_NS.cfdrmnb(),
    ",
  0x50380034u64 => "
      CANFD_0_NS.cfdrmnd(),
    ",
  0x50380038u64 => "
      CANFD_0_NS.cfdrmiec(),
    ",
  0x5038003cu64 => "
      CANFD_0_NS.cfdrfcc()[0],
    ",
  0x50380040u64 => "
      CANFD_0_NS.cfdrfcc()[1],
    ",
  0x50380044u64 => "
      CANFD_0_NS.cfdrfsts()[0],
    ",
  0x50380048u64 => "
      CANFD_0_NS.cfdrfsts()[1],
    ",
  0x5038004cu64 => "
      CANFD_0_NS.cfdrfpctr()[0],
    ",
  0x50380050u64 => "
      CANFD_0_NS.cfdrfpctr()[1],
    ",
  0x50380054u64 => "
      CANFD_0_NS.cfdcfcc(),
    ",
  0x50380058u64 => "
      CANFD_0_NS.cfdcfsts(),
    ",
  0x5038005cu64 => "
      CANFD_0_NS.cfdcfpctr(),
    ",
  0x50380060u64 => "
      CANFD_0_NS.cfdfests(),
    ",
  0x50380064u64 => "
      CANFD_0_NS.cfdffsts(),
    ",
  0x50380068u64 => "
      CANFD_0_NS.cfdfmsts(),
    ",
  0x5038006cu64 => "
      CANFD_0_NS.cfdrfists(),
    ",
  0x50380070u64 => "
      CANFD_0_NS.cfdtmc()[0],
    ",
  0x50380071u64 => "
      CANFD_0_NS.cfdtmc()[1],
    ",
  0x50380072u64 => "
      CANFD_0_NS.cfdtmc()[2],
    ",
  0x50380073u64 => "
      CANFD_0_NS.cfdtmc()[3],
    ",
  0x50380074u64 => "
      CANFD_0_NS.cfdtmsts()[0],
    ",
  0x50380075u64 => "
      CANFD_0_NS.cfdtmsts()[1],
    ",
  0x50380076u64 => "
      CANFD_0_NS.cfdtmsts()[2],
    ",
  0x50380077u64 => "
      CANFD_0_NS.cfdtmsts()[3],
    ",
  0x50380078u64 => "
      CANFD_0_NS.cfdtmtrsts(),
    ",
  0x5038007cu64 => "
      CANFD_0_NS.cfdtmtarsts(),
    ",
  0x50380080u64 => "
      CANFD_0_NS.cfdtmtcsts(),
    ",
  0x50380084u64 => "
      CANFD_0_NS.cfdtmtasts(),
    ",
  0x50380088u64 => "
      CANFD_0_NS.cfdtmiec(),
    ",
  0x5038008cu64 => "
      CANFD_0_NS.cfdtxqcc(),
    ",
  0x50380090u64 => "
      CANFD_0_NS.cfdtxqsts(),
    ",
  0x50380094u64 => "
      CANFD_0_NS.cfdtxqpctr(),
    ",
  0x50380098u64 => "
      CANFD_0_NS.cfdthlcc(),
    ",
  0x5038009cu64 => "
      CANFD_0_NS.cfdthlsts(),
    ",
  0x503800a0u64 => "
      CANFD_0_NS.cfdthlpctr(),
    ",
  0x503800a4u64 => "
      CANFD_0_NS.cfdgtintsts(),
    ",
  0x503800a8u64 => "
      CANFD_0_NS.cfdgtstcfg(),
    ",
  0x503800acu64 => "
      CANFD_0_NS.cfdgtstctr(),
    ",
  0x503800b0u64 => "
      CANFD_0_NS.cfdgfdcfg(),
    ",
  0x503800b8u64 => "
      CANFD_0_NS.cfdglockk(),
    ",
  0x503800c0u64 => "
      CANFD_0_NS.cfdgaflignent(),
    ",
  0x503800c4u64 => "
      CANFD_0_NS.cfdgaflignctr(),
    ",
  0x503800c8u64 => "
      CANFD_0_NS.cfdcdtct(),
    ",
  0x503800ccu64 => "
      CANFD_0_NS.cfdcdtsts(),
    ",
  0x503800d8u64 => "
      CANFD_0_NS.cfdgrstc(),
    ",
  0x50380100u64 => "
      CANFD_0_NS.cfdc0dcfg(),
    ",
  0x50380104u64 => "
      CANFD_0_NS.cfdc0fdcfg(),
    ",
  0x50380108u64 => "
      CANFD_0_NS.cfdc0fdctr(),
    ",
  0x5038010cu64 => "
      CANFD_0_NS.cfdc0fdsts(),
    ",
  0x50380110u64 => "
      CANFD_0_NS.cfdc0fdcrc(),
    ",
  0x50380120u64 => "
      CANFD_0_NS.cfdgaflid()[0],
    ",
  0x50380130u64 => "
      CANFD_0_NS.cfdgaflid()[1],
    ",
  0x50380140u64 => "
      CANFD_0_NS.cfdgaflid()[2],
    ",
  0x50380150u64 => "
      CANFD_0_NS.cfdgaflid()[3],
    ",
  0x50380160u64 => "
      CANFD_0_NS.cfdgaflid()[4],
    ",
  0x50380170u64 => "
      CANFD_0_NS.cfdgaflid()[5],
    ",
  0x50380180u64 => "
      CANFD_0_NS.cfdgaflid()[6],
    ",
  0x50380190u64 => "
      CANFD_0_NS.cfdgaflid()[7],
    ",
  0x503801a0u64 => "
      CANFD_0_NS.cfdgaflid()[8],
    ",
  0x503801b0u64 => "
      CANFD_0_NS.cfdgaflid()[9],
    ",
  0x503801c0u64 => "
      CANFD_0_NS.cfdgaflid()[10],
    ",
  0x503801d0u64 => "
      CANFD_0_NS.cfdgaflid()[11],
    ",
  0x503801e0u64 => "
      CANFD_0_NS.cfdgaflid()[12],
    ",
  0x503801f0u64 => "
      CANFD_0_NS.cfdgaflid()[13],
    ",
  0x50380200u64 => "
      CANFD_0_NS.cfdgaflid()[14],
    ",
  0x50380210u64 => "
      CANFD_0_NS.cfdgaflid()[15],
    ",
  0x50380124u64 => "
      CANFD_0_NS.cfdgaflm()[0],
    ",
  0x50380134u64 => "
      CANFD_0_NS.cfdgaflm()[1],
    ",
  0x50380144u64 => "
      CANFD_0_NS.cfdgaflm()[2],
    ",
  0x50380154u64 => "
      CANFD_0_NS.cfdgaflm()[3],
    ",
  0x50380164u64 => "
      CANFD_0_NS.cfdgaflm()[4],
    ",
  0x50380174u64 => "
      CANFD_0_NS.cfdgaflm()[5],
    ",
  0x50380184u64 => "
      CANFD_0_NS.cfdgaflm()[6],
    ",
  0x50380194u64 => "
      CANFD_0_NS.cfdgaflm()[7],
    ",
  0x503801a4u64 => "
      CANFD_0_NS.cfdgaflm()[8],
    ",
  0x503801b4u64 => "
      CANFD_0_NS.cfdgaflm()[9],
    ",
  0x503801c4u64 => "
      CANFD_0_NS.cfdgaflm()[10],
    ",
  0x503801d4u64 => "
      CANFD_0_NS.cfdgaflm()[11],
    ",
  0x503801e4u64 => "
      CANFD_0_NS.cfdgaflm()[12],
    ",
  0x503801f4u64 => "
      CANFD_0_NS.cfdgaflm()[13],
    ",
  0x50380204u64 => "
      CANFD_0_NS.cfdgaflm()[14],
    ",
  0x50380214u64 => "
      CANFD_0_NS.cfdgaflm()[15],
    ",
  0x50380128u64 => "
      CANFD_0_NS.cfdgaflp0()[0],
    ",
  0x50380138u64 => "
      CANFD_0_NS.cfdgaflp0()[1],
    ",
  0x50380148u64 => "
      CANFD_0_NS.cfdgaflp0()[2],
    ",
  0x50380158u64 => "
      CANFD_0_NS.cfdgaflp0()[3],
    ",
  0x50380168u64 => "
      CANFD_0_NS.cfdgaflp0()[4],
    ",
  0x50380178u64 => "
      CANFD_0_NS.cfdgaflp0()[5],
    ",
  0x50380188u64 => "
      CANFD_0_NS.cfdgaflp0()[6],
    ",
  0x50380198u64 => "
      CANFD_0_NS.cfdgaflp0()[7],
    ",
  0x503801a8u64 => "
      CANFD_0_NS.cfdgaflp0()[8],
    ",
  0x503801b8u64 => "
      CANFD_0_NS.cfdgaflp0()[9],
    ",
  0x503801c8u64 => "
      CANFD_0_NS.cfdgaflp0()[10],
    ",
  0x503801d8u64 => "
      CANFD_0_NS.cfdgaflp0()[11],
    ",
  0x503801e8u64 => "
      CANFD_0_NS.cfdgaflp0()[12],
    ",
  0x503801f8u64 => "
      CANFD_0_NS.cfdgaflp0()[13],
    ",
  0x50380208u64 => "
      CANFD_0_NS.cfdgaflp0()[14],
    ",
  0x50380218u64 => "
      CANFD_0_NS.cfdgaflp0()[15],
    ",
  0x5038012cu64 => "
      CANFD_0_NS.cfdgaflp1()[0],
    ",
  0x5038013cu64 => "
      CANFD_0_NS.cfdgaflp1()[1],
    ",
  0x5038014cu64 => "
      CANFD_0_NS.cfdgaflp1()[2],
    ",
  0x5038015cu64 => "
      CANFD_0_NS.cfdgaflp1()[3],
    ",
  0x5038016cu64 => "
      CANFD_0_NS.cfdgaflp1()[4],
    ",
  0x5038017cu64 => "
      CANFD_0_NS.cfdgaflp1()[5],
    ",
  0x5038018cu64 => "
      CANFD_0_NS.cfdgaflp1()[6],
    ",
  0x5038019cu64 => "
      CANFD_0_NS.cfdgaflp1()[7],
    ",
  0x503801acu64 => "
      CANFD_0_NS.cfdgaflp1()[8],
    ",
  0x503801bcu64 => "
      CANFD_0_NS.cfdgaflp1()[9],
    ",
  0x503801ccu64 => "
      CANFD_0_NS.cfdgaflp1()[10],
    ",
  0x503801dcu64 => "
      CANFD_0_NS.cfdgaflp1()[11],
    ",
  0x503801ecu64 => "
      CANFD_0_NS.cfdgaflp1()[12],
    ",
  0x503801fcu64 => "
      CANFD_0_NS.cfdgaflp1()[13],
    ",
  0x5038020cu64 => "
      CANFD_0_NS.cfdgaflp1()[14],
    ",
  0x5038021cu64 => "
      CANFD_0_NS.cfdgaflp1()[15],
    ",
  0x50380280u64 => "
      CANFD_0_NS.cfdrpgacc()[0],
    ",
  0x50380284u64 => "
      CANFD_0_NS.cfdrpgacc()[1],
    ",
  0x50380288u64 => "
      CANFD_0_NS.cfdrpgacc()[2],
    ",
  0x5038028cu64 => "
      CANFD_0_NS.cfdrpgacc()[3],
    ",
  0x50380290u64 => "
      CANFD_0_NS.cfdrpgacc()[4],
    ",
  0x50380294u64 => "
      CANFD_0_NS.cfdrpgacc()[5],
    ",
  0x50380298u64 => "
      CANFD_0_NS.cfdrpgacc()[6],
    ",
  0x5038029cu64 => "
      CANFD_0_NS.cfdrpgacc()[7],
    ",
  0x503802a0u64 => "
      CANFD_0_NS.cfdrpgacc()[8],
    ",
  0x503802a4u64 => "
      CANFD_0_NS.cfdrpgacc()[9],
    ",
  0x503802a8u64 => "
      CANFD_0_NS.cfdrpgacc()[10],
    ",
  0x503802acu64 => "
      CANFD_0_NS.cfdrpgacc()[11],
    ",
  0x503802b0u64 => "
      CANFD_0_NS.cfdrpgacc()[12],
    ",
  0x503802b4u64 => "
      CANFD_0_NS.cfdrpgacc()[13],
    ",
  0x503802b8u64 => "
      CANFD_0_NS.cfdrpgacc()[14],
    ",
  0x503802bcu64 => "
      CANFD_0_NS.cfdrpgacc()[15],
    ",
  0x503802c0u64 => "
      CANFD_0_NS.cfdrpgacc()[16],
    ",
  0x503802c4u64 => "
      CANFD_0_NS.cfdrpgacc()[17],
    ",
  0x503802c8u64 => "
      CANFD_0_NS.cfdrpgacc()[18],
    ",
  0x503802ccu64 => "
      CANFD_0_NS.cfdrpgacc()[19],
    ",
  0x503802d0u64 => "
      CANFD_0_NS.cfdrpgacc()[20],
    ",
  0x503802d4u64 => "
      CANFD_0_NS.cfdrpgacc()[21],
    ",
  0x503802d8u64 => "
      CANFD_0_NS.cfdrpgacc()[22],
    ",
  0x503802dcu64 => "
      CANFD_0_NS.cfdrpgacc()[23],
    ",
  0x503802e0u64 => "
      CANFD_0_NS.cfdrpgacc()[24],
    ",
  0x503802e4u64 => "
      CANFD_0_NS.cfdrpgacc()[25],
    ",
  0x503802e8u64 => "
      CANFD_0_NS.cfdrpgacc()[26],
    ",
  0x503802ecu64 => "
      CANFD_0_NS.cfdrpgacc()[27],
    ",
  0x503802f0u64 => "
      CANFD_0_NS.cfdrpgacc()[28],
    ",
  0x503802f4u64 => "
      CANFD_0_NS.cfdrpgacc()[29],
    ",
  0x503802f8u64 => "
      CANFD_0_NS.cfdrpgacc()[30],
    ",
  0x503802fcu64 => "
      CANFD_0_NS.cfdrpgacc()[31],
    ",
  0x50380300u64 => "
      CANFD_0_NS.cfdrpgacc()[32],
    ",
  0x50380304u64 => "
      CANFD_0_NS.cfdrpgacc()[33],
    ",
  0x50380308u64 => "
      CANFD_0_NS.cfdrpgacc()[34],
    ",
  0x5038030cu64 => "
      CANFD_0_NS.cfdrpgacc()[35],
    ",
  0x50380310u64 => "
      CANFD_0_NS.cfdrpgacc()[36],
    ",
  0x50380314u64 => "
      CANFD_0_NS.cfdrpgacc()[37],
    ",
  0x50380318u64 => "
      CANFD_0_NS.cfdrpgacc()[38],
    ",
  0x5038031cu64 => "
      CANFD_0_NS.cfdrpgacc()[39],
    ",
  0x50380320u64 => "
      CANFD_0_NS.cfdrpgacc()[40],
    ",
  0x50380324u64 => "
      CANFD_0_NS.cfdrpgacc()[41],
    ",
  0x50380328u64 => "
      CANFD_0_NS.cfdrpgacc()[42],
    ",
  0x5038032cu64 => "
      CANFD_0_NS.cfdrpgacc()[43],
    ",
  0x50380330u64 => "
      CANFD_0_NS.cfdrpgacc()[44],
    ",
  0x50380334u64 => "
      CANFD_0_NS.cfdrpgacc()[45],
    ",
  0x50380338u64 => "
      CANFD_0_NS.cfdrpgacc()[46],
    ",
  0x5038033cu64 => "
      CANFD_0_NS.cfdrpgacc()[47],
    ",
  0x50380340u64 => "
      CANFD_0_NS.cfdrpgacc()[48],
    ",
  0x50380344u64 => "
      CANFD_0_NS.cfdrpgacc()[49],
    ",
  0x50380348u64 => "
      CANFD_0_NS.cfdrpgacc()[50],
    ",
  0x5038034cu64 => "
      CANFD_0_NS.cfdrpgacc()[51],
    ",
  0x50380350u64 => "
      CANFD_0_NS.cfdrpgacc()[52],
    ",
  0x50380354u64 => "
      CANFD_0_NS.cfdrpgacc()[53],
    ",
  0x50380358u64 => "
      CANFD_0_NS.cfdrpgacc()[54],
    ",
  0x5038035cu64 => "
      CANFD_0_NS.cfdrpgacc()[55],
    ",
  0x50380360u64 => "
      CANFD_0_NS.cfdrpgacc()[56],
    ",
  0x50380364u64 => "
      CANFD_0_NS.cfdrpgacc()[57],
    ",
  0x50380368u64 => "
      CANFD_0_NS.cfdrpgacc()[58],
    ",
  0x5038036cu64 => "
      CANFD_0_NS.cfdrpgacc()[59],
    ",
  0x50380370u64 => "
      CANFD_0_NS.cfdrpgacc()[60],
    ",
  0x50380374u64 => "
      CANFD_0_NS.cfdrpgacc()[61],
    ",
  0x50380378u64 => "
      CANFD_0_NS.cfdrpgacc()[62],
    ",
  0x5038037cu64 => "
      CANFD_0_NS.cfdrpgacc()[63],
    ",
  0x50380520u64 => "
      CANFD_0_NS.cfdrfid()[0],
    ",
  0x5038056cu64 => "
      CANFD_0_NS.cfdrfid()[1],
    ",
  0x50380524u64 => "
      CANFD_0_NS.cfdrfptr()[0],
    ",
  0x50380570u64 => "
      CANFD_0_NS.cfdrfptr()[1],
    ",
  0x50380528u64 => "
      CANFD_0_NS.cfdrffdsts()[0],
    ",
  0x50380574u64 => "
      CANFD_0_NS.cfdrffdsts()[1],
    ",
  0x5038052cu64 => "
      CANFD_0_NS.cfdrfdf_0()[0],
    ",
  0x50380578u64 => "
      CANFD_0_NS.cfdrfdf_0()[1],
    ",
  0x50380530u64 => "
      CANFD_0_NS.cfdrfdf_1()[0],
    ",
  0x5038057cu64 => "
      CANFD_0_NS.cfdrfdf_1()[1],
    ",
  0x50380534u64 => "
      CANFD_0_NS.cfdrfdf_2()[0],
    ",
  0x50380580u64 => "
      CANFD_0_NS.cfdrfdf_2()[1],
    ",
  0x50380538u64 => "
      CANFD_0_NS.cfdrfdf_3()[0],
    ",
  0x50380584u64 => "
      CANFD_0_NS.cfdrfdf_3()[1],
    ",
  0x5038053cu64 => "
      CANFD_0_NS.cfdrfdf_4()[0],
    ",
  0x50380588u64 => "
      CANFD_0_NS.cfdrfdf_4()[1],
    ",
  0x50380540u64 => "
      CANFD_0_NS.cfdrfdf_5()[0],
    ",
  0x5038058cu64 => "
      CANFD_0_NS.cfdrfdf_5()[1],
    ",
  0x50380544u64 => "
      CANFD_0_NS.cfdrfdf_6()[0],
    ",
  0x50380590u64 => "
      CANFD_0_NS.cfdrfdf_6()[1],
    ",
  0x50380548u64 => "
      CANFD_0_NS.cfdrfdf_7()[0],
    ",
  0x50380594u64 => "
      CANFD_0_NS.cfdrfdf_7()[1],
    ",
  0x5038054cu64 => "
      CANFD_0_NS.cfdrfdf_8()[0],
    ",
  0x50380598u64 => "
      CANFD_0_NS.cfdrfdf_8()[1],
    ",
  0x50380550u64 => "
      CANFD_0_NS.cfdrfdf_9()[0],
    ",
  0x5038059cu64 => "
      CANFD_0_NS.cfdrfdf_9()[1],
    ",
  0x50380554u64 => "
      CANFD_0_NS.cfdrfdf_10()[0],
    ",
  0x503805a0u64 => "
      CANFD_0_NS.cfdrfdf_10()[1],
    ",
  0x50380558u64 => "
      CANFD_0_NS.cfdrfdf_11()[0],
    ",
  0x503805a4u64 => "
      CANFD_0_NS.cfdrfdf_11()[1],
    ",
  0x5038055cu64 => "
      CANFD_0_NS.cfdrfdf_12()[0],
    ",
  0x503805a8u64 => "
      CANFD_0_NS.cfdrfdf_12()[1],
    ",
  0x50380560u64 => "
      CANFD_0_NS.cfdrfdf_13()[0],
    ",
  0x503805acu64 => "
      CANFD_0_NS.cfdrfdf_13()[1],
    ",
  0x50380564u64 => "
      CANFD_0_NS.cfdrfdf_14()[0],
    ",
  0x503805b0u64 => "
      CANFD_0_NS.cfdrfdf_14()[1],
    ",
  0x50380568u64 => "
      CANFD_0_NS.cfdrfdf_15()[0],
    ",
  0x503805b4u64 => "
      CANFD_0_NS.cfdrfdf_15()[1],
    ",
  0x503805b8u64 => "
      CANFD_0_NS.cfdcfid(),
    ",
  0x503805bcu64 => "
      CANFD_0_NS.cfdcfptr(),
    ",
  0x503805c0u64 => "
      CANFD_0_NS.cfdcffdcsts(),
    ",
  0x503805c4u64 => "
      CANFD_0_NS.cfdcfdf()[0],
    ",
  0x503805c8u64 => "
      CANFD_0_NS.cfdcfdf()[1],
    ",
  0x503805ccu64 => "
      CANFD_0_NS.cfdcfdf()[2],
    ",
  0x503805d0u64 => "
      CANFD_0_NS.cfdcfdf()[3],
    ",
  0x503805d4u64 => "
      CANFD_0_NS.cfdcfdf()[4],
    ",
  0x503805d8u64 => "
      CANFD_0_NS.cfdcfdf()[5],
    ",
  0x503805dcu64 => "
      CANFD_0_NS.cfdcfdf()[6],
    ",
  0x503805e0u64 => "
      CANFD_0_NS.cfdcfdf()[7],
    ",
  0x503805e4u64 => "
      CANFD_0_NS.cfdcfdf()[8],
    ",
  0x503805e8u64 => "
      CANFD_0_NS.cfdcfdf()[9],
    ",
  0x503805ecu64 => "
      CANFD_0_NS.cfdcfdf()[10],
    ",
  0x503805f0u64 => "
      CANFD_0_NS.cfdcfdf()[11],
    ",
  0x503805f4u64 => "
      CANFD_0_NS.cfdcfdf()[12],
    ",
  0x503805f8u64 => "
      CANFD_0_NS.cfdcfdf()[13],
    ",
  0x503805fcu64 => "
      CANFD_0_NS.cfdcfdf()[14],
    ",
  0x50380600u64 => "
      CANFD_0_NS.cfdcfdf()[15],
    ",
  0x50380604u64 => "
      CANFD_0_NS.cfdtmid()[0],
    ",
  0x50380650u64 => "
      CANFD_0_NS.cfdtmid()[1],
    ",
  0x5038069cu64 => "
      CANFD_0_NS.cfdtmid()[2],
    ",
  0x503806e8u64 => "
      CANFD_0_NS.cfdtmid()[3],
    ",
  0x50380608u64 => "
      CANFD_0_NS.cfdtmptr()[0],
    ",
  0x50380654u64 => "
      CANFD_0_NS.cfdtmptr()[1],
    ",
  0x503806a0u64 => "
      CANFD_0_NS.cfdtmptr()[2],
    ",
  0x503806ecu64 => "
      CANFD_0_NS.cfdtmptr()[3],
    ",
  0x5038060cu64 => "
      CANFD_0_NS.cfdtmfdctr()[0],
    ",
  0x50380658u64 => "
      CANFD_0_NS.cfdtmfdctr()[1],
    ",
  0x503806a4u64 => "
      CANFD_0_NS.cfdtmfdctr()[2],
    ",
  0x503806f0u64 => "
      CANFD_0_NS.cfdtmfdctr()[3],
    ",
  0x50380610u64 => "
      CANFD_0_NS.cfdtmdf_0()[0],
    ",
  0x5038065cu64 => "
      CANFD_0_NS.cfdtmdf_0()[1],
    ",
  0x503806a8u64 => "
      CANFD_0_NS.cfdtmdf_0()[2],
    ",
  0x503806f4u64 => "
      CANFD_0_NS.cfdtmdf_0()[3],
    ",
  0x50380614u64 => "
      CANFD_0_NS.cfdtmdf_1()[0],
    ",
  0x50380660u64 => "
      CANFD_0_NS.cfdtmdf_1()[1],
    ",
  0x503806acu64 => "
      CANFD_0_NS.cfdtmdf_1()[2],
    ",
  0x503806f8u64 => "
      CANFD_0_NS.cfdtmdf_1()[3],
    ",
  0x50380618u64 => "
      CANFD_0_NS.cfdtmdf_2()[0],
    ",
  0x50380664u64 => "
      CANFD_0_NS.cfdtmdf_2()[1],
    ",
  0x503806b0u64 => "
      CANFD_0_NS.cfdtmdf_2()[2],
    ",
  0x503806fcu64 => "
      CANFD_0_NS.cfdtmdf_2()[3],
    ",
  0x5038061cu64 => "
      CANFD_0_NS.cfdtmdf_3()[0],
    ",
  0x50380668u64 => "
      CANFD_0_NS.cfdtmdf_3()[1],
    ",
  0x503806b4u64 => "
      CANFD_0_NS.cfdtmdf_3()[2],
    ",
  0x50380700u64 => "
      CANFD_0_NS.cfdtmdf_3()[3],
    ",
  0x50380620u64 => "
      CANFD_0_NS.cfdtmdf_4()[0],
    ",
  0x5038066cu64 => "
      CANFD_0_NS.cfdtmdf_4()[1],
    ",
  0x503806b8u64 => "
      CANFD_0_NS.cfdtmdf_4()[2],
    ",
  0x50380704u64 => "
      CANFD_0_NS.cfdtmdf_4()[3],
    ",
  0x50380624u64 => "
      CANFD_0_NS.cfdtmdf_5()[0],
    ",
  0x50380670u64 => "
      CANFD_0_NS.cfdtmdf_5()[1],
    ",
  0x503806bcu64 => "
      CANFD_0_NS.cfdtmdf_5()[2],
    ",
  0x50380708u64 => "
      CANFD_0_NS.cfdtmdf_5()[3],
    ",
  0x50380628u64 => "
      CANFD_0_NS.cfdtmdf_6()[0],
    ",
  0x50380674u64 => "
      CANFD_0_NS.cfdtmdf_6()[1],
    ",
  0x503806c0u64 => "
      CANFD_0_NS.cfdtmdf_6()[2],
    ",
  0x5038070cu64 => "
      CANFD_0_NS.cfdtmdf_6()[3],
    ",
  0x5038062cu64 => "
      CANFD_0_NS.cfdtmdf_7()[0],
    ",
  0x50380678u64 => "
      CANFD_0_NS.cfdtmdf_7()[1],
    ",
  0x503806c4u64 => "
      CANFD_0_NS.cfdtmdf_7()[2],
    ",
  0x50380710u64 => "
      CANFD_0_NS.cfdtmdf_7()[3],
    ",
  0x50380630u64 => "
      CANFD_0_NS.cfdtmdf_8()[0],
    ",
  0x5038067cu64 => "
      CANFD_0_NS.cfdtmdf_8()[1],
    ",
  0x503806c8u64 => "
      CANFD_0_NS.cfdtmdf_8()[2],
    ",
  0x50380714u64 => "
      CANFD_0_NS.cfdtmdf_8()[3],
    ",
  0x50380634u64 => "
      CANFD_0_NS.cfdtmdf_9()[0],
    ",
  0x50380680u64 => "
      CANFD_0_NS.cfdtmdf_9()[1],
    ",
  0x503806ccu64 => "
      CANFD_0_NS.cfdtmdf_9()[2],
    ",
  0x50380718u64 => "
      CANFD_0_NS.cfdtmdf_9()[3],
    ",
  0x50380638u64 => "
      CANFD_0_NS.cfdtmdf_10()[0],
    ",
  0x50380684u64 => "
      CANFD_0_NS.cfdtmdf_10()[1],
    ",
  0x503806d0u64 => "
      CANFD_0_NS.cfdtmdf_10()[2],
    ",
  0x5038071cu64 => "
      CANFD_0_NS.cfdtmdf_10()[3],
    ",
  0x5038063cu64 => "
      CANFD_0_NS.cfdtmdf_11()[0],
    ",
  0x50380688u64 => "
      CANFD_0_NS.cfdtmdf_11()[1],
    ",
  0x503806d4u64 => "
      CANFD_0_NS.cfdtmdf_11()[2],
    ",
  0x50380720u64 => "
      CANFD_0_NS.cfdtmdf_11()[3],
    ",
  0x50380640u64 => "
      CANFD_0_NS.cfdtmdf_12()[0],
    ",
  0x5038068cu64 => "
      CANFD_0_NS.cfdtmdf_12()[1],
    ",
  0x503806d8u64 => "
      CANFD_0_NS.cfdtmdf_12()[2],
    ",
  0x50380724u64 => "
      CANFD_0_NS.cfdtmdf_12()[3],
    ",
  0x50380644u64 => "
      CANFD_0_NS.cfdtmdf_13()[0],
    ",
  0x50380690u64 => "
      CANFD_0_NS.cfdtmdf_13()[1],
    ",
  0x503806dcu64 => "
      CANFD_0_NS.cfdtmdf_13()[2],
    ",
  0x50380728u64 => "
      CANFD_0_NS.cfdtmdf_13()[3],
    ",
  0x50380648u64 => "
      CANFD_0_NS.cfdtmdf_14()[0],
    ",
  0x50380694u64 => "
      CANFD_0_NS.cfdtmdf_14()[1],
    ",
  0x503806e0u64 => "
      CANFD_0_NS.cfdtmdf_14()[2],
    ",
  0x5038072cu64 => "
      CANFD_0_NS.cfdtmdf_14()[3],
    ",
  0x5038064cu64 => "
      CANFD_0_NS.cfdtmdf_15()[0],
    ",
  0x50380698u64 => "
      CANFD_0_NS.cfdtmdf_15()[1],
    ",
  0x503806e4u64 => "
      CANFD_0_NS.cfdtmdf_15()[2],
    ",
  0x50380730u64 => "
      CANFD_0_NS.cfdtmdf_15()[3],
    ",
  0x50380740u64 => "
      CANFD_0_NS.cfdthlacc0(),
    ",
  0x50380744u64 => "
      CANFD_0_NS.cfdthlacc1(),
    ",
  0x50380d20u64 => "
      CANFD_0_NS.cfdrmid()[0],
    ",
  0x50380d6cu64 => "
      CANFD_0_NS.cfdrmid()[1],
    ",
  0x50380db8u64 => "
      CANFD_0_NS.cfdrmid()[2],
    ",
  0x50380e04u64 => "
      CANFD_0_NS.cfdrmid()[3],
    ",
  0x50380e50u64 => "
      CANFD_0_NS.cfdrmid()[4],
    ",
  0x50380e9cu64 => "
      CANFD_0_NS.cfdrmid()[5],
    ",
  0x50380ee8u64 => "
      CANFD_0_NS.cfdrmid()[6],
    ",
  0x50380f34u64 => "
      CANFD_0_NS.cfdrmid()[7],
    ",
  0x50381524u64 => "
      CANFD_0_NS.cfdrmptr()[0],
    ",
  0x50381570u64 => "
      CANFD_0_NS.cfdrmptr()[1],
    ",
  0x503815bcu64 => "
      CANFD_0_NS.cfdrmptr()[2],
    ",
  0x50381608u64 => "
      CANFD_0_NS.cfdrmptr()[3],
    ",
  0x50381654u64 => "
      CANFD_0_NS.cfdrmptr()[4],
    ",
  0x503816a0u64 => "
      CANFD_0_NS.cfdrmptr()[5],
    ",
  0x503816ecu64 => "
      CANFD_0_NS.cfdrmptr()[6],
    ",
  0x50381738u64 => "
      CANFD_0_NS.cfdrmptr()[7],
    ",
  0x50381528u64 => "
      CANFD_0_NS.cfdrmfdsts()[0],
    ",
  0x50381574u64 => "
      CANFD_0_NS.cfdrmfdsts()[1],
    ",
  0x503815c0u64 => "
      CANFD_0_NS.cfdrmfdsts()[2],
    ",
  0x5038160cu64 => "
      CANFD_0_NS.cfdrmfdsts()[3],
    ",
  0x50381658u64 => "
      CANFD_0_NS.cfdrmfdsts()[4],
    ",
  0x503816a4u64 => "
      CANFD_0_NS.cfdrmfdsts()[5],
    ",
  0x503816f0u64 => "
      CANFD_0_NS.cfdrmfdsts()[6],
    ",
  0x5038173cu64 => "
      CANFD_0_NS.cfdrmfdsts()[7],
    ",
  0x5038152cu64 => "
      CANFD_0_NS.cfdrmdf_0()[0],
    ",
  0x50381578u64 => "
      CANFD_0_NS.cfdrmdf_0()[1],
    ",
  0x503815c4u64 => "
      CANFD_0_NS.cfdrmdf_0()[2],
    ",
  0x50381610u64 => "
      CANFD_0_NS.cfdrmdf_0()[3],
    ",
  0x5038165cu64 => "
      CANFD_0_NS.cfdrmdf_0()[4],
    ",
  0x503816a8u64 => "
      CANFD_0_NS.cfdrmdf_0()[5],
    ",
  0x503816f4u64 => "
      CANFD_0_NS.cfdrmdf_0()[6],
    ",
  0x50381740u64 => "
      CANFD_0_NS.cfdrmdf_0()[7],
    ",
  0x50381530u64 => "
      CANFD_0_NS.cfdrmdf_1()[0],
    ",
  0x5038157cu64 => "
      CANFD_0_NS.cfdrmdf_1()[1],
    ",
  0x503815c8u64 => "
      CANFD_0_NS.cfdrmdf_1()[2],
    ",
  0x50381614u64 => "
      CANFD_0_NS.cfdrmdf_1()[3],
    ",
  0x50381660u64 => "
      CANFD_0_NS.cfdrmdf_1()[4],
    ",
  0x503816acu64 => "
      CANFD_0_NS.cfdrmdf_1()[5],
    ",
  0x503816f8u64 => "
      CANFD_0_NS.cfdrmdf_1()[6],
    ",
  0x50381744u64 => "
      CANFD_0_NS.cfdrmdf_1()[7],
    ",
  0x50381534u64 => "
      CANFD_0_NS.cfdrmdf_2()[0],
    ",
  0x50381580u64 => "
      CANFD_0_NS.cfdrmdf_2()[1],
    ",
  0x503815ccu64 => "
      CANFD_0_NS.cfdrmdf_2()[2],
    ",
  0x50381618u64 => "
      CANFD_0_NS.cfdrmdf_2()[3],
    ",
  0x50381664u64 => "
      CANFD_0_NS.cfdrmdf_2()[4],
    ",
  0x503816b0u64 => "
      CANFD_0_NS.cfdrmdf_2()[5],
    ",
  0x503816fcu64 => "
      CANFD_0_NS.cfdrmdf_2()[6],
    ",
  0x50381748u64 => "
      CANFD_0_NS.cfdrmdf_2()[7],
    ",
  0x50381538u64 => "
      CANFD_0_NS.cfdrmdf_3()[0],
    ",
  0x50381584u64 => "
      CANFD_0_NS.cfdrmdf_3()[1],
    ",
  0x503815d0u64 => "
      CANFD_0_NS.cfdrmdf_3()[2],
    ",
  0x5038161cu64 => "
      CANFD_0_NS.cfdrmdf_3()[3],
    ",
  0x50381668u64 => "
      CANFD_0_NS.cfdrmdf_3()[4],
    ",
  0x503816b4u64 => "
      CANFD_0_NS.cfdrmdf_3()[5],
    ",
  0x50381700u64 => "
      CANFD_0_NS.cfdrmdf_3()[6],
    ",
  0x5038174cu64 => "
      CANFD_0_NS.cfdrmdf_3()[7],
    ",
  0x5038153cu64 => "
      CANFD_0_NS.cfdrmdf_4()[0],
    ",
  0x50381588u64 => "
      CANFD_0_NS.cfdrmdf_4()[1],
    ",
  0x503815d4u64 => "
      CANFD_0_NS.cfdrmdf_4()[2],
    ",
  0x50381620u64 => "
      CANFD_0_NS.cfdrmdf_4()[3],
    ",
  0x5038166cu64 => "
      CANFD_0_NS.cfdrmdf_4()[4],
    ",
  0x503816b8u64 => "
      CANFD_0_NS.cfdrmdf_4()[5],
    ",
  0x50381704u64 => "
      CANFD_0_NS.cfdrmdf_4()[6],
    ",
  0x50381750u64 => "
      CANFD_0_NS.cfdrmdf_4()[7],
    ",
  0x50381540u64 => "
      CANFD_0_NS.cfdrmdf_5()[0],
    ",
  0x5038158cu64 => "
      CANFD_0_NS.cfdrmdf_5()[1],
    ",
  0x503815d8u64 => "
      CANFD_0_NS.cfdrmdf_5()[2],
    ",
  0x50381624u64 => "
      CANFD_0_NS.cfdrmdf_5()[3],
    ",
  0x50381670u64 => "
      CANFD_0_NS.cfdrmdf_5()[4],
    ",
  0x503816bcu64 => "
      CANFD_0_NS.cfdrmdf_5()[5],
    ",
  0x50381708u64 => "
      CANFD_0_NS.cfdrmdf_5()[6],
    ",
  0x50381754u64 => "
      CANFD_0_NS.cfdrmdf_5()[7],
    ",
  0x50381544u64 => "
      CANFD_0_NS.cfdrmdf_6()[0],
    ",
  0x50381590u64 => "
      CANFD_0_NS.cfdrmdf_6()[1],
    ",
  0x503815dcu64 => "
      CANFD_0_NS.cfdrmdf_6()[2],
    ",
  0x50381628u64 => "
      CANFD_0_NS.cfdrmdf_6()[3],
    ",
  0x50381674u64 => "
      CANFD_0_NS.cfdrmdf_6()[4],
    ",
  0x503816c0u64 => "
      CANFD_0_NS.cfdrmdf_6()[5],
    ",
  0x5038170cu64 => "
      CANFD_0_NS.cfdrmdf_6()[6],
    ",
  0x50381758u64 => "
      CANFD_0_NS.cfdrmdf_6()[7],
    ",
  0x50381548u64 => "
      CANFD_0_NS.cfdrmdf_7()[0],
    ",
  0x50381594u64 => "
      CANFD_0_NS.cfdrmdf_7()[1],
    ",
  0x503815e0u64 => "
      CANFD_0_NS.cfdrmdf_7()[2],
    ",
  0x5038162cu64 => "
      CANFD_0_NS.cfdrmdf_7()[3],
    ",
  0x50381678u64 => "
      CANFD_0_NS.cfdrmdf_7()[4],
    ",
  0x503816c4u64 => "
      CANFD_0_NS.cfdrmdf_7()[5],
    ",
  0x50381710u64 => "
      CANFD_0_NS.cfdrmdf_7()[6],
    ",
  0x5038175cu64 => "
      CANFD_0_NS.cfdrmdf_7()[7],
    ",
  0x5038154cu64 => "
      CANFD_0_NS.cfdrmdf_8()[0],
    ",
  0x50381598u64 => "
      CANFD_0_NS.cfdrmdf_8()[1],
    ",
  0x503815e4u64 => "
      CANFD_0_NS.cfdrmdf_8()[2],
    ",
  0x50381630u64 => "
      CANFD_0_NS.cfdrmdf_8()[3],
    ",
  0x5038167cu64 => "
      CANFD_0_NS.cfdrmdf_8()[4],
    ",
  0x503816c8u64 => "
      CANFD_0_NS.cfdrmdf_8()[5],
    ",
  0x50381714u64 => "
      CANFD_0_NS.cfdrmdf_8()[6],
    ",
  0x50381760u64 => "
      CANFD_0_NS.cfdrmdf_8()[7],
    ",
  0x50381550u64 => "
      CANFD_0_NS.cfdrmdf_9()[0],
    ",
  0x5038159cu64 => "
      CANFD_0_NS.cfdrmdf_9()[1],
    ",
  0x503815e8u64 => "
      CANFD_0_NS.cfdrmdf_9()[2],
    ",
  0x50381634u64 => "
      CANFD_0_NS.cfdrmdf_9()[3],
    ",
  0x50381680u64 => "
      CANFD_0_NS.cfdrmdf_9()[4],
    ",
  0x503816ccu64 => "
      CANFD_0_NS.cfdrmdf_9()[5],
    ",
  0x50381718u64 => "
      CANFD_0_NS.cfdrmdf_9()[6],
    ",
  0x50381764u64 => "
      CANFD_0_NS.cfdrmdf_9()[7],
    ",
  0x50381554u64 => "
      CANFD_0_NS.cfdrmdf_10()[0],
    ",
  0x503815a0u64 => "
      CANFD_0_NS.cfdrmdf_10()[1],
    ",
  0x503815ecu64 => "
      CANFD_0_NS.cfdrmdf_10()[2],
    ",
  0x50381638u64 => "
      CANFD_0_NS.cfdrmdf_10()[3],
    ",
  0x50381684u64 => "
      CANFD_0_NS.cfdrmdf_10()[4],
    ",
  0x503816d0u64 => "
      CANFD_0_NS.cfdrmdf_10()[5],
    ",
  0x5038171cu64 => "
      CANFD_0_NS.cfdrmdf_10()[6],
    ",
  0x50381768u64 => "
      CANFD_0_NS.cfdrmdf_10()[7],
    ",
  0x50381558u64 => "
      CANFD_0_NS.cfdrmdf_11()[0],
    ",
  0x503815a4u64 => "
      CANFD_0_NS.cfdrmdf_11()[1],
    ",
  0x503815f0u64 => "
      CANFD_0_NS.cfdrmdf_11()[2],
    ",
  0x5038163cu64 => "
      CANFD_0_NS.cfdrmdf_11()[3],
    ",
  0x50381688u64 => "
      CANFD_0_NS.cfdrmdf_11()[4],
    ",
  0x503816d4u64 => "
      CANFD_0_NS.cfdrmdf_11()[5],
    ",
  0x50381720u64 => "
      CANFD_0_NS.cfdrmdf_11()[6],
    ",
  0x5038176cu64 => "
      CANFD_0_NS.cfdrmdf_11()[7],
    ",
  0x5038155cu64 => "
      CANFD_0_NS.cfdrmdf_12()[0],
    ",
  0x503815a8u64 => "
      CANFD_0_NS.cfdrmdf_12()[1],
    ",
  0x503815f4u64 => "
      CANFD_0_NS.cfdrmdf_12()[2],
    ",
  0x50381640u64 => "
      CANFD_0_NS.cfdrmdf_12()[3],
    ",
  0x5038168cu64 => "
      CANFD_0_NS.cfdrmdf_12()[4],
    ",
  0x503816d8u64 => "
      CANFD_0_NS.cfdrmdf_12()[5],
    ",
  0x50381724u64 => "
      CANFD_0_NS.cfdrmdf_12()[6],
    ",
  0x50381770u64 => "
      CANFD_0_NS.cfdrmdf_12()[7],
    ",
  0x50381560u64 => "
      CANFD_0_NS.cfdrmdf_13()[0],
    ",
  0x503815acu64 => "
      CANFD_0_NS.cfdrmdf_13()[1],
    ",
  0x503815f8u64 => "
      CANFD_0_NS.cfdrmdf_13()[2],
    ",
  0x50381644u64 => "
      CANFD_0_NS.cfdrmdf_13()[3],
    ",
  0x50381690u64 => "
      CANFD_0_NS.cfdrmdf_13()[4],
    ",
  0x503816dcu64 => "
      CANFD_0_NS.cfdrmdf_13()[5],
    ",
  0x50381728u64 => "
      CANFD_0_NS.cfdrmdf_13()[6],
    ",
  0x50381774u64 => "
      CANFD_0_NS.cfdrmdf_13()[7],
    ",
  0x50381564u64 => "
      CANFD_0_NS.cfdrmdf_14()[0],
    ",
  0x503815b0u64 => "
      CANFD_0_NS.cfdrmdf_14()[1],
    ",
  0x503815fcu64 => "
      CANFD_0_NS.cfdrmdf_14()[2],
    ",
  0x50381648u64 => "
      CANFD_0_NS.cfdrmdf_14()[3],
    ",
  0x50381694u64 => "
      CANFD_0_NS.cfdrmdf_14()[4],
    ",
  0x503816e0u64 => "
      CANFD_0_NS.cfdrmdf_14()[5],
    ",
  0x5038172cu64 => "
      CANFD_0_NS.cfdrmdf_14()[6],
    ",
  0x50381778u64 => "
      CANFD_0_NS.cfdrmdf_14()[7],
    ",
  0x50381568u64 => "
      CANFD_0_NS.cfdrmdf_15()[0],
    ",
  0x503815b4u64 => "
      CANFD_0_NS.cfdrmdf_15()[1],
    ",
  0x50381600u64 => "
      CANFD_0_NS.cfdrmdf_15()[2],
    ",
  0x5038164cu64 => "
      CANFD_0_NS.cfdrmdf_15()[3],
    ",
  0x50381698u64 => "
      CANFD_0_NS.cfdrmdf_15()[4],
    ",
  0x503816e4u64 => "
      CANFD_0_NS.cfdrmdf_15()[5],
    ",
  0x50381730u64 => "
      CANFD_0_NS.cfdrmdf_15()[6],
    ",
  0x5038177cu64 => "
      CANFD_0_NS.cfdrmdf_15()[7],
    ",
  0x50400000u64 => "
      PORT_0_NS.pcntr1(),
      PORT_0_NS.pdr(),
    ",
  0x50400002u64 => "
      PORT_0_NS.podr(),
    ",
  0x50400004u64 => "
      PORT_0_NS.pcntr2(),
      PORT_0_NS.pidr(),
    ",
  0x50400008u64 => "
      PORT_0_NS.pcntr3(),
      PORT_0_NS.posr(),
    ",
  0x5040000au64 => "
      PORT_0_NS.porr(),
    ",
  0x50400020u64 => "
      PORT_1_NS.pcntr1(),
      PORT_1_NS.pdr(),
    ",
  0x50400022u64 => "
      PORT_1_NS.podr(),
    ",
  0x50400024u64 => "
      PORT_1_NS.pcntr2(),
      PORT_1_NS.pidr(),
    ",
  0x50400026u64 => "
      PORT_1_NS.eidr(),
    ",
  0x50400028u64 => "
      PORT_1_NS.pcntr3(),
      PORT_1_NS.posr(),
    ",
  0x5040002au64 => "
      PORT_1_NS.porr(),
    ",
  0x5040002cu64 => "
      PORT_1_NS.pcntr4(),
      PORT_1_NS.eosr(),
    ",
  0x5040002eu64 => "
      PORT_1_NS.eorr(),
    ",
  0x50400144u64 => "
      PORTA_NS.pcntr2(),
      PORTA_NS.pidr(),
    ",
  0x50400148u64 => "
      PORTA_NS.pcntr3(),
      PORTA_NS.posr(),
    ",
  0x5040014au64 => "
      PORTA_NS.porr(),
    ",
  0x50400164u64 => "
      PORTB_NS.pcntr2(),
      PORTB_NS.pidr(),
    ",
  0x50400168u64 => "
      PORTB_NS.pcntr3(),
      PORTB_NS.posr(),
    ",
  0x5040016au64 => "
      PORTB_NS.porr(),
    ",
  0x50400800u64 => "
      PFS_NS.p00pfs()[0],
      PFS_NS.p00pfs_ha()[0],
      PFS_NS.p00pfs_by()[0],
    ",
  0x50400804u64 => "
      PFS_NS.p00pfs()[1],
      PFS_NS.p00pfs_ha()[1],
      PFS_NS.p00pfs_by()[1],
    ",
  0x50400808u64 => "
      PFS_NS.p00pfs()[2],
      PFS_NS.p00pfs_ha()[2],
      PFS_NS.p00pfs_by()[2],
    ",
  0x5040080cu64 => "
      PFS_NS.p00pfs()[3],
      PFS_NS.p00pfs_ha()[3],
      PFS_NS.p00pfs_by()[3],
    ",
  0x50400810u64 => "
      PFS_NS.p00pfs()[4],
      PFS_NS.p00pfs_ha()[4],
      PFS_NS.p00pfs_by()[4],
    ",
  0x50400814u64 => "
      PFS_NS.p00pfs()[5],
      PFS_NS.p00pfs_ha()[5],
      PFS_NS.p00pfs_by()[5],
    ",
  0x50400818u64 => "
      PFS_NS.p00pfs()[6],
      PFS_NS.p00pfs_ha()[6],
      PFS_NS.p00pfs_by()[6],
    ",
  0x5040081cu64 => "
      PFS_NS.p00pfs()[7],
      PFS_NS.p00pfs_ha()[7],
      PFS_NS.p00pfs_by()[7],
    ",
  0x50400820u64 => "
      PFS_NS.p00pfs()[8],
      PFS_NS.p00pfs_ha()[8],
      PFS_NS.p00pfs_by()[8],
    ",
  0x50400824u64 => "
      PFS_NS.p00pfs()[9],
      PFS_NS.p00pfs_ha()[9],
      PFS_NS.p00pfs_by()[9],
    ",
  0x50400838u64 => "
      PFS_NS.p0pfs()[0],
      PFS_NS.p0pfs_ha()[0],
      PFS_NS.p0pfs_by()[0],
    ",
  0x5040083cu64 => "
      PFS_NS.p0pfs()[1],
      PFS_NS.p0pfs_ha()[1],
      PFS_NS.p0pfs_by()[1],
    ",
  0x50400840u64 => "
      PFS_NS.p10pfs()[0],
      PFS_NS.p10pfs_ha()[0],
      PFS_NS.p10pfs_by()[0],
    ",
  0x50400844u64 => "
      PFS_NS.p10pfs()[1],
      PFS_NS.p10pfs_ha()[1],
      PFS_NS.p10pfs_by()[1],
    ",
  0x50400848u64 => "
      PFS_NS.p10pfs()[2],
      PFS_NS.p10pfs_ha()[2],
      PFS_NS.p10pfs_by()[2],
    ",
  0x5040084cu64 => "
      PFS_NS.p10pfs()[3],
      PFS_NS.p10pfs_ha()[3],
      PFS_NS.p10pfs_by()[3],
    ",
  0x50400850u64 => "
      PFS_NS.p10pfs()[4],
      PFS_NS.p10pfs_ha()[4],
      PFS_NS.p10pfs_by()[4],
    ",
  0x50400854u64 => "
      PFS_NS.p10pfs()[5],
      PFS_NS.p10pfs_ha()[5],
      PFS_NS.p10pfs_by()[5],
    ",
  0x50400858u64 => "
      PFS_NS.p10pfs()[6],
      PFS_NS.p10pfs_ha()[6],
      PFS_NS.p10pfs_by()[6],
    ",
  0x5040085cu64 => "
      PFS_NS.p10pfs()[7],
      PFS_NS.p10pfs_ha()[7],
      PFS_NS.p10pfs_by()[7],
    ",
  0x50400870u64 => "
      PFS_NS.p1pfs()[0],
      PFS_NS.p1pfs_ha()[0],
      PFS_NS.p1pfs_by()[0],
    ",
  0x50400874u64 => "
      PFS_NS.p1pfs()[1],
      PFS_NS.p1pfs_ha()[1],
      PFS_NS.p1pfs_by()[1],
    ",
  0x50400878u64 => "
      PFS_NS.p1pfs()[2],
      PFS_NS.p1pfs_ha()[2],
      PFS_NS.p1pfs_by()[2],
    ",
  0x5040087cu64 => "
      PFS_NS.p1pfs()[3],
      PFS_NS.p1pfs_ha()[3],
      PFS_NS.p1pfs_by()[3],
    ",
  0x50400880u64 => "
      PFS_NS.p200pfs(),
      PFS_NS.p200pfs_ha(),
      PFS_NS.p200pfs_by(),
    ",
  0x50400884u64 => "
      PFS_NS.p201pfs(),
      PFS_NS.p201pfs_ha(),
      PFS_NS.p201pfs_by(),
    ",
  0x50400888u64 => "
      PFS_NS.p20pfs()[0],
      PFS_NS.p20pfs_ha()[0],
      PFS_NS.p20pfs_by()[0],
    ",
  0x5040088cu64 => "
      PFS_NS.p20pfs()[1],
      PFS_NS.p20pfs_ha()[1],
      PFS_NS.p20pfs_by()[1],
    ",
  0x50400890u64 => "
      PFS_NS.p20pfs()[2],
      PFS_NS.p20pfs_ha()[2],
      PFS_NS.p20pfs_by()[2],
    ",
  0x50400894u64 => "
      PFS_NS.p20pfs()[3],
      PFS_NS.p20pfs_ha()[3],
      PFS_NS.p20pfs_by()[3],
    ",
  0x50400898u64 => "
      PFS_NS.p20pfs()[4],
      PFS_NS.p20pfs_ha()[4],
      PFS_NS.p20pfs_by()[4],
    ",
  0x5040089cu64 => "
      PFS_NS.p20pfs()[5],
      PFS_NS.p20pfs_ha()[5],
      PFS_NS.p20pfs_by()[5],
    ",
  0x504008a0u64 => "
      PFS_NS.p208pfs(),
      PFS_NS.p208pfs_ha(),
      PFS_NS.p208pfs_by(),
    ",
  0x504008a4u64 => "
      PFS_NS.p209pfs(),
      PFS_NS.p209pfs_ha(),
      PFS_NS.p209pfs_by(),
    ",
  0x504008a8u64 => "
      PFS_NS.p210pfs(),
      PFS_NS.p210pfs_ha(),
      PFS_NS.p210pfs_by(),
    ",
  0x504008acu64 => "
      PFS_NS.p211pfs(),
      PFS_NS.p211pfs_ha(),
      PFS_NS.p211pfs_by(),
    ",
  0x504008b0u64 => "
      PFS_NS.p2pfs()[0],
      PFS_NS.p2pfs_ha()[0],
      PFS_NS.p2pfs_by()[0],
    ",
  0x504008b4u64 => "
      PFS_NS.p2pfs()[1],
      PFS_NS.p2pfs_ha()[1],
      PFS_NS.p2pfs_by()[1],
    ",
  0x504008c0u64 => "
      PFS_NS.p30pfs()[0],
      PFS_NS.p30pfs_ha()[0],
      PFS_NS.p30pfs_by()[0],
    ",
  0x504008c4u64 => "
      PFS_NS.p30pfs()[1],
      PFS_NS.p30pfs_ha()[1],
      PFS_NS.p30pfs_by()[1],
    ",
  0x504008c8u64 => "
      PFS_NS.p30pfs()[2],
      PFS_NS.p30pfs_ha()[2],
      PFS_NS.p30pfs_by()[2],
    ",
  0x504008ccu64 => "
      PFS_NS.p30pfs()[3],
      PFS_NS.p30pfs_ha()[3],
      PFS_NS.p30pfs_by()[3],
    ",
  0x504008d0u64 => "
      PFS_NS.p30pfs()[4],
      PFS_NS.p30pfs_ha()[4],
      PFS_NS.p30pfs_by()[4],
    ",
  0x504008d4u64 => "
      PFS_NS.p30pfs()[5],
      PFS_NS.p30pfs_ha()[5],
      PFS_NS.p30pfs_by()[5],
    ",
  0x504008d8u64 => "
      PFS_NS.p30pfs()[6],
      PFS_NS.p30pfs_ha()[6],
      PFS_NS.p30pfs_by()[6],
    ",
  0x504008dcu64 => "
      PFS_NS.p30pfs()[7],
      PFS_NS.p30pfs_ha()[7],
      PFS_NS.p30pfs_by()[7],
    ",
  0x504008e0u64 => "
      PFS_NS.p30pfs()[8],
      PFS_NS.p30pfs_ha()[8],
      PFS_NS.p30pfs_by()[8],
    ",
  0x504008e4u64 => "
      PFS_NS.p30pfs()[9],
      PFS_NS.p30pfs_ha()[9],
      PFS_NS.p30pfs_by()[9],
    ",
  0x504008e8u64 => "
      PFS_NS.p3pfs()[0],
      PFS_NS.p3pfs_ha()[0],
      PFS_NS.p3pfs_by()[0],
    ",
  0x504008ecu64 => "
      PFS_NS.p3pfs()[1],
      PFS_NS.p3pfs_ha()[1],
      PFS_NS.p3pfs_by()[1],
    ",
  0x504008f0u64 => "
      PFS_NS.p3pfs()[2],
      PFS_NS.p3pfs_ha()[2],
      PFS_NS.p3pfs_by()[2],
    ",
  0x504008f4u64 => "
      PFS_NS.p3pfs()[3],
      PFS_NS.p3pfs_ha()[3],
      PFS_NS.p3pfs_by()[3],
    ",
  0x504008f8u64 => "
      PFS_NS.p3pfs()[4],
      PFS_NS.p3pfs_ha()[4],
      PFS_NS.p3pfs_by()[4],
    ",
  0x504008fcu64 => "
      PFS_NS.p3pfs()[5],
      PFS_NS.p3pfs_ha()[5],
      PFS_NS.p3pfs_by()[5],
    ",
  0x50400900u64 => "
      PFS_NS.p40pfs()[0],
      PFS_NS.p40pfs_ha()[0],
      PFS_NS.p40pfs_by()[0],
    ",
  0x50400904u64 => "
      PFS_NS.p40pfs()[1],
      PFS_NS.p40pfs_ha()[1],
      PFS_NS.p40pfs_by()[1],
    ",
  0x50400908u64 => "
      PFS_NS.p40pfs()[2],
      PFS_NS.p40pfs_ha()[2],
      PFS_NS.p40pfs_by()[2],
    ",
  0x5040090cu64 => "
      PFS_NS.p40pfs()[3],
      PFS_NS.p40pfs_ha()[3],
      PFS_NS.p40pfs_by()[3],
    ",
  0x50400910u64 => "
      PFS_NS.p40pfs()[4],
      PFS_NS.p40pfs_ha()[4],
      PFS_NS.p40pfs_by()[4],
    ",
  0x50400914u64 => "
      PFS_NS.p40pfs()[5],
      PFS_NS.p40pfs_ha()[5],
      PFS_NS.p40pfs_by()[5],
    ",
  0x50400918u64 => "
      PFS_NS.p40pfs()[6],
      PFS_NS.p40pfs_ha()[6],
      PFS_NS.p40pfs_by()[6],
    ",
  0x5040091cu64 => "
      PFS_NS.p40pfs()[7],
      PFS_NS.p40pfs_ha()[7],
      PFS_NS.p40pfs_by()[7],
    ",
  0x50400920u64 => "
      PFS_NS.p40pfs()[8],
      PFS_NS.p40pfs_ha()[8],
      PFS_NS.p40pfs_by()[8],
    ",
  0x50400924u64 => "
      PFS_NS.p40pfs()[9],
      PFS_NS.p40pfs_ha()[9],
      PFS_NS.p40pfs_by()[9],
    ",
  0x50400928u64 => "
      PFS_NS.p4pfs()[0],
      PFS_NS.p4pfs_ha()[0],
      PFS_NS.p4pfs_by()[0],
    ",
  0x5040092cu64 => "
      PFS_NS.p4pfs()[1],
      PFS_NS.p4pfs_ha()[1],
      PFS_NS.p4pfs_by()[1],
    ",
  0x50400930u64 => "
      PFS_NS.p4pfs()[2],
      PFS_NS.p4pfs_ha()[2],
      PFS_NS.p4pfs_by()[2],
    ",
  0x50400934u64 => "
      PFS_NS.p4pfs()[3],
      PFS_NS.p4pfs_ha()[3],
      PFS_NS.p4pfs_by()[3],
    ",
  0x50400938u64 => "
      PFS_NS.p4pfs()[4],
      PFS_NS.p4pfs_ha()[4],
      PFS_NS.p4pfs_by()[4],
    ",
  0x5040093cu64 => "
      PFS_NS.p4pfs()[5],
      PFS_NS.p4pfs_ha()[5],
      PFS_NS.p4pfs_by()[5],
    ",
  0x50400940u64 => "
      PFS_NS.p50pfs()[0],
      PFS_NS.p50pfs_ha()[0],
      PFS_NS.p50pfs_by()[0],
    ",
  0x50400944u64 => "
      PFS_NS.p50pfs()[1],
      PFS_NS.p50pfs_ha()[1],
      PFS_NS.p50pfs_by()[1],
    ",
  0x50400948u64 => "
      PFS_NS.p50pfs()[2],
      PFS_NS.p50pfs_ha()[2],
      PFS_NS.p50pfs_by()[2],
    ",
  0x5040094cu64 => "
      PFS_NS.p50pfs()[3],
      PFS_NS.p50pfs_ha()[3],
      PFS_NS.p50pfs_by()[3],
    ",
  0x50400950u64 => "
      PFS_NS.p50pfs()[4],
      PFS_NS.p50pfs_ha()[4],
      PFS_NS.p50pfs_by()[4],
    ",
  0x50400954u64 => "
      PFS_NS.p50pfs()[5],
      PFS_NS.p50pfs_ha()[5],
      PFS_NS.p50pfs_by()[5],
    ",
  0x50400958u64 => "
      PFS_NS.p50pfs()[6],
      PFS_NS.p50pfs_ha()[6],
      PFS_NS.p50pfs_by()[6],
    ",
  0x5040095cu64 => "
      PFS_NS.p50pfs()[7],
      PFS_NS.p50pfs_ha()[7],
      PFS_NS.p50pfs_by()[7],
    ",
  0x50400960u64 => "
      PFS_NS.p50pfs()[8],
      PFS_NS.p50pfs_ha()[8],
      PFS_NS.p50pfs_by()[8],
    ",
  0x50400964u64 => "
      PFS_NS.p50pfs()[9],
      PFS_NS.p50pfs_ha()[9],
      PFS_NS.p50pfs_by()[9],
    ",
  0x50400968u64 => "
      PFS_NS.p5pfs()[0],
      PFS_NS.p5pfs_ha()[0],
      PFS_NS.p5pfs_by()[0],
    ",
  0x5040096cu64 => "
      PFS_NS.p5pfs()[1],
      PFS_NS.p5pfs_ha()[1],
      PFS_NS.p5pfs_by()[1],
    ",
  0x50400970u64 => "
      PFS_NS.p5pfs()[2],
      PFS_NS.p5pfs_ha()[2],
      PFS_NS.p5pfs_by()[2],
    ",
  0x50400974u64 => "
      PFS_NS.p5pfs()[3],
      PFS_NS.p5pfs_ha()[3],
      PFS_NS.p5pfs_by()[3],
    ",
  0x50400978u64 => "
      PFS_NS.p5pfs()[4],
      PFS_NS.p5pfs_ha()[4],
      PFS_NS.p5pfs_by()[4],
    ",
  0x5040097cu64 => "
      PFS_NS.p5pfs()[5],
      PFS_NS.p5pfs_ha()[5],
      PFS_NS.p5pfs_by()[5],
    ",
  0x50400980u64 => "
      PFS_NS.p60pfs()[0],
      PFS_NS.p60pfs_ha()[0],
      PFS_NS.p60pfs_by()[0],
    ",
  0x50400984u64 => "
      PFS_NS.p60pfs()[1],
      PFS_NS.p60pfs_ha()[1],
      PFS_NS.p60pfs_by()[1],
    ",
  0x50400988u64 => "
      PFS_NS.p60pfs()[2],
      PFS_NS.p60pfs_ha()[2],
      PFS_NS.p60pfs_by()[2],
    ",
  0x5040098cu64 => "
      PFS_NS.p60pfs()[3],
      PFS_NS.p60pfs_ha()[3],
      PFS_NS.p60pfs_by()[3],
    ",
  0x50400990u64 => "
      PFS_NS.p60pfs()[4],
      PFS_NS.p60pfs_ha()[4],
      PFS_NS.p60pfs_by()[4],
    ",
  0x50400994u64 => "
      PFS_NS.p60pfs()[5],
      PFS_NS.p60pfs_ha()[5],
      PFS_NS.p60pfs_by()[5],
    ",
  0x50400998u64 => "
      PFS_NS.p60pfs()[6],
      PFS_NS.p60pfs_ha()[6],
      PFS_NS.p60pfs_by()[6],
    ",
  0x5040099cu64 => "
      PFS_NS.p60pfs()[7],
      PFS_NS.p60pfs_ha()[7],
      PFS_NS.p60pfs_by()[7],
    ",
  0x504009a0u64 => "
      PFS_NS.p60pfs()[8],
      PFS_NS.p60pfs_ha()[8],
      PFS_NS.p60pfs_by()[8],
    ",
  0x504009a4u64 => "
      PFS_NS.p60pfs()[9],
      PFS_NS.p60pfs_ha()[9],
      PFS_NS.p60pfs_by()[9],
    ",
  0x504009a8u64 => "
      PFS_NS.p6pfs()[0],
      PFS_NS.p6pfs_ha()[0],
      PFS_NS.p6pfs_by()[0],
    ",
  0x504009acu64 => "
      PFS_NS.p6pfs()[1],
      PFS_NS.p6pfs_ha()[1],
      PFS_NS.p6pfs_by()[1],
    ",
  0x504009b0u64 => "
      PFS_NS.p6pfs()[2],
      PFS_NS.p6pfs_ha()[2],
      PFS_NS.p6pfs_by()[2],
    ",
  0x504009b4u64 => "
      PFS_NS.p6pfs()[3],
      PFS_NS.p6pfs_ha()[3],
      PFS_NS.p6pfs_by()[3],
    ",
  0x504009b8u64 => "
      PFS_NS.p6pfs()[4],
      PFS_NS.p6pfs_ha()[4],
      PFS_NS.p6pfs_by()[4],
    ",
  0x504009bcu64 => "
      PFS_NS.p6pfs()[5],
      PFS_NS.p6pfs_ha()[5],
      PFS_NS.p6pfs_by()[5],
    ",
  0x504009c0u64 => "
      PFS_NS.p70pfs()[0],
      PFS_NS.p70pfs_ha()[0],
      PFS_NS.p70pfs_by()[0],
    ",
  0x504009c4u64 => "
      PFS_NS.p70pfs()[1],
      PFS_NS.p70pfs_ha()[1],
      PFS_NS.p70pfs_by()[1],
    ",
  0x504009c8u64 => "
      PFS_NS.p70pfs()[2],
      PFS_NS.p70pfs_ha()[2],
      PFS_NS.p70pfs_by()[2],
    ",
  0x504009ccu64 => "
      PFS_NS.p70pfs()[3],
      PFS_NS.p70pfs_ha()[3],
      PFS_NS.p70pfs_by()[3],
    ",
  0x504009d0u64 => "
      PFS_NS.p70pfs()[4],
      PFS_NS.p70pfs_ha()[4],
      PFS_NS.p70pfs_by()[4],
    ",
  0x504009d4u64 => "
      PFS_NS.p70pfs()[5],
      PFS_NS.p70pfs_ha()[5],
      PFS_NS.p70pfs_by()[5],
    ",
  0x504009d8u64 => "
      PFS_NS.p70pfs()[6],
      PFS_NS.p70pfs_ha()[6],
      PFS_NS.p70pfs_by()[6],
    ",
  0x504009dcu64 => "
      PFS_NS.p70pfs()[7],
      PFS_NS.p70pfs_ha()[7],
      PFS_NS.p70pfs_by()[7],
    ",
  0x504009e0u64 => "
      PFS_NS.p70pfs()[8],
      PFS_NS.p70pfs_ha()[8],
      PFS_NS.p70pfs_by()[8],
    ",
  0x504009e4u64 => "
      PFS_NS.p70pfs()[9],
      PFS_NS.p70pfs_ha()[9],
      PFS_NS.p70pfs_by()[9],
    ",
  0x504009e8u64 => "
      PFS_NS.p7pfs()[0],
      PFS_NS.p7pfs_ha()[0],
      PFS_NS.p7pfs_by()[0],
    ",
  0x504009ecu64 => "
      PFS_NS.p7pfs()[1],
      PFS_NS.p7pfs_ha()[1],
      PFS_NS.p7pfs_by()[1],
    ",
  0x504009f0u64 => "
      PFS_NS.p7pfs()[2],
      PFS_NS.p7pfs_ha()[2],
      PFS_NS.p7pfs_by()[2],
    ",
  0x504009f4u64 => "
      PFS_NS.p7pfs()[3],
      PFS_NS.p7pfs_ha()[3],
      PFS_NS.p7pfs_by()[3],
    ",
  0x504009f8u64 => "
      PFS_NS.p7pfs()[4],
      PFS_NS.p7pfs_ha()[4],
      PFS_NS.p7pfs_by()[4],
    ",
  0x504009fcu64 => "
      PFS_NS.p7pfs()[5],
      PFS_NS.p7pfs_ha()[5],
      PFS_NS.p7pfs_by()[5],
    ",
  0x50400a00u64 => "
      PFS_NS.p80pfs()[0],
      PFS_NS.p80pfs_ha()[0],
      PFS_NS.p80pfs_by()[0],
    ",
  0x50400a04u64 => "
      PFS_NS.p80pfs()[1],
      PFS_NS.p80pfs_ha()[1],
      PFS_NS.p80pfs_by()[1],
    ",
  0x50400a08u64 => "
      PFS_NS.p80pfs()[2],
      PFS_NS.p80pfs_ha()[2],
      PFS_NS.p80pfs_by()[2],
    ",
  0x50400a0cu64 => "
      PFS_NS.p80pfs()[3],
      PFS_NS.p80pfs_ha()[3],
      PFS_NS.p80pfs_by()[3],
    ",
  0x50400a10u64 => "
      PFS_NS.p80pfs()[4],
      PFS_NS.p80pfs_ha()[4],
      PFS_NS.p80pfs_by()[4],
    ",
  0x50400a14u64 => "
      PFS_NS.p80pfs()[5],
      PFS_NS.p80pfs_ha()[5],
      PFS_NS.p80pfs_by()[5],
    ",
  0x50400a18u64 => "
      PFS_NS.p80pfs()[6],
      PFS_NS.p80pfs_ha()[6],
      PFS_NS.p80pfs_by()[6],
    ",
  0x50400a1cu64 => "
      PFS_NS.p80pfs()[7],
      PFS_NS.p80pfs_ha()[7],
      PFS_NS.p80pfs_by()[7],
    ",
  0x50400a20u64 => "
      PFS_NS.p80pfs()[8],
      PFS_NS.p80pfs_ha()[8],
      PFS_NS.p80pfs_by()[8],
    ",
  0x50400a24u64 => "
      PFS_NS.p80pfs()[9],
      PFS_NS.p80pfs_ha()[9],
      PFS_NS.p80pfs_by()[9],
    ",
  0x50400a30u64 => "
      PFS_NS.p8pfs()[2],
      PFS_NS.p8pfs_ha()[2],
      PFS_NS.p8pfs_by()[2],
    ",
  0x50400a34u64 => "
      PFS_NS.p8pfs()[3],
      PFS_NS.p8pfs_ha()[3],
      PFS_NS.p8pfs_by()[3],
    ",
  0x50400a38u64 => "
      PFS_NS.p8pfs()[0],
      PFS_NS.p8pfs_ha()[0],
      PFS_NS.p8pfs_by()[0],
    ",
  0x50400a3cu64 => "
      PFS_NS.p8pfs()[1],
      PFS_NS.p8pfs_ha()[1],
      PFS_NS.p8pfs_by()[1],
    ",
  0x50400a40u64 => "
      PFS_NS.p90pfs()[0],
      PFS_NS.p90pfs_ha()[0],
      PFS_NS.p90pfs_by()[0],
    ",
  0x50400a44u64 => "
      PFS_NS.p90pfs()[1],
      PFS_NS.p90pfs_ha()[1],
      PFS_NS.p90pfs_by()[1],
    ",
  0x50400a48u64 => "
      PFS_NS.p90pfs()[2],
      PFS_NS.p90pfs_ha()[2],
      PFS_NS.p90pfs_by()[2],
    ",
  0x50400a4cu64 => "
      PFS_NS.p90pfs()[3],
      PFS_NS.p90pfs_ha()[3],
      PFS_NS.p90pfs_by()[3],
    ",
  0x50400a50u64 => "
      PFS_NS.p90pfs()[4],
      PFS_NS.p90pfs_ha()[4],
      PFS_NS.p90pfs_by()[4],
    ",
  0x50400a54u64 => "
      PFS_NS.p90pfs()[5],
      PFS_NS.p90pfs_ha()[5],
      PFS_NS.p90pfs_by()[5],
    ",
  0x50400a58u64 => "
      PFS_NS.p90pfs()[6],
      PFS_NS.p90pfs_ha()[6],
      PFS_NS.p90pfs_by()[6],
    ",
  0x50400a5cu64 => "
      PFS_NS.p90pfs()[7],
      PFS_NS.p90pfs_ha()[7],
      PFS_NS.p90pfs_by()[7],
    ",
  0x50400a60u64 => "
      PFS_NS.p90pfs()[8],
      PFS_NS.p90pfs_ha()[8],
      PFS_NS.p90pfs_by()[8],
    ",
  0x50400a64u64 => "
      PFS_NS.p90pfs()[9],
      PFS_NS.p90pfs_ha()[9],
      PFS_NS.p90pfs_by()[9],
    ",
  0x50400a68u64 => "
      PFS_NS.p9pfs()[0],
      PFS_NS.p9pfs_ha()[0],
      PFS_NS.p9pfs_by()[0],
    ",
  0x50400a6cu64 => "
      PFS_NS.p9pfs()[1],
      PFS_NS.p9pfs_ha()[1],
      PFS_NS.p9pfs_by()[1],
    ",
  0x50400a70u64 => "
      PFS_NS.p9pfs()[2],
      PFS_NS.p9pfs_ha()[2],
      PFS_NS.p9pfs_by()[2],
    ",
  0x50400a74u64 => "
      PFS_NS.p9pfs()[3],
      PFS_NS.p9pfs_ha()[3],
      PFS_NS.p9pfs_by()[3],
    ",
  0x50400a78u64 => "
      PFS_NS.p9pfs()[4],
      PFS_NS.p9pfs_ha()[4],
      PFS_NS.p9pfs_by()[4],
    ",
  0x50400a7cu64 => "
      PFS_NS.p9pfs()[5],
      PFS_NS.p9pfs_ha()[5],
      PFS_NS.p9pfs_by()[5],
    ",
  0x50400a80u64 => "
      PFS_NS.pa0pfs()[0],
      PFS_NS.pa0pfs_ha()[0],
      PFS_NS.pa0pfs_by()[0],
    ",
  0x50400a84u64 => "
      PFS_NS.pa0pfs()[1],
      PFS_NS.pa0pfs_ha()[1],
      PFS_NS.pa0pfs_by()[1],
    ",
  0x50400a88u64 => "
      PFS_NS.pa0pfs()[2],
      PFS_NS.pa0pfs_ha()[2],
      PFS_NS.pa0pfs_by()[2],
    ",
  0x50400a8cu64 => "
      PFS_NS.pa0pfs()[3],
      PFS_NS.pa0pfs_ha()[3],
      PFS_NS.pa0pfs_by()[3],
    ",
  0x50400a90u64 => "
      PFS_NS.pa0pfs()[4],
      PFS_NS.pa0pfs_ha()[4],
      PFS_NS.pa0pfs_by()[4],
    ",
  0x50400a94u64 => "
      PFS_NS.pa0pfs()[5],
      PFS_NS.pa0pfs_ha()[5],
      PFS_NS.pa0pfs_by()[5],
    ",
  0x50400a98u64 => "
      PFS_NS.pa0pfs()[6],
      PFS_NS.pa0pfs_ha()[6],
      PFS_NS.pa0pfs_by()[6],
    ",
  0x50400a9cu64 => "
      PFS_NS.pa0pfs()[7],
      PFS_NS.pa0pfs_ha()[7],
      PFS_NS.pa0pfs_by()[7],
    ",
  0x50400aa0u64 => "
      PFS_NS.pa0pfs()[8],
      PFS_NS.pa0pfs_ha()[8],
      PFS_NS.pa0pfs_by()[8],
    ",
  0x50400aa4u64 => "
      PFS_NS.pa0pfs()[9],
      PFS_NS.pa0pfs_ha()[9],
      PFS_NS.pa0pfs_by()[9],
    ",
  0x50400aa8u64 => "
      PFS_NS.papfs()[0],
      PFS_NS.papfs_ha()[0],
      PFS_NS.papfs_by()[0],
    ",
  0x50400aacu64 => "
      PFS_NS.papfs()[1],
      PFS_NS.papfs_ha()[1],
      PFS_NS.papfs_by()[1],
    ",
  0x50400ab0u64 => "
      PFS_NS.papfs()[2],
      PFS_NS.papfs_ha()[2],
      PFS_NS.papfs_by()[2],
    ",
  0x50400ab4u64 => "
      PFS_NS.papfs()[3],
      PFS_NS.papfs_ha()[3],
      PFS_NS.papfs_by()[3],
    ",
  0x50400ab8u64 => "
      PFS_NS.papfs()[4],
      PFS_NS.papfs_ha()[4],
      PFS_NS.papfs_by()[4],
    ",
  0x50400abcu64 => "
      PFS_NS.papfs()[5],
      PFS_NS.papfs_ha()[5],
      PFS_NS.papfs_by()[5],
    ",
  0x50400ac0u64 => "
      PFS_NS.pb0pfs()[0],
      PFS_NS.pb0pfs_ha()[0],
      PFS_NS.pb0pfs_by()[0],
    ",
  0x50400ac4u64 => "
      PFS_NS.pb0pfs()[1],
      PFS_NS.pb0pfs_ha()[1],
      PFS_NS.pb0pfs_by()[1],
    ",
  0x50400ac8u64 => "
      PFS_NS.pb0pfs()[2],
      PFS_NS.pb0pfs_ha()[2],
      PFS_NS.pb0pfs_by()[2],
    ",
  0x50400accu64 => "
      PFS_NS.pb0pfs()[3],
      PFS_NS.pb0pfs_ha()[3],
      PFS_NS.pb0pfs_by()[3],
    ",
  0x50400ad0u64 => "
      PFS_NS.pb0pfs()[4],
      PFS_NS.pb0pfs_ha()[4],
      PFS_NS.pb0pfs_by()[4],
    ",
  0x50400ad4u64 => "
      PFS_NS.pb0pfs()[5],
      PFS_NS.pb0pfs_ha()[5],
      PFS_NS.pb0pfs_by()[5],
    ",
  0x50400ad8u64 => "
      PFS_NS.pb0pfs()[6],
      PFS_NS.pb0pfs_ha()[6],
      PFS_NS.pb0pfs_by()[6],
    ",
  0x50400adcu64 => "
      PFS_NS.pb0pfs()[7],
      PFS_NS.pb0pfs_ha()[7],
      PFS_NS.pb0pfs_by()[7],
    ",
  0x50400d00u64 => "
      PFS_NS.pfenet(),
    ",
  0x50400d0cu64 => "
      PFS_NS.pwpr_ns(),
    ",
  0x50400d38u64 => "
      PFS_NS.psar()[2],
    ",
  0x50400d3cu64 => "
      PFS_NS.psar()[3],
    ",
  0x50400d40u64 => "
      PFS_NS.psar()[4],
    ",
  0x50400d44u64 => "
      PFS_NS.psar()[5],
    ",
  0x50400d48u64 => "
      PFS_NS.psar()[6],
    ",
  0x50400d4cu64 => "
      PFS_NS.psar()[7],
    ",
  0x50400d50u64 => "
      PFS_NS.psar()[8],
    ",
  0x50400d54u64 => "
      PFS_NS.psar()[9],
    ",
  0x50400d58u64 => "
      PFS_NS.psar()[0],
    ",
  0x50400d5cu64 => "
      PFS_NS.psar()[1],
    ",
};
