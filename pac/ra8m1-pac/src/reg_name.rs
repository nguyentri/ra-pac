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
// Generated from SVD 1.2, with svd2pac 0.6.1 on Sun, 15 Mar 2026 06:57:20 +0000

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
  0x40000b00u64 => "
      RMPU.mmpuenmipi(),
    ",
  0x40000b04u64 => "
      RMPU.mmpuenptmipi(),
    ",
  0x40000b08u64 => "
      RMPU.mmpurptmipi(),
    ",
  0x40000c00u64 => "
      RMPU.mmpuacmipi(),
    ",
  0x40000c04u64 => "
      RMPU.mmpusmipi(),
    ",
  0x40000c08u64 => "
      RMPU.mmpuemipi(),
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
  0x40002004u64 => "
      SRAM.sramprcr_ns(),
    ",
  0x40002008u64 => "
      SRAM.sramwtsc(),
    ",
  0x40002010u64 => "
      SRAM.sramcr0(),
    ",
  0x40002014u64 => "
      SRAM.sramcr1(),
    ",
  0x40002030u64 => "
      SRAM.srameccrgn0(),
    ",
  0x40002040u64 => "
      SRAM.sramesr(),
    ",
  0x40002048u64 => "
      SRAM.sramesclr(),
    ",
  0x40002050u64 => "
      SRAM.sramear()[0],
    ",
  0x40002054u64 => "
      SRAM.sramear()[1],
    ",
  0x40002058u64 => "
      SRAM.sramear()[2],
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
      BUS.bussabt2ecbi(),
    ",
  0x40004258u64 => "
      BUS.bussabt2eobi(),
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
  0x40004400u64 => "
      BUS.qospri(),
    ",
  0x40004410u64 => "
      BUS.qoscyc()[0],
    ",
  0x40004418u64 => "
      BUS.qoscyc()[1],
    ",
  0x40004420u64 => "
      BUS.qoscyc()[2],
    ",
  0x40004428u64 => "
      BUS.qoscyc()[3],
    ",
  0x40004430u64 => "
      BUS.busmpri(),
    ",
  0x40004438u64 => "
      BUS.qoscyc4(),
    ",
  0x40004500u64 => "
      BUS.qosthd()[0],
    ",
  0x40004508u64 => "
      BUS.qosthd()[1],
    ",
  0x40004510u64 => "
      BUS.qosthd()[2],
    ",
  0x40004518u64 => "
      BUS.qosthd()[3],
    ",
  0x40004528u64 => "
      BUS.qosthd4(),
    ",
  0x40004600u64 => "
      BUS.qosdmon()[0],
    ",
  0x40004608u64 => "
      BUS.qosdmon()[1],
    ",
  0x40004610u64 => "
      BUS.qosdmon()[2],
    ",
  0x40004618u64 => "
      BUS.qosdmon()[3],
    ",
  0x40004628u64 => "
      BUS.qosdmon4(),
    ",
  0x40004830u64 => "
      BUS.buserradd()[0],
    ",
  0x40004840u64 => "
      BUS.buserradd()[1],
    ",
  0x40004850u64 => "
      BUS.buserradd()[2],
    ",
  0x40004860u64 => "
      BUS.buserradd()[3],
    ",
  0x40004870u64 => "
      BUS.buserradd()[4],
    ",
  0x40004880u64 => "
      BUS.buserradd()[5],
    ",
  0x40004834u64 => "
      BUS.buserrrw()[0],
    ",
  0x40004844u64 => "
      BUS.buserrrw()[1],
    ",
  0x40004854u64 => "
      BUS.buserrrw()[2],
    ",
  0x40004864u64 => "
      BUS.buserrrw()[3],
    ",
  0x40004874u64 => "
      BUS.buserrrw()[4],
    ",
  0x40004884u64 => "
      BUS.buserrrw()[5],
    ",
  0x40004930u64 => "
      BUS.btzferradd()[0],
    ",
  0x40004940u64 => "
      BUS.btzferradd()[1],
    ",
  0x40004950u64 => "
      BUS.btzferradd()[2],
    ",
  0x40004960u64 => "
      BUS.btzferradd()[3],
    ",
  0x40004970u64 => "
      BUS.btzferradd()[4],
    ",
  0x40004980u64 => "
      BUS.btzferradd()[5],
    ",
  0x40004934u64 => "
      BUS.btzferrrw()[0],
    ",
  0x40004944u64 => "
      BUS.btzferrrw()[1],
    ",
  0x40004954u64 => "
      BUS.btzferrrw()[2],
    ",
  0x40004964u64 => "
      BUS.btzferrrw()[3],
    ",
  0x40004974u64 => "
      BUS.btzferrrw()[4],
    ",
  0x40004984u64 => "
      BUS.btzferrrw()[5],
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
  0x40004a40u64 => "
      BUS.buserrstat()[4],
    ",
  0x40004a50u64 => "
      BUS.buserrstat()[5],
    ",
  0x40004a60u64 => "
      BUS.buserrstat()[6],
    ",
  0x40004a70u64 => "
      BUS.buserrstat()[7],
    ",
  0x40004a80u64 => "
      BUS.buserrstat()[8],
    ",
  0x40004a90u64 => "
      BUS.buserrstat()[9],
    ",
  0x40004aa0u64 => "
      BUS.buserrstat()[10],
    ",
  0x40004ab0u64 => "
      BUS.buserrstat()[11],
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
  0x40004a48u64 => "
      BUS.buserrclr()[4],
    ",
  0x40004a58u64 => "
      BUS.buserrclr()[5],
    ",
  0x40004a68u64 => "
      BUS.buserrclr()[6],
    ",
  0x40004a78u64 => "
      BUS.buserrclr()[7],
    ",
  0x40004a88u64 => "
      BUS.buserrclr()[8],
    ",
  0x40004a98u64 => "
      BUS.buserrclr()[9],
    ",
  0x40004aa8u64 => "
      BUS.buserrclr()[10],
    ",
  0x40004ab8u64 => "
      BUS.buserrclr()[11],
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
  0x40004010u64 => "
      TZF.tzfoad(),
    ",
  0x40004014u64 => "
      TZF.tzfpt(),
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
  0x40008420u64 => "
      CPSCU.stbramsabar(),
    ",
  0x40008490u64 => "
      CPSCU.stbrampabar_ns(),
    ",
  0x40008494u64 => "
      CPSCU.stbrampabar_s(),
    ",
  0x40008600u64 => "
      CPSCU.tevtrcr(),
    ",
  0x40008d00u64 => "
      CPSCU.busmemerrsts(),
    ",
  0x4000a000u64 => "
      DMAC_0.dmsar(),
    ",
  0x4000a004u64 => "
      DMAC_0.dmdar(),
    ",
  0x4000a008u64 => "
      DMAC_0.dmcra(),
    ",
  0x4000a00cu64 => "
      DMAC_0.dmcrb(),
    ",
  0x4000a010u64 => "
      DMAC_0.dmtmd(),
    ",
  0x4000a013u64 => "
      DMAC_0.dmint(),
    ",
  0x4000a014u64 => "
      DMAC_0.dmamd(),
    ",
  0x4000a018u64 => "
      DMAC_0.dmofr(),
    ",
  0x4000a01cu64 => "
      DMAC_0.dmcnt(),
    ",
  0x4000a01du64 => "
      DMAC_0.dmreq(),
    ",
  0x4000a01eu64 => "
      DMAC_0.dmsts(),
    ",
  0x4000a020u64 => "
      DMAC_0.dmsrr(),
    ",
  0x4000a024u64 => "
      DMAC_0.dmdrr(),
    ",
  0x4000a028u64 => "
      DMAC_0.dmsbs(),
    ",
  0x4000a02cu64 => "
      DMAC_0.dmdbs(),
    ",
  0x4000a030u64 => "
      DMAC_0.dmbwr(),
    ",
  0x4000a800u64 => "
      DMA.dmast(),
    ",
  0x4000a840u64 => "
      DMA.dmechr(),
    ",
  0x4000a880u64 => "
      DMA.delsr()[0],
    ",
  0x4000a884u64 => "
      DMA.delsr()[1],
    ",
  0x4000a888u64 => "
      DMA.delsr()[2],
    ",
  0x4000a88cu64 => "
      DMA.delsr()[3],
    ",
  0x4000a890u64 => "
      DMA.delsr()[4],
    ",
  0x4000a894u64 => "
      DMA.delsr()[5],
    ",
  0x4000a898u64 => "
      DMA.delsr()[6],
    ",
  0x4000a89cu64 => "
      DMA.delsr()[7],
    ",
  0x4000ac00u64 => "
      DTC.dtccr(),
      DTC.dtccr_ns(),
    ",
  0x4000ac04u64 => "
      DTC.dtcvbr(),
      DTC.dtcvbr_ns(),
    ",
  0x4000ac0cu64 => "
      DTC.dtcst(),
    ",
  0x4000ac0eu64 => "
      DTC.dtcsts(),
    ",
  0x4000ac10u64 => "
      DTC.dtccr_s(),
      DTC.dtccr_sec(),
    ",
  0x4000ac14u64 => "
      DTC.dtcvbr_s(),
      DTC.dtcvbr_sec(),
    ",
  0x4000ac20u64 => "
      DTC.dtevr(),
    ",
  0x4000c010u64 => "
      ICU.swirq_s(),
    ",
  0x4000c020u64 => "
      ICU.swirq_ns(),
    ",
  0x4000c060u64 => "
      ICU.ienmier(),
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
  0x4000c200u64 => "
      ICU.selsr0(),
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
  0x40011004u64 => "
      CPU_OCD.mcuctrl(),
    ",
  0x40011100u64 => "
      CPU_OCD.jbmdr(),
    ",
  0x40011120u64 => "
      CPU_OCD.jbrdr(),
    ",
  0x40011130u64 => "
      CPU_OCD.jbtdr(),
    ",
  0x40011140u64 => "
      CPU_OCD.jbstr(),
    ",
  0x40011150u64 => "
      CPU_OCD.jbicr(),
    ",
  0x40011300u64 => "
      CPU_OCD.fsblstatm(),
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
  0x4001b034u64 => "
      CPU_DBG.tracecr(),
    ",
  0x4001b040u64 => "
      CPU_DBG.cachedbgcr(),
    ",
  0x4001b100u64 => "
      CPU_DBG.alctrl(),
    ",
  0x4001b200u64 => "
      CPU_DBG.fsblstat(),
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
  0x4001e00eu64 => "
      SYSC.sscr2(),
    ",
  0x4001e010u64 => "
      SYSC.flscr(),
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
  0x4001e05au64 => "
      SYSC.adcckdivcr(),
    ",
  0x4001e05bu64 => "
      SYSC.adcckcr(),
    ",
  0x4001e05cu64 => "
      SYSC.gptckdivcr(),
    ",
  0x4001e05du64 => "
      SYSC.gptckcr(),
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
  0x4001e06fu64 => "
      SYSC.usb60ckdivcr(),
    ",
  0x4001e070u64 => "
      SYSC.i3cckdivcr(),
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
  0x4001e077u64 => "
      SYSC.usb60ckcr(),
    ",
  0x4001e078u64 => "
      SYSC.i3cckcr(),
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
  0x4001e0f0u64 => "
      SYSC.crvsyscr(),
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
  0x4001e3b8u64 => "
      SYSC.vbrpabarns(),
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
  0x4001e3feu64 => "
      SYSC.prcr_ns(),
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
  0x4001ea48u64 => "
      SYSC.rstsr3(),
    ",
  0x4001ea50u64 => "
      SYSC.momcr(),
    ",
  0x4001ea54u64 => "
      SYSC.fwepror(),
    ",
  0x4001ea58u64 => "
      SYSC.pvd1cmpcr(),
    ",
  0x4001ea5cu64 => "
      SYSC.pvd2cmpcr(),
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
  0x4001ead4u64 => "
      SYSC.syrstmsk1(),
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
  0x4001eb10u64 => "
      SYSC.momcr2(),
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
      FACI.fcntdatar0(),
    ",
  0x4011e050u64 => "
      FACI.fcntdatar1(),
    ",
  0x4011e060u64 => "
      FACI.fctrcntr(),
    ",
  0x4011e064u64 => "
      FACI.fctrlsr(),
    ",
  0x4011e068u64 => "
      FACI.fctraddr(),
    ",
  0x4011e06cu64 => "
      FACI.fctrstatr(),
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
      RTC.bcnt0(),
      RTC.rseccnt(),
    ",
  0x40202004u64 => "
      RTC.bcnt1(),
      RTC.rmincnt(),
    ",
  0x40202006u64 => "
      RTC.bcnt2(),
      RTC.rhrcnt(),
    ",
  0x40202008u64 => "
      RTC.bcnt3(),
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
      RTC.bcnt0ar(),
      RTC.rsecar(),
    ",
  0x40202012u64 => "
      RTC.bcnt1ar(),
      RTC.rminar(),
    ",
  0x40202014u64 => "
      RTC.bcnt2ar(),
      RTC.rhrar(),
    ",
  0x40202016u64 => "
      RTC.bcnt3ar(),
      RTC.rwkar(),
    ",
  0x40202018u64 => "
      RTC.bcnt0aer(),
      RTC.rdayar(),
    ",
  0x4020201au64 => "
      RTC.bcnt1aer(),
      RTC.rmonar(),
    ",
  0x4020201cu64 => "
      RTC.bcnt2aer(),
      RTC.ryrar(),
    ",
  0x4020201eu64 => "
      RTC.bcnt3aer(),
      RTC.ryraren(),
    ",
  0x40202020u64 => "
      RTC.rsr(),
    ",
  0x40202022u64 => "
      RTC.rcr1(),
    ",
  0x40202024u64 => "
      RTC.rcr2(),
    ",
  0x40202026u64 => "
      RTC.rcr3(),
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
  0x40202032u64 => "
      RTC.rtest(),
    ",
  0x40202038u64 => "
      RTC.rkey(),
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
      WDT.wdtrr(),
    ",
  0x40202602u64 => "
      WDT.wdtcr(),
    ",
  0x40202604u64 => "
      WDT.wdtsr(),
    ",
  0x40202606u64 => "
      WDT.wdtrcr(),
    ",
  0x40202608u64 => "
      WDT.wdtcstpr(),
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
      POEG.poegg()[0],
    ",
  0x40212100u64 => "
      POEG.poegg()[1],
    ",
  0x40212200u64 => "
      POEG.poegg()[2],
    ",
  0x40212300u64 => "
      POEG.poegg()[3],
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
  0x40235001u64 => "
      TSN.tstrm0(),
    ",
  0x40235002u64 => "
      TSN.tstrm1(),
    ",
  0x40235003u64 => "
      TSN.tstst(),
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
  0x40250015u64 => "
      USBFS.cfifoh(),
    ",
  0x40250018u64 => "
      USBFS.d0fifo(),
      USBFS.d0fifol(),
    ",
  0x40250019u64 => "
      USBFS.d0fifoh(),
    ",
  0x4025001cu64 => "
      USBFS.d1fifo(),
      USBFS.d1fifol(),
    ",
  0x4025001du64 => "
      USBFS.d1fifoh(),
    ",
  0x40250020u64 => "
      USBFS.cfifosel(),
    ",
  0x40250022u64 => "
      USBFS.cfifoctr(),
    ",
  0x40250028u64 => "
      USBFS.d0fifosel(),
    ",
  0x4025002au64 => "
      USBFS.d0fifoctr(),
    ",
  0x4025002cu64 => "
      USBFS.d1fifosel(),
    ",
  0x4025002eu64 => "
      USBFS.d1fifoctr(),
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
  0x402500b0u64 => "
      USBFS.bcctrl1(),
    ",
  0x402500b4u64 => "
      USBFS.bcctrl2(),
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
  0x402500f8u64 => "
      USBFS.vrcgctrl(),
    ",
  0x40250400u64 => "
      USBFS.dpusr0r(),
    ",
  0x40250404u64 => "
      USBFS.dpusr1r(),
    ",
  0x40250408u64 => "
      USBFS.dpbcctrl(),
    ",
  0x40252000u64 => "
      SDHI_0.sd_cmd(),
    ",
  0x40252008u64 => "
      SDHI_0.sd_arg(),
    ",
  0x4025200cu64 => "
      SDHI_0.sd_arg1(),
    ",
  0x40252010u64 => "
      SDHI_0.sd_stop(),
    ",
  0x40252014u64 => "
      SDHI_0.sd_seccnt(),
    ",
  0x40252018u64 => "
      SDHI_0.sd_rsp10(),
    ",
  0x4025201cu64 => "
      SDHI_0.sd_rsp1(),
    ",
  0x40252020u64 => "
      SDHI_0.sd_rsp32(),
    ",
  0x40252024u64 => "
      SDHI_0.sd_rsp3(),
    ",
  0x40252028u64 => "
      SDHI_0.sd_rsp54(),
    ",
  0x4025202cu64 => "
      SDHI_0.sd_rsp5(),
    ",
  0x40252030u64 => "
      SDHI_0.sd_rsp76(),
    ",
  0x40252034u64 => "
      SDHI_0.sd_rsp7(),
    ",
  0x40252038u64 => "
      SDHI_0.sd_info1(),
    ",
  0x4025203cu64 => "
      SDHI_0.sd_info2(),
    ",
  0x40252040u64 => "
      SDHI_0.sd_info1_mask(),
    ",
  0x40252044u64 => "
      SDHI_0.sd_info2_mask(),
    ",
  0x40252048u64 => "
      SDHI_0.sd_clk_ctrl(),
    ",
  0x4025204cu64 => "
      SDHI_0.sd_size(),
    ",
  0x40252050u64 => "
      SDHI_0.sd_option(),
    ",
  0x40252058u64 => "
      SDHI_0.sd_err_sts1(),
    ",
  0x4025205cu64 => "
      SDHI_0.sd_err_sts2(),
    ",
  0x40252060u64 => "
      SDHI_0.sd_buf0(),
    ",
  0x40252068u64 => "
      SDHI_0.sdio_mode(),
    ",
  0x4025206cu64 => "
      SDHI_0.sdio_info1(),
    ",
  0x40252070u64 => "
      SDHI_0.sdio_info1_mask(),
    ",
  0x402521b0u64 => "
      SDHI_0.sd_dmaen(),
    ",
  0x402521c0u64 => "
      SDHI_0.soft_rst(),
    ",
  0x402521ccu64 => "
      SDHI_0.sdif_mode(),
    ",
  0x402521e0u64 => "
      SDHI_0.ext_swap(),
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
  0x4025e100u64 => "
      IIC_1.iccr1(),
    ",
  0x4025e101u64 => "
      IIC_1.iccr2(),
    ",
  0x4025e102u64 => "
      IIC_1.icmr1(),
    ",
  0x4025e103u64 => "
      IIC_1.icmr2(),
    ",
  0x4025e104u64 => "
      IIC_1.icmr3(),
    ",
  0x4025e105u64 => "
      IIC_1.icfer(),
    ",
  0x4025e106u64 => "
      IIC_1.icser(),
    ",
  0x4025e107u64 => "
      IIC_1.icier(),
    ",
  0x4025e108u64 => "
      IIC_1.icsr1(),
    ",
  0x4025e109u64 => "
      IIC_1.icsr2(),
    ",
  0x4025e10au64 => "
      IIC_1.sarl()[0],
    ",
  0x4025e10cu64 => "
      IIC_1.sarl()[1],
    ",
  0x4025e10eu64 => "
      IIC_1.sarl()[2],
    ",
  0x4025e10bu64 => "
      IIC_1.saru()[0],
    ",
  0x4025e10du64 => "
      IIC_1.saru()[1],
    ",
  0x4025e10fu64 => "
      IIC_1.saru()[2],
    ",
  0x4025e110u64 => "
      IIC_1.icbrl(),
    ",
  0x4025e111u64 => "
      IIC_1.icbrh(),
    ",
  0x4025e112u64 => "
      IIC_1.icdrt(),
    ",
  0x4025e113u64 => "
      IIC_1.icdrr(),
    ",
  0x40268000u64 => "
      X_SPI.wrapcfg(),
    ",
  0x40268004u64 => "
      X_SPI.comcfg(),
    ",
  0x40268008u64 => "
      X_SPI.bmcfgch()[0],
    ",
  0x4026800cu64 => "
      X_SPI.bmcfgch()[1],
    ",
  0x40268010u64 => "
      X_SPI.cmcfg0cs()[0],
    ",
  0x40268020u64 => "
      X_SPI.cmcfg0cs()[1],
    ",
  0x40268014u64 => "
      X_SPI.cmcfg1cs()[0],
    ",
  0x40268024u64 => "
      X_SPI.cmcfg1cs()[1],
    ",
  0x40268018u64 => "
      X_SPI.cmcfg2cs()[0],
    ",
  0x40268028u64 => "
      X_SPI.cmcfg2cs()[1],
    ",
  0x40268030u64 => "
      X_SPI.aibmcfgch0()[0],
    ",
  0x40268034u64 => "
      X_SPI.aibmcfgch0()[1],
    ",
  0x40268038u64 => "
      X_SPI.aibmcfgch0()[2],
    ",
  0x4026803cu64 => "
      X_SPI.aibmcfgch0()[3],
    ",
  0x40268040u64 => "
      X_SPI.aibmcfgch1()[0],
    ",
  0x40268044u64 => "
      X_SPI.aibmcfgch1()[1],
    ",
  0x40268048u64 => "
      X_SPI.aibmcfgch1()[2],
    ",
  0x4026804cu64 => "
      X_SPI.aibmcfgch1()[3],
    ",
  0x40268050u64 => "
      X_SPI.liocfgcs()[0],
    ",
  0x40268054u64 => "
      X_SPI.liocfgcs()[1],
    ",
  0x40268058u64 => "
      X_SPI.abmcfg(),
    ",
  0x40268060u64 => "
      X_SPI.bmctl0(),
    ",
  0x40268064u64 => "
      X_SPI.bmctl1(),
    ",
  0x40268068u64 => "
      X_SPI.cmctlch()[0],
    ",
  0x4026806cu64 => "
      X_SPI.cmctlch()[1],
    ",
  0x40268070u64 => "
      X_SPI.cdctl0(),
    ",
  0x40268074u64 => "
      X_SPI.cdctl1(),
    ",
  0x40268078u64 => "
      X_SPI.cdctl2(),
    ",
  0x40268080u64 => "
      X_SPI.cdtbuf()[0],
    ",
  0x40268090u64 => "
      X_SPI.cdtbuf()[1],
    ",
  0x402680a0u64 => "
      X_SPI.cdtbuf()[2],
    ",
  0x402680b0u64 => "
      X_SPI.cdtbuf()[3],
    ",
  0x40268084u64 => "
      X_SPI.cdabuf()[0],
    ",
  0x40268094u64 => "
      X_SPI.cdabuf()[1],
    ",
  0x402680a4u64 => "
      X_SPI.cdabuf()[2],
    ",
  0x402680b4u64 => "
      X_SPI.cdabuf()[3],
    ",
  0x40268088u64 => "
      X_SPI.cdd0buf()[0],
    ",
  0x40268098u64 => "
      X_SPI.cdd0buf()[1],
    ",
  0x402680a8u64 => "
      X_SPI.cdd0buf()[2],
    ",
  0x402680b8u64 => "
      X_SPI.cdd0buf()[3],
    ",
  0x4026808cu64 => "
      X_SPI.cdd1buf()[0],
    ",
  0x4026809cu64 => "
      X_SPI.cdd1buf()[1],
    ",
  0x402680acu64 => "
      X_SPI.cdd1buf()[2],
    ",
  0x402680bcu64 => "
      X_SPI.cdd1buf()[3],
    ",
  0x40268100u64 => "
      X_SPI.lpctl0(),
    ",
  0x40268104u64 => "
      X_SPI.lpctl1(),
    ",
  0x40268108u64 => "
      X_SPI.lioctl(),
    ",
  0x40268130u64 => "
      X_SPI.ccctl0cs()[0],
    ",
  0x40268150u64 => "
      X_SPI.ccctl0cs()[1],
    ",
  0x40268134u64 => "
      X_SPI.ccctl1cs()[0],
    ",
  0x40268154u64 => "
      X_SPI.ccctl1cs()[1],
    ",
  0x40268138u64 => "
      X_SPI.ccctl2cs()[0],
    ",
  0x40268158u64 => "
      X_SPI.ccctl2cs()[1],
    ",
  0x4026813cu64 => "
      X_SPI.ccctl3cs()[0],
    ",
  0x4026815cu64 => "
      X_SPI.ccctl3cs()[1],
    ",
  0x40268140u64 => "
      X_SPI.ccctl4cs()[0],
    ",
  0x40268160u64 => "
      X_SPI.ccctl4cs()[1],
    ",
  0x40268144u64 => "
      X_SPI.ccctl5cs()[0],
    ",
  0x40268164u64 => "
      X_SPI.ccctl5cs()[1],
    ",
  0x40268148u64 => "
      X_SPI.ccctl6cs()[0],
    ",
  0x40268168u64 => "
      X_SPI.ccctl6cs()[1],
    ",
  0x4026814cu64 => "
      X_SPI.ccctl7cs()[0],
    ",
  0x4026816cu64 => "
      X_SPI.ccctl7cs()[1],
    ",
  0x40268180u64 => "
      X_SPI.verstt(),
    ",
  0x40268184u64 => "
      X_SPI.comstt(),
    ",
  0x40268188u64 => "
      X_SPI.casttcs()[0],
    ",
  0x4026818cu64 => "
      X_SPI.casttcs()[1],
    ",
  0x40268190u64 => "
      X_SPI.ints(),
    ",
  0x40268194u64 => "
      X_SPI.intc(),
    ",
  0x40268198u64 => "
      X_SPI.inte(),
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
      DOC.docr(),
    ",
  0x40311004u64 => "
      DOC.dosr(),
    ",
  0x40311008u64 => "
      DOC.doscr(),
    ",
  0x4031100cu64 => "
      DOC.dodir(),
      DOC.dodir_ha(),
    ",
  0x40311010u64 => "
      DOC.dodsr0(),
      DOC.dodsr0_ha(),
    ",
  0x40311014u64 => "
      DOC.dodsr1(),
      DOC.dodsr1_ha(),
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
  0x40323f00u64 => "
      GPT_OPS.opscr(),
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
  0x40332032u64 => "
      ADC_120.addr()[9],
    ",
  0x40332034u64 => "
      ADC_120.addr()[10],
    ",
  0x40332036u64 => "
      ADC_120.addr()[11],
    ",
  0x40332038u64 => "
      ADC_120.addr()[12],
    ",
  0x4033203au64 => "
      ADC_120.addr()[13],
    ",
  0x4033203cu64 => "
      ADC_120.addr()[14],
    ",
  0x4033203eu64 => "
      ADC_120.addr()[15],
    ",
  0x40332040u64 => "
      ADC_120.addr()[16],
    ",
  0x40332042u64 => "
      ADC_120.addr()[17],
    ",
  0x40332044u64 => "
      ADC_120.addr()[18],
    ",
  0x40332046u64 => "
      ADC_120.addr()[19],
    ",
  0x40332048u64 => "
      ADC_120.addr()[20],
    ",
  0x4033204au64 => "
      ADC_120.addr()[21],
    ",
  0x4033204cu64 => "
      ADC_120.addr()[22],
    ",
  0x4033204eu64 => "
      ADC_120.addr()[23],
    ",
  0x40332050u64 => "
      ADC_120.addr()[24],
    ",
  0x40332052u64 => "
      ADC_120.addr()[25],
    ",
  0x40332054u64 => "
      ADC_120.addr()[26],
    ",
  0x40332056u64 => "
      ADC_120.addr()[27],
    ",
  0x40332058u64 => "
      ADC_120.addr()[28],
    ",
  0x4033205au64 => "
      ADC_120.addr()[29],
    ",
  0x4033205cu64 => "
      ADC_120.addr()[30],
    ",
  0x4033205eu64 => "
      ADC_120.addr()[31],
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
  0x40332096u64 => "
      ADC_120.adcmpansr1(),
    ",
  0x40332098u64 => "
      ADC_120.adcmplr0(),
    ",
  0x4033209au64 => "
      ADC_120.adcmplr1(),
    ",
  0x4033209cu64 => "
      ADC_120.adcmpdr0(),
    ",
  0x4033209eu64 => "
      ADC_120.adcmpdr1(),
    ",
  0x403320a0u64 => "
      ADC_120.adcmpsr0(),
    ",
  0x403320a2u64 => "
      ADC_120.adcmpsr1(),
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
  0x403320e0u64 => "
      ADC_120.adsstr()[0],
    ",
  0x403320e1u64 => "
      ADC_120.adsstr()[1],
    ",
  0x403320e2u64 => "
      ADC_120.adsstr()[2],
    ",
  0x403320e3u64 => "
      ADC_120.adsstr()[3],
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
  0x403320e9u64 => "
      ADC_120.adsstr()[9],
    ",
  0x403320eau64 => "
      ADC_120.adsstr()[10],
    ",
  0x403320ebu64 => "
      ADC_120.adsstr()[11],
    ",
  0x403320ecu64 => "
      ADC_120.adsstr()[12],
    ",
  0x403320edu64 => "
      ADC_120.adsstr()[13],
    ",
  0x403320eeu64 => "
      ADC_120.adsstr()[14],
    ",
  0x403320efu64 => "
      ADC_120.adsstr()[15],
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
  0x4033222eu64 => "
      ADC_121.addr()[7],
    ",
  0x40332230u64 => "
      ADC_121.addr()[8],
    ",
  0x40332232u64 => "
      ADC_121.addr()[9],
    ",
  0x40332234u64 => "
      ADC_121.addr()[10],
    ",
  0x40332236u64 => "
      ADC_121.addr()[11],
    ",
  0x40332238u64 => "
      ADC_121.addr()[12],
    ",
  0x4033223au64 => "
      ADC_121.addr()[13],
    ",
  0x4033223cu64 => "
      ADC_121.addr()[14],
    ",
  0x4033223eu64 => "
      ADC_121.addr()[15],
    ",
  0x40332240u64 => "
      ADC_121.addr()[16],
    ",
  0x40332242u64 => "
      ADC_121.addr()[17],
    ",
  0x40332244u64 => "
      ADC_121.addr()[18],
    ",
  0x40332246u64 => "
      ADC_121.addr()[19],
    ",
  0x40332248u64 => "
      ADC_121.addr()[20],
    ",
  0x4033224au64 => "
      ADC_121.addr()[21],
    ",
  0x4033224cu64 => "
      ADC_121.addr()[22],
    ",
  0x4033224eu64 => "
      ADC_121.addr()[23],
    ",
  0x40332250u64 => "
      ADC_121.addr()[24],
    ",
  0x40332252u64 => "
      ADC_121.addr()[25],
    ",
  0x40332254u64 => "
      ADC_121.addr()[26],
    ",
  0x40332256u64 => "
      ADC_121.addr()[27],
    ",
  0x40332258u64 => "
      ADC_121.addr()[28],
    ",
  0x4033225au64 => "
      ADC_121.addr()[29],
    ",
  0x4033225cu64 => "
      ADC_121.addr()[30],
    ",
  0x4033225eu64 => "
      ADC_121.addr()[31],
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
  0x40332296u64 => "
      ADC_121.adcmpansr1(),
    ",
  0x40332298u64 => "
      ADC_121.adcmplr0(),
    ",
  0x4033229au64 => "
      ADC_121.adcmplr1(),
    ",
  0x4033229cu64 => "
      ADC_121.adcmpdr0(),
    ",
  0x4033229eu64 => "
      ADC_121.adcmpdr1(),
    ",
  0x403322a0u64 => "
      ADC_121.adcmpsr0(),
    ",
  0x403322a2u64 => "
      ADC_121.adcmpsr1(),
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
  0x403322e7u64 => "
      ADC_121.adsstr()[7],
    ",
  0x403322e8u64 => "
      ADC_121.adsstr()[8],
    ",
  0x403322e9u64 => "
      ADC_121.adsstr()[9],
    ",
  0x403322eau64 => "
      ADC_121.adsstr()[10],
    ",
  0x403322ebu64 => "
      ADC_121.adsstr()[11],
    ",
  0x403322ecu64 => "
      ADC_121.adsstr()[12],
    ",
  0x403322edu64 => "
      ADC_121.adsstr()[13],
    ",
  0x403322eeu64 => "
      ADC_121.adsstr()[14],
    ",
  0x403322efu64 => "
      ADC_121.adsstr()[15],
    ",
  0x40333000u64 => "
      DAC_12.dadr()[0],
    ",
  0x40333002u64 => "
      DAC_12.dadr()[1],
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
      GLCDC.gam_latch()[0],
    ",
  0x40343340u64 => "
      GLCDC.gam_latch()[1],
    ",
  0x40343380u64 => "
      GLCDC.gam_latch()[2],
    ",
  0x40343304u64 => "
      GLCDC.gam_sw()[0],
      GLCDC.gam_sw(),
    ",
  0x40343344u64 => "
      GLCDC.gam_sw()[1],
    ",
  0x40343384u64 => "
      GLCDC.gam_sw()[2],
    ",
  0x40343308u64 => "
      GLCDC.gam_lut1()[0],
    ",
  0x40343348u64 => "
      GLCDC.gam_lut1()[1],
    ",
  0x40343388u64 => "
      GLCDC.gam_lut1()[2],
    ",
  0x4034330cu64 => "
      GLCDC.gam_lut2()[0],
    ",
  0x4034334cu64 => "
      GLCDC.gam_lut2()[1],
    ",
  0x4034338cu64 => "
      GLCDC.gam_lut2()[2],
    ",
  0x40343310u64 => "
      GLCDC.gam_lut3()[0],
    ",
  0x40343350u64 => "
      GLCDC.gam_lut3()[1],
    ",
  0x40343390u64 => "
      GLCDC.gam_lut3()[2],
    ",
  0x40343314u64 => "
      GLCDC.gam_lut4()[0],
    ",
  0x40343354u64 => "
      GLCDC.gam_lut4()[1],
    ",
  0x40343394u64 => "
      GLCDC.gam_lut4()[2],
    ",
  0x40343318u64 => "
      GLCDC.gam_lut5()[0],
    ",
  0x40343358u64 => "
      GLCDC.gam_lut5()[1],
    ",
  0x40343398u64 => "
      GLCDC.gam_lut5()[2],
    ",
  0x4034331cu64 => "
      GLCDC.gam_lut6()[0],
    ",
  0x4034335cu64 => "
      GLCDC.gam_lut6()[1],
    ",
  0x4034339cu64 => "
      GLCDC.gam_lut6()[2],
    ",
  0x40343320u64 => "
      GLCDC.gam_lut7()[0],
    ",
  0x40343360u64 => "
      GLCDC.gam_lut7()[1],
    ",
  0x403433a0u64 => "
      GLCDC.gam_lut7()[2],
    ",
  0x40343324u64 => "
      GLCDC.gam_lut8()[0],
    ",
  0x40343364u64 => "
      GLCDC.gam_lut8()[1],
    ",
  0x403433a4u64 => "
      GLCDC.gam_lut8()[2],
    ",
  0x40343328u64 => "
      GLCDC.gam_area1()[0],
    ",
  0x40343368u64 => "
      GLCDC.gam_area1()[1],
    ",
  0x403433a8u64 => "
      GLCDC.gam_area1()[2],
    ",
  0x4034332cu64 => "
      GLCDC.gam_area2()[0],
    ",
  0x4034336cu64 => "
      GLCDC.gam_area2()[1],
    ",
  0x403433acu64 => "
      GLCDC.gam_area2()[2],
    ",
  0x40343330u64 => "
      GLCDC.gam_area3()[0],
    ",
  0x40343370u64 => "
      GLCDC.gam_area3()[1],
    ",
  0x403433b0u64 => "
      GLCDC.gam_area3()[2],
    ",
  0x40343334u64 => "
      GLCDC.gam_area4()[0],
    ",
  0x40343374u64 => "
      GLCDC.gam_area4()[1],
    ",
  0x403433b4u64 => "
      GLCDC.gam_area4()[2],
    ",
  0x40343338u64 => "
      GLCDC.gam_area5()[0],
    ",
  0x40343378u64 => "
      GLCDC.gam_area5()[1],
    ",
  0x403433b8u64 => "
      GLCDC.gam_area5()[2],
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
  0x40343400u64 => "
      GLCDC.tcon_vlatch(),
    ",
  0x40343404u64 => "
      GLCDC.tcon_tim(),
    ",
  0x40343408u64 => "
      GLCDC.tcon_stv1()[0],
    ",
  0x40343410u64 => "
      GLCDC.tcon_stv1()[1],
    ",
  0x4034340cu64 => "
      GLCDC.tcon_stv2()[0],
    ",
  0x40343414u64 => "
      GLCDC.tcon_stv2()[1],
    ",
  0x40343418u64 => "
      GLCDC.tcon_sth1()[0],
    ",
  0x40343420u64 => "
      GLCDC.tcon_sth1()[1],
    ",
  0x4034341cu64 => "
      GLCDC.tcon_sth2()[0],
    ",
  0x40343424u64 => "
      GLCDC.tcon_sth2()[1],
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
      DRW.perfcount()[0],
    ",
  0x403440d0u64 => "
      DRW.perfcount()[1],
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
  0x40346000u64 => "
      DSILINK.isr(),
      DSILINK.isr_l(),
      DSILINK.isr_ll(),
    ",
  0x40346001u64 => "
      DSILINK.isr_lh(),
    ",
  0x40346002u64 => "
      DSILINK.isr_h(),
      DSILINK.isr_hl(),
    ",
  0x40346003u64 => "
      DSILINK.isr_hh(),
    ",
  0x40346010u64 => "
      DSILINK.linksr_l(),
      DSILINK.linksr_ll(),
    ",
  0x40346011u64 => "
      DSILINK.linksr_lh(),
    ",
  0x40346012u64 => "
      DSILINK.linksr_h(),
      DSILINK.linksr_hl(),
    ",
  0x40346013u64 => "
      DSILINK.linksr_hh(),
    ",
  0x40346100u64 => "
      DSILINK.txsetr_l(),
      DSILINK.txsetr_ll(),
    ",
  0x40346101u64 => "
      DSILINK.txsetr_lh(),
    ",
  0x40346102u64 => "
      DSILINK.txsetr_h(),
      DSILINK.txsetr_hl(),
    ",
  0x40346104u64 => "
      DSILINK.hsclksetr(),
      DSILINK.hsclksetr_l(),
      DSILINK.hsclksetr_ll(),
    ",
  0x40346108u64 => "
      DSILINK.ulpssetr(),
      DSILINK.ulpssetr_l(),
      DSILINK.ulpssetr_ll(),
    ",
  0x4034610cu64 => "
      DSILINK.ulpscr(),
    ",
  0x4034610eu64 => "
      DSILINK.ulpscr_h(),
    ",
  0x4034610fu64 => "
      DSILINK.ulpscr_hh(),
    ",
  0x40346110u64 => "
      DSILINK.rstcr(),
      DSILINK.rstcr_l(),
      DSILINK.rstcr_ll(),
    ",
  0x40346112u64 => "
      DSILINK.rstcr_h(),
      DSILINK.rstcr_hl(),
    ",
  0x40346113u64 => "
      DSILINK.rstcr_hh(),
    ",
  0x40346114u64 => "
      DSILINK.rstsr(),
      DSILINK.rstsr_l(),
      DSILINK.rstsr_ll(),
    ",
  0x40346115u64 => "
      DSILINK.rstsr_lh(),
    ",
  0x40346120u64 => "
      DSILINK.dsisetr(),
      DSILINK.dsisetr_l(),
      DSILINK.dsisetr_ll(),
    ",
  0x40346121u64 => "
      DSILINK.dsisetr_lh(),
    ",
  0x40346122u64 => "
      DSILINK.dsisetr_h(),
      DSILINK.dsisetr_hl(),
    ",
  0x40346123u64 => "
      DSILINK.dsisetr_hh(),
    ",
  0x40346160u64 => "
      DSILINK.txppd0r(),
      DSILINK.txppd0r_l(),
      DSILINK.txppd0r_ll(),
    ",
  0x40346161u64 => "
      DSILINK.txppd0r_lh(),
    ",
  0x40346162u64 => "
      DSILINK.txppd0r_h(),
      DSILINK.txppd0r_hl(),
    ",
  0x40346163u64 => "
      DSILINK.txppd0r_hh(),
    ",
  0x40346164u64 => "
      DSILINK.txppd1r(),
      DSILINK.txppd1r_l(),
      DSILINK.txppd1r_ll(),
    ",
  0x40346165u64 => "
      DSILINK.txppd1r_lh(),
    ",
  0x40346166u64 => "
      DSILINK.txppd1r_h(),
      DSILINK.txppd1r_hl(),
    ",
  0x40346167u64 => "
      DSILINK.txppd1r_hh(),
    ",
  0x40346168u64 => "
      DSILINK.txppd2r(),
      DSILINK.txppd2r_l(),
      DSILINK.txppd2r_ll(),
    ",
  0x40346169u64 => "
      DSILINK.txppd2r_lh(),
    ",
  0x4034616au64 => "
      DSILINK.txppd2r_h(),
      DSILINK.txppd2r_hl(),
    ",
  0x4034616bu64 => "
      DSILINK.txppd2r_hh(),
    ",
  0x4034616cu64 => "
      DSILINK.txppd3r(),
      DSILINK.txppd3r_l(),
      DSILINK.txppd3r_ll(),
    ",
  0x4034616du64 => "
      DSILINK.txppd3r_lh(),
    ",
  0x4034616eu64 => "
      DSILINK.txppd3r_h(),
      DSILINK.txppd3r_hl(),
    ",
  0x4034616fu64 => "
      DSILINK.txppd3r_hh(),
    ",
  0x40346200u64 => "
      DSILINK.rxsr(),
      DSILINK.rxsr_l(),
      DSILINK.rxsr_ll(),
    ",
  0x40346201u64 => "
      DSILINK.rxsr_lh(),
    ",
  0x40346202u64 => "
      DSILINK.rxsr_h(),
      DSILINK.rxsr_hl(),
    ",
  0x40346203u64 => "
      DSILINK.rxsr_hh(),
    ",
  0x40346204u64 => "
      DSILINK.rxscr(),
      DSILINK.rxscr_l(),
      DSILINK.rxscr_ll(),
    ",
  0x40346205u64 => "
      DSILINK.rxscr_lh(),
    ",
  0x40346206u64 => "
      DSILINK.rxscr_h(),
      DSILINK.rxscr_hl(),
    ",
  0x40346207u64 => "
      DSILINK.rxscr_hh(),
    ",
  0x40346208u64 => "
      DSILINK.rxier(),
      DSILINK.rxier_l(),
      DSILINK.rxier_ll(),
    ",
  0x40346209u64 => "
      DSILINK.rxier_lh(),
    ",
  0x4034620au64 => "
      DSILINK.rxier_h(),
      DSILINK.rxier_hl(),
    ",
  0x4034620bu64 => "
      DSILINK.rxier_hh(),
    ",
  0x40346210u64 => "
      DSILINK.presptobtasetr(),
    ",
  0x40346214u64 => "
      DSILINK.presptolpsetr(),
      DSILINK.presptolpsetr_l(),
    ",
  0x40346216u64 => "
      DSILINK.presptolpsetr_h(),
    ",
  0x40346218u64 => "
      DSILINK.presptohssetr(),
      DSILINK.presptohssetr_l(),
    ",
  0x4034621au64 => "
      DSILINK.presptohssetr_h(),
    ",
  0x40346220u64 => "
      DSILINK.akeplatir(),
      DSILINK.akeplatir_l(),
      DSILINK.akeplatir_ll(),
    ",
  0x40346221u64 => "
      DSILINK.akeplatir_lh(),
    ",
  0x40346222u64 => "
      DSILINK.akeplatir_h(),
      DSILINK.akeplatir_hl(),
    ",
  0x40346224u64 => "
      DSILINK.akepacmsr(),
      DSILINK.akepacmsr_l(),
      DSILINK.akepacmsr_ll(),
    ",
  0x40346225u64 => "
      DSILINK.akepacmsr_lh(),
    ",
  0x40346226u64 => "
      DSILINK.akepacmsr_h(),
      DSILINK.akepacmsr_hl(),
    ",
  0x40346228u64 => "
      DSILINK.akepscr(),
      DSILINK.akepscr_l(),
      DSILINK.akepscr_ll(),
    ",
  0x40346229u64 => "
      DSILINK.akepscr_lh(),
    ",
  0x4034622au64 => "
      DSILINK.akepscr_h(),
      DSILINK.akepscr_hl(),
    ",
  0x40346230u64 => "
      DSILINK.rxrssr(),
      DSILINK.rxrssr_l(),
      DSILINK.rxrssr_ll(),
    ",
  0x40346232u64 => "
      DSILINK.rxrssr_h(),
    ",
  0x40346233u64 => "
      DSILINK.rxrssr_hh(),
    ",
  0x40346234u64 => "
      DSILINK.rxrsscr(),
      DSILINK.rxrsscr_l(),
      DSILINK.rxrsscr_ll(),
    ",
  0x40346236u64 => "
      DSILINK.rxrsscr_h(),
    ",
  0x40346237u64 => "
      DSILINK.rxrsscr_hh(),
    ",
  0x40346238u64 => "
      DSILINK.rxrinfoowsr(),
      DSILINK.rxrinfoowsr_l(),
      DSILINK.rxrinfoowsr_ll(),
    ",
  0x4034623au64 => "
      DSILINK.rxrinfoowsr_h(),
    ",
  0x4034623bu64 => "
      DSILINK.rxrinfoowsr_hh(),
    ",
  0x4034623cu64 => "
      DSILINK.rxrinfoowscr(),
      DSILINK.rxrinfoowscr_l(),
      DSILINK.rxrinfoowscr_ll(),
    ",
  0x4034623eu64 => "
      DSILINK.rxrinfoowscr_h(),
    ",
  0x4034623fu64 => "
      DSILINK.rxrinfoowscr_hh(),
    ",
  0x40346240u64 => "
      DSILINK.rxrssr()[0],
      DSILINK.rxrssr_l()[0],
      DSILINK.rxrssr_ll()[0],
    ",
  0x40346244u64 => "
      DSILINK.rxrssr()[1],
      DSILINK.rxrssr_l()[1],
      DSILINK.rxrssr_ll()[1],
    ",
  0x40346248u64 => "
      DSILINK.rxrssr()[2],
      DSILINK.rxrssr_l()[2],
      DSILINK.rxrssr_ll()[2],
    ",
  0x4034624cu64 => "
      DSILINK.rxrssr()[3],
      DSILINK.rxrssr_l()[3],
      DSILINK.rxrssr_ll()[3],
    ",
  0x40346241u64 => "
      DSILINK.rxrssr_lh()[0],
    ",
  0x40346245u64 => "
      DSILINK.rxrssr_lh()[1],
    ",
  0x40346249u64 => "
      DSILINK.rxrssr_lh()[2],
    ",
  0x4034624du64 => "
      DSILINK.rxrssr_lh()[3],
    ",
  0x40346242u64 => "
      DSILINK.rxrssr_h()[0],
      DSILINK.rxrssr_hl()[0],
    ",
  0x40346246u64 => "
      DSILINK.rxrssr_h()[1],
      DSILINK.rxrssr_hl()[1],
    ",
  0x4034624au64 => "
      DSILINK.rxrssr_h()[2],
      DSILINK.rxrssr_hl()[2],
    ",
  0x4034624eu64 => "
      DSILINK.rxrssr_h()[3],
      DSILINK.rxrssr_hl()[3],
    ",
  0x40346243u64 => "
      DSILINK.rxrssr_hh()[0],
    ",
  0x40346247u64 => "
      DSILINK.rxrssr_hh()[1],
    ",
  0x4034624bu64 => "
      DSILINK.rxrssr_hh()[2],
    ",
  0x4034624fu64 => "
      DSILINK.rxrssr_hh()[3],
    ",
  0x403462c0u64 => "
      DSILINK.rxppd0r(),
      DSILINK.rxppd0r_l(),
      DSILINK.rxppd0r_ll(),
    ",
  0x403462c1u64 => "
      DSILINK.rxppd0r_lh(),
    ",
  0x403462c2u64 => "
      DSILINK.rxppd0r_h(),
      DSILINK.rxppd0r_hl(),
    ",
  0x403462c3u64 => "
      DSILINK.rxppd0r_hh(),
    ",
  0x403462c4u64 => "
      DSILINK.rxppd1r(),
      DSILINK.rxppd1r_l(),
      DSILINK.rxppd1r_ll(),
    ",
  0x403462c5u64 => "
      DSILINK.rxppd1r_lh(),
    ",
  0x403462c6u64 => "
      DSILINK.rxppd1r_h(),
      DSILINK.rxppd1r_hl(),
    ",
  0x403462c7u64 => "
      DSILINK.rxppd1r_hh(),
    ",
  0x403462c8u64 => "
      DSILINK.rxppd2r(),
      DSILINK.rxppd2r_l(),
      DSILINK.rxppd2r_ll(),
    ",
  0x403462c9u64 => "
      DSILINK.rxppd2r_lh(),
    ",
  0x403462cau64 => "
      DSILINK.rxppd2r_h(),
      DSILINK.rxppd2r_hl(),
    ",
  0x403462cbu64 => "
      DSILINK.rxppd2r_hh(),
    ",
  0x403462ccu64 => "
      DSILINK.rxppd3r(),
      DSILINK.rxppd3r_l(),
      DSILINK.rxppd3r_ll(),
    ",
  0x403462cdu64 => "
      DSILINK.rxppd3r_lh(),
    ",
  0x403462ceu64 => "
      DSILINK.rxppd3r_h(),
      DSILINK.rxppd3r_hl(),
    ",
  0x403462cfu64 => "
      DSILINK.rxppd3r_hh(),
    ",
  0x403462e0u64 => "
      DSILINK.hstxtosetr(),
    ",
  0x403462e4u64 => "
      DSILINK.lrxhtosetr(),
    ",
  0x403462e8u64 => "
      DSILINK.tatosetr(),
    ",
  0x40346300u64 => "
      DSILINK.ferrsr(),
      DSILINK.ferrsr_l(),
      DSILINK.ferrsr_ll(),
    ",
  0x40346302u64 => "
      DSILINK.ferrsr_h(),
      DSILINK.ferrsr_hl(),
    ",
  0x40346303u64 => "
      DSILINK.ferrsr_hh(),
    ",
  0x40346304u64 => "
      DSILINK.ferrscr(),
      DSILINK.ferrscr_l(),
      DSILINK.ferrscr_ll(),
    ",
  0x40346306u64 => "
      DSILINK.ferrscr_h(),
      DSILINK.ferrscr_hl(),
    ",
  0x40346308u64 => "
      DSILINK.ferrier(),
      DSILINK.ferrier_l(),
      DSILINK.ferrier_ll(),
    ",
  0x4034630au64 => "
      DSILINK.ferrier_h(),
      DSILINK.ferrier_hl(),
    ",
  0x40346314u64 => "
      DSILINK.clstptsetr(),
      DSILINK.clstptsetr_l(),
      DSILINK.clstptsetr_ll(),
    ",
  0x40346315u64 => "
      DSILINK.clstptsetr_lh(),
    ",
  0x40346316u64 => "
      DSILINK.clstptsetr_h(),
      DSILINK.clstptsetr_hl(),
    ",
  0x40346317u64 => "
      DSILINK.clstptsetr_hh(),
    ",
  0x40346318u64 => "
      DSILINK.lptrnstsetr(),
      DSILINK.lptrnstsetr_l(),
      DSILINK.lptrnstsetr_ll(),
    ",
  0x40346319u64 => "
      DSILINK.lptrnstsetr_lh(),
    ",
  0x40346320u64 => "
      DSILINK.plsr(),
      DSILINK.plsr_l(),
      DSILINK.plsr_ll(),
    ",
  0x40346321u64 => "
      DSILINK.plsr_lh(),
    ",
  0x40346322u64 => "
      DSILINK.plsr_h(),
    ",
  0x40346323u64 => "
      DSILINK.plsr_hh(),
    ",
  0x40346324u64 => "
      DSILINK.plscr(),
      DSILINK.plscr_l(),
    ",
  0x40346325u64 => "
      DSILINK.plscr_lh(),
    ",
  0x40346326u64 => "
      DSILINK.plscr_h(),
    ",
  0x40346327u64 => "
      DSILINK.plscr_hh(),
    ",
  0x40346328u64 => "
      DSILINK.plier(),
      DSILINK.plier_l(),
    ",
  0x40346329u64 => "
      DSILINK.plier_lh(),
    ",
  0x4034632au64 => "
      DSILINK.plier_h(),
    ",
  0x4034632bu64 => "
      DSILINK.plier_hh(),
    ",
  0x40346400u64 => "
      DSILINK.vmset0r(),
      DSILINK.vmset0r_l(),
      DSILINK.vmset0r_ll(),
    ",
  0x40346401u64 => "
      DSILINK.vmset0r_lh(),
    ",
  0x40346404u64 => "
      DSILINK.vmset1r_l(),
    ",
  0x40346406u64 => "
      DSILINK.vmset1r_h(),
    ",
  0x40346410u64 => "
      DSILINK.vmsr(),
      DSILINK.vmsr_l(),
      DSILINK.vmsr_ll(),
    ",
  0x40346411u64 => "
      DSILINK.vmsr_lh(),
    ",
  0x40346412u64 => "
      DSILINK.vmsr_h(),
      DSILINK.vmsr_hl(),
    ",
  0x40346413u64 => "
      DSILINK.vmsr_hh(),
    ",
  0x40346414u64 => "
      DSILINK.vmscr(),
      DSILINK.vmscr_l(),
      DSILINK.vmscr_ll(),
    ",
  0x40346415u64 => "
      DSILINK.vmscr_lh(),
    ",
  0x40346416u64 => "
      DSILINK.vmscr_h(),
      DSILINK.vmscr_hl(),
    ",
  0x40346417u64 => "
      DSILINK.vmscr_hh(),
    ",
  0x40346418u64 => "
      DSILINK.vmier(),
      DSILINK.vmier_l(),
      DSILINK.vmier_ll(),
    ",
  0x40346419u64 => "
      DSILINK.vmier_lh(),
    ",
  0x4034641au64 => "
      DSILINK.vmier_h(),
      DSILINK.vmier_hl(),
    ",
  0x4034641bu64 => "
      DSILINK.vmier_hh(),
    ",
  0x40346420u64 => "
      DSILINK.vmppsetr(),
      DSILINK.vmppsetr_l(),
      DSILINK.vmppsetr_ll(),
    ",
  0x40346421u64 => "
      DSILINK.vmppsetr_lh(),
    ",
  0x40346422u64 => "
      DSILINK.vmppsetr_h(),
      DSILINK.vmppsetr_hl(),
    ",
  0x40346428u64 => "
      DSILINK.vmvssetr(),
      DSILINK.vmvssetr_l(),
      DSILINK.vmvssetr_ll(),
    ",
  0x40346429u64 => "
      DSILINK.vmvssetr_lh(),
    ",
  0x4034642au64 => "
      DSILINK.vmvssetr_h(),
      DSILINK.vmvssetr_hl(),
    ",
  0x4034642bu64 => "
      DSILINK.vmvssetr_hh(),
    ",
  0x4034642cu64 => "
      DSILINK.vmvpsetr(),
      DSILINK.vmvpsetr_l(),
      DSILINK.vmvpsetr_ll(),
    ",
  0x4034642du64 => "
      DSILINK.vmvpsetr_lh(),
    ",
  0x4034642eu64 => "
      DSILINK.vmvpsetr_h(),
      DSILINK.vmvpsetr_hl(),
    ",
  0x4034642fu64 => "
      DSILINK.vmvpsetr_hh(),
    ",
  0x40346430u64 => "
      DSILINK.vmhssetr(),
      DSILINK.vmhssetr_l(),
      DSILINK.vmhssetr_ll(),
    ",
  0x40346431u64 => "
      DSILINK.vmhssetr_lh(),
    ",
  0x40346432u64 => "
      DSILINK.vmhssetr_h(),
      DSILINK.vmhssetr_hl(),
    ",
  0x40346433u64 => "
      DSILINK.vmhssetr_hh(),
    ",
  0x40346434u64 => "
      DSILINK.vmhpsetr(),
      DSILINK.vmhpsetr_l(),
      DSILINK.vmhpsetr_ll(),
    ",
  0x40346435u64 => "
      DSILINK.vmhpsetr_lh(),
    ",
  0x40346436u64 => "
      DSILINK.vmhpsetr_h(),
      DSILINK.vmhpsetr_hl(),
    ",
  0x40346437u64 => "
      DSILINK.vmhpsetr_hh(),
    ",
  0x403465c0u64 => "
      DSILINK.sqch0set0r(),
      DSILINK.sqch0set0r_l(),
      DSILINK.sqch0set0r_ll(),
    ",
  0x403465c1u64 => "
      DSILINK.sqch0set0r_lh(),
    ",
  0x403465c2u64 => "
      DSILINK.sqch0set0r_h(),
      DSILINK.sqch0set0r_hl(),
    ",
  0x403465c3u64 => "
      DSILINK.sqch0set0r_hh(),
    ",
  0x403465d0u64 => "
      DSILINK.sqch0sr(),
      DSILINK.sqch0sr_l(),
      DSILINK.sqch0sr_ll(),
    ",
  0x403465d1u64 => "
      DSILINK.sqch0sr_lh(),
    ",
  0x403465d2u64 => "
      DSILINK.sqch0sr_h(),
      DSILINK.sqch0sr_hl(),
    ",
  0x403465d3u64 => "
      DSILINK.sqch0sr_hh(),
    ",
  0x403465d4u64 => "
      DSILINK.sqch0scr(),
      DSILINK.sqch0scr_l(),
      DSILINK.sqch0scr_ll(),
    ",
  0x403465d5u64 => "
      DSILINK.sqch0scr_lh(),
    ",
  0x403465d6u64 => "
      DSILINK.sqch0scr_h(),
      DSILINK.sqch0scr_hl(),
    ",
  0x403465d7u64 => "
      DSILINK.sqch0scr_hh(),
    ",
  0x403465d8u64 => "
      DSILINK.sqch0ier(),
      DSILINK.sqch0ier_l(),
      DSILINK.sqch0ier_ll(),
    ",
  0x403465d9u64 => "
      DSILINK.sqch0ier_lh(),
    ",
  0x403465dau64 => "
      DSILINK.sqch0ier_h(),
      DSILINK.sqch0ier_hl(),
    ",
  0x403465dbu64 => "
      DSILINK.sqch0ier_hh(),
    ",
  0x40346600u64 => "
      DSILINK.sqch1set0r(),
      DSILINK.sqch1set0r_l(),
      DSILINK.sqch1set0r_ll(),
    ",
  0x40346601u64 => "
      DSILINK.sqch1set0r_lh(),
    ",
  0x40346602u64 => "
      DSILINK.sqch1set0r_h(),
      DSILINK.sqch1set0r_hl(),
    ",
  0x40346603u64 => "
      DSILINK.sqch1set0r_hh(),
    ",
  0x40346610u64 => "
      DSILINK.sqch1sr(),
      DSILINK.sqch1sr_l(),
      DSILINK.sqch1sr_ll(),
    ",
  0x40346611u64 => "
      DSILINK.sqch1sr_lh(),
    ",
  0x40346612u64 => "
      DSILINK.sqch1sr_h(),
      DSILINK.sqch1sr_hl(),
    ",
  0x40346613u64 => "
      DSILINK.sqch1sr_hh(),
    ",
  0x40346614u64 => "
      DSILINK.sqch1scr(),
      DSILINK.sqch1scr_l(),
      DSILINK.sqch1scr_ll(),
    ",
  0x40346615u64 => "
      DSILINK.sqch1scr_lh(),
    ",
  0x40346616u64 => "
      DSILINK.sqch1scr_h(),
      DSILINK.sqch1scr_hl(),
    ",
  0x40346617u64 => "
      DSILINK.sqch1scr_hh(),
    ",
  0x40346618u64 => "
      DSILINK.sqch1ier(),
      DSILINK.sqch1ier_l(),
      DSILINK.sqch1ier_ll(),
    ",
  0x40346619u64 => "
      DSILINK.sqch1ier_lh(),
    ",
  0x4034661au64 => "
      DSILINK.sqch1ier_h(),
      DSILINK.sqch1ier_hl(),
    ",
  0x4034661bu64 => "
      DSILINK.sqch1ier_hh(),
    ",
  0x40346780u64 => "
      DSILINK.sqch0dscar()[0],
      DSILINK.sqch0dscar_l()[0],
      DSILINK.sqch0dscar_ll()[0],
    ",
  0x40346790u64 => "
      DSILINK.sqch0dscar()[1],
      DSILINK.sqch0dscar_l()[1],
      DSILINK.sqch0dscar_ll()[1],
    ",
  0x403467a0u64 => "
      DSILINK.sqch0dscar()[2],
      DSILINK.sqch0dscar_l()[2],
      DSILINK.sqch0dscar_ll()[2],
    ",
  0x403467b0u64 => "
      DSILINK.sqch0dscar()[3],
      DSILINK.sqch0dscar_l()[3],
      DSILINK.sqch0dscar_ll()[3],
    ",
  0x403467c0u64 => "
      DSILINK.sqch0dscar()[4],
      DSILINK.sqch0dscar_l()[4],
      DSILINK.sqch0dscar_ll()[4],
    ",
  0x403467d0u64 => "
      DSILINK.sqch0dscar()[5],
      DSILINK.sqch0dscar_l()[5],
      DSILINK.sqch0dscar_ll()[5],
    ",
  0x403467e0u64 => "
      DSILINK.sqch0dscar()[6],
      DSILINK.sqch0dscar_l()[6],
      DSILINK.sqch0dscar_ll()[6],
    ",
  0x403467f0u64 => "
      DSILINK.sqch0dscar()[7],
      DSILINK.sqch0dscar_l()[7],
      DSILINK.sqch0dscar_ll()[7],
    ",
  0x40346781u64 => "
      DSILINK.sqch0dscar_lh()[0],
    ",
  0x40346791u64 => "
      DSILINK.sqch0dscar_lh()[1],
    ",
  0x403467a1u64 => "
      DSILINK.sqch0dscar_lh()[2],
    ",
  0x403467b1u64 => "
      DSILINK.sqch0dscar_lh()[3],
    ",
  0x403467c1u64 => "
      DSILINK.sqch0dscar_lh()[4],
    ",
  0x403467d1u64 => "
      DSILINK.sqch0dscar_lh()[5],
    ",
  0x403467e1u64 => "
      DSILINK.sqch0dscar_lh()[6],
    ",
  0x403467f1u64 => "
      DSILINK.sqch0dscar_lh()[7],
    ",
  0x40346782u64 => "
      DSILINK.sqch0dscar_h()[0],
      DSILINK.sqch0dscar_hl()[0],
    ",
  0x40346792u64 => "
      DSILINK.sqch0dscar_h()[1],
      DSILINK.sqch0dscar_hl()[1],
    ",
  0x403467a2u64 => "
      DSILINK.sqch0dscar_h()[2],
      DSILINK.sqch0dscar_hl()[2],
    ",
  0x403467b2u64 => "
      DSILINK.sqch0dscar_h()[3],
      DSILINK.sqch0dscar_hl()[3],
    ",
  0x403467c2u64 => "
      DSILINK.sqch0dscar_h()[4],
      DSILINK.sqch0dscar_hl()[4],
    ",
  0x403467d2u64 => "
      DSILINK.sqch0dscar_h()[5],
      DSILINK.sqch0dscar_hl()[5],
    ",
  0x403467e2u64 => "
      DSILINK.sqch0dscar_h()[6],
      DSILINK.sqch0dscar_hl()[6],
    ",
  0x403467f2u64 => "
      DSILINK.sqch0dscar_h()[7],
      DSILINK.sqch0dscar_hl()[7],
    ",
  0x40346783u64 => "
      DSILINK.sqch0dscar_hh()[0],
    ",
  0x40346793u64 => "
      DSILINK.sqch0dscar_hh()[1],
    ",
  0x403467a3u64 => "
      DSILINK.sqch0dscar_hh()[2],
    ",
  0x403467b3u64 => "
      DSILINK.sqch0dscar_hh()[3],
    ",
  0x403467c3u64 => "
      DSILINK.sqch0dscar_hh()[4],
    ",
  0x403467d3u64 => "
      DSILINK.sqch0dscar_hh()[5],
    ",
  0x403467e3u64 => "
      DSILINK.sqch0dscar_hh()[6],
    ",
  0x403467f3u64 => "
      DSILINK.sqch0dscar_hh()[7],
    ",
  0x40346784u64 => "
      DSILINK.sqch0dscbr()[0],
    ",
  0x40346794u64 => "
      DSILINK.sqch0dscbr()[1],
    ",
  0x403467a4u64 => "
      DSILINK.sqch0dscbr()[2],
    ",
  0x403467b4u64 => "
      DSILINK.sqch0dscbr()[3],
    ",
  0x403467c4u64 => "
      DSILINK.sqch0dscbr()[4],
    ",
  0x403467d4u64 => "
      DSILINK.sqch0dscbr()[5],
    ",
  0x403467e4u64 => "
      DSILINK.sqch0dscbr()[6],
    ",
  0x403467f4u64 => "
      DSILINK.sqch0dscbr()[7],
    ",
  0x40346788u64 => "
      DSILINK.sqch0dsccr()[0],
      DSILINK.sqch0dsccr_l()[0],
      DSILINK.sqch0dsccr_ll()[0],
    ",
  0x40346798u64 => "
      DSILINK.sqch0dsccr()[1],
      DSILINK.sqch0dsccr_l()[1],
      DSILINK.sqch0dsccr_ll()[1],
    ",
  0x403467a8u64 => "
      DSILINK.sqch0dsccr()[2],
      DSILINK.sqch0dsccr_l()[2],
      DSILINK.sqch0dsccr_ll()[2],
    ",
  0x403467b8u64 => "
      DSILINK.sqch0dsccr()[3],
      DSILINK.sqch0dsccr_l()[3],
      DSILINK.sqch0dsccr_ll()[3],
    ",
  0x403467c8u64 => "
      DSILINK.sqch0dsccr()[4],
      DSILINK.sqch0dsccr_l()[4],
      DSILINK.sqch0dsccr_ll()[4],
    ",
  0x403467d8u64 => "
      DSILINK.sqch0dsccr()[5],
      DSILINK.sqch0dsccr_l()[5],
      DSILINK.sqch0dsccr_ll()[5],
    ",
  0x403467e8u64 => "
      DSILINK.sqch0dsccr()[6],
      DSILINK.sqch0dsccr_l()[6],
      DSILINK.sqch0dsccr_ll()[6],
    ",
  0x403467f8u64 => "
      DSILINK.sqch0dsccr()[7],
      DSILINK.sqch0dsccr_l()[7],
      DSILINK.sqch0dsccr_ll()[7],
    ",
  0x4034678au64 => "
      DSILINK.sqch0dsccr_h()[0],
      DSILINK.sqch0dsccr_hl()[0],
    ",
  0x4034679au64 => "
      DSILINK.sqch0dsccr_h()[1],
      DSILINK.sqch0dsccr_hl()[1],
    ",
  0x403467aau64 => "
      DSILINK.sqch0dsccr_h()[2],
      DSILINK.sqch0dsccr_hl()[2],
    ",
  0x403467bau64 => "
      DSILINK.sqch0dsccr_h()[3],
      DSILINK.sqch0dsccr_hl()[3],
    ",
  0x403467cau64 => "
      DSILINK.sqch0dsccr_h()[4],
      DSILINK.sqch0dsccr_hl()[4],
    ",
  0x403467dau64 => "
      DSILINK.sqch0dsccr_h()[5],
      DSILINK.sqch0dsccr_hl()[5],
    ",
  0x403467eau64 => "
      DSILINK.sqch0dsccr_h()[6],
      DSILINK.sqch0dsccr_hl()[6],
    ",
  0x403467fau64 => "
      DSILINK.sqch0dsccr_h()[7],
      DSILINK.sqch0dsccr_hl()[7],
    ",
  0x4034678bu64 => "
      DSILINK.sqch0dsccr_hh()[0],
    ",
  0x4034679bu64 => "
      DSILINK.sqch0dsccr_hh()[1],
    ",
  0x403467abu64 => "
      DSILINK.sqch0dsccr_hh()[2],
    ",
  0x403467bbu64 => "
      DSILINK.sqch0dsccr_hh()[3],
    ",
  0x403467cbu64 => "
      DSILINK.sqch0dsccr_hh()[4],
    ",
  0x403467dbu64 => "
      DSILINK.sqch0dsccr_hh()[5],
    ",
  0x403467ebu64 => "
      DSILINK.sqch0dsccr_hh()[6],
    ",
  0x403467fbu64 => "
      DSILINK.sqch0dsccr_hh()[7],
    ",
  0x4034678cu64 => "
      DSILINK.sqch0dscdr()[0],
      DSILINK.sqch0dscdr_l()[0],
      DSILINK.sqch0dscdr_ll()[0],
    ",
  0x4034679cu64 => "
      DSILINK.sqch0dscdr()[1],
      DSILINK.sqch0dscdr_l()[1],
      DSILINK.sqch0dscdr_ll()[1],
    ",
  0x403467acu64 => "
      DSILINK.sqch0dscdr()[2],
      DSILINK.sqch0dscdr_l()[2],
      DSILINK.sqch0dscdr_ll()[2],
    ",
  0x403467bcu64 => "
      DSILINK.sqch0dscdr()[3],
      DSILINK.sqch0dscdr_l()[3],
      DSILINK.sqch0dscdr_ll()[3],
    ",
  0x403467ccu64 => "
      DSILINK.sqch0dscdr()[4],
      DSILINK.sqch0dscdr_l()[4],
      DSILINK.sqch0dscdr_ll()[4],
    ",
  0x403467dcu64 => "
      DSILINK.sqch0dscdr()[5],
      DSILINK.sqch0dscdr_l()[5],
      DSILINK.sqch0dscdr_ll()[5],
    ",
  0x403467ecu64 => "
      DSILINK.sqch0dscdr()[6],
      DSILINK.sqch0dscdr_l()[6],
      DSILINK.sqch0dscdr_ll()[6],
    ",
  0x403467fcu64 => "
      DSILINK.sqch0dscdr()[7],
      DSILINK.sqch0dscdr_l()[7],
      DSILINK.sqch0dscdr_ll()[7],
    ",
  0x4034678du64 => "
      DSILINK.sqch0dscdr_lh()[0],
    ",
  0x4034679du64 => "
      DSILINK.sqch0dscdr_lh()[1],
    ",
  0x403467adu64 => "
      DSILINK.sqch0dscdr_lh()[2],
    ",
  0x403467bdu64 => "
      DSILINK.sqch0dscdr_lh()[3],
    ",
  0x403467cdu64 => "
      DSILINK.sqch0dscdr_lh()[4],
    ",
  0x403467ddu64 => "
      DSILINK.sqch0dscdr_lh()[5],
    ",
  0x403467edu64 => "
      DSILINK.sqch0dscdr_lh()[6],
    ",
  0x403467fdu64 => "
      DSILINK.sqch0dscdr_lh()[7],
    ",
  0x4034678eu64 => "
      DSILINK.sqch0dscdr_h()[0],
      DSILINK.sqch0dscdr_hl()[0],
    ",
  0x4034679eu64 => "
      DSILINK.sqch0dscdr_h()[1],
      DSILINK.sqch0dscdr_hl()[1],
    ",
  0x403467aeu64 => "
      DSILINK.sqch0dscdr_h()[2],
      DSILINK.sqch0dscdr_hl()[2],
    ",
  0x403467beu64 => "
      DSILINK.sqch0dscdr_h()[3],
      DSILINK.sqch0dscdr_hl()[3],
    ",
  0x403467ceu64 => "
      DSILINK.sqch0dscdr_h()[4],
      DSILINK.sqch0dscdr_hl()[4],
    ",
  0x403467deu64 => "
      DSILINK.sqch0dscdr_h()[5],
      DSILINK.sqch0dscdr_hl()[5],
    ",
  0x403467eeu64 => "
      DSILINK.sqch0dscdr_h()[6],
      DSILINK.sqch0dscdr_hl()[6],
    ",
  0x403467feu64 => "
      DSILINK.sqch0dscdr_h()[7],
      DSILINK.sqch0dscdr_hl()[7],
    ",
  0x4034678fu64 => "
      DSILINK.sqch0dscdr_hh()[0],
    ",
  0x4034679fu64 => "
      DSILINK.sqch0dscdr_hh()[1],
    ",
  0x403467afu64 => "
      DSILINK.sqch0dscdr_hh()[2],
    ",
  0x403467bfu64 => "
      DSILINK.sqch0dscdr_hh()[3],
    ",
  0x403467cfu64 => "
      DSILINK.sqch0dscdr_hh()[4],
    ",
  0x403467dfu64 => "
      DSILINK.sqch0dscdr_hh()[5],
    ",
  0x403467efu64 => "
      DSILINK.sqch0dscdr_hh()[6],
    ",
  0x403467ffu64 => "
      DSILINK.sqch0dscdr_hh()[7],
    ",
  0x40346800u64 => "
      DSILINK.sqch1dscar()[0],
      DSILINK.sqch1dscar_l()[0],
      DSILINK.sqch1dscar_ll()[0],
    ",
  0x40346810u64 => "
      DSILINK.sqch1dscar()[1],
      DSILINK.sqch1dscar_l()[1],
      DSILINK.sqch1dscar_ll()[1],
    ",
  0x40346820u64 => "
      DSILINK.sqch1dscar()[2],
      DSILINK.sqch1dscar_l()[2],
      DSILINK.sqch1dscar_ll()[2],
    ",
  0x40346830u64 => "
      DSILINK.sqch1dscar()[3],
      DSILINK.sqch1dscar_l()[3],
      DSILINK.sqch1dscar_ll()[3],
    ",
  0x40346840u64 => "
      DSILINK.sqch1dscar()[4],
      DSILINK.sqch1dscar_l()[4],
      DSILINK.sqch1dscar_ll()[4],
    ",
  0x40346850u64 => "
      DSILINK.sqch1dscar()[5],
      DSILINK.sqch1dscar_l()[5],
      DSILINK.sqch1dscar_ll()[5],
    ",
  0x40346860u64 => "
      DSILINK.sqch1dscar()[6],
      DSILINK.sqch1dscar_l()[6],
      DSILINK.sqch1dscar_ll()[6],
    ",
  0x40346870u64 => "
      DSILINK.sqch1dscar()[7],
      DSILINK.sqch1dscar_l()[7],
      DSILINK.sqch1dscar_ll()[7],
    ",
  0x40346801u64 => "
      DSILINK.sqch1dscar_lh()[0],
    ",
  0x40346811u64 => "
      DSILINK.sqch1dscar_lh()[1],
    ",
  0x40346821u64 => "
      DSILINK.sqch1dscar_lh()[2],
    ",
  0x40346831u64 => "
      DSILINK.sqch1dscar_lh()[3],
    ",
  0x40346841u64 => "
      DSILINK.sqch1dscar_lh()[4],
    ",
  0x40346851u64 => "
      DSILINK.sqch1dscar_lh()[5],
    ",
  0x40346861u64 => "
      DSILINK.sqch1dscar_lh()[6],
    ",
  0x40346871u64 => "
      DSILINK.sqch1dscar_lh()[7],
    ",
  0x40346802u64 => "
      DSILINK.sqch1dscar_h()[0],
      DSILINK.sqch1dscar_hl()[0],
    ",
  0x40346812u64 => "
      DSILINK.sqch1dscar_h()[1],
      DSILINK.sqch1dscar_hl()[1],
    ",
  0x40346822u64 => "
      DSILINK.sqch1dscar_h()[2],
      DSILINK.sqch1dscar_hl()[2],
    ",
  0x40346832u64 => "
      DSILINK.sqch1dscar_h()[3],
      DSILINK.sqch1dscar_hl()[3],
    ",
  0x40346842u64 => "
      DSILINK.sqch1dscar_h()[4],
      DSILINK.sqch1dscar_hl()[4],
    ",
  0x40346852u64 => "
      DSILINK.sqch1dscar_h()[5],
      DSILINK.sqch1dscar_hl()[5],
    ",
  0x40346862u64 => "
      DSILINK.sqch1dscar_h()[6],
      DSILINK.sqch1dscar_hl()[6],
    ",
  0x40346872u64 => "
      DSILINK.sqch1dscar_h()[7],
      DSILINK.sqch1dscar_hl()[7],
    ",
  0x40346803u64 => "
      DSILINK.sqch1dscar_hh()[0],
    ",
  0x40346813u64 => "
      DSILINK.sqch1dscar_hh()[1],
    ",
  0x40346823u64 => "
      DSILINK.sqch1dscar_hh()[2],
    ",
  0x40346833u64 => "
      DSILINK.sqch1dscar_hh()[3],
    ",
  0x40346843u64 => "
      DSILINK.sqch1dscar_hh()[4],
    ",
  0x40346853u64 => "
      DSILINK.sqch1dscar_hh()[5],
    ",
  0x40346863u64 => "
      DSILINK.sqch1dscar_hh()[6],
    ",
  0x40346873u64 => "
      DSILINK.sqch1dscar_hh()[7],
    ",
  0x40346804u64 => "
      DSILINK.sqch1dscbr()[0],
    ",
  0x40346814u64 => "
      DSILINK.sqch1dscbr()[1],
    ",
  0x40346824u64 => "
      DSILINK.sqch1dscbr()[2],
    ",
  0x40346834u64 => "
      DSILINK.sqch1dscbr()[3],
    ",
  0x40346844u64 => "
      DSILINK.sqch1dscbr()[4],
    ",
  0x40346854u64 => "
      DSILINK.sqch1dscbr()[5],
    ",
  0x40346864u64 => "
      DSILINK.sqch1dscbr()[6],
    ",
  0x40346874u64 => "
      DSILINK.sqch1dscbr()[7],
    ",
  0x40346808u64 => "
      DSILINK.sqch1dsccr()[0],
      DSILINK.sqch1dsccr_l()[0],
      DSILINK.sqch1dsccr_ll()[0],
    ",
  0x40346818u64 => "
      DSILINK.sqch1dsccr()[1],
      DSILINK.sqch1dsccr_l()[1],
      DSILINK.sqch1dsccr_ll()[1],
    ",
  0x40346828u64 => "
      DSILINK.sqch1dsccr()[2],
      DSILINK.sqch1dsccr_l()[2],
      DSILINK.sqch1dsccr_ll()[2],
    ",
  0x40346838u64 => "
      DSILINK.sqch1dsccr()[3],
      DSILINK.sqch1dsccr_l()[3],
      DSILINK.sqch1dsccr_ll()[3],
    ",
  0x40346848u64 => "
      DSILINK.sqch1dsccr()[4],
      DSILINK.sqch1dsccr_l()[4],
      DSILINK.sqch1dsccr_ll()[4],
    ",
  0x40346858u64 => "
      DSILINK.sqch1dsccr()[5],
      DSILINK.sqch1dsccr_l()[5],
      DSILINK.sqch1dsccr_ll()[5],
    ",
  0x40346868u64 => "
      DSILINK.sqch1dsccr()[6],
      DSILINK.sqch1dsccr_l()[6],
      DSILINK.sqch1dsccr_ll()[6],
    ",
  0x40346878u64 => "
      DSILINK.sqch1dsccr()[7],
      DSILINK.sqch1dsccr_l()[7],
      DSILINK.sqch1dsccr_ll()[7],
    ",
  0x4034680au64 => "
      DSILINK.sqch1dsccr_h()[0],
      DSILINK.sqch1dsccr_hl()[0],
    ",
  0x4034681au64 => "
      DSILINK.sqch1dsccr_h()[1],
      DSILINK.sqch1dsccr_hl()[1],
    ",
  0x4034682au64 => "
      DSILINK.sqch1dsccr_h()[2],
      DSILINK.sqch1dsccr_hl()[2],
    ",
  0x4034683au64 => "
      DSILINK.sqch1dsccr_h()[3],
      DSILINK.sqch1dsccr_hl()[3],
    ",
  0x4034684au64 => "
      DSILINK.sqch1dsccr_h()[4],
      DSILINK.sqch1dsccr_hl()[4],
    ",
  0x4034685au64 => "
      DSILINK.sqch1dsccr_h()[5],
      DSILINK.sqch1dsccr_hl()[5],
    ",
  0x4034686au64 => "
      DSILINK.sqch1dsccr_h()[6],
      DSILINK.sqch1dsccr_hl()[6],
    ",
  0x4034687au64 => "
      DSILINK.sqch1dsccr_h()[7],
      DSILINK.sqch1dsccr_hl()[7],
    ",
  0x4034680bu64 => "
      DSILINK.sqch1dsccr_hh()[0],
    ",
  0x4034681bu64 => "
      DSILINK.sqch1dsccr_hh()[1],
    ",
  0x4034682bu64 => "
      DSILINK.sqch1dsccr_hh()[2],
    ",
  0x4034683bu64 => "
      DSILINK.sqch1dsccr_hh()[3],
    ",
  0x4034684bu64 => "
      DSILINK.sqch1dsccr_hh()[4],
    ",
  0x4034685bu64 => "
      DSILINK.sqch1dsccr_hh()[5],
    ",
  0x4034686bu64 => "
      DSILINK.sqch1dsccr_hh()[6],
    ",
  0x4034687bu64 => "
      DSILINK.sqch1dsccr_hh()[7],
    ",
  0x4034680cu64 => "
      DSILINK.sqch1dscdr()[0],
      DSILINK.sqch1dscdr_l()[0],
      DSILINK.sqch1dscdr_ll()[0],
    ",
  0x4034681cu64 => "
      DSILINK.sqch1dscdr()[1],
      DSILINK.sqch1dscdr_l()[1],
      DSILINK.sqch1dscdr_ll()[1],
    ",
  0x4034682cu64 => "
      DSILINK.sqch1dscdr()[2],
      DSILINK.sqch1dscdr_l()[2],
      DSILINK.sqch1dscdr_ll()[2],
    ",
  0x4034683cu64 => "
      DSILINK.sqch1dscdr()[3],
      DSILINK.sqch1dscdr_l()[3],
      DSILINK.sqch1dscdr_ll()[3],
    ",
  0x4034684cu64 => "
      DSILINK.sqch1dscdr()[4],
      DSILINK.sqch1dscdr_l()[4],
      DSILINK.sqch1dscdr_ll()[4],
    ",
  0x4034685cu64 => "
      DSILINK.sqch1dscdr()[5],
      DSILINK.sqch1dscdr_l()[5],
      DSILINK.sqch1dscdr_ll()[5],
    ",
  0x4034686cu64 => "
      DSILINK.sqch1dscdr()[6],
      DSILINK.sqch1dscdr_l()[6],
      DSILINK.sqch1dscdr_ll()[6],
    ",
  0x4034687cu64 => "
      DSILINK.sqch1dscdr()[7],
      DSILINK.sqch1dscdr_l()[7],
      DSILINK.sqch1dscdr_ll()[7],
    ",
  0x4034680du64 => "
      DSILINK.sqch1dscdr_lh()[0],
    ",
  0x4034681du64 => "
      DSILINK.sqch1dscdr_lh()[1],
    ",
  0x4034682du64 => "
      DSILINK.sqch1dscdr_lh()[2],
    ",
  0x4034683du64 => "
      DSILINK.sqch1dscdr_lh()[3],
    ",
  0x4034684du64 => "
      DSILINK.sqch1dscdr_lh()[4],
    ",
  0x4034685du64 => "
      DSILINK.sqch1dscdr_lh()[5],
    ",
  0x4034686du64 => "
      DSILINK.sqch1dscdr_lh()[6],
    ",
  0x4034687du64 => "
      DSILINK.sqch1dscdr_lh()[7],
    ",
  0x4034680eu64 => "
      DSILINK.sqch1dscdr_h()[0],
      DSILINK.sqch1dscdr_hl()[0],
    ",
  0x4034681eu64 => "
      DSILINK.sqch1dscdr_h()[1],
      DSILINK.sqch1dscdr_hl()[1],
    ",
  0x4034682eu64 => "
      DSILINK.sqch1dscdr_h()[2],
      DSILINK.sqch1dscdr_hl()[2],
    ",
  0x4034683eu64 => "
      DSILINK.sqch1dscdr_h()[3],
      DSILINK.sqch1dscdr_hl()[3],
    ",
  0x4034684eu64 => "
      DSILINK.sqch1dscdr_h()[4],
      DSILINK.sqch1dscdr_hl()[4],
    ",
  0x4034685eu64 => "
      DSILINK.sqch1dscdr_h()[5],
      DSILINK.sqch1dscdr_hl()[5],
    ",
  0x4034686eu64 => "
      DSILINK.sqch1dscdr_h()[6],
      DSILINK.sqch1dscdr_hl()[6],
    ",
  0x4034687eu64 => "
      DSILINK.sqch1dscdr_h()[7],
      DSILINK.sqch1dscdr_hl()[7],
    ",
  0x4034680fu64 => "
      DSILINK.sqch1dscdr_hh()[0],
    ",
  0x4034681fu64 => "
      DSILINK.sqch1dscdr_hh()[1],
    ",
  0x4034682fu64 => "
      DSILINK.sqch1dscdr_hh()[2],
    ",
  0x4034683fu64 => "
      DSILINK.sqch1dscdr_hh()[3],
    ",
  0x4034684fu64 => "
      DSILINK.sqch1dscdr_hh()[4],
    ",
  0x4034685fu64 => "
      DSILINK.sqch1dscdr_hh()[5],
    ",
  0x4034686fu64 => "
      DSILINK.sqch1dscdr_hh()[6],
    ",
  0x4034687fu64 => "
      DSILINK.sqch1dscdr_hh()[7],
    ",
  0x40346c00u64 => "
      DPHYCNT.dphyrefcr(),
      DPHYCNT.dphyrefcr_l(),
      DPHYCNT.dphyrefcr_ll(),
    ",
  0x40346c04u64 => "
      DPHYCNT.dphyplfcr(),
      DPHYCNT.dphyplfcr_l(),
      DPHYCNT.dphyplfcr_ll(),
    ",
  0x40346c05u64 => "
      DPHYCNT.dphyplfcr_lh(),
    ",
  0x40346c06u64 => "
      DPHYCNT.dphyplfcr_h(),
      DPHYCNT.dphyplfcr_hl(),
    ",
  0x40346c08u64 => "
      DPHYCNT.dphyplocr(),
      DPHYCNT.dphyplocr_l(),
      DPHYCNT.dphyplocr_ll(),
    ",
  0x40346c0cu64 => "
      DPHYCNT.dphyesccr(),
      DPHYCNT.dphyesccr_l(),
      DPHYCNT.dphyesccr_ll(),
    ",
  0x40346c10u64 => "
      DPHYCNT.dphypwrcr(),
      DPHYCNT.dphypwrcr_l(),
      DPHYCNT.dphypwrcr_ll(),
    ",
  0x40346c1cu64 => "
      DPHYCNT.dphysfr(),
      DPHYCNT.dphysfr_l(),
      DPHYCNT.dphysfr_ll(),
    ",
  0x40346c1du64 => "
      DPHYCNT.dphysfr_lh(),
    ",
  0x40346c20u64 => "
      DPHYCNT.dphyocr(),
      DPHYCNT.dphyocr_l(),
      DPHYCNT.dphyocr_ll(),
    ",
  0x40346c24u64 => "
      DPHYCNT.dphytim1(),
      DPHYCNT.dphytim1_l(),
      DPHYCNT.dphytim1_ll(),
    ",
  0x40346c25u64 => "
      DPHYCNT.dphytim1_lh(),
    ",
  0x40346c26u64 => "
      DPHYCNT.dphytim1_h(),
      DPHYCNT.dphytim1_hl(),
    ",
  0x40346c28u64 => "
      DPHYCNT.dphytim2_ll(),
    ",
  0x40346c29u64 => "
      DPHYCNT.dphytim2_lh(),
    ",
  0x40346c2au64 => "
      DPHYCNT.dphytim2_hl(),
    ",
  0x40346c2cu64 => "
      DPHYCNT.dphytim3_ll(),
    ",
  0x40346c2du64 => "
      DPHYCNT.dphytim3_lh(),
    ",
  0x40346c30u64 => "
      DPHYCNT.dphytim4(),
      DPHYCNT.dphytim4_l(),
      DPHYCNT.dphytim4_ll(),
    ",
  0x40346c31u64 => "
      DPHYCNT.dphytim4_lh(),
    ",
  0x40346c32u64 => "
      DPHYCNT.dphytim4_h(),
      DPHYCNT.dphytim4_hl(),
    ",
  0x40346c33u64 => "
      DPHYCNT.dphytim4_hh(),
    ",
  0x40346c34u64 => "
      DPHYCNT.dphytim5(),
      DPHYCNT.dphytim5_l(),
      DPHYCNT.dphytim5_ll(),
    ",
  0x40346c35u64 => "
      DPHYCNT.dphytim5_lh(),
    ",
  0x40346c36u64 => "
      DPHYCNT.dphytim5_h(),
      DPHYCNT.dphytim5_hl(),
    ",
  0x40346c38u64 => "
      DPHYCNT.dphytim6(),
      DPHYCNT.dphytim6_l(),
      DPHYCNT.dphytim6_ll(),
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
      CEU.axibusctl2(),
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
  0x40351000u64 => "
      USBHS.syscfg(),
    ",
  0x40351002u64 => "
      USBHS.buswait(),
    ",
  0x40351004u64 => "
      USBHS.syssts0(),
    ",
  0x40351006u64 => "
      USBHS.pllsta(),
    ",
  0x40351008u64 => "
      USBHS.dvstctr0(),
    ",
  0x4035100cu64 => "
      USBHS.testmode(),
    ",
  0x40351014u64 => "
      USBHS.cfifo(),
      USBHS.cfifol(),
      USBHS.cfifoll(),
    ",
  0x40351016u64 => "
      USBHS.cfifoh(),
    ",
  0x40351017u64 => "
      USBHS.cfifohh(),
    ",
  0x40351018u64 => "
      USBHS.d0fifo(),
      USBHS.d0fifol(),
      USBHS.d0fifoll(),
    ",
  0x4035101au64 => "
      USBHS.d0fifoh(),
    ",
  0x4035101bu64 => "
      USBHS.d0fifohh(),
    ",
  0x4035101cu64 => "
      USBHS.d1fifo(),
      USBHS.d1fifol(),
      USBHS.d1fifoll(),
    ",
  0x4035101eu64 => "
      USBHS.d1fifoh(),
    ",
  0x4035101fu64 => "
      USBHS.d1fifohh(),
    ",
  0x40351020u64 => "
      USBHS.cfifosel(),
    ",
  0x40351022u64 => "
      USBHS.cfifoctr(),
    ",
  0x40351028u64 => "
      USBHS.d0fifosel(),
    ",
  0x4035102au64 => "
      USBHS.d0fifoctr(),
    ",
  0x4035102cu64 => "
      USBHS.d1fifosel(),
    ",
  0x4035102eu64 => "
      USBHS.d1fifoctr(),
    ",
  0x40351030u64 => "
      USBHS.intenb0(),
    ",
  0x40351032u64 => "
      USBHS.intenb1(),
    ",
  0x40351036u64 => "
      USBHS.brdyenb(),
    ",
  0x40351038u64 => "
      USBHS.nrdyenb(),
    ",
  0x4035103au64 => "
      USBHS.bempenb(),
    ",
  0x4035103cu64 => "
      USBHS.sofcfg(),
    ",
  0x4035103eu64 => "
      USBHS.physet(),
    ",
  0x40351040u64 => "
      USBHS.intsts0(),
    ",
  0x40351042u64 => "
      USBHS.intsts1(),
    ",
  0x40351046u64 => "
      USBHS.brdysts(),
    ",
  0x40351048u64 => "
      USBHS.nrdysts(),
    ",
  0x4035104au64 => "
      USBHS.bempsts(),
    ",
  0x4035104cu64 => "
      USBHS.frmnum(),
    ",
  0x4035104eu64 => "
      USBHS.ufrmnum(),
    ",
  0x40351050u64 => "
      USBHS.usbaddr(),
    ",
  0x40351054u64 => "
      USBHS.usbreq(),
    ",
  0x40351056u64 => "
      USBHS.usbval(),
    ",
  0x40351058u64 => "
      USBHS.usbindx(),
    ",
  0x4035105au64 => "
      USBHS.usbleng(),
    ",
  0x4035105cu64 => "
      USBHS.dcpcfg(),
    ",
  0x4035105eu64 => "
      USBHS.dcpmaxp(),
    ",
  0x40351060u64 => "
      USBHS.dcpctr(),
    ",
  0x40351064u64 => "
      USBHS.pipesel(),
    ",
  0x40351068u64 => "
      USBHS.pipecfg(),
    ",
  0x4035106au64 => "
      USBHS.pipebuf(),
    ",
  0x4035106cu64 => "
      USBHS.pipemaxp(),
    ",
  0x4035106eu64 => "
      USBHS.pipeperi(),
    ",
  0x40351070u64 => "
      USBHS.pipectr()[0],
    ",
  0x40351072u64 => "
      USBHS.pipectr()[1],
    ",
  0x40351074u64 => "
      USBHS.pipectr()[2],
    ",
  0x40351076u64 => "
      USBHS.pipectr()[3],
    ",
  0x40351078u64 => "
      USBHS.pipectr()[4],
    ",
  0x4035107au64 => "
      USBHS.pipectr()[5],
    ",
  0x4035107cu64 => "
      USBHS.pipectr()[6],
    ",
  0x4035107eu64 => "
      USBHS.pipectr()[7],
    ",
  0x40351080u64 => "
      USBHS.pipectr()[8],
    ",
  0x40351090u64 => "
      USBHS.pipetre()[0],
    ",
  0x40351094u64 => "
      USBHS.pipetre()[1],
    ",
  0x40351098u64 => "
      USBHS.pipetre()[2],
    ",
  0x4035109cu64 => "
      USBHS.pipetre()[3],
    ",
  0x403510a0u64 => "
      USBHS.pipetre()[4],
    ",
  0x40351092u64 => "
      USBHS.pipetrn()[0],
    ",
  0x40351096u64 => "
      USBHS.pipetrn()[1],
    ",
  0x4035109au64 => "
      USBHS.pipetrn()[2],
    ",
  0x4035109eu64 => "
      USBHS.pipetrn()[3],
    ",
  0x403510a2u64 => "
      USBHS.pipetrn()[4],
    ",
  0x403510d0u64 => "
      USBHS.devadd()[0],
    ",
  0x403510d2u64 => "
      USBHS.devadd()[1],
    ",
  0x403510d4u64 => "
      USBHS.devadd()[2],
    ",
  0x403510d6u64 => "
      USBHS.devadd()[3],
    ",
  0x403510d8u64 => "
      USBHS.devadd()[4],
    ",
  0x403510dau64 => "
      USBHS.devadd()[5],
    ",
  0x403510dcu64 => "
      USBHS.devadd()[6],
    ",
  0x403510deu64 => "
      USBHS.devadd()[7],
    ",
  0x403510e0u64 => "
      USBHS.devadd()[8],
    ",
  0x403510e2u64 => "
      USBHS.devadd()[9],
    ",
  0x403510e4u64 => "
      USBHS.devadda(),
    ",
  0x40351100u64 => "
      USBHS.lpctrl(),
    ",
  0x40351102u64 => "
      USBHS.lpsts(),
    ",
  0x40351140u64 => "
      USBHS.bcctrl(),
    ",
  0x40351144u64 => "
      USBHS.pl1ctrl1(),
    ",
  0x40351146u64 => "
      USBHS.pl1ctrl2(),
    ",
  0x40351148u64 => "
      USBHS.hl1ctrl1(),
    ",
  0x4035114au64 => "
      USBHS.hl1ctrl2(),
    ",
  0x40351154u64 => "
      USBHS.vrcgctrl1(),
    ",
  0x40351156u64 => "
      USBHS.vrcgctrl2(),
    ",
  0x40351160u64 => "
      USBHS.dpusr0r(),
    ",
  0x40351164u64 => "
      USBHS.dpusr1r(),
    ",
  0x40351168u64 => "
      USBHS.dpusr2r(),
    ",
  0x4035116au64 => "
      USBHS.dpusrcr(),
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
      SCI_0.rdr(),
    ",
  0x40358004u64 => "
      SCI_0.tdr(),
      SCI_0.tdr_ha_l(),
      SCI_0.tdr_by_ll(),
    ",
  0x40358005u64 => "
      SCI_0.tdr_by_lh(),
    ",
  0x40358008u64 => "
      SCI_0.ccr0(),
      SCI_0.ccr0_ha_l(),
      SCI_0.ccr0_by_ll(),
    ",
  0x40358009u64 => "
      SCI_0.ccr0_by_lh(),
    ",
  0x4035800au64 => "
      SCI_0.ccr0_ha_h(),
      SCI_0.ccr0_by_hl(),
    ",
  0x4035800bu64 => "
      SCI_0.ccr0_by_hh(),
    ",
  0x4035800cu64 => "
      SCI_0.ccr1(),
      SCI_0.ccr1_ha_l(),
      SCI_0.ccr1_by_ll(),
    ",
  0x4035800du64 => "
      SCI_0.ccr1_by_lh(),
    ",
  0x4035800eu64 => "
      SCI_0.ccr1_ha_h(),
      SCI_0.ccr1_by_hl(),
    ",
  0x4035800fu64 => "
      SCI_0.ccr1_by_hh(),
    ",
  0x40358010u64 => "
      SCI_0.ccr2(),
      SCI_0.ccr2_ha_l(),
      SCI_0.ccr2_by_ll(),
    ",
  0x40358011u64 => "
      SCI_0.ccr2_by_lh(),
    ",
  0x40358012u64 => "
      SCI_0.ccr2_ha_h(),
      SCI_0.ccr2_by_hl(),
    ",
  0x40358013u64 => "
      SCI_0.ccr2_by_hh(),
    ",
  0x40358014u64 => "
      SCI_0.ccr3(),
      SCI_0.ccr3_ha_l(),
      SCI_0.ccr3_by_ll(),
    ",
  0x40358015u64 => "
      SCI_0.ccr3_by_lh(),
    ",
  0x40358016u64 => "
      SCI_0.ccr3_ha_h(),
      SCI_0.ccr3_by_hl(),
    ",
  0x40358017u64 => "
      SCI_0.ccr3_by_hh(),
    ",
  0x40358018u64 => "
      SCI_0.ccr4(),
      SCI_0.ccr4_ha_l(),
      SCI_0.ccr4_by_ll(),
    ",
  0x40358019u64 => "
      SCI_0.ccr4_by_lh(),
    ",
  0x4035801au64 => "
      SCI_0.ccr4_ha_h(),
      SCI_0.ccr4_by_hl(),
    ",
  0x4035801bu64 => "
      SCI_0.ccr4_by_hh(),
    ",
  0x4035801cu64 => "
      SCI_0.cesr(),
    ",
  0x4035801eu64 => "
      SCI_0.hcr(),
    ",
  0x40358020u64 => "
      SCI_0.icr(),
      SCI_0.icr_ha_l(),
      SCI_0.icr_by_ll(),
    ",
  0x40358021u64 => "
      SCI_0.icr_by_lh(),
    ",
  0x40358022u64 => "
      SCI_0.icr_ha_h(),
      SCI_0.icr_by_hl(),
    ",
  0x40358024u64 => "
      SCI_0.fcr(),
      SCI_0.fcr_ha_l(),
      SCI_0.fcr_by_ll(),
    ",
  0x40358025u64 => "
      SCI_0.fcr_by_lh(),
    ",
  0x40358026u64 => "
      SCI_0.fcr_ha_h(),
      SCI_0.fcr_by_hl(),
    ",
  0x40358027u64 => "
      SCI_0.fcr_by_hh(),
    ",
  0x4035802cu64 => "
      SCI_0.mcr(),
      SCI_0.mcr_ha_l(),
      SCI_0.mcr_by_ll(),
    ",
  0x4035802du64 => "
      SCI_0.mcr_by_lh(),
    ",
  0x4035802eu64 => "
      SCI_0.mcr_ha_h(),
      SCI_0.mcr_by_hl(),
    ",
  0x4035802fu64 => "
      SCI_0.mcr_by_hh(),
    ",
  0x40358030u64 => "
      SCI_0.dcr(),
      SCI_0.dcr_ha_l(),
      SCI_0.dcr_by_ll(),
    ",
  0x40358031u64 => "
      SCI_0.dcr_by_lh(),
    ",
  0x40358032u64 => "
      SCI_0.dcr_ha_h(),
      SCI_0.dcr_by_hl(),
    ",
  0x40358034u64 => "
      SCI_0.xcr0(),
      SCI_0.xcr0_ha_l(),
      SCI_0.xcr0_by_ll(),
    ",
  0x40358035u64 => "
      SCI_0.xcr0_by_lh(),
    ",
  0x40358036u64 => "
      SCI_0.xcr0_ha_h(),
      SCI_0.xcr0_by_hl(),
    ",
  0x40358037u64 => "
      SCI_0.xcr0_by_hh(),
    ",
  0x40358038u64 => "
      SCI_0.xcr1(),
      SCI_0.xcr1_ha_l(),
      SCI_0.xcr1_by_ll(),
    ",
  0x40358039u64 => "
      SCI_0.xcr1_by_lh(),
    ",
  0x4035803au64 => "
      SCI_0.xcr1_ha_h(),
      SCI_0.xcr1_by_hl(),
    ",
  0x4035803bu64 => "
      SCI_0.xcr1_by_hh(),
    ",
  0x4035803cu64 => "
      SCI_0.xcr2(),
      SCI_0.xcr2_ha_l(),
      SCI_0.xcr2_by_ll(),
    ",
  0x4035803du64 => "
      SCI_0.xcr2_by_lh(),
    ",
  0x4035803eu64 => "
      SCI_0.xcr2_ha_h(),
      SCI_0.xcr2_by_hl(),
    ",
  0x4035803fu64 => "
      SCI_0.xcr2_by_hh(),
    ",
  0x40358048u64 => "
      SCI_0.csr(),
    ",
  0x4035804cu64 => "
      SCI_0.isr(),
    ",
  0x40358050u64 => "
      SCI_0.frsr(),
    ",
  0x40358054u64 => "
      SCI_0.ftsr(),
    ",
  0x40358058u64 => "
      SCI_0.msr(),
    ",
  0x4035805cu64 => "
      SCI_0.xsr0(),
    ",
  0x40358060u64 => "
      SCI_0.xsr1(),
    ",
  0x40358068u64 => "
      SCI_0.cfclr(),
      SCI_0.cfclr_ha_l(),
      SCI_0.cfclr_by_ll(),
    ",
  0x4035806au64 => "
      SCI_0.cfclr_ha_h(),
      SCI_0.cfclr_by_hl(),
    ",
  0x4035806bu64 => "
      SCI_0.cfclr_by_hh(),
    ",
  0x4035806cu64 => "
      SCI_0.icfclr(),
      SCI_0.icfclr_ha_l(),
      SCI_0.icfclr_by_ll(),
    ",
  0x40358070u64 => "
      SCI_0.ffclr(),
      SCI_0.ffclr_ha_l(),
      SCI_0.ffclr_by_ll(),
    ",
  0x40358074u64 => "
      SCI_0.mfclr(),
      SCI_0.mfclr_ha_l(),
      SCI_0.mfclr_by_ll(),
    ",
  0x40358078u64 => "
      SCI_0.xfclr(),
      SCI_0.xfclr_ha_l(),
    ",
  0x40358079u64 => "
      SCI_0.xfclr_by_lh(),
    ",
  0x4035c000u64 => "
      SPI_0.spdr(),
    ",
  0x4035c004u64 => "
      SPI_0.spdecr(),
    ",
  0x4035c008u64 => "
      SPI_0.spcr(),
    ",
  0x4035c00cu64 => "
      SPI_0.spcr2(),
    ",
  0x4035c010u64 => "
      SPI_0.spcr3(),
    ",
  0x4035c014u64 => "
      SPI_0.spcmd0(),
    ",
  0x4035c018u64 => "
      SPI_0.spcmd1(),
    ",
  0x4035c01cu64 => "
      SPI_0.spcmd2(),
    ",
  0x4035c020u64 => "
      SPI_0.spcmd3(),
    ",
  0x4035c024u64 => "
      SPI_0.spcmd4(),
    ",
  0x4035c028u64 => "
      SPI_0.spcmd5(),
    ",
  0x4035c02cu64 => "
      SPI_0.spcmd6(),
    ",
  0x4035c030u64 => "
      SPI_0.spcmd7(),
    ",
  0x4035c040u64 => "
      SPI_0.spdcr(),
    ",
  0x4035c044u64 => "
      SPI_0.spdcr2(),
    ",
  0x4035c050u64 => "
      SPI_0.spsr(),
    ",
  0x4035c058u64 => "
      SPI_0.sptfsr(),
    ",
  0x4035c05cu64 => "
      SPI_0.sprfsr(),
    ",
  0x4035c060u64 => "
      SPI_0.sppsr(),
    ",
  0x4035c068u64 => "
      SPI_0.spsrc(),
    ",
  0x4035c06cu64 => "
      SPI_0.spfcr(),
    ",
  0x4035f000u64 => "
      I_3_C.prts(),
      I_3_C.prts_ha_l(),
      I_3_C.prts_by_ll(),
    ",
  0x4035f014u64 => "
      I_3_C.bctl(),
      I_3_C.bctl_ha_l(),
      I_3_C.bctl_by_ll(),
    ",
  0x4035f015u64 => "
      I_3_C.bctl_by_lh(),
    ",
  0x4035f016u64 => "
      I_3_C.bctl_ha_h(),
    ",
  0x4035f017u64 => "
      I_3_C.bctl_by_hh(),
    ",
  0x4035f018u64 => "
      I_3_C.msdvad(),
    ",
  0x4035f01au64 => "
      I_3_C.msdvad_ha_h(),
      I_3_C.msdvad_by_hl(),
    ",
  0x4035f01bu64 => "
      I_3_C.msdvad_by_hh(),
    ",
  0x4035f020u64 => "
      I_3_C.rstctl(),
      I_3_C.rstctl_ha_l(),
      I_3_C.rstctl_by_ll(),
    ",
  0x4035f021u64 => "
      I_3_C.rstctl_by_lh(),
    ",
  0x4035f022u64 => "
      I_3_C.rstctl_ha_h(),
      I_3_C.rstctl_by_hl(),
    ",
  0x4035f024u64 => "
      I_3_C.prsst(),
      I_3_C.prsst_ha_l(),
      I_3_C.prsst_by_ll(),
    ",
  0x4035f030u64 => "
      I_3_C.inst(),
      I_3_C.inst_ha_l(),
    ",
  0x4035f031u64 => "
      I_3_C.inst_by_lh(),
    ",
  0x4035f034u64 => "
      I_3_C.inste(),
      I_3_C.inste_ha_l(),
    ",
  0x4035f035u64 => "
      I_3_C.inste_by_lh(),
    ",
  0x4035f038u64 => "
      I_3_C.inie(),
      I_3_C.inie_ha_l(),
    ",
  0x4035f039u64 => "
      I_3_C.inie_by_lh(),
    ",
  0x4035f03cu64 => "
      I_3_C.instfc(),
      I_3_C.instfc_ha_l(),
    ",
  0x4035f03du64 => "
      I_3_C.instfc_by_lh(),
    ",
  0x4035f044u64 => "
      I_3_C.dvct(),
    ",
  0x4035f046u64 => "
      I_3_C.dvct_ha_h(),
      I_3_C.dvct_by_hl(),
    ",
  0x4035f058u64 => "
      I_3_C.ibinctl(),
      I_3_C.ibinctl_ha_l(),
      I_3_C.ibinctl_by_ll(),
    ",
  0x4035f060u64 => "
      I_3_C.bfctl(),
      I_3_C.bfctl_ha_l(),
      I_3_C.bfctl_by_ll(),
    ",
  0x4035f061u64 => "
      I_3_C.bfctl_by_lh(),
    ",
  0x4035f064u64 => "
      I_3_C.svctl(),
      I_3_C.svctl_ha_l(),
      I_3_C.svctl_by_ll(),
    ",
  0x4035f065u64 => "
      I_3_C.svctl_by_lh(),
    ",
  0x4035f066u64 => "
      I_3_C.svctl_ha_h(),
      I_3_C.svctl_by_hl(),
    ",
  0x4035f070u64 => "
      I_3_C.refckctl(),
      I_3_C.refckctl_ha_l(),
      I_3_C.refckctl_by_ll(),
    ",
  0x4035f074u64 => "
      I_3_C.stdbr(),
      I_3_C.stdbr_ha_l(),
      I_3_C.stdbr_by_ll(),
    ",
  0x4035f075u64 => "
      I_3_C.stdbr_by_lh(),
    ",
  0x4035f076u64 => "
      I_3_C.stdbr_ha_h(),
      I_3_C.stdbr_by_hl(),
    ",
  0x4035f077u64 => "
      I_3_C.stdbr_by_hh(),
    ",
  0x4035f078u64 => "
      I_3_C.extbr(),
      I_3_C.extbr_ha_l(),
      I_3_C.extbr_by_ll(),
    ",
  0x4035f079u64 => "
      I_3_C.extbr_by_lh(),
    ",
  0x4035f07au64 => "
      I_3_C.extbr_ha_h(),
      I_3_C.extbr_by_hl(),
    ",
  0x4035f07bu64 => "
      I_3_C.extbr_by_hh(),
    ",
  0x4035f07cu64 => "
      I_3_C.bfrecdt(),
      I_3_C.bfrecdt_ha_l(),
      I_3_C.bfrecdt_by_ll(),
    ",
  0x4035f07du64 => "
      I_3_C.bfrecdt_by_lh(),
    ",
  0x4035f080u64 => "
      I_3_C.bavlcdt(),
      I_3_C.bavlcdt_ha_l(),
      I_3_C.bavlcdt_by_ll(),
    ",
  0x4035f081u64 => "
      I_3_C.bavlcdt_by_lh(),
    ",
  0x4035f084u64 => "
      I_3_C.bidlcdt(),
      I_3_C.bidlcdt_ha_l(),
      I_3_C.bidlcdt_by_ll(),
    ",
  0x4035f085u64 => "
      I_3_C.bidlcdt_by_lh(),
    ",
  0x4035f086u64 => "
      I_3_C.bidlcdt_ha_h(),
      I_3_C.bidlcdt_by_hl(),
    ",
  0x4035f088u64 => "
      I_3_C.outctl(),
      I_3_C.outctl_ha_l(),
      I_3_C.outctl_by_ll(),
    ",
  0x4035f089u64 => "
      I_3_C.outctl_by_lh(),
    ",
  0x4035f08cu64 => "
      I_3_C.inctl(),
      I_3_C.inctl_ha_l(),
      I_3_C.inctl_by_ll(),
    ",
  0x4035f090u64 => "
      I_3_C.tmoctl(),
      I_3_C.tmoctl_ha_l(),
      I_3_C.tmoctl_by_ll(),
    ",
  0x4035f098u64 => "
      I_3_C.wuctl(),
      I_3_C.wuctl_ha_l(),
      I_3_C.wuctl_by_ll(),
    ",
  0x4035f0a0u64 => "
      I_3_C.ackctl(),
      I_3_C.ackctl_ha_l(),
      I_3_C.ackctl_by_ll(),
    ",
  0x4035f0a4u64 => "
      I_3_C.scstrctl(),
      I_3_C.scstrctl_ha_l(),
      I_3_C.scstrctl_by_ll(),
    ",
  0x4035f0b0u64 => "
      I_3_C.scstlctl(),
      I_3_C.scstlctl_ha_l(),
      I_3_C.scstlctl_by_ll(),
    ",
  0x4035f0b1u64 => "
      I_3_C.scstlctl_by_lh(),
    ",
  0x4035f0b2u64 => "
      I_3_C.scstlctl_ha_h(),
    ",
  0x4035f0b3u64 => "
      I_3_C.scstlctl_by_hh(),
    ",
  0x4035f0c0u64 => "
      I_3_C.svtdlg0(),
    ",
  0x4035f0c2u64 => "
      I_3_C.svtdlg0_ha_h(),
      I_3_C.svtdlg0_by_hl(),
    ",
  0x4035f0c3u64 => "
      I_3_C.svtdlg0_by_hh(),
    ",
  0x4035f120u64 => "
      I_3_C.stctl(),
      I_3_C.stctl_ha_l(),
      I_3_C.stctl_by_ll(),
    ",
  0x4035f124u64 => "
      I_3_C.atctl(),
      I_3_C.atctl_ha_l(),
      I_3_C.atctl_by_ll(),
    ",
  0x4035f125u64 => "
      I_3_C.atctl_by_lh(),
    ",
  0x4035f128u64 => "
      I_3_C.attrg(),
      I_3_C.attrg_ha_l(),
      I_3_C.attrg_by_ll(),
    ",
  0x4035f12cu64 => "
      I_3_C.atccnte(),
      I_3_C.atccnte_ha_l(),
      I_3_C.atccnte_by_ll(),
    ",
  0x4035f140u64 => "
      I_3_C.cndctl(),
      I_3_C.cndctl_ha_l(),
      I_3_C.cndctl_by_ll(),
    ",
  0x4035f150u64 => "
      I_3_C.ncmdqp(),
    ",
  0x4035f154u64 => "
      I_3_C.nrspqp(),
    ",
  0x4035f158u64 => "
      I_3_C.ntdtbp0(),
      I_3_C.ntdtbp0_by_ll(),
    ",
  0x4035f17cu64 => "
      I_3_C.nibiqp(),
    ",
  0x4035f180u64 => "
      I_3_C.nrsqp(),
    ",
  0x4035f184u64 => "
      I_3_C.hcmdqp(),
    ",
  0x4035f188u64 => "
      I_3_C.hrspqp(),
    ",
  0x4035f18cu64 => "
      I_3_C.htdtbp(),
    ",
  0x4035f190u64 => "
      I_3_C.nqthctl(),
      I_3_C.nqthctl_ha_l(),
      I_3_C.nqthctl_by_ll(),
    ",
  0x4035f191u64 => "
      I_3_C.nqthctl_by_lh(),
    ",
  0x4035f192u64 => "
      I_3_C.nqthctl_ha_h(),
      I_3_C.nqthctl_by_hl(),
    ",
  0x4035f193u64 => "
      I_3_C.nqthctl_by_hh(),
    ",
  0x4035f194u64 => "
      I_3_C.ntbthctl0(),
      I_3_C.ntbthctl0_ha_l(),
      I_3_C.ntbthctl0_by_ll(),
    ",
  0x4035f195u64 => "
      I_3_C.ntbthctl0_by_lh(),
    ",
  0x4035f196u64 => "
      I_3_C.ntbthctl0_ha_h(),
      I_3_C.ntbthctl0_by_hl(),
    ",
  0x4035f197u64 => "
      I_3_C.ntbthctl0_by_hh(),
    ",
  0x4035f1c0u64 => "
      I_3_C.nrqthctl(),
      I_3_C.nrqthctl_ha_l(),
      I_3_C.nrqthctl_by_ll(),
    ",
  0x4035f1c4u64 => "
      I_3_C.hqthctl(),
      I_3_C.hqthctl_ha_l(),
      I_3_C.hqthctl_by_ll(),
    ",
  0x4035f1c5u64 => "
      I_3_C.hqthctl_by_lh(),
    ",
  0x4035f1c8u64 => "
      I_3_C.htbthctl(),
      I_3_C.htbthctl_ha_l(),
      I_3_C.htbthctl_by_ll(),
    ",
  0x4035f1c9u64 => "
      I_3_C.htbthctl_by_lh(),
    ",
  0x4035f1cau64 => "
      I_3_C.htbthctl_ha_h(),
      I_3_C.htbthctl_by_hl(),
    ",
  0x4035f1cbu64 => "
      I_3_C.htbthctl_by_hh(),
    ",
  0x4035f1d0u64 => "
      I_3_C.bst(),
      I_3_C.bst_ha_l(),
      I_3_C.bst_by_ll(),
    ",
  0x4035f1d1u64 => "
      I_3_C.bst_by_lh(),
    ",
  0x4035f1d2u64 => "
      I_3_C.bst_ha_h(),
      I_3_C.bst_by_hl(),
    ",
  0x4035f1d3u64 => "
      I_3_C.bst_by_hh(),
    ",
  0x4035f1d4u64 => "
      I_3_C.bste(),
      I_3_C.bste_ha_l(),
      I_3_C.bste_by_ll(),
    ",
  0x4035f1d5u64 => "
      I_3_C.bste_by_lh(),
    ",
  0x4035f1d6u64 => "
      I_3_C.bste_ha_h(),
      I_3_C.bste_by_hl(),
    ",
  0x4035f1d7u64 => "
      I_3_C.bste_by_hh(),
    ",
  0x4035f1d8u64 => "
      I_3_C.bie(),
      I_3_C.bie_ha_l(),
      I_3_C.bie_by_ll(),
    ",
  0x4035f1d9u64 => "
      I_3_C.bie_by_lh(),
    ",
  0x4035f1dau64 => "
      I_3_C.bie_ha_h(),
      I_3_C.bie_by_hl(),
    ",
  0x4035f1dbu64 => "
      I_3_C.bie_by_hh(),
    ",
  0x4035f1dcu64 => "
      I_3_C.bstfc(),
      I_3_C.bstfc_ha_l(),
      I_3_C.bstfc_by_ll(),
    ",
  0x4035f1ddu64 => "
      I_3_C.bstfc_by_lh(),
    ",
  0x4035f1deu64 => "
      I_3_C.bstfc_ha_h(),
      I_3_C.bstfc_by_hl(),
    ",
  0x4035f1dfu64 => "
      I_3_C.bstfc_by_hh(),
    ",
  0x4035f1e0u64 => "
      I_3_C.ntst(),
      I_3_C.ntst_ha_l(),
      I_3_C.ntst_by_ll(),
    ",
  0x4035f1e1u64 => "
      I_3_C.ntst_by_lh(),
    ",
  0x4035f1e2u64 => "
      I_3_C.ntst_ha_h(),
      I_3_C.ntst_by_hl(),
    ",
  0x4035f1e4u64 => "
      I_3_C.ntste(),
      I_3_C.ntste_ha_l(),
      I_3_C.ntste_by_ll(),
    ",
  0x4035f1e5u64 => "
      I_3_C.ntste_by_lh(),
    ",
  0x4035f1e6u64 => "
      I_3_C.ntste_ha_h(),
      I_3_C.ntste_by_hl(),
    ",
  0x4035f1e8u64 => "
      I_3_C.ntie(),
      I_3_C.ntie_ha_l(),
      I_3_C.ntie_by_ll(),
    ",
  0x4035f1e9u64 => "
      I_3_C.ntie_by_lh(),
    ",
  0x4035f1eau64 => "
      I_3_C.ntie_ha_h(),
      I_3_C.ntie_by_hl(),
    ",
  0x4035f1ecu64 => "
      I_3_C.ntstfc(),
      I_3_C.ntstfc_ha_l(),
      I_3_C.ntstfc_by_ll(),
    ",
  0x4035f1edu64 => "
      I_3_C.ntstfc_by_lh(),
    ",
  0x4035f1eeu64 => "
      I_3_C.ntstfc_ha_h(),
      I_3_C.ntstfc_by_hl(),
    ",
  0x4035f200u64 => "
      I_3_C.htst(),
      I_3_C.htst_ha_l(),
      I_3_C.htst_by_ll(),
    ",
  0x4035f201u64 => "
      I_3_C.htst_by_lh(),
    ",
  0x4035f204u64 => "
      I_3_C.htste(),
      I_3_C.htste_ha_l(),
      I_3_C.htste_by_ll(),
    ",
  0x4035f205u64 => "
      I_3_C.htste_by_lh(),
    ",
  0x4035f208u64 => "
      I_3_C.htie(),
      I_3_C.htie_ha_l(),
      I_3_C.htie_by_ll(),
    ",
  0x4035f209u64 => "
      I_3_C.htie_by_lh(),
    ",
  0x4035f20cu64 => "
      I_3_C.htstfc(),
      I_3_C.htstfc_ha_l(),
      I_3_C.htstfc_by_ll(),
    ",
  0x4035f20du64 => "
      I_3_C.htstfc_by_lh(),
    ",
  0x4035f210u64 => "
      I_3_C.bcst(),
      I_3_C.bcst_ha_l(),
      I_3_C.bcst_by_ll(),
    ",
  0x4035f214u64 => "
      I_3_C.svst(),
      I_3_C.svst_ha_l(),
      I_3_C.svst_by_ll(),
    ",
  0x4035f215u64 => "
      I_3_C.svst_by_lh(),
    ",
  0x4035f216u64 => "
      I_3_C.svst_ha_h(),
      I_3_C.svst_by_hl(),
    ",
  0x4035f218u64 => "
      I_3_C.wust(),
      I_3_C.wust_ha_l(),
      I_3_C.wust_by_ll(),
    ",
  0x4035f21cu64 => "
      I_3_C.mrccpt(),
    ",
  0x4035f224u64 => "
      I_3_C.datbas0(),
      I_3_C.datbas0_ha_l(),
      I_3_C.datbas0_by_ll(),
    ",
  0x4035f225u64 => "
      I_3_C.datbas0_by_lh(),
    ",
  0x4035f226u64 => "
      I_3_C.datbas0_ha_h(),
      I_3_C.datbas0_by_hl(),
    ",
  0x4035f227u64 => "
      I_3_C.datbas0_by_hh(),
    ",
  0x4035f22cu64 => "
      I_3_C.datbas1(),
      I_3_C.datbas1_ha_l(),
      I_3_C.datbas1_by_ll(),
    ",
  0x4035f22du64 => "
      I_3_C.datbas1_by_lh(),
    ",
  0x4035f22eu64 => "
      I_3_C.datbas1_ha_h(),
      I_3_C.datbas1_by_hl(),
    ",
  0x4035f22fu64 => "
      I_3_C.datbas1_by_hh(),
    ",
  0x4035f234u64 => "
      I_3_C.datbas2(),
      I_3_C.datbas2_ha_l(),
      I_3_C.datbas2_by_ll(),
    ",
  0x4035f235u64 => "
      I_3_C.datbas2_by_lh(),
    ",
  0x4035f236u64 => "
      I_3_C.datbas2_ha_h(),
      I_3_C.datbas2_by_hl(),
    ",
  0x4035f237u64 => "
      I_3_C.datbas2_by_hh(),
    ",
  0x4035f23cu64 => "
      I_3_C.datbas3(),
      I_3_C.datbas3_ha_l(),
      I_3_C.datbas3_by_ll(),
    ",
  0x4035f23du64 => "
      I_3_C.datbas3_by_lh(),
    ",
  0x4035f23eu64 => "
      I_3_C.datbas3_ha_h(),
      I_3_C.datbas3_by_hl(),
    ",
  0x4035f23fu64 => "
      I_3_C.datbas3_by_hh(),
    ",
  0x4035f244u64 => "
      I_3_C.datbas4(),
      I_3_C.datbas4_ha_l(),
      I_3_C.datbas4_by_ll(),
    ",
  0x4035f245u64 => "
      I_3_C.datbas4_by_lh(),
    ",
  0x4035f246u64 => "
      I_3_C.datbas4_ha_h(),
      I_3_C.datbas4_by_hl(),
    ",
  0x4035f247u64 => "
      I_3_C.datbas4_by_hh(),
    ",
  0x4035f24cu64 => "
      I_3_C.datbas5(),
      I_3_C.datbas5_ha_l(),
      I_3_C.datbas5_by_ll(),
    ",
  0x4035f24du64 => "
      I_3_C.datbas5_by_lh(),
    ",
  0x4035f24eu64 => "
      I_3_C.datbas5_ha_h(),
      I_3_C.datbas5_by_hl(),
    ",
  0x4035f24fu64 => "
      I_3_C.datbas5_by_hh(),
    ",
  0x4035f254u64 => "
      I_3_C.datbas6(),
      I_3_C.datbas6_ha_l(),
      I_3_C.datbas6_by_ll(),
    ",
  0x4035f255u64 => "
      I_3_C.datbas6_by_lh(),
    ",
  0x4035f256u64 => "
      I_3_C.datbas6_ha_h(),
      I_3_C.datbas6_by_hl(),
    ",
  0x4035f257u64 => "
      I_3_C.datbas6_by_hh(),
    ",
  0x4035f25cu64 => "
      I_3_C.datbas7(),
      I_3_C.datbas7_ha_l(),
      I_3_C.datbas7_by_ll(),
    ",
  0x4035f25du64 => "
      I_3_C.datbas7_by_lh(),
    ",
  0x4035f25eu64 => "
      I_3_C.datbas7_ha_h(),
      I_3_C.datbas7_by_hl(),
    ",
  0x4035f25fu64 => "
      I_3_C.datbas7_by_hh(),
    ",
  0x4035f2a0u64 => "
      I_3_C.exdatbas(),
      I_3_C.exdatbas_ha_l(),
      I_3_C.exdatbas_by_ll(),
    ",
  0x4035f2a1u64 => "
      I_3_C.exdatbas_by_lh(),
    ",
  0x4035f2a2u64 => "
      I_3_C.exdatbas_ha_h(),
      I_3_C.exdatbas_by_hl(),
    ",
  0x4035f2a3u64 => "
      I_3_C.exdatbas_by_hh(),
    ",
  0x4035f2b0u64 => "
      I_3_C.sdatbas0(),
      I_3_C.sdatbas0_ha_l(),
      I_3_C.sdatbas0_by_ll(),
    ",
  0x4035f2b1u64 => "
      I_3_C.sdatbas0_by_lh(),
    ",
  0x4035f2b2u64 => "
      I_3_C.sdatbas0_ha_h(),
      I_3_C.sdatbas0_by_hl(),
    ",
  0x4035f2b4u64 => "
      I_3_C.sdatbas1(),
      I_3_C.sdatbas1_ha_l(),
      I_3_C.sdatbas1_by_ll(),
    ",
  0x4035f2b5u64 => "
      I_3_C.sdatbas1_by_lh(),
    ",
  0x4035f2b8u64 => "
      I_3_C.sdatbas2(),
      I_3_C.sdatbas2_ha_l(),
      I_3_C.sdatbas2_by_ll(),
    ",
  0x4035f2b9u64 => "
      I_3_C.sdatbas2_by_lh(),
    ",
  0x4035f2d0u64 => "
      I_3_C.msdct0(),
      I_3_C.msdct0_ha_l(),
    ",
  0x4035f2d1u64 => "
      I_3_C.msdct0_by_lh(),
    ",
  0x4035f2d4u64 => "
      I_3_C.msdct1(),
      I_3_C.msdct1_ha_l(),
    ",
  0x4035f2d5u64 => "
      I_3_C.msdct1_by_lh(),
    ",
  0x4035f2d8u64 => "
      I_3_C.msdct2(),
      I_3_C.msdct2_ha_l(),
    ",
  0x4035f2d9u64 => "
      I_3_C.msdct2_by_lh(),
    ",
  0x4035f2dcu64 => "
      I_3_C.msdct3(),
      I_3_C.msdct3_ha_l(),
    ",
  0x4035f2ddu64 => "
      I_3_C.msdct3_by_lh(),
    ",
  0x4035f2e0u64 => "
      I_3_C.msdct4(),
      I_3_C.msdct4_ha_l(),
    ",
  0x4035f2e1u64 => "
      I_3_C.msdct4_by_lh(),
    ",
  0x4035f2e4u64 => "
      I_3_C.msdct5(),
      I_3_C.msdct5_ha_l(),
    ",
  0x4035f2e5u64 => "
      I_3_C.msdct5_by_lh(),
    ",
  0x4035f2e8u64 => "
      I_3_C.msdct6(),
      I_3_C.msdct6_ha_l(),
    ",
  0x4035f2e9u64 => "
      I_3_C.msdct6_by_lh(),
    ",
  0x4035f2ecu64 => "
      I_3_C.msdct7(),
      I_3_C.msdct7_ha_l(),
    ",
  0x4035f2edu64 => "
      I_3_C.msdct7_by_lh(),
    ",
  0x4035f320u64 => "
      I_3_C.svdct(),
      I_3_C.svdct_ha_l(),
      I_3_C.svdct_by_ll(),
    ",
  0x4035f321u64 => "
      I_3_C.svdct_by_lh(),
    ",
  0x4035f324u64 => "
      I_3_C.sdctpidl(),
      I_3_C.sdctpidl_ha_l(),
      I_3_C.sdctpidl_by_ll(),
    ",
  0x4035f325u64 => "
      I_3_C.sdctpidl_by_lh(),
    ",
  0x4035f328u64 => "
      I_3_C.sdctpidh(),
      I_3_C.sdctpidh_ha_l(),
      I_3_C.sdctpidh_by_ll(),
    ",
  0x4035f329u64 => "
      I_3_C.sdctpidh_by_lh(),
    ",
  0x4035f32au64 => "
      I_3_C.sdctpidh_ha_h(),
      I_3_C.sdctpidh_by_hl(),
    ",
  0x4035f32bu64 => "
      I_3_C.sdctpidh_by_hh(),
    ",
  0x4035f330u64 => "
      I_3_C.svdvad0(),
    ",
  0x4035f332u64 => "
      I_3_C.svdvad0_ha_h(),
      I_3_C.svdvad0_by_hl(),
    ",
  0x4035f333u64 => "
      I_3_C.svdvad0_by_hh(),
    ",
  0x4035f334u64 => "
      I_3_C.svdvad1(),
    ",
  0x4035f336u64 => "
      I_3_C.svdvad1_ha_h(),
      I_3_C.svdvad1_by_hl(),
    ",
  0x4035f337u64 => "
      I_3_C.svdvad1_by_hh(),
    ",
  0x4035f338u64 => "
      I_3_C.svdvad2(),
    ",
  0x4035f33au64 => "
      I_3_C.svdvad2_ha_h(),
      I_3_C.svdvad2_by_hl(),
    ",
  0x4035f33bu64 => "
      I_3_C.svdvad2_by_hh(),
    ",
  0x4035f350u64 => "
      I_3_C.csecmd(),
      I_3_C.csecmd_ha_l(),
      I_3_C.csecmd_by_ll(),
    ",
  0x4035f354u64 => "
      I_3_C.ceactst(),
      I_3_C.ceactst_ha_l(),
      I_3_C.ceactst_by_ll(),
    ",
  0x4035f358u64 => "
      I_3_C.cmwlg(),
      I_3_C.cmwlg_ha_l(),
      I_3_C.cmwlg_by_ll(),
    ",
  0x4035f359u64 => "
      I_3_C.cmwlg_by_lh(),
    ",
  0x4035f35cu64 => "
      I_3_C.cmrlg(),
      I_3_C.cmrlg_ha_l(),
      I_3_C.cmrlg_by_ll(),
    ",
  0x4035f35du64 => "
      I_3_C.cmrlg_by_lh(),
    ",
  0x4035f35eu64 => "
      I_3_C.cmrlg_ha_h(),
      I_3_C.cmrlg_by_hl(),
    ",
  0x4035f360u64 => "
      I_3_C.cetstmd(),
      I_3_C.cetstmd_ha_l(),
      I_3_C.cetstmd_by_ll(),
    ",
  0x4035f364u64 => "
      I_3_C.cgdvst(),
      I_3_C.cgdvst_ha_l(),
      I_3_C.cgdvst_by_ll(),
    ",
  0x4035f365u64 => "
      I_3_C.cgdvst_by_lh(),
    ",
  0x4035f368u64 => "
      I_3_C.cmdspw(),
      I_3_C.cmdspw_ha_l(),
      I_3_C.cmdspw_by_ll(),
    ",
  0x4035f36cu64 => "
      I_3_C.cmdspr(),
      I_3_C.cmdspr_ha_l(),
      I_3_C.cmdspr_by_ll(),
    ",
  0x4035f370u64 => "
      I_3_C.cmdspt(),
      I_3_C.cmdspt_ha_l(),
      I_3_C.cmdspt_by_ll(),
    ",
  0x4035f371u64 => "
      I_3_C.cmdspt_by_lh(),
    ",
  0x4035f372u64 => "
      I_3_C.cmdspt_ha_h(),
      I_3_C.cmdspt_by_hl(),
    ",
  0x4035f373u64 => "
      I_3_C.cmdspt_by_hh(),
    ",
  0x4035f374u64 => "
      I_3_C.cetsm(),
      I_3_C.cetsm_ha_l(),
      I_3_C.cetsm_by_ll(),
    ",
  0x4035f375u64 => "
      I_3_C.cetsm_by_lh(),
    ",
  0x4035f376u64 => "
      I_3_C.cetsm_ha_h(),
      I_3_C.cetsm_by_hl(),
    ",
  0x4035f378u64 => "
      I_3_C.cetss(),
      I_3_C.cetss_ha_l(),
      I_3_C.cetss_by_ll(),
    ",
  0x4035f37cu64 => "
      I_3_C.cghdrcap(),
      I_3_C.cghdrcap_ha_l(),
      I_3_C.cghdrcap_by_ll(),
    ",
  0x4035f380u64 => "
      I_3_C.bitcnt(),
      I_3_C.bitcnt_ha_l(),
      I_3_C.bitcnt_by_ll(),
    ",
  0x4035f394u64 => "
      I_3_C.nqstlv(),
      I_3_C.nqstlv_ha_l(),
      I_3_C.nqstlv_by_ll(),
    ",
  0x4035f395u64 => "
      I_3_C.nqstlv_by_lh(),
    ",
  0x4035f396u64 => "
      I_3_C.nqstlv_ha_h(),
      I_3_C.nqstlv_by_hl(),
    ",
  0x4035f397u64 => "
      I_3_C.nqstlv_by_hh(),
    ",
  0x4035f398u64 => "
      I_3_C.ndbstlv0(),
      I_3_C.ndbstlv0_ha_l(),
      I_3_C.ndbstlv0_by_ll(),
    ",
  0x4035f399u64 => "
      I_3_C.ndbstlv0_by_lh(),
    ",
  0x4035f3c0u64 => "
      I_3_C.nrsqstlv(),
      I_3_C.nrsqstlv_ha_l(),
      I_3_C.nrsqstlv_by_ll(),
    ",
  0x4035f3c4u64 => "
      I_3_C.hqstlv(),
      I_3_C.hqstlv_ha_l(),
      I_3_C.hqstlv_by_ll(),
    ",
  0x4035f3c5u64 => "
      I_3_C.hqstlv_by_lh(),
    ",
  0x4035f3c8u64 => "
      I_3_C.hdbstlv(),
      I_3_C.hdbstlv_ha_l(),
      I_3_C.hdbstlv_by_ll(),
    ",
  0x4035f3c9u64 => "
      I_3_C.hdbstlv_by_lh(),
    ",
  0x4035f3ccu64 => "
      I_3_C.prstdbg(),
      I_3_C.prstdbg_ha_l(),
      I_3_C.prstdbg_by_ll(),
    ",
  0x4035f3cdu64 => "
      I_3_C.prstdbg_by_lh(),
    ",
  0x4035f3ceu64 => "
      I_3_C.prstdbg_ha_h(),
      I_3_C.prstdbg_by_hl(),
    ",
  0x4035f3cfu64 => "
      I_3_C.prstdbg_by_hh(),
    ",
  0x4035f3d0u64 => "
      I_3_C.mserrcnt(),
      I_3_C.mserrcnt_ha_l(),
      I_3_C.mserrcnt_by_ll(),
    ",
  0x4035f3e0u64 => "
      I_3_C.sc1cpt(),
      I_3_C.sc1cpt_ha_l(),
      I_3_C.sc1cpt_by_ll(),
    ",
  0x4035f3e1u64 => "
      I_3_C.sc1cpt_by_lh(),
    ",
  0x4035f3e4u64 => "
      I_3_C.sc2cpt(),
      I_3_C.sc2cpt_ha_l(),
      I_3_C.sc2cpt_by_ll(),
    ",
  0x4035f3e5u64 => "
      I_3_C.sc2cpt_by_lh(),
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
  0x40380010u64 => "
      CANFD_0.cfdgipv(),
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
  0x403800bcu64 => "
      CANFD_0.cfdglotb(),
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
  0x403800d0u64 => "
      CANFD_0.cfdgpflectr(),
    ",
  0x403800d4u64 => "
      CANFD_0.cfdgpflcfg(),
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
  0x40380220u64 => "
      CANFD_0.cfdgpflid()[0],
    ",
  0x40380244u64 => "
      CANFD_0.cfdgpflid()[1],
    ",
  0x40380224u64 => "
      CANFD_0.cfdgpflm()[0],
    ",
  0x40380248u64 => "
      CANFD_0.cfdgpflm()[1],
    ",
  0x40380228u64 => "
      CANFD_0.cfdgpflp0()[0],
    ",
  0x4038024cu64 => "
      CANFD_0.cfdgpflp0()[1],
    ",
  0x4038022cu64 => "
      CANFD_0.cfdgpflp1()[0],
    ",
  0x40380250u64 => "
      CANFD_0.cfdgpflp1()[1],
    ",
  0x40380230u64 => "
      CANFD_0.cfdgpflpt()[0],
    ",
  0x40380254u64 => "
      CANFD_0.cfdgpflpt()[1],
    ",
  0x40380234u64 => "
      CANFD_0.cfdgpflpd0()[0],
    ",
  0x40380258u64 => "
      CANFD_0.cfdgpflpd0()[1],
    ",
  0x40380238u64 => "
      CANFD_0.cfdgpflpm0()[0],
    ",
  0x4038025cu64 => "
      CANFD_0.cfdgpflpm0()[1],
    ",
  0x4038023cu64 => "
      CANFD_0.cfdgpflpd1()[0],
    ",
  0x40380260u64 => "
      CANFD_0.cfdgpflpd1()[1],
    ",
  0x40380240u64 => "
      CANFD_0.cfdgpflpm1()[0],
    ",
  0x40380264u64 => "
      CANFD_0.cfdgpflpm1()[1],
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
      CANFD_0.cfdrfdf0()[0],
    ",
  0x40380530u64 => "
      CANFD_0.cfdrfdf0()[1],
    ",
  0x40380534u64 => "
      CANFD_0.cfdrfdf0()[2],
    ",
  0x40380538u64 => "
      CANFD_0.cfdrfdf0()[3],
    ",
  0x4038053cu64 => "
      CANFD_0.cfdrfdf0()[4],
    ",
  0x40380540u64 => "
      CANFD_0.cfdrfdf0()[5],
    ",
  0x40380544u64 => "
      CANFD_0.cfdrfdf0()[6],
    ",
  0x40380548u64 => "
      CANFD_0.cfdrfdf0()[7],
    ",
  0x4038054cu64 => "
      CANFD_0.cfdrfdf0()[8],
    ",
  0x40380550u64 => "
      CANFD_0.cfdrfdf0()[9],
    ",
  0x40380554u64 => "
      CANFD_0.cfdrfdf0()[10],
    ",
  0x40380558u64 => "
      CANFD_0.cfdrfdf0()[11],
    ",
  0x4038055cu64 => "
      CANFD_0.cfdrfdf0()[12],
    ",
  0x40380560u64 => "
      CANFD_0.cfdrfdf0()[13],
    ",
  0x40380564u64 => "
      CANFD_0.cfdrfdf0()[14],
    ",
  0x40380568u64 => "
      CANFD_0.cfdrfdf0()[15],
    ",
  0x40380578u64 => "
      CANFD_0.cfdrfdf1()[0],
    ",
  0x4038057cu64 => "
      CANFD_0.cfdrfdf1()[1],
    ",
  0x40380580u64 => "
      CANFD_0.cfdrfdf1()[2],
    ",
  0x40380584u64 => "
      CANFD_0.cfdrfdf1()[3],
    ",
  0x40380588u64 => "
      CANFD_0.cfdrfdf1()[4],
    ",
  0x4038058cu64 => "
      CANFD_0.cfdrfdf1()[5],
    ",
  0x40380590u64 => "
      CANFD_0.cfdrfdf1()[6],
    ",
  0x40380594u64 => "
      CANFD_0.cfdrfdf1()[7],
    ",
  0x40380598u64 => "
      CANFD_0.cfdrfdf1()[8],
    ",
  0x4038059cu64 => "
      CANFD_0.cfdrfdf1()[9],
    ",
  0x403805a0u64 => "
      CANFD_0.cfdrfdf1()[10],
    ",
  0x403805a4u64 => "
      CANFD_0.cfdrfdf1()[11],
    ",
  0x403805a8u64 => "
      CANFD_0.cfdrfdf1()[12],
    ",
  0x403805acu64 => "
      CANFD_0.cfdrfdf1()[13],
    ",
  0x403805b0u64 => "
      CANFD_0.cfdrfdf1()[14],
    ",
  0x403805b4u64 => "
      CANFD_0.cfdrfdf1()[15],
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
      CANFD_0.cfdcfdf0(),
    ",
  0x403805c8u64 => "
      CANFD_0.cfdcfdf1(),
    ",
  0x403805ccu64 => "
      CANFD_0.cfdcfdf2(),
    ",
  0x403805d0u64 => "
      CANFD_0.cfdcfdf3(),
    ",
  0x403805d4u64 => "
      CANFD_0.cfdcfdf4(),
    ",
  0x403805d8u64 => "
      CANFD_0.cfdcfdf5(),
    ",
  0x403805dcu64 => "
      CANFD_0.cfdcfdf6(),
    ",
  0x403805e0u64 => "
      CANFD_0.cfdcfdf7(),
    ",
  0x403805e4u64 => "
      CANFD_0.cfdcfdf8(),
    ",
  0x403805e8u64 => "
      CANFD_0.cfdcfdf9(),
    ",
  0x403805ecu64 => "
      CANFD_0.cfdcfdf10(),
    ",
  0x403805f0u64 => "
      CANFD_0.cfdcfdf11(),
    ",
  0x403805f4u64 => "
      CANFD_0.cfdcfdf12(),
    ",
  0x403805f8u64 => "
      CANFD_0.cfdcfdf13(),
    ",
  0x403805fcu64 => "
      CANFD_0.cfdcfdf14(),
    ",
  0x40380600u64 => "
      CANFD_0.cfdcfdf15(),
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
      CANFD_0.cfdtmdf0_()[0],
    ",
  0x40380614u64 => "
      CANFD_0.cfdtmdf0_()[1],
    ",
  0x40380618u64 => "
      CANFD_0.cfdtmdf0_()[2],
    ",
  0x4038061cu64 => "
      CANFD_0.cfdtmdf0_()[3],
    ",
  0x40380620u64 => "
      CANFD_0.cfdtmdf0_()[4],
    ",
  0x40380624u64 => "
      CANFD_0.cfdtmdf0_()[5],
    ",
  0x40380628u64 => "
      CANFD_0.cfdtmdf0_()[6],
    ",
  0x4038062cu64 => "
      CANFD_0.cfdtmdf0_()[7],
    ",
  0x40380630u64 => "
      CANFD_0.cfdtmdf0_()[8],
    ",
  0x40380634u64 => "
      CANFD_0.cfdtmdf0_()[9],
    ",
  0x40380638u64 => "
      CANFD_0.cfdtmdf0_()[10],
    ",
  0x4038063cu64 => "
      CANFD_0.cfdtmdf0_()[11],
    ",
  0x40380640u64 => "
      CANFD_0.cfdtmdf0_()[12],
    ",
  0x40380644u64 => "
      CANFD_0.cfdtmdf0_()[13],
    ",
  0x40380648u64 => "
      CANFD_0.cfdtmdf0_()[14],
    ",
  0x4038064cu64 => "
      CANFD_0.cfdtmdf0_()[15],
    ",
  0x4038065cu64 => "
      CANFD_0.cfdtmdf1_()[0],
    ",
  0x40380660u64 => "
      CANFD_0.cfdtmdf1_()[1],
    ",
  0x40380664u64 => "
      CANFD_0.cfdtmdf1_()[2],
    ",
  0x40380668u64 => "
      CANFD_0.cfdtmdf1_()[3],
    ",
  0x4038066cu64 => "
      CANFD_0.cfdtmdf1_()[4],
    ",
  0x40380670u64 => "
      CANFD_0.cfdtmdf1_()[5],
    ",
  0x40380674u64 => "
      CANFD_0.cfdtmdf1_()[6],
    ",
  0x40380678u64 => "
      CANFD_0.cfdtmdf1_()[7],
    ",
  0x4038067cu64 => "
      CANFD_0.cfdtmdf1_()[8],
    ",
  0x40380680u64 => "
      CANFD_0.cfdtmdf1_()[9],
    ",
  0x40380684u64 => "
      CANFD_0.cfdtmdf1_()[10],
    ",
  0x40380688u64 => "
      CANFD_0.cfdtmdf1_()[11],
    ",
  0x4038068cu64 => "
      CANFD_0.cfdtmdf1_()[12],
    ",
  0x40380690u64 => "
      CANFD_0.cfdtmdf1_()[13],
    ",
  0x40380694u64 => "
      CANFD_0.cfdtmdf1_()[14],
    ",
  0x40380698u64 => "
      CANFD_0.cfdtmdf1_()[15],
    ",
  0x403806a8u64 => "
      CANFD_0.cfdtmdf2_()[0],
    ",
  0x403806acu64 => "
      CANFD_0.cfdtmdf2_()[1],
    ",
  0x403806b0u64 => "
      CANFD_0.cfdtmdf2_()[2],
    ",
  0x403806b4u64 => "
      CANFD_0.cfdtmdf2_()[3],
    ",
  0x403806b8u64 => "
      CANFD_0.cfdtmdf2_()[4],
    ",
  0x403806bcu64 => "
      CANFD_0.cfdtmdf2_()[5],
    ",
  0x403806c0u64 => "
      CANFD_0.cfdtmdf2_()[6],
    ",
  0x403806c4u64 => "
      CANFD_0.cfdtmdf2_()[7],
    ",
  0x403806c8u64 => "
      CANFD_0.cfdtmdf2_()[8],
    ",
  0x403806ccu64 => "
      CANFD_0.cfdtmdf2_()[9],
    ",
  0x403806d0u64 => "
      CANFD_0.cfdtmdf2_()[10],
    ",
  0x403806d4u64 => "
      CANFD_0.cfdtmdf2_()[11],
    ",
  0x403806d8u64 => "
      CANFD_0.cfdtmdf2_()[12],
    ",
  0x403806dcu64 => "
      CANFD_0.cfdtmdf2_()[13],
    ",
  0x403806e0u64 => "
      CANFD_0.cfdtmdf2_()[14],
    ",
  0x403806e4u64 => "
      CANFD_0.cfdtmdf2_()[15],
    ",
  0x403806f4u64 => "
      CANFD_0.cfdtmdf3_()[0],
    ",
  0x403806f8u64 => "
      CANFD_0.cfdtmdf3_()[1],
    ",
  0x403806fcu64 => "
      CANFD_0.cfdtmdf3_()[2],
    ",
  0x40380700u64 => "
      CANFD_0.cfdtmdf3_()[3],
    ",
  0x40380704u64 => "
      CANFD_0.cfdtmdf3_()[4],
    ",
  0x40380708u64 => "
      CANFD_0.cfdtmdf3_()[5],
    ",
  0x4038070cu64 => "
      CANFD_0.cfdtmdf3_()[6],
    ",
  0x40380710u64 => "
      CANFD_0.cfdtmdf3_()[7],
    ",
  0x40380714u64 => "
      CANFD_0.cfdtmdf3_()[8],
    ",
  0x40380718u64 => "
      CANFD_0.cfdtmdf3_()[9],
    ",
  0x4038071cu64 => "
      CANFD_0.cfdtmdf3_()[10],
    ",
  0x40380720u64 => "
      CANFD_0.cfdtmdf3_()[11],
    ",
  0x40380724u64 => "
      CANFD_0.cfdtmdf3_()[12],
    ",
  0x40380728u64 => "
      CANFD_0.cfdtmdf3_()[13],
    ",
  0x4038072cu64 => "
      CANFD_0.cfdtmdf3_()[14],
    ",
  0x40380730u64 => "
      CANFD_0.cfdtmdf3_()[15],
    ",
  0x40380740u64 => "
      CANFD_0.cfdthlacc0(),
    ",
  0x40380744u64 => "
      CANFD_0.cfdthlacc1(),
    ",
  0x4038092cu64 => "
      CANFD_0.cfdrmdf0_()[0],
    ",
  0x40380930u64 => "
      CANFD_0.cfdrmdf0_()[1],
    ",
  0x40380934u64 => "
      CANFD_0.cfdrmdf0_()[2],
    ",
  0x40380938u64 => "
      CANFD_0.cfdrmdf0_()[3],
    ",
  0x4038093cu64 => "
      CANFD_0.cfdrmdf0_()[4],
    ",
  0x40380940u64 => "
      CANFD_0.cfdrmdf0_()[5],
    ",
  0x40380944u64 => "
      CANFD_0.cfdrmdf0_()[6],
    ",
  0x40380948u64 => "
      CANFD_0.cfdrmdf0_()[7],
    ",
  0x4038094cu64 => "
      CANFD_0.cfdrmdf0_()[8],
    ",
  0x40380950u64 => "
      CANFD_0.cfdrmdf0_()[9],
    ",
  0x40380954u64 => "
      CANFD_0.cfdrmdf0_()[10],
    ",
  0x40380958u64 => "
      CANFD_0.cfdrmdf0_()[11],
    ",
  0x4038095cu64 => "
      CANFD_0.cfdrmdf0_()[12],
    ",
  0x40380960u64 => "
      CANFD_0.cfdrmdf0_()[13],
    ",
  0x40380964u64 => "
      CANFD_0.cfdrmdf0_()[14],
    ",
  0x40380968u64 => "
      CANFD_0.cfdrmdf0_()[15],
    ",
  0x40380978u64 => "
      CANFD_0.cfdrmdf1_()[0],
    ",
  0x4038097cu64 => "
      CANFD_0.cfdrmdf1_()[1],
    ",
  0x40380980u64 => "
      CANFD_0.cfdrmdf1_()[2],
    ",
  0x40380984u64 => "
      CANFD_0.cfdrmdf1_()[3],
    ",
  0x40380988u64 => "
      CANFD_0.cfdrmdf1_()[4],
    ",
  0x4038098cu64 => "
      CANFD_0.cfdrmdf1_()[5],
    ",
  0x40380990u64 => "
      CANFD_0.cfdrmdf1_()[6],
    ",
  0x40380994u64 => "
      CANFD_0.cfdrmdf1_()[7],
    ",
  0x40380998u64 => "
      CANFD_0.cfdrmdf1_()[8],
    ",
  0x4038099cu64 => "
      CANFD_0.cfdrmdf1_()[9],
    ",
  0x403809a0u64 => "
      CANFD_0.cfdrmdf1_()[10],
    ",
  0x403809a4u64 => "
      CANFD_0.cfdrmdf1_()[11],
    ",
  0x403809a8u64 => "
      CANFD_0.cfdrmdf1_()[12],
    ",
  0x403809acu64 => "
      CANFD_0.cfdrmdf1_()[13],
    ",
  0x403809b0u64 => "
      CANFD_0.cfdrmdf1_()[14],
    ",
  0x403809b4u64 => "
      CANFD_0.cfdrmdf1_()[15],
    ",
  0x403809c4u64 => "
      CANFD_0.cfdrmdf2_()[0],
    ",
  0x403809c8u64 => "
      CANFD_0.cfdrmdf2_()[1],
    ",
  0x403809ccu64 => "
      CANFD_0.cfdrmdf2_()[2],
    ",
  0x403809d0u64 => "
      CANFD_0.cfdrmdf2_()[3],
    ",
  0x403809d4u64 => "
      CANFD_0.cfdrmdf2_()[4],
    ",
  0x403809d8u64 => "
      CANFD_0.cfdrmdf2_()[5],
    ",
  0x403809dcu64 => "
      CANFD_0.cfdrmdf2_()[6],
    ",
  0x403809e0u64 => "
      CANFD_0.cfdrmdf2_()[7],
    ",
  0x403809e4u64 => "
      CANFD_0.cfdrmdf2_()[8],
    ",
  0x403809e8u64 => "
      CANFD_0.cfdrmdf2_()[9],
    ",
  0x403809ecu64 => "
      CANFD_0.cfdrmdf2_()[10],
    ",
  0x403809f0u64 => "
      CANFD_0.cfdrmdf2_()[11],
    ",
  0x403809f4u64 => "
      CANFD_0.cfdrmdf2_()[12],
    ",
  0x403809f8u64 => "
      CANFD_0.cfdrmdf2_()[13],
    ",
  0x403809fcu64 => "
      CANFD_0.cfdrmdf2_()[14],
    ",
  0x40380a00u64 => "
      CANFD_0.cfdrmdf2_()[15],
    ",
  0x40380a10u64 => "
      CANFD_0.cfdrmdf3_()[0],
    ",
  0x40380a14u64 => "
      CANFD_0.cfdrmdf3_()[1],
    ",
  0x40380a18u64 => "
      CANFD_0.cfdrmdf3_()[2],
    ",
  0x40380a1cu64 => "
      CANFD_0.cfdrmdf3_()[3],
    ",
  0x40380a20u64 => "
      CANFD_0.cfdrmdf3_()[4],
    ",
  0x40380a24u64 => "
      CANFD_0.cfdrmdf3_()[5],
    ",
  0x40380a28u64 => "
      CANFD_0.cfdrmdf3_()[6],
    ",
  0x40380a2cu64 => "
      CANFD_0.cfdrmdf3_()[7],
    ",
  0x40380a30u64 => "
      CANFD_0.cfdrmdf3_()[8],
    ",
  0x40380a34u64 => "
      CANFD_0.cfdrmdf3_()[9],
    ",
  0x40380a38u64 => "
      CANFD_0.cfdrmdf3_()[10],
    ",
  0x40380a3cu64 => "
      CANFD_0.cfdrmdf3_()[11],
    ",
  0x40380a40u64 => "
      CANFD_0.cfdrmdf3_()[12],
    ",
  0x40380a44u64 => "
      CANFD_0.cfdrmdf3_()[13],
    ",
  0x40380a48u64 => "
      CANFD_0.cfdrmdf3_()[14],
    ",
  0x40380a4cu64 => "
      CANFD_0.cfdrmdf3_()[15],
    ",
  0x40380a5cu64 => "
      CANFD_0.cfdrmdf4_()[0],
    ",
  0x40380a60u64 => "
      CANFD_0.cfdrmdf4_()[1],
    ",
  0x40380a64u64 => "
      CANFD_0.cfdrmdf4_()[2],
    ",
  0x40380a68u64 => "
      CANFD_0.cfdrmdf4_()[3],
    ",
  0x40380a6cu64 => "
      CANFD_0.cfdrmdf4_()[4],
    ",
  0x40380a70u64 => "
      CANFD_0.cfdrmdf4_()[5],
    ",
  0x40380a74u64 => "
      CANFD_0.cfdrmdf4_()[6],
    ",
  0x40380a78u64 => "
      CANFD_0.cfdrmdf4_()[7],
    ",
  0x40380a7cu64 => "
      CANFD_0.cfdrmdf4_()[8],
    ",
  0x40380a80u64 => "
      CANFD_0.cfdrmdf4_()[9],
    ",
  0x40380a84u64 => "
      CANFD_0.cfdrmdf4_()[10],
    ",
  0x40380a88u64 => "
      CANFD_0.cfdrmdf4_()[11],
    ",
  0x40380a8cu64 => "
      CANFD_0.cfdrmdf4_()[12],
    ",
  0x40380a90u64 => "
      CANFD_0.cfdrmdf4_()[13],
    ",
  0x40380a94u64 => "
      CANFD_0.cfdrmdf4_()[14],
    ",
  0x40380a98u64 => "
      CANFD_0.cfdrmdf4_()[15],
    ",
  0x40380aa8u64 => "
      CANFD_0.cfdrmdf5_()[0],
    ",
  0x40380aacu64 => "
      CANFD_0.cfdrmdf5_()[1],
    ",
  0x40380ab0u64 => "
      CANFD_0.cfdrmdf5_()[2],
    ",
  0x40380ab4u64 => "
      CANFD_0.cfdrmdf5_()[3],
    ",
  0x40380ab8u64 => "
      CANFD_0.cfdrmdf5_()[4],
    ",
  0x40380abcu64 => "
      CANFD_0.cfdrmdf5_()[5],
    ",
  0x40380ac0u64 => "
      CANFD_0.cfdrmdf5_()[6],
    ",
  0x40380ac4u64 => "
      CANFD_0.cfdrmdf5_()[7],
    ",
  0x40380ac8u64 => "
      CANFD_0.cfdrmdf5_()[8],
    ",
  0x40380accu64 => "
      CANFD_0.cfdrmdf5_()[9],
    ",
  0x40380ad0u64 => "
      CANFD_0.cfdrmdf5_()[10],
    ",
  0x40380ad4u64 => "
      CANFD_0.cfdrmdf5_()[11],
    ",
  0x40380ad8u64 => "
      CANFD_0.cfdrmdf5_()[12],
    ",
  0x40380adcu64 => "
      CANFD_0.cfdrmdf5_()[13],
    ",
  0x40380ae0u64 => "
      CANFD_0.cfdrmdf5_()[14],
    ",
  0x40380ae4u64 => "
      CANFD_0.cfdrmdf5_()[15],
    ",
  0x40380af4u64 => "
      CANFD_0.cfdrmdf6_()[0],
    ",
  0x40380af8u64 => "
      CANFD_0.cfdrmdf6_()[1],
    ",
  0x40380afcu64 => "
      CANFD_0.cfdrmdf6_()[2],
    ",
  0x40380b00u64 => "
      CANFD_0.cfdrmdf6_()[3],
    ",
  0x40380b04u64 => "
      CANFD_0.cfdrmdf6_()[4],
    ",
  0x40380b08u64 => "
      CANFD_0.cfdrmdf6_()[5],
    ",
  0x40380b0cu64 => "
      CANFD_0.cfdrmdf6_()[6],
    ",
  0x40380b10u64 => "
      CANFD_0.cfdrmdf6_()[7],
    ",
  0x40380b14u64 => "
      CANFD_0.cfdrmdf6_()[8],
    ",
  0x40380b18u64 => "
      CANFD_0.cfdrmdf6_()[9],
    ",
  0x40380b1cu64 => "
      CANFD_0.cfdrmdf6_()[10],
    ",
  0x40380b20u64 => "
      CANFD_0.cfdrmdf6_()[11],
    ",
  0x40380b24u64 => "
      CANFD_0.cfdrmdf6_()[12],
    ",
  0x40380b28u64 => "
      CANFD_0.cfdrmdf6_()[13],
    ",
  0x40380b2cu64 => "
      CANFD_0.cfdrmdf6_()[14],
    ",
  0x40380b30u64 => "
      CANFD_0.cfdrmdf6_()[15],
    ",
  0x40380b40u64 => "
      CANFD_0.cfdrmdf7_()[0],
    ",
  0x40380b44u64 => "
      CANFD_0.cfdrmdf7_()[1],
    ",
  0x40380b48u64 => "
      CANFD_0.cfdrmdf7_()[2],
    ",
  0x40380b4cu64 => "
      CANFD_0.cfdrmdf7_()[3],
    ",
  0x40380b50u64 => "
      CANFD_0.cfdrmdf7_()[4],
    ",
  0x40380b54u64 => "
      CANFD_0.cfdrmdf7_()[5],
    ",
  0x40380b58u64 => "
      CANFD_0.cfdrmdf7_()[6],
    ",
  0x40380b5cu64 => "
      CANFD_0.cfdrmdf7_()[7],
    ",
  0x40380b60u64 => "
      CANFD_0.cfdrmdf7_()[8],
    ",
  0x40380b64u64 => "
      CANFD_0.cfdrmdf7_()[9],
    ",
  0x40380b68u64 => "
      CANFD_0.cfdrmdf7_()[10],
    ",
  0x40380b6cu64 => "
      CANFD_0.cfdrmdf7_()[11],
    ",
  0x40380b70u64 => "
      CANFD_0.cfdrmdf7_()[12],
    ",
  0x40380b74u64 => "
      CANFD_0.cfdrmdf7_()[13],
    ",
  0x40380b78u64 => "
      CANFD_0.cfdrmdf7_()[14],
    ",
  0x40380b7cu64 => "
      CANFD_0.cfdrmdf7_()[15],
    ",
  0x40380d2cu64 => "
      CANFD_0.cfdrmdf8_()[0],
    ",
  0x40380d30u64 => "
      CANFD_0.cfdrmdf8_()[1],
    ",
  0x40380d34u64 => "
      CANFD_0.cfdrmdf8_()[2],
    ",
  0x40380d38u64 => "
      CANFD_0.cfdrmdf8_()[3],
    ",
  0x40380d3cu64 => "
      CANFD_0.cfdrmdf8_()[4],
    ",
  0x40380d40u64 => "
      CANFD_0.cfdrmdf8_()[5],
    ",
  0x40380d44u64 => "
      CANFD_0.cfdrmdf8_()[6],
    ",
  0x40380d48u64 => "
      CANFD_0.cfdrmdf8_()[7],
    ",
  0x40380d4cu64 => "
      CANFD_0.cfdrmdf8_()[8],
    ",
  0x40380d50u64 => "
      CANFD_0.cfdrmdf8_()[9],
    ",
  0x40380d54u64 => "
      CANFD_0.cfdrmdf8_()[10],
    ",
  0x40380d58u64 => "
      CANFD_0.cfdrmdf8_()[11],
    ",
  0x40380d5cu64 => "
      CANFD_0.cfdrmdf8_()[12],
    ",
  0x40380d60u64 => "
      CANFD_0.cfdrmdf8_()[13],
    ",
  0x40380d64u64 => "
      CANFD_0.cfdrmdf8_()[14],
    ",
  0x40380d68u64 => "
      CANFD_0.cfdrmdf8_()[15],
    ",
  0x40380d78u64 => "
      CANFD_0.cfdrmdf9_()[0],
    ",
  0x40380d7cu64 => "
      CANFD_0.cfdrmdf9_()[1],
    ",
  0x40380d80u64 => "
      CANFD_0.cfdrmdf9_()[2],
    ",
  0x40380d84u64 => "
      CANFD_0.cfdrmdf9_()[3],
    ",
  0x40380d88u64 => "
      CANFD_0.cfdrmdf9_()[4],
    ",
  0x40380d8cu64 => "
      CANFD_0.cfdrmdf9_()[5],
    ",
  0x40380d90u64 => "
      CANFD_0.cfdrmdf9_()[6],
    ",
  0x40380d94u64 => "
      CANFD_0.cfdrmdf9_()[7],
    ",
  0x40380d98u64 => "
      CANFD_0.cfdrmdf9_()[8],
    ",
  0x40380d9cu64 => "
      CANFD_0.cfdrmdf9_()[9],
    ",
  0x40380da0u64 => "
      CANFD_0.cfdrmdf9_()[10],
    ",
  0x40380da4u64 => "
      CANFD_0.cfdrmdf9_()[11],
    ",
  0x40380da8u64 => "
      CANFD_0.cfdrmdf9_()[12],
    ",
  0x40380dacu64 => "
      CANFD_0.cfdrmdf9_()[13],
    ",
  0x40380db0u64 => "
      CANFD_0.cfdrmdf9_()[14],
    ",
  0x40380db4u64 => "
      CANFD_0.cfdrmdf9_()[15],
    ",
  0x40380dc4u64 => "
      CANFD_0.cfdrmdf10_()[0],
    ",
  0x40380dc8u64 => "
      CANFD_0.cfdrmdf10_()[1],
    ",
  0x40380dccu64 => "
      CANFD_0.cfdrmdf10_()[2],
    ",
  0x40380dd0u64 => "
      CANFD_0.cfdrmdf10_()[3],
    ",
  0x40380dd4u64 => "
      CANFD_0.cfdrmdf10_()[4],
    ",
  0x40380dd8u64 => "
      CANFD_0.cfdrmdf10_()[5],
    ",
  0x40380ddcu64 => "
      CANFD_0.cfdrmdf10_()[6],
    ",
  0x40380de0u64 => "
      CANFD_0.cfdrmdf10_()[7],
    ",
  0x40380de4u64 => "
      CANFD_0.cfdrmdf10_()[8],
    ",
  0x40380de8u64 => "
      CANFD_0.cfdrmdf10_()[9],
    ",
  0x40380decu64 => "
      CANFD_0.cfdrmdf10_()[10],
    ",
  0x40380df0u64 => "
      CANFD_0.cfdrmdf10_()[11],
    ",
  0x40380df4u64 => "
      CANFD_0.cfdrmdf10_()[12],
    ",
  0x40380df8u64 => "
      CANFD_0.cfdrmdf10_()[13],
    ",
  0x40380dfcu64 => "
      CANFD_0.cfdrmdf10_()[14],
    ",
  0x40380e00u64 => "
      CANFD_0.cfdrmdf10_()[15],
    ",
  0x40380e10u64 => "
      CANFD_0.cfdrmdf11_()[0],
    ",
  0x40380e14u64 => "
      CANFD_0.cfdrmdf11_()[1],
    ",
  0x40380e18u64 => "
      CANFD_0.cfdrmdf11_()[2],
    ",
  0x40380e1cu64 => "
      CANFD_0.cfdrmdf11_()[3],
    ",
  0x40380e20u64 => "
      CANFD_0.cfdrmdf11_()[4],
    ",
  0x40380e24u64 => "
      CANFD_0.cfdrmdf11_()[5],
    ",
  0x40380e28u64 => "
      CANFD_0.cfdrmdf11_()[6],
    ",
  0x40380e2cu64 => "
      CANFD_0.cfdrmdf11_()[7],
    ",
  0x40380e30u64 => "
      CANFD_0.cfdrmdf11_()[8],
    ",
  0x40380e34u64 => "
      CANFD_0.cfdrmdf11_()[9],
    ",
  0x40380e38u64 => "
      CANFD_0.cfdrmdf11_()[10],
    ",
  0x40380e3cu64 => "
      CANFD_0.cfdrmdf11_()[11],
    ",
  0x40380e40u64 => "
      CANFD_0.cfdrmdf11_()[12],
    ",
  0x40380e44u64 => "
      CANFD_0.cfdrmdf11_()[13],
    ",
  0x40380e48u64 => "
      CANFD_0.cfdrmdf11_()[14],
    ",
  0x40380e4cu64 => "
      CANFD_0.cfdrmdf11_()[15],
    ",
  0x40380e5cu64 => "
      CANFD_0.cfdrmdf12_()[0],
    ",
  0x40380e60u64 => "
      CANFD_0.cfdrmdf12_()[1],
    ",
  0x40380e64u64 => "
      CANFD_0.cfdrmdf12_()[2],
    ",
  0x40380e68u64 => "
      CANFD_0.cfdrmdf12_()[3],
    ",
  0x40380e6cu64 => "
      CANFD_0.cfdrmdf12_()[4],
    ",
  0x40380e70u64 => "
      CANFD_0.cfdrmdf12_()[5],
    ",
  0x40380e74u64 => "
      CANFD_0.cfdrmdf12_()[6],
    ",
  0x40380e78u64 => "
      CANFD_0.cfdrmdf12_()[7],
    ",
  0x40380e7cu64 => "
      CANFD_0.cfdrmdf12_()[8],
    ",
  0x40380e80u64 => "
      CANFD_0.cfdrmdf12_()[9],
    ",
  0x40380e84u64 => "
      CANFD_0.cfdrmdf12_()[10],
    ",
  0x40380e88u64 => "
      CANFD_0.cfdrmdf12_()[11],
    ",
  0x40380e8cu64 => "
      CANFD_0.cfdrmdf12_()[12],
    ",
  0x40380e90u64 => "
      CANFD_0.cfdrmdf12_()[13],
    ",
  0x40380e94u64 => "
      CANFD_0.cfdrmdf12_()[14],
    ",
  0x40380e98u64 => "
      CANFD_0.cfdrmdf12_()[15],
    ",
  0x40380ea8u64 => "
      CANFD_0.cfdrmdf13_()[0],
    ",
  0x40380eacu64 => "
      CANFD_0.cfdrmdf13_()[1],
    ",
  0x40380eb0u64 => "
      CANFD_0.cfdrmdf13_()[2],
    ",
  0x40380eb4u64 => "
      CANFD_0.cfdrmdf13_()[3],
    ",
  0x40380eb8u64 => "
      CANFD_0.cfdrmdf13_()[4],
    ",
  0x40380ebcu64 => "
      CANFD_0.cfdrmdf13_()[5],
    ",
  0x40380ec0u64 => "
      CANFD_0.cfdrmdf13_()[6],
    ",
  0x40380ec4u64 => "
      CANFD_0.cfdrmdf13_()[7],
    ",
  0x40380ec8u64 => "
      CANFD_0.cfdrmdf13_()[8],
    ",
  0x40380eccu64 => "
      CANFD_0.cfdrmdf13_()[9],
    ",
  0x40380ed0u64 => "
      CANFD_0.cfdrmdf13_()[10],
    ",
  0x40380ed4u64 => "
      CANFD_0.cfdrmdf13_()[11],
    ",
  0x40380ed8u64 => "
      CANFD_0.cfdrmdf13_()[12],
    ",
  0x40380edcu64 => "
      CANFD_0.cfdrmdf13_()[13],
    ",
  0x40380ee0u64 => "
      CANFD_0.cfdrmdf13_()[14],
    ",
  0x40380ee4u64 => "
      CANFD_0.cfdrmdf13_()[15],
    ",
  0x40380ef4u64 => "
      CANFD_0.cfdrmdf14_()[0],
    ",
  0x40380ef8u64 => "
      CANFD_0.cfdrmdf14_()[1],
    ",
  0x40380efcu64 => "
      CANFD_0.cfdrmdf14_()[2],
    ",
  0x40380f00u64 => "
      CANFD_0.cfdrmdf14_()[3],
    ",
  0x40380f04u64 => "
      CANFD_0.cfdrmdf14_()[4],
    ",
  0x40380f08u64 => "
      CANFD_0.cfdrmdf14_()[5],
    ",
  0x40380f0cu64 => "
      CANFD_0.cfdrmdf14_()[6],
    ",
  0x40380f10u64 => "
      CANFD_0.cfdrmdf14_()[7],
    ",
  0x40380f14u64 => "
      CANFD_0.cfdrmdf14_()[8],
    ",
  0x40380f18u64 => "
      CANFD_0.cfdrmdf14_()[9],
    ",
  0x40380f1cu64 => "
      CANFD_0.cfdrmdf14_()[10],
    ",
  0x40380f20u64 => "
      CANFD_0.cfdrmdf14_()[11],
    ",
  0x40380f24u64 => "
      CANFD_0.cfdrmdf14_()[12],
    ",
  0x40380f28u64 => "
      CANFD_0.cfdrmdf14_()[13],
    ",
  0x40380f2cu64 => "
      CANFD_0.cfdrmdf14_()[14],
    ",
  0x40380f30u64 => "
      CANFD_0.cfdrmdf14_()[15],
    ",
  0x40380f40u64 => "
      CANFD_0.cfdrmdf15_()[0],
    ",
  0x40380f44u64 => "
      CANFD_0.cfdrmdf15_()[1],
    ",
  0x40380f48u64 => "
      CANFD_0.cfdrmdf15_()[2],
    ",
  0x40380f4cu64 => "
      CANFD_0.cfdrmdf15_()[3],
    ",
  0x40380f50u64 => "
      CANFD_0.cfdrmdf15_()[4],
    ",
  0x40380f54u64 => "
      CANFD_0.cfdrmdf15_()[5],
    ",
  0x40380f58u64 => "
      CANFD_0.cfdrmdf15_()[6],
    ",
  0x40380f5cu64 => "
      CANFD_0.cfdrmdf15_()[7],
    ",
  0x40380f60u64 => "
      CANFD_0.cfdrmdf15_()[8],
    ",
  0x40380f64u64 => "
      CANFD_0.cfdrmdf15_()[9],
    ",
  0x40380f68u64 => "
      CANFD_0.cfdrmdf15_()[10],
    ",
  0x40380f6cu64 => "
      CANFD_0.cfdrmdf15_()[11],
    ",
  0x40380f70u64 => "
      CANFD_0.cfdrmdf15_()[12],
    ",
  0x40380f74u64 => "
      CANFD_0.cfdrmdf15_()[13],
    ",
  0x40380f78u64 => "
      CANFD_0.cfdrmdf15_()[14],
    ",
  0x40380f7cu64 => "
      CANFD_0.cfdrmdf15_()[15],
    ",
  0x4038112cu64 => "
      CANFD_0.cfdrmdf16_()[0],
    ",
  0x40381130u64 => "
      CANFD_0.cfdrmdf16_()[1],
    ",
  0x40381134u64 => "
      CANFD_0.cfdrmdf16_()[2],
    ",
  0x40381138u64 => "
      CANFD_0.cfdrmdf16_()[3],
    ",
  0x4038113cu64 => "
      CANFD_0.cfdrmdf16_()[4],
    ",
  0x40381140u64 => "
      CANFD_0.cfdrmdf16_()[5],
    ",
  0x40381144u64 => "
      CANFD_0.cfdrmdf16_()[6],
    ",
  0x40381148u64 => "
      CANFD_0.cfdrmdf16_()[7],
    ",
  0x4038114cu64 => "
      CANFD_0.cfdrmdf16_()[8],
    ",
  0x40381150u64 => "
      CANFD_0.cfdrmdf16_()[9],
    ",
  0x40381154u64 => "
      CANFD_0.cfdrmdf16_()[10],
    ",
  0x40381158u64 => "
      CANFD_0.cfdrmdf16_()[11],
    ",
  0x4038115cu64 => "
      CANFD_0.cfdrmdf16_()[12],
    ",
  0x40381160u64 => "
      CANFD_0.cfdrmdf16_()[13],
    ",
  0x40381164u64 => "
      CANFD_0.cfdrmdf16_()[14],
    ",
  0x40381168u64 => "
      CANFD_0.cfdrmdf16_()[15],
    ",
  0x40381178u64 => "
      CANFD_0.cfdrmdf17_()[0],
    ",
  0x4038117cu64 => "
      CANFD_0.cfdrmdf17_()[1],
    ",
  0x40381180u64 => "
      CANFD_0.cfdrmdf17_()[2],
    ",
  0x40381184u64 => "
      CANFD_0.cfdrmdf17_()[3],
    ",
  0x40381188u64 => "
      CANFD_0.cfdrmdf17_()[4],
    ",
  0x4038118cu64 => "
      CANFD_0.cfdrmdf17_()[5],
    ",
  0x40381190u64 => "
      CANFD_0.cfdrmdf17_()[6],
    ",
  0x40381194u64 => "
      CANFD_0.cfdrmdf17_()[7],
    ",
  0x40381198u64 => "
      CANFD_0.cfdrmdf17_()[8],
    ",
  0x4038119cu64 => "
      CANFD_0.cfdrmdf17_()[9],
    ",
  0x403811a0u64 => "
      CANFD_0.cfdrmdf17_()[10],
    ",
  0x403811a4u64 => "
      CANFD_0.cfdrmdf17_()[11],
    ",
  0x403811a8u64 => "
      CANFD_0.cfdrmdf17_()[12],
    ",
  0x403811acu64 => "
      CANFD_0.cfdrmdf17_()[13],
    ",
  0x403811b0u64 => "
      CANFD_0.cfdrmdf17_()[14],
    ",
  0x403811b4u64 => "
      CANFD_0.cfdrmdf17_()[15],
    ",
  0x403811c4u64 => "
      CANFD_0.cfdrmdf18_()[0],
    ",
  0x403811c8u64 => "
      CANFD_0.cfdrmdf18_()[1],
    ",
  0x403811ccu64 => "
      CANFD_0.cfdrmdf18_()[2],
    ",
  0x403811d0u64 => "
      CANFD_0.cfdrmdf18_()[3],
    ",
  0x403811d4u64 => "
      CANFD_0.cfdrmdf18_()[4],
    ",
  0x403811d8u64 => "
      CANFD_0.cfdrmdf18_()[5],
    ",
  0x403811dcu64 => "
      CANFD_0.cfdrmdf18_()[6],
    ",
  0x403811e0u64 => "
      CANFD_0.cfdrmdf18_()[7],
    ",
  0x403811e4u64 => "
      CANFD_0.cfdrmdf18_()[8],
    ",
  0x403811e8u64 => "
      CANFD_0.cfdrmdf18_()[9],
    ",
  0x403811ecu64 => "
      CANFD_0.cfdrmdf18_()[10],
    ",
  0x403811f0u64 => "
      CANFD_0.cfdrmdf18_()[11],
    ",
  0x403811f4u64 => "
      CANFD_0.cfdrmdf18_()[12],
    ",
  0x403811f8u64 => "
      CANFD_0.cfdrmdf18_()[13],
    ",
  0x403811fcu64 => "
      CANFD_0.cfdrmdf18_()[14],
    ",
  0x40381200u64 => "
      CANFD_0.cfdrmdf18_()[15],
    ",
  0x40381210u64 => "
      CANFD_0.cfdrmdf19_()[0],
    ",
  0x40381214u64 => "
      CANFD_0.cfdrmdf19_()[1],
    ",
  0x40381218u64 => "
      CANFD_0.cfdrmdf19_()[2],
    ",
  0x4038121cu64 => "
      CANFD_0.cfdrmdf19_()[3],
    ",
  0x40381220u64 => "
      CANFD_0.cfdrmdf19_()[4],
    ",
  0x40381224u64 => "
      CANFD_0.cfdrmdf19_()[5],
    ",
  0x40381228u64 => "
      CANFD_0.cfdrmdf19_()[6],
    ",
  0x4038122cu64 => "
      CANFD_0.cfdrmdf19_()[7],
    ",
  0x40381230u64 => "
      CANFD_0.cfdrmdf19_()[8],
    ",
  0x40381234u64 => "
      CANFD_0.cfdrmdf19_()[9],
    ",
  0x40381238u64 => "
      CANFD_0.cfdrmdf19_()[10],
    ",
  0x4038123cu64 => "
      CANFD_0.cfdrmdf19_()[11],
    ",
  0x40381240u64 => "
      CANFD_0.cfdrmdf19_()[12],
    ",
  0x40381244u64 => "
      CANFD_0.cfdrmdf19_()[13],
    ",
  0x40381248u64 => "
      CANFD_0.cfdrmdf19_()[14],
    ",
  0x4038124cu64 => "
      CANFD_0.cfdrmdf19_()[15],
    ",
  0x4038125cu64 => "
      CANFD_0.cfdrmdf20_()[0],
    ",
  0x40381260u64 => "
      CANFD_0.cfdrmdf20_()[1],
    ",
  0x40381264u64 => "
      CANFD_0.cfdrmdf20_()[2],
    ",
  0x40381268u64 => "
      CANFD_0.cfdrmdf20_()[3],
    ",
  0x4038126cu64 => "
      CANFD_0.cfdrmdf20_()[4],
    ",
  0x40381270u64 => "
      CANFD_0.cfdrmdf20_()[5],
    ",
  0x40381274u64 => "
      CANFD_0.cfdrmdf20_()[6],
    ",
  0x40381278u64 => "
      CANFD_0.cfdrmdf20_()[7],
    ",
  0x4038127cu64 => "
      CANFD_0.cfdrmdf20_()[8],
    ",
  0x40381280u64 => "
      CANFD_0.cfdrmdf20_()[9],
    ",
  0x40381284u64 => "
      CANFD_0.cfdrmdf20_()[10],
    ",
  0x40381288u64 => "
      CANFD_0.cfdrmdf20_()[11],
    ",
  0x4038128cu64 => "
      CANFD_0.cfdrmdf20_()[12],
    ",
  0x40381290u64 => "
      CANFD_0.cfdrmdf20_()[13],
    ",
  0x40381294u64 => "
      CANFD_0.cfdrmdf20_()[14],
    ",
  0x40381298u64 => "
      CANFD_0.cfdrmdf20_()[15],
    ",
  0x403812a8u64 => "
      CANFD_0.cfdrmdf21_()[0],
    ",
  0x403812acu64 => "
      CANFD_0.cfdrmdf21_()[1],
    ",
  0x403812b0u64 => "
      CANFD_0.cfdrmdf21_()[2],
    ",
  0x403812b4u64 => "
      CANFD_0.cfdrmdf21_()[3],
    ",
  0x403812b8u64 => "
      CANFD_0.cfdrmdf21_()[4],
    ",
  0x403812bcu64 => "
      CANFD_0.cfdrmdf21_()[5],
    ",
  0x403812c0u64 => "
      CANFD_0.cfdrmdf21_()[6],
    ",
  0x403812c4u64 => "
      CANFD_0.cfdrmdf21_()[7],
    ",
  0x403812c8u64 => "
      CANFD_0.cfdrmdf21_()[8],
    ",
  0x403812ccu64 => "
      CANFD_0.cfdrmdf21_()[9],
    ",
  0x403812d0u64 => "
      CANFD_0.cfdrmdf21_()[10],
    ",
  0x403812d4u64 => "
      CANFD_0.cfdrmdf21_()[11],
    ",
  0x403812d8u64 => "
      CANFD_0.cfdrmdf21_()[12],
    ",
  0x403812dcu64 => "
      CANFD_0.cfdrmdf21_()[13],
    ",
  0x403812e0u64 => "
      CANFD_0.cfdrmdf21_()[14],
    ",
  0x403812e4u64 => "
      CANFD_0.cfdrmdf21_()[15],
    ",
  0x403812f4u64 => "
      CANFD_0.cfdrmdf22_()[0],
    ",
  0x403812f8u64 => "
      CANFD_0.cfdrmdf22_()[1],
    ",
  0x403812fcu64 => "
      CANFD_0.cfdrmdf22_()[2],
    ",
  0x40381300u64 => "
      CANFD_0.cfdrmdf22_()[3],
    ",
  0x40381304u64 => "
      CANFD_0.cfdrmdf22_()[4],
    ",
  0x40381308u64 => "
      CANFD_0.cfdrmdf22_()[5],
    ",
  0x4038130cu64 => "
      CANFD_0.cfdrmdf22_()[6],
    ",
  0x40381310u64 => "
      CANFD_0.cfdrmdf22_()[7],
    ",
  0x40381314u64 => "
      CANFD_0.cfdrmdf22_()[8],
    ",
  0x40381318u64 => "
      CANFD_0.cfdrmdf22_()[9],
    ",
  0x4038131cu64 => "
      CANFD_0.cfdrmdf22_()[10],
    ",
  0x40381320u64 => "
      CANFD_0.cfdrmdf22_()[11],
    ",
  0x40381324u64 => "
      CANFD_0.cfdrmdf22_()[12],
    ",
  0x40381328u64 => "
      CANFD_0.cfdrmdf22_()[13],
    ",
  0x4038132cu64 => "
      CANFD_0.cfdrmdf22_()[14],
    ",
  0x40381330u64 => "
      CANFD_0.cfdrmdf22_()[15],
    ",
  0x40381340u64 => "
      CANFD_0.cfdrmdf23_()[0],
    ",
  0x40381344u64 => "
      CANFD_0.cfdrmdf23_()[1],
    ",
  0x40381348u64 => "
      CANFD_0.cfdrmdf23_()[2],
    ",
  0x4038134cu64 => "
      CANFD_0.cfdrmdf23_()[3],
    ",
  0x40381350u64 => "
      CANFD_0.cfdrmdf23_()[4],
    ",
  0x40381354u64 => "
      CANFD_0.cfdrmdf23_()[5],
    ",
  0x40381358u64 => "
      CANFD_0.cfdrmdf23_()[6],
    ",
  0x4038135cu64 => "
      CANFD_0.cfdrmdf23_()[7],
    ",
  0x40381360u64 => "
      CANFD_0.cfdrmdf23_()[8],
    ",
  0x40381364u64 => "
      CANFD_0.cfdrmdf23_()[9],
    ",
  0x40381368u64 => "
      CANFD_0.cfdrmdf23_()[10],
    ",
  0x4038136cu64 => "
      CANFD_0.cfdrmdf23_()[11],
    ",
  0x40381370u64 => "
      CANFD_0.cfdrmdf23_()[12],
    ",
  0x40381374u64 => "
      CANFD_0.cfdrmdf23_()[13],
    ",
  0x40381378u64 => "
      CANFD_0.cfdrmdf23_()[14],
    ",
  0x4038137cu64 => "
      CANFD_0.cfdrmdf23_()[15],
    ",
  0x40381520u64 => "
      CANFD_0.cfdrmid()[0],
    ",
  0x4038156cu64 => "
      CANFD_0.cfdrmid()[1],
    ",
  0x403815b8u64 => "
      CANFD_0.cfdrmid()[2],
    ",
  0x40381604u64 => "
      CANFD_0.cfdrmid()[3],
    ",
  0x40381650u64 => "
      CANFD_0.cfdrmid()[4],
    ",
  0x4038169cu64 => "
      CANFD_0.cfdrmid()[5],
    ",
  0x403816e8u64 => "
      CANFD_0.cfdrmid()[6],
    ",
  0x40381734u64 => "
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
      CANFD_0.cfdrmdf24_()[0],
    ",
  0x40381530u64 => "
      CANFD_0.cfdrmdf24_()[1],
    ",
  0x40381534u64 => "
      CANFD_0.cfdrmdf24_()[2],
    ",
  0x40381538u64 => "
      CANFD_0.cfdrmdf24_()[3],
    ",
  0x4038153cu64 => "
      CANFD_0.cfdrmdf24_()[4],
    ",
  0x40381540u64 => "
      CANFD_0.cfdrmdf24_()[5],
    ",
  0x40381544u64 => "
      CANFD_0.cfdrmdf24_()[6],
    ",
  0x40381548u64 => "
      CANFD_0.cfdrmdf24_()[7],
    ",
  0x4038154cu64 => "
      CANFD_0.cfdrmdf24_()[8],
    ",
  0x40381550u64 => "
      CANFD_0.cfdrmdf24_()[9],
    ",
  0x40381554u64 => "
      CANFD_0.cfdrmdf24_()[10],
    ",
  0x40381558u64 => "
      CANFD_0.cfdrmdf24_()[11],
    ",
  0x4038155cu64 => "
      CANFD_0.cfdrmdf24_()[12],
    ",
  0x40381560u64 => "
      CANFD_0.cfdrmdf24_()[13],
    ",
  0x40381564u64 => "
      CANFD_0.cfdrmdf24_()[14],
    ",
  0x40381568u64 => "
      CANFD_0.cfdrmdf24_()[15],
    ",
  0x40381578u64 => "
      CANFD_0.cfdrmdf25_()[0],
    ",
  0x4038157cu64 => "
      CANFD_0.cfdrmdf25_()[1],
    ",
  0x40381580u64 => "
      CANFD_0.cfdrmdf25_()[2],
    ",
  0x40381584u64 => "
      CANFD_0.cfdrmdf25_()[3],
    ",
  0x40381588u64 => "
      CANFD_0.cfdrmdf25_()[4],
    ",
  0x4038158cu64 => "
      CANFD_0.cfdrmdf25_()[5],
    ",
  0x40381590u64 => "
      CANFD_0.cfdrmdf25_()[6],
    ",
  0x40381594u64 => "
      CANFD_0.cfdrmdf25_()[7],
    ",
  0x40381598u64 => "
      CANFD_0.cfdrmdf25_()[8],
    ",
  0x4038159cu64 => "
      CANFD_0.cfdrmdf25_()[9],
    ",
  0x403815a0u64 => "
      CANFD_0.cfdrmdf25_()[10],
    ",
  0x403815a4u64 => "
      CANFD_0.cfdrmdf25_()[11],
    ",
  0x403815a8u64 => "
      CANFD_0.cfdrmdf25_()[12],
    ",
  0x403815acu64 => "
      CANFD_0.cfdrmdf25_()[13],
    ",
  0x403815b0u64 => "
      CANFD_0.cfdrmdf25_()[14],
    ",
  0x403815b4u64 => "
      CANFD_0.cfdrmdf25_()[15],
    ",
  0x403815c4u64 => "
      CANFD_0.cfdrmdf26_()[0],
    ",
  0x403815c8u64 => "
      CANFD_0.cfdrmdf26_()[1],
    ",
  0x403815ccu64 => "
      CANFD_0.cfdrmdf26_()[2],
    ",
  0x403815d0u64 => "
      CANFD_0.cfdrmdf26_()[3],
    ",
  0x403815d4u64 => "
      CANFD_0.cfdrmdf26_()[4],
    ",
  0x403815d8u64 => "
      CANFD_0.cfdrmdf26_()[5],
    ",
  0x403815dcu64 => "
      CANFD_0.cfdrmdf26_()[6],
    ",
  0x403815e0u64 => "
      CANFD_0.cfdrmdf26_()[7],
    ",
  0x403815e4u64 => "
      CANFD_0.cfdrmdf26_()[8],
    ",
  0x403815e8u64 => "
      CANFD_0.cfdrmdf26_()[9],
    ",
  0x403815ecu64 => "
      CANFD_0.cfdrmdf26_()[10],
    ",
  0x403815f0u64 => "
      CANFD_0.cfdrmdf26_()[11],
    ",
  0x403815f4u64 => "
      CANFD_0.cfdrmdf26_()[12],
    ",
  0x403815f8u64 => "
      CANFD_0.cfdrmdf26_()[13],
    ",
  0x403815fcu64 => "
      CANFD_0.cfdrmdf26_()[14],
    ",
  0x40381600u64 => "
      CANFD_0.cfdrmdf26_()[15],
    ",
  0x40381610u64 => "
      CANFD_0.cfdrmdf27_()[0],
    ",
  0x40381614u64 => "
      CANFD_0.cfdrmdf27_()[1],
    ",
  0x40381618u64 => "
      CANFD_0.cfdrmdf27_()[2],
    ",
  0x4038161cu64 => "
      CANFD_0.cfdrmdf27_()[3],
    ",
  0x40381620u64 => "
      CANFD_0.cfdrmdf27_()[4],
    ",
  0x40381624u64 => "
      CANFD_0.cfdrmdf27_()[5],
    ",
  0x40381628u64 => "
      CANFD_0.cfdrmdf27_()[6],
    ",
  0x4038162cu64 => "
      CANFD_0.cfdrmdf27_()[7],
    ",
  0x40381630u64 => "
      CANFD_0.cfdrmdf27_()[8],
    ",
  0x40381634u64 => "
      CANFD_0.cfdrmdf27_()[9],
    ",
  0x40381638u64 => "
      CANFD_0.cfdrmdf27_()[10],
    ",
  0x4038163cu64 => "
      CANFD_0.cfdrmdf27_()[11],
    ",
  0x40381640u64 => "
      CANFD_0.cfdrmdf27_()[12],
    ",
  0x40381644u64 => "
      CANFD_0.cfdrmdf27_()[13],
    ",
  0x40381648u64 => "
      CANFD_0.cfdrmdf27_()[14],
    ",
  0x4038164cu64 => "
      CANFD_0.cfdrmdf27_()[15],
    ",
  0x4038165cu64 => "
      CANFD_0.cfdrmdf28_()[0],
    ",
  0x40381660u64 => "
      CANFD_0.cfdrmdf28_()[1],
    ",
  0x40381664u64 => "
      CANFD_0.cfdrmdf28_()[2],
    ",
  0x40381668u64 => "
      CANFD_0.cfdrmdf28_()[3],
    ",
  0x4038166cu64 => "
      CANFD_0.cfdrmdf28_()[4],
    ",
  0x40381670u64 => "
      CANFD_0.cfdrmdf28_()[5],
    ",
  0x40381674u64 => "
      CANFD_0.cfdrmdf28_()[6],
    ",
  0x40381678u64 => "
      CANFD_0.cfdrmdf28_()[7],
    ",
  0x4038167cu64 => "
      CANFD_0.cfdrmdf28_()[8],
    ",
  0x40381680u64 => "
      CANFD_0.cfdrmdf28_()[9],
    ",
  0x40381684u64 => "
      CANFD_0.cfdrmdf28_()[10],
    ",
  0x40381688u64 => "
      CANFD_0.cfdrmdf28_()[11],
    ",
  0x4038168cu64 => "
      CANFD_0.cfdrmdf28_()[12],
    ",
  0x40381690u64 => "
      CANFD_0.cfdrmdf28_()[13],
    ",
  0x40381694u64 => "
      CANFD_0.cfdrmdf28_()[14],
    ",
  0x40381698u64 => "
      CANFD_0.cfdrmdf28_()[15],
    ",
  0x403816a8u64 => "
      CANFD_0.cfdrmdf29_()[0],
    ",
  0x403816acu64 => "
      CANFD_0.cfdrmdf29_()[1],
    ",
  0x403816b0u64 => "
      CANFD_0.cfdrmdf29_()[2],
    ",
  0x403816b4u64 => "
      CANFD_0.cfdrmdf29_()[3],
    ",
  0x403816b8u64 => "
      CANFD_0.cfdrmdf29_()[4],
    ",
  0x403816bcu64 => "
      CANFD_0.cfdrmdf29_()[5],
    ",
  0x403816c0u64 => "
      CANFD_0.cfdrmdf29_()[6],
    ",
  0x403816c4u64 => "
      CANFD_0.cfdrmdf29_()[7],
    ",
  0x403816c8u64 => "
      CANFD_0.cfdrmdf29_()[8],
    ",
  0x403816ccu64 => "
      CANFD_0.cfdrmdf29_()[9],
    ",
  0x403816d0u64 => "
      CANFD_0.cfdrmdf29_()[10],
    ",
  0x403816d4u64 => "
      CANFD_0.cfdrmdf29_()[11],
    ",
  0x403816d8u64 => "
      CANFD_0.cfdrmdf29_()[12],
    ",
  0x403816dcu64 => "
      CANFD_0.cfdrmdf29_()[13],
    ",
  0x403816e0u64 => "
      CANFD_0.cfdrmdf29_()[14],
    ",
  0x403816e4u64 => "
      CANFD_0.cfdrmdf29_()[15],
    ",
  0x403816f4u64 => "
      CANFD_0.cfdrmdf30_()[0],
    ",
  0x403816f8u64 => "
      CANFD_0.cfdrmdf30_()[1],
    ",
  0x403816fcu64 => "
      CANFD_0.cfdrmdf30_()[2],
    ",
  0x40381700u64 => "
      CANFD_0.cfdrmdf30_()[3],
    ",
  0x40381704u64 => "
      CANFD_0.cfdrmdf30_()[4],
    ",
  0x40381708u64 => "
      CANFD_0.cfdrmdf30_()[5],
    ",
  0x4038170cu64 => "
      CANFD_0.cfdrmdf30_()[6],
    ",
  0x40381710u64 => "
      CANFD_0.cfdrmdf30_()[7],
    ",
  0x40381714u64 => "
      CANFD_0.cfdrmdf30_()[8],
    ",
  0x40381718u64 => "
      CANFD_0.cfdrmdf30_()[9],
    ",
  0x4038171cu64 => "
      CANFD_0.cfdrmdf30_()[10],
    ",
  0x40381720u64 => "
      CANFD_0.cfdrmdf30_()[11],
    ",
  0x40381724u64 => "
      CANFD_0.cfdrmdf30_()[12],
    ",
  0x40381728u64 => "
      CANFD_0.cfdrmdf30_()[13],
    ",
  0x4038172cu64 => "
      CANFD_0.cfdrmdf30_()[14],
    ",
  0x40381730u64 => "
      CANFD_0.cfdrmdf30_()[15],
    ",
  0x40381740u64 => "
      CANFD_0.cfdrmdf31_()[0],
    ",
  0x40381744u64 => "
      CANFD_0.cfdrmdf31_()[1],
    ",
  0x40381748u64 => "
      CANFD_0.cfdrmdf31_()[2],
    ",
  0x4038174cu64 => "
      CANFD_0.cfdrmdf31_()[3],
    ",
  0x40381750u64 => "
      CANFD_0.cfdrmdf31_()[4],
    ",
  0x40381754u64 => "
      CANFD_0.cfdrmdf31_()[5],
    ",
  0x40381758u64 => "
      CANFD_0.cfdrmdf31_()[6],
    ",
  0x4038175cu64 => "
      CANFD_0.cfdrmdf31_()[7],
    ",
  0x40381760u64 => "
      CANFD_0.cfdrmdf31_()[8],
    ",
  0x40381764u64 => "
      CANFD_0.cfdrmdf31_()[9],
    ",
  0x40381768u64 => "
      CANFD_0.cfdrmdf31_()[10],
    ",
  0x4038176cu64 => "
      CANFD_0.cfdrmdf31_()[11],
    ",
  0x40381770u64 => "
      CANFD_0.cfdrmdf31_()[12],
    ",
  0x40381774u64 => "
      CANFD_0.cfdrmdf31_()[13],
    ",
  0x40381778u64 => "
      CANFD_0.cfdrmdf31_()[14],
    ",
  0x4038177cu64 => "
      CANFD_0.cfdrmdf31_()[15],
    ",
  0x40381920u64 => "
      CANFD_0.cfdrfide()[0],
    ",
  0x4038196cu64 => "
      CANFD_0.cfdrfide()[1],
    ",
  0x40381924u64 => "
      CANFD_0.cfdrfptre()[0],
    ",
  0x40381970u64 => "
      CANFD_0.cfdrfptre()[1],
    ",
  0x40381928u64 => "
      CANFD_0.cfdrffdstse()[0],
    ",
  0x40381974u64 => "
      CANFD_0.cfdrffdstse()[1],
    ",
  0x4038192cu64 => "
      CANFD_0.cfdrfdf0e()[0],
    ",
  0x40381978u64 => "
      CANFD_0.cfdrfdf0e()[1],
    ",
  0x40381930u64 => "
      CANFD_0.cfdrfdf1e()[0],
    ",
  0x4038197cu64 => "
      CANFD_0.cfdrfdf1e()[1],
    ",
  0x40381934u64 => "
      CANFD_0.cfdrfdf2e()[0],
    ",
  0x40381980u64 => "
      CANFD_0.cfdrfdf2e()[1],
    ",
  0x40381938u64 => "
      CANFD_0.cfdrfdf3e()[0],
    ",
  0x40381984u64 => "
      CANFD_0.cfdrfdf3e()[1],
    ",
  0x4038193cu64 => "
      CANFD_0.cfdrfdf4e()[0],
    ",
  0x40381988u64 => "
      CANFD_0.cfdrfdf4e()[1],
    ",
  0x40381940u64 => "
      CANFD_0.cfdrfdf5e()[0],
    ",
  0x4038198cu64 => "
      CANFD_0.cfdrfdf5e()[1],
    ",
  0x40381944u64 => "
      CANFD_0.cfdrfdf6e()[0],
    ",
  0x40381990u64 => "
      CANFD_0.cfdrfdf6e()[1],
    ",
  0x40381948u64 => "
      CANFD_0.cfdrfdf7e()[0],
    ",
  0x40381994u64 => "
      CANFD_0.cfdrfdf7e()[1],
    ",
  0x4038194cu64 => "
      CANFD_0.cfdrfdf8e()[0],
    ",
  0x40381998u64 => "
      CANFD_0.cfdrfdf8e()[1],
    ",
  0x40381950u64 => "
      CANFD_0.cfdrfdf9e()[0],
    ",
  0x4038199cu64 => "
      CANFD_0.cfdrfdf9e()[1],
    ",
  0x40381954u64 => "
      CANFD_0.cfdrfdf10e()[0],
    ",
  0x403819a0u64 => "
      CANFD_0.cfdrfdf10e()[1],
    ",
  0x40381958u64 => "
      CANFD_0.cfdrfdf11e()[0],
    ",
  0x403819a4u64 => "
      CANFD_0.cfdrfdf11e()[1],
    ",
  0x4038195cu64 => "
      CANFD_0.cfdrfdf12e()[0],
    ",
  0x403819a8u64 => "
      CANFD_0.cfdrfdf12e()[1],
    ",
  0x40381960u64 => "
      CANFD_0.cfdrfdf13e()[0],
    ",
  0x403819acu64 => "
      CANFD_0.cfdrfdf13e()[1],
    ",
  0x40381964u64 => "
      CANFD_0.cfdrfdf14e()[0],
    ",
  0x403819b0u64 => "
      CANFD_0.cfdrfdf14e()[1],
    ",
  0x40381968u64 => "
      CANFD_0.cfdrfdf15e()[0],
    ",
  0x403819b4u64 => "
      CANFD_0.cfdrfdf15e()[1],
    ",
  0x403819b8u64 => "
      CANFD_0.cfdcfide(),
    ",
  0x403819bcu64 => "
      CANFD_0.cfdcfptre(),
    ",
  0x403819c0u64 => "
      CANFD_0.cfdcffdcstse(),
    ",
  0x403819c4u64 => "
      CANFD_0.cfdcfdfe()[0],
    ",
  0x403819c8u64 => "
      CANFD_0.cfdcfdfe()[1],
    ",
  0x403819ccu64 => "
      CANFD_0.cfdcfdfe()[2],
    ",
  0x403819d0u64 => "
      CANFD_0.cfdcfdfe()[3],
    ",
  0x403819d4u64 => "
      CANFD_0.cfdcfdfe()[4],
    ",
  0x403819d8u64 => "
      CANFD_0.cfdcfdfe()[5],
    ",
  0x403819dcu64 => "
      CANFD_0.cfdcfdfe()[6],
    ",
  0x403819e0u64 => "
      CANFD_0.cfdcfdfe()[7],
    ",
  0x403819e4u64 => "
      CANFD_0.cfdcfdfe()[8],
    ",
  0x403819e8u64 => "
      CANFD_0.cfdcfdfe()[9],
    ",
  0x403819ecu64 => "
      CANFD_0.cfdcfdfe()[10],
    ",
  0x403819f0u64 => "
      CANFD_0.cfdcfdfe()[11],
    ",
  0x403819f4u64 => "
      CANFD_0.cfdcfdfe()[12],
    ",
  0x403819f8u64 => "
      CANFD_0.cfdcfdfe()[13],
    ",
  0x403819fcu64 => "
      CANFD_0.cfdcfdfe()[14],
    ",
  0x40381a00u64 => "
      CANFD_0.cfdcfdfe()[15],
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
  0x40400006u64 => "
      PORT_0.eidr(),
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
  0x40400800u64 => "
      PFS.p000pfs(),
      PFS.p000pfs_ha(),
      PFS.p000pfs_by(),
    ",
  0x4040080cu64 => "
      PFS.p00pfs()[2],
      PFS.p00pfs_ha()[2],
      PFS.p00pfs_by()[2],
    ",
  0x40400810u64 => "
      PFS.p00pfs()[3],
      PFS.p00pfs_ha()[3],
      PFS.p00pfs_by()[3],
    ",
  0x40400814u64 => "
      PFS.p00pfs()[4],
      PFS.p00pfs_ha()[4],
      PFS.p00pfs_by()[4],
    ",
  0x40400818u64 => "
      PFS.p00pfs()[5],
      PFS.p00pfs_ha()[5],
      PFS.p00pfs_by()[5],
    ",
  0x4040081cu64 => "
      PFS.p00pfs()[6],
      PFS.p00pfs_ha()[6],
      PFS.p00pfs_by()[6],
    ",
  0x40400820u64 => "
      PFS.p00pfs()[0],
      PFS.p00pfs_ha()[0],
      PFS.p00pfs_by()[0],
    ",
  0x40400824u64 => "
      PFS.p00pfs()[1],
      PFS.p00pfs_ha()[1],
      PFS.p00pfs_by()[1],
    ",
  0x40400828u64 => "
      PFS.p0pfs()[0],
      PFS.p0pfs_ha()[0],
      PFS.p0pfs_by()[0],
    ",
  0x4040082cu64 => "
      PFS.p0pfs()[1],
      PFS.p0pfs_ha()[1],
      PFS.p0pfs_by()[1],
    ",
  0x40400830u64 => "
      PFS.p0pfs()[2],
      PFS.p0pfs_ha()[2],
      PFS.p0pfs_by()[2],
    ",
  0x40400834u64 => "
      PFS.p0pfs()[3],
      PFS.p0pfs_ha()[3],
      PFS.p0pfs_by()[3],
    ",
  0x40400838u64 => "
      PFS.p0pfs()[4],
      PFS.p0pfs_ha()[4],
      PFS.p0pfs_by()[4],
    ",
  0x4040083cu64 => "
      PFS.p0pfs()[5],
      PFS.p0pfs_ha()[5],
      PFS.p0pfs_by()[5],
    ",
  0x40400840u64 => "
      PFS.p100pfs(),
      PFS.p100pfs_ha(),
      PFS.p100pfs_by(),
    ",
  0x40400844u64 => "
      PFS.p10pfs()[0],
      PFS.p10pfs_ha()[0],
      PFS.p10pfs_by()[0],
    ",
  0x40400848u64 => "
      PFS.p10pfs()[1],
      PFS.p10pfs_ha()[1],
      PFS.p10pfs_by()[1],
    ",
  0x4040084cu64 => "
      PFS.p10pfs()[2],
      PFS.p10pfs_ha()[2],
      PFS.p10pfs_by()[2],
    ",
  0x40400850u64 => "
      PFS.p10pfs()[3],
      PFS.p10pfs_ha()[3],
      PFS.p10pfs_by()[3],
    ",
  0x40400854u64 => "
      PFS.p10pfs()[4],
      PFS.p10pfs_ha()[4],
      PFS.p10pfs_by()[4],
    ",
  0x40400858u64 => "
      PFS.p10pfs()[5],
      PFS.p10pfs_ha()[5],
      PFS.p10pfs_by()[5],
    ",
  0x4040085cu64 => "
      PFS.p10pfs()[6],
      PFS.p10pfs_ha()[6],
      PFS.p10pfs_by()[6],
    ",
  0x40400860u64 => "
      PFS.p108pfs(),
      PFS.p108pfs_ha(),
      PFS.p108pfs_by(),
    ",
  0x40400864u64 => "
      PFS.p109pfs(),
      PFS.p109pfs_ha(),
      PFS.p109pfs_by(),
    ",
  0x40400868u64 => "
      PFS.p110pfs(),
      PFS.p110pfs_ha(),
      PFS.p110pfs_by(),
    ",
  0x4040086cu64 => "
      PFS.p1pfs()[0],
      PFS.p1pfs_ha()[0],
      PFS.p1pfs_by()[0],
    ",
  0x40400870u64 => "
      PFS.p1pfs()[1],
      PFS.p1pfs_ha()[1],
      PFS.p1pfs_by()[1],
    ",
  0x40400874u64 => "
      PFS.p1pfs()[2],
      PFS.p1pfs_ha()[2],
      PFS.p1pfs_by()[2],
    ",
  0x40400878u64 => "
      PFS.p1pfs()[3],
      PFS.p1pfs_ha()[3],
      PFS.p1pfs_by()[3],
    ",
  0x4040087cu64 => "
      PFS.p1pfs()[4],
      PFS.p1pfs_ha()[4],
      PFS.p1pfs_by()[4],
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
      PFS.p20pfs()[6],
      PFS.p20pfs_ha()[6],
      PFS.p20pfs_by()[6],
    ",
  0x404008a4u64 => "
      PFS.p20pfs()[7],
      PFS.p20pfs_ha()[7],
      PFS.p20pfs_by()[7],
    ",
  0x404008a8u64 => "
      PFS.p2pfs()[0],
      PFS.p2pfs_ha()[0],
      PFS.p2pfs_by()[0],
    ",
  0x404008acu64 => "
      PFS.p2pfs()[1],
      PFS.p2pfs_ha()[1],
      PFS.p2pfs_by()[1],
    ",
  0x404008b0u64 => "
      PFS.p2pfs()[2],
      PFS.p2pfs_ha()[2],
      PFS.p2pfs_by()[2],
    ",
  0x404008b4u64 => "
      PFS.p2pfs()[3],
      PFS.p2pfs_ha()[3],
      PFS.p2pfs_by()[3],
    ",
  0x404008b8u64 => "
      PFS.p2pfs()[4],
      PFS.p2pfs_ha()[4],
      PFS.p2pfs_by()[4],
    ",
  0x404008bcu64 => "
      PFS.p2pfs()[5],
      PFS.p2pfs_ha()[5],
      PFS.p2pfs_by()[5],
    ",
  0x404008c0u64 => "
      PFS.p300pfs(),
      PFS.p300pfs_ha(),
      PFS.p300pfs_by(),
    ",
  0x404008c4u64 => "
      PFS.p30pfs()[0],
      PFS.p30pfs_ha()[0],
      PFS.p30pfs_by()[0],
    ",
  0x404008c8u64 => "
      PFS.p30pfs()[1],
      PFS.p30pfs_ha()[1],
      PFS.p30pfs_by()[1],
    ",
  0x404008ccu64 => "
      PFS.p30pfs()[2],
      PFS.p30pfs_ha()[2],
      PFS.p30pfs_by()[2],
    ",
  0x404008d0u64 => "
      PFS.p30pfs()[3],
      PFS.p30pfs_ha()[3],
      PFS.p30pfs_by()[3],
    ",
  0x404008d4u64 => "
      PFS.p30pfs()[4],
      PFS.p30pfs_ha()[4],
      PFS.p30pfs_by()[4],
    ",
  0x404008d8u64 => "
      PFS.p30pfs()[5],
      PFS.p30pfs_ha()[5],
      PFS.p30pfs_by()[5],
    ",
  0x404008dcu64 => "
      PFS.p30pfs()[6],
      PFS.p30pfs_ha()[6],
      PFS.p30pfs_by()[6],
    ",
  0x404008e0u64 => "
      PFS.p30pfs()[7],
      PFS.p30pfs_ha()[7],
      PFS.p30pfs_by()[7],
    ",
  0x404008e4u64 => "
      PFS.p30pfs()[8],
      PFS.p30pfs_ha()[8],
      PFS.p30pfs_by()[8],
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
      PFS.p50pfs()[0],
      PFS.p50pfs_ha()[0],
      PFS.p50pfs_by()[0],
    ",
  0x40400964u64 => "
      PFS.p50pfs()[1],
      PFS.p50pfs_ha()[1],
      PFS.p50pfs_by()[1],
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
      PFS.p60pfs()[0],
      PFS.p60pfs_ha()[0],
      PFS.p60pfs_by()[0],
    ",
  0x404009a4u64 => "
      PFS.p60pfs()[1],
      PFS.p60pfs_ha()[1],
      PFS.p60pfs_by()[1],
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
  0x40400a28u64 => "
      PFS.p8pfs()[0],
      PFS.p8pfs_ha()[0],
      PFS.p8pfs_by()[0],
    ",
  0x40400a2cu64 => "
      PFS.p8pfs()[1],
      PFS.p8pfs_ha()[1],
      PFS.p8pfs_by()[1],
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
      PFS.p8pfs()[4],
      PFS.p8pfs_ha()[4],
      PFS.p8pfs_by()[4],
    ",
  0x40400a3cu64 => "
      PFS.p8pfs()[5],
      PFS.p8pfs_ha()[5],
      PFS.p8pfs_by()[5],
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
  0x40400ae0u64 => "
      PFS.pb0pfs()[8],
      PFS.pb0pfs_ha()[8],
      PFS.pb0pfs_by()[8],
    ",
  0x40400ae4u64 => "
      PFS.pb0pfs()[9],
      PFS.pb0pfs_ha()[9],
      PFS.pb0pfs_by()[9],
    ",
  0x40400ae8u64 => "
      PFS.pbpfs()[0],
      PFS.pbpfs_ha()[0],
      PFS.pbpfs_by()[0],
    ",
  0x40400aecu64 => "
      PFS.pbpfs()[1],
      PFS.pbpfs_ha()[1],
      PFS.pbpfs_by()[1],
    ",
  0x40400af0u64 => "
      PFS.pbpfs()[2],
      PFS.pbpfs_ha()[2],
      PFS.pbpfs_by()[2],
    ",
  0x40400af4u64 => "
      PFS.pbpfs()[3],
      PFS.pbpfs_ha()[3],
      PFS.pbpfs_by()[3],
    ",
  0x40400af8u64 => "
      PFS.pbpfs()[4],
      PFS.pbpfs_ha()[4],
      PFS.pbpfs_by()[4],
    ",
  0x40400afcu64 => "
      PFS.pbpfs()[5],
      PFS.pbpfs_ha()[5],
      PFS.pbpfs_by()[5],
    ",
  0x40400d00u64 => "
      PFS.pfenet(),
    ",
  0x40400d0cu64 => "
      PFS.pwpr_ns(),
    ",
  0x40400d14u64 => "
      PFS.pwpr_s(),
    ",
  0x40400d20u64 => "
      PFS.pfi3c(),
    ",
  0x40400d30u64 => "
      PFS.p0sar(),
    ",
  0x40400d34u64 => "
      PFS.p1sar(),
    ",
  0x40400d38u64 => "
      PFS.p2sar(),
    ",
  0x40400d3cu64 => "
      PFS.p3sar(),
    ",
  0x40400d40u64 => "
      PFS.p4sar(),
    ",
  0x40400d44u64 => "
      PFS.p5sar(),
    ",
  0x40400d48u64 => "
      PFS.p6sar(),
    ",
  0x40400d4cu64 => "
      PFS.p7sar(),
    ",
  0x40400d50u64 => "
      PFS.p8sar(),
    ",
  0x40400d54u64 => "
      PFS.p9sar(),
    ",
  0x40400d58u64 => "
      PFS.pasar(),
    ",
  0x40400d5cu64 => "
      PFS.pbsar(),
    ",
  0x40400d60u64 => "
      PFS.pcsar(),
    ",
  0x40400d64u64 => "
      PFS.pdsar(),
    ",
  0x40400d68u64 => "
      PFS.pesar(),
    ",
  0x40400d6cu64 => "
      PFS.pfsar(),
    ",
  0x40400d70u64 => "
      PFS.pgsar(),
    ",
};
