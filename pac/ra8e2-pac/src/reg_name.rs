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
// Generated from SVD 1.00.01, with svd2pac 0.6.1 on Sun, 15 Mar 2026 06:40:06 +0000

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
  0x40000700u64 => "
      RMPU.mmpuenglcdc(),
    ",
  0x40000704u64 => "
      RMPU.mmpuenptglcdc(),
    ",
  0x40000708u64 => "
      RMPU.mmpurptglcdc(),
    ",
  0x40000800u64 => "
      RMPU.mmpuacglcdc()[0],
    ",
  0x40000810u64 => "
      RMPU.mmpuacglcdc()[1],
    ",
  0x40000804u64 => "
      RMPU.mmpusglcdc()[0],
    ",
  0x40000814u64 => "
      RMPU.mmpusglcdc()[1],
    ",
  0x40000808u64 => "
      RMPU.mmpueglcdc()[0],
    ",
  0x40000818u64 => "
      RMPU.mmpueglcdc()[1],
    ",
  0x40000900u64 => "
      RMPU.mmpuendrw(),
    ",
  0x40000904u64 => "
      RMPU.mmpuenpdrw(),
    ",
  0x40000908u64 => "
      RMPU.mmpurptdrw(),
    ",
  0x40000a00u64 => "
      RMPU.mmpuacdrw()[0],
    ",
  0x40000a10u64 => "
      RMPU.mmpuacdrw()[1],
    ",
  0x40000a20u64 => "
      RMPU.mmpuacdrw()[2],
    ",
  0x40000a04u64 => "
      RMPU.mmpusdrw()[0],
    ",
  0x40000a14u64 => "
      RMPU.mmpusdrw()[1],
    ",
  0x40000a24u64 => "
      RMPU.mmpusdrw()[2],
    ",
  0x40000a08u64 => "
      RMPU.mmpuedrw()[0],
    ",
  0x40000a18u64 => "
      RMPU.mmpuedrw()[1],
    ",
  0x40000a28u64 => "
      RMPU.mmpuedrw()[2],
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
      BUS.cs0cr(),
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
  0x40003880u64 => "
      BUS.csrecen(),
    ",
  0x40003c00u64 => "
      BUS.sdccr(),
    ",
  0x40003c01u64 => "
      BUS.sdcmod(),
    ",
  0x40003c02u64 => "
      BUS.sdamod(),
    ",
  0x40003c10u64 => "
      BUS.sdself(),
    ",
  0x40003c14u64 => "
      BUS.sdrfcr(),
    ",
  0x40003c16u64 => "
      BUS.sdrfen(),
    ",
  0x40003c20u64 => "
      BUS.sdicr(),
    ",
  0x40003c24u64 => "
      BUS.sdir(),
    ",
  0x40003c40u64 => "
      BUS.sdadr(),
    ",
  0x40003c44u64 => "
      BUS.sdtr(),
    ",
  0x40003c48u64 => "
      BUS.sdmod(),
    ",
  0x40003c50u64 => "
      BUS.sdsr(),
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
  0x40004100u64 => "
      BUS.busmabt(),
    ",
  0x40004200u64 => "
      BUS.bussabt1fhbi(),
    ",
  0x40004210u64 => "
      BUS.bussabt0flbi(),
    ",
  0x40004218u64 => "
      BUS.bussabt1s0bi(),
    ",
  0x40004220u64 => "
      BUS.bussabt1s1bi(),
    ",
  0x40004248u64 => "
      BUS.bussabt0stbysbi(),
    ",
  0x40004250u64 => "
      BUS.bussabt0ecbi(),
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
  0x40004824u64 => "
      BUS.buserrrw()[0],
    ",
  0x40004834u64 => "
      BUS.buserrrw()[1],
    ",
  0x40004844u64 => "
      BUS.buserrrw()[2],
    ",
  0x40004854u64 => "
      BUS.buserrrw()[3],
    ",
  0x40004850u64 => "
      BUS.buserradd()[0],
    ",
  0x40004860u64 => "
      BUS.buserradd()[1],
    ",
  0x40004870u64 => "
      BUS.buserradd()[2],
    ",
  0x40004880u64 => "
      BUS.buserradd()[3],
    ",
  0x40004920u64 => "
      BUS.bmsaerradd()[0],
    ",
  0x40004930u64 => "
      BUS.bmsaerradd()[1],
    ",
  0x40004940u64 => "
      BUS.bmsaerradd()[2],
    ",
  0x40004950u64 => "
      BUS.bmsaerradd()[3],
    ",
  0x40004924u64 => "
      BUS.bmsaerrrw()[0],
    ",
  0x40004934u64 => "
      BUS.bmsaerrrw()[1],
    ",
  0x40004944u64 => "
      BUS.bmsaerrrw()[2],
    ",
  0x40004954u64 => "
      BUS.bmsaerrrw()[3],
    ",
  0x40004a50u64 => "
      BUS.buserrstat()[0],
    ",
  0x40004a60u64 => "
      BUS.buserrstat()[1],
    ",
  0x40004a70u64 => "
      BUS.buserrstat()[2],
    ",
  0x40004a80u64 => "
      BUS.buserrstat()[3],
    ",
  0x40004a58u64 => "
      BUS.buserrclr()[0],
    ",
  0x40004a68u64 => "
      BUS.buserrclr()[1],
    ",
  0x40004a78u64 => "
      BUS.buserrclr()[2],
    ",
  0x40004a88u64 => "
      BUS.buserrclr()[3],
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
  0x40008110u64 => "
      CPSCU.bussarc(),
    ",
  0x40008114u64 => "
      CPSCU.busparc(),
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
  0x40008400u64 => "
      CPSCU.sramsabar()[0],
    ",
  0x40008404u64 => "
      CPSCU.sramsabar()[1],
    ",
  0x40008408u64 => "
      CPSCU.sramsabar()[2],
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
  0x4001e00cu64 => "
      SYSC.sbycr(),
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
  0x4001e052u64 => "
      SYSC.ebckocr(),
    ",
  0x4001e053u64 => "
      SYSC.sdckocr(),
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
  0x4001e05eu64 => "
      SYSC.lcdckdivcr(),
    ",
  0x4001e05fu64 => "
      SYSC.lcdckcr(),
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
  0x4001e110u64 => "
      SYSC.pdctrgd(),
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
  0x4001e3d8u64 => "
      SYSC.pgcsar(),
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
  0x40342000u64 => "
      GLCDC.gr1_clut0()[0],
    ",
  0x40342004u64 => "
      GLCDC.gr1_clut0()[1],
    ",
  0x40342008u64 => "
      GLCDC.gr1_clut0()[2],
    ",
  0x4034200cu64 => "
      GLCDC.gr1_clut0()[3],
    ",
  0x40342010u64 => "
      GLCDC.gr1_clut0()[4],
    ",
  0x40342014u64 => "
      GLCDC.gr1_clut0()[5],
    ",
  0x40342018u64 => "
      GLCDC.gr1_clut0()[6],
    ",
  0x4034201cu64 => "
      GLCDC.gr1_clut0()[7],
    ",
  0x40342020u64 => "
      GLCDC.gr1_clut0()[8],
    ",
  0x40342024u64 => "
      GLCDC.gr1_clut0()[9],
    ",
  0x40342028u64 => "
      GLCDC.gr1_clut0()[10],
    ",
  0x4034202cu64 => "
      GLCDC.gr1_clut0()[11],
    ",
  0x40342030u64 => "
      GLCDC.gr1_clut0()[12],
    ",
  0x40342034u64 => "
      GLCDC.gr1_clut0()[13],
    ",
  0x40342038u64 => "
      GLCDC.gr1_clut0()[14],
    ",
  0x4034203cu64 => "
      GLCDC.gr1_clut0()[15],
    ",
  0x40342040u64 => "
      GLCDC.gr1_clut0()[16],
    ",
  0x40342044u64 => "
      GLCDC.gr1_clut0()[17],
    ",
  0x40342048u64 => "
      GLCDC.gr1_clut0()[18],
    ",
  0x4034204cu64 => "
      GLCDC.gr1_clut0()[19],
    ",
  0x40342050u64 => "
      GLCDC.gr1_clut0()[20],
    ",
  0x40342054u64 => "
      GLCDC.gr1_clut0()[21],
    ",
  0x40342058u64 => "
      GLCDC.gr1_clut0()[22],
    ",
  0x4034205cu64 => "
      GLCDC.gr1_clut0()[23],
    ",
  0x40342060u64 => "
      GLCDC.gr1_clut0()[24],
    ",
  0x40342064u64 => "
      GLCDC.gr1_clut0()[25],
    ",
  0x40342068u64 => "
      GLCDC.gr1_clut0()[26],
    ",
  0x4034206cu64 => "
      GLCDC.gr1_clut0()[27],
    ",
  0x40342070u64 => "
      GLCDC.gr1_clut0()[28],
    ",
  0x40342074u64 => "
      GLCDC.gr1_clut0()[29],
    ",
  0x40342078u64 => "
      GLCDC.gr1_clut0()[30],
    ",
  0x4034207cu64 => "
      GLCDC.gr1_clut0()[31],
    ",
  0x40342080u64 => "
      GLCDC.gr1_clut0()[32],
    ",
  0x40342084u64 => "
      GLCDC.gr1_clut0()[33],
    ",
  0x40342088u64 => "
      GLCDC.gr1_clut0()[34],
    ",
  0x4034208cu64 => "
      GLCDC.gr1_clut0()[35],
    ",
  0x40342090u64 => "
      GLCDC.gr1_clut0()[36],
    ",
  0x40342094u64 => "
      GLCDC.gr1_clut0()[37],
    ",
  0x40342098u64 => "
      GLCDC.gr1_clut0()[38],
    ",
  0x4034209cu64 => "
      GLCDC.gr1_clut0()[39],
    ",
  0x403420a0u64 => "
      GLCDC.gr1_clut0()[40],
    ",
  0x403420a4u64 => "
      GLCDC.gr1_clut0()[41],
    ",
  0x403420a8u64 => "
      GLCDC.gr1_clut0()[42],
    ",
  0x403420acu64 => "
      GLCDC.gr1_clut0()[43],
    ",
  0x403420b0u64 => "
      GLCDC.gr1_clut0()[44],
    ",
  0x403420b4u64 => "
      GLCDC.gr1_clut0()[45],
    ",
  0x403420b8u64 => "
      GLCDC.gr1_clut0()[46],
    ",
  0x403420bcu64 => "
      GLCDC.gr1_clut0()[47],
    ",
  0x403420c0u64 => "
      GLCDC.gr1_clut0()[48],
    ",
  0x403420c4u64 => "
      GLCDC.gr1_clut0()[49],
    ",
  0x403420c8u64 => "
      GLCDC.gr1_clut0()[50],
    ",
  0x403420ccu64 => "
      GLCDC.gr1_clut0()[51],
    ",
  0x403420d0u64 => "
      GLCDC.gr1_clut0()[52],
    ",
  0x403420d4u64 => "
      GLCDC.gr1_clut0()[53],
    ",
  0x403420d8u64 => "
      GLCDC.gr1_clut0()[54],
    ",
  0x403420dcu64 => "
      GLCDC.gr1_clut0()[55],
    ",
  0x403420e0u64 => "
      GLCDC.gr1_clut0()[56],
    ",
  0x403420e4u64 => "
      GLCDC.gr1_clut0()[57],
    ",
  0x403420e8u64 => "
      GLCDC.gr1_clut0()[58],
    ",
  0x403420ecu64 => "
      GLCDC.gr1_clut0()[59],
    ",
  0x403420f0u64 => "
      GLCDC.gr1_clut0()[60],
    ",
  0x403420f4u64 => "
      GLCDC.gr1_clut0()[61],
    ",
  0x403420f8u64 => "
      GLCDC.gr1_clut0()[62],
    ",
  0x403420fcu64 => "
      GLCDC.gr1_clut0()[63],
    ",
  0x40342100u64 => "
      GLCDC.gr1_clut0()[64],
    ",
  0x40342104u64 => "
      GLCDC.gr1_clut0()[65],
    ",
  0x40342108u64 => "
      GLCDC.gr1_clut0()[66],
    ",
  0x4034210cu64 => "
      GLCDC.gr1_clut0()[67],
    ",
  0x40342110u64 => "
      GLCDC.gr1_clut0()[68],
    ",
  0x40342114u64 => "
      GLCDC.gr1_clut0()[69],
    ",
  0x40342118u64 => "
      GLCDC.gr1_clut0()[70],
    ",
  0x4034211cu64 => "
      GLCDC.gr1_clut0()[71],
    ",
  0x40342120u64 => "
      GLCDC.gr1_clut0()[72],
    ",
  0x40342124u64 => "
      GLCDC.gr1_clut0()[73],
    ",
  0x40342128u64 => "
      GLCDC.gr1_clut0()[74],
    ",
  0x4034212cu64 => "
      GLCDC.gr1_clut0()[75],
    ",
  0x40342130u64 => "
      GLCDC.gr1_clut0()[76],
    ",
  0x40342134u64 => "
      GLCDC.gr1_clut0()[77],
    ",
  0x40342138u64 => "
      GLCDC.gr1_clut0()[78],
    ",
  0x4034213cu64 => "
      GLCDC.gr1_clut0()[79],
    ",
  0x40342140u64 => "
      GLCDC.gr1_clut0()[80],
    ",
  0x40342144u64 => "
      GLCDC.gr1_clut0()[81],
    ",
  0x40342148u64 => "
      GLCDC.gr1_clut0()[82],
    ",
  0x4034214cu64 => "
      GLCDC.gr1_clut0()[83],
    ",
  0x40342150u64 => "
      GLCDC.gr1_clut0()[84],
    ",
  0x40342154u64 => "
      GLCDC.gr1_clut0()[85],
    ",
  0x40342158u64 => "
      GLCDC.gr1_clut0()[86],
    ",
  0x4034215cu64 => "
      GLCDC.gr1_clut0()[87],
    ",
  0x40342160u64 => "
      GLCDC.gr1_clut0()[88],
    ",
  0x40342164u64 => "
      GLCDC.gr1_clut0()[89],
    ",
  0x40342168u64 => "
      GLCDC.gr1_clut0()[90],
    ",
  0x4034216cu64 => "
      GLCDC.gr1_clut0()[91],
    ",
  0x40342170u64 => "
      GLCDC.gr1_clut0()[92],
    ",
  0x40342174u64 => "
      GLCDC.gr1_clut0()[93],
    ",
  0x40342178u64 => "
      GLCDC.gr1_clut0()[94],
    ",
  0x4034217cu64 => "
      GLCDC.gr1_clut0()[95],
    ",
  0x40342180u64 => "
      GLCDC.gr1_clut0()[96],
    ",
  0x40342184u64 => "
      GLCDC.gr1_clut0()[97],
    ",
  0x40342188u64 => "
      GLCDC.gr1_clut0()[98],
    ",
  0x4034218cu64 => "
      GLCDC.gr1_clut0()[99],
    ",
  0x40342190u64 => "
      GLCDC.gr1_clut0()[100],
    ",
  0x40342194u64 => "
      GLCDC.gr1_clut0()[101],
    ",
  0x40342198u64 => "
      GLCDC.gr1_clut0()[102],
    ",
  0x4034219cu64 => "
      GLCDC.gr1_clut0()[103],
    ",
  0x403421a0u64 => "
      GLCDC.gr1_clut0()[104],
    ",
  0x403421a4u64 => "
      GLCDC.gr1_clut0()[105],
    ",
  0x403421a8u64 => "
      GLCDC.gr1_clut0()[106],
    ",
  0x403421acu64 => "
      GLCDC.gr1_clut0()[107],
    ",
  0x403421b0u64 => "
      GLCDC.gr1_clut0()[108],
    ",
  0x403421b4u64 => "
      GLCDC.gr1_clut0()[109],
    ",
  0x403421b8u64 => "
      GLCDC.gr1_clut0()[110],
    ",
  0x403421bcu64 => "
      GLCDC.gr1_clut0()[111],
    ",
  0x403421c0u64 => "
      GLCDC.gr1_clut0()[112],
    ",
  0x403421c4u64 => "
      GLCDC.gr1_clut0()[113],
    ",
  0x403421c8u64 => "
      GLCDC.gr1_clut0()[114],
    ",
  0x403421ccu64 => "
      GLCDC.gr1_clut0()[115],
    ",
  0x403421d0u64 => "
      GLCDC.gr1_clut0()[116],
    ",
  0x403421d4u64 => "
      GLCDC.gr1_clut0()[117],
    ",
  0x403421d8u64 => "
      GLCDC.gr1_clut0()[118],
    ",
  0x403421dcu64 => "
      GLCDC.gr1_clut0()[119],
    ",
  0x403421e0u64 => "
      GLCDC.gr1_clut0()[120],
    ",
  0x403421e4u64 => "
      GLCDC.gr1_clut0()[121],
    ",
  0x403421e8u64 => "
      GLCDC.gr1_clut0()[122],
    ",
  0x403421ecu64 => "
      GLCDC.gr1_clut0()[123],
    ",
  0x403421f0u64 => "
      GLCDC.gr1_clut0()[124],
    ",
  0x403421f4u64 => "
      GLCDC.gr1_clut0()[125],
    ",
  0x403421f8u64 => "
      GLCDC.gr1_clut0()[126],
    ",
  0x403421fcu64 => "
      GLCDC.gr1_clut0()[127],
    ",
  0x40342200u64 => "
      GLCDC.gr1_clut0()[128],
    ",
  0x40342204u64 => "
      GLCDC.gr1_clut0()[129],
    ",
  0x40342208u64 => "
      GLCDC.gr1_clut0()[130],
    ",
  0x4034220cu64 => "
      GLCDC.gr1_clut0()[131],
    ",
  0x40342210u64 => "
      GLCDC.gr1_clut0()[132],
    ",
  0x40342214u64 => "
      GLCDC.gr1_clut0()[133],
    ",
  0x40342218u64 => "
      GLCDC.gr1_clut0()[134],
    ",
  0x4034221cu64 => "
      GLCDC.gr1_clut0()[135],
    ",
  0x40342220u64 => "
      GLCDC.gr1_clut0()[136],
    ",
  0x40342224u64 => "
      GLCDC.gr1_clut0()[137],
    ",
  0x40342228u64 => "
      GLCDC.gr1_clut0()[138],
    ",
  0x4034222cu64 => "
      GLCDC.gr1_clut0()[139],
    ",
  0x40342230u64 => "
      GLCDC.gr1_clut0()[140],
    ",
  0x40342234u64 => "
      GLCDC.gr1_clut0()[141],
    ",
  0x40342238u64 => "
      GLCDC.gr1_clut0()[142],
    ",
  0x4034223cu64 => "
      GLCDC.gr1_clut0()[143],
    ",
  0x40342240u64 => "
      GLCDC.gr1_clut0()[144],
    ",
  0x40342244u64 => "
      GLCDC.gr1_clut0()[145],
    ",
  0x40342248u64 => "
      GLCDC.gr1_clut0()[146],
    ",
  0x4034224cu64 => "
      GLCDC.gr1_clut0()[147],
    ",
  0x40342250u64 => "
      GLCDC.gr1_clut0()[148],
    ",
  0x40342254u64 => "
      GLCDC.gr1_clut0()[149],
    ",
  0x40342258u64 => "
      GLCDC.gr1_clut0()[150],
    ",
  0x4034225cu64 => "
      GLCDC.gr1_clut0()[151],
    ",
  0x40342260u64 => "
      GLCDC.gr1_clut0()[152],
    ",
  0x40342264u64 => "
      GLCDC.gr1_clut0()[153],
    ",
  0x40342268u64 => "
      GLCDC.gr1_clut0()[154],
    ",
  0x4034226cu64 => "
      GLCDC.gr1_clut0()[155],
    ",
  0x40342270u64 => "
      GLCDC.gr1_clut0()[156],
    ",
  0x40342274u64 => "
      GLCDC.gr1_clut0()[157],
    ",
  0x40342278u64 => "
      GLCDC.gr1_clut0()[158],
    ",
  0x4034227cu64 => "
      GLCDC.gr1_clut0()[159],
    ",
  0x40342280u64 => "
      GLCDC.gr1_clut0()[160],
    ",
  0x40342284u64 => "
      GLCDC.gr1_clut0()[161],
    ",
  0x40342288u64 => "
      GLCDC.gr1_clut0()[162],
    ",
  0x4034228cu64 => "
      GLCDC.gr1_clut0()[163],
    ",
  0x40342290u64 => "
      GLCDC.gr1_clut0()[164],
    ",
  0x40342294u64 => "
      GLCDC.gr1_clut0()[165],
    ",
  0x40342298u64 => "
      GLCDC.gr1_clut0()[166],
    ",
  0x4034229cu64 => "
      GLCDC.gr1_clut0()[167],
    ",
  0x403422a0u64 => "
      GLCDC.gr1_clut0()[168],
    ",
  0x403422a4u64 => "
      GLCDC.gr1_clut0()[169],
    ",
  0x403422a8u64 => "
      GLCDC.gr1_clut0()[170],
    ",
  0x403422acu64 => "
      GLCDC.gr1_clut0()[171],
    ",
  0x403422b0u64 => "
      GLCDC.gr1_clut0()[172],
    ",
  0x403422b4u64 => "
      GLCDC.gr1_clut0()[173],
    ",
  0x403422b8u64 => "
      GLCDC.gr1_clut0()[174],
    ",
  0x403422bcu64 => "
      GLCDC.gr1_clut0()[175],
    ",
  0x403422c0u64 => "
      GLCDC.gr1_clut0()[176],
    ",
  0x403422c4u64 => "
      GLCDC.gr1_clut0()[177],
    ",
  0x403422c8u64 => "
      GLCDC.gr1_clut0()[178],
    ",
  0x403422ccu64 => "
      GLCDC.gr1_clut0()[179],
    ",
  0x403422d0u64 => "
      GLCDC.gr1_clut0()[180],
    ",
  0x403422d4u64 => "
      GLCDC.gr1_clut0()[181],
    ",
  0x403422d8u64 => "
      GLCDC.gr1_clut0()[182],
    ",
  0x403422dcu64 => "
      GLCDC.gr1_clut0()[183],
    ",
  0x403422e0u64 => "
      GLCDC.gr1_clut0()[184],
    ",
  0x403422e4u64 => "
      GLCDC.gr1_clut0()[185],
    ",
  0x403422e8u64 => "
      GLCDC.gr1_clut0()[186],
    ",
  0x403422ecu64 => "
      GLCDC.gr1_clut0()[187],
    ",
  0x403422f0u64 => "
      GLCDC.gr1_clut0()[188],
    ",
  0x403422f4u64 => "
      GLCDC.gr1_clut0()[189],
    ",
  0x403422f8u64 => "
      GLCDC.gr1_clut0()[190],
    ",
  0x403422fcu64 => "
      GLCDC.gr1_clut0()[191],
    ",
  0x40342300u64 => "
      GLCDC.gr1_clut0()[192],
    ",
  0x40342304u64 => "
      GLCDC.gr1_clut0()[193],
    ",
  0x40342308u64 => "
      GLCDC.gr1_clut0()[194],
    ",
  0x4034230cu64 => "
      GLCDC.gr1_clut0()[195],
    ",
  0x40342310u64 => "
      GLCDC.gr1_clut0()[196],
    ",
  0x40342314u64 => "
      GLCDC.gr1_clut0()[197],
    ",
  0x40342318u64 => "
      GLCDC.gr1_clut0()[198],
    ",
  0x4034231cu64 => "
      GLCDC.gr1_clut0()[199],
    ",
  0x40342320u64 => "
      GLCDC.gr1_clut0()[200],
    ",
  0x40342324u64 => "
      GLCDC.gr1_clut0()[201],
    ",
  0x40342328u64 => "
      GLCDC.gr1_clut0()[202],
    ",
  0x4034232cu64 => "
      GLCDC.gr1_clut0()[203],
    ",
  0x40342330u64 => "
      GLCDC.gr1_clut0()[204],
    ",
  0x40342334u64 => "
      GLCDC.gr1_clut0()[205],
    ",
  0x40342338u64 => "
      GLCDC.gr1_clut0()[206],
    ",
  0x4034233cu64 => "
      GLCDC.gr1_clut0()[207],
    ",
  0x40342340u64 => "
      GLCDC.gr1_clut0()[208],
    ",
  0x40342344u64 => "
      GLCDC.gr1_clut0()[209],
    ",
  0x40342348u64 => "
      GLCDC.gr1_clut0()[210],
    ",
  0x4034234cu64 => "
      GLCDC.gr1_clut0()[211],
    ",
  0x40342350u64 => "
      GLCDC.gr1_clut0()[212],
    ",
  0x40342354u64 => "
      GLCDC.gr1_clut0()[213],
    ",
  0x40342358u64 => "
      GLCDC.gr1_clut0()[214],
    ",
  0x4034235cu64 => "
      GLCDC.gr1_clut0()[215],
    ",
  0x40342360u64 => "
      GLCDC.gr1_clut0()[216],
    ",
  0x40342364u64 => "
      GLCDC.gr1_clut0()[217],
    ",
  0x40342368u64 => "
      GLCDC.gr1_clut0()[218],
    ",
  0x4034236cu64 => "
      GLCDC.gr1_clut0()[219],
    ",
  0x40342370u64 => "
      GLCDC.gr1_clut0()[220],
    ",
  0x40342374u64 => "
      GLCDC.gr1_clut0()[221],
    ",
  0x40342378u64 => "
      GLCDC.gr1_clut0()[222],
    ",
  0x4034237cu64 => "
      GLCDC.gr1_clut0()[223],
    ",
  0x40342380u64 => "
      GLCDC.gr1_clut0()[224],
    ",
  0x40342384u64 => "
      GLCDC.gr1_clut0()[225],
    ",
  0x40342388u64 => "
      GLCDC.gr1_clut0()[226],
    ",
  0x4034238cu64 => "
      GLCDC.gr1_clut0()[227],
    ",
  0x40342390u64 => "
      GLCDC.gr1_clut0()[228],
    ",
  0x40342394u64 => "
      GLCDC.gr1_clut0()[229],
    ",
  0x40342398u64 => "
      GLCDC.gr1_clut0()[230],
    ",
  0x4034239cu64 => "
      GLCDC.gr1_clut0()[231],
    ",
  0x403423a0u64 => "
      GLCDC.gr1_clut0()[232],
    ",
  0x403423a4u64 => "
      GLCDC.gr1_clut0()[233],
    ",
  0x403423a8u64 => "
      GLCDC.gr1_clut0()[234],
    ",
  0x403423acu64 => "
      GLCDC.gr1_clut0()[235],
    ",
  0x403423b0u64 => "
      GLCDC.gr1_clut0()[236],
    ",
  0x403423b4u64 => "
      GLCDC.gr1_clut0()[237],
    ",
  0x403423b8u64 => "
      GLCDC.gr1_clut0()[238],
    ",
  0x403423bcu64 => "
      GLCDC.gr1_clut0()[239],
    ",
  0x403423c0u64 => "
      GLCDC.gr1_clut0()[240],
    ",
  0x403423c4u64 => "
      GLCDC.gr1_clut0()[241],
    ",
  0x403423c8u64 => "
      GLCDC.gr1_clut0()[242],
    ",
  0x403423ccu64 => "
      GLCDC.gr1_clut0()[243],
    ",
  0x403423d0u64 => "
      GLCDC.gr1_clut0()[244],
    ",
  0x403423d4u64 => "
      GLCDC.gr1_clut0()[245],
    ",
  0x403423d8u64 => "
      GLCDC.gr1_clut0()[246],
    ",
  0x403423dcu64 => "
      GLCDC.gr1_clut0()[247],
    ",
  0x403423e0u64 => "
      GLCDC.gr1_clut0()[248],
    ",
  0x403423e4u64 => "
      GLCDC.gr1_clut0()[249],
    ",
  0x403423e8u64 => "
      GLCDC.gr1_clut0()[250],
    ",
  0x403423ecu64 => "
      GLCDC.gr1_clut0()[251],
    ",
  0x403423f0u64 => "
      GLCDC.gr1_clut0()[252],
    ",
  0x403423f4u64 => "
      GLCDC.gr1_clut0()[253],
    ",
  0x403423f8u64 => "
      GLCDC.gr1_clut0()[254],
    ",
  0x403423fcu64 => "
      GLCDC.gr1_clut0()[255],
    ",
  0x40342400u64 => "
      GLCDC.gr1_clut1()[0],
    ",
  0x40342404u64 => "
      GLCDC.gr1_clut1()[1],
    ",
  0x40342408u64 => "
      GLCDC.gr1_clut1()[2],
    ",
  0x4034240cu64 => "
      GLCDC.gr1_clut1()[3],
    ",
  0x40342410u64 => "
      GLCDC.gr1_clut1()[4],
    ",
  0x40342414u64 => "
      GLCDC.gr1_clut1()[5],
    ",
  0x40342418u64 => "
      GLCDC.gr1_clut1()[6],
    ",
  0x4034241cu64 => "
      GLCDC.gr1_clut1()[7],
    ",
  0x40342420u64 => "
      GLCDC.gr1_clut1()[8],
    ",
  0x40342424u64 => "
      GLCDC.gr1_clut1()[9],
    ",
  0x40342428u64 => "
      GLCDC.gr1_clut1()[10],
    ",
  0x4034242cu64 => "
      GLCDC.gr1_clut1()[11],
    ",
  0x40342430u64 => "
      GLCDC.gr1_clut1()[12],
    ",
  0x40342434u64 => "
      GLCDC.gr1_clut1()[13],
    ",
  0x40342438u64 => "
      GLCDC.gr1_clut1()[14],
    ",
  0x4034243cu64 => "
      GLCDC.gr1_clut1()[15],
    ",
  0x40342440u64 => "
      GLCDC.gr1_clut1()[16],
    ",
  0x40342444u64 => "
      GLCDC.gr1_clut1()[17],
    ",
  0x40342448u64 => "
      GLCDC.gr1_clut1()[18],
    ",
  0x4034244cu64 => "
      GLCDC.gr1_clut1()[19],
    ",
  0x40342450u64 => "
      GLCDC.gr1_clut1()[20],
    ",
  0x40342454u64 => "
      GLCDC.gr1_clut1()[21],
    ",
  0x40342458u64 => "
      GLCDC.gr1_clut1()[22],
    ",
  0x4034245cu64 => "
      GLCDC.gr1_clut1()[23],
    ",
  0x40342460u64 => "
      GLCDC.gr1_clut1()[24],
    ",
  0x40342464u64 => "
      GLCDC.gr1_clut1()[25],
    ",
  0x40342468u64 => "
      GLCDC.gr1_clut1()[26],
    ",
  0x4034246cu64 => "
      GLCDC.gr1_clut1()[27],
    ",
  0x40342470u64 => "
      GLCDC.gr1_clut1()[28],
    ",
  0x40342474u64 => "
      GLCDC.gr1_clut1()[29],
    ",
  0x40342478u64 => "
      GLCDC.gr1_clut1()[30],
    ",
  0x4034247cu64 => "
      GLCDC.gr1_clut1()[31],
    ",
  0x40342480u64 => "
      GLCDC.gr1_clut1()[32],
    ",
  0x40342484u64 => "
      GLCDC.gr1_clut1()[33],
    ",
  0x40342488u64 => "
      GLCDC.gr1_clut1()[34],
    ",
  0x4034248cu64 => "
      GLCDC.gr1_clut1()[35],
    ",
  0x40342490u64 => "
      GLCDC.gr1_clut1()[36],
    ",
  0x40342494u64 => "
      GLCDC.gr1_clut1()[37],
    ",
  0x40342498u64 => "
      GLCDC.gr1_clut1()[38],
    ",
  0x4034249cu64 => "
      GLCDC.gr1_clut1()[39],
    ",
  0x403424a0u64 => "
      GLCDC.gr1_clut1()[40],
    ",
  0x403424a4u64 => "
      GLCDC.gr1_clut1()[41],
    ",
  0x403424a8u64 => "
      GLCDC.gr1_clut1()[42],
    ",
  0x403424acu64 => "
      GLCDC.gr1_clut1()[43],
    ",
  0x403424b0u64 => "
      GLCDC.gr1_clut1()[44],
    ",
  0x403424b4u64 => "
      GLCDC.gr1_clut1()[45],
    ",
  0x403424b8u64 => "
      GLCDC.gr1_clut1()[46],
    ",
  0x403424bcu64 => "
      GLCDC.gr1_clut1()[47],
    ",
  0x403424c0u64 => "
      GLCDC.gr1_clut1()[48],
    ",
  0x403424c4u64 => "
      GLCDC.gr1_clut1()[49],
    ",
  0x403424c8u64 => "
      GLCDC.gr1_clut1()[50],
    ",
  0x403424ccu64 => "
      GLCDC.gr1_clut1()[51],
    ",
  0x403424d0u64 => "
      GLCDC.gr1_clut1()[52],
    ",
  0x403424d4u64 => "
      GLCDC.gr1_clut1()[53],
    ",
  0x403424d8u64 => "
      GLCDC.gr1_clut1()[54],
    ",
  0x403424dcu64 => "
      GLCDC.gr1_clut1()[55],
    ",
  0x403424e0u64 => "
      GLCDC.gr1_clut1()[56],
    ",
  0x403424e4u64 => "
      GLCDC.gr1_clut1()[57],
    ",
  0x403424e8u64 => "
      GLCDC.gr1_clut1()[58],
    ",
  0x403424ecu64 => "
      GLCDC.gr1_clut1()[59],
    ",
  0x403424f0u64 => "
      GLCDC.gr1_clut1()[60],
    ",
  0x403424f4u64 => "
      GLCDC.gr1_clut1()[61],
    ",
  0x403424f8u64 => "
      GLCDC.gr1_clut1()[62],
    ",
  0x403424fcu64 => "
      GLCDC.gr1_clut1()[63],
    ",
  0x40342500u64 => "
      GLCDC.gr1_clut1()[64],
    ",
  0x40342504u64 => "
      GLCDC.gr1_clut1()[65],
    ",
  0x40342508u64 => "
      GLCDC.gr1_clut1()[66],
    ",
  0x4034250cu64 => "
      GLCDC.gr1_clut1()[67],
    ",
  0x40342510u64 => "
      GLCDC.gr1_clut1()[68],
    ",
  0x40342514u64 => "
      GLCDC.gr1_clut1()[69],
    ",
  0x40342518u64 => "
      GLCDC.gr1_clut1()[70],
    ",
  0x4034251cu64 => "
      GLCDC.gr1_clut1()[71],
    ",
  0x40342520u64 => "
      GLCDC.gr1_clut1()[72],
    ",
  0x40342524u64 => "
      GLCDC.gr1_clut1()[73],
    ",
  0x40342528u64 => "
      GLCDC.gr1_clut1()[74],
    ",
  0x4034252cu64 => "
      GLCDC.gr1_clut1()[75],
    ",
  0x40342530u64 => "
      GLCDC.gr1_clut1()[76],
    ",
  0x40342534u64 => "
      GLCDC.gr1_clut1()[77],
    ",
  0x40342538u64 => "
      GLCDC.gr1_clut1()[78],
    ",
  0x4034253cu64 => "
      GLCDC.gr1_clut1()[79],
    ",
  0x40342540u64 => "
      GLCDC.gr1_clut1()[80],
    ",
  0x40342544u64 => "
      GLCDC.gr1_clut1()[81],
    ",
  0x40342548u64 => "
      GLCDC.gr1_clut1()[82],
    ",
  0x4034254cu64 => "
      GLCDC.gr1_clut1()[83],
    ",
  0x40342550u64 => "
      GLCDC.gr1_clut1()[84],
    ",
  0x40342554u64 => "
      GLCDC.gr1_clut1()[85],
    ",
  0x40342558u64 => "
      GLCDC.gr1_clut1()[86],
    ",
  0x4034255cu64 => "
      GLCDC.gr1_clut1()[87],
    ",
  0x40342560u64 => "
      GLCDC.gr1_clut1()[88],
    ",
  0x40342564u64 => "
      GLCDC.gr1_clut1()[89],
    ",
  0x40342568u64 => "
      GLCDC.gr1_clut1()[90],
    ",
  0x4034256cu64 => "
      GLCDC.gr1_clut1()[91],
    ",
  0x40342570u64 => "
      GLCDC.gr1_clut1()[92],
    ",
  0x40342574u64 => "
      GLCDC.gr1_clut1()[93],
    ",
  0x40342578u64 => "
      GLCDC.gr1_clut1()[94],
    ",
  0x4034257cu64 => "
      GLCDC.gr1_clut1()[95],
    ",
  0x40342580u64 => "
      GLCDC.gr1_clut1()[96],
    ",
  0x40342584u64 => "
      GLCDC.gr1_clut1()[97],
    ",
  0x40342588u64 => "
      GLCDC.gr1_clut1()[98],
    ",
  0x4034258cu64 => "
      GLCDC.gr1_clut1()[99],
    ",
  0x40342590u64 => "
      GLCDC.gr1_clut1()[100],
    ",
  0x40342594u64 => "
      GLCDC.gr1_clut1()[101],
    ",
  0x40342598u64 => "
      GLCDC.gr1_clut1()[102],
    ",
  0x4034259cu64 => "
      GLCDC.gr1_clut1()[103],
    ",
  0x403425a0u64 => "
      GLCDC.gr1_clut1()[104],
    ",
  0x403425a4u64 => "
      GLCDC.gr1_clut1()[105],
    ",
  0x403425a8u64 => "
      GLCDC.gr1_clut1()[106],
    ",
  0x403425acu64 => "
      GLCDC.gr1_clut1()[107],
    ",
  0x403425b0u64 => "
      GLCDC.gr1_clut1()[108],
    ",
  0x403425b4u64 => "
      GLCDC.gr1_clut1()[109],
    ",
  0x403425b8u64 => "
      GLCDC.gr1_clut1()[110],
    ",
  0x403425bcu64 => "
      GLCDC.gr1_clut1()[111],
    ",
  0x403425c0u64 => "
      GLCDC.gr1_clut1()[112],
    ",
  0x403425c4u64 => "
      GLCDC.gr1_clut1()[113],
    ",
  0x403425c8u64 => "
      GLCDC.gr1_clut1()[114],
    ",
  0x403425ccu64 => "
      GLCDC.gr1_clut1()[115],
    ",
  0x403425d0u64 => "
      GLCDC.gr1_clut1()[116],
    ",
  0x403425d4u64 => "
      GLCDC.gr1_clut1()[117],
    ",
  0x403425d8u64 => "
      GLCDC.gr1_clut1()[118],
    ",
  0x403425dcu64 => "
      GLCDC.gr1_clut1()[119],
    ",
  0x403425e0u64 => "
      GLCDC.gr1_clut1()[120],
    ",
  0x403425e4u64 => "
      GLCDC.gr1_clut1()[121],
    ",
  0x403425e8u64 => "
      GLCDC.gr1_clut1()[122],
    ",
  0x403425ecu64 => "
      GLCDC.gr1_clut1()[123],
    ",
  0x403425f0u64 => "
      GLCDC.gr1_clut1()[124],
    ",
  0x403425f4u64 => "
      GLCDC.gr1_clut1()[125],
    ",
  0x403425f8u64 => "
      GLCDC.gr1_clut1()[126],
    ",
  0x403425fcu64 => "
      GLCDC.gr1_clut1()[127],
    ",
  0x40342600u64 => "
      GLCDC.gr1_clut1()[128],
    ",
  0x40342604u64 => "
      GLCDC.gr1_clut1()[129],
    ",
  0x40342608u64 => "
      GLCDC.gr1_clut1()[130],
    ",
  0x4034260cu64 => "
      GLCDC.gr1_clut1()[131],
    ",
  0x40342610u64 => "
      GLCDC.gr1_clut1()[132],
    ",
  0x40342614u64 => "
      GLCDC.gr1_clut1()[133],
    ",
  0x40342618u64 => "
      GLCDC.gr1_clut1()[134],
    ",
  0x4034261cu64 => "
      GLCDC.gr1_clut1()[135],
    ",
  0x40342620u64 => "
      GLCDC.gr1_clut1()[136],
    ",
  0x40342624u64 => "
      GLCDC.gr1_clut1()[137],
    ",
  0x40342628u64 => "
      GLCDC.gr1_clut1()[138],
    ",
  0x4034262cu64 => "
      GLCDC.gr1_clut1()[139],
    ",
  0x40342630u64 => "
      GLCDC.gr1_clut1()[140],
    ",
  0x40342634u64 => "
      GLCDC.gr1_clut1()[141],
    ",
  0x40342638u64 => "
      GLCDC.gr1_clut1()[142],
    ",
  0x4034263cu64 => "
      GLCDC.gr1_clut1()[143],
    ",
  0x40342640u64 => "
      GLCDC.gr1_clut1()[144],
    ",
  0x40342644u64 => "
      GLCDC.gr1_clut1()[145],
    ",
  0x40342648u64 => "
      GLCDC.gr1_clut1()[146],
    ",
  0x4034264cu64 => "
      GLCDC.gr1_clut1()[147],
    ",
  0x40342650u64 => "
      GLCDC.gr1_clut1()[148],
    ",
  0x40342654u64 => "
      GLCDC.gr1_clut1()[149],
    ",
  0x40342658u64 => "
      GLCDC.gr1_clut1()[150],
    ",
  0x4034265cu64 => "
      GLCDC.gr1_clut1()[151],
    ",
  0x40342660u64 => "
      GLCDC.gr1_clut1()[152],
    ",
  0x40342664u64 => "
      GLCDC.gr1_clut1()[153],
    ",
  0x40342668u64 => "
      GLCDC.gr1_clut1()[154],
    ",
  0x4034266cu64 => "
      GLCDC.gr1_clut1()[155],
    ",
  0x40342670u64 => "
      GLCDC.gr1_clut1()[156],
    ",
  0x40342674u64 => "
      GLCDC.gr1_clut1()[157],
    ",
  0x40342678u64 => "
      GLCDC.gr1_clut1()[158],
    ",
  0x4034267cu64 => "
      GLCDC.gr1_clut1()[159],
    ",
  0x40342680u64 => "
      GLCDC.gr1_clut1()[160],
    ",
  0x40342684u64 => "
      GLCDC.gr1_clut1()[161],
    ",
  0x40342688u64 => "
      GLCDC.gr1_clut1()[162],
    ",
  0x4034268cu64 => "
      GLCDC.gr1_clut1()[163],
    ",
  0x40342690u64 => "
      GLCDC.gr1_clut1()[164],
    ",
  0x40342694u64 => "
      GLCDC.gr1_clut1()[165],
    ",
  0x40342698u64 => "
      GLCDC.gr1_clut1()[166],
    ",
  0x4034269cu64 => "
      GLCDC.gr1_clut1()[167],
    ",
  0x403426a0u64 => "
      GLCDC.gr1_clut1()[168],
    ",
  0x403426a4u64 => "
      GLCDC.gr1_clut1()[169],
    ",
  0x403426a8u64 => "
      GLCDC.gr1_clut1()[170],
    ",
  0x403426acu64 => "
      GLCDC.gr1_clut1()[171],
    ",
  0x403426b0u64 => "
      GLCDC.gr1_clut1()[172],
    ",
  0x403426b4u64 => "
      GLCDC.gr1_clut1()[173],
    ",
  0x403426b8u64 => "
      GLCDC.gr1_clut1()[174],
    ",
  0x403426bcu64 => "
      GLCDC.gr1_clut1()[175],
    ",
  0x403426c0u64 => "
      GLCDC.gr1_clut1()[176],
    ",
  0x403426c4u64 => "
      GLCDC.gr1_clut1()[177],
    ",
  0x403426c8u64 => "
      GLCDC.gr1_clut1()[178],
    ",
  0x403426ccu64 => "
      GLCDC.gr1_clut1()[179],
    ",
  0x403426d0u64 => "
      GLCDC.gr1_clut1()[180],
    ",
  0x403426d4u64 => "
      GLCDC.gr1_clut1()[181],
    ",
  0x403426d8u64 => "
      GLCDC.gr1_clut1()[182],
    ",
  0x403426dcu64 => "
      GLCDC.gr1_clut1()[183],
    ",
  0x403426e0u64 => "
      GLCDC.gr1_clut1()[184],
    ",
  0x403426e4u64 => "
      GLCDC.gr1_clut1()[185],
    ",
  0x403426e8u64 => "
      GLCDC.gr1_clut1()[186],
    ",
  0x403426ecu64 => "
      GLCDC.gr1_clut1()[187],
    ",
  0x403426f0u64 => "
      GLCDC.gr1_clut1()[188],
    ",
  0x403426f4u64 => "
      GLCDC.gr1_clut1()[189],
    ",
  0x403426f8u64 => "
      GLCDC.gr1_clut1()[190],
    ",
  0x403426fcu64 => "
      GLCDC.gr1_clut1()[191],
    ",
  0x40342700u64 => "
      GLCDC.gr1_clut1()[192],
    ",
  0x40342704u64 => "
      GLCDC.gr1_clut1()[193],
    ",
  0x40342708u64 => "
      GLCDC.gr1_clut1()[194],
    ",
  0x4034270cu64 => "
      GLCDC.gr1_clut1()[195],
    ",
  0x40342710u64 => "
      GLCDC.gr1_clut1()[196],
    ",
  0x40342714u64 => "
      GLCDC.gr1_clut1()[197],
    ",
  0x40342718u64 => "
      GLCDC.gr1_clut1()[198],
    ",
  0x4034271cu64 => "
      GLCDC.gr1_clut1()[199],
    ",
  0x40342720u64 => "
      GLCDC.gr1_clut1()[200],
    ",
  0x40342724u64 => "
      GLCDC.gr1_clut1()[201],
    ",
  0x40342728u64 => "
      GLCDC.gr1_clut1()[202],
    ",
  0x4034272cu64 => "
      GLCDC.gr1_clut1()[203],
    ",
  0x40342730u64 => "
      GLCDC.gr1_clut1()[204],
    ",
  0x40342734u64 => "
      GLCDC.gr1_clut1()[205],
    ",
  0x40342738u64 => "
      GLCDC.gr1_clut1()[206],
    ",
  0x4034273cu64 => "
      GLCDC.gr1_clut1()[207],
    ",
  0x40342740u64 => "
      GLCDC.gr1_clut1()[208],
    ",
  0x40342744u64 => "
      GLCDC.gr1_clut1()[209],
    ",
  0x40342748u64 => "
      GLCDC.gr1_clut1()[210],
    ",
  0x4034274cu64 => "
      GLCDC.gr1_clut1()[211],
    ",
  0x40342750u64 => "
      GLCDC.gr1_clut1()[212],
    ",
  0x40342754u64 => "
      GLCDC.gr1_clut1()[213],
    ",
  0x40342758u64 => "
      GLCDC.gr1_clut1()[214],
    ",
  0x4034275cu64 => "
      GLCDC.gr1_clut1()[215],
    ",
  0x40342760u64 => "
      GLCDC.gr1_clut1()[216],
    ",
  0x40342764u64 => "
      GLCDC.gr1_clut1()[217],
    ",
  0x40342768u64 => "
      GLCDC.gr1_clut1()[218],
    ",
  0x4034276cu64 => "
      GLCDC.gr1_clut1()[219],
    ",
  0x40342770u64 => "
      GLCDC.gr1_clut1()[220],
    ",
  0x40342774u64 => "
      GLCDC.gr1_clut1()[221],
    ",
  0x40342778u64 => "
      GLCDC.gr1_clut1()[222],
    ",
  0x4034277cu64 => "
      GLCDC.gr1_clut1()[223],
    ",
  0x40342780u64 => "
      GLCDC.gr1_clut1()[224],
    ",
  0x40342784u64 => "
      GLCDC.gr1_clut1()[225],
    ",
  0x40342788u64 => "
      GLCDC.gr1_clut1()[226],
    ",
  0x4034278cu64 => "
      GLCDC.gr1_clut1()[227],
    ",
  0x40342790u64 => "
      GLCDC.gr1_clut1()[228],
    ",
  0x40342794u64 => "
      GLCDC.gr1_clut1()[229],
    ",
  0x40342798u64 => "
      GLCDC.gr1_clut1()[230],
    ",
  0x4034279cu64 => "
      GLCDC.gr1_clut1()[231],
    ",
  0x403427a0u64 => "
      GLCDC.gr1_clut1()[232],
    ",
  0x403427a4u64 => "
      GLCDC.gr1_clut1()[233],
    ",
  0x403427a8u64 => "
      GLCDC.gr1_clut1()[234],
    ",
  0x403427acu64 => "
      GLCDC.gr1_clut1()[235],
    ",
  0x403427b0u64 => "
      GLCDC.gr1_clut1()[236],
    ",
  0x403427b4u64 => "
      GLCDC.gr1_clut1()[237],
    ",
  0x403427b8u64 => "
      GLCDC.gr1_clut1()[238],
    ",
  0x403427bcu64 => "
      GLCDC.gr1_clut1()[239],
    ",
  0x403427c0u64 => "
      GLCDC.gr1_clut1()[240],
    ",
  0x403427c4u64 => "
      GLCDC.gr1_clut1()[241],
    ",
  0x403427c8u64 => "
      GLCDC.gr1_clut1()[242],
    ",
  0x403427ccu64 => "
      GLCDC.gr1_clut1()[243],
    ",
  0x403427d0u64 => "
      GLCDC.gr1_clut1()[244],
    ",
  0x403427d4u64 => "
      GLCDC.gr1_clut1()[245],
    ",
  0x403427d8u64 => "
      GLCDC.gr1_clut1()[246],
    ",
  0x403427dcu64 => "
      GLCDC.gr1_clut1()[247],
    ",
  0x403427e0u64 => "
      GLCDC.gr1_clut1()[248],
    ",
  0x403427e4u64 => "
      GLCDC.gr1_clut1()[249],
    ",
  0x403427e8u64 => "
      GLCDC.gr1_clut1()[250],
    ",
  0x403427ecu64 => "
      GLCDC.gr1_clut1()[251],
    ",
  0x403427f0u64 => "
      GLCDC.gr1_clut1()[252],
    ",
  0x403427f4u64 => "
      GLCDC.gr1_clut1()[253],
    ",
  0x403427f8u64 => "
      GLCDC.gr1_clut1()[254],
    ",
  0x403427fcu64 => "
      GLCDC.gr1_clut1()[255],
    ",
  0x40342800u64 => "
      GLCDC.gr2_clut0()[0],
    ",
  0x40342804u64 => "
      GLCDC.gr2_clut0()[1],
    ",
  0x40342808u64 => "
      GLCDC.gr2_clut0()[2],
    ",
  0x4034280cu64 => "
      GLCDC.gr2_clut0()[3],
    ",
  0x40342810u64 => "
      GLCDC.gr2_clut0()[4],
    ",
  0x40342814u64 => "
      GLCDC.gr2_clut0()[5],
    ",
  0x40342818u64 => "
      GLCDC.gr2_clut0()[6],
    ",
  0x4034281cu64 => "
      GLCDC.gr2_clut0()[7],
    ",
  0x40342820u64 => "
      GLCDC.gr2_clut0()[8],
    ",
  0x40342824u64 => "
      GLCDC.gr2_clut0()[9],
    ",
  0x40342828u64 => "
      GLCDC.gr2_clut0()[10],
    ",
  0x4034282cu64 => "
      GLCDC.gr2_clut0()[11],
    ",
  0x40342830u64 => "
      GLCDC.gr2_clut0()[12],
    ",
  0x40342834u64 => "
      GLCDC.gr2_clut0()[13],
    ",
  0x40342838u64 => "
      GLCDC.gr2_clut0()[14],
    ",
  0x4034283cu64 => "
      GLCDC.gr2_clut0()[15],
    ",
  0x40342840u64 => "
      GLCDC.gr2_clut0()[16],
    ",
  0x40342844u64 => "
      GLCDC.gr2_clut0()[17],
    ",
  0x40342848u64 => "
      GLCDC.gr2_clut0()[18],
    ",
  0x4034284cu64 => "
      GLCDC.gr2_clut0()[19],
    ",
  0x40342850u64 => "
      GLCDC.gr2_clut0()[20],
    ",
  0x40342854u64 => "
      GLCDC.gr2_clut0()[21],
    ",
  0x40342858u64 => "
      GLCDC.gr2_clut0()[22],
    ",
  0x4034285cu64 => "
      GLCDC.gr2_clut0()[23],
    ",
  0x40342860u64 => "
      GLCDC.gr2_clut0()[24],
    ",
  0x40342864u64 => "
      GLCDC.gr2_clut0()[25],
    ",
  0x40342868u64 => "
      GLCDC.gr2_clut0()[26],
    ",
  0x4034286cu64 => "
      GLCDC.gr2_clut0()[27],
    ",
  0x40342870u64 => "
      GLCDC.gr2_clut0()[28],
    ",
  0x40342874u64 => "
      GLCDC.gr2_clut0()[29],
    ",
  0x40342878u64 => "
      GLCDC.gr2_clut0()[30],
    ",
  0x4034287cu64 => "
      GLCDC.gr2_clut0()[31],
    ",
  0x40342880u64 => "
      GLCDC.gr2_clut0()[32],
    ",
  0x40342884u64 => "
      GLCDC.gr2_clut0()[33],
    ",
  0x40342888u64 => "
      GLCDC.gr2_clut0()[34],
    ",
  0x4034288cu64 => "
      GLCDC.gr2_clut0()[35],
    ",
  0x40342890u64 => "
      GLCDC.gr2_clut0()[36],
    ",
  0x40342894u64 => "
      GLCDC.gr2_clut0()[37],
    ",
  0x40342898u64 => "
      GLCDC.gr2_clut0()[38],
    ",
  0x4034289cu64 => "
      GLCDC.gr2_clut0()[39],
    ",
  0x403428a0u64 => "
      GLCDC.gr2_clut0()[40],
    ",
  0x403428a4u64 => "
      GLCDC.gr2_clut0()[41],
    ",
  0x403428a8u64 => "
      GLCDC.gr2_clut0()[42],
    ",
  0x403428acu64 => "
      GLCDC.gr2_clut0()[43],
    ",
  0x403428b0u64 => "
      GLCDC.gr2_clut0()[44],
    ",
  0x403428b4u64 => "
      GLCDC.gr2_clut0()[45],
    ",
  0x403428b8u64 => "
      GLCDC.gr2_clut0()[46],
    ",
  0x403428bcu64 => "
      GLCDC.gr2_clut0()[47],
    ",
  0x403428c0u64 => "
      GLCDC.gr2_clut0()[48],
    ",
  0x403428c4u64 => "
      GLCDC.gr2_clut0()[49],
    ",
  0x403428c8u64 => "
      GLCDC.gr2_clut0()[50],
    ",
  0x403428ccu64 => "
      GLCDC.gr2_clut0()[51],
    ",
  0x403428d0u64 => "
      GLCDC.gr2_clut0()[52],
    ",
  0x403428d4u64 => "
      GLCDC.gr2_clut0()[53],
    ",
  0x403428d8u64 => "
      GLCDC.gr2_clut0()[54],
    ",
  0x403428dcu64 => "
      GLCDC.gr2_clut0()[55],
    ",
  0x403428e0u64 => "
      GLCDC.gr2_clut0()[56],
    ",
  0x403428e4u64 => "
      GLCDC.gr2_clut0()[57],
    ",
  0x403428e8u64 => "
      GLCDC.gr2_clut0()[58],
    ",
  0x403428ecu64 => "
      GLCDC.gr2_clut0()[59],
    ",
  0x403428f0u64 => "
      GLCDC.gr2_clut0()[60],
    ",
  0x403428f4u64 => "
      GLCDC.gr2_clut0()[61],
    ",
  0x403428f8u64 => "
      GLCDC.gr2_clut0()[62],
    ",
  0x403428fcu64 => "
      GLCDC.gr2_clut0()[63],
    ",
  0x40342900u64 => "
      GLCDC.gr2_clut0()[64],
    ",
  0x40342904u64 => "
      GLCDC.gr2_clut0()[65],
    ",
  0x40342908u64 => "
      GLCDC.gr2_clut0()[66],
    ",
  0x4034290cu64 => "
      GLCDC.gr2_clut0()[67],
    ",
  0x40342910u64 => "
      GLCDC.gr2_clut0()[68],
    ",
  0x40342914u64 => "
      GLCDC.gr2_clut0()[69],
    ",
  0x40342918u64 => "
      GLCDC.gr2_clut0()[70],
    ",
  0x4034291cu64 => "
      GLCDC.gr2_clut0()[71],
    ",
  0x40342920u64 => "
      GLCDC.gr2_clut0()[72],
    ",
  0x40342924u64 => "
      GLCDC.gr2_clut0()[73],
    ",
  0x40342928u64 => "
      GLCDC.gr2_clut0()[74],
    ",
  0x4034292cu64 => "
      GLCDC.gr2_clut0()[75],
    ",
  0x40342930u64 => "
      GLCDC.gr2_clut0()[76],
    ",
  0x40342934u64 => "
      GLCDC.gr2_clut0()[77],
    ",
  0x40342938u64 => "
      GLCDC.gr2_clut0()[78],
    ",
  0x4034293cu64 => "
      GLCDC.gr2_clut0()[79],
    ",
  0x40342940u64 => "
      GLCDC.gr2_clut0()[80],
    ",
  0x40342944u64 => "
      GLCDC.gr2_clut0()[81],
    ",
  0x40342948u64 => "
      GLCDC.gr2_clut0()[82],
    ",
  0x4034294cu64 => "
      GLCDC.gr2_clut0()[83],
    ",
  0x40342950u64 => "
      GLCDC.gr2_clut0()[84],
    ",
  0x40342954u64 => "
      GLCDC.gr2_clut0()[85],
    ",
  0x40342958u64 => "
      GLCDC.gr2_clut0()[86],
    ",
  0x4034295cu64 => "
      GLCDC.gr2_clut0()[87],
    ",
  0x40342960u64 => "
      GLCDC.gr2_clut0()[88],
    ",
  0x40342964u64 => "
      GLCDC.gr2_clut0()[89],
    ",
  0x40342968u64 => "
      GLCDC.gr2_clut0()[90],
    ",
  0x4034296cu64 => "
      GLCDC.gr2_clut0()[91],
    ",
  0x40342970u64 => "
      GLCDC.gr2_clut0()[92],
    ",
  0x40342974u64 => "
      GLCDC.gr2_clut0()[93],
    ",
  0x40342978u64 => "
      GLCDC.gr2_clut0()[94],
    ",
  0x4034297cu64 => "
      GLCDC.gr2_clut0()[95],
    ",
  0x40342980u64 => "
      GLCDC.gr2_clut0()[96],
    ",
  0x40342984u64 => "
      GLCDC.gr2_clut0()[97],
    ",
  0x40342988u64 => "
      GLCDC.gr2_clut0()[98],
    ",
  0x4034298cu64 => "
      GLCDC.gr2_clut0()[99],
    ",
  0x40342990u64 => "
      GLCDC.gr2_clut0()[100],
    ",
  0x40342994u64 => "
      GLCDC.gr2_clut0()[101],
    ",
  0x40342998u64 => "
      GLCDC.gr2_clut0()[102],
    ",
  0x4034299cu64 => "
      GLCDC.gr2_clut0()[103],
    ",
  0x403429a0u64 => "
      GLCDC.gr2_clut0()[104],
    ",
  0x403429a4u64 => "
      GLCDC.gr2_clut0()[105],
    ",
  0x403429a8u64 => "
      GLCDC.gr2_clut0()[106],
    ",
  0x403429acu64 => "
      GLCDC.gr2_clut0()[107],
    ",
  0x403429b0u64 => "
      GLCDC.gr2_clut0()[108],
    ",
  0x403429b4u64 => "
      GLCDC.gr2_clut0()[109],
    ",
  0x403429b8u64 => "
      GLCDC.gr2_clut0()[110],
    ",
  0x403429bcu64 => "
      GLCDC.gr2_clut0()[111],
    ",
  0x403429c0u64 => "
      GLCDC.gr2_clut0()[112],
    ",
  0x403429c4u64 => "
      GLCDC.gr2_clut0()[113],
    ",
  0x403429c8u64 => "
      GLCDC.gr2_clut0()[114],
    ",
  0x403429ccu64 => "
      GLCDC.gr2_clut0()[115],
    ",
  0x403429d0u64 => "
      GLCDC.gr2_clut0()[116],
    ",
  0x403429d4u64 => "
      GLCDC.gr2_clut0()[117],
    ",
  0x403429d8u64 => "
      GLCDC.gr2_clut0()[118],
    ",
  0x403429dcu64 => "
      GLCDC.gr2_clut0()[119],
    ",
  0x403429e0u64 => "
      GLCDC.gr2_clut0()[120],
    ",
  0x403429e4u64 => "
      GLCDC.gr2_clut0()[121],
    ",
  0x403429e8u64 => "
      GLCDC.gr2_clut0()[122],
    ",
  0x403429ecu64 => "
      GLCDC.gr2_clut0()[123],
    ",
  0x403429f0u64 => "
      GLCDC.gr2_clut0()[124],
    ",
  0x403429f4u64 => "
      GLCDC.gr2_clut0()[125],
    ",
  0x403429f8u64 => "
      GLCDC.gr2_clut0()[126],
    ",
  0x403429fcu64 => "
      GLCDC.gr2_clut0()[127],
    ",
  0x40342a00u64 => "
      GLCDC.gr2_clut0()[128],
    ",
  0x40342a04u64 => "
      GLCDC.gr2_clut0()[129],
    ",
  0x40342a08u64 => "
      GLCDC.gr2_clut0()[130],
    ",
  0x40342a0cu64 => "
      GLCDC.gr2_clut0()[131],
    ",
  0x40342a10u64 => "
      GLCDC.gr2_clut0()[132],
    ",
  0x40342a14u64 => "
      GLCDC.gr2_clut0()[133],
    ",
  0x40342a18u64 => "
      GLCDC.gr2_clut0()[134],
    ",
  0x40342a1cu64 => "
      GLCDC.gr2_clut0()[135],
    ",
  0x40342a20u64 => "
      GLCDC.gr2_clut0()[136],
    ",
  0x40342a24u64 => "
      GLCDC.gr2_clut0()[137],
    ",
  0x40342a28u64 => "
      GLCDC.gr2_clut0()[138],
    ",
  0x40342a2cu64 => "
      GLCDC.gr2_clut0()[139],
    ",
  0x40342a30u64 => "
      GLCDC.gr2_clut0()[140],
    ",
  0x40342a34u64 => "
      GLCDC.gr2_clut0()[141],
    ",
  0x40342a38u64 => "
      GLCDC.gr2_clut0()[142],
    ",
  0x40342a3cu64 => "
      GLCDC.gr2_clut0()[143],
    ",
  0x40342a40u64 => "
      GLCDC.gr2_clut0()[144],
    ",
  0x40342a44u64 => "
      GLCDC.gr2_clut0()[145],
    ",
  0x40342a48u64 => "
      GLCDC.gr2_clut0()[146],
    ",
  0x40342a4cu64 => "
      GLCDC.gr2_clut0()[147],
    ",
  0x40342a50u64 => "
      GLCDC.gr2_clut0()[148],
    ",
  0x40342a54u64 => "
      GLCDC.gr2_clut0()[149],
    ",
  0x40342a58u64 => "
      GLCDC.gr2_clut0()[150],
    ",
  0x40342a5cu64 => "
      GLCDC.gr2_clut0()[151],
    ",
  0x40342a60u64 => "
      GLCDC.gr2_clut0()[152],
    ",
  0x40342a64u64 => "
      GLCDC.gr2_clut0()[153],
    ",
  0x40342a68u64 => "
      GLCDC.gr2_clut0()[154],
    ",
  0x40342a6cu64 => "
      GLCDC.gr2_clut0()[155],
    ",
  0x40342a70u64 => "
      GLCDC.gr2_clut0()[156],
    ",
  0x40342a74u64 => "
      GLCDC.gr2_clut0()[157],
    ",
  0x40342a78u64 => "
      GLCDC.gr2_clut0()[158],
    ",
  0x40342a7cu64 => "
      GLCDC.gr2_clut0()[159],
    ",
  0x40342a80u64 => "
      GLCDC.gr2_clut0()[160],
    ",
  0x40342a84u64 => "
      GLCDC.gr2_clut0()[161],
    ",
  0x40342a88u64 => "
      GLCDC.gr2_clut0()[162],
    ",
  0x40342a8cu64 => "
      GLCDC.gr2_clut0()[163],
    ",
  0x40342a90u64 => "
      GLCDC.gr2_clut0()[164],
    ",
  0x40342a94u64 => "
      GLCDC.gr2_clut0()[165],
    ",
  0x40342a98u64 => "
      GLCDC.gr2_clut0()[166],
    ",
  0x40342a9cu64 => "
      GLCDC.gr2_clut0()[167],
    ",
  0x40342aa0u64 => "
      GLCDC.gr2_clut0()[168],
    ",
  0x40342aa4u64 => "
      GLCDC.gr2_clut0()[169],
    ",
  0x40342aa8u64 => "
      GLCDC.gr2_clut0()[170],
    ",
  0x40342aacu64 => "
      GLCDC.gr2_clut0()[171],
    ",
  0x40342ab0u64 => "
      GLCDC.gr2_clut0()[172],
    ",
  0x40342ab4u64 => "
      GLCDC.gr2_clut0()[173],
    ",
  0x40342ab8u64 => "
      GLCDC.gr2_clut0()[174],
    ",
  0x40342abcu64 => "
      GLCDC.gr2_clut0()[175],
    ",
  0x40342ac0u64 => "
      GLCDC.gr2_clut0()[176],
    ",
  0x40342ac4u64 => "
      GLCDC.gr2_clut0()[177],
    ",
  0x40342ac8u64 => "
      GLCDC.gr2_clut0()[178],
    ",
  0x40342accu64 => "
      GLCDC.gr2_clut0()[179],
    ",
  0x40342ad0u64 => "
      GLCDC.gr2_clut0()[180],
    ",
  0x40342ad4u64 => "
      GLCDC.gr2_clut0()[181],
    ",
  0x40342ad8u64 => "
      GLCDC.gr2_clut0()[182],
    ",
  0x40342adcu64 => "
      GLCDC.gr2_clut0()[183],
    ",
  0x40342ae0u64 => "
      GLCDC.gr2_clut0()[184],
    ",
  0x40342ae4u64 => "
      GLCDC.gr2_clut0()[185],
    ",
  0x40342ae8u64 => "
      GLCDC.gr2_clut0()[186],
    ",
  0x40342aecu64 => "
      GLCDC.gr2_clut0()[187],
    ",
  0x40342af0u64 => "
      GLCDC.gr2_clut0()[188],
    ",
  0x40342af4u64 => "
      GLCDC.gr2_clut0()[189],
    ",
  0x40342af8u64 => "
      GLCDC.gr2_clut0()[190],
    ",
  0x40342afcu64 => "
      GLCDC.gr2_clut0()[191],
    ",
  0x40342b00u64 => "
      GLCDC.gr2_clut0()[192],
    ",
  0x40342b04u64 => "
      GLCDC.gr2_clut0()[193],
    ",
  0x40342b08u64 => "
      GLCDC.gr2_clut0()[194],
    ",
  0x40342b0cu64 => "
      GLCDC.gr2_clut0()[195],
    ",
  0x40342b10u64 => "
      GLCDC.gr2_clut0()[196],
    ",
  0x40342b14u64 => "
      GLCDC.gr2_clut0()[197],
    ",
  0x40342b18u64 => "
      GLCDC.gr2_clut0()[198],
    ",
  0x40342b1cu64 => "
      GLCDC.gr2_clut0()[199],
    ",
  0x40342b20u64 => "
      GLCDC.gr2_clut0()[200],
    ",
  0x40342b24u64 => "
      GLCDC.gr2_clut0()[201],
    ",
  0x40342b28u64 => "
      GLCDC.gr2_clut0()[202],
    ",
  0x40342b2cu64 => "
      GLCDC.gr2_clut0()[203],
    ",
  0x40342b30u64 => "
      GLCDC.gr2_clut0()[204],
    ",
  0x40342b34u64 => "
      GLCDC.gr2_clut0()[205],
    ",
  0x40342b38u64 => "
      GLCDC.gr2_clut0()[206],
    ",
  0x40342b3cu64 => "
      GLCDC.gr2_clut0()[207],
    ",
  0x40342b40u64 => "
      GLCDC.gr2_clut0()[208],
    ",
  0x40342b44u64 => "
      GLCDC.gr2_clut0()[209],
    ",
  0x40342b48u64 => "
      GLCDC.gr2_clut0()[210],
    ",
  0x40342b4cu64 => "
      GLCDC.gr2_clut0()[211],
    ",
  0x40342b50u64 => "
      GLCDC.gr2_clut0()[212],
    ",
  0x40342b54u64 => "
      GLCDC.gr2_clut0()[213],
    ",
  0x40342b58u64 => "
      GLCDC.gr2_clut0()[214],
    ",
  0x40342b5cu64 => "
      GLCDC.gr2_clut0()[215],
    ",
  0x40342b60u64 => "
      GLCDC.gr2_clut0()[216],
    ",
  0x40342b64u64 => "
      GLCDC.gr2_clut0()[217],
    ",
  0x40342b68u64 => "
      GLCDC.gr2_clut0()[218],
    ",
  0x40342b6cu64 => "
      GLCDC.gr2_clut0()[219],
    ",
  0x40342b70u64 => "
      GLCDC.gr2_clut0()[220],
    ",
  0x40342b74u64 => "
      GLCDC.gr2_clut0()[221],
    ",
  0x40342b78u64 => "
      GLCDC.gr2_clut0()[222],
    ",
  0x40342b7cu64 => "
      GLCDC.gr2_clut0()[223],
    ",
  0x40342b80u64 => "
      GLCDC.gr2_clut0()[224],
    ",
  0x40342b84u64 => "
      GLCDC.gr2_clut0()[225],
    ",
  0x40342b88u64 => "
      GLCDC.gr2_clut0()[226],
    ",
  0x40342b8cu64 => "
      GLCDC.gr2_clut0()[227],
    ",
  0x40342b90u64 => "
      GLCDC.gr2_clut0()[228],
    ",
  0x40342b94u64 => "
      GLCDC.gr2_clut0()[229],
    ",
  0x40342b98u64 => "
      GLCDC.gr2_clut0()[230],
    ",
  0x40342b9cu64 => "
      GLCDC.gr2_clut0()[231],
    ",
  0x40342ba0u64 => "
      GLCDC.gr2_clut0()[232],
    ",
  0x40342ba4u64 => "
      GLCDC.gr2_clut0()[233],
    ",
  0x40342ba8u64 => "
      GLCDC.gr2_clut0()[234],
    ",
  0x40342bacu64 => "
      GLCDC.gr2_clut0()[235],
    ",
  0x40342bb0u64 => "
      GLCDC.gr2_clut0()[236],
    ",
  0x40342bb4u64 => "
      GLCDC.gr2_clut0()[237],
    ",
  0x40342bb8u64 => "
      GLCDC.gr2_clut0()[238],
    ",
  0x40342bbcu64 => "
      GLCDC.gr2_clut0()[239],
    ",
  0x40342bc0u64 => "
      GLCDC.gr2_clut0()[240],
    ",
  0x40342bc4u64 => "
      GLCDC.gr2_clut0()[241],
    ",
  0x40342bc8u64 => "
      GLCDC.gr2_clut0()[242],
    ",
  0x40342bccu64 => "
      GLCDC.gr2_clut0()[243],
    ",
  0x40342bd0u64 => "
      GLCDC.gr2_clut0()[244],
    ",
  0x40342bd4u64 => "
      GLCDC.gr2_clut0()[245],
    ",
  0x40342bd8u64 => "
      GLCDC.gr2_clut0()[246],
    ",
  0x40342bdcu64 => "
      GLCDC.gr2_clut0()[247],
    ",
  0x40342be0u64 => "
      GLCDC.gr2_clut0()[248],
    ",
  0x40342be4u64 => "
      GLCDC.gr2_clut0()[249],
    ",
  0x40342be8u64 => "
      GLCDC.gr2_clut0()[250],
    ",
  0x40342becu64 => "
      GLCDC.gr2_clut0()[251],
    ",
  0x40342bf0u64 => "
      GLCDC.gr2_clut0()[252],
    ",
  0x40342bf4u64 => "
      GLCDC.gr2_clut0()[253],
    ",
  0x40342bf8u64 => "
      GLCDC.gr2_clut0()[254],
    ",
  0x40342bfcu64 => "
      GLCDC.gr2_clut0()[255],
    ",
  0x40342c00u64 => "
      GLCDC.gr2_clut1()[0],
    ",
  0x40342c04u64 => "
      GLCDC.gr2_clut1()[1],
    ",
  0x40342c08u64 => "
      GLCDC.gr2_clut1()[2],
    ",
  0x40342c0cu64 => "
      GLCDC.gr2_clut1()[3],
    ",
  0x40342c10u64 => "
      GLCDC.gr2_clut1()[4],
    ",
  0x40342c14u64 => "
      GLCDC.gr2_clut1()[5],
    ",
  0x40342c18u64 => "
      GLCDC.gr2_clut1()[6],
    ",
  0x40342c1cu64 => "
      GLCDC.gr2_clut1()[7],
    ",
  0x40342c20u64 => "
      GLCDC.gr2_clut1()[8],
    ",
  0x40342c24u64 => "
      GLCDC.gr2_clut1()[9],
    ",
  0x40342c28u64 => "
      GLCDC.gr2_clut1()[10],
    ",
  0x40342c2cu64 => "
      GLCDC.gr2_clut1()[11],
    ",
  0x40342c30u64 => "
      GLCDC.gr2_clut1()[12],
    ",
  0x40342c34u64 => "
      GLCDC.gr2_clut1()[13],
    ",
  0x40342c38u64 => "
      GLCDC.gr2_clut1()[14],
    ",
  0x40342c3cu64 => "
      GLCDC.gr2_clut1()[15],
    ",
  0x40342c40u64 => "
      GLCDC.gr2_clut1()[16],
    ",
  0x40342c44u64 => "
      GLCDC.gr2_clut1()[17],
    ",
  0x40342c48u64 => "
      GLCDC.gr2_clut1()[18],
    ",
  0x40342c4cu64 => "
      GLCDC.gr2_clut1()[19],
    ",
  0x40342c50u64 => "
      GLCDC.gr2_clut1()[20],
    ",
  0x40342c54u64 => "
      GLCDC.gr2_clut1()[21],
    ",
  0x40342c58u64 => "
      GLCDC.gr2_clut1()[22],
    ",
  0x40342c5cu64 => "
      GLCDC.gr2_clut1()[23],
    ",
  0x40342c60u64 => "
      GLCDC.gr2_clut1()[24],
    ",
  0x40342c64u64 => "
      GLCDC.gr2_clut1()[25],
    ",
  0x40342c68u64 => "
      GLCDC.gr2_clut1()[26],
    ",
  0x40342c6cu64 => "
      GLCDC.gr2_clut1()[27],
    ",
  0x40342c70u64 => "
      GLCDC.gr2_clut1()[28],
    ",
  0x40342c74u64 => "
      GLCDC.gr2_clut1()[29],
    ",
  0x40342c78u64 => "
      GLCDC.gr2_clut1()[30],
    ",
  0x40342c7cu64 => "
      GLCDC.gr2_clut1()[31],
    ",
  0x40342c80u64 => "
      GLCDC.gr2_clut1()[32],
    ",
  0x40342c84u64 => "
      GLCDC.gr2_clut1()[33],
    ",
  0x40342c88u64 => "
      GLCDC.gr2_clut1()[34],
    ",
  0x40342c8cu64 => "
      GLCDC.gr2_clut1()[35],
    ",
  0x40342c90u64 => "
      GLCDC.gr2_clut1()[36],
    ",
  0x40342c94u64 => "
      GLCDC.gr2_clut1()[37],
    ",
  0x40342c98u64 => "
      GLCDC.gr2_clut1()[38],
    ",
  0x40342c9cu64 => "
      GLCDC.gr2_clut1()[39],
    ",
  0x40342ca0u64 => "
      GLCDC.gr2_clut1()[40],
    ",
  0x40342ca4u64 => "
      GLCDC.gr2_clut1()[41],
    ",
  0x40342ca8u64 => "
      GLCDC.gr2_clut1()[42],
    ",
  0x40342cacu64 => "
      GLCDC.gr2_clut1()[43],
    ",
  0x40342cb0u64 => "
      GLCDC.gr2_clut1()[44],
    ",
  0x40342cb4u64 => "
      GLCDC.gr2_clut1()[45],
    ",
  0x40342cb8u64 => "
      GLCDC.gr2_clut1()[46],
    ",
  0x40342cbcu64 => "
      GLCDC.gr2_clut1()[47],
    ",
  0x40342cc0u64 => "
      GLCDC.gr2_clut1()[48],
    ",
  0x40342cc4u64 => "
      GLCDC.gr2_clut1()[49],
    ",
  0x40342cc8u64 => "
      GLCDC.gr2_clut1()[50],
    ",
  0x40342cccu64 => "
      GLCDC.gr2_clut1()[51],
    ",
  0x40342cd0u64 => "
      GLCDC.gr2_clut1()[52],
    ",
  0x40342cd4u64 => "
      GLCDC.gr2_clut1()[53],
    ",
  0x40342cd8u64 => "
      GLCDC.gr2_clut1()[54],
    ",
  0x40342cdcu64 => "
      GLCDC.gr2_clut1()[55],
    ",
  0x40342ce0u64 => "
      GLCDC.gr2_clut1()[56],
    ",
  0x40342ce4u64 => "
      GLCDC.gr2_clut1()[57],
    ",
  0x40342ce8u64 => "
      GLCDC.gr2_clut1()[58],
    ",
  0x40342cecu64 => "
      GLCDC.gr2_clut1()[59],
    ",
  0x40342cf0u64 => "
      GLCDC.gr2_clut1()[60],
    ",
  0x40342cf4u64 => "
      GLCDC.gr2_clut1()[61],
    ",
  0x40342cf8u64 => "
      GLCDC.gr2_clut1()[62],
    ",
  0x40342cfcu64 => "
      GLCDC.gr2_clut1()[63],
    ",
  0x40342d00u64 => "
      GLCDC.gr2_clut1()[64],
    ",
  0x40342d04u64 => "
      GLCDC.gr2_clut1()[65],
    ",
  0x40342d08u64 => "
      GLCDC.gr2_clut1()[66],
    ",
  0x40342d0cu64 => "
      GLCDC.gr2_clut1()[67],
    ",
  0x40342d10u64 => "
      GLCDC.gr2_clut1()[68],
    ",
  0x40342d14u64 => "
      GLCDC.gr2_clut1()[69],
    ",
  0x40342d18u64 => "
      GLCDC.gr2_clut1()[70],
    ",
  0x40342d1cu64 => "
      GLCDC.gr2_clut1()[71],
    ",
  0x40342d20u64 => "
      GLCDC.gr2_clut1()[72],
    ",
  0x40342d24u64 => "
      GLCDC.gr2_clut1()[73],
    ",
  0x40342d28u64 => "
      GLCDC.gr2_clut1()[74],
    ",
  0x40342d2cu64 => "
      GLCDC.gr2_clut1()[75],
    ",
  0x40342d30u64 => "
      GLCDC.gr2_clut1()[76],
    ",
  0x40342d34u64 => "
      GLCDC.gr2_clut1()[77],
    ",
  0x40342d38u64 => "
      GLCDC.gr2_clut1()[78],
    ",
  0x40342d3cu64 => "
      GLCDC.gr2_clut1()[79],
    ",
  0x40342d40u64 => "
      GLCDC.gr2_clut1()[80],
    ",
  0x40342d44u64 => "
      GLCDC.gr2_clut1()[81],
    ",
  0x40342d48u64 => "
      GLCDC.gr2_clut1()[82],
    ",
  0x40342d4cu64 => "
      GLCDC.gr2_clut1()[83],
    ",
  0x40342d50u64 => "
      GLCDC.gr2_clut1()[84],
    ",
  0x40342d54u64 => "
      GLCDC.gr2_clut1()[85],
    ",
  0x40342d58u64 => "
      GLCDC.gr2_clut1()[86],
    ",
  0x40342d5cu64 => "
      GLCDC.gr2_clut1()[87],
    ",
  0x40342d60u64 => "
      GLCDC.gr2_clut1()[88],
    ",
  0x40342d64u64 => "
      GLCDC.gr2_clut1()[89],
    ",
  0x40342d68u64 => "
      GLCDC.gr2_clut1()[90],
    ",
  0x40342d6cu64 => "
      GLCDC.gr2_clut1()[91],
    ",
  0x40342d70u64 => "
      GLCDC.gr2_clut1()[92],
    ",
  0x40342d74u64 => "
      GLCDC.gr2_clut1()[93],
    ",
  0x40342d78u64 => "
      GLCDC.gr2_clut1()[94],
    ",
  0x40342d7cu64 => "
      GLCDC.gr2_clut1()[95],
    ",
  0x40342d80u64 => "
      GLCDC.gr2_clut1()[96],
    ",
  0x40342d84u64 => "
      GLCDC.gr2_clut1()[97],
    ",
  0x40342d88u64 => "
      GLCDC.gr2_clut1()[98],
    ",
  0x40342d8cu64 => "
      GLCDC.gr2_clut1()[99],
    ",
  0x40342d90u64 => "
      GLCDC.gr2_clut1()[100],
    ",
  0x40342d94u64 => "
      GLCDC.gr2_clut1()[101],
    ",
  0x40342d98u64 => "
      GLCDC.gr2_clut1()[102],
    ",
  0x40342d9cu64 => "
      GLCDC.gr2_clut1()[103],
    ",
  0x40342da0u64 => "
      GLCDC.gr2_clut1()[104],
    ",
  0x40342da4u64 => "
      GLCDC.gr2_clut1()[105],
    ",
  0x40342da8u64 => "
      GLCDC.gr2_clut1()[106],
    ",
  0x40342dacu64 => "
      GLCDC.gr2_clut1()[107],
    ",
  0x40342db0u64 => "
      GLCDC.gr2_clut1()[108],
    ",
  0x40342db4u64 => "
      GLCDC.gr2_clut1()[109],
    ",
  0x40342db8u64 => "
      GLCDC.gr2_clut1()[110],
    ",
  0x40342dbcu64 => "
      GLCDC.gr2_clut1()[111],
    ",
  0x40342dc0u64 => "
      GLCDC.gr2_clut1()[112],
    ",
  0x40342dc4u64 => "
      GLCDC.gr2_clut1()[113],
    ",
  0x40342dc8u64 => "
      GLCDC.gr2_clut1()[114],
    ",
  0x40342dccu64 => "
      GLCDC.gr2_clut1()[115],
    ",
  0x40342dd0u64 => "
      GLCDC.gr2_clut1()[116],
    ",
  0x40342dd4u64 => "
      GLCDC.gr2_clut1()[117],
    ",
  0x40342dd8u64 => "
      GLCDC.gr2_clut1()[118],
    ",
  0x40342ddcu64 => "
      GLCDC.gr2_clut1()[119],
    ",
  0x40342de0u64 => "
      GLCDC.gr2_clut1()[120],
    ",
  0x40342de4u64 => "
      GLCDC.gr2_clut1()[121],
    ",
  0x40342de8u64 => "
      GLCDC.gr2_clut1()[122],
    ",
  0x40342decu64 => "
      GLCDC.gr2_clut1()[123],
    ",
  0x40342df0u64 => "
      GLCDC.gr2_clut1()[124],
    ",
  0x40342df4u64 => "
      GLCDC.gr2_clut1()[125],
    ",
  0x40342df8u64 => "
      GLCDC.gr2_clut1()[126],
    ",
  0x40342dfcu64 => "
      GLCDC.gr2_clut1()[127],
    ",
  0x40342e00u64 => "
      GLCDC.gr2_clut1()[128],
    ",
  0x40342e04u64 => "
      GLCDC.gr2_clut1()[129],
    ",
  0x40342e08u64 => "
      GLCDC.gr2_clut1()[130],
    ",
  0x40342e0cu64 => "
      GLCDC.gr2_clut1()[131],
    ",
  0x40342e10u64 => "
      GLCDC.gr2_clut1()[132],
    ",
  0x40342e14u64 => "
      GLCDC.gr2_clut1()[133],
    ",
  0x40342e18u64 => "
      GLCDC.gr2_clut1()[134],
    ",
  0x40342e1cu64 => "
      GLCDC.gr2_clut1()[135],
    ",
  0x40342e20u64 => "
      GLCDC.gr2_clut1()[136],
    ",
  0x40342e24u64 => "
      GLCDC.gr2_clut1()[137],
    ",
  0x40342e28u64 => "
      GLCDC.gr2_clut1()[138],
    ",
  0x40342e2cu64 => "
      GLCDC.gr2_clut1()[139],
    ",
  0x40342e30u64 => "
      GLCDC.gr2_clut1()[140],
    ",
  0x40342e34u64 => "
      GLCDC.gr2_clut1()[141],
    ",
  0x40342e38u64 => "
      GLCDC.gr2_clut1()[142],
    ",
  0x40342e3cu64 => "
      GLCDC.gr2_clut1()[143],
    ",
  0x40342e40u64 => "
      GLCDC.gr2_clut1()[144],
    ",
  0x40342e44u64 => "
      GLCDC.gr2_clut1()[145],
    ",
  0x40342e48u64 => "
      GLCDC.gr2_clut1()[146],
    ",
  0x40342e4cu64 => "
      GLCDC.gr2_clut1()[147],
    ",
  0x40342e50u64 => "
      GLCDC.gr2_clut1()[148],
    ",
  0x40342e54u64 => "
      GLCDC.gr2_clut1()[149],
    ",
  0x40342e58u64 => "
      GLCDC.gr2_clut1()[150],
    ",
  0x40342e5cu64 => "
      GLCDC.gr2_clut1()[151],
    ",
  0x40342e60u64 => "
      GLCDC.gr2_clut1()[152],
    ",
  0x40342e64u64 => "
      GLCDC.gr2_clut1()[153],
    ",
  0x40342e68u64 => "
      GLCDC.gr2_clut1()[154],
    ",
  0x40342e6cu64 => "
      GLCDC.gr2_clut1()[155],
    ",
  0x40342e70u64 => "
      GLCDC.gr2_clut1()[156],
    ",
  0x40342e74u64 => "
      GLCDC.gr2_clut1()[157],
    ",
  0x40342e78u64 => "
      GLCDC.gr2_clut1()[158],
    ",
  0x40342e7cu64 => "
      GLCDC.gr2_clut1()[159],
    ",
  0x40342e80u64 => "
      GLCDC.gr2_clut1()[160],
    ",
  0x40342e84u64 => "
      GLCDC.gr2_clut1()[161],
    ",
  0x40342e88u64 => "
      GLCDC.gr2_clut1()[162],
    ",
  0x40342e8cu64 => "
      GLCDC.gr2_clut1()[163],
    ",
  0x40342e90u64 => "
      GLCDC.gr2_clut1()[164],
    ",
  0x40342e94u64 => "
      GLCDC.gr2_clut1()[165],
    ",
  0x40342e98u64 => "
      GLCDC.gr2_clut1()[166],
    ",
  0x40342e9cu64 => "
      GLCDC.gr2_clut1()[167],
    ",
  0x40342ea0u64 => "
      GLCDC.gr2_clut1()[168],
    ",
  0x40342ea4u64 => "
      GLCDC.gr2_clut1()[169],
    ",
  0x40342ea8u64 => "
      GLCDC.gr2_clut1()[170],
    ",
  0x40342eacu64 => "
      GLCDC.gr2_clut1()[171],
    ",
  0x40342eb0u64 => "
      GLCDC.gr2_clut1()[172],
    ",
  0x40342eb4u64 => "
      GLCDC.gr2_clut1()[173],
    ",
  0x40342eb8u64 => "
      GLCDC.gr2_clut1()[174],
    ",
  0x40342ebcu64 => "
      GLCDC.gr2_clut1()[175],
    ",
  0x40342ec0u64 => "
      GLCDC.gr2_clut1()[176],
    ",
  0x40342ec4u64 => "
      GLCDC.gr2_clut1()[177],
    ",
  0x40342ec8u64 => "
      GLCDC.gr2_clut1()[178],
    ",
  0x40342eccu64 => "
      GLCDC.gr2_clut1()[179],
    ",
  0x40342ed0u64 => "
      GLCDC.gr2_clut1()[180],
    ",
  0x40342ed4u64 => "
      GLCDC.gr2_clut1()[181],
    ",
  0x40342ed8u64 => "
      GLCDC.gr2_clut1()[182],
    ",
  0x40342edcu64 => "
      GLCDC.gr2_clut1()[183],
    ",
  0x40342ee0u64 => "
      GLCDC.gr2_clut1()[184],
    ",
  0x40342ee4u64 => "
      GLCDC.gr2_clut1()[185],
    ",
  0x40342ee8u64 => "
      GLCDC.gr2_clut1()[186],
    ",
  0x40342eecu64 => "
      GLCDC.gr2_clut1()[187],
    ",
  0x40342ef0u64 => "
      GLCDC.gr2_clut1()[188],
    ",
  0x40342ef4u64 => "
      GLCDC.gr2_clut1()[189],
    ",
  0x40342ef8u64 => "
      GLCDC.gr2_clut1()[190],
    ",
  0x40342efcu64 => "
      GLCDC.gr2_clut1()[191],
    ",
  0x40342f00u64 => "
      GLCDC.gr2_clut1()[192],
    ",
  0x40342f04u64 => "
      GLCDC.gr2_clut1()[193],
    ",
  0x40342f08u64 => "
      GLCDC.gr2_clut1()[194],
    ",
  0x40342f0cu64 => "
      GLCDC.gr2_clut1()[195],
    ",
  0x40342f10u64 => "
      GLCDC.gr2_clut1()[196],
    ",
  0x40342f14u64 => "
      GLCDC.gr2_clut1()[197],
    ",
  0x40342f18u64 => "
      GLCDC.gr2_clut1()[198],
    ",
  0x40342f1cu64 => "
      GLCDC.gr2_clut1()[199],
    ",
  0x40342f20u64 => "
      GLCDC.gr2_clut1()[200],
    ",
  0x40342f24u64 => "
      GLCDC.gr2_clut1()[201],
    ",
  0x40342f28u64 => "
      GLCDC.gr2_clut1()[202],
    ",
  0x40342f2cu64 => "
      GLCDC.gr2_clut1()[203],
    ",
  0x40342f30u64 => "
      GLCDC.gr2_clut1()[204],
    ",
  0x40342f34u64 => "
      GLCDC.gr2_clut1()[205],
    ",
  0x40342f38u64 => "
      GLCDC.gr2_clut1()[206],
    ",
  0x40342f3cu64 => "
      GLCDC.gr2_clut1()[207],
    ",
  0x40342f40u64 => "
      GLCDC.gr2_clut1()[208],
    ",
  0x40342f44u64 => "
      GLCDC.gr2_clut1()[209],
    ",
  0x40342f48u64 => "
      GLCDC.gr2_clut1()[210],
    ",
  0x40342f4cu64 => "
      GLCDC.gr2_clut1()[211],
    ",
  0x40342f50u64 => "
      GLCDC.gr2_clut1()[212],
    ",
  0x40342f54u64 => "
      GLCDC.gr2_clut1()[213],
    ",
  0x40342f58u64 => "
      GLCDC.gr2_clut1()[214],
    ",
  0x40342f5cu64 => "
      GLCDC.gr2_clut1()[215],
    ",
  0x40342f60u64 => "
      GLCDC.gr2_clut1()[216],
    ",
  0x40342f64u64 => "
      GLCDC.gr2_clut1()[217],
    ",
  0x40342f68u64 => "
      GLCDC.gr2_clut1()[218],
    ",
  0x40342f6cu64 => "
      GLCDC.gr2_clut1()[219],
    ",
  0x40342f70u64 => "
      GLCDC.gr2_clut1()[220],
    ",
  0x40342f74u64 => "
      GLCDC.gr2_clut1()[221],
    ",
  0x40342f78u64 => "
      GLCDC.gr2_clut1()[222],
    ",
  0x40342f7cu64 => "
      GLCDC.gr2_clut1()[223],
    ",
  0x40342f80u64 => "
      GLCDC.gr2_clut1()[224],
    ",
  0x40342f84u64 => "
      GLCDC.gr2_clut1()[225],
    ",
  0x40342f88u64 => "
      GLCDC.gr2_clut1()[226],
    ",
  0x40342f8cu64 => "
      GLCDC.gr2_clut1()[227],
    ",
  0x40342f90u64 => "
      GLCDC.gr2_clut1()[228],
    ",
  0x40342f94u64 => "
      GLCDC.gr2_clut1()[229],
    ",
  0x40342f98u64 => "
      GLCDC.gr2_clut1()[230],
    ",
  0x40342f9cu64 => "
      GLCDC.gr2_clut1()[231],
    ",
  0x40342fa0u64 => "
      GLCDC.gr2_clut1()[232],
    ",
  0x40342fa4u64 => "
      GLCDC.gr2_clut1()[233],
    ",
  0x40342fa8u64 => "
      GLCDC.gr2_clut1()[234],
    ",
  0x40342facu64 => "
      GLCDC.gr2_clut1()[235],
    ",
  0x40342fb0u64 => "
      GLCDC.gr2_clut1()[236],
    ",
  0x40342fb4u64 => "
      GLCDC.gr2_clut1()[237],
    ",
  0x40342fb8u64 => "
      GLCDC.gr2_clut1()[238],
    ",
  0x40342fbcu64 => "
      GLCDC.gr2_clut1()[239],
    ",
  0x40342fc0u64 => "
      GLCDC.gr2_clut1()[240],
    ",
  0x40342fc4u64 => "
      GLCDC.gr2_clut1()[241],
    ",
  0x40342fc8u64 => "
      GLCDC.gr2_clut1()[242],
    ",
  0x40342fccu64 => "
      GLCDC.gr2_clut1()[243],
    ",
  0x40342fd0u64 => "
      GLCDC.gr2_clut1()[244],
    ",
  0x40342fd4u64 => "
      GLCDC.gr2_clut1()[245],
    ",
  0x40342fd8u64 => "
      GLCDC.gr2_clut1()[246],
    ",
  0x40342fdcu64 => "
      GLCDC.gr2_clut1()[247],
    ",
  0x40342fe0u64 => "
      GLCDC.gr2_clut1()[248],
    ",
  0x40342fe4u64 => "
      GLCDC.gr2_clut1()[249],
    ",
  0x40342fe8u64 => "
      GLCDC.gr2_clut1()[250],
    ",
  0x40342fecu64 => "
      GLCDC.gr2_clut1()[251],
    ",
  0x40342ff0u64 => "
      GLCDC.gr2_clut1()[252],
    ",
  0x40342ff4u64 => "
      GLCDC.gr2_clut1()[253],
    ",
  0x40342ff8u64 => "
      GLCDC.gr2_clut1()[254],
    ",
  0x40342ffcu64 => "
      GLCDC.gr2_clut1()[255],
    ",
  0x40343000u64 => "
      GLCDC.bg_en(),
    ",
  0x40343004u64 => "
      GLCDC.bg_peri(),
    ",
  0x40343008u64 => "
      GLCDC.bg_sync(),
    ",
  0x4034300cu64 => "
      GLCDC.bg_vsize(),
    ",
  0x40343010u64 => "
      GLCDC.bg_hsize(),
    ",
  0x40343014u64 => "
      GLCDC.bg_bgc(),
    ",
  0x40343018u64 => "
      GLCDC.bg_mon(),
    ",
  0x40343100u64 => "
      GLCDC.gr_ven()[0],
    ",
  0x40343200u64 => "
      GLCDC.gr_ven()[1],
    ",
  0x40343104u64 => "
      GLCDC.gr_flmrd()[0],
    ",
  0x40343204u64 => "
      GLCDC.gr_flmrd()[1],
    ",
  0x40343108u64 => "
      GLCDC.gr_flm1()[0],
    ",
  0x40343208u64 => "
      GLCDC.gr_flm1()[1],
    ",
  0x4034310cu64 => "
      GLCDC.gr_flm2()[0],
    ",
  0x4034320cu64 => "
      GLCDC.gr_flm2()[1],
    ",
  0x40343110u64 => "
      GLCDC.gr_flm3()[0],
    ",
  0x40343210u64 => "
      GLCDC.gr_flm3()[1],
    ",
  0x40343118u64 => "
      GLCDC.gr_flm5()[0],
    ",
  0x40343218u64 => "
      GLCDC.gr_flm5()[1],
    ",
  0x4034311cu64 => "
      GLCDC.gr_flm6()[0],
    ",
  0x4034321cu64 => "
      GLCDC.gr_flm6()[1],
    ",
  0x40343120u64 => "
      GLCDC.gr_ab1()[0],
    ",
  0x40343220u64 => "
      GLCDC.gr_ab1()[1],
    ",
  0x40343124u64 => "
      GLCDC.gr_ab2()[0],
    ",
  0x40343224u64 => "
      GLCDC.gr_ab2()[1],
    ",
  0x40343128u64 => "
      GLCDC.gr_ab3()[0],
    ",
  0x40343228u64 => "
      GLCDC.gr_ab3()[1],
    ",
  0x4034312cu64 => "
      GLCDC.gr_ab4()[0],
    ",
  0x4034322cu64 => "
      GLCDC.gr_ab4()[1],
    ",
  0x40343130u64 => "
      GLCDC.gr_ab5()[0],
    ",
  0x40343230u64 => "
      GLCDC.gr_ab5()[1],
    ",
  0x40343134u64 => "
      GLCDC.gr_ab6()[0],
    ",
  0x40343234u64 => "
      GLCDC.gr_ab6()[1],
    ",
  0x40343138u64 => "
      GLCDC.gr_ab7()[0],
    ",
  0x40343238u64 => "
      GLCDC.gr_ab7()[1],
    ",
  0x4034313cu64 => "
      GLCDC.gr_ab8()[0],
    ",
  0x4034323cu64 => "
      GLCDC.gr_ab8()[1],
    ",
  0x40343140u64 => "
      GLCDC.gr_ab9()[0],
    ",
  0x40343240u64 => "
      GLCDC.gr_ab9()[1],
    ",
  0x4034314cu64 => "
      GLCDC.gr_base()[0],
    ",
  0x4034324cu64 => "
      GLCDC.gr_base()[1],
    ",
  0x40343150u64 => "
      GLCDC.gr_clutint()[0],
    ",
  0x40343250u64 => "
      GLCDC.gr_clutint()[1],
    ",
  0x40343154u64 => "
      GLCDC.gr_mon()[0],
    ",
  0x40343254u64 => "
      GLCDC.gr_mon()[1],
    ",
  0x40343300u64 => "
      GLCDC.gamg_latch(),
    ",
  0x40343304u64 => "
      GLCDC.gam_sw(),
    ",
  0x40343308u64 => "
      GLCDC.gamg_lut1(),
    ",
  0x4034330cu64 => "
      GLCDC.gamg_lut2(),
    ",
  0x40343310u64 => "
      GLCDC.gamg_lut3(),
    ",
  0x40343314u64 => "
      GLCDC.gamg_lut4(),
    ",
  0x40343318u64 => "
      GLCDC.gamg_lut5(),
    ",
  0x4034331cu64 => "
      GLCDC.gamg_lut6(),
    ",
  0x40343320u64 => "
      GLCDC.gamg_lut7(),
    ",
  0x40343324u64 => "
      GLCDC.gamg_lut8(),
    ",
  0x40343328u64 => "
      GLCDC.gamg_area1(),
    ",
  0x4034332cu64 => "
      GLCDC.gamg_area2(),
    ",
  0x40343330u64 => "
      GLCDC.gamg_area3(),
    ",
  0x40343334u64 => "
      GLCDC.gamg_area4(),
    ",
  0x40343338u64 => "
      GLCDC.gamg_area5(),
    ",
  0x40343340u64 => "
      GLCDC.gamb_latch(),
    ",
  0x40343348u64 => "
      GLCDC.gamb_lut1(),
    ",
  0x4034334cu64 => "
      GLCDC.gamb_lut2(),
    ",
  0x40343350u64 => "
      GLCDC.gamb_lut3(),
    ",
  0x40343354u64 => "
      GLCDC.gamb_lut4(),
    ",
  0x40343358u64 => "
      GLCDC.gamb_lut5(),
    ",
  0x4034335cu64 => "
      GLCDC.gamb_lut6(),
    ",
  0x40343360u64 => "
      GLCDC.gamb_lut7(),
    ",
  0x40343364u64 => "
      GLCDC.gamb_lut8(),
    ",
  0x40343368u64 => "
      GLCDC.gamb_area1(),
    ",
  0x4034336cu64 => "
      GLCDC.gamb_area2(),
    ",
  0x40343370u64 => "
      GLCDC.gamb_area3(),
    ",
  0x40343374u64 => "
      GLCDC.gamb_area4(),
    ",
  0x40343378u64 => "
      GLCDC.gamb_area5(),
    ",
  0x40343380u64 => "
      GLCDC.gamr_latch(),
    ",
  0x40343388u64 => "
      GLCDC.gamr_lut1(),
    ",
  0x4034338cu64 => "
      GLCDC.gamr_lut2(),
    ",
  0x40343390u64 => "
      GLCDC.gamr_lut3(),
    ",
  0x40343394u64 => "
      GLCDC.gamr_lut4(),
    ",
  0x40343398u64 => "
      GLCDC.gamr_lut5(),
    ",
  0x4034339cu64 => "
      GLCDC.gamr_lut6(),
    ",
  0x403433a0u64 => "
      GLCDC.gamr_lut7(),
    ",
  0x403433a4u64 => "
      GLCDC.gamr_lut8(),
    ",
  0x403433a8u64 => "
      GLCDC.gamr_area1(),
    ",
  0x403433acu64 => "
      GLCDC.gamr_area2(),
    ",
  0x403433b0u64 => "
      GLCDC.gamr_area3(),
    ",
  0x403433b4u64 => "
      GLCDC.gamr_area4(),
    ",
  0x403433b8u64 => "
      GLCDC.gamr_area5(),
    ",
  0x403433c0u64 => "
      GLCDC.out_vlatch(),
    ",
  0x403433c4u64 => "
      GLCDC.out_set(),
    ",
  0x403433c8u64 => "
      GLCDC.out_bright1(),
    ",
  0x403433ccu64 => "
      GLCDC.out_bright2(),
    ",
  0x403433d0u64 => "
      GLCDC.out_contrast(),
    ",
  0x403433d4u64 => "
      GLCDC.out_pdtha(),
    ",
  0x403433e4u64 => "
      GLCDC.out_clkphase(),
    ",
  0x40343404u64 => "
      GLCDC.tcon_tim(),
    ",
  0x40343408u64 => "
      GLCDC.tcon_stva1(),
    ",
  0x4034340cu64 => "
      GLCDC.tcon_stva2(),
    ",
  0x40343410u64 => "
      GLCDC.tcon_stvb1(),
    ",
  0x40343414u64 => "
      GLCDC.tcon_stvb2(),
    ",
  0x40343418u64 => "
      GLCDC.tcon_stha1(),
    ",
  0x4034341cu64 => "
      GLCDC.tcon_stha2(),
    ",
  0x40343420u64 => "
      GLCDC.tcon_sthb1(),
    ",
  0x40343424u64 => "
      GLCDC.tcon_sthb2(),
    ",
  0x40343428u64 => "
      GLCDC.tcon_de(),
    ",
  0x40343440u64 => "
      GLCDC.syscnt_dtcten(),
    ",
  0x40343444u64 => "
      GLCDC.syscnt_inten(),
    ",
  0x40343448u64 => "
      GLCDC.syscnt_stclr(),
    ",
  0x4034344cu64 => "
      GLCDC.syscnt_stmon(),
    ",
  0x40343450u64 => "
      GLCDC.syscnt_panel_clk(),
    ",
  0x40344000u64 => "
      DRW.control(),
      DRW.status(),
    ",
  0x40344004u64 => "
      DRW.control2(),
      DRW.hwrevision(),
    ",
  0x40344010u64 => "
      DRW.lstart()[0],
    ",
  0x40344014u64 => "
      DRW.lstart()[1],
    ",
  0x40344018u64 => "
      DRW.lstart()[2],
    ",
  0x4034401cu64 => "
      DRW.lstart()[3],
    ",
  0x40344020u64 => "
      DRW.lstart()[4],
    ",
  0x40344024u64 => "
      DRW.lstart()[5],
    ",
  0x40344028u64 => "
      DRW.lxadd()[0],
    ",
  0x4034402cu64 => "
      DRW.lxadd()[1],
    ",
  0x40344030u64 => "
      DRW.lxadd()[2],
    ",
  0x40344034u64 => "
      DRW.lxadd()[3],
    ",
  0x40344038u64 => "
      DRW.lxadd()[4],
    ",
  0x4034403cu64 => "
      DRW.lxadd()[5],
    ",
  0x40344040u64 => "
      DRW.lyadd()[0],
    ",
  0x40344044u64 => "
      DRW.lyadd()[1],
    ",
  0x40344048u64 => "
      DRW.lyadd()[2],
    ",
  0x4034404cu64 => "
      DRW.lyadd()[3],
    ",
  0x40344050u64 => "
      DRW.lyadd()[4],
    ",
  0x40344054u64 => "
      DRW.lyadd()[5],
    ",
  0x40344058u64 => "
      DRW.lband()[0],
    ",
  0x4034405cu64 => "
      DRW.lband()[1],
    ",
  0x40344064u64 => "
      DRW.color1(),
    ",
  0x40344068u64 => "
      DRW.color2(),
    ",
  0x40344074u64 => "
      DRW.pattern(),
    ",
  0x40344078u64 => "
      DRW.size(),
    ",
  0x4034407cu64 => "
      DRW.pitch(),
    ",
  0x40344080u64 => "
      DRW.origin(),
    ",
  0x40344090u64 => "
      DRW.lustart(),
    ",
  0x40344094u64 => "
      DRW.luxadd(),
    ",
  0x40344098u64 => "
      DRW.luyadd(),
    ",
  0x4034409cu64 => "
      DRW.lvstarti(),
    ",
  0x403440a0u64 => "
      DRW.lvstartf(),
    ",
  0x403440a4u64 => "
      DRW.lvxaddi(),
    ",
  0x403440a8u64 => "
      DRW.lvyaddi(),
    ",
  0x403440acu64 => "
      DRW.lvyxaddf(),
    ",
  0x403440b4u64 => "
      DRW.texpitch(),
    ",
  0x403440b8u64 => "
      DRW.texmask(),
    ",
  0x403440bcu64 => "
      DRW.texorigin(),
    ",
  0x403440c0u64 => "
      DRW.irqctl(),
    ",
  0x403440c4u64 => "
      DRW.cachectl(),
    ",
  0x403440c8u64 => "
      DRW.dliststart(),
    ",
  0x403440ccu64 => "
      DRW.perfcount1(),
      DRW.perfcount2(),
    ",
  0x403440d4u64 => "
      DRW.perftrigger(),
    ",
  0x403440dcu64 => "
      DRW.texcladdr(),
    ",
  0x403440e0u64 => "
      DRW.texcldata(),
    ",
  0x403440e4u64 => "
      DRW.texcloffset(),
    ",
  0x403440e8u64 => "
      DRW.colkey(),
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
  0x50000700u64 => "
      RMPU_NS.mmpuenglcdc(),
    ",
  0x50000704u64 => "
      RMPU_NS.mmpuenptglcdc(),
    ",
  0x50000708u64 => "
      RMPU_NS.mmpurptglcdc(),
    ",
  0x50000800u64 => "
      RMPU_NS.mmpuacglcdc()[0],
    ",
  0x50000810u64 => "
      RMPU_NS.mmpuacglcdc()[1],
    ",
  0x50000804u64 => "
      RMPU_NS.mmpusglcdc()[0],
    ",
  0x50000814u64 => "
      RMPU_NS.mmpusglcdc()[1],
    ",
  0x50000808u64 => "
      RMPU_NS.mmpueglcdc()[0],
    ",
  0x50000818u64 => "
      RMPU_NS.mmpueglcdc()[1],
    ",
  0x50000900u64 => "
      RMPU_NS.mmpuendrw(),
    ",
  0x50000904u64 => "
      RMPU_NS.mmpuenpdrw(),
    ",
  0x50000908u64 => "
      RMPU_NS.mmpurptdrw(),
    ",
  0x50000a00u64 => "
      RMPU_NS.mmpuacdrw()[0],
    ",
  0x50000a10u64 => "
      RMPU_NS.mmpuacdrw()[1],
    ",
  0x50000a20u64 => "
      RMPU_NS.mmpuacdrw()[2],
    ",
  0x50000a04u64 => "
      RMPU_NS.mmpusdrw()[0],
    ",
  0x50000a14u64 => "
      RMPU_NS.mmpusdrw()[1],
    ",
  0x50000a24u64 => "
      RMPU_NS.mmpusdrw()[2],
    ",
  0x50000a08u64 => "
      RMPU_NS.mmpuedrw()[0],
    ",
  0x50000a18u64 => "
      RMPU_NS.mmpuedrw()[1],
    ",
  0x50000a28u64 => "
      RMPU_NS.mmpuedrw()[2],
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
  0x50003002u64 => "
      BUS_NS.csmod()[0],
    ",
  0x50003012u64 => "
      BUS_NS.csmod()[1],
    ",
  0x50003022u64 => "
      BUS_NS.csmod()[2],
    ",
  0x50003032u64 => "
      BUS_NS.csmod()[3],
    ",
  0x50003042u64 => "
      BUS_NS.csmod()[4],
    ",
  0x50003052u64 => "
      BUS_NS.csmod()[5],
    ",
  0x50003062u64 => "
      BUS_NS.csmod()[6],
    ",
  0x50003072u64 => "
      BUS_NS.csmod()[7],
    ",
  0x50003004u64 => "
      BUS_NS.cswcr1()[0],
    ",
  0x50003014u64 => "
      BUS_NS.cswcr1()[1],
    ",
  0x50003024u64 => "
      BUS_NS.cswcr1()[2],
    ",
  0x50003034u64 => "
      BUS_NS.cswcr1()[3],
    ",
  0x50003044u64 => "
      BUS_NS.cswcr1()[4],
    ",
  0x50003054u64 => "
      BUS_NS.cswcr1()[5],
    ",
  0x50003064u64 => "
      BUS_NS.cswcr1()[6],
    ",
  0x50003074u64 => "
      BUS_NS.cswcr1()[7],
    ",
  0x50003008u64 => "
      BUS_NS.cswcr2()[0],
    ",
  0x50003018u64 => "
      BUS_NS.cswcr2()[1],
    ",
  0x50003028u64 => "
      BUS_NS.cswcr2()[2],
    ",
  0x50003038u64 => "
      BUS_NS.cswcr2()[3],
    ",
  0x50003048u64 => "
      BUS_NS.cswcr2()[4],
    ",
  0x50003058u64 => "
      BUS_NS.cswcr2()[5],
    ",
  0x50003068u64 => "
      BUS_NS.cswcr2()[6],
    ",
  0x50003078u64 => "
      BUS_NS.cswcr2()[7],
    ",
  0x50003802u64 => "
      BUS_NS.cs0cr(),
    ",
  0x5000380au64 => "
      BUS_NS.csrec()[0],
    ",
  0x5000381au64 => "
      BUS_NS.csrec()[1],
    ",
  0x5000382au64 => "
      BUS_NS.csrec()[2],
    ",
  0x5000383au64 => "
      BUS_NS.csrec()[3],
    ",
  0x5000384au64 => "
      BUS_NS.csrec()[4],
    ",
  0x5000385au64 => "
      BUS_NS.csrec()[5],
    ",
  0x5000386au64 => "
      BUS_NS.csrec()[6],
    ",
  0x5000387au64 => "
      BUS_NS.csrec()[7],
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
  0x50003880u64 => "
      BUS_NS.csrecen(),
    ",
  0x50003c00u64 => "
      BUS_NS.sdccr(),
    ",
  0x50003c01u64 => "
      BUS_NS.sdcmod(),
    ",
  0x50003c02u64 => "
      BUS_NS.sdamod(),
    ",
  0x50003c10u64 => "
      BUS_NS.sdself(),
    ",
  0x50003c14u64 => "
      BUS_NS.sdrfcr(),
    ",
  0x50003c16u64 => "
      BUS_NS.sdrfen(),
    ",
  0x50003c20u64 => "
      BUS_NS.sdicr(),
    ",
  0x50003c24u64 => "
      BUS_NS.sdir(),
    ",
  0x50003c40u64 => "
      BUS_NS.sdadr(),
    ",
  0x50003c44u64 => "
      BUS_NS.sdtr(),
    ",
  0x50003c48u64 => "
      BUS_NS.sdmod(),
    ",
  0x50003c50u64 => "
      BUS_NS.sdsr(),
    ",
  0x50004000u64 => "
      BUS_NS.busoad(),
    ",
  0x50004004u64 => "
      BUS_NS.busoadpt(),
    ",
  0x50004100u64 => "
      BUS_NS.busmabt(),
    ",
  0x50004200u64 => "
      BUS_NS.bussabt1fhbi(),
    ",
  0x50004210u64 => "
      BUS_NS.bussabt0flbi(),
    ",
  0x50004218u64 => "
      BUS_NS.bussabt1s0bi(),
    ",
  0x50004220u64 => "
      BUS_NS.bussabt1s1bi(),
    ",
  0x50004248u64 => "
      BUS_NS.bussabt0stbysbi(),
    ",
  0x50004250u64 => "
      BUS_NS.bussabt0ecbi(),
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
  0x50004824u64 => "
      BUS_NS.buserrrw()[0],
    ",
  0x50004834u64 => "
      BUS_NS.buserrrw()[1],
    ",
  0x50004844u64 => "
      BUS_NS.buserrrw()[2],
    ",
  0x50004854u64 => "
      BUS_NS.buserrrw()[3],
    ",
  0x50004850u64 => "
      BUS_NS.buserradd()[0],
    ",
  0x50004860u64 => "
      BUS_NS.buserradd()[1],
    ",
  0x50004870u64 => "
      BUS_NS.buserradd()[2],
    ",
  0x50004880u64 => "
      BUS_NS.buserradd()[3],
    ",
  0x50004920u64 => "
      BUS_NS.bmsaerradd()[0],
    ",
  0x50004930u64 => "
      BUS_NS.bmsaerradd()[1],
    ",
  0x50004940u64 => "
      BUS_NS.bmsaerradd()[2],
    ",
  0x50004950u64 => "
      BUS_NS.bmsaerradd()[3],
    ",
  0x50004924u64 => "
      BUS_NS.bmsaerrrw()[0],
    ",
  0x50004934u64 => "
      BUS_NS.bmsaerrrw()[1],
    ",
  0x50004944u64 => "
      BUS_NS.bmsaerrrw()[2],
    ",
  0x50004954u64 => "
      BUS_NS.bmsaerrrw()[3],
    ",
  0x50004a50u64 => "
      BUS_NS.buserrstat()[0],
    ",
  0x50004a60u64 => "
      BUS_NS.buserrstat()[1],
    ",
  0x50004a70u64 => "
      BUS_NS.buserrstat()[2],
    ",
  0x50004a80u64 => "
      BUS_NS.buserrstat()[3],
    ",
  0x50004a58u64 => "
      BUS_NS.buserrclr()[0],
    ",
  0x50004a68u64 => "
      BUS_NS.buserrclr()[1],
    ",
  0x50004a78u64 => "
      BUS_NS.buserrclr()[2],
    ",
  0x50004a88u64 => "
      BUS_NS.buserrclr()[3],
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
  0x50008110u64 => "
      CPSCU_NS.bussarc(),
    ",
  0x50008114u64 => "
      CPSCU_NS.busparc(),
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
  0x50008400u64 => "
      CPSCU_NS.sramsabar()[0],
    ",
  0x50008404u64 => "
      CPSCU_NS.sramsabar()[1],
    ",
  0x50008408u64 => "
      CPSCU_NS.sramsabar()[2],
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
  0x5001e00cu64 => "
      SYSC_NS.sbycr(),
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
  0x5001e030u64 => "
      SYSC_NS.bckcr(),
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
  0x5001e052u64 => "
      SYSC_NS.ebckocr(),
    ",
  0x5001e053u64 => "
      SYSC_NS.sdckocr(),
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
  0x5001e05eu64 => "
      SYSC_NS.lcdckdivcr(),
    ",
  0x5001e05fu64 => "
      SYSC_NS.lcdckcr(),
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
  0x5001e110u64 => "
      SYSC_NS.pdctrgd(),
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
  0x5001e3d8u64 => "
      SYSC_NS.pgcsar(),
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
  0x50342000u64 => "
      GLCDC_NS.gr1_clut0()[0],
    ",
  0x50342004u64 => "
      GLCDC_NS.gr1_clut0()[1],
    ",
  0x50342008u64 => "
      GLCDC_NS.gr1_clut0()[2],
    ",
  0x5034200cu64 => "
      GLCDC_NS.gr1_clut0()[3],
    ",
  0x50342010u64 => "
      GLCDC_NS.gr1_clut0()[4],
    ",
  0x50342014u64 => "
      GLCDC_NS.gr1_clut0()[5],
    ",
  0x50342018u64 => "
      GLCDC_NS.gr1_clut0()[6],
    ",
  0x5034201cu64 => "
      GLCDC_NS.gr1_clut0()[7],
    ",
  0x50342020u64 => "
      GLCDC_NS.gr1_clut0()[8],
    ",
  0x50342024u64 => "
      GLCDC_NS.gr1_clut0()[9],
    ",
  0x50342028u64 => "
      GLCDC_NS.gr1_clut0()[10],
    ",
  0x5034202cu64 => "
      GLCDC_NS.gr1_clut0()[11],
    ",
  0x50342030u64 => "
      GLCDC_NS.gr1_clut0()[12],
    ",
  0x50342034u64 => "
      GLCDC_NS.gr1_clut0()[13],
    ",
  0x50342038u64 => "
      GLCDC_NS.gr1_clut0()[14],
    ",
  0x5034203cu64 => "
      GLCDC_NS.gr1_clut0()[15],
    ",
  0x50342040u64 => "
      GLCDC_NS.gr1_clut0()[16],
    ",
  0x50342044u64 => "
      GLCDC_NS.gr1_clut0()[17],
    ",
  0x50342048u64 => "
      GLCDC_NS.gr1_clut0()[18],
    ",
  0x5034204cu64 => "
      GLCDC_NS.gr1_clut0()[19],
    ",
  0x50342050u64 => "
      GLCDC_NS.gr1_clut0()[20],
    ",
  0x50342054u64 => "
      GLCDC_NS.gr1_clut0()[21],
    ",
  0x50342058u64 => "
      GLCDC_NS.gr1_clut0()[22],
    ",
  0x5034205cu64 => "
      GLCDC_NS.gr1_clut0()[23],
    ",
  0x50342060u64 => "
      GLCDC_NS.gr1_clut0()[24],
    ",
  0x50342064u64 => "
      GLCDC_NS.gr1_clut0()[25],
    ",
  0x50342068u64 => "
      GLCDC_NS.gr1_clut0()[26],
    ",
  0x5034206cu64 => "
      GLCDC_NS.gr1_clut0()[27],
    ",
  0x50342070u64 => "
      GLCDC_NS.gr1_clut0()[28],
    ",
  0x50342074u64 => "
      GLCDC_NS.gr1_clut0()[29],
    ",
  0x50342078u64 => "
      GLCDC_NS.gr1_clut0()[30],
    ",
  0x5034207cu64 => "
      GLCDC_NS.gr1_clut0()[31],
    ",
  0x50342080u64 => "
      GLCDC_NS.gr1_clut0()[32],
    ",
  0x50342084u64 => "
      GLCDC_NS.gr1_clut0()[33],
    ",
  0x50342088u64 => "
      GLCDC_NS.gr1_clut0()[34],
    ",
  0x5034208cu64 => "
      GLCDC_NS.gr1_clut0()[35],
    ",
  0x50342090u64 => "
      GLCDC_NS.gr1_clut0()[36],
    ",
  0x50342094u64 => "
      GLCDC_NS.gr1_clut0()[37],
    ",
  0x50342098u64 => "
      GLCDC_NS.gr1_clut0()[38],
    ",
  0x5034209cu64 => "
      GLCDC_NS.gr1_clut0()[39],
    ",
  0x503420a0u64 => "
      GLCDC_NS.gr1_clut0()[40],
    ",
  0x503420a4u64 => "
      GLCDC_NS.gr1_clut0()[41],
    ",
  0x503420a8u64 => "
      GLCDC_NS.gr1_clut0()[42],
    ",
  0x503420acu64 => "
      GLCDC_NS.gr1_clut0()[43],
    ",
  0x503420b0u64 => "
      GLCDC_NS.gr1_clut0()[44],
    ",
  0x503420b4u64 => "
      GLCDC_NS.gr1_clut0()[45],
    ",
  0x503420b8u64 => "
      GLCDC_NS.gr1_clut0()[46],
    ",
  0x503420bcu64 => "
      GLCDC_NS.gr1_clut0()[47],
    ",
  0x503420c0u64 => "
      GLCDC_NS.gr1_clut0()[48],
    ",
  0x503420c4u64 => "
      GLCDC_NS.gr1_clut0()[49],
    ",
  0x503420c8u64 => "
      GLCDC_NS.gr1_clut0()[50],
    ",
  0x503420ccu64 => "
      GLCDC_NS.gr1_clut0()[51],
    ",
  0x503420d0u64 => "
      GLCDC_NS.gr1_clut0()[52],
    ",
  0x503420d4u64 => "
      GLCDC_NS.gr1_clut0()[53],
    ",
  0x503420d8u64 => "
      GLCDC_NS.gr1_clut0()[54],
    ",
  0x503420dcu64 => "
      GLCDC_NS.gr1_clut0()[55],
    ",
  0x503420e0u64 => "
      GLCDC_NS.gr1_clut0()[56],
    ",
  0x503420e4u64 => "
      GLCDC_NS.gr1_clut0()[57],
    ",
  0x503420e8u64 => "
      GLCDC_NS.gr1_clut0()[58],
    ",
  0x503420ecu64 => "
      GLCDC_NS.gr1_clut0()[59],
    ",
  0x503420f0u64 => "
      GLCDC_NS.gr1_clut0()[60],
    ",
  0x503420f4u64 => "
      GLCDC_NS.gr1_clut0()[61],
    ",
  0x503420f8u64 => "
      GLCDC_NS.gr1_clut0()[62],
    ",
  0x503420fcu64 => "
      GLCDC_NS.gr1_clut0()[63],
    ",
  0x50342100u64 => "
      GLCDC_NS.gr1_clut0()[64],
    ",
  0x50342104u64 => "
      GLCDC_NS.gr1_clut0()[65],
    ",
  0x50342108u64 => "
      GLCDC_NS.gr1_clut0()[66],
    ",
  0x5034210cu64 => "
      GLCDC_NS.gr1_clut0()[67],
    ",
  0x50342110u64 => "
      GLCDC_NS.gr1_clut0()[68],
    ",
  0x50342114u64 => "
      GLCDC_NS.gr1_clut0()[69],
    ",
  0x50342118u64 => "
      GLCDC_NS.gr1_clut0()[70],
    ",
  0x5034211cu64 => "
      GLCDC_NS.gr1_clut0()[71],
    ",
  0x50342120u64 => "
      GLCDC_NS.gr1_clut0()[72],
    ",
  0x50342124u64 => "
      GLCDC_NS.gr1_clut0()[73],
    ",
  0x50342128u64 => "
      GLCDC_NS.gr1_clut0()[74],
    ",
  0x5034212cu64 => "
      GLCDC_NS.gr1_clut0()[75],
    ",
  0x50342130u64 => "
      GLCDC_NS.gr1_clut0()[76],
    ",
  0x50342134u64 => "
      GLCDC_NS.gr1_clut0()[77],
    ",
  0x50342138u64 => "
      GLCDC_NS.gr1_clut0()[78],
    ",
  0x5034213cu64 => "
      GLCDC_NS.gr1_clut0()[79],
    ",
  0x50342140u64 => "
      GLCDC_NS.gr1_clut0()[80],
    ",
  0x50342144u64 => "
      GLCDC_NS.gr1_clut0()[81],
    ",
  0x50342148u64 => "
      GLCDC_NS.gr1_clut0()[82],
    ",
  0x5034214cu64 => "
      GLCDC_NS.gr1_clut0()[83],
    ",
  0x50342150u64 => "
      GLCDC_NS.gr1_clut0()[84],
    ",
  0x50342154u64 => "
      GLCDC_NS.gr1_clut0()[85],
    ",
  0x50342158u64 => "
      GLCDC_NS.gr1_clut0()[86],
    ",
  0x5034215cu64 => "
      GLCDC_NS.gr1_clut0()[87],
    ",
  0x50342160u64 => "
      GLCDC_NS.gr1_clut0()[88],
    ",
  0x50342164u64 => "
      GLCDC_NS.gr1_clut0()[89],
    ",
  0x50342168u64 => "
      GLCDC_NS.gr1_clut0()[90],
    ",
  0x5034216cu64 => "
      GLCDC_NS.gr1_clut0()[91],
    ",
  0x50342170u64 => "
      GLCDC_NS.gr1_clut0()[92],
    ",
  0x50342174u64 => "
      GLCDC_NS.gr1_clut0()[93],
    ",
  0x50342178u64 => "
      GLCDC_NS.gr1_clut0()[94],
    ",
  0x5034217cu64 => "
      GLCDC_NS.gr1_clut0()[95],
    ",
  0x50342180u64 => "
      GLCDC_NS.gr1_clut0()[96],
    ",
  0x50342184u64 => "
      GLCDC_NS.gr1_clut0()[97],
    ",
  0x50342188u64 => "
      GLCDC_NS.gr1_clut0()[98],
    ",
  0x5034218cu64 => "
      GLCDC_NS.gr1_clut0()[99],
    ",
  0x50342190u64 => "
      GLCDC_NS.gr1_clut0()[100],
    ",
  0x50342194u64 => "
      GLCDC_NS.gr1_clut0()[101],
    ",
  0x50342198u64 => "
      GLCDC_NS.gr1_clut0()[102],
    ",
  0x5034219cu64 => "
      GLCDC_NS.gr1_clut0()[103],
    ",
  0x503421a0u64 => "
      GLCDC_NS.gr1_clut0()[104],
    ",
  0x503421a4u64 => "
      GLCDC_NS.gr1_clut0()[105],
    ",
  0x503421a8u64 => "
      GLCDC_NS.gr1_clut0()[106],
    ",
  0x503421acu64 => "
      GLCDC_NS.gr1_clut0()[107],
    ",
  0x503421b0u64 => "
      GLCDC_NS.gr1_clut0()[108],
    ",
  0x503421b4u64 => "
      GLCDC_NS.gr1_clut0()[109],
    ",
  0x503421b8u64 => "
      GLCDC_NS.gr1_clut0()[110],
    ",
  0x503421bcu64 => "
      GLCDC_NS.gr1_clut0()[111],
    ",
  0x503421c0u64 => "
      GLCDC_NS.gr1_clut0()[112],
    ",
  0x503421c4u64 => "
      GLCDC_NS.gr1_clut0()[113],
    ",
  0x503421c8u64 => "
      GLCDC_NS.gr1_clut0()[114],
    ",
  0x503421ccu64 => "
      GLCDC_NS.gr1_clut0()[115],
    ",
  0x503421d0u64 => "
      GLCDC_NS.gr1_clut0()[116],
    ",
  0x503421d4u64 => "
      GLCDC_NS.gr1_clut0()[117],
    ",
  0x503421d8u64 => "
      GLCDC_NS.gr1_clut0()[118],
    ",
  0x503421dcu64 => "
      GLCDC_NS.gr1_clut0()[119],
    ",
  0x503421e0u64 => "
      GLCDC_NS.gr1_clut0()[120],
    ",
  0x503421e4u64 => "
      GLCDC_NS.gr1_clut0()[121],
    ",
  0x503421e8u64 => "
      GLCDC_NS.gr1_clut0()[122],
    ",
  0x503421ecu64 => "
      GLCDC_NS.gr1_clut0()[123],
    ",
  0x503421f0u64 => "
      GLCDC_NS.gr1_clut0()[124],
    ",
  0x503421f4u64 => "
      GLCDC_NS.gr1_clut0()[125],
    ",
  0x503421f8u64 => "
      GLCDC_NS.gr1_clut0()[126],
    ",
  0x503421fcu64 => "
      GLCDC_NS.gr1_clut0()[127],
    ",
  0x50342200u64 => "
      GLCDC_NS.gr1_clut0()[128],
    ",
  0x50342204u64 => "
      GLCDC_NS.gr1_clut0()[129],
    ",
  0x50342208u64 => "
      GLCDC_NS.gr1_clut0()[130],
    ",
  0x5034220cu64 => "
      GLCDC_NS.gr1_clut0()[131],
    ",
  0x50342210u64 => "
      GLCDC_NS.gr1_clut0()[132],
    ",
  0x50342214u64 => "
      GLCDC_NS.gr1_clut0()[133],
    ",
  0x50342218u64 => "
      GLCDC_NS.gr1_clut0()[134],
    ",
  0x5034221cu64 => "
      GLCDC_NS.gr1_clut0()[135],
    ",
  0x50342220u64 => "
      GLCDC_NS.gr1_clut0()[136],
    ",
  0x50342224u64 => "
      GLCDC_NS.gr1_clut0()[137],
    ",
  0x50342228u64 => "
      GLCDC_NS.gr1_clut0()[138],
    ",
  0x5034222cu64 => "
      GLCDC_NS.gr1_clut0()[139],
    ",
  0x50342230u64 => "
      GLCDC_NS.gr1_clut0()[140],
    ",
  0x50342234u64 => "
      GLCDC_NS.gr1_clut0()[141],
    ",
  0x50342238u64 => "
      GLCDC_NS.gr1_clut0()[142],
    ",
  0x5034223cu64 => "
      GLCDC_NS.gr1_clut0()[143],
    ",
  0x50342240u64 => "
      GLCDC_NS.gr1_clut0()[144],
    ",
  0x50342244u64 => "
      GLCDC_NS.gr1_clut0()[145],
    ",
  0x50342248u64 => "
      GLCDC_NS.gr1_clut0()[146],
    ",
  0x5034224cu64 => "
      GLCDC_NS.gr1_clut0()[147],
    ",
  0x50342250u64 => "
      GLCDC_NS.gr1_clut0()[148],
    ",
  0x50342254u64 => "
      GLCDC_NS.gr1_clut0()[149],
    ",
  0x50342258u64 => "
      GLCDC_NS.gr1_clut0()[150],
    ",
  0x5034225cu64 => "
      GLCDC_NS.gr1_clut0()[151],
    ",
  0x50342260u64 => "
      GLCDC_NS.gr1_clut0()[152],
    ",
  0x50342264u64 => "
      GLCDC_NS.gr1_clut0()[153],
    ",
  0x50342268u64 => "
      GLCDC_NS.gr1_clut0()[154],
    ",
  0x5034226cu64 => "
      GLCDC_NS.gr1_clut0()[155],
    ",
  0x50342270u64 => "
      GLCDC_NS.gr1_clut0()[156],
    ",
  0x50342274u64 => "
      GLCDC_NS.gr1_clut0()[157],
    ",
  0x50342278u64 => "
      GLCDC_NS.gr1_clut0()[158],
    ",
  0x5034227cu64 => "
      GLCDC_NS.gr1_clut0()[159],
    ",
  0x50342280u64 => "
      GLCDC_NS.gr1_clut0()[160],
    ",
  0x50342284u64 => "
      GLCDC_NS.gr1_clut0()[161],
    ",
  0x50342288u64 => "
      GLCDC_NS.gr1_clut0()[162],
    ",
  0x5034228cu64 => "
      GLCDC_NS.gr1_clut0()[163],
    ",
  0x50342290u64 => "
      GLCDC_NS.gr1_clut0()[164],
    ",
  0x50342294u64 => "
      GLCDC_NS.gr1_clut0()[165],
    ",
  0x50342298u64 => "
      GLCDC_NS.gr1_clut0()[166],
    ",
  0x5034229cu64 => "
      GLCDC_NS.gr1_clut0()[167],
    ",
  0x503422a0u64 => "
      GLCDC_NS.gr1_clut0()[168],
    ",
  0x503422a4u64 => "
      GLCDC_NS.gr1_clut0()[169],
    ",
  0x503422a8u64 => "
      GLCDC_NS.gr1_clut0()[170],
    ",
  0x503422acu64 => "
      GLCDC_NS.gr1_clut0()[171],
    ",
  0x503422b0u64 => "
      GLCDC_NS.gr1_clut0()[172],
    ",
  0x503422b4u64 => "
      GLCDC_NS.gr1_clut0()[173],
    ",
  0x503422b8u64 => "
      GLCDC_NS.gr1_clut0()[174],
    ",
  0x503422bcu64 => "
      GLCDC_NS.gr1_clut0()[175],
    ",
  0x503422c0u64 => "
      GLCDC_NS.gr1_clut0()[176],
    ",
  0x503422c4u64 => "
      GLCDC_NS.gr1_clut0()[177],
    ",
  0x503422c8u64 => "
      GLCDC_NS.gr1_clut0()[178],
    ",
  0x503422ccu64 => "
      GLCDC_NS.gr1_clut0()[179],
    ",
  0x503422d0u64 => "
      GLCDC_NS.gr1_clut0()[180],
    ",
  0x503422d4u64 => "
      GLCDC_NS.gr1_clut0()[181],
    ",
  0x503422d8u64 => "
      GLCDC_NS.gr1_clut0()[182],
    ",
  0x503422dcu64 => "
      GLCDC_NS.gr1_clut0()[183],
    ",
  0x503422e0u64 => "
      GLCDC_NS.gr1_clut0()[184],
    ",
  0x503422e4u64 => "
      GLCDC_NS.gr1_clut0()[185],
    ",
  0x503422e8u64 => "
      GLCDC_NS.gr1_clut0()[186],
    ",
  0x503422ecu64 => "
      GLCDC_NS.gr1_clut0()[187],
    ",
  0x503422f0u64 => "
      GLCDC_NS.gr1_clut0()[188],
    ",
  0x503422f4u64 => "
      GLCDC_NS.gr1_clut0()[189],
    ",
  0x503422f8u64 => "
      GLCDC_NS.gr1_clut0()[190],
    ",
  0x503422fcu64 => "
      GLCDC_NS.gr1_clut0()[191],
    ",
  0x50342300u64 => "
      GLCDC_NS.gr1_clut0()[192],
    ",
  0x50342304u64 => "
      GLCDC_NS.gr1_clut0()[193],
    ",
  0x50342308u64 => "
      GLCDC_NS.gr1_clut0()[194],
    ",
  0x5034230cu64 => "
      GLCDC_NS.gr1_clut0()[195],
    ",
  0x50342310u64 => "
      GLCDC_NS.gr1_clut0()[196],
    ",
  0x50342314u64 => "
      GLCDC_NS.gr1_clut0()[197],
    ",
  0x50342318u64 => "
      GLCDC_NS.gr1_clut0()[198],
    ",
  0x5034231cu64 => "
      GLCDC_NS.gr1_clut0()[199],
    ",
  0x50342320u64 => "
      GLCDC_NS.gr1_clut0()[200],
    ",
  0x50342324u64 => "
      GLCDC_NS.gr1_clut0()[201],
    ",
  0x50342328u64 => "
      GLCDC_NS.gr1_clut0()[202],
    ",
  0x5034232cu64 => "
      GLCDC_NS.gr1_clut0()[203],
    ",
  0x50342330u64 => "
      GLCDC_NS.gr1_clut0()[204],
    ",
  0x50342334u64 => "
      GLCDC_NS.gr1_clut0()[205],
    ",
  0x50342338u64 => "
      GLCDC_NS.gr1_clut0()[206],
    ",
  0x5034233cu64 => "
      GLCDC_NS.gr1_clut0()[207],
    ",
  0x50342340u64 => "
      GLCDC_NS.gr1_clut0()[208],
    ",
  0x50342344u64 => "
      GLCDC_NS.gr1_clut0()[209],
    ",
  0x50342348u64 => "
      GLCDC_NS.gr1_clut0()[210],
    ",
  0x5034234cu64 => "
      GLCDC_NS.gr1_clut0()[211],
    ",
  0x50342350u64 => "
      GLCDC_NS.gr1_clut0()[212],
    ",
  0x50342354u64 => "
      GLCDC_NS.gr1_clut0()[213],
    ",
  0x50342358u64 => "
      GLCDC_NS.gr1_clut0()[214],
    ",
  0x5034235cu64 => "
      GLCDC_NS.gr1_clut0()[215],
    ",
  0x50342360u64 => "
      GLCDC_NS.gr1_clut0()[216],
    ",
  0x50342364u64 => "
      GLCDC_NS.gr1_clut0()[217],
    ",
  0x50342368u64 => "
      GLCDC_NS.gr1_clut0()[218],
    ",
  0x5034236cu64 => "
      GLCDC_NS.gr1_clut0()[219],
    ",
  0x50342370u64 => "
      GLCDC_NS.gr1_clut0()[220],
    ",
  0x50342374u64 => "
      GLCDC_NS.gr1_clut0()[221],
    ",
  0x50342378u64 => "
      GLCDC_NS.gr1_clut0()[222],
    ",
  0x5034237cu64 => "
      GLCDC_NS.gr1_clut0()[223],
    ",
  0x50342380u64 => "
      GLCDC_NS.gr1_clut0()[224],
    ",
  0x50342384u64 => "
      GLCDC_NS.gr1_clut0()[225],
    ",
  0x50342388u64 => "
      GLCDC_NS.gr1_clut0()[226],
    ",
  0x5034238cu64 => "
      GLCDC_NS.gr1_clut0()[227],
    ",
  0x50342390u64 => "
      GLCDC_NS.gr1_clut0()[228],
    ",
  0x50342394u64 => "
      GLCDC_NS.gr1_clut0()[229],
    ",
  0x50342398u64 => "
      GLCDC_NS.gr1_clut0()[230],
    ",
  0x5034239cu64 => "
      GLCDC_NS.gr1_clut0()[231],
    ",
  0x503423a0u64 => "
      GLCDC_NS.gr1_clut0()[232],
    ",
  0x503423a4u64 => "
      GLCDC_NS.gr1_clut0()[233],
    ",
  0x503423a8u64 => "
      GLCDC_NS.gr1_clut0()[234],
    ",
  0x503423acu64 => "
      GLCDC_NS.gr1_clut0()[235],
    ",
  0x503423b0u64 => "
      GLCDC_NS.gr1_clut0()[236],
    ",
  0x503423b4u64 => "
      GLCDC_NS.gr1_clut0()[237],
    ",
  0x503423b8u64 => "
      GLCDC_NS.gr1_clut0()[238],
    ",
  0x503423bcu64 => "
      GLCDC_NS.gr1_clut0()[239],
    ",
  0x503423c0u64 => "
      GLCDC_NS.gr1_clut0()[240],
    ",
  0x503423c4u64 => "
      GLCDC_NS.gr1_clut0()[241],
    ",
  0x503423c8u64 => "
      GLCDC_NS.gr1_clut0()[242],
    ",
  0x503423ccu64 => "
      GLCDC_NS.gr1_clut0()[243],
    ",
  0x503423d0u64 => "
      GLCDC_NS.gr1_clut0()[244],
    ",
  0x503423d4u64 => "
      GLCDC_NS.gr1_clut0()[245],
    ",
  0x503423d8u64 => "
      GLCDC_NS.gr1_clut0()[246],
    ",
  0x503423dcu64 => "
      GLCDC_NS.gr1_clut0()[247],
    ",
  0x503423e0u64 => "
      GLCDC_NS.gr1_clut0()[248],
    ",
  0x503423e4u64 => "
      GLCDC_NS.gr1_clut0()[249],
    ",
  0x503423e8u64 => "
      GLCDC_NS.gr1_clut0()[250],
    ",
  0x503423ecu64 => "
      GLCDC_NS.gr1_clut0()[251],
    ",
  0x503423f0u64 => "
      GLCDC_NS.gr1_clut0()[252],
    ",
  0x503423f4u64 => "
      GLCDC_NS.gr1_clut0()[253],
    ",
  0x503423f8u64 => "
      GLCDC_NS.gr1_clut0()[254],
    ",
  0x503423fcu64 => "
      GLCDC_NS.gr1_clut0()[255],
    ",
  0x50342400u64 => "
      GLCDC_NS.gr1_clut1()[0],
    ",
  0x50342404u64 => "
      GLCDC_NS.gr1_clut1()[1],
    ",
  0x50342408u64 => "
      GLCDC_NS.gr1_clut1()[2],
    ",
  0x5034240cu64 => "
      GLCDC_NS.gr1_clut1()[3],
    ",
  0x50342410u64 => "
      GLCDC_NS.gr1_clut1()[4],
    ",
  0x50342414u64 => "
      GLCDC_NS.gr1_clut1()[5],
    ",
  0x50342418u64 => "
      GLCDC_NS.gr1_clut1()[6],
    ",
  0x5034241cu64 => "
      GLCDC_NS.gr1_clut1()[7],
    ",
  0x50342420u64 => "
      GLCDC_NS.gr1_clut1()[8],
    ",
  0x50342424u64 => "
      GLCDC_NS.gr1_clut1()[9],
    ",
  0x50342428u64 => "
      GLCDC_NS.gr1_clut1()[10],
    ",
  0x5034242cu64 => "
      GLCDC_NS.gr1_clut1()[11],
    ",
  0x50342430u64 => "
      GLCDC_NS.gr1_clut1()[12],
    ",
  0x50342434u64 => "
      GLCDC_NS.gr1_clut1()[13],
    ",
  0x50342438u64 => "
      GLCDC_NS.gr1_clut1()[14],
    ",
  0x5034243cu64 => "
      GLCDC_NS.gr1_clut1()[15],
    ",
  0x50342440u64 => "
      GLCDC_NS.gr1_clut1()[16],
    ",
  0x50342444u64 => "
      GLCDC_NS.gr1_clut1()[17],
    ",
  0x50342448u64 => "
      GLCDC_NS.gr1_clut1()[18],
    ",
  0x5034244cu64 => "
      GLCDC_NS.gr1_clut1()[19],
    ",
  0x50342450u64 => "
      GLCDC_NS.gr1_clut1()[20],
    ",
  0x50342454u64 => "
      GLCDC_NS.gr1_clut1()[21],
    ",
  0x50342458u64 => "
      GLCDC_NS.gr1_clut1()[22],
    ",
  0x5034245cu64 => "
      GLCDC_NS.gr1_clut1()[23],
    ",
  0x50342460u64 => "
      GLCDC_NS.gr1_clut1()[24],
    ",
  0x50342464u64 => "
      GLCDC_NS.gr1_clut1()[25],
    ",
  0x50342468u64 => "
      GLCDC_NS.gr1_clut1()[26],
    ",
  0x5034246cu64 => "
      GLCDC_NS.gr1_clut1()[27],
    ",
  0x50342470u64 => "
      GLCDC_NS.gr1_clut1()[28],
    ",
  0x50342474u64 => "
      GLCDC_NS.gr1_clut1()[29],
    ",
  0x50342478u64 => "
      GLCDC_NS.gr1_clut1()[30],
    ",
  0x5034247cu64 => "
      GLCDC_NS.gr1_clut1()[31],
    ",
  0x50342480u64 => "
      GLCDC_NS.gr1_clut1()[32],
    ",
  0x50342484u64 => "
      GLCDC_NS.gr1_clut1()[33],
    ",
  0x50342488u64 => "
      GLCDC_NS.gr1_clut1()[34],
    ",
  0x5034248cu64 => "
      GLCDC_NS.gr1_clut1()[35],
    ",
  0x50342490u64 => "
      GLCDC_NS.gr1_clut1()[36],
    ",
  0x50342494u64 => "
      GLCDC_NS.gr1_clut1()[37],
    ",
  0x50342498u64 => "
      GLCDC_NS.gr1_clut1()[38],
    ",
  0x5034249cu64 => "
      GLCDC_NS.gr1_clut1()[39],
    ",
  0x503424a0u64 => "
      GLCDC_NS.gr1_clut1()[40],
    ",
  0x503424a4u64 => "
      GLCDC_NS.gr1_clut1()[41],
    ",
  0x503424a8u64 => "
      GLCDC_NS.gr1_clut1()[42],
    ",
  0x503424acu64 => "
      GLCDC_NS.gr1_clut1()[43],
    ",
  0x503424b0u64 => "
      GLCDC_NS.gr1_clut1()[44],
    ",
  0x503424b4u64 => "
      GLCDC_NS.gr1_clut1()[45],
    ",
  0x503424b8u64 => "
      GLCDC_NS.gr1_clut1()[46],
    ",
  0x503424bcu64 => "
      GLCDC_NS.gr1_clut1()[47],
    ",
  0x503424c0u64 => "
      GLCDC_NS.gr1_clut1()[48],
    ",
  0x503424c4u64 => "
      GLCDC_NS.gr1_clut1()[49],
    ",
  0x503424c8u64 => "
      GLCDC_NS.gr1_clut1()[50],
    ",
  0x503424ccu64 => "
      GLCDC_NS.gr1_clut1()[51],
    ",
  0x503424d0u64 => "
      GLCDC_NS.gr1_clut1()[52],
    ",
  0x503424d4u64 => "
      GLCDC_NS.gr1_clut1()[53],
    ",
  0x503424d8u64 => "
      GLCDC_NS.gr1_clut1()[54],
    ",
  0x503424dcu64 => "
      GLCDC_NS.gr1_clut1()[55],
    ",
  0x503424e0u64 => "
      GLCDC_NS.gr1_clut1()[56],
    ",
  0x503424e4u64 => "
      GLCDC_NS.gr1_clut1()[57],
    ",
  0x503424e8u64 => "
      GLCDC_NS.gr1_clut1()[58],
    ",
  0x503424ecu64 => "
      GLCDC_NS.gr1_clut1()[59],
    ",
  0x503424f0u64 => "
      GLCDC_NS.gr1_clut1()[60],
    ",
  0x503424f4u64 => "
      GLCDC_NS.gr1_clut1()[61],
    ",
  0x503424f8u64 => "
      GLCDC_NS.gr1_clut1()[62],
    ",
  0x503424fcu64 => "
      GLCDC_NS.gr1_clut1()[63],
    ",
  0x50342500u64 => "
      GLCDC_NS.gr1_clut1()[64],
    ",
  0x50342504u64 => "
      GLCDC_NS.gr1_clut1()[65],
    ",
  0x50342508u64 => "
      GLCDC_NS.gr1_clut1()[66],
    ",
  0x5034250cu64 => "
      GLCDC_NS.gr1_clut1()[67],
    ",
  0x50342510u64 => "
      GLCDC_NS.gr1_clut1()[68],
    ",
  0x50342514u64 => "
      GLCDC_NS.gr1_clut1()[69],
    ",
  0x50342518u64 => "
      GLCDC_NS.gr1_clut1()[70],
    ",
  0x5034251cu64 => "
      GLCDC_NS.gr1_clut1()[71],
    ",
  0x50342520u64 => "
      GLCDC_NS.gr1_clut1()[72],
    ",
  0x50342524u64 => "
      GLCDC_NS.gr1_clut1()[73],
    ",
  0x50342528u64 => "
      GLCDC_NS.gr1_clut1()[74],
    ",
  0x5034252cu64 => "
      GLCDC_NS.gr1_clut1()[75],
    ",
  0x50342530u64 => "
      GLCDC_NS.gr1_clut1()[76],
    ",
  0x50342534u64 => "
      GLCDC_NS.gr1_clut1()[77],
    ",
  0x50342538u64 => "
      GLCDC_NS.gr1_clut1()[78],
    ",
  0x5034253cu64 => "
      GLCDC_NS.gr1_clut1()[79],
    ",
  0x50342540u64 => "
      GLCDC_NS.gr1_clut1()[80],
    ",
  0x50342544u64 => "
      GLCDC_NS.gr1_clut1()[81],
    ",
  0x50342548u64 => "
      GLCDC_NS.gr1_clut1()[82],
    ",
  0x5034254cu64 => "
      GLCDC_NS.gr1_clut1()[83],
    ",
  0x50342550u64 => "
      GLCDC_NS.gr1_clut1()[84],
    ",
  0x50342554u64 => "
      GLCDC_NS.gr1_clut1()[85],
    ",
  0x50342558u64 => "
      GLCDC_NS.gr1_clut1()[86],
    ",
  0x5034255cu64 => "
      GLCDC_NS.gr1_clut1()[87],
    ",
  0x50342560u64 => "
      GLCDC_NS.gr1_clut1()[88],
    ",
  0x50342564u64 => "
      GLCDC_NS.gr1_clut1()[89],
    ",
  0x50342568u64 => "
      GLCDC_NS.gr1_clut1()[90],
    ",
  0x5034256cu64 => "
      GLCDC_NS.gr1_clut1()[91],
    ",
  0x50342570u64 => "
      GLCDC_NS.gr1_clut1()[92],
    ",
  0x50342574u64 => "
      GLCDC_NS.gr1_clut1()[93],
    ",
  0x50342578u64 => "
      GLCDC_NS.gr1_clut1()[94],
    ",
  0x5034257cu64 => "
      GLCDC_NS.gr1_clut1()[95],
    ",
  0x50342580u64 => "
      GLCDC_NS.gr1_clut1()[96],
    ",
  0x50342584u64 => "
      GLCDC_NS.gr1_clut1()[97],
    ",
  0x50342588u64 => "
      GLCDC_NS.gr1_clut1()[98],
    ",
  0x5034258cu64 => "
      GLCDC_NS.gr1_clut1()[99],
    ",
  0x50342590u64 => "
      GLCDC_NS.gr1_clut1()[100],
    ",
  0x50342594u64 => "
      GLCDC_NS.gr1_clut1()[101],
    ",
  0x50342598u64 => "
      GLCDC_NS.gr1_clut1()[102],
    ",
  0x5034259cu64 => "
      GLCDC_NS.gr1_clut1()[103],
    ",
  0x503425a0u64 => "
      GLCDC_NS.gr1_clut1()[104],
    ",
  0x503425a4u64 => "
      GLCDC_NS.gr1_clut1()[105],
    ",
  0x503425a8u64 => "
      GLCDC_NS.gr1_clut1()[106],
    ",
  0x503425acu64 => "
      GLCDC_NS.gr1_clut1()[107],
    ",
  0x503425b0u64 => "
      GLCDC_NS.gr1_clut1()[108],
    ",
  0x503425b4u64 => "
      GLCDC_NS.gr1_clut1()[109],
    ",
  0x503425b8u64 => "
      GLCDC_NS.gr1_clut1()[110],
    ",
  0x503425bcu64 => "
      GLCDC_NS.gr1_clut1()[111],
    ",
  0x503425c0u64 => "
      GLCDC_NS.gr1_clut1()[112],
    ",
  0x503425c4u64 => "
      GLCDC_NS.gr1_clut1()[113],
    ",
  0x503425c8u64 => "
      GLCDC_NS.gr1_clut1()[114],
    ",
  0x503425ccu64 => "
      GLCDC_NS.gr1_clut1()[115],
    ",
  0x503425d0u64 => "
      GLCDC_NS.gr1_clut1()[116],
    ",
  0x503425d4u64 => "
      GLCDC_NS.gr1_clut1()[117],
    ",
  0x503425d8u64 => "
      GLCDC_NS.gr1_clut1()[118],
    ",
  0x503425dcu64 => "
      GLCDC_NS.gr1_clut1()[119],
    ",
  0x503425e0u64 => "
      GLCDC_NS.gr1_clut1()[120],
    ",
  0x503425e4u64 => "
      GLCDC_NS.gr1_clut1()[121],
    ",
  0x503425e8u64 => "
      GLCDC_NS.gr1_clut1()[122],
    ",
  0x503425ecu64 => "
      GLCDC_NS.gr1_clut1()[123],
    ",
  0x503425f0u64 => "
      GLCDC_NS.gr1_clut1()[124],
    ",
  0x503425f4u64 => "
      GLCDC_NS.gr1_clut1()[125],
    ",
  0x503425f8u64 => "
      GLCDC_NS.gr1_clut1()[126],
    ",
  0x503425fcu64 => "
      GLCDC_NS.gr1_clut1()[127],
    ",
  0x50342600u64 => "
      GLCDC_NS.gr1_clut1()[128],
    ",
  0x50342604u64 => "
      GLCDC_NS.gr1_clut1()[129],
    ",
  0x50342608u64 => "
      GLCDC_NS.gr1_clut1()[130],
    ",
  0x5034260cu64 => "
      GLCDC_NS.gr1_clut1()[131],
    ",
  0x50342610u64 => "
      GLCDC_NS.gr1_clut1()[132],
    ",
  0x50342614u64 => "
      GLCDC_NS.gr1_clut1()[133],
    ",
  0x50342618u64 => "
      GLCDC_NS.gr1_clut1()[134],
    ",
  0x5034261cu64 => "
      GLCDC_NS.gr1_clut1()[135],
    ",
  0x50342620u64 => "
      GLCDC_NS.gr1_clut1()[136],
    ",
  0x50342624u64 => "
      GLCDC_NS.gr1_clut1()[137],
    ",
  0x50342628u64 => "
      GLCDC_NS.gr1_clut1()[138],
    ",
  0x5034262cu64 => "
      GLCDC_NS.gr1_clut1()[139],
    ",
  0x50342630u64 => "
      GLCDC_NS.gr1_clut1()[140],
    ",
  0x50342634u64 => "
      GLCDC_NS.gr1_clut1()[141],
    ",
  0x50342638u64 => "
      GLCDC_NS.gr1_clut1()[142],
    ",
  0x5034263cu64 => "
      GLCDC_NS.gr1_clut1()[143],
    ",
  0x50342640u64 => "
      GLCDC_NS.gr1_clut1()[144],
    ",
  0x50342644u64 => "
      GLCDC_NS.gr1_clut1()[145],
    ",
  0x50342648u64 => "
      GLCDC_NS.gr1_clut1()[146],
    ",
  0x5034264cu64 => "
      GLCDC_NS.gr1_clut1()[147],
    ",
  0x50342650u64 => "
      GLCDC_NS.gr1_clut1()[148],
    ",
  0x50342654u64 => "
      GLCDC_NS.gr1_clut1()[149],
    ",
  0x50342658u64 => "
      GLCDC_NS.gr1_clut1()[150],
    ",
  0x5034265cu64 => "
      GLCDC_NS.gr1_clut1()[151],
    ",
  0x50342660u64 => "
      GLCDC_NS.gr1_clut1()[152],
    ",
  0x50342664u64 => "
      GLCDC_NS.gr1_clut1()[153],
    ",
  0x50342668u64 => "
      GLCDC_NS.gr1_clut1()[154],
    ",
  0x5034266cu64 => "
      GLCDC_NS.gr1_clut1()[155],
    ",
  0x50342670u64 => "
      GLCDC_NS.gr1_clut1()[156],
    ",
  0x50342674u64 => "
      GLCDC_NS.gr1_clut1()[157],
    ",
  0x50342678u64 => "
      GLCDC_NS.gr1_clut1()[158],
    ",
  0x5034267cu64 => "
      GLCDC_NS.gr1_clut1()[159],
    ",
  0x50342680u64 => "
      GLCDC_NS.gr1_clut1()[160],
    ",
  0x50342684u64 => "
      GLCDC_NS.gr1_clut1()[161],
    ",
  0x50342688u64 => "
      GLCDC_NS.gr1_clut1()[162],
    ",
  0x5034268cu64 => "
      GLCDC_NS.gr1_clut1()[163],
    ",
  0x50342690u64 => "
      GLCDC_NS.gr1_clut1()[164],
    ",
  0x50342694u64 => "
      GLCDC_NS.gr1_clut1()[165],
    ",
  0x50342698u64 => "
      GLCDC_NS.gr1_clut1()[166],
    ",
  0x5034269cu64 => "
      GLCDC_NS.gr1_clut1()[167],
    ",
  0x503426a0u64 => "
      GLCDC_NS.gr1_clut1()[168],
    ",
  0x503426a4u64 => "
      GLCDC_NS.gr1_clut1()[169],
    ",
  0x503426a8u64 => "
      GLCDC_NS.gr1_clut1()[170],
    ",
  0x503426acu64 => "
      GLCDC_NS.gr1_clut1()[171],
    ",
  0x503426b0u64 => "
      GLCDC_NS.gr1_clut1()[172],
    ",
  0x503426b4u64 => "
      GLCDC_NS.gr1_clut1()[173],
    ",
  0x503426b8u64 => "
      GLCDC_NS.gr1_clut1()[174],
    ",
  0x503426bcu64 => "
      GLCDC_NS.gr1_clut1()[175],
    ",
  0x503426c0u64 => "
      GLCDC_NS.gr1_clut1()[176],
    ",
  0x503426c4u64 => "
      GLCDC_NS.gr1_clut1()[177],
    ",
  0x503426c8u64 => "
      GLCDC_NS.gr1_clut1()[178],
    ",
  0x503426ccu64 => "
      GLCDC_NS.gr1_clut1()[179],
    ",
  0x503426d0u64 => "
      GLCDC_NS.gr1_clut1()[180],
    ",
  0x503426d4u64 => "
      GLCDC_NS.gr1_clut1()[181],
    ",
  0x503426d8u64 => "
      GLCDC_NS.gr1_clut1()[182],
    ",
  0x503426dcu64 => "
      GLCDC_NS.gr1_clut1()[183],
    ",
  0x503426e0u64 => "
      GLCDC_NS.gr1_clut1()[184],
    ",
  0x503426e4u64 => "
      GLCDC_NS.gr1_clut1()[185],
    ",
  0x503426e8u64 => "
      GLCDC_NS.gr1_clut1()[186],
    ",
  0x503426ecu64 => "
      GLCDC_NS.gr1_clut1()[187],
    ",
  0x503426f0u64 => "
      GLCDC_NS.gr1_clut1()[188],
    ",
  0x503426f4u64 => "
      GLCDC_NS.gr1_clut1()[189],
    ",
  0x503426f8u64 => "
      GLCDC_NS.gr1_clut1()[190],
    ",
  0x503426fcu64 => "
      GLCDC_NS.gr1_clut1()[191],
    ",
  0x50342700u64 => "
      GLCDC_NS.gr1_clut1()[192],
    ",
  0x50342704u64 => "
      GLCDC_NS.gr1_clut1()[193],
    ",
  0x50342708u64 => "
      GLCDC_NS.gr1_clut1()[194],
    ",
  0x5034270cu64 => "
      GLCDC_NS.gr1_clut1()[195],
    ",
  0x50342710u64 => "
      GLCDC_NS.gr1_clut1()[196],
    ",
  0x50342714u64 => "
      GLCDC_NS.gr1_clut1()[197],
    ",
  0x50342718u64 => "
      GLCDC_NS.gr1_clut1()[198],
    ",
  0x5034271cu64 => "
      GLCDC_NS.gr1_clut1()[199],
    ",
  0x50342720u64 => "
      GLCDC_NS.gr1_clut1()[200],
    ",
  0x50342724u64 => "
      GLCDC_NS.gr1_clut1()[201],
    ",
  0x50342728u64 => "
      GLCDC_NS.gr1_clut1()[202],
    ",
  0x5034272cu64 => "
      GLCDC_NS.gr1_clut1()[203],
    ",
  0x50342730u64 => "
      GLCDC_NS.gr1_clut1()[204],
    ",
  0x50342734u64 => "
      GLCDC_NS.gr1_clut1()[205],
    ",
  0x50342738u64 => "
      GLCDC_NS.gr1_clut1()[206],
    ",
  0x5034273cu64 => "
      GLCDC_NS.gr1_clut1()[207],
    ",
  0x50342740u64 => "
      GLCDC_NS.gr1_clut1()[208],
    ",
  0x50342744u64 => "
      GLCDC_NS.gr1_clut1()[209],
    ",
  0x50342748u64 => "
      GLCDC_NS.gr1_clut1()[210],
    ",
  0x5034274cu64 => "
      GLCDC_NS.gr1_clut1()[211],
    ",
  0x50342750u64 => "
      GLCDC_NS.gr1_clut1()[212],
    ",
  0x50342754u64 => "
      GLCDC_NS.gr1_clut1()[213],
    ",
  0x50342758u64 => "
      GLCDC_NS.gr1_clut1()[214],
    ",
  0x5034275cu64 => "
      GLCDC_NS.gr1_clut1()[215],
    ",
  0x50342760u64 => "
      GLCDC_NS.gr1_clut1()[216],
    ",
  0x50342764u64 => "
      GLCDC_NS.gr1_clut1()[217],
    ",
  0x50342768u64 => "
      GLCDC_NS.gr1_clut1()[218],
    ",
  0x5034276cu64 => "
      GLCDC_NS.gr1_clut1()[219],
    ",
  0x50342770u64 => "
      GLCDC_NS.gr1_clut1()[220],
    ",
  0x50342774u64 => "
      GLCDC_NS.gr1_clut1()[221],
    ",
  0x50342778u64 => "
      GLCDC_NS.gr1_clut1()[222],
    ",
  0x5034277cu64 => "
      GLCDC_NS.gr1_clut1()[223],
    ",
  0x50342780u64 => "
      GLCDC_NS.gr1_clut1()[224],
    ",
  0x50342784u64 => "
      GLCDC_NS.gr1_clut1()[225],
    ",
  0x50342788u64 => "
      GLCDC_NS.gr1_clut1()[226],
    ",
  0x5034278cu64 => "
      GLCDC_NS.gr1_clut1()[227],
    ",
  0x50342790u64 => "
      GLCDC_NS.gr1_clut1()[228],
    ",
  0x50342794u64 => "
      GLCDC_NS.gr1_clut1()[229],
    ",
  0x50342798u64 => "
      GLCDC_NS.gr1_clut1()[230],
    ",
  0x5034279cu64 => "
      GLCDC_NS.gr1_clut1()[231],
    ",
  0x503427a0u64 => "
      GLCDC_NS.gr1_clut1()[232],
    ",
  0x503427a4u64 => "
      GLCDC_NS.gr1_clut1()[233],
    ",
  0x503427a8u64 => "
      GLCDC_NS.gr1_clut1()[234],
    ",
  0x503427acu64 => "
      GLCDC_NS.gr1_clut1()[235],
    ",
  0x503427b0u64 => "
      GLCDC_NS.gr1_clut1()[236],
    ",
  0x503427b4u64 => "
      GLCDC_NS.gr1_clut1()[237],
    ",
  0x503427b8u64 => "
      GLCDC_NS.gr1_clut1()[238],
    ",
  0x503427bcu64 => "
      GLCDC_NS.gr1_clut1()[239],
    ",
  0x503427c0u64 => "
      GLCDC_NS.gr1_clut1()[240],
    ",
  0x503427c4u64 => "
      GLCDC_NS.gr1_clut1()[241],
    ",
  0x503427c8u64 => "
      GLCDC_NS.gr1_clut1()[242],
    ",
  0x503427ccu64 => "
      GLCDC_NS.gr1_clut1()[243],
    ",
  0x503427d0u64 => "
      GLCDC_NS.gr1_clut1()[244],
    ",
  0x503427d4u64 => "
      GLCDC_NS.gr1_clut1()[245],
    ",
  0x503427d8u64 => "
      GLCDC_NS.gr1_clut1()[246],
    ",
  0x503427dcu64 => "
      GLCDC_NS.gr1_clut1()[247],
    ",
  0x503427e0u64 => "
      GLCDC_NS.gr1_clut1()[248],
    ",
  0x503427e4u64 => "
      GLCDC_NS.gr1_clut1()[249],
    ",
  0x503427e8u64 => "
      GLCDC_NS.gr1_clut1()[250],
    ",
  0x503427ecu64 => "
      GLCDC_NS.gr1_clut1()[251],
    ",
  0x503427f0u64 => "
      GLCDC_NS.gr1_clut1()[252],
    ",
  0x503427f4u64 => "
      GLCDC_NS.gr1_clut1()[253],
    ",
  0x503427f8u64 => "
      GLCDC_NS.gr1_clut1()[254],
    ",
  0x503427fcu64 => "
      GLCDC_NS.gr1_clut1()[255],
    ",
  0x50342800u64 => "
      GLCDC_NS.gr2_clut0()[0],
    ",
  0x50342804u64 => "
      GLCDC_NS.gr2_clut0()[1],
    ",
  0x50342808u64 => "
      GLCDC_NS.gr2_clut0()[2],
    ",
  0x5034280cu64 => "
      GLCDC_NS.gr2_clut0()[3],
    ",
  0x50342810u64 => "
      GLCDC_NS.gr2_clut0()[4],
    ",
  0x50342814u64 => "
      GLCDC_NS.gr2_clut0()[5],
    ",
  0x50342818u64 => "
      GLCDC_NS.gr2_clut0()[6],
    ",
  0x5034281cu64 => "
      GLCDC_NS.gr2_clut0()[7],
    ",
  0x50342820u64 => "
      GLCDC_NS.gr2_clut0()[8],
    ",
  0x50342824u64 => "
      GLCDC_NS.gr2_clut0()[9],
    ",
  0x50342828u64 => "
      GLCDC_NS.gr2_clut0()[10],
    ",
  0x5034282cu64 => "
      GLCDC_NS.gr2_clut0()[11],
    ",
  0x50342830u64 => "
      GLCDC_NS.gr2_clut0()[12],
    ",
  0x50342834u64 => "
      GLCDC_NS.gr2_clut0()[13],
    ",
  0x50342838u64 => "
      GLCDC_NS.gr2_clut0()[14],
    ",
  0x5034283cu64 => "
      GLCDC_NS.gr2_clut0()[15],
    ",
  0x50342840u64 => "
      GLCDC_NS.gr2_clut0()[16],
    ",
  0x50342844u64 => "
      GLCDC_NS.gr2_clut0()[17],
    ",
  0x50342848u64 => "
      GLCDC_NS.gr2_clut0()[18],
    ",
  0x5034284cu64 => "
      GLCDC_NS.gr2_clut0()[19],
    ",
  0x50342850u64 => "
      GLCDC_NS.gr2_clut0()[20],
    ",
  0x50342854u64 => "
      GLCDC_NS.gr2_clut0()[21],
    ",
  0x50342858u64 => "
      GLCDC_NS.gr2_clut0()[22],
    ",
  0x5034285cu64 => "
      GLCDC_NS.gr2_clut0()[23],
    ",
  0x50342860u64 => "
      GLCDC_NS.gr2_clut0()[24],
    ",
  0x50342864u64 => "
      GLCDC_NS.gr2_clut0()[25],
    ",
  0x50342868u64 => "
      GLCDC_NS.gr2_clut0()[26],
    ",
  0x5034286cu64 => "
      GLCDC_NS.gr2_clut0()[27],
    ",
  0x50342870u64 => "
      GLCDC_NS.gr2_clut0()[28],
    ",
  0x50342874u64 => "
      GLCDC_NS.gr2_clut0()[29],
    ",
  0x50342878u64 => "
      GLCDC_NS.gr2_clut0()[30],
    ",
  0x5034287cu64 => "
      GLCDC_NS.gr2_clut0()[31],
    ",
  0x50342880u64 => "
      GLCDC_NS.gr2_clut0()[32],
    ",
  0x50342884u64 => "
      GLCDC_NS.gr2_clut0()[33],
    ",
  0x50342888u64 => "
      GLCDC_NS.gr2_clut0()[34],
    ",
  0x5034288cu64 => "
      GLCDC_NS.gr2_clut0()[35],
    ",
  0x50342890u64 => "
      GLCDC_NS.gr2_clut0()[36],
    ",
  0x50342894u64 => "
      GLCDC_NS.gr2_clut0()[37],
    ",
  0x50342898u64 => "
      GLCDC_NS.gr2_clut0()[38],
    ",
  0x5034289cu64 => "
      GLCDC_NS.gr2_clut0()[39],
    ",
  0x503428a0u64 => "
      GLCDC_NS.gr2_clut0()[40],
    ",
  0x503428a4u64 => "
      GLCDC_NS.gr2_clut0()[41],
    ",
  0x503428a8u64 => "
      GLCDC_NS.gr2_clut0()[42],
    ",
  0x503428acu64 => "
      GLCDC_NS.gr2_clut0()[43],
    ",
  0x503428b0u64 => "
      GLCDC_NS.gr2_clut0()[44],
    ",
  0x503428b4u64 => "
      GLCDC_NS.gr2_clut0()[45],
    ",
  0x503428b8u64 => "
      GLCDC_NS.gr2_clut0()[46],
    ",
  0x503428bcu64 => "
      GLCDC_NS.gr2_clut0()[47],
    ",
  0x503428c0u64 => "
      GLCDC_NS.gr2_clut0()[48],
    ",
  0x503428c4u64 => "
      GLCDC_NS.gr2_clut0()[49],
    ",
  0x503428c8u64 => "
      GLCDC_NS.gr2_clut0()[50],
    ",
  0x503428ccu64 => "
      GLCDC_NS.gr2_clut0()[51],
    ",
  0x503428d0u64 => "
      GLCDC_NS.gr2_clut0()[52],
    ",
  0x503428d4u64 => "
      GLCDC_NS.gr2_clut0()[53],
    ",
  0x503428d8u64 => "
      GLCDC_NS.gr2_clut0()[54],
    ",
  0x503428dcu64 => "
      GLCDC_NS.gr2_clut0()[55],
    ",
  0x503428e0u64 => "
      GLCDC_NS.gr2_clut0()[56],
    ",
  0x503428e4u64 => "
      GLCDC_NS.gr2_clut0()[57],
    ",
  0x503428e8u64 => "
      GLCDC_NS.gr2_clut0()[58],
    ",
  0x503428ecu64 => "
      GLCDC_NS.gr2_clut0()[59],
    ",
  0x503428f0u64 => "
      GLCDC_NS.gr2_clut0()[60],
    ",
  0x503428f4u64 => "
      GLCDC_NS.gr2_clut0()[61],
    ",
  0x503428f8u64 => "
      GLCDC_NS.gr2_clut0()[62],
    ",
  0x503428fcu64 => "
      GLCDC_NS.gr2_clut0()[63],
    ",
  0x50342900u64 => "
      GLCDC_NS.gr2_clut0()[64],
    ",
  0x50342904u64 => "
      GLCDC_NS.gr2_clut0()[65],
    ",
  0x50342908u64 => "
      GLCDC_NS.gr2_clut0()[66],
    ",
  0x5034290cu64 => "
      GLCDC_NS.gr2_clut0()[67],
    ",
  0x50342910u64 => "
      GLCDC_NS.gr2_clut0()[68],
    ",
  0x50342914u64 => "
      GLCDC_NS.gr2_clut0()[69],
    ",
  0x50342918u64 => "
      GLCDC_NS.gr2_clut0()[70],
    ",
  0x5034291cu64 => "
      GLCDC_NS.gr2_clut0()[71],
    ",
  0x50342920u64 => "
      GLCDC_NS.gr2_clut0()[72],
    ",
  0x50342924u64 => "
      GLCDC_NS.gr2_clut0()[73],
    ",
  0x50342928u64 => "
      GLCDC_NS.gr2_clut0()[74],
    ",
  0x5034292cu64 => "
      GLCDC_NS.gr2_clut0()[75],
    ",
  0x50342930u64 => "
      GLCDC_NS.gr2_clut0()[76],
    ",
  0x50342934u64 => "
      GLCDC_NS.gr2_clut0()[77],
    ",
  0x50342938u64 => "
      GLCDC_NS.gr2_clut0()[78],
    ",
  0x5034293cu64 => "
      GLCDC_NS.gr2_clut0()[79],
    ",
  0x50342940u64 => "
      GLCDC_NS.gr2_clut0()[80],
    ",
  0x50342944u64 => "
      GLCDC_NS.gr2_clut0()[81],
    ",
  0x50342948u64 => "
      GLCDC_NS.gr2_clut0()[82],
    ",
  0x5034294cu64 => "
      GLCDC_NS.gr2_clut0()[83],
    ",
  0x50342950u64 => "
      GLCDC_NS.gr2_clut0()[84],
    ",
  0x50342954u64 => "
      GLCDC_NS.gr2_clut0()[85],
    ",
  0x50342958u64 => "
      GLCDC_NS.gr2_clut0()[86],
    ",
  0x5034295cu64 => "
      GLCDC_NS.gr2_clut0()[87],
    ",
  0x50342960u64 => "
      GLCDC_NS.gr2_clut0()[88],
    ",
  0x50342964u64 => "
      GLCDC_NS.gr2_clut0()[89],
    ",
  0x50342968u64 => "
      GLCDC_NS.gr2_clut0()[90],
    ",
  0x5034296cu64 => "
      GLCDC_NS.gr2_clut0()[91],
    ",
  0x50342970u64 => "
      GLCDC_NS.gr2_clut0()[92],
    ",
  0x50342974u64 => "
      GLCDC_NS.gr2_clut0()[93],
    ",
  0x50342978u64 => "
      GLCDC_NS.gr2_clut0()[94],
    ",
  0x5034297cu64 => "
      GLCDC_NS.gr2_clut0()[95],
    ",
  0x50342980u64 => "
      GLCDC_NS.gr2_clut0()[96],
    ",
  0x50342984u64 => "
      GLCDC_NS.gr2_clut0()[97],
    ",
  0x50342988u64 => "
      GLCDC_NS.gr2_clut0()[98],
    ",
  0x5034298cu64 => "
      GLCDC_NS.gr2_clut0()[99],
    ",
  0x50342990u64 => "
      GLCDC_NS.gr2_clut0()[100],
    ",
  0x50342994u64 => "
      GLCDC_NS.gr2_clut0()[101],
    ",
  0x50342998u64 => "
      GLCDC_NS.gr2_clut0()[102],
    ",
  0x5034299cu64 => "
      GLCDC_NS.gr2_clut0()[103],
    ",
  0x503429a0u64 => "
      GLCDC_NS.gr2_clut0()[104],
    ",
  0x503429a4u64 => "
      GLCDC_NS.gr2_clut0()[105],
    ",
  0x503429a8u64 => "
      GLCDC_NS.gr2_clut0()[106],
    ",
  0x503429acu64 => "
      GLCDC_NS.gr2_clut0()[107],
    ",
  0x503429b0u64 => "
      GLCDC_NS.gr2_clut0()[108],
    ",
  0x503429b4u64 => "
      GLCDC_NS.gr2_clut0()[109],
    ",
  0x503429b8u64 => "
      GLCDC_NS.gr2_clut0()[110],
    ",
  0x503429bcu64 => "
      GLCDC_NS.gr2_clut0()[111],
    ",
  0x503429c0u64 => "
      GLCDC_NS.gr2_clut0()[112],
    ",
  0x503429c4u64 => "
      GLCDC_NS.gr2_clut0()[113],
    ",
  0x503429c8u64 => "
      GLCDC_NS.gr2_clut0()[114],
    ",
  0x503429ccu64 => "
      GLCDC_NS.gr2_clut0()[115],
    ",
  0x503429d0u64 => "
      GLCDC_NS.gr2_clut0()[116],
    ",
  0x503429d4u64 => "
      GLCDC_NS.gr2_clut0()[117],
    ",
  0x503429d8u64 => "
      GLCDC_NS.gr2_clut0()[118],
    ",
  0x503429dcu64 => "
      GLCDC_NS.gr2_clut0()[119],
    ",
  0x503429e0u64 => "
      GLCDC_NS.gr2_clut0()[120],
    ",
  0x503429e4u64 => "
      GLCDC_NS.gr2_clut0()[121],
    ",
  0x503429e8u64 => "
      GLCDC_NS.gr2_clut0()[122],
    ",
  0x503429ecu64 => "
      GLCDC_NS.gr2_clut0()[123],
    ",
  0x503429f0u64 => "
      GLCDC_NS.gr2_clut0()[124],
    ",
  0x503429f4u64 => "
      GLCDC_NS.gr2_clut0()[125],
    ",
  0x503429f8u64 => "
      GLCDC_NS.gr2_clut0()[126],
    ",
  0x503429fcu64 => "
      GLCDC_NS.gr2_clut0()[127],
    ",
  0x50342a00u64 => "
      GLCDC_NS.gr2_clut0()[128],
    ",
  0x50342a04u64 => "
      GLCDC_NS.gr2_clut0()[129],
    ",
  0x50342a08u64 => "
      GLCDC_NS.gr2_clut0()[130],
    ",
  0x50342a0cu64 => "
      GLCDC_NS.gr2_clut0()[131],
    ",
  0x50342a10u64 => "
      GLCDC_NS.gr2_clut0()[132],
    ",
  0x50342a14u64 => "
      GLCDC_NS.gr2_clut0()[133],
    ",
  0x50342a18u64 => "
      GLCDC_NS.gr2_clut0()[134],
    ",
  0x50342a1cu64 => "
      GLCDC_NS.gr2_clut0()[135],
    ",
  0x50342a20u64 => "
      GLCDC_NS.gr2_clut0()[136],
    ",
  0x50342a24u64 => "
      GLCDC_NS.gr2_clut0()[137],
    ",
  0x50342a28u64 => "
      GLCDC_NS.gr2_clut0()[138],
    ",
  0x50342a2cu64 => "
      GLCDC_NS.gr2_clut0()[139],
    ",
  0x50342a30u64 => "
      GLCDC_NS.gr2_clut0()[140],
    ",
  0x50342a34u64 => "
      GLCDC_NS.gr2_clut0()[141],
    ",
  0x50342a38u64 => "
      GLCDC_NS.gr2_clut0()[142],
    ",
  0x50342a3cu64 => "
      GLCDC_NS.gr2_clut0()[143],
    ",
  0x50342a40u64 => "
      GLCDC_NS.gr2_clut0()[144],
    ",
  0x50342a44u64 => "
      GLCDC_NS.gr2_clut0()[145],
    ",
  0x50342a48u64 => "
      GLCDC_NS.gr2_clut0()[146],
    ",
  0x50342a4cu64 => "
      GLCDC_NS.gr2_clut0()[147],
    ",
  0x50342a50u64 => "
      GLCDC_NS.gr2_clut0()[148],
    ",
  0x50342a54u64 => "
      GLCDC_NS.gr2_clut0()[149],
    ",
  0x50342a58u64 => "
      GLCDC_NS.gr2_clut0()[150],
    ",
  0x50342a5cu64 => "
      GLCDC_NS.gr2_clut0()[151],
    ",
  0x50342a60u64 => "
      GLCDC_NS.gr2_clut0()[152],
    ",
  0x50342a64u64 => "
      GLCDC_NS.gr2_clut0()[153],
    ",
  0x50342a68u64 => "
      GLCDC_NS.gr2_clut0()[154],
    ",
  0x50342a6cu64 => "
      GLCDC_NS.gr2_clut0()[155],
    ",
  0x50342a70u64 => "
      GLCDC_NS.gr2_clut0()[156],
    ",
  0x50342a74u64 => "
      GLCDC_NS.gr2_clut0()[157],
    ",
  0x50342a78u64 => "
      GLCDC_NS.gr2_clut0()[158],
    ",
  0x50342a7cu64 => "
      GLCDC_NS.gr2_clut0()[159],
    ",
  0x50342a80u64 => "
      GLCDC_NS.gr2_clut0()[160],
    ",
  0x50342a84u64 => "
      GLCDC_NS.gr2_clut0()[161],
    ",
  0x50342a88u64 => "
      GLCDC_NS.gr2_clut0()[162],
    ",
  0x50342a8cu64 => "
      GLCDC_NS.gr2_clut0()[163],
    ",
  0x50342a90u64 => "
      GLCDC_NS.gr2_clut0()[164],
    ",
  0x50342a94u64 => "
      GLCDC_NS.gr2_clut0()[165],
    ",
  0x50342a98u64 => "
      GLCDC_NS.gr2_clut0()[166],
    ",
  0x50342a9cu64 => "
      GLCDC_NS.gr2_clut0()[167],
    ",
  0x50342aa0u64 => "
      GLCDC_NS.gr2_clut0()[168],
    ",
  0x50342aa4u64 => "
      GLCDC_NS.gr2_clut0()[169],
    ",
  0x50342aa8u64 => "
      GLCDC_NS.gr2_clut0()[170],
    ",
  0x50342aacu64 => "
      GLCDC_NS.gr2_clut0()[171],
    ",
  0x50342ab0u64 => "
      GLCDC_NS.gr2_clut0()[172],
    ",
  0x50342ab4u64 => "
      GLCDC_NS.gr2_clut0()[173],
    ",
  0x50342ab8u64 => "
      GLCDC_NS.gr2_clut0()[174],
    ",
  0x50342abcu64 => "
      GLCDC_NS.gr2_clut0()[175],
    ",
  0x50342ac0u64 => "
      GLCDC_NS.gr2_clut0()[176],
    ",
  0x50342ac4u64 => "
      GLCDC_NS.gr2_clut0()[177],
    ",
  0x50342ac8u64 => "
      GLCDC_NS.gr2_clut0()[178],
    ",
  0x50342accu64 => "
      GLCDC_NS.gr2_clut0()[179],
    ",
  0x50342ad0u64 => "
      GLCDC_NS.gr2_clut0()[180],
    ",
  0x50342ad4u64 => "
      GLCDC_NS.gr2_clut0()[181],
    ",
  0x50342ad8u64 => "
      GLCDC_NS.gr2_clut0()[182],
    ",
  0x50342adcu64 => "
      GLCDC_NS.gr2_clut0()[183],
    ",
  0x50342ae0u64 => "
      GLCDC_NS.gr2_clut0()[184],
    ",
  0x50342ae4u64 => "
      GLCDC_NS.gr2_clut0()[185],
    ",
  0x50342ae8u64 => "
      GLCDC_NS.gr2_clut0()[186],
    ",
  0x50342aecu64 => "
      GLCDC_NS.gr2_clut0()[187],
    ",
  0x50342af0u64 => "
      GLCDC_NS.gr2_clut0()[188],
    ",
  0x50342af4u64 => "
      GLCDC_NS.gr2_clut0()[189],
    ",
  0x50342af8u64 => "
      GLCDC_NS.gr2_clut0()[190],
    ",
  0x50342afcu64 => "
      GLCDC_NS.gr2_clut0()[191],
    ",
  0x50342b00u64 => "
      GLCDC_NS.gr2_clut0()[192],
    ",
  0x50342b04u64 => "
      GLCDC_NS.gr2_clut0()[193],
    ",
  0x50342b08u64 => "
      GLCDC_NS.gr2_clut0()[194],
    ",
  0x50342b0cu64 => "
      GLCDC_NS.gr2_clut0()[195],
    ",
  0x50342b10u64 => "
      GLCDC_NS.gr2_clut0()[196],
    ",
  0x50342b14u64 => "
      GLCDC_NS.gr2_clut0()[197],
    ",
  0x50342b18u64 => "
      GLCDC_NS.gr2_clut0()[198],
    ",
  0x50342b1cu64 => "
      GLCDC_NS.gr2_clut0()[199],
    ",
  0x50342b20u64 => "
      GLCDC_NS.gr2_clut0()[200],
    ",
  0x50342b24u64 => "
      GLCDC_NS.gr2_clut0()[201],
    ",
  0x50342b28u64 => "
      GLCDC_NS.gr2_clut0()[202],
    ",
  0x50342b2cu64 => "
      GLCDC_NS.gr2_clut0()[203],
    ",
  0x50342b30u64 => "
      GLCDC_NS.gr2_clut0()[204],
    ",
  0x50342b34u64 => "
      GLCDC_NS.gr2_clut0()[205],
    ",
  0x50342b38u64 => "
      GLCDC_NS.gr2_clut0()[206],
    ",
  0x50342b3cu64 => "
      GLCDC_NS.gr2_clut0()[207],
    ",
  0x50342b40u64 => "
      GLCDC_NS.gr2_clut0()[208],
    ",
  0x50342b44u64 => "
      GLCDC_NS.gr2_clut0()[209],
    ",
  0x50342b48u64 => "
      GLCDC_NS.gr2_clut0()[210],
    ",
  0x50342b4cu64 => "
      GLCDC_NS.gr2_clut0()[211],
    ",
  0x50342b50u64 => "
      GLCDC_NS.gr2_clut0()[212],
    ",
  0x50342b54u64 => "
      GLCDC_NS.gr2_clut0()[213],
    ",
  0x50342b58u64 => "
      GLCDC_NS.gr2_clut0()[214],
    ",
  0x50342b5cu64 => "
      GLCDC_NS.gr2_clut0()[215],
    ",
  0x50342b60u64 => "
      GLCDC_NS.gr2_clut0()[216],
    ",
  0x50342b64u64 => "
      GLCDC_NS.gr2_clut0()[217],
    ",
  0x50342b68u64 => "
      GLCDC_NS.gr2_clut0()[218],
    ",
  0x50342b6cu64 => "
      GLCDC_NS.gr2_clut0()[219],
    ",
  0x50342b70u64 => "
      GLCDC_NS.gr2_clut0()[220],
    ",
  0x50342b74u64 => "
      GLCDC_NS.gr2_clut0()[221],
    ",
  0x50342b78u64 => "
      GLCDC_NS.gr2_clut0()[222],
    ",
  0x50342b7cu64 => "
      GLCDC_NS.gr2_clut0()[223],
    ",
  0x50342b80u64 => "
      GLCDC_NS.gr2_clut0()[224],
    ",
  0x50342b84u64 => "
      GLCDC_NS.gr2_clut0()[225],
    ",
  0x50342b88u64 => "
      GLCDC_NS.gr2_clut0()[226],
    ",
  0x50342b8cu64 => "
      GLCDC_NS.gr2_clut0()[227],
    ",
  0x50342b90u64 => "
      GLCDC_NS.gr2_clut0()[228],
    ",
  0x50342b94u64 => "
      GLCDC_NS.gr2_clut0()[229],
    ",
  0x50342b98u64 => "
      GLCDC_NS.gr2_clut0()[230],
    ",
  0x50342b9cu64 => "
      GLCDC_NS.gr2_clut0()[231],
    ",
  0x50342ba0u64 => "
      GLCDC_NS.gr2_clut0()[232],
    ",
  0x50342ba4u64 => "
      GLCDC_NS.gr2_clut0()[233],
    ",
  0x50342ba8u64 => "
      GLCDC_NS.gr2_clut0()[234],
    ",
  0x50342bacu64 => "
      GLCDC_NS.gr2_clut0()[235],
    ",
  0x50342bb0u64 => "
      GLCDC_NS.gr2_clut0()[236],
    ",
  0x50342bb4u64 => "
      GLCDC_NS.gr2_clut0()[237],
    ",
  0x50342bb8u64 => "
      GLCDC_NS.gr2_clut0()[238],
    ",
  0x50342bbcu64 => "
      GLCDC_NS.gr2_clut0()[239],
    ",
  0x50342bc0u64 => "
      GLCDC_NS.gr2_clut0()[240],
    ",
  0x50342bc4u64 => "
      GLCDC_NS.gr2_clut0()[241],
    ",
  0x50342bc8u64 => "
      GLCDC_NS.gr2_clut0()[242],
    ",
  0x50342bccu64 => "
      GLCDC_NS.gr2_clut0()[243],
    ",
  0x50342bd0u64 => "
      GLCDC_NS.gr2_clut0()[244],
    ",
  0x50342bd4u64 => "
      GLCDC_NS.gr2_clut0()[245],
    ",
  0x50342bd8u64 => "
      GLCDC_NS.gr2_clut0()[246],
    ",
  0x50342bdcu64 => "
      GLCDC_NS.gr2_clut0()[247],
    ",
  0x50342be0u64 => "
      GLCDC_NS.gr2_clut0()[248],
    ",
  0x50342be4u64 => "
      GLCDC_NS.gr2_clut0()[249],
    ",
  0x50342be8u64 => "
      GLCDC_NS.gr2_clut0()[250],
    ",
  0x50342becu64 => "
      GLCDC_NS.gr2_clut0()[251],
    ",
  0x50342bf0u64 => "
      GLCDC_NS.gr2_clut0()[252],
    ",
  0x50342bf4u64 => "
      GLCDC_NS.gr2_clut0()[253],
    ",
  0x50342bf8u64 => "
      GLCDC_NS.gr2_clut0()[254],
    ",
  0x50342bfcu64 => "
      GLCDC_NS.gr2_clut0()[255],
    ",
  0x50342c00u64 => "
      GLCDC_NS.gr2_clut1()[0],
    ",
  0x50342c04u64 => "
      GLCDC_NS.gr2_clut1()[1],
    ",
  0x50342c08u64 => "
      GLCDC_NS.gr2_clut1()[2],
    ",
  0x50342c0cu64 => "
      GLCDC_NS.gr2_clut1()[3],
    ",
  0x50342c10u64 => "
      GLCDC_NS.gr2_clut1()[4],
    ",
  0x50342c14u64 => "
      GLCDC_NS.gr2_clut1()[5],
    ",
  0x50342c18u64 => "
      GLCDC_NS.gr2_clut1()[6],
    ",
  0x50342c1cu64 => "
      GLCDC_NS.gr2_clut1()[7],
    ",
  0x50342c20u64 => "
      GLCDC_NS.gr2_clut1()[8],
    ",
  0x50342c24u64 => "
      GLCDC_NS.gr2_clut1()[9],
    ",
  0x50342c28u64 => "
      GLCDC_NS.gr2_clut1()[10],
    ",
  0x50342c2cu64 => "
      GLCDC_NS.gr2_clut1()[11],
    ",
  0x50342c30u64 => "
      GLCDC_NS.gr2_clut1()[12],
    ",
  0x50342c34u64 => "
      GLCDC_NS.gr2_clut1()[13],
    ",
  0x50342c38u64 => "
      GLCDC_NS.gr2_clut1()[14],
    ",
  0x50342c3cu64 => "
      GLCDC_NS.gr2_clut1()[15],
    ",
  0x50342c40u64 => "
      GLCDC_NS.gr2_clut1()[16],
    ",
  0x50342c44u64 => "
      GLCDC_NS.gr2_clut1()[17],
    ",
  0x50342c48u64 => "
      GLCDC_NS.gr2_clut1()[18],
    ",
  0x50342c4cu64 => "
      GLCDC_NS.gr2_clut1()[19],
    ",
  0x50342c50u64 => "
      GLCDC_NS.gr2_clut1()[20],
    ",
  0x50342c54u64 => "
      GLCDC_NS.gr2_clut1()[21],
    ",
  0x50342c58u64 => "
      GLCDC_NS.gr2_clut1()[22],
    ",
  0x50342c5cu64 => "
      GLCDC_NS.gr2_clut1()[23],
    ",
  0x50342c60u64 => "
      GLCDC_NS.gr2_clut1()[24],
    ",
  0x50342c64u64 => "
      GLCDC_NS.gr2_clut1()[25],
    ",
  0x50342c68u64 => "
      GLCDC_NS.gr2_clut1()[26],
    ",
  0x50342c6cu64 => "
      GLCDC_NS.gr2_clut1()[27],
    ",
  0x50342c70u64 => "
      GLCDC_NS.gr2_clut1()[28],
    ",
  0x50342c74u64 => "
      GLCDC_NS.gr2_clut1()[29],
    ",
  0x50342c78u64 => "
      GLCDC_NS.gr2_clut1()[30],
    ",
  0x50342c7cu64 => "
      GLCDC_NS.gr2_clut1()[31],
    ",
  0x50342c80u64 => "
      GLCDC_NS.gr2_clut1()[32],
    ",
  0x50342c84u64 => "
      GLCDC_NS.gr2_clut1()[33],
    ",
  0x50342c88u64 => "
      GLCDC_NS.gr2_clut1()[34],
    ",
  0x50342c8cu64 => "
      GLCDC_NS.gr2_clut1()[35],
    ",
  0x50342c90u64 => "
      GLCDC_NS.gr2_clut1()[36],
    ",
  0x50342c94u64 => "
      GLCDC_NS.gr2_clut1()[37],
    ",
  0x50342c98u64 => "
      GLCDC_NS.gr2_clut1()[38],
    ",
  0x50342c9cu64 => "
      GLCDC_NS.gr2_clut1()[39],
    ",
  0x50342ca0u64 => "
      GLCDC_NS.gr2_clut1()[40],
    ",
  0x50342ca4u64 => "
      GLCDC_NS.gr2_clut1()[41],
    ",
  0x50342ca8u64 => "
      GLCDC_NS.gr2_clut1()[42],
    ",
  0x50342cacu64 => "
      GLCDC_NS.gr2_clut1()[43],
    ",
  0x50342cb0u64 => "
      GLCDC_NS.gr2_clut1()[44],
    ",
  0x50342cb4u64 => "
      GLCDC_NS.gr2_clut1()[45],
    ",
  0x50342cb8u64 => "
      GLCDC_NS.gr2_clut1()[46],
    ",
  0x50342cbcu64 => "
      GLCDC_NS.gr2_clut1()[47],
    ",
  0x50342cc0u64 => "
      GLCDC_NS.gr2_clut1()[48],
    ",
  0x50342cc4u64 => "
      GLCDC_NS.gr2_clut1()[49],
    ",
  0x50342cc8u64 => "
      GLCDC_NS.gr2_clut1()[50],
    ",
  0x50342cccu64 => "
      GLCDC_NS.gr2_clut1()[51],
    ",
  0x50342cd0u64 => "
      GLCDC_NS.gr2_clut1()[52],
    ",
  0x50342cd4u64 => "
      GLCDC_NS.gr2_clut1()[53],
    ",
  0x50342cd8u64 => "
      GLCDC_NS.gr2_clut1()[54],
    ",
  0x50342cdcu64 => "
      GLCDC_NS.gr2_clut1()[55],
    ",
  0x50342ce0u64 => "
      GLCDC_NS.gr2_clut1()[56],
    ",
  0x50342ce4u64 => "
      GLCDC_NS.gr2_clut1()[57],
    ",
  0x50342ce8u64 => "
      GLCDC_NS.gr2_clut1()[58],
    ",
  0x50342cecu64 => "
      GLCDC_NS.gr2_clut1()[59],
    ",
  0x50342cf0u64 => "
      GLCDC_NS.gr2_clut1()[60],
    ",
  0x50342cf4u64 => "
      GLCDC_NS.gr2_clut1()[61],
    ",
  0x50342cf8u64 => "
      GLCDC_NS.gr2_clut1()[62],
    ",
  0x50342cfcu64 => "
      GLCDC_NS.gr2_clut1()[63],
    ",
  0x50342d00u64 => "
      GLCDC_NS.gr2_clut1()[64],
    ",
  0x50342d04u64 => "
      GLCDC_NS.gr2_clut1()[65],
    ",
  0x50342d08u64 => "
      GLCDC_NS.gr2_clut1()[66],
    ",
  0x50342d0cu64 => "
      GLCDC_NS.gr2_clut1()[67],
    ",
  0x50342d10u64 => "
      GLCDC_NS.gr2_clut1()[68],
    ",
  0x50342d14u64 => "
      GLCDC_NS.gr2_clut1()[69],
    ",
  0x50342d18u64 => "
      GLCDC_NS.gr2_clut1()[70],
    ",
  0x50342d1cu64 => "
      GLCDC_NS.gr2_clut1()[71],
    ",
  0x50342d20u64 => "
      GLCDC_NS.gr2_clut1()[72],
    ",
  0x50342d24u64 => "
      GLCDC_NS.gr2_clut1()[73],
    ",
  0x50342d28u64 => "
      GLCDC_NS.gr2_clut1()[74],
    ",
  0x50342d2cu64 => "
      GLCDC_NS.gr2_clut1()[75],
    ",
  0x50342d30u64 => "
      GLCDC_NS.gr2_clut1()[76],
    ",
  0x50342d34u64 => "
      GLCDC_NS.gr2_clut1()[77],
    ",
  0x50342d38u64 => "
      GLCDC_NS.gr2_clut1()[78],
    ",
  0x50342d3cu64 => "
      GLCDC_NS.gr2_clut1()[79],
    ",
  0x50342d40u64 => "
      GLCDC_NS.gr2_clut1()[80],
    ",
  0x50342d44u64 => "
      GLCDC_NS.gr2_clut1()[81],
    ",
  0x50342d48u64 => "
      GLCDC_NS.gr2_clut1()[82],
    ",
  0x50342d4cu64 => "
      GLCDC_NS.gr2_clut1()[83],
    ",
  0x50342d50u64 => "
      GLCDC_NS.gr2_clut1()[84],
    ",
  0x50342d54u64 => "
      GLCDC_NS.gr2_clut1()[85],
    ",
  0x50342d58u64 => "
      GLCDC_NS.gr2_clut1()[86],
    ",
  0x50342d5cu64 => "
      GLCDC_NS.gr2_clut1()[87],
    ",
  0x50342d60u64 => "
      GLCDC_NS.gr2_clut1()[88],
    ",
  0x50342d64u64 => "
      GLCDC_NS.gr2_clut1()[89],
    ",
  0x50342d68u64 => "
      GLCDC_NS.gr2_clut1()[90],
    ",
  0x50342d6cu64 => "
      GLCDC_NS.gr2_clut1()[91],
    ",
  0x50342d70u64 => "
      GLCDC_NS.gr2_clut1()[92],
    ",
  0x50342d74u64 => "
      GLCDC_NS.gr2_clut1()[93],
    ",
  0x50342d78u64 => "
      GLCDC_NS.gr2_clut1()[94],
    ",
  0x50342d7cu64 => "
      GLCDC_NS.gr2_clut1()[95],
    ",
  0x50342d80u64 => "
      GLCDC_NS.gr2_clut1()[96],
    ",
  0x50342d84u64 => "
      GLCDC_NS.gr2_clut1()[97],
    ",
  0x50342d88u64 => "
      GLCDC_NS.gr2_clut1()[98],
    ",
  0x50342d8cu64 => "
      GLCDC_NS.gr2_clut1()[99],
    ",
  0x50342d90u64 => "
      GLCDC_NS.gr2_clut1()[100],
    ",
  0x50342d94u64 => "
      GLCDC_NS.gr2_clut1()[101],
    ",
  0x50342d98u64 => "
      GLCDC_NS.gr2_clut1()[102],
    ",
  0x50342d9cu64 => "
      GLCDC_NS.gr2_clut1()[103],
    ",
  0x50342da0u64 => "
      GLCDC_NS.gr2_clut1()[104],
    ",
  0x50342da4u64 => "
      GLCDC_NS.gr2_clut1()[105],
    ",
  0x50342da8u64 => "
      GLCDC_NS.gr2_clut1()[106],
    ",
  0x50342dacu64 => "
      GLCDC_NS.gr2_clut1()[107],
    ",
  0x50342db0u64 => "
      GLCDC_NS.gr2_clut1()[108],
    ",
  0x50342db4u64 => "
      GLCDC_NS.gr2_clut1()[109],
    ",
  0x50342db8u64 => "
      GLCDC_NS.gr2_clut1()[110],
    ",
  0x50342dbcu64 => "
      GLCDC_NS.gr2_clut1()[111],
    ",
  0x50342dc0u64 => "
      GLCDC_NS.gr2_clut1()[112],
    ",
  0x50342dc4u64 => "
      GLCDC_NS.gr2_clut1()[113],
    ",
  0x50342dc8u64 => "
      GLCDC_NS.gr2_clut1()[114],
    ",
  0x50342dccu64 => "
      GLCDC_NS.gr2_clut1()[115],
    ",
  0x50342dd0u64 => "
      GLCDC_NS.gr2_clut1()[116],
    ",
  0x50342dd4u64 => "
      GLCDC_NS.gr2_clut1()[117],
    ",
  0x50342dd8u64 => "
      GLCDC_NS.gr2_clut1()[118],
    ",
  0x50342ddcu64 => "
      GLCDC_NS.gr2_clut1()[119],
    ",
  0x50342de0u64 => "
      GLCDC_NS.gr2_clut1()[120],
    ",
  0x50342de4u64 => "
      GLCDC_NS.gr2_clut1()[121],
    ",
  0x50342de8u64 => "
      GLCDC_NS.gr2_clut1()[122],
    ",
  0x50342decu64 => "
      GLCDC_NS.gr2_clut1()[123],
    ",
  0x50342df0u64 => "
      GLCDC_NS.gr2_clut1()[124],
    ",
  0x50342df4u64 => "
      GLCDC_NS.gr2_clut1()[125],
    ",
  0x50342df8u64 => "
      GLCDC_NS.gr2_clut1()[126],
    ",
  0x50342dfcu64 => "
      GLCDC_NS.gr2_clut1()[127],
    ",
  0x50342e00u64 => "
      GLCDC_NS.gr2_clut1()[128],
    ",
  0x50342e04u64 => "
      GLCDC_NS.gr2_clut1()[129],
    ",
  0x50342e08u64 => "
      GLCDC_NS.gr2_clut1()[130],
    ",
  0x50342e0cu64 => "
      GLCDC_NS.gr2_clut1()[131],
    ",
  0x50342e10u64 => "
      GLCDC_NS.gr2_clut1()[132],
    ",
  0x50342e14u64 => "
      GLCDC_NS.gr2_clut1()[133],
    ",
  0x50342e18u64 => "
      GLCDC_NS.gr2_clut1()[134],
    ",
  0x50342e1cu64 => "
      GLCDC_NS.gr2_clut1()[135],
    ",
  0x50342e20u64 => "
      GLCDC_NS.gr2_clut1()[136],
    ",
  0x50342e24u64 => "
      GLCDC_NS.gr2_clut1()[137],
    ",
  0x50342e28u64 => "
      GLCDC_NS.gr2_clut1()[138],
    ",
  0x50342e2cu64 => "
      GLCDC_NS.gr2_clut1()[139],
    ",
  0x50342e30u64 => "
      GLCDC_NS.gr2_clut1()[140],
    ",
  0x50342e34u64 => "
      GLCDC_NS.gr2_clut1()[141],
    ",
  0x50342e38u64 => "
      GLCDC_NS.gr2_clut1()[142],
    ",
  0x50342e3cu64 => "
      GLCDC_NS.gr2_clut1()[143],
    ",
  0x50342e40u64 => "
      GLCDC_NS.gr2_clut1()[144],
    ",
  0x50342e44u64 => "
      GLCDC_NS.gr2_clut1()[145],
    ",
  0x50342e48u64 => "
      GLCDC_NS.gr2_clut1()[146],
    ",
  0x50342e4cu64 => "
      GLCDC_NS.gr2_clut1()[147],
    ",
  0x50342e50u64 => "
      GLCDC_NS.gr2_clut1()[148],
    ",
  0x50342e54u64 => "
      GLCDC_NS.gr2_clut1()[149],
    ",
  0x50342e58u64 => "
      GLCDC_NS.gr2_clut1()[150],
    ",
  0x50342e5cu64 => "
      GLCDC_NS.gr2_clut1()[151],
    ",
  0x50342e60u64 => "
      GLCDC_NS.gr2_clut1()[152],
    ",
  0x50342e64u64 => "
      GLCDC_NS.gr2_clut1()[153],
    ",
  0x50342e68u64 => "
      GLCDC_NS.gr2_clut1()[154],
    ",
  0x50342e6cu64 => "
      GLCDC_NS.gr2_clut1()[155],
    ",
  0x50342e70u64 => "
      GLCDC_NS.gr2_clut1()[156],
    ",
  0x50342e74u64 => "
      GLCDC_NS.gr2_clut1()[157],
    ",
  0x50342e78u64 => "
      GLCDC_NS.gr2_clut1()[158],
    ",
  0x50342e7cu64 => "
      GLCDC_NS.gr2_clut1()[159],
    ",
  0x50342e80u64 => "
      GLCDC_NS.gr2_clut1()[160],
    ",
  0x50342e84u64 => "
      GLCDC_NS.gr2_clut1()[161],
    ",
  0x50342e88u64 => "
      GLCDC_NS.gr2_clut1()[162],
    ",
  0x50342e8cu64 => "
      GLCDC_NS.gr2_clut1()[163],
    ",
  0x50342e90u64 => "
      GLCDC_NS.gr2_clut1()[164],
    ",
  0x50342e94u64 => "
      GLCDC_NS.gr2_clut1()[165],
    ",
  0x50342e98u64 => "
      GLCDC_NS.gr2_clut1()[166],
    ",
  0x50342e9cu64 => "
      GLCDC_NS.gr2_clut1()[167],
    ",
  0x50342ea0u64 => "
      GLCDC_NS.gr2_clut1()[168],
    ",
  0x50342ea4u64 => "
      GLCDC_NS.gr2_clut1()[169],
    ",
  0x50342ea8u64 => "
      GLCDC_NS.gr2_clut1()[170],
    ",
  0x50342eacu64 => "
      GLCDC_NS.gr2_clut1()[171],
    ",
  0x50342eb0u64 => "
      GLCDC_NS.gr2_clut1()[172],
    ",
  0x50342eb4u64 => "
      GLCDC_NS.gr2_clut1()[173],
    ",
  0x50342eb8u64 => "
      GLCDC_NS.gr2_clut1()[174],
    ",
  0x50342ebcu64 => "
      GLCDC_NS.gr2_clut1()[175],
    ",
  0x50342ec0u64 => "
      GLCDC_NS.gr2_clut1()[176],
    ",
  0x50342ec4u64 => "
      GLCDC_NS.gr2_clut1()[177],
    ",
  0x50342ec8u64 => "
      GLCDC_NS.gr2_clut1()[178],
    ",
  0x50342eccu64 => "
      GLCDC_NS.gr2_clut1()[179],
    ",
  0x50342ed0u64 => "
      GLCDC_NS.gr2_clut1()[180],
    ",
  0x50342ed4u64 => "
      GLCDC_NS.gr2_clut1()[181],
    ",
  0x50342ed8u64 => "
      GLCDC_NS.gr2_clut1()[182],
    ",
  0x50342edcu64 => "
      GLCDC_NS.gr2_clut1()[183],
    ",
  0x50342ee0u64 => "
      GLCDC_NS.gr2_clut1()[184],
    ",
  0x50342ee4u64 => "
      GLCDC_NS.gr2_clut1()[185],
    ",
  0x50342ee8u64 => "
      GLCDC_NS.gr2_clut1()[186],
    ",
  0x50342eecu64 => "
      GLCDC_NS.gr2_clut1()[187],
    ",
  0x50342ef0u64 => "
      GLCDC_NS.gr2_clut1()[188],
    ",
  0x50342ef4u64 => "
      GLCDC_NS.gr2_clut1()[189],
    ",
  0x50342ef8u64 => "
      GLCDC_NS.gr2_clut1()[190],
    ",
  0x50342efcu64 => "
      GLCDC_NS.gr2_clut1()[191],
    ",
  0x50342f00u64 => "
      GLCDC_NS.gr2_clut1()[192],
    ",
  0x50342f04u64 => "
      GLCDC_NS.gr2_clut1()[193],
    ",
  0x50342f08u64 => "
      GLCDC_NS.gr2_clut1()[194],
    ",
  0x50342f0cu64 => "
      GLCDC_NS.gr2_clut1()[195],
    ",
  0x50342f10u64 => "
      GLCDC_NS.gr2_clut1()[196],
    ",
  0x50342f14u64 => "
      GLCDC_NS.gr2_clut1()[197],
    ",
  0x50342f18u64 => "
      GLCDC_NS.gr2_clut1()[198],
    ",
  0x50342f1cu64 => "
      GLCDC_NS.gr2_clut1()[199],
    ",
  0x50342f20u64 => "
      GLCDC_NS.gr2_clut1()[200],
    ",
  0x50342f24u64 => "
      GLCDC_NS.gr2_clut1()[201],
    ",
  0x50342f28u64 => "
      GLCDC_NS.gr2_clut1()[202],
    ",
  0x50342f2cu64 => "
      GLCDC_NS.gr2_clut1()[203],
    ",
  0x50342f30u64 => "
      GLCDC_NS.gr2_clut1()[204],
    ",
  0x50342f34u64 => "
      GLCDC_NS.gr2_clut1()[205],
    ",
  0x50342f38u64 => "
      GLCDC_NS.gr2_clut1()[206],
    ",
  0x50342f3cu64 => "
      GLCDC_NS.gr2_clut1()[207],
    ",
  0x50342f40u64 => "
      GLCDC_NS.gr2_clut1()[208],
    ",
  0x50342f44u64 => "
      GLCDC_NS.gr2_clut1()[209],
    ",
  0x50342f48u64 => "
      GLCDC_NS.gr2_clut1()[210],
    ",
  0x50342f4cu64 => "
      GLCDC_NS.gr2_clut1()[211],
    ",
  0x50342f50u64 => "
      GLCDC_NS.gr2_clut1()[212],
    ",
  0x50342f54u64 => "
      GLCDC_NS.gr2_clut1()[213],
    ",
  0x50342f58u64 => "
      GLCDC_NS.gr2_clut1()[214],
    ",
  0x50342f5cu64 => "
      GLCDC_NS.gr2_clut1()[215],
    ",
  0x50342f60u64 => "
      GLCDC_NS.gr2_clut1()[216],
    ",
  0x50342f64u64 => "
      GLCDC_NS.gr2_clut1()[217],
    ",
  0x50342f68u64 => "
      GLCDC_NS.gr2_clut1()[218],
    ",
  0x50342f6cu64 => "
      GLCDC_NS.gr2_clut1()[219],
    ",
  0x50342f70u64 => "
      GLCDC_NS.gr2_clut1()[220],
    ",
  0x50342f74u64 => "
      GLCDC_NS.gr2_clut1()[221],
    ",
  0x50342f78u64 => "
      GLCDC_NS.gr2_clut1()[222],
    ",
  0x50342f7cu64 => "
      GLCDC_NS.gr2_clut1()[223],
    ",
  0x50342f80u64 => "
      GLCDC_NS.gr2_clut1()[224],
    ",
  0x50342f84u64 => "
      GLCDC_NS.gr2_clut1()[225],
    ",
  0x50342f88u64 => "
      GLCDC_NS.gr2_clut1()[226],
    ",
  0x50342f8cu64 => "
      GLCDC_NS.gr2_clut1()[227],
    ",
  0x50342f90u64 => "
      GLCDC_NS.gr2_clut1()[228],
    ",
  0x50342f94u64 => "
      GLCDC_NS.gr2_clut1()[229],
    ",
  0x50342f98u64 => "
      GLCDC_NS.gr2_clut1()[230],
    ",
  0x50342f9cu64 => "
      GLCDC_NS.gr2_clut1()[231],
    ",
  0x50342fa0u64 => "
      GLCDC_NS.gr2_clut1()[232],
    ",
  0x50342fa4u64 => "
      GLCDC_NS.gr2_clut1()[233],
    ",
  0x50342fa8u64 => "
      GLCDC_NS.gr2_clut1()[234],
    ",
  0x50342facu64 => "
      GLCDC_NS.gr2_clut1()[235],
    ",
  0x50342fb0u64 => "
      GLCDC_NS.gr2_clut1()[236],
    ",
  0x50342fb4u64 => "
      GLCDC_NS.gr2_clut1()[237],
    ",
  0x50342fb8u64 => "
      GLCDC_NS.gr2_clut1()[238],
    ",
  0x50342fbcu64 => "
      GLCDC_NS.gr2_clut1()[239],
    ",
  0x50342fc0u64 => "
      GLCDC_NS.gr2_clut1()[240],
    ",
  0x50342fc4u64 => "
      GLCDC_NS.gr2_clut1()[241],
    ",
  0x50342fc8u64 => "
      GLCDC_NS.gr2_clut1()[242],
    ",
  0x50342fccu64 => "
      GLCDC_NS.gr2_clut1()[243],
    ",
  0x50342fd0u64 => "
      GLCDC_NS.gr2_clut1()[244],
    ",
  0x50342fd4u64 => "
      GLCDC_NS.gr2_clut1()[245],
    ",
  0x50342fd8u64 => "
      GLCDC_NS.gr2_clut1()[246],
    ",
  0x50342fdcu64 => "
      GLCDC_NS.gr2_clut1()[247],
    ",
  0x50342fe0u64 => "
      GLCDC_NS.gr2_clut1()[248],
    ",
  0x50342fe4u64 => "
      GLCDC_NS.gr2_clut1()[249],
    ",
  0x50342fe8u64 => "
      GLCDC_NS.gr2_clut1()[250],
    ",
  0x50342fecu64 => "
      GLCDC_NS.gr2_clut1()[251],
    ",
  0x50342ff0u64 => "
      GLCDC_NS.gr2_clut1()[252],
    ",
  0x50342ff4u64 => "
      GLCDC_NS.gr2_clut1()[253],
    ",
  0x50342ff8u64 => "
      GLCDC_NS.gr2_clut1()[254],
    ",
  0x50342ffcu64 => "
      GLCDC_NS.gr2_clut1()[255],
    ",
  0x50343000u64 => "
      GLCDC_NS.bg_en(),
    ",
  0x50343004u64 => "
      GLCDC_NS.bg_peri(),
    ",
  0x50343008u64 => "
      GLCDC_NS.bg_sync(),
    ",
  0x5034300cu64 => "
      GLCDC_NS.bg_vsize(),
    ",
  0x50343010u64 => "
      GLCDC_NS.bg_hsize(),
    ",
  0x50343014u64 => "
      GLCDC_NS.bg_bgc(),
    ",
  0x50343018u64 => "
      GLCDC_NS.bg_mon(),
    ",
  0x50343100u64 => "
      GLCDC_NS.gr_ven()[0],
    ",
  0x50343200u64 => "
      GLCDC_NS.gr_ven()[1],
    ",
  0x50343104u64 => "
      GLCDC_NS.gr_flmrd()[0],
    ",
  0x50343204u64 => "
      GLCDC_NS.gr_flmrd()[1],
    ",
  0x50343108u64 => "
      GLCDC_NS.gr_flm1()[0],
    ",
  0x50343208u64 => "
      GLCDC_NS.gr_flm1()[1],
    ",
  0x5034310cu64 => "
      GLCDC_NS.gr_flm2()[0],
    ",
  0x5034320cu64 => "
      GLCDC_NS.gr_flm2()[1],
    ",
  0x50343110u64 => "
      GLCDC_NS.gr_flm3()[0],
    ",
  0x50343210u64 => "
      GLCDC_NS.gr_flm3()[1],
    ",
  0x50343118u64 => "
      GLCDC_NS.gr_flm5()[0],
    ",
  0x50343218u64 => "
      GLCDC_NS.gr_flm5()[1],
    ",
  0x5034311cu64 => "
      GLCDC_NS.gr_flm6()[0],
    ",
  0x5034321cu64 => "
      GLCDC_NS.gr_flm6()[1],
    ",
  0x50343120u64 => "
      GLCDC_NS.gr_ab1()[0],
    ",
  0x50343220u64 => "
      GLCDC_NS.gr_ab1()[1],
    ",
  0x50343124u64 => "
      GLCDC_NS.gr_ab2()[0],
    ",
  0x50343224u64 => "
      GLCDC_NS.gr_ab2()[1],
    ",
  0x50343128u64 => "
      GLCDC_NS.gr_ab3()[0],
    ",
  0x50343228u64 => "
      GLCDC_NS.gr_ab3()[1],
    ",
  0x5034312cu64 => "
      GLCDC_NS.gr_ab4()[0],
    ",
  0x5034322cu64 => "
      GLCDC_NS.gr_ab4()[1],
    ",
  0x50343130u64 => "
      GLCDC_NS.gr_ab5()[0],
    ",
  0x50343230u64 => "
      GLCDC_NS.gr_ab5()[1],
    ",
  0x50343134u64 => "
      GLCDC_NS.gr_ab6()[0],
    ",
  0x50343234u64 => "
      GLCDC_NS.gr_ab6()[1],
    ",
  0x50343138u64 => "
      GLCDC_NS.gr_ab7()[0],
    ",
  0x50343238u64 => "
      GLCDC_NS.gr_ab7()[1],
    ",
  0x5034313cu64 => "
      GLCDC_NS.gr_ab8()[0],
    ",
  0x5034323cu64 => "
      GLCDC_NS.gr_ab8()[1],
    ",
  0x50343140u64 => "
      GLCDC_NS.gr_ab9()[0],
    ",
  0x50343240u64 => "
      GLCDC_NS.gr_ab9()[1],
    ",
  0x5034314cu64 => "
      GLCDC_NS.gr_base()[0],
    ",
  0x5034324cu64 => "
      GLCDC_NS.gr_base()[1],
    ",
  0x50343150u64 => "
      GLCDC_NS.gr_clutint()[0],
    ",
  0x50343250u64 => "
      GLCDC_NS.gr_clutint()[1],
    ",
  0x50343154u64 => "
      GLCDC_NS.gr_mon()[0],
    ",
  0x50343254u64 => "
      GLCDC_NS.gr_mon()[1],
    ",
  0x50343300u64 => "
      GLCDC_NS.gamg_latch(),
    ",
  0x50343304u64 => "
      GLCDC_NS.gam_sw(),
    ",
  0x50343308u64 => "
      GLCDC_NS.gamg_lut1(),
    ",
  0x5034330cu64 => "
      GLCDC_NS.gamg_lut2(),
    ",
  0x50343310u64 => "
      GLCDC_NS.gamg_lut3(),
    ",
  0x50343314u64 => "
      GLCDC_NS.gamg_lut4(),
    ",
  0x50343318u64 => "
      GLCDC_NS.gamg_lut5(),
    ",
  0x5034331cu64 => "
      GLCDC_NS.gamg_lut6(),
    ",
  0x50343320u64 => "
      GLCDC_NS.gamg_lut7(),
    ",
  0x50343324u64 => "
      GLCDC_NS.gamg_lut8(),
    ",
  0x50343328u64 => "
      GLCDC_NS.gamg_area1(),
    ",
  0x5034332cu64 => "
      GLCDC_NS.gamg_area2(),
    ",
  0x50343330u64 => "
      GLCDC_NS.gamg_area3(),
    ",
  0x50343334u64 => "
      GLCDC_NS.gamg_area4(),
    ",
  0x50343338u64 => "
      GLCDC_NS.gamg_area5(),
    ",
  0x50343340u64 => "
      GLCDC_NS.gamb_latch(),
    ",
  0x50343348u64 => "
      GLCDC_NS.gamb_lut1(),
    ",
  0x5034334cu64 => "
      GLCDC_NS.gamb_lut2(),
    ",
  0x50343350u64 => "
      GLCDC_NS.gamb_lut3(),
    ",
  0x50343354u64 => "
      GLCDC_NS.gamb_lut4(),
    ",
  0x50343358u64 => "
      GLCDC_NS.gamb_lut5(),
    ",
  0x5034335cu64 => "
      GLCDC_NS.gamb_lut6(),
    ",
  0x50343360u64 => "
      GLCDC_NS.gamb_lut7(),
    ",
  0x50343364u64 => "
      GLCDC_NS.gamb_lut8(),
    ",
  0x50343368u64 => "
      GLCDC_NS.gamb_area1(),
    ",
  0x5034336cu64 => "
      GLCDC_NS.gamb_area2(),
    ",
  0x50343370u64 => "
      GLCDC_NS.gamb_area3(),
    ",
  0x50343374u64 => "
      GLCDC_NS.gamb_area4(),
    ",
  0x50343378u64 => "
      GLCDC_NS.gamb_area5(),
    ",
  0x50343380u64 => "
      GLCDC_NS.gamr_latch(),
    ",
  0x50343388u64 => "
      GLCDC_NS.gamr_lut1(),
    ",
  0x5034338cu64 => "
      GLCDC_NS.gamr_lut2(),
    ",
  0x50343390u64 => "
      GLCDC_NS.gamr_lut3(),
    ",
  0x50343394u64 => "
      GLCDC_NS.gamr_lut4(),
    ",
  0x50343398u64 => "
      GLCDC_NS.gamr_lut5(),
    ",
  0x5034339cu64 => "
      GLCDC_NS.gamr_lut6(),
    ",
  0x503433a0u64 => "
      GLCDC_NS.gamr_lut7(),
    ",
  0x503433a4u64 => "
      GLCDC_NS.gamr_lut8(),
    ",
  0x503433a8u64 => "
      GLCDC_NS.gamr_area1(),
    ",
  0x503433acu64 => "
      GLCDC_NS.gamr_area2(),
    ",
  0x503433b0u64 => "
      GLCDC_NS.gamr_area3(),
    ",
  0x503433b4u64 => "
      GLCDC_NS.gamr_area4(),
    ",
  0x503433b8u64 => "
      GLCDC_NS.gamr_area5(),
    ",
  0x503433c0u64 => "
      GLCDC_NS.out_vlatch(),
    ",
  0x503433c4u64 => "
      GLCDC_NS.out_set(),
    ",
  0x503433c8u64 => "
      GLCDC_NS.out_bright1(),
    ",
  0x503433ccu64 => "
      GLCDC_NS.out_bright2(),
    ",
  0x503433d0u64 => "
      GLCDC_NS.out_contrast(),
    ",
  0x503433d4u64 => "
      GLCDC_NS.out_pdtha(),
    ",
  0x503433e4u64 => "
      GLCDC_NS.out_clkphase(),
    ",
  0x50343404u64 => "
      GLCDC_NS.tcon_tim(),
    ",
  0x50343408u64 => "
      GLCDC_NS.tcon_stva1(),
    ",
  0x5034340cu64 => "
      GLCDC_NS.tcon_stva2(),
    ",
  0x50343410u64 => "
      GLCDC_NS.tcon_stvb1(),
    ",
  0x50343414u64 => "
      GLCDC_NS.tcon_stvb2(),
    ",
  0x50343418u64 => "
      GLCDC_NS.tcon_stha1(),
    ",
  0x5034341cu64 => "
      GLCDC_NS.tcon_stha2(),
    ",
  0x50343420u64 => "
      GLCDC_NS.tcon_sthb1(),
    ",
  0x50343424u64 => "
      GLCDC_NS.tcon_sthb2(),
    ",
  0x50343428u64 => "
      GLCDC_NS.tcon_de(),
    ",
  0x50343440u64 => "
      GLCDC_NS.syscnt_dtcten(),
    ",
  0x50343444u64 => "
      GLCDC_NS.syscnt_inten(),
    ",
  0x50343448u64 => "
      GLCDC_NS.syscnt_stclr(),
    ",
  0x5034344cu64 => "
      GLCDC_NS.syscnt_stmon(),
    ",
  0x50343450u64 => "
      GLCDC_NS.syscnt_panel_clk(),
    ",
  0x50344000u64 => "
      DRW_NS.control(),
      DRW_NS.status(),
    ",
  0x50344004u64 => "
      DRW_NS.control2(),
      DRW_NS.hwrevision(),
    ",
  0x50344010u64 => "
      DRW_NS.lstart()[0],
    ",
  0x50344014u64 => "
      DRW_NS.lstart()[1],
    ",
  0x50344018u64 => "
      DRW_NS.lstart()[2],
    ",
  0x5034401cu64 => "
      DRW_NS.lstart()[3],
    ",
  0x50344020u64 => "
      DRW_NS.lstart()[4],
    ",
  0x50344024u64 => "
      DRW_NS.lstart()[5],
    ",
  0x50344028u64 => "
      DRW_NS.lxadd()[0],
    ",
  0x5034402cu64 => "
      DRW_NS.lxadd()[1],
    ",
  0x50344030u64 => "
      DRW_NS.lxadd()[2],
    ",
  0x50344034u64 => "
      DRW_NS.lxadd()[3],
    ",
  0x50344038u64 => "
      DRW_NS.lxadd()[4],
    ",
  0x5034403cu64 => "
      DRW_NS.lxadd()[5],
    ",
  0x50344040u64 => "
      DRW_NS.lyadd()[0],
    ",
  0x50344044u64 => "
      DRW_NS.lyadd()[1],
    ",
  0x50344048u64 => "
      DRW_NS.lyadd()[2],
    ",
  0x5034404cu64 => "
      DRW_NS.lyadd()[3],
    ",
  0x50344050u64 => "
      DRW_NS.lyadd()[4],
    ",
  0x50344054u64 => "
      DRW_NS.lyadd()[5],
    ",
  0x50344058u64 => "
      DRW_NS.lband()[0],
    ",
  0x5034405cu64 => "
      DRW_NS.lband()[1],
    ",
  0x50344064u64 => "
      DRW_NS.color1(),
    ",
  0x50344068u64 => "
      DRW_NS.color2(),
    ",
  0x50344074u64 => "
      DRW_NS.pattern(),
    ",
  0x50344078u64 => "
      DRW_NS.size(),
    ",
  0x5034407cu64 => "
      DRW_NS.pitch(),
    ",
  0x50344080u64 => "
      DRW_NS.origin(),
    ",
  0x50344090u64 => "
      DRW_NS.lustart(),
    ",
  0x50344094u64 => "
      DRW_NS.luxadd(),
    ",
  0x50344098u64 => "
      DRW_NS.luyadd(),
    ",
  0x5034409cu64 => "
      DRW_NS.lvstarti(),
    ",
  0x503440a0u64 => "
      DRW_NS.lvstartf(),
    ",
  0x503440a4u64 => "
      DRW_NS.lvxaddi(),
    ",
  0x503440a8u64 => "
      DRW_NS.lvyaddi(),
    ",
  0x503440acu64 => "
      DRW_NS.lvyxaddf(),
    ",
  0x503440b4u64 => "
      DRW_NS.texpitch(),
    ",
  0x503440b8u64 => "
      DRW_NS.texmask(),
    ",
  0x503440bcu64 => "
      DRW_NS.texorigin(),
    ",
  0x503440c0u64 => "
      DRW_NS.irqctl(),
    ",
  0x503440c4u64 => "
      DRW_NS.cachectl(),
    ",
  0x503440c8u64 => "
      DRW_NS.dliststart(),
    ",
  0x503440ccu64 => "
      DRW_NS.perfcount1(),
      DRW_NS.perfcount2(),
    ",
  0x503440d4u64 => "
      DRW_NS.perftrigger(),
    ",
  0x503440dcu64 => "
      DRW_NS.texcladdr(),
    ",
  0x503440e0u64 => "
      DRW_NS.texcldata(),
    ",
  0x503440e4u64 => "
      DRW_NS.texcloffset(),
    ",
  0x503440e8u64 => "
      DRW_NS.colkey(),
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
