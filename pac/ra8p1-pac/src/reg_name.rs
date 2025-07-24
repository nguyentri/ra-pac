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
  0x2c1eda0u64 => "
      TSD.tscdr(),
    ",
  0x2c1eda4u64 => "
      TSD.tscdr2(),
    ",
  0x12c1eda0u64 => "
      TSD_NS.tscdr(),
    ",
  0x12c1eda4u64 => "
      TSD_NS.tscdr2(),
    ",
  0x40000000u64 => "
      RMPU.mmpuoad(),
    ",
  0x40000004u64 => "
      RMPU.mmpuoadpt(),
    ",
  0x40000100u64 => "
      RMPU.mmpuendmac()[0],
    ",
  0x40000300u64 => "
      RMPU.mmpuendmac()[1],
    ",
  0x40000104u64 => "
      RMPU.mmpuenptdmac()[0],
    ",
  0x40000304u64 => "
      RMPU.mmpuenptdmac()[1],
    ",
  0x4000010cu64 => "
      RMPU.mmpurptdmac_sec()[0],
    ",
  0x4000030cu64 => "
      RMPU.mmpurptdmac_sec()[1],
    ",
  0x40000200u64 => "
      RMPU.mmpuacdmac0()[0],
    ",
  0x40000210u64 => "
      RMPU.mmpuacdmac0()[1],
    ",
  0x40000220u64 => "
      RMPU.mmpuacdmac0()[2],
    ",
  0x40000230u64 => "
      RMPU.mmpuacdmac0()[3],
    ",
  0x40000240u64 => "
      RMPU.mmpuacdmac0()[4],
    ",
  0x40000250u64 => "
      RMPU.mmpuacdmac0()[5],
    ",
  0x40000260u64 => "
      RMPU.mmpuacdmac0()[6],
    ",
  0x40000270u64 => "
      RMPU.mmpuacdmac0()[7],
    ",
  0x40000204u64 => "
      RMPU.mmpusdmac0()[0],
    ",
  0x40000214u64 => "
      RMPU.mmpusdmac0()[1],
    ",
  0x40000224u64 => "
      RMPU.mmpusdmac0()[2],
    ",
  0x40000234u64 => "
      RMPU.mmpusdmac0()[3],
    ",
  0x40000244u64 => "
      RMPU.mmpusdmac0()[4],
    ",
  0x40000254u64 => "
      RMPU.mmpusdmac0()[5],
    ",
  0x40000264u64 => "
      RMPU.mmpusdmac0()[6],
    ",
  0x40000274u64 => "
      RMPU.mmpusdmac0()[7],
    ",
  0x40000208u64 => "
      RMPU.mmpuedmac0()[0],
    ",
  0x40000218u64 => "
      RMPU.mmpuedmac0()[1],
    ",
  0x40000228u64 => "
      RMPU.mmpuedmac0()[2],
    ",
  0x40000238u64 => "
      RMPU.mmpuedmac0()[3],
    ",
  0x40000248u64 => "
      RMPU.mmpuedmac0()[4],
    ",
  0x40000258u64 => "
      RMPU.mmpuedmac0()[5],
    ",
  0x40000268u64 => "
      RMPU.mmpuedmac0()[6],
    ",
  0x40000278u64 => "
      RMPU.mmpuedmac0()[7],
    ",
  0x40000400u64 => "
      RMPU.mmpuacdmac1()[0],
    ",
  0x40000410u64 => "
      RMPU.mmpuacdmac1()[1],
    ",
  0x40000420u64 => "
      RMPU.mmpuacdmac1()[2],
    ",
  0x40000430u64 => "
      RMPU.mmpuacdmac1()[3],
    ",
  0x40000440u64 => "
      RMPU.mmpuacdmac1()[4],
    ",
  0x40000450u64 => "
      RMPU.mmpuacdmac1()[5],
    ",
  0x40000460u64 => "
      RMPU.mmpuacdmac1()[6],
    ",
  0x40000470u64 => "
      RMPU.mmpuacdmac1()[7],
    ",
  0x40000404u64 => "
      RMPU.mmpusdmac1()[0],
    ",
  0x40000414u64 => "
      RMPU.mmpusdmac1()[1],
    ",
  0x40000424u64 => "
      RMPU.mmpusdmac1()[2],
    ",
  0x40000434u64 => "
      RMPU.mmpusdmac1()[3],
    ",
  0x40000444u64 => "
      RMPU.mmpusdmac1()[4],
    ",
  0x40000454u64 => "
      RMPU.mmpusdmac1()[5],
    ",
  0x40000464u64 => "
      RMPU.mmpusdmac1()[6],
    ",
  0x40000474u64 => "
      RMPU.mmpusdmac1()[7],
    ",
  0x40000408u64 => "
      RMPU.mmpuedmac1()[0],
    ",
  0x40000418u64 => "
      RMPU.mmpuedmac1()[1],
    ",
  0x40000428u64 => "
      RMPU.mmpuedmac1()[2],
    ",
  0x40000438u64 => "
      RMPU.mmpuedmac1()[3],
    ",
  0x40000448u64 => "
      RMPU.mmpuedmac1()[4],
    ",
  0x40000458u64 => "
      RMPU.mmpuedmac1()[5],
    ",
  0x40000468u64 => "
      RMPU.mmpuedmac1()[6],
    ",
  0x40000478u64 => "
      RMPU.mmpuedmac1()[7],
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
  0x40000640u64 => "
      RMPU.mmpuacedmac()[4],
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
  0x40000644u64 => "
      RMPU.mmpusedmac()[4],
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
  0x40000648u64 => "
      RMPU.mmpueedmac()[4],
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
  0x40000b00u64 => "
      RMPU.mmpuenmipid(),
    ",
  0x40000b04u64 => "
      RMPU.mmpuenptmipid(),
    ",
  0x40000b08u64 => "
      RMPU.mmpurptmipid(),
    ",
  0x40000c00u64 => "
      RMPU.mmpuacmipid(),
    ",
  0x40000c04u64 => "
      RMPU.mmpusmipid(),
    ",
  0x40000c08u64 => "
      RMPU.mmpuemipid(),
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
  0x40000f00u64 => "
      RMPU.mmpuenmipic(),
    ",
  0x40000f04u64 => "
      RMPU.mmpuenptmipic(),
    ",
  0x40000f08u64 => "
      RMPU.mmpurptmipic(),
    ",
  0x40001000u64 => "
      RMPU.mmpuacmipic()[0],
    ",
  0x40001010u64 => "
      RMPU.mmpuacmipic()[1],
    ",
  0x40001020u64 => "
      RMPU.mmpuacmipic()[2],
    ",
  0x40001004u64 => "
      RMPU.mmpusmipic()[0],
    ",
  0x40001014u64 => "
      RMPU.mmpusmipic()[1],
    ",
  0x40001024u64 => "
      RMPU.mmpusmipic()[2],
    ",
  0x40001008u64 => "
      RMPU.mmpuemipic()[0],
    ",
  0x40001018u64 => "
      RMPU.mmpuemipic()[1],
    ",
  0x40001028u64 => "
      RMPU.mmpuemipic()[2],
    ",
  0x40001100u64 => "
      RMPU.mmpuennpu(),
    ",
  0x40001104u64 => "
      RMPU.mmpuenptnpu(),
    ",
  0x40001108u64 => "
      RMPU.mmpurptnpu(),
    ",
  0x40001200u64 => "
      RMPU.mmpuacnpu()[0],
    ",
  0x40001210u64 => "
      RMPU.mmpuacnpu()[1],
    ",
  0x40001220u64 => "
      RMPU.mmpuacnpu()[2],
    ",
  0x40001230u64 => "
      RMPU.mmpuacnpu()[3],
    ",
  0x40001240u64 => "
      RMPU.mmpuacnpu()[4],
    ",
  0x40001204u64 => "
      RMPU.mmpusnpu()[0],
    ",
  0x40001214u64 => "
      RMPU.mmpusnpu()[1],
    ",
  0x40001224u64 => "
      RMPU.mmpusnpu()[2],
    ",
  0x40001234u64 => "
      RMPU.mmpusnpu()[3],
    ",
  0x40001244u64 => "
      RMPU.mmpusnpu()[4],
    ",
  0x40001208u64 => "
      RMPU.mmpuenpu()[0],
    ",
  0x40001218u64 => "
      RMPU.mmpuenpu()[1],
    ",
  0x40001228u64 => "
      RMPU.mmpuenpu()[2],
    ",
  0x40001238u64 => "
      RMPU.mmpuenpu()[3],
    ",
  0x40001248u64 => "
      RMPU.mmpuenpu()[4],
    ",
  0x40002000u64 => "
      SRAM.sramprcr_s(),
    ",
  0x40002008u64 => "
      SRAM.sramwtsc(),
    ",
  0x40002010u64 => "
      SRAM.sramcr()[0],
    ",
  0x40002014u64 => "
      SRAM.sramcr()[1],
    ",
  0x40002018u64 => "
      SRAM.sramcr()[2],
    ",
  0x4000201cu64 => "
      SRAM.sramcr()[3],
    ",
  0x40002030u64 => "
      SRAM.srameccrgn0(),
    ",
  0x40002034u64 => "
      SRAM.srameccrgn1(),
    ",
  0x40002038u64 => "
      SRAM.srameccrgn2(),
    ",
  0x4000203cu64 => "
      SRAM.srameccrgn3(),
    ",
  0x40002040u64 => "
      SRAM.sramesr(),
    ",
  0x40002048u64 => "
      SRAM.sramesclr(),
    ",
  0x40002050u64 => "
      SRAM.sramear0()[0],
    ",
  0x40002060u64 => "
      SRAM.sramear0()[1],
    ",
  0x40002070u64 => "
      SRAM.sramear0()[2],
    ",
  0x40002080u64 => "
      SRAM.sramear0()[3],
    ",
  0x40002054u64 => "
      SRAM.sramear1()[0],
    ",
  0x40002064u64 => "
      SRAM.sramear1()[1],
    ",
  0x40002074u64 => "
      SRAM.sramear1()[2],
    ",
  0x40002084u64 => "
      SRAM.sramear1()[3],
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
      BUS.busmabtgraphbi(),
    ",
  0x40004200u64 => "
      BUS.bussabt1mrc0bi(),
    ",
  0x40004208u64 => "
      BUS.bussabt0mre0bi(),
      BUS.bussabt0s0bi(),
      BUS.bussabt0s1bi(),
      BUS.bussabt0s2bi(),
    ",
  0x40004238u64 => "
      BUS.bussabt0ecbi(),
      BUS.bussabt0ospi0bi(),
      BUS.bussabt0ospi1bi(),
      BUS.bussabt0s3bi(),
    ",
  0x40004268u64 => "
      BUS.bussabt0cpu0sahbi(),
      BUS.bussabt0cpu1tcmbi(),
      BUS.bussabt0pabi(),
      BUS.bussabt0pbbi(),
    ",
  0x40004288u64 => "
      BUS.bussabt0pibi(),
      BUS.bussabt0psbi(),
    ",
  0x40004a00u64 => "
      BUS.buserrstatcpu0(),
    ",
  0x40004a04u64 => "
      BUS.buserrclrcpu0(),
    ",
  0x40004a08u64 => "
      BUS.busirqencpu0(),
    ",
  0x40004a10u64 => "
      BUS.buserrstatcpu1(),
    ",
  0x40004a14u64 => "
      BUS.buserrclrcpu1(),
    ",
  0x40004a18u64 => "
      BUS.busirqencpu1(),
    ",
  0x40004a20u64 => "
      BUS.buserrstatdmac0(),
    ",
  0x40004a24u64 => "
      BUS.buserrclrdmac0(),
    ",
  0x40004a28u64 => "
      BUS.busirqendmac0(),
    ",
  0x40004a30u64 => "
      BUS.buserrstatdmac1(),
    ",
  0x40004a34u64 => "
      BUS.buserrclrdmac1(),
    ",
  0x40004a38u64 => "
      BUS.busirqendmac1(),
    ",
  0x40004a40u64 => "
      BUS.buserrstatnpu(),
    ",
  0x40004a44u64 => "
      BUS.buserrclrnpu(),
    ",
  0x40004a48u64 => "
      BUS.busirqennpu(),
    ",
  0x40004a50u64 => "
      BUS.buserrstatedmac(),
    ",
  0x40004a54u64 => "
      BUS.buserrclredmac(),
    ",
  0x40004a58u64 => "
      BUS.busirqenedmac(),
    ",
  0x40004a60u64 => "
      BUS.buserrstatglcdc(),
    ",
  0x40004a64u64 => "
      BUS.buserrclrglcdc(),
    ",
  0x40004a68u64 => "
      BUS.busirqenglcdc(),
    ",
  0x40004a70u64 => "
      BUS.buserrstattdrw(),
    ",
  0x40004a74u64 => "
      BUS.buserrclrtdrw(),
    ",
  0x40004a78u64 => "
      BUS.busirqentdrw(),
    ",
  0x40004a80u64 => "
      BUS.buserrstatmipi0(),
    ",
  0x40004a84u64 => "
      BUS.buserrclrmipi0(),
    ",
  0x40004a88u64 => "
      BUS.busirqenmipi0(),
    ",
  0x40004a90u64 => "
      BUS.buserrstatmipi1(),
    ",
  0x40004a94u64 => "
      BUS.buserrclrmipi1(),
    ",
  0x40004a98u64 => "
      BUS.busirqenmipi1(),
    ",
  0x40004aa0u64 => "
      BUS.buserrstatceu(),
    ",
  0x40004aa4u64 => "
      BUS.buserrclrceu(),
    ",
  0x40004aa8u64 => "
      BUS.busirqenceu(),
    ",
  0x40004b00u64 => "
      BUS.mbwerrstat(),
    ",
  0x40004b08u64 => "
      BUS.mbwerrclr(),
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
      ICU_COMMON.irqcr()[16],
      ICU_COMMON.nmicr(),
    ",
  0x40006011u64 => "
      ICU_COMMON.irqcr()[17],
    ",
  0x40006012u64 => "
      ICU_COMMON.irqcr()[18],
    ",
  0x40006013u64 => "
      ICU_COMMON.irqcr()[19],
    ",
  0x40006014u64 => "
      ICU_COMMON.irqcr()[20],
    ",
  0x40006015u64 => "
      ICU_COMMON.irqcr()[21],
    ",
  0x40006016u64 => "
      ICU_COMMON.irqcr()[22],
    ",
  0x40006017u64 => "
      ICU_COMMON.irqcr()[23],
    ",
  0x40006018u64 => "
      ICU_COMMON.irqcr()[24],
    ",
  0x40006019u64 => "
      ICU_COMMON.irqcr()[25],
    ",
  0x4000601au64 => "
      ICU_COMMON.irqcr()[26],
    ",
  0x4000601bu64 => "
      ICU_COMMON.irqcr()[27],
    ",
  0x4000601cu64 => "
      ICU_COMMON.irqcr()[28],
    ",
  0x4000601du64 => "
      ICU_COMMON.irqcr()[29],
    ",
  0x4000601eu64 => "
      ICU_COMMON.irqcr()[30],
    ",
  0x4000601fu64 => "
      ICU_COMMON.irqcr()[31],
    ",
  0x40006040u64 => "
      ICU_COMMON.intselr()[0],
    ",
  0x40006044u64 => "
      ICU_COMMON.intselr()[1],
    ",
  0x40006048u64 => "
      ICU_COMMON.intselr()[2],
    ",
  0x4000604cu64 => "
      ICU_COMMON.intselr()[3],
    ",
  0x40006050u64 => "
      ICU_COMMON.intselr()[4],
    ",
  0x40006054u64 => "
      ICU_COMMON.intselr()[5],
    ",
  0x40006058u64 => "
      ICU_COMMON.intselr()[6],
    ",
  0x4000605cu64 => "
      ICU_COMMON.intselr()[7],
    ",
  0x40006060u64 => "
      ICU_COMMON.intselr()[8],
    ",
  0x40006064u64 => "
      ICU_COMMON.intselr()[9],
    ",
  0x40006068u64 => "
      ICU_COMMON.intselr()[10],
    ",
  0x4000606cu64 => "
      ICU_COMMON.intselr()[11],
    ",
  0x40006070u64 => "
      ICU_COMMON.intselr()[12],
    ",
  0x40006074u64 => "
      ICU_COMMON.intselr()[13],
    ",
  0x40006078u64 => "
      ICU_COMMON.intselr()[14],
    ",
  0x4000607cu64 => "
      ICU_COMMON.intselr()[15],
    ",
  0x40006080u64 => "
      ICU_COMMON.intselr()[16],
    ",
  0x40006084u64 => "
      ICU_COMMON.intselr()[17],
    ",
  0x40006088u64 => "
      ICU_COMMON.intselr()[18],
    ",
  0x4000608cu64 => "
      ICU_COMMON.intselr()[19],
    ",
  0x40006090u64 => "
      ICU_COMMON.intselr()[20],
    ",
  0x40006094u64 => "
      ICU_COMMON.intselr()[21],
    ",
  0x40006098u64 => "
      ICU_COMMON.intselr()[22],
    ",
  0x4000609cu64 => "
      ICU_COMMON.intselr()[23],
    ",
  0x400060a0u64 => "
      ICU_COMMON.intselr()[24],
    ",
  0x400060a4u64 => "
      ICU_COMMON.intselr()[25],
    ",
  0x400060a8u64 => "
      ICU_COMMON.intselr()[26],
    ",
  0x400060acu64 => "
      ICU_COMMON.intselr()[27],
    ",
  0x400060b0u64 => "
      ICU_COMMON.intselr()[28],
    ",
  0x400060b4u64 => "
      ICU_COMMON.intselr()[29],
    ",
  0x400060b8u64 => "
      ICU_COMMON.intselr()[30],
    ",
  0x400060bcu64 => "
      ICU_COMMON.intselr()[31],
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
  0x4000807cu64 => "
      CPSCU.icusarj(),
    ",
  0x40008080u64 => "
      CPSCU.icusark(),
    ",
  0x40008084u64 => "
      CPSCU.icusarl(),
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
  0x4000840cu64 => "
      CPSCU.sramsabar()[3],
    ",
  0x40008500u64 => "
      CPSCU.cachesar(),
    ",
  0x40008504u64 => "
      CPSCU.tcmsar(),
    ",
  0x40008508u64 => "
      CPSCU.tcmsabarc(),
      CPSCU.tcmsabars(),
    ",
  0x40008510u64 => "
      CPSCU.sramesar(),
    ",
  0x40008600u64 => "
      CPSCU.tevtrcr(),
    ",
  0x40008610u64 => "
      CPSCU.ipcsar(),
    ",
  0x40008614u64 => "
      CPSCU.ipcpar(),
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
  0x4000a810u64 => "
      DMA_0.dmctl(),
    ",
  0x4000a840u64 => "
      DMA_0.dmechr(),
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
  0x4000ac18u64 => "
      DTC_0.dtcdisp(),
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
  0x4000c214u64 => "
      ICU.dslpwupirqen0(),
    ",
  0x4000c218u64 => "
      ICU.dslpwupirqen1(),
    ",
  0x4000c21cu64 => "
      ICU.dslpwupirqen2(),
    ",
  0x4000c280u64 => "
      ICU.delsrm(),
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
      CPU_CTRL.cpulckupcr()[0],
    ",
  0x4000f034u64 => "
      CPU_CTRL.cpulckupcr()[1],
    ",
  0x4000f040u64 => "
      CPU_CTRL.cpuinitvtor()[0],
    ",
  0x4000f044u64 => "
      CPU_CTRL.cpuinitvtor()[1],
    ",
  0x4000f050u64 => "
      CPU_CTRL.cpuwaitcr()[0],
    ",
  0x4000f054u64 => "
      CPU_CTRL.cpuwaitcr()[1],
    ",
  0x4000f060u64 => "
      CPU_CTRL.cpuactcsr()[0],
    ",
  0x4000f064u64 => "
      CPU_CTRL.cpuactcsr()[1],
    ",
  0x4000f070u64 => "
      CPU_CTRL.cpu0lmecr(),
    ",
  0x4000f078u64 => "
      CPU_CTRL.cpuidr(),
    ",
  0x4000f080u64 => "
      CPU_CTRL.cpu0statm(),
    ",
  0x4000f084u64 => "
      CPU_CTRL.cpu1statm(),
    ",
  0x4000f090u64 => "
      CPU_CTRL.secextmon(),
    ",
  0x4000f094u64 => "
      CPU_CTRL.nscpucr(),
    ",
  0x4000f400u64 => "
      CPU_CTRL.cpu0lockcr(),
    ",
  0x4000f404u64 => "
      CPU_CTRL.cpu1lockcr(),
    ",
  0x4000f840u64 => "
      CPU_CTRL.cpucrpt()[0],
    ",
  0x4000f844u64 => "
      CPU_CTRL.cpucrpt()[1],
    ",
  0x40011000u64 => "
      CPU_OCD.mcuerrstat(),
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
  0x4001b030u64 => "
      CPU_DBG.trportcr(),
    ",
  0x4001b038u64 => "
      CPU_DBG.trportsz(),
    ",
  0x4001b040u64 => "
      CPU_DBG.cachedbgcr(),
    ",
  0x4001b050u64 => "
      CPU_DBG.dbgnvmcr(),
    ",
  0x4001b100u64 => "
      CPU_DBG.alctrl(),
    ",
  0x4001b200u64 => "
      CPU_DBG.fsblstat(),
    ",
  0x4001c000u64 => "
      CACHE.ccactl(),
    ",
  0x4001c004u64 => "
      CACHE.ccafct(),
    ",
  0x4001c00cu64 => "
      CACHE.ccawta(),
    ",
  0x4001c010u64 => "
      CACHE.ccaedst(),
    ",
  0x4001c014u64 => "
      CACHE.ccataa(),
    ",
  0x4001c018u64 => "
      CACHE.ccatad_data(),
      CACHE.ccatad_ecc(),
      CACHE.ccatad_lru(),
      CACHE.ccatad_tag(),
      CACHE.ccatad_tagecc(),
    ",
  0x4001c040u64 => "
      CACHE.scactl(),
    ",
  0x4001c044u64 => "
      CACHE.scafct(),
    ",
  0x4001c04cu64 => "
      CACHE.scawta(),
    ",
  0x4001c050u64 => "
      CACHE.scaedst(),
    ",
  0x4001c054u64 => "
      CACHE.scataa(),
    ",
  0x4001c058u64 => "
      CACHE.scatad_data(),
      CACHE.scatad_ecc(),
      CACHE.scatad_lru(),
      CACHE.scatad_tag(),
    ",
  0x4001c200u64 => "
      CACHE.capoad(),
    ",
  0x4001c204u64 => "
      CACHE.caprcr(),
    ",
  0x4001c140u64 => "
      FCACHE.fsar(),
    ",
  0x4001c800u64 => "
      TCM.tcmprcr_s(),
    ",
  0x4001c810u64 => "
      TCM.tcmcrc(),
      TCM.tcmcrs(),
    ",
  0x4001c840u64 => "
      TCM.tcmesr(),
    ",
  0x4001c848u64 => "
      TCM.tcmesclr(),
    ",
  0x4001c850u64 => "
      TCM.tcmearc0(),
      TCM.tcmearc1(),
      TCM.tcmears0(),
      TCM.tcmears1(),
    ",
  0x4001e00cu64 => "
      SYSC.sbycr(),
    ",
  0x4001e014u64 => "
      SYSC.vscr(),
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
  0x4001e084u64 => "
      SYSC.mocoscr(),
    ",
  0x4001e0a0u64 => "
      SYSC.opccr(),
    ",
  0x4001e0a2u64 => "
      SYSC.moscwtcr(),
    ",
  0x4001e0acu64 => "
      SYSC.pllccr(),
    ",
  0x4001e0c0u64 => "
      SYSC.rstsr1(),
    ",
  0x4001e0c8u64 => "
      SYSC.pll2ccr(),
    ",
  0x4001e0ccu64 => "
      SYSC.syraccr(),
    ",
  0x4001e0d4u64 => "
      SYSC.bckadivcr(),
    ",
  0x4001e0d5u64 => "
      SYSC.eswckdivcr(),
    ",
  0x4001e0d6u64 => "
      SYSC.eswpckdivcr(),
    ",
  0x4001e0d8u64 => "
      SYSC.ethpckdivcr(),
    ",
  0x4001e0dau64 => "
      SYSC.bckacr(),
    ",
  0x4001e0dbu64 => "
      SYSC.eswckcr(),
    ",
  0x4001e0dcu64 => "
      SYSC.eswpckcr(),
    ",
  0x4001e0deu64 => "
      SYSC.ethpckcr(),
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
  0x4001e100u64 => "
      SYSC.cpudscr(),
    ",
  0x4001e104u64 => "
      SYSC.pgscr(),
    ",
  0x4001e110u64 => "
      SYSC.pdctrgd(),
    ",
  0x4001e114u64 => "
      SYSC.pdctrnpu(),
    ",
  0x4001e118u64 => "
      SYSC.pdctreswm(),
    ",
  0x4001e140u64 => "
      SYSC.pdramscr0(),
    ",
  0x4001e142u64 => "
      SYSC.pdramscr1(),
    ",
  0x4001e210u64 => "
      SYSC.psstcr()[0],
    ",
  0x4001e212u64 => "
      SYSC.psstcr()[1],
    ",
  0x4001e214u64 => "
      SYSC.psstcr()[2],
    ",
  0x4001e216u64 => "
      SYSC.psstcr()[3],
    ",
  0x4001e218u64 => "
      SYSC.psstcr()[4],
    ",
  0x4001e21au64 => "
      SYSC.psstcr()[5],
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
  0x4001e3e8u64 => "
      SYSC.dpfsar1(),
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
  0x4001ea34u64 => "
      SYSC.dpsiegr3(),
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
  0x4001ea64u64 => "
      SYSC.pvdcmpcr()[0],
    ",
  0x4001ea68u64 => "
      SYSC.pvdcmpcr()[1],
    ",
  0x4001ea7cu64 => "
      SYSC.pvdcr0()[0],
    ",
  0x4001ea80u64 => "
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
  0x4001ea9cu64 => "
      SYSC.svscr(),
    ",
  0x4001eab0u64 => "
      SYSC.lvocr(),
    ",
  0x4001eab4u64 => "
      SYSC.mwmcr(),
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
  0x4001eadcu64 => "
      SYSC.temprcr(),
    ",
  0x4001eae0u64 => "
      SYSC.temprlr(),
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
  0x4001eb2cu64 => "
      SYSC.pvdfcr()[0],
    ",
  0x4001eb30u64 => "
      SYSC.pvdfcr()[1],
    ",
  0x4001eb34u64 => "
      SYSC.pvdlr(),
    ",
  0x4001eb40u64 => "
      SYSC.dpsier4(),
    ",
  0x4001eb44u64 => "
      SYSC.dpsier5(),
    ",
  0x4001eb48u64 => "
      SYSC.dpsifr4(),
    ",
  0x4001eb4cu64 => "
      SYSC.dpsifr5(),
    ",
  0x4001eb50u64 => "
      SYSC.dpsiegr4(),
    ",
  0x4001ec00u64 => "
      SYSC.sosccr(),
    ",
  0x4001ec01u64 => "
      SYSC.somcr(),
    ",
  0x4001ec04u64 => "
      SYSC.sostdcr(),
    ",
  0x4001ec05u64 => "
      SYSC.sostdsr(),
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
  0x4001ec50u64 => "
      SYSC.vbtncwcr(),
    ",
  0x4001ec54u64 => "
      SYSC.vbtadcr3(),
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
  0x40020000u64 => "
      IPC.ipcsem()[0],
    ",
  0x40020004u64 => "
      IPC.ipcsem()[1],
    ",
  0x40020008u64 => "
      IPC.ipcsem()[2],
    ",
  0x4002000cu64 => "
      IPC.ipcsem()[3],
    ",
  0x40020010u64 => "
      IPC.ipcsem()[4],
    ",
  0x40020014u64 => "
      IPC.ipcsem()[5],
    ",
  0x40020018u64 => "
      IPC.ipcsem()[6],
    ",
  0x4002001cu64 => "
      IPC.ipcsem()[7],
    ",
  0x40020020u64 => "
      IPC.ipcsem()[8],
    ",
  0x40020024u64 => "
      IPC.ipcsem()[9],
    ",
  0x40020028u64 => "
      IPC.ipcsem()[10],
    ",
  0x4002002cu64 => "
      IPC.ipcsem()[11],
    ",
  0x40020030u64 => "
      IPC.ipcsem()[12],
    ",
  0x40020034u64 => "
      IPC.ipcsem()[13],
    ",
  0x40020038u64 => "
      IPC.ipcsem()[14],
    ",
  0x4002003cu64 => "
      IPC.ipcsem()[15],
    ",
  0x40020080u64 => "
      IPC.ipc0nmista(),
    ",
  0x40020084u64 => "
      IPC.ipc0nmiset(),
    ",
  0x40020088u64 => "
      IPC.ipc0nmiclr(),
    ",
  0x40020090u64 => "
      IPC.ipc1nmista(),
    ",
  0x40020094u64 => "
      IPC.ipc1nmiset(),
    ",
  0x40020098u64 => "
      IPC.ipc1nmiclr(),
    ",
  0x400200c0u64 => "
      IPC.ipc0sta0(),
    ",
  0x400200c4u64 => "
      IPC.ipc0iset0(),
    ",
  0x400200c8u64 => "
      IPC.ipc0txd0(),
    ",
  0x400200ccu64 => "
      IPC.ipc0rxd0(),
    ",
  0x400200d0u64 => "
      IPC.ipc0clr0(),
    ",
  0x400200e0u64 => "
      IPC.ipc0sta1(),
    ",
  0x400200e4u64 => "
      IPC.ipc0iset1(),
    ",
  0x400200e8u64 => "
      IPC.ipc0txd1(),
    ",
  0x400200ecu64 => "
      IPC.ipc0rxd1(),
    ",
  0x400200f0u64 => "
      IPC.ipc0clr1(),
    ",
  0x40020100u64 => "
      IPC.ipc1sta0(),
    ",
  0x40020104u64 => "
      IPC.ipc1iset0(),
    ",
  0x40020108u64 => "
      IPC.ipc1txd0(),
    ",
  0x4002010cu64 => "
      IPC.ipc1rxd0(),
    ",
  0x40020110u64 => "
      IPC.ipc1clr0(),
    ",
  0x40020120u64 => "
      IPC.ipc1sta1(),
    ",
  0x40020124u64 => "
      IPC.ipc1iset1(),
    ",
  0x40020128u64 => "
      IPC.ipc1txd1(),
    ",
  0x4002012cu64 => "
      IPC.ipc1rxd1(),
    ",
  0x40020130u64 => "
      IPC.ipc1clr1(),
    ",
  0x4011e078u64 => "
      FACI.fbprot0(),
    ",
  0x4011e07cu64 => "
      FACI.fbprot1(),
    ",
  0x4013c000u64 => "
      MRAM.mrcpfb(),
    ",
  0x4013c004u64 => "
      MRAM.mrcfreq(),
    ",
  0x4013c008u64 => "
      MRAM.mrefreq(),
    ",
  0x4013c010u64 => "
      MRAM.mrcdecc(),
    ",
  0x4013c014u64 => "
      MRAM.mrcraeint(),
    ",
  0x4013c018u64 => "
      MRAM.mrcraes(),
    ",
  0x4013c01cu64 => "
      MRAM.mrcrtea(),
    ",
  0x4013c020u64 => "
      MRAM.mrcrdea(),
    ",
  0x4013c034u64 => "
      MRAM.mreraeint(),
    ",
  0x4013c038u64 => "
      MRAM.mreraes(),
    ",
  0x4013c03cu64 => "
      MRAM.mrertea(),
    ",
  0x4013c040u64 => "
      MRAM.mrerdea(),
    ",
  0x4013c100u64 => "
      MRAM.msar(),
    ",
  0x4013c400u64 => "
      MRAM.mrezs(),
    ",
  0x4013c404u64 => "
      MRAM.mrezc(),
    ",
  0x4013e010u64 => "
      MRAM.mastat(),
    ",
  0x4013e014u64 => "
      MRAM.mpaeint(),
    ",
  0x4013e018u64 => "
      MRAM.mrdyie(),
    ",
  0x4013e030u64 => "
      MRAM.msaddr(),
    ",
  0x4013e048u64 => "
      MRAM.mcntselr(),
    ",
  0x4013e04cu64 => "
      MRAM.mcntdtr()[0],
    ",
  0x4013e050u64 => "
      MRAM.mcntdtr()[1],
    ",
  0x4013e060u64 => "
      MRAM.mctrcntr(),
    ",
  0x4013e064u64 => "
      MRAM.mctrlsr(),
    ",
  0x4013e06cu64 => "
      MRAM.mctrstatr(),
    ",
  0x4013e080u64 => "
      MRAM.mstatr(),
    ",
  0x4013e084u64 => "
      MRAM.mentryr(),
    ",
  0x4013e08cu64 => "
      MRAM.msuinitr(),
    ",
  0x4013e0a0u64 => "
      MRAM.mcmdr(),
    ",
  0x4013e0dcu64 => "
      MRAM.msuasmon(),
    ",
  0x4013e0e8u64 => "
      MRAM.msuacr(),
    ",
  0x4013e800u64 => "
      MRAM.mrpsc(),
    ",
  0x4013f000u64 => "
      MRAM.mrcpc0(),
    ",
  0x4013f004u64 => "
      MRAM.mrcpc1(),
    ",
  0x4013f008u64 => "
      MRAM.mrcbprot0(),
    ",
  0x4013f00cu64 => "
      MRAM.mrcbprot1(),
    ",
  0x4013f010u64 => "
      MRAM.mrcps(),
    ",
  0x4013f014u64 => "
      MRAM.mrcpaeint(),
    ",
  0x4013f018u64 => "
      MRAM.mrcpea(),
    ",
  0x4013f030u64 => "
      MRAM.mrcflr(),
    ",
  0x4013f804u64 => "
      MRAM.mrceecc(),
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
  0x4020100cu64 => "
      ELC.elsegr()[2],
    ",
  0x40201010u64 => "
      ELC.elsegr()[3],
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
  0x40201068u64 => "
      ELC.elsr()[18],
    ",
  0x4020106cu64 => "
      ELC.elsr()[19],
    ",
  0x40201070u64 => "
      ELC.elsr()[20],
    ",
  0x40201074u64 => "
      ELC.elsr()[21],
    ",
  0x40201078u64 => "
      ELC.elsr()[22],
    ",
  0x4020107cu64 => "
      ELC.elsr()[23],
    ",
  0x40201080u64 => "
      ELC.elsr()[24],
    ",
  0x40201084u64 => "
      ELC.elsr()[25],
    ",
  0x40201088u64 => "
      ELC.elsr()[26],
    ",
  0x4020108cu64 => "
      ELC.elsr()[27],
    ",
  0x40201090u64 => "
      ELC.elsr()[28],
    ",
  0x40201094u64 => "
      ELC.elsr()[29],
    ",
  0x40201098u64 => "
      ELC.elsr()[30],
    ",
  0x4020109cu64 => "
      ELC.elsr()[31],
    ",
  0x402010a0u64 => "
      ELC.elsr()[32],
    ",
  0x402010a4u64 => "
      ELC.elsr()[33],
    ",
  0x402010a8u64 => "
      ELC.elsr()[34],
    ",
  0x402010acu64 => "
      ELC.elsr()[35],
    ",
  0x402010b0u64 => "
      ELC.elsr()[36],
    ",
  0x402010b4u64 => "
      ELC.elsr()[37],
    ",
  0x402010b8u64 => "
      ELC.elsr()[38],
    ",
  0x402010bcu64 => "
      ELC.elsr()[39],
    ",
  0x402010c0u64 => "
      ELC.elsr()[40],
    ",
  0x402010c4u64 => "
      ELC.elsr()[41],
    ",
  0x402010c8u64 => "
      ELC.elsr()[42],
    ",
  0x402010ccu64 => "
      ELC.elsr()[43],
    ",
  0x402010d0u64 => "
      ELC.elsr()[44],
    ",
  0x402010d4u64 => "
      ELC.elsr()[45],
    ",
  0x402010d8u64 => "
      ELC.elsr()[46],
    ",
  0x402010dcu64 => "
      ELC.elsr()[47],
    ",
  0x402010e0u64 => "
      ELC.elsr()[48],
    ",
  0x402010e4u64 => "
      ELC.elsr()[49],
    ",
  0x402010e8u64 => "
      ELC.elsr()[50],
    ",
  0x402010ecu64 => "
      ELC.elsr()[51],
    ",
  0x402010f0u64 => "
      ELC.elsr()[52],
    ",
  0x40201100u64 => "
      ELC.elcsara(),
    ",
  0x40201104u64 => "
      ELC.elcsarb(),
    ",
  0x40201108u64 => "
      ELC.elcsarc(),
    ",
  0x40201110u64 => "
      ELC.elcpara(),
    ",
  0x40201114u64 => "
      ELC.elcparb(),
    ",
  0x40201118u64 => "
      ELC.elcparc(),
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
      PSCU.cmsamon(),
    ",
  0x40204038u64 => "
      PSCU.dlmmon(),
    ",
  0x4020403cu64 => "
      PSCU.sfsamon(),
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
  0x40233000u64 => "
      DAC_120.dadr(),
    ",
  0x40233004u64 => "
      DAC_120.dacr0(),
    ",
  0x40233008u64 => "
      DAC_120.dacr1(),
    ",
  0x4023300cu64 => "
      DAC_120.dacr2(),
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
  0x40256000u64 => "
      PDMIF.pdcstrtr(),
    ",
  0x40256004u64 => "
      PDMIF.pdcstptr(),
    ",
  0x40256008u64 => "
      PDMIF.pdcchgtr(),
    ",
  0x4025600cu64 => "
      PDMIF.pdcicr(),
    ",
  0x40256010u64 => "
      PDMIF.pdcsr(),
    ",
  0x40256014u64 => "
      PDMIF.pdcscr(),
    ",
  0x40256020u64 => "
      PDMIF.pdcsdcr(),
    ",
  0x40256024u64 => "
      PDMIF.pdcdrcr(),
    ",
  0x40256028u64 => "
      PDMIF.pdcdcr(),
    ",
  0x40256080u64 => "
      PDMIF.pdvr(),
    ",
  0x40256100u64 => "
      PDMIF.pdstrtrch()[0],
    ",
  0x40256200u64 => "
      PDMIF.pdstrtrch()[1],
    ",
  0x40256300u64 => "
      PDMIF.pdstrtrch()[2],
    ",
  0x40256104u64 => "
      PDMIF.pdstptrch()[0],
    ",
  0x40256204u64 => "
      PDMIF.pdstptrch()[1],
    ",
  0x40256304u64 => "
      PDMIF.pdstptrch()[2],
    ",
  0x40256108u64 => "
      PDMIF.pdchgtrch()[0],
    ",
  0x40256208u64 => "
      PDMIF.pdchgtrch()[1],
    ",
  0x40256308u64 => "
      PDMIF.pdchgtrch()[2],
    ",
  0x4025610cu64 => "
      PDMIF.pdicrch()[0],
    ",
  0x4025620cu64 => "
      PDMIF.pdicrch()[1],
    ",
  0x4025630cu64 => "
      PDMIF.pdicrch()[2],
    ",
  0x40256110u64 => "
      PDMIF.pdsdcrch()[0],
    ",
  0x40256210u64 => "
      PDMIF.pdsdcrch()[1],
    ",
  0x40256310u64 => "
      PDMIF.pdsdcrch()[2],
    ",
  0x40256114u64 => "
      PDMIF.pdsrch()[0],
    ",
  0x40256214u64 => "
      PDMIF.pdsrch()[1],
    ",
  0x40256314u64 => "
      PDMIF.pdsrch()[2],
    ",
  0x40256118u64 => "
      PDMIF.pdscrch()[0],
    ",
  0x40256218u64 => "
      PDMIF.pdscrch()[1],
    ",
  0x40256318u64 => "
      PDMIF.pdscrch()[2],
    ",
  0x40256120u64 => "
      PDMIF.pdmdsrch()[0],
    ",
  0x40256220u64 => "
      PDMIF.pdmdsrch()[1],
    ",
  0x40256320u64 => "
      PDMIF.pdmdsrch()[2],
    ",
  0x40256124u64 => "
      PDMIF.pdsfcrch()[0],
    ",
  0x40256224u64 => "
      PDMIF.pdsfcrch()[1],
    ",
  0x40256324u64 => "
      PDMIF.pdsfcrch()[2],
    ",
  0x40256128u64 => "
      PDMIF.pdhfcs0rch()[0],
    ",
  0x40256228u64 => "
      PDMIF.pdhfcs0rch()[1],
    ",
  0x40256328u64 => "
      PDMIF.pdhfcs0rch()[2],
    ",
  0x4025612cu64 => "
      PDMIF.pdhfck1rch()[0],
    ",
  0x4025622cu64 => "
      PDMIF.pdhfck1rch()[1],
    ",
  0x4025632cu64 => "
      PDMIF.pdhfck1rch()[2],
    ",
  0x40256130u64 => "
      PDMIF.pdhfch0rch()[0],
    ",
  0x40256230u64 => "
      PDMIF.pdhfch0rch()[1],
    ",
  0x40256330u64 => "
      PDMIF.pdhfch0rch()[2],
    ",
  0x40256134u64 => "
      PDMIF.pdhfch1rch()[0],
    ",
  0x40256234u64 => "
      PDMIF.pdhfch1rch()[1],
    ",
  0x40256334u64 => "
      PDMIF.pdhfch1rch()[2],
    ",
  0x40256138u64 => "
      PDMIF.pdcfch00rch()[0],
    ",
  0x40256238u64 => "
      PDMIF.pdcfch00rch()[1],
    ",
  0x40256338u64 => "
      PDMIF.pdcfch00rch()[2],
    ",
  0x4025613cu64 => "
      PDMIF.pdcfch01rch()[0],
    ",
  0x4025623cu64 => "
      PDMIF.pdcfch01rch()[1],
    ",
  0x4025633cu64 => "
      PDMIF.pdcfch01rch()[2],
    ",
  0x40256140u64 => "
      PDMIF.pdcfch02rch()[0],
    ",
  0x40256240u64 => "
      PDMIF.pdcfch02rch()[1],
    ",
  0x40256340u64 => "
      PDMIF.pdcfch02rch()[2],
    ",
  0x40256144u64 => "
      PDMIF.pdcfch03rch()[0],
    ",
  0x40256244u64 => "
      PDMIF.pdcfch03rch()[1],
    ",
  0x40256344u64 => "
      PDMIF.pdcfch03rch()[2],
    ",
  0x40256148u64 => "
      PDMIF.pdcfch04rch()[0],
    ",
  0x40256248u64 => "
      PDMIF.pdcfch04rch()[1],
    ",
  0x40256348u64 => "
      PDMIF.pdcfch04rch()[2],
    ",
  0x4025614cu64 => "
      PDMIF.pdcfch05rch()[0],
    ",
  0x4025624cu64 => "
      PDMIF.pdcfch05rch()[1],
    ",
  0x4025634cu64 => "
      PDMIF.pdcfch05rch()[2],
    ",
  0x40256150u64 => "
      PDMIF.pdcfch06rchn(),
    ",
  0x40256154u64 => "
      PDMIF.pdcfch07rch()[0],
    ",
  0x40256254u64 => "
      PDMIF.pdcfch07rch()[1],
    ",
  0x40256354u64 => "
      PDMIF.pdcfch07rch()[2],
    ",
  0x40256158u64 => "
      PDMIF.pdcfch08rch()[0],
    ",
  0x40256258u64 => "
      PDMIF.pdcfch08rch()[1],
    ",
  0x40256358u64 => "
      PDMIF.pdcfch08rch()[2],
    ",
  0x4025615cu64 => "
      PDMIF.pdcfch09rch()[0],
    ",
  0x4025625cu64 => "
      PDMIF.pdcfch09rch()[1],
    ",
  0x4025635cu64 => "
      PDMIF.pdcfch09rch()[2],
    ",
  0x40256160u64 => "
      PDMIF.pdcfch10rch()[0],
    ",
  0x40256260u64 => "
      PDMIF.pdcfch10rch()[1],
    ",
  0x40256360u64 => "
      PDMIF.pdcfch10rch()[2],
    ",
  0x40256164u64 => "
      PDMIF.pdlfch010rch()[0],
    ",
  0x40256264u64 => "
      PDMIF.pdlfch010rch()[1],
    ",
  0x40256364u64 => "
      PDMIF.pdlfch010rch()[2],
    ",
  0x40256168u64 => "
      PDMIF.pdlfch100rch()[0],
    ",
  0x40256268u64 => "
      PDMIF.pdlfch100rch()[1],
    ",
  0x40256368u64 => "
      PDMIF.pdlfch100rch()[2],
    ",
  0x4025616cu64 => "
      PDMIF.pdlfch101rch()[0],
    ",
  0x4025626cu64 => "
      PDMIF.pdlfch101rch()[1],
    ",
  0x4025636cu64 => "
      PDMIF.pdlfch101rch()[2],
    ",
  0x40256170u64 => "
      PDMIF.pdlfch102rch()[0],
    ",
  0x40256270u64 => "
      PDMIF.pdlfch102rch()[1],
    ",
  0x40256370u64 => "
      PDMIF.pdlfch102rch()[2],
    ",
  0x40256174u64 => "
      PDMIF.pdlfch103rch()[0],
    ",
  0x40256274u64 => "
      PDMIF.pdlfch103rch()[1],
    ",
  0x40256374u64 => "
      PDMIF.pdlfch103rch()[2],
    ",
  0x40256178u64 => "
      PDMIF.pdlfch104rch()[0],
    ",
  0x40256278u64 => "
      PDMIF.pdlfch104rch()[1],
    ",
  0x40256378u64 => "
      PDMIF.pdlfch104rch()[2],
    ",
  0x4025617cu64 => "
      PDMIF.pdlfch105rch()[0],
    ",
  0x4025627cu64 => "
      PDMIF.pdlfch105rch()[1],
    ",
  0x4025637cu64 => "
      PDMIF.pdlfch105rch()[2],
    ",
  0x40256180u64 => "
      PDMIF.pdlfch106rch()[0],
    ",
  0x40256280u64 => "
      PDMIF.pdlfch106rch()[1],
    ",
  0x40256380u64 => "
      PDMIF.pdlfch106rch()[2],
    ",
  0x40256184u64 => "
      PDMIF.pdlfch107rch()[0],
    ",
  0x40256284u64 => "
      PDMIF.pdlfch107rch()[1],
    ",
  0x40256384u64 => "
      PDMIF.pdlfch107rch()[2],
    ",
  0x40256188u64 => "
      PDMIF.pdlfch108rch()[0],
    ",
  0x40256288u64 => "
      PDMIF.pdlfch108rch()[1],
    ",
  0x40256388u64 => "
      PDMIF.pdlfch108rch()[2],
    ",
  0x4025618cu64 => "
      PDMIF.pdlfch109rch()[0],
    ",
  0x4025628cu64 => "
      PDMIF.pdlfch109rch()[1],
    ",
  0x4025638cu64 => "
      PDMIF.pdlfch109rch()[2],
    ",
  0x40256190u64 => "
      PDMIF.pdlfch110rch()[0],
    ",
  0x40256290u64 => "
      PDMIF.pdlfch110rch()[1],
    ",
  0x40256390u64 => "
      PDMIF.pdlfch110rch()[2],
    ",
  0x40256194u64 => "
      PDMIF.pdlfch111rch()[0],
    ",
  0x40256294u64 => "
      PDMIF.pdlfch111rch()[1],
    ",
  0x40256394u64 => "
      PDMIF.pdlfch111rch()[2],
    ",
  0x40256198u64 => "
      PDMIF.pdlfch112rch()[0],
    ",
  0x40256298u64 => "
      PDMIF.pdlfch112rch()[1],
    ",
  0x40256398u64 => "
      PDMIF.pdlfch112rch()[2],
    ",
  0x4025619cu64 => "
      PDMIF.pdlfch113rch()[0],
    ",
  0x4025629cu64 => "
      PDMIF.pdlfch113rch()[1],
    ",
  0x4025639cu64 => "
      PDMIF.pdlfch113rch()[2],
    ",
  0x402561a0u64 => "
      PDMIF.pdlfch114rch()[0],
    ",
  0x402562a0u64 => "
      PDMIF.pdlfch114rch()[1],
    ",
  0x402563a0u64 => "
      PDMIF.pdlfch114rch()[2],
    ",
  0x402561a4u64 => "
      PDMIF.pdlfch115rch()[0],
    ",
  0x402562a4u64 => "
      PDMIF.pdlfch115rch()[1],
    ",
  0x402563a4u64 => "
      PDMIF.pdlfch115rch()[2],
    ",
  0x402561a8u64 => "
      PDMIF.pdlfch116rch()[0],
    ",
  0x402562a8u64 => "
      PDMIF.pdlfch116rch()[1],
    ",
  0x402563a8u64 => "
      PDMIF.pdlfch116rch()[2],
    ",
  0x402561acu64 => "
      PDMIF.pdlfch117rch()[0],
    ",
  0x402562acu64 => "
      PDMIF.pdlfch117rch()[1],
    ",
  0x402563acu64 => "
      PDMIF.pdlfch117rch()[2],
    ",
  0x402561b0u64 => "
      PDMIF.pdlfch118rch()[0],
    ",
  0x402562b0u64 => "
      PDMIF.pdlfch118rch()[1],
    ",
  0x402563b0u64 => "
      PDMIF.pdlfch118rch()[2],
    ",
  0x402561b4u64 => "
      PDMIF.pdlfch119rch()[0],
    ",
  0x402562b4u64 => "
      PDMIF.pdlfch119rch()[1],
    ",
  0x402563b4u64 => "
      PDMIF.pdlfch119rch()[2],
    ",
  0x402561b8u64 => "
      PDMIF.pdsdltrch()[0],
    ",
  0x402562b8u64 => "
      PDMIF.pdsdltrch()[1],
    ",
  0x402563b8u64 => "
      PDMIF.pdsdltrch()[2],
    ",
  0x402561bcu64 => "
      PDMIF.pdsdutrch()[0],
    ",
  0x402562bcu64 => "
      PDMIF.pdsdutrch()[1],
    ",
  0x402563bcu64 => "
      PDMIF.pdsdutrch()[2],
    ",
  0x402561c0u64 => "
      PDMIF.pddbcrch()[0],
    ",
  0x402562c0u64 => "
      PDMIF.pddbcrch()[1],
    ",
  0x402563c0u64 => "
      PDMIF.pddbcrch()[2],
    ",
  0x402561c4u64 => "
      PDMIF.pdsctsrch()[0],
    ",
  0x402562c4u64 => "
      PDMIF.pdsctsrch()[1],
    ",
  0x402563c4u64 => "
      PDMIF.pdsctsrch()[2],
    ",
  0x402561c8u64 => "
      PDMIF.pdovltrch()[0],
    ",
  0x402562c8u64 => "
      PDMIF.pdovltrch()[1],
    ",
  0x402563c8u64 => "
      PDMIF.pdovltrch()[2],
    ",
  0x402561ccu64 => "
      PDMIF.pdovutrch()[0],
    ",
  0x402562ccu64 => "
      PDMIF.pdovutrch()[1],
    ",
  0x402563ccu64 => "
      PDMIF.pdovutrch()[2],
    ",
  0x402561e0u64 => "
      PDMIF.pddrcrch()[0],
    ",
  0x402562e0u64 => "
      PDMIF.pddrcrch()[1],
    ",
  0x402563e0u64 => "
      PDMIF.pddrcrch()[2],
    ",
  0x402561e4u64 => "
      PDMIF.pddcrch()[0],
    ",
  0x402562e4u64 => "
      PDMIF.pddcrch()[1],
    ",
  0x402563e4u64 => "
      PDMIF.pddcrch()[2],
    ",
  0x402561e8u64 => "
      PDMIF.pddrrch()[0],
    ",
  0x402562e8u64 => "
      PDMIF.pddrrch()[1],
    ",
  0x402563e8u64 => "
      PDMIF.pddrrch()[2],
    ",
  0x402561ecu64 => "
      PDMIF.pddsrch()[0],
    ",
  0x402562ecu64 => "
      PDMIF.pddsrch()[1],
    ",
  0x402563ecu64 => "
      PDMIF.pddsrch()[2],
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
  0x40268800u64 => "
      DOTF_0.convareast(),
    ",
  0x40268804u64 => "
      DOTF_0.convaread(),
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
  0x40322044u64 => "
      GPT_320.gtitc(),
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
  0x4032206cu64 => "
      GPT_320.gtpdbr(),
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
  0x40322090u64 => "
      GPT_320.gtdvd(),
    ",
  0x40322094u64 => "
      GPT_320.gtdbu(),
    ",
  0x40322098u64 => "
      GPT_320.gtdbd(),
    ",
  0x4032209cu64 => "
      GPT_320.gtsos(),
    ",
  0x403220a0u64 => "
      GPT_320.gtsotr(),
    ",
  0x403220a4u64 => "
      GPT_320.gtadsmr(),
    ",
  0x403220a8u64 => "
      GPT_320.gteitc(),
    ",
  0x403220acu64 => "
      GPT_320.gteitli1(),
    ",
  0x403220b0u64 => "
      GPT_320.gteitli2(),
    ",
  0x403220b4u64 => "
      GPT_320.gteitlb(),
    ",
  0x403220b8u64 => "
      GPT_320.gticlf(),
    ",
  0x403220bcu64 => "
      GPT_320.gtpc(),
    ",
  0x403220c0u64 => "
      GPT_320.gtadcmsc(),
    ",
  0x403220c4u64 => "
      GPT_320.gtadcmss(),
    ",
  0x403220d0u64 => "
      GPT_320.gtsecsr(),
    ",
  0x403220d4u64 => "
      GPT_320.gtsecr(),
    ",
  0x403220e0u64 => "
      GPT_320.gtber2(),
    ",
  0x403220e4u64 => "
      GPT_320.gtolbr(),
    ",
  0x403220ecu64 => "
      GPT_320.gticcr(),
    ",
  0x40323f00u64 => "
      GPT_OPS.opscr(),
    ",
  0x40323f10u64 => "
      GPT_GTCLK.gtclkcr(),
    ",
  0x40324000u64 => "
      PDG.gtdlycr(),
    ",
  0x40324002u64 => "
      PDG.gtdlycr2(),
    ",
  0x40324018u64 => "
      PDG.gtdlyra()[0],
    ",
  0x4032401cu64 => "
      PDG.gtdlyra()[1],
    ",
  0x40324020u64 => "
      PDG.gtdlyra()[2],
    ",
  0x40324024u64 => "
      PDG.gtdlyra()[3],
    ",
  0x4032401au64 => "
      PDG.gtdlyrb()[0],
    ",
  0x4032401eu64 => "
      PDG.gtdlyrb()[1],
    ",
  0x40324022u64 => "
      PDG.gtdlyrb()[2],
    ",
  0x40324026u64 => "
      PDG.gtdlyrb()[3],
    ",
  0x40324028u64 => "
      PDG.gtdlyfa()[0],
    ",
  0x4032402cu64 => "
      PDG.gtdlyfa()[1],
    ",
  0x40324030u64 => "
      PDG.gtdlyfa()[2],
    ",
  0x40324034u64 => "
      PDG.gtdlyfa()[3],
    ",
  0x4032402au64 => "
      PDG.gtdlyfb()[0],
    ",
  0x4032402eu64 => "
      PDG.gtdlyfb()[1],
    ",
  0x40324032u64 => "
      PDG.gtdlyfb()[2],
    ",
  0x40324036u64 => "
      PDG.gtdlyfb()[3],
    ",
  0x40338000u64 => "
      ADC_B.adclkenr(),
    ",
  0x40338004u64 => "
      ADC_B.adclksr(),
    ",
  0x40338008u64 => "
      ADC_B.adclkcr(),
    ",
  0x4033800cu64 => "
      ADC_B.adsycr(),
    ",
  0x40338010u64 => "
      ADC_B.aduslpcr0(),
    ",
  0x40338014u64 => "
      ADC_B.aduslpcr1(),
    ",
  0x40338020u64 => "
      ADC_B.aderintcr(),
    ",
  0x40338024u64 => "
      ADC_B.adovfintcr(),
    ",
  0x40338028u64 => "
      ADC_B.adcalintcr(),
    ",
  0x40338040u64 => "
      ADC_B.admdr(),
    ",
  0x40338044u64 => "
      ADC_B.adgspcr(),
    ",
  0x40338048u64 => "
      ADC_B.adsger(),
    ",
  0x4033804cu64 => "
      ADC_B.adsgcr0(),
    ",
  0x40338050u64 => "
      ADC_B.adsgcr1(),
    ",
  0x40338054u64 => "
      ADC_B.adsgcr2(),
    ",
  0x4033805cu64 => "
      ADC_B.adintcr(),
    ",
  0x40338060u64 => "
      ADC_B.adswnr0(),
    ",
  0x40338064u64 => "
      ADC_B.adswnr1(),
    ",
  0x40338080u64 => "
      ADC_B.addeccr(),
    ",
  0x40338084u64 => "
      ADC_B.adacmdr(),
    ",
  0x403380c0u64 => "
      ADC_B.adtrgext()[0],
    ",
  0x403380d0u64 => "
      ADC_B.adtrgext()[1],
    ",
  0x403380e0u64 => "
      ADC_B.adtrgext()[2],
    ",
  0x403380f0u64 => "
      ADC_B.adtrgext()[3],
    ",
  0x40338100u64 => "
      ADC_B.adtrgext()[4],
    ",
  0x40338110u64 => "
      ADC_B.adtrgext()[5],
    ",
  0x40338120u64 => "
      ADC_B.adtrgext()[6],
    ",
  0x40338130u64 => "
      ADC_B.adtrgext()[7],
    ",
  0x40338140u64 => "
      ADC_B.adtrgext()[8],
    ",
  0x403380c4u64 => "
      ADC_B.adtrgelc()[0],
    ",
  0x403380d4u64 => "
      ADC_B.adtrgelc()[1],
    ",
  0x403380e4u64 => "
      ADC_B.adtrgelc()[2],
    ",
  0x403380f4u64 => "
      ADC_B.adtrgelc()[3],
    ",
  0x40338104u64 => "
      ADC_B.adtrgelc()[4],
    ",
  0x40338114u64 => "
      ADC_B.adtrgelc()[5],
    ",
  0x40338124u64 => "
      ADC_B.adtrgelc()[6],
    ",
  0x40338134u64 => "
      ADC_B.adtrgelc()[7],
    ",
  0x40338144u64 => "
      ADC_B.adtrgelc()[8],
    ",
  0x403380c8u64 => "
      ADC_B.adtrggpt()[0],
    ",
  0x403380d8u64 => "
      ADC_B.adtrggpt()[1],
    ",
  0x403380e8u64 => "
      ADC_B.adtrggpt()[2],
    ",
  0x403380f8u64 => "
      ADC_B.adtrggpt()[3],
    ",
  0x40338108u64 => "
      ADC_B.adtrggpt()[4],
    ",
  0x40338118u64 => "
      ADC_B.adtrggpt()[5],
    ",
  0x40338128u64 => "
      ADC_B.adtrggpt()[6],
    ",
  0x40338138u64 => "
      ADC_B.adtrggpt()[7],
    ",
  0x40338148u64 => "
      ADC_B.adtrggpt()[8],
    ",
  0x403381c0u64 => "
      ADC_B.adtrgdlr0(),
    ",
  0x403381c4u64 => "
      ADC_B.adtrgdlr1(),
    ",
  0x403381c8u64 => "
      ADC_B.adtrgdlr2(),
    ",
  0x403381ccu64 => "
      ADC_B.adtrgdlr3(),
    ",
  0x403381d0u64 => "
      ADC_B.adtrgdlr4(),
    ",
  0x40338200u64 => "
      ADC_B.adsgdcr()[0],
    ",
  0x40338204u64 => "
      ADC_B.adsgdcr()[1],
    ",
  0x40338208u64 => "
      ADC_B.adsgdcr()[2],
    ",
  0x4033820cu64 => "
      ADC_B.adsgdcr()[3],
    ",
  0x40338210u64 => "
      ADC_B.adsgdcr()[4],
    ",
  0x40338214u64 => "
      ADC_B.adsgdcr()[5],
    ",
  0x40338218u64 => "
      ADC_B.adsgdcr()[6],
    ",
  0x4033821cu64 => "
      ADC_B.adsgdcr()[7],
    ",
  0x40338220u64 => "
      ADC_B.adsgdcr()[8],
    ",
  0x40338240u64 => "
      ADC_B.adsstr0(),
    ",
  0x40338244u64 => "
      ADC_B.adsstr1(),
    ",
  0x40338248u64 => "
      ADC_B.adsstr2(),
    ",
  0x4033824cu64 => "
      ADC_B.adsstr3(),
    ",
  0x40338250u64 => "
      ADC_B.adsstr4(),
    ",
  0x40338254u64 => "
      ADC_B.adsstr5(),
    ",
  0x40338258u64 => "
      ADC_B.adsstr6(),
    ",
  0x4033825cu64 => "
      ADC_B.adsstr7(),
    ",
  0x40338260u64 => "
      ADC_B.adcnvstr(),
    ",
  0x40338264u64 => "
      ADC_B.adcalstcr(),
    ",
  0x40338280u64 => "
      ADC_B.adshcr0(),
    ",
  0x40338284u64 => "
      ADC_B.adshdcr0(),
    ",
  0x40338288u64 => "
      ADC_B.adshstr0(),
    ",
  0x4033828cu64 => "
      ADC_B.adshcr1(),
    ",
  0x40338290u64 => "
      ADC_B.adshdcr1(),
    ",
  0x40338294u64 => "
      ADC_B.adshstr1(),
    ",
  0x403382b0u64 => "
      ADC_B.adcalshcr(),
    ",
  0x40338310u64 => "
      ADC_B.adshsbpcr(),
    ",
  0x40338314u64 => "
      ADC_B.adshdbpcr(),
    ",
  0x40338318u64 => "
      ADC_B.adshsdcr0(),
    ",
  0x4033831cu64 => "
      ADC_B.adshsdcr1(),
    ",
  0x40338320u64 => "
      ADC_B.adrefcr(),
    ",
  0x40338340u64 => "
      ADC_B.addfsr()[0],
    ",
  0x40338344u64 => "
      ADC_B.addfsr()[1],
    ",
  0x40338360u64 => "
      ADC_B.aduoftr()[0],
    ",
  0x40338364u64 => "
      ADC_B.aduoftr()[1],
    ",
  0x40338368u64 => "
      ADC_B.aduoftr()[2],
    ",
  0x4033836cu64 => "
      ADC_B.aduoftr()[3],
    ",
  0x40338370u64 => "
      ADC_B.aduoftr()[4],
    ",
  0x40338374u64 => "
      ADC_B.aduoftr()[5],
    ",
  0x40338378u64 => "
      ADC_B.aduoftr()[6],
    ",
  0x4033837cu64 => "
      ADC_B.aduoftr()[7],
    ",
  0x40338380u64 => "
      ADC_B.adugtr()[0],
    ",
  0x40338384u64 => "
      ADC_B.adugtr()[1],
    ",
  0x40338388u64 => "
      ADC_B.adugtr()[2],
    ",
  0x4033838cu64 => "
      ADC_B.adugtr()[3],
    ",
  0x40338390u64 => "
      ADC_B.adugtr()[4],
    ",
  0x40338394u64 => "
      ADC_B.adugtr()[5],
    ",
  0x40338398u64 => "
      ADC_B.adugtr()[6],
    ",
  0x4033839cu64 => "
      ADC_B.adugtr()[7],
    ",
  0x403383a0u64 => "
      ADC_B.adlimintcr(),
    ",
  0x403383a4u64 => "
      ADC_B.adlimtr()[0],
    ",
  0x403383a8u64 => "
      ADC_B.adlimtr()[1],
    ",
  0x403383acu64 => "
      ADC_B.adlimtr()[2],
    ",
  0x403383b0u64 => "
      ADC_B.adlimtr()[3],
    ",
  0x403383b4u64 => "
      ADC_B.adlimtr()[4],
    ",
  0x403383b8u64 => "
      ADC_B.adlimtr()[5],
    ",
  0x403383bcu64 => "
      ADC_B.adlimtr()[6],
    ",
  0x403383c0u64 => "
      ADC_B.adlimtr()[7],
    ",
  0x40338400u64 => "
      ADC_B.adcmpenr(),
    ",
  0x40338404u64 => "
      ADC_B.adcmpintcr(),
    ",
  0x40338408u64 => "
      ADC_B.adccmpcr()[0],
    ",
  0x4033840cu64 => "
      ADC_B.adccmpcr()[1],
    ",
  0x40338448u64 => "
      ADC_B.adcmpmdr0(),
    ",
  0x4033844cu64 => "
      ADC_B.adcmpmdr1(),
    ",
  0x40338458u64 => "
      ADC_B.adcmptbr()[0],
    ",
  0x4033845cu64 => "
      ADC_B.adcmptbr()[1],
    ",
  0x40338460u64 => "
      ADC_B.adcmptbr()[2],
    ",
  0x40338464u64 => "
      ADC_B.adcmptbr()[3],
    ",
  0x40338468u64 => "
      ADC_B.adcmptbr()[4],
    ",
  0x4033846cu64 => "
      ADC_B.adcmptbr()[5],
    ",
  0x40338470u64 => "
      ADC_B.adcmptbr()[6],
    ",
  0x40338474u64 => "
      ADC_B.adcmptbr()[7],
    ",
  0x403384c0u64 => "
      ADC_B.adfifocr(),
    ",
  0x403384c4u64 => "
      ADC_B.adfifointcr(),
    ",
  0x403384c8u64 => "
      ADC_B.adfifointlr0(),
    ",
  0x403384ccu64 => "
      ADC_B.adfifointlr1(),
    ",
  0x403384d0u64 => "
      ADC_B.adfifointlr2(),
    ",
  0x403384d4u64 => "
      ADC_B.adfifointlr3(),
    ",
  0x403384d8u64 => "
      ADC_B.adfifointlr4(),
    ",
  0x40338600u64 => "
      ADC_B.adchcr()[0],
    ",
  0x40338610u64 => "
      ADC_B.adchcr()[1],
    ",
  0x40338620u64 => "
      ADC_B.adchcr()[2],
    ",
  0x40338630u64 => "
      ADC_B.adchcr()[3],
    ",
  0x40338640u64 => "
      ADC_B.adchcr()[4],
    ",
  0x40338650u64 => "
      ADC_B.adchcr()[5],
    ",
  0x40338660u64 => "
      ADC_B.adchcr()[6],
    ",
  0x40338670u64 => "
      ADC_B.adchcr()[7],
    ",
  0x40338680u64 => "
      ADC_B.adchcr()[8],
    ",
  0x40338690u64 => "
      ADC_B.adchcr()[9],
    ",
  0x403386a0u64 => "
      ADC_B.adchcr()[10],
    ",
  0x403386b0u64 => "
      ADC_B.adchcr()[11],
    ",
  0x403386c0u64 => "
      ADC_B.adchcr()[12],
    ",
  0x403386d0u64 => "
      ADC_B.adchcr()[13],
    ",
  0x403386e0u64 => "
      ADC_B.adchcr()[14],
    ",
  0x403386f0u64 => "
      ADC_B.adchcr()[15],
    ",
  0x40338700u64 => "
      ADC_B.adchcr()[16],
    ",
  0x40338710u64 => "
      ADC_B.adchcr()[17],
    ",
  0x40338720u64 => "
      ADC_B.adchcr()[18],
    ",
  0x40338730u64 => "
      ADC_B.adchcr()[19],
    ",
  0x40338740u64 => "
      ADC_B.adchcr()[20],
    ",
  0x40338750u64 => "
      ADC_B.adchcr()[21],
    ",
  0x40338760u64 => "
      ADC_B.adchcr()[22],
    ",
  0x40338770u64 => "
      ADC_B.adchcr()[23],
    ",
  0x40338780u64 => "
      ADC_B.adchcr()[24],
    ",
  0x40338790u64 => "
      ADC_B.adchcr()[25],
    ",
  0x403387a0u64 => "
      ADC_B.adchcr()[26],
    ",
  0x403387b0u64 => "
      ADC_B.adchcr()[27],
    ",
  0x403387c0u64 => "
      ADC_B.adchcr()[28],
    ",
  0x403387d0u64 => "
      ADC_B.adchcr()[29],
    ",
  0x403387e0u64 => "
      ADC_B.adchcr()[30],
    ",
  0x403387f0u64 => "
      ADC_B.adchcr()[31],
    ",
  0x40338800u64 => "
      ADC_B.adchcr()[32],
    ",
  0x40338604u64 => "
      ADC_B.addopcra()[0],
    ",
  0x40338614u64 => "
      ADC_B.addopcra()[1],
    ",
  0x40338624u64 => "
      ADC_B.addopcra()[2],
    ",
  0x40338634u64 => "
      ADC_B.addopcra()[3],
    ",
  0x40338644u64 => "
      ADC_B.addopcra()[4],
    ",
  0x40338654u64 => "
      ADC_B.addopcra()[5],
    ",
  0x40338664u64 => "
      ADC_B.addopcra()[6],
    ",
  0x40338674u64 => "
      ADC_B.addopcra()[7],
    ",
  0x40338684u64 => "
      ADC_B.addopcra()[8],
    ",
  0x40338694u64 => "
      ADC_B.addopcra()[9],
    ",
  0x403386a4u64 => "
      ADC_B.addopcra()[10],
    ",
  0x403386b4u64 => "
      ADC_B.addopcra()[11],
    ",
  0x403386c4u64 => "
      ADC_B.addopcra()[12],
    ",
  0x403386d4u64 => "
      ADC_B.addopcra()[13],
    ",
  0x403386e4u64 => "
      ADC_B.addopcra()[14],
    ",
  0x403386f4u64 => "
      ADC_B.addopcra()[15],
    ",
  0x40338704u64 => "
      ADC_B.addopcra()[16],
    ",
  0x40338714u64 => "
      ADC_B.addopcra()[17],
    ",
  0x40338724u64 => "
      ADC_B.addopcra()[18],
    ",
  0x40338734u64 => "
      ADC_B.addopcra()[19],
    ",
  0x40338744u64 => "
      ADC_B.addopcra()[20],
    ",
  0x40338754u64 => "
      ADC_B.addopcra()[21],
    ",
  0x40338764u64 => "
      ADC_B.addopcra()[22],
    ",
  0x40338774u64 => "
      ADC_B.addopcra()[23],
    ",
  0x40338784u64 => "
      ADC_B.addopcra()[24],
    ",
  0x40338794u64 => "
      ADC_B.addopcra()[25],
    ",
  0x403387a4u64 => "
      ADC_B.addopcra()[26],
    ",
  0x403387b4u64 => "
      ADC_B.addopcra()[27],
    ",
  0x403387c4u64 => "
      ADC_B.addopcra()[28],
    ",
  0x403387d4u64 => "
      ADC_B.addopcra()[29],
    ",
  0x403387e4u64 => "
      ADC_B.addopcra()[30],
    ",
  0x403387f4u64 => "
      ADC_B.addopcra()[31],
    ",
  0x40338804u64 => "
      ADC_B.addopcra()[32],
    ",
  0x40338608u64 => "
      ADC_B.addopcrb()[0],
    ",
  0x40338618u64 => "
      ADC_B.addopcrb()[1],
    ",
  0x40338628u64 => "
      ADC_B.addopcrb()[2],
    ",
  0x40338638u64 => "
      ADC_B.addopcrb()[3],
    ",
  0x40338648u64 => "
      ADC_B.addopcrb()[4],
    ",
  0x40338658u64 => "
      ADC_B.addopcrb()[5],
    ",
  0x40338668u64 => "
      ADC_B.addopcrb()[6],
    ",
  0x40338678u64 => "
      ADC_B.addopcrb()[7],
    ",
  0x40338688u64 => "
      ADC_B.addopcrb()[8],
    ",
  0x40338698u64 => "
      ADC_B.addopcrb()[9],
    ",
  0x403386a8u64 => "
      ADC_B.addopcrb()[10],
    ",
  0x403386b8u64 => "
      ADC_B.addopcrb()[11],
    ",
  0x403386c8u64 => "
      ADC_B.addopcrb()[12],
    ",
  0x403386d8u64 => "
      ADC_B.addopcrb()[13],
    ",
  0x403386e8u64 => "
      ADC_B.addopcrb()[14],
    ",
  0x403386f8u64 => "
      ADC_B.addopcrb()[15],
    ",
  0x40338708u64 => "
      ADC_B.addopcrb()[16],
    ",
  0x40338718u64 => "
      ADC_B.addopcrb()[17],
    ",
  0x40338728u64 => "
      ADC_B.addopcrb()[18],
    ",
  0x40338738u64 => "
      ADC_B.addopcrb()[19],
    ",
  0x40338748u64 => "
      ADC_B.addopcrb()[20],
    ",
  0x40338758u64 => "
      ADC_B.addopcrb()[21],
    ",
  0x40338768u64 => "
      ADC_B.addopcrb()[22],
    ",
  0x40338778u64 => "
      ADC_B.addopcrb()[23],
    ",
  0x40338788u64 => "
      ADC_B.addopcrb()[24],
    ",
  0x40338798u64 => "
      ADC_B.addopcrb()[25],
    ",
  0x403387a8u64 => "
      ADC_B.addopcrb()[26],
    ",
  0x403387b8u64 => "
      ADC_B.addopcrb()[27],
    ",
  0x403387c8u64 => "
      ADC_B.addopcrb()[28],
    ",
  0x403387d8u64 => "
      ADC_B.addopcrb()[29],
    ",
  0x403387e8u64 => "
      ADC_B.addopcrb()[30],
    ",
  0x403387f8u64 => "
      ADC_B.addopcrb()[31],
    ",
  0x40338808u64 => "
      ADC_B.addopcrb()[32],
    ",
  0x4033860cu64 => "
      ADC_B.addopcrc()[0],
    ",
  0x4033861cu64 => "
      ADC_B.addopcrc()[1],
    ",
  0x4033862cu64 => "
      ADC_B.addopcrc()[2],
    ",
  0x4033863cu64 => "
      ADC_B.addopcrc()[3],
    ",
  0x4033864cu64 => "
      ADC_B.addopcrc()[4],
    ",
  0x4033865cu64 => "
      ADC_B.addopcrc()[5],
    ",
  0x4033866cu64 => "
      ADC_B.addopcrc()[6],
    ",
  0x4033867cu64 => "
      ADC_B.addopcrc()[7],
    ",
  0x4033868cu64 => "
      ADC_B.addopcrc()[8],
    ",
  0x4033869cu64 => "
      ADC_B.addopcrc()[9],
    ",
  0x403386acu64 => "
      ADC_B.addopcrc()[10],
    ",
  0x403386bcu64 => "
      ADC_B.addopcrc()[11],
    ",
  0x403386ccu64 => "
      ADC_B.addopcrc()[12],
    ",
  0x403386dcu64 => "
      ADC_B.addopcrc()[13],
    ",
  0x403386ecu64 => "
      ADC_B.addopcrc()[14],
    ",
  0x403386fcu64 => "
      ADC_B.addopcrc()[15],
    ",
  0x4033870cu64 => "
      ADC_B.addopcrc()[16],
    ",
  0x4033871cu64 => "
      ADC_B.addopcrc()[17],
    ",
  0x4033872cu64 => "
      ADC_B.addopcrc()[18],
    ",
  0x4033873cu64 => "
      ADC_B.addopcrc()[19],
    ",
  0x4033874cu64 => "
      ADC_B.addopcrc()[20],
    ",
  0x4033875cu64 => "
      ADC_B.addopcrc()[21],
    ",
  0x4033876cu64 => "
      ADC_B.addopcrc()[22],
    ",
  0x4033877cu64 => "
      ADC_B.addopcrc()[23],
    ",
  0x4033878cu64 => "
      ADC_B.addopcrc()[24],
    ",
  0x4033879cu64 => "
      ADC_B.addopcrc()[25],
    ",
  0x403387acu64 => "
      ADC_B.addopcrc()[26],
    ",
  0x403387bcu64 => "
      ADC_B.addopcrc()[27],
    ",
  0x403387ccu64 => "
      ADC_B.addopcrc()[28],
    ",
  0x403387dcu64 => "
      ADC_B.addopcrc()[29],
    ",
  0x403387ecu64 => "
      ADC_B.addopcrc()[30],
    ",
  0x403387fcu64 => "
      ADC_B.addopcrc()[31],
    ",
  0x4033880cu64 => "
      ADC_B.addopcrc()[32],
    ",
  0x40338c00u64 => "
      ADC_B.adcalstr(),
    ",
  0x40338c04u64 => "
      ADC_B.adshcscr(),
    ",
  0x40338c08u64 => "
      ADC_B.adtrgenr(),
    ",
  0x40338c10u64 => "
      ADC_B.adsystr(),
    ",
  0x40338c20u64 => "
      ADC_B.adstr()[0],
    ",
  0x40338c24u64 => "
      ADC_B.adstr()[1],
    ",
  0x40338c28u64 => "
      ADC_B.adstr()[2],
    ",
  0x40338c2cu64 => "
      ADC_B.adstr()[3],
    ",
  0x40338c30u64 => "
      ADC_B.adstr()[4],
    ",
  0x40338c34u64 => "
      ADC_B.adstr()[5],
    ",
  0x40338c38u64 => "
      ADC_B.adstr()[6],
    ",
  0x40338c3cu64 => "
      ADC_B.adstr()[7],
    ",
  0x40338c40u64 => "
      ADC_B.adstr()[8],
    ",
  0x40338c60u64 => "
      ADC_B.adstopr(),
    ",
  0x40338c80u64 => "
      ADC_B.adsr(),
    ",
  0x40338c84u64 => "
      ADC_B.adgrsr(),
    ",
  0x40338c88u64 => "
      ADC_B.adersr(),
    ",
  0x40338c8cu64 => "
      ADC_B.aderscr(),
    ",
  0x40338c98u64 => "
      ADC_B.adcalendsr(),
    ",
  0x40338c9cu64 => "
      ADC_B.adcalendscr(),
    ",
  0x40338ca0u64 => "
      ADC_B.adovfersr(),
    ",
  0x40338ca4u64 => "
      ADC_B.adovfchsr0(),
    ",
  0x40338cb0u64 => "
      ADC_B.adovfexsr(),
    ",
  0x40338cb4u64 => "
      ADC_B.adovferscr(),
    ",
  0x40338cb8u64 => "
      ADC_B.adovfchscr0(),
    ",
  0x40338cc4u64 => "
      ADC_B.adovfexscr(),
    ",
  0x40338cd0u64 => "
      ADC_B.adfifosr0(),
    ",
  0x40338cd4u64 => "
      ADC_B.adfifosr1(),
    ",
  0x40338cd8u64 => "
      ADC_B.adfifosr2(),
    ",
  0x40338cdcu64 => "
      ADC_B.adfifosr3(),
    ",
  0x40338ce0u64 => "
      ADC_B.adfifosr4(),
    ",
  0x40338cf0u64 => "
      ADC_B.adfifodcr(),
    ",
  0x40338cf4u64 => "
      ADC_B.adfifoersr(),
    ",
  0x40338cf8u64 => "
      ADC_B.adfifoerscr(),
    ",
  0x40338d00u64 => "
      ADC_B.adcmptbsr(),
    ",
  0x40338d04u64 => "
      ADC_B.adcmptbscr(),
    ",
  0x40338d08u64 => "
      ADC_B.adcmpchsr0(),
    ",
  0x40338d14u64 => "
      ADC_B.adcmpexsr(),
    ",
  0x40338d18u64 => "
      ADC_B.adcmpchscr0(),
    ",
  0x40338d24u64 => "
      ADC_B.adcmpexscr(),
    ",
  0x40338d28u64 => "
      ADC_B.adlimgrsr(),
    ",
  0x40338d2cu64 => "
      ADC_B.adlimchsr0(),
    ",
  0x40338d38u64 => "
      ADC_B.adlimexsr(),
    ",
  0x40338d3cu64 => "
      ADC_B.adlimgrscr(),
    ",
  0x40338d40u64 => "
      ADC_B.adlimchscr0(),
    ",
  0x40338d4cu64 => "
      ADC_B.adlimexscr(),
    ",
  0x40338d50u64 => "
      ADC_B.adscanendsr(),
    ",
  0x40338d54u64 => "
      ADC_B.adscanendscr(),
    ",
  0x4033a000u64 => "
      ADC_B.addr()[0],
    ",
  0x4033a004u64 => "
      ADC_B.addr()[1],
    ",
  0x4033a008u64 => "
      ADC_B.addr()[2],
    ",
  0x4033a00cu64 => "
      ADC_B.addr()[3],
    ",
  0x4033a010u64 => "
      ADC_B.addr()[4],
    ",
  0x4033a014u64 => "
      ADC_B.addr()[5],
    ",
  0x4033a018u64 => "
      ADC_B.addr()[6],
    ",
  0x4033a01cu64 => "
      ADC_B.addr()[7],
    ",
  0x4033a020u64 => "
      ADC_B.addr()[8],
    ",
  0x4033a024u64 => "
      ADC_B.addr()[9],
    ",
  0x4033a028u64 => "
      ADC_B.addr()[10],
    ",
  0x4033a02cu64 => "
      ADC_B.addr()[11],
    ",
  0x4033a030u64 => "
      ADC_B.addr()[12],
    ",
  0x4033a034u64 => "
      ADC_B.addr()[13],
    ",
  0x4033a038u64 => "
      ADC_B.addr()[14],
    ",
  0x4033a03cu64 => "
      ADC_B.addr()[15],
    ",
  0x4033a040u64 => "
      ADC_B.addr()[16],
    ",
  0x4033a044u64 => "
      ADC_B.addr()[17],
    ",
  0x4033a048u64 => "
      ADC_B.addr()[18],
    ",
  0x4033a04cu64 => "
      ADC_B.addr()[19],
    ",
  0x4033a050u64 => "
      ADC_B.addr()[20],
    ",
  0x4033a054u64 => "
      ADC_B.addr()[21],
    ",
  0x4033a058u64 => "
      ADC_B.addr()[22],
    ",
  0x4033a200u64 => "
      ADC_B.adfifodr()[0],
    ",
  0x4033a204u64 => "
      ADC_B.adfifodr()[1],
    ",
  0x4033a208u64 => "
      ADC_B.adfifodr()[2],
    ",
  0x4033a20cu64 => "
      ADC_B.adfifodr()[3],
    ",
  0x4033a210u64 => "
      ADC_B.adfifodr()[4],
    ",
  0x4033a214u64 => "
      ADC_B.adfifodr()[5],
    ",
  0x4033a218u64 => "
      ADC_B.adfifodr()[6],
    ",
  0x4033a21cu64 => "
      ADC_B.adfifodr()[7],
    ",
  0x4033a220u64 => "
      ADC_B.adfifodr()[8],
    ",
  0x40342000u64 => "
      GLCDC.gr1_clut0()[0],
      GLCDC.gr1_clut1()[0],
      GLCDC.gr2_clut0()[0],
      GLCDC.gr2_clut1()[0],
    ",
  0x40342004u64 => "
      GLCDC.gr1_clut0()[1],
      GLCDC.gr1_clut1()[1],
      GLCDC.gr2_clut0()[1],
      GLCDC.gr2_clut1()[1],
    ",
  0x40342008u64 => "
      GLCDC.gr1_clut0()[2],
      GLCDC.gr1_clut1()[2],
      GLCDC.gr2_clut0()[2],
      GLCDC.gr2_clut1()[2],
    ",
  0x4034200cu64 => "
      GLCDC.gr1_clut0()[3],
      GLCDC.gr1_clut1()[3],
      GLCDC.gr2_clut0()[3],
      GLCDC.gr2_clut1()[3],
    ",
  0x40342010u64 => "
      GLCDC.gr1_clut0()[4],
      GLCDC.gr1_clut1()[4],
      GLCDC.gr2_clut0()[4],
      GLCDC.gr2_clut1()[4],
    ",
  0x40342014u64 => "
      GLCDC.gr1_clut0()[5],
      GLCDC.gr1_clut1()[5],
      GLCDC.gr2_clut0()[5],
      GLCDC.gr2_clut1()[5],
    ",
  0x40342018u64 => "
      GLCDC.gr1_clut0()[6],
      GLCDC.gr1_clut1()[6],
      GLCDC.gr2_clut0()[6],
      GLCDC.gr2_clut1()[6],
    ",
  0x4034201cu64 => "
      GLCDC.gr1_clut0()[7],
      GLCDC.gr1_clut1()[7],
      GLCDC.gr2_clut0()[7],
      GLCDC.gr2_clut1()[7],
    ",
  0x40342020u64 => "
      GLCDC.gr1_clut0()[8],
      GLCDC.gr1_clut1()[8],
      GLCDC.gr2_clut0()[8],
      GLCDC.gr2_clut1()[8],
    ",
  0x40342024u64 => "
      GLCDC.gr1_clut0()[9],
      GLCDC.gr1_clut1()[9],
      GLCDC.gr2_clut0()[9],
      GLCDC.gr2_clut1()[9],
    ",
  0x40342028u64 => "
      GLCDC.gr1_clut0()[10],
      GLCDC.gr1_clut1()[10],
      GLCDC.gr2_clut0()[10],
      GLCDC.gr2_clut1()[10],
    ",
  0x4034202cu64 => "
      GLCDC.gr1_clut0()[11],
      GLCDC.gr1_clut1()[11],
      GLCDC.gr2_clut0()[11],
      GLCDC.gr2_clut1()[11],
    ",
  0x40342030u64 => "
      GLCDC.gr1_clut0()[12],
      GLCDC.gr1_clut1()[12],
      GLCDC.gr2_clut0()[12],
      GLCDC.gr2_clut1()[12],
    ",
  0x40342034u64 => "
      GLCDC.gr1_clut0()[13],
      GLCDC.gr1_clut1()[13],
      GLCDC.gr2_clut0()[13],
      GLCDC.gr2_clut1()[13],
    ",
  0x40342038u64 => "
      GLCDC.gr1_clut0()[14],
      GLCDC.gr1_clut1()[14],
      GLCDC.gr2_clut0()[14],
      GLCDC.gr2_clut1()[14],
    ",
  0x4034203cu64 => "
      GLCDC.gr1_clut0()[15],
      GLCDC.gr1_clut1()[15],
      GLCDC.gr2_clut0()[15],
      GLCDC.gr2_clut1()[15],
    ",
  0x40342040u64 => "
      GLCDC.gr1_clut0()[16],
      GLCDC.gr1_clut1()[16],
      GLCDC.gr2_clut0()[16],
      GLCDC.gr2_clut1()[16],
    ",
  0x40342044u64 => "
      GLCDC.gr1_clut0()[17],
      GLCDC.gr1_clut1()[17],
      GLCDC.gr2_clut0()[17],
      GLCDC.gr2_clut1()[17],
    ",
  0x40342048u64 => "
      GLCDC.gr1_clut0()[18],
      GLCDC.gr1_clut1()[18],
      GLCDC.gr2_clut0()[18],
      GLCDC.gr2_clut1()[18],
    ",
  0x4034204cu64 => "
      GLCDC.gr1_clut0()[19],
      GLCDC.gr1_clut1()[19],
      GLCDC.gr2_clut0()[19],
      GLCDC.gr2_clut1()[19],
    ",
  0x40342050u64 => "
      GLCDC.gr1_clut0()[20],
      GLCDC.gr1_clut1()[20],
      GLCDC.gr2_clut0()[20],
      GLCDC.gr2_clut1()[20],
    ",
  0x40342054u64 => "
      GLCDC.gr1_clut0()[21],
      GLCDC.gr1_clut1()[21],
      GLCDC.gr2_clut0()[21],
      GLCDC.gr2_clut1()[21],
    ",
  0x40342058u64 => "
      GLCDC.gr1_clut0()[22],
      GLCDC.gr1_clut1()[22],
      GLCDC.gr2_clut0()[22],
      GLCDC.gr2_clut1()[22],
    ",
  0x4034205cu64 => "
      GLCDC.gr1_clut0()[23],
      GLCDC.gr1_clut1()[23],
      GLCDC.gr2_clut0()[23],
      GLCDC.gr2_clut1()[23],
    ",
  0x40342060u64 => "
      GLCDC.gr1_clut0()[24],
      GLCDC.gr1_clut1()[24],
      GLCDC.gr2_clut0()[24],
      GLCDC.gr2_clut1()[24],
    ",
  0x40342064u64 => "
      GLCDC.gr1_clut0()[25],
      GLCDC.gr1_clut1()[25],
      GLCDC.gr2_clut0()[25],
      GLCDC.gr2_clut1()[25],
    ",
  0x40342068u64 => "
      GLCDC.gr1_clut0()[26],
      GLCDC.gr1_clut1()[26],
      GLCDC.gr2_clut0()[26],
      GLCDC.gr2_clut1()[26],
    ",
  0x4034206cu64 => "
      GLCDC.gr1_clut0()[27],
      GLCDC.gr1_clut1()[27],
      GLCDC.gr2_clut0()[27],
      GLCDC.gr2_clut1()[27],
    ",
  0x40342070u64 => "
      GLCDC.gr1_clut0()[28],
      GLCDC.gr1_clut1()[28],
      GLCDC.gr2_clut0()[28],
      GLCDC.gr2_clut1()[28],
    ",
  0x40342074u64 => "
      GLCDC.gr1_clut0()[29],
      GLCDC.gr1_clut1()[29],
      GLCDC.gr2_clut0()[29],
      GLCDC.gr2_clut1()[29],
    ",
  0x40342078u64 => "
      GLCDC.gr1_clut0()[30],
      GLCDC.gr1_clut1()[30],
      GLCDC.gr2_clut0()[30],
      GLCDC.gr2_clut1()[30],
    ",
  0x4034207cu64 => "
      GLCDC.gr1_clut0()[31],
      GLCDC.gr1_clut1()[31],
      GLCDC.gr2_clut0()[31],
      GLCDC.gr2_clut1()[31],
    ",
  0x40342080u64 => "
      GLCDC.gr1_clut0()[32],
      GLCDC.gr1_clut1()[32],
      GLCDC.gr2_clut0()[32],
      GLCDC.gr2_clut1()[32],
    ",
  0x40342084u64 => "
      GLCDC.gr1_clut0()[33],
      GLCDC.gr1_clut1()[33],
      GLCDC.gr2_clut0()[33],
      GLCDC.gr2_clut1()[33],
    ",
  0x40342088u64 => "
      GLCDC.gr1_clut0()[34],
      GLCDC.gr1_clut1()[34],
      GLCDC.gr2_clut0()[34],
      GLCDC.gr2_clut1()[34],
    ",
  0x4034208cu64 => "
      GLCDC.gr1_clut0()[35],
      GLCDC.gr1_clut1()[35],
      GLCDC.gr2_clut0()[35],
      GLCDC.gr2_clut1()[35],
    ",
  0x40342090u64 => "
      GLCDC.gr1_clut0()[36],
      GLCDC.gr1_clut1()[36],
      GLCDC.gr2_clut0()[36],
      GLCDC.gr2_clut1()[36],
    ",
  0x40342094u64 => "
      GLCDC.gr1_clut0()[37],
      GLCDC.gr1_clut1()[37],
      GLCDC.gr2_clut0()[37],
      GLCDC.gr2_clut1()[37],
    ",
  0x40342098u64 => "
      GLCDC.gr1_clut0()[38],
      GLCDC.gr1_clut1()[38],
      GLCDC.gr2_clut0()[38],
      GLCDC.gr2_clut1()[38],
    ",
  0x4034209cu64 => "
      GLCDC.gr1_clut0()[39],
      GLCDC.gr1_clut1()[39],
      GLCDC.gr2_clut0()[39],
      GLCDC.gr2_clut1()[39],
    ",
  0x403420a0u64 => "
      GLCDC.gr1_clut0()[40],
      GLCDC.gr1_clut1()[40],
      GLCDC.gr2_clut0()[40],
      GLCDC.gr2_clut1()[40],
    ",
  0x403420a4u64 => "
      GLCDC.gr1_clut0()[41],
      GLCDC.gr1_clut1()[41],
      GLCDC.gr2_clut0()[41],
      GLCDC.gr2_clut1()[41],
    ",
  0x403420a8u64 => "
      GLCDC.gr1_clut0()[42],
      GLCDC.gr1_clut1()[42],
      GLCDC.gr2_clut0()[42],
      GLCDC.gr2_clut1()[42],
    ",
  0x403420acu64 => "
      GLCDC.gr1_clut0()[43],
      GLCDC.gr1_clut1()[43],
      GLCDC.gr2_clut0()[43],
      GLCDC.gr2_clut1()[43],
    ",
  0x403420b0u64 => "
      GLCDC.gr1_clut0()[44],
      GLCDC.gr1_clut1()[44],
      GLCDC.gr2_clut0()[44],
      GLCDC.gr2_clut1()[44],
    ",
  0x403420b4u64 => "
      GLCDC.gr1_clut0()[45],
      GLCDC.gr1_clut1()[45],
      GLCDC.gr2_clut0()[45],
      GLCDC.gr2_clut1()[45],
    ",
  0x403420b8u64 => "
      GLCDC.gr1_clut0()[46],
      GLCDC.gr1_clut1()[46],
      GLCDC.gr2_clut0()[46],
      GLCDC.gr2_clut1()[46],
    ",
  0x403420bcu64 => "
      GLCDC.gr1_clut0()[47],
      GLCDC.gr1_clut1()[47],
      GLCDC.gr2_clut0()[47],
      GLCDC.gr2_clut1()[47],
    ",
  0x403420c0u64 => "
      GLCDC.gr1_clut0()[48],
      GLCDC.gr1_clut1()[48],
      GLCDC.gr2_clut0()[48],
      GLCDC.gr2_clut1()[48],
    ",
  0x403420c4u64 => "
      GLCDC.gr1_clut0()[49],
      GLCDC.gr1_clut1()[49],
      GLCDC.gr2_clut0()[49],
      GLCDC.gr2_clut1()[49],
    ",
  0x403420c8u64 => "
      GLCDC.gr1_clut0()[50],
      GLCDC.gr1_clut1()[50],
      GLCDC.gr2_clut0()[50],
      GLCDC.gr2_clut1()[50],
    ",
  0x403420ccu64 => "
      GLCDC.gr1_clut0()[51],
      GLCDC.gr1_clut1()[51],
      GLCDC.gr2_clut0()[51],
      GLCDC.gr2_clut1()[51],
    ",
  0x403420d0u64 => "
      GLCDC.gr1_clut0()[52],
      GLCDC.gr1_clut1()[52],
      GLCDC.gr2_clut0()[52],
      GLCDC.gr2_clut1()[52],
    ",
  0x403420d4u64 => "
      GLCDC.gr1_clut0()[53],
      GLCDC.gr1_clut1()[53],
      GLCDC.gr2_clut0()[53],
      GLCDC.gr2_clut1()[53],
    ",
  0x403420d8u64 => "
      GLCDC.gr1_clut0()[54],
      GLCDC.gr1_clut1()[54],
      GLCDC.gr2_clut0()[54],
      GLCDC.gr2_clut1()[54],
    ",
  0x403420dcu64 => "
      GLCDC.gr1_clut0()[55],
      GLCDC.gr1_clut1()[55],
      GLCDC.gr2_clut0()[55],
      GLCDC.gr2_clut1()[55],
    ",
  0x403420e0u64 => "
      GLCDC.gr1_clut0()[56],
      GLCDC.gr1_clut1()[56],
      GLCDC.gr2_clut0()[56],
      GLCDC.gr2_clut1()[56],
    ",
  0x403420e4u64 => "
      GLCDC.gr1_clut0()[57],
      GLCDC.gr1_clut1()[57],
      GLCDC.gr2_clut0()[57],
      GLCDC.gr2_clut1()[57],
    ",
  0x403420e8u64 => "
      GLCDC.gr1_clut0()[58],
      GLCDC.gr1_clut1()[58],
      GLCDC.gr2_clut0()[58],
      GLCDC.gr2_clut1()[58],
    ",
  0x403420ecu64 => "
      GLCDC.gr1_clut0()[59],
      GLCDC.gr1_clut1()[59],
      GLCDC.gr2_clut0()[59],
      GLCDC.gr2_clut1()[59],
    ",
  0x403420f0u64 => "
      GLCDC.gr1_clut0()[60],
      GLCDC.gr1_clut1()[60],
      GLCDC.gr2_clut0()[60],
      GLCDC.gr2_clut1()[60],
    ",
  0x403420f4u64 => "
      GLCDC.gr1_clut0()[61],
      GLCDC.gr1_clut1()[61],
      GLCDC.gr2_clut0()[61],
      GLCDC.gr2_clut1()[61],
    ",
  0x403420f8u64 => "
      GLCDC.gr1_clut0()[62],
      GLCDC.gr1_clut1()[62],
      GLCDC.gr2_clut0()[62],
      GLCDC.gr2_clut1()[62],
    ",
  0x403420fcu64 => "
      GLCDC.gr1_clut0()[63],
      GLCDC.gr1_clut1()[63],
      GLCDC.gr2_clut0()[63],
      GLCDC.gr2_clut1()[63],
    ",
  0x40342100u64 => "
      GLCDC.gr1_clut0()[64],
      GLCDC.gr1_clut1()[64],
      GLCDC.gr2_clut0()[64],
      GLCDC.gr2_clut1()[64],
    ",
  0x40342104u64 => "
      GLCDC.gr1_clut0()[65],
      GLCDC.gr1_clut1()[65],
      GLCDC.gr2_clut0()[65],
      GLCDC.gr2_clut1()[65],
    ",
  0x40342108u64 => "
      GLCDC.gr1_clut0()[66],
      GLCDC.gr1_clut1()[66],
      GLCDC.gr2_clut0()[66],
      GLCDC.gr2_clut1()[66],
    ",
  0x4034210cu64 => "
      GLCDC.gr1_clut0()[67],
      GLCDC.gr1_clut1()[67],
      GLCDC.gr2_clut0()[67],
      GLCDC.gr2_clut1()[67],
    ",
  0x40342110u64 => "
      GLCDC.gr1_clut0()[68],
      GLCDC.gr1_clut1()[68],
      GLCDC.gr2_clut0()[68],
      GLCDC.gr2_clut1()[68],
    ",
  0x40342114u64 => "
      GLCDC.gr1_clut0()[69],
      GLCDC.gr1_clut1()[69],
      GLCDC.gr2_clut0()[69],
      GLCDC.gr2_clut1()[69],
    ",
  0x40342118u64 => "
      GLCDC.gr1_clut0()[70],
      GLCDC.gr1_clut1()[70],
      GLCDC.gr2_clut0()[70],
      GLCDC.gr2_clut1()[70],
    ",
  0x4034211cu64 => "
      GLCDC.gr1_clut0()[71],
      GLCDC.gr1_clut1()[71],
      GLCDC.gr2_clut0()[71],
      GLCDC.gr2_clut1()[71],
    ",
  0x40342120u64 => "
      GLCDC.gr1_clut0()[72],
      GLCDC.gr1_clut1()[72],
      GLCDC.gr2_clut0()[72],
      GLCDC.gr2_clut1()[72],
    ",
  0x40342124u64 => "
      GLCDC.gr1_clut0()[73],
      GLCDC.gr1_clut1()[73],
      GLCDC.gr2_clut0()[73],
      GLCDC.gr2_clut1()[73],
    ",
  0x40342128u64 => "
      GLCDC.gr1_clut0()[74],
      GLCDC.gr1_clut1()[74],
      GLCDC.gr2_clut0()[74],
      GLCDC.gr2_clut1()[74],
    ",
  0x4034212cu64 => "
      GLCDC.gr1_clut0()[75],
      GLCDC.gr1_clut1()[75],
      GLCDC.gr2_clut0()[75],
      GLCDC.gr2_clut1()[75],
    ",
  0x40342130u64 => "
      GLCDC.gr1_clut0()[76],
      GLCDC.gr1_clut1()[76],
      GLCDC.gr2_clut0()[76],
      GLCDC.gr2_clut1()[76],
    ",
  0x40342134u64 => "
      GLCDC.gr1_clut0()[77],
      GLCDC.gr1_clut1()[77],
      GLCDC.gr2_clut0()[77],
      GLCDC.gr2_clut1()[77],
    ",
  0x40342138u64 => "
      GLCDC.gr1_clut0()[78],
      GLCDC.gr1_clut1()[78],
      GLCDC.gr2_clut0()[78],
      GLCDC.gr2_clut1()[78],
    ",
  0x4034213cu64 => "
      GLCDC.gr1_clut0()[79],
      GLCDC.gr1_clut1()[79],
      GLCDC.gr2_clut0()[79],
      GLCDC.gr2_clut1()[79],
    ",
  0x40342140u64 => "
      GLCDC.gr1_clut0()[80],
      GLCDC.gr1_clut1()[80],
      GLCDC.gr2_clut0()[80],
      GLCDC.gr2_clut1()[80],
    ",
  0x40342144u64 => "
      GLCDC.gr1_clut0()[81],
      GLCDC.gr1_clut1()[81],
      GLCDC.gr2_clut0()[81],
      GLCDC.gr2_clut1()[81],
    ",
  0x40342148u64 => "
      GLCDC.gr1_clut0()[82],
      GLCDC.gr1_clut1()[82],
      GLCDC.gr2_clut0()[82],
      GLCDC.gr2_clut1()[82],
    ",
  0x4034214cu64 => "
      GLCDC.gr1_clut0()[83],
      GLCDC.gr1_clut1()[83],
      GLCDC.gr2_clut0()[83],
      GLCDC.gr2_clut1()[83],
    ",
  0x40342150u64 => "
      GLCDC.gr1_clut0()[84],
      GLCDC.gr1_clut1()[84],
      GLCDC.gr2_clut0()[84],
      GLCDC.gr2_clut1()[84],
    ",
  0x40342154u64 => "
      GLCDC.gr1_clut0()[85],
      GLCDC.gr1_clut1()[85],
      GLCDC.gr2_clut0()[85],
      GLCDC.gr2_clut1()[85],
    ",
  0x40342158u64 => "
      GLCDC.gr1_clut0()[86],
      GLCDC.gr1_clut1()[86],
      GLCDC.gr2_clut0()[86],
      GLCDC.gr2_clut1()[86],
    ",
  0x4034215cu64 => "
      GLCDC.gr1_clut0()[87],
      GLCDC.gr1_clut1()[87],
      GLCDC.gr2_clut0()[87],
      GLCDC.gr2_clut1()[87],
    ",
  0x40342160u64 => "
      GLCDC.gr1_clut0()[88],
      GLCDC.gr1_clut1()[88],
      GLCDC.gr2_clut0()[88],
      GLCDC.gr2_clut1()[88],
    ",
  0x40342164u64 => "
      GLCDC.gr1_clut0()[89],
      GLCDC.gr1_clut1()[89],
      GLCDC.gr2_clut0()[89],
      GLCDC.gr2_clut1()[89],
    ",
  0x40342168u64 => "
      GLCDC.gr1_clut0()[90],
      GLCDC.gr1_clut1()[90],
      GLCDC.gr2_clut0()[90],
      GLCDC.gr2_clut1()[90],
    ",
  0x4034216cu64 => "
      GLCDC.gr1_clut0()[91],
      GLCDC.gr1_clut1()[91],
      GLCDC.gr2_clut0()[91],
      GLCDC.gr2_clut1()[91],
    ",
  0x40342170u64 => "
      GLCDC.gr1_clut0()[92],
      GLCDC.gr1_clut1()[92],
      GLCDC.gr2_clut0()[92],
      GLCDC.gr2_clut1()[92],
    ",
  0x40342174u64 => "
      GLCDC.gr1_clut0()[93],
      GLCDC.gr1_clut1()[93],
      GLCDC.gr2_clut0()[93],
      GLCDC.gr2_clut1()[93],
    ",
  0x40342178u64 => "
      GLCDC.gr1_clut0()[94],
      GLCDC.gr1_clut1()[94],
      GLCDC.gr2_clut0()[94],
      GLCDC.gr2_clut1()[94],
    ",
  0x4034217cu64 => "
      GLCDC.gr1_clut0()[95],
      GLCDC.gr1_clut1()[95],
      GLCDC.gr2_clut0()[95],
      GLCDC.gr2_clut1()[95],
    ",
  0x40342180u64 => "
      GLCDC.gr1_clut0()[96],
      GLCDC.gr1_clut1()[96],
      GLCDC.gr2_clut0()[96],
      GLCDC.gr2_clut1()[96],
    ",
  0x40342184u64 => "
      GLCDC.gr1_clut0()[97],
      GLCDC.gr1_clut1()[97],
      GLCDC.gr2_clut0()[97],
      GLCDC.gr2_clut1()[97],
    ",
  0x40342188u64 => "
      GLCDC.gr1_clut0()[98],
      GLCDC.gr1_clut1()[98],
      GLCDC.gr2_clut0()[98],
      GLCDC.gr2_clut1()[98],
    ",
  0x4034218cu64 => "
      GLCDC.gr1_clut0()[99],
      GLCDC.gr1_clut1()[99],
      GLCDC.gr2_clut0()[99],
      GLCDC.gr2_clut1()[99],
    ",
  0x40342190u64 => "
      GLCDC.gr1_clut0()[100],
      GLCDC.gr1_clut1()[100],
      GLCDC.gr2_clut0()[100],
      GLCDC.gr2_clut1()[100],
    ",
  0x40342194u64 => "
      GLCDC.gr1_clut0()[101],
      GLCDC.gr1_clut1()[101],
      GLCDC.gr2_clut0()[101],
      GLCDC.gr2_clut1()[101],
    ",
  0x40342198u64 => "
      GLCDC.gr1_clut0()[102],
      GLCDC.gr1_clut1()[102],
      GLCDC.gr2_clut0()[102],
      GLCDC.gr2_clut1()[102],
    ",
  0x4034219cu64 => "
      GLCDC.gr1_clut0()[103],
      GLCDC.gr1_clut1()[103],
      GLCDC.gr2_clut0()[103],
      GLCDC.gr2_clut1()[103],
    ",
  0x403421a0u64 => "
      GLCDC.gr1_clut0()[104],
      GLCDC.gr1_clut1()[104],
      GLCDC.gr2_clut0()[104],
      GLCDC.gr2_clut1()[104],
    ",
  0x403421a4u64 => "
      GLCDC.gr1_clut0()[105],
      GLCDC.gr1_clut1()[105],
      GLCDC.gr2_clut0()[105],
      GLCDC.gr2_clut1()[105],
    ",
  0x403421a8u64 => "
      GLCDC.gr1_clut0()[106],
      GLCDC.gr1_clut1()[106],
      GLCDC.gr2_clut0()[106],
      GLCDC.gr2_clut1()[106],
    ",
  0x403421acu64 => "
      GLCDC.gr1_clut0()[107],
      GLCDC.gr1_clut1()[107],
      GLCDC.gr2_clut0()[107],
      GLCDC.gr2_clut1()[107],
    ",
  0x403421b0u64 => "
      GLCDC.gr1_clut0()[108],
      GLCDC.gr1_clut1()[108],
      GLCDC.gr2_clut0()[108],
      GLCDC.gr2_clut1()[108],
    ",
  0x403421b4u64 => "
      GLCDC.gr1_clut0()[109],
      GLCDC.gr1_clut1()[109],
      GLCDC.gr2_clut0()[109],
      GLCDC.gr2_clut1()[109],
    ",
  0x403421b8u64 => "
      GLCDC.gr1_clut0()[110],
      GLCDC.gr1_clut1()[110],
      GLCDC.gr2_clut0()[110],
      GLCDC.gr2_clut1()[110],
    ",
  0x403421bcu64 => "
      GLCDC.gr1_clut0()[111],
      GLCDC.gr1_clut1()[111],
      GLCDC.gr2_clut0()[111],
      GLCDC.gr2_clut1()[111],
    ",
  0x403421c0u64 => "
      GLCDC.gr1_clut0()[112],
      GLCDC.gr1_clut1()[112],
      GLCDC.gr2_clut0()[112],
      GLCDC.gr2_clut1()[112],
    ",
  0x403421c4u64 => "
      GLCDC.gr1_clut0()[113],
      GLCDC.gr1_clut1()[113],
      GLCDC.gr2_clut0()[113],
      GLCDC.gr2_clut1()[113],
    ",
  0x403421c8u64 => "
      GLCDC.gr1_clut0()[114],
      GLCDC.gr1_clut1()[114],
      GLCDC.gr2_clut0()[114],
      GLCDC.gr2_clut1()[114],
    ",
  0x403421ccu64 => "
      GLCDC.gr1_clut0()[115],
      GLCDC.gr1_clut1()[115],
      GLCDC.gr2_clut0()[115],
      GLCDC.gr2_clut1()[115],
    ",
  0x403421d0u64 => "
      GLCDC.gr1_clut0()[116],
      GLCDC.gr1_clut1()[116],
      GLCDC.gr2_clut0()[116],
      GLCDC.gr2_clut1()[116],
    ",
  0x403421d4u64 => "
      GLCDC.gr1_clut0()[117],
      GLCDC.gr1_clut1()[117],
      GLCDC.gr2_clut0()[117],
      GLCDC.gr2_clut1()[117],
    ",
  0x403421d8u64 => "
      GLCDC.gr1_clut0()[118],
      GLCDC.gr1_clut1()[118],
      GLCDC.gr2_clut0()[118],
      GLCDC.gr2_clut1()[118],
    ",
  0x403421dcu64 => "
      GLCDC.gr1_clut0()[119],
      GLCDC.gr1_clut1()[119],
      GLCDC.gr2_clut0()[119],
      GLCDC.gr2_clut1()[119],
    ",
  0x403421e0u64 => "
      GLCDC.gr1_clut0()[120],
      GLCDC.gr1_clut1()[120],
      GLCDC.gr2_clut0()[120],
      GLCDC.gr2_clut1()[120],
    ",
  0x403421e4u64 => "
      GLCDC.gr1_clut0()[121],
      GLCDC.gr1_clut1()[121],
      GLCDC.gr2_clut0()[121],
      GLCDC.gr2_clut1()[121],
    ",
  0x403421e8u64 => "
      GLCDC.gr1_clut0()[122],
      GLCDC.gr1_clut1()[122],
      GLCDC.gr2_clut0()[122],
      GLCDC.gr2_clut1()[122],
    ",
  0x403421ecu64 => "
      GLCDC.gr1_clut0()[123],
      GLCDC.gr1_clut1()[123],
      GLCDC.gr2_clut0()[123],
      GLCDC.gr2_clut1()[123],
    ",
  0x403421f0u64 => "
      GLCDC.gr1_clut0()[124],
      GLCDC.gr1_clut1()[124],
      GLCDC.gr2_clut0()[124],
      GLCDC.gr2_clut1()[124],
    ",
  0x403421f4u64 => "
      GLCDC.gr1_clut0()[125],
      GLCDC.gr1_clut1()[125],
      GLCDC.gr2_clut0()[125],
      GLCDC.gr2_clut1()[125],
    ",
  0x403421f8u64 => "
      GLCDC.gr1_clut0()[126],
      GLCDC.gr1_clut1()[126],
      GLCDC.gr2_clut0()[126],
      GLCDC.gr2_clut1()[126],
    ",
  0x403421fcu64 => "
      GLCDC.gr1_clut0()[127],
      GLCDC.gr1_clut1()[127],
      GLCDC.gr2_clut0()[127],
      GLCDC.gr2_clut1()[127],
    ",
  0x40342200u64 => "
      GLCDC.gr1_clut0()[128],
      GLCDC.gr1_clut1()[128],
      GLCDC.gr2_clut0()[128],
      GLCDC.gr2_clut1()[128],
    ",
  0x40342204u64 => "
      GLCDC.gr1_clut0()[129],
      GLCDC.gr1_clut1()[129],
      GLCDC.gr2_clut0()[129],
      GLCDC.gr2_clut1()[129],
    ",
  0x40342208u64 => "
      GLCDC.gr1_clut0()[130],
      GLCDC.gr1_clut1()[130],
      GLCDC.gr2_clut0()[130],
      GLCDC.gr2_clut1()[130],
    ",
  0x4034220cu64 => "
      GLCDC.gr1_clut0()[131],
      GLCDC.gr1_clut1()[131],
      GLCDC.gr2_clut0()[131],
      GLCDC.gr2_clut1()[131],
    ",
  0x40342210u64 => "
      GLCDC.gr1_clut0()[132],
      GLCDC.gr1_clut1()[132],
      GLCDC.gr2_clut0()[132],
      GLCDC.gr2_clut1()[132],
    ",
  0x40342214u64 => "
      GLCDC.gr1_clut0()[133],
      GLCDC.gr1_clut1()[133],
      GLCDC.gr2_clut0()[133],
      GLCDC.gr2_clut1()[133],
    ",
  0x40342218u64 => "
      GLCDC.gr1_clut0()[134],
      GLCDC.gr1_clut1()[134],
      GLCDC.gr2_clut0()[134],
      GLCDC.gr2_clut1()[134],
    ",
  0x4034221cu64 => "
      GLCDC.gr1_clut0()[135],
      GLCDC.gr1_clut1()[135],
      GLCDC.gr2_clut0()[135],
      GLCDC.gr2_clut1()[135],
    ",
  0x40342220u64 => "
      GLCDC.gr1_clut0()[136],
      GLCDC.gr1_clut1()[136],
      GLCDC.gr2_clut0()[136],
      GLCDC.gr2_clut1()[136],
    ",
  0x40342224u64 => "
      GLCDC.gr1_clut0()[137],
      GLCDC.gr1_clut1()[137],
      GLCDC.gr2_clut0()[137],
      GLCDC.gr2_clut1()[137],
    ",
  0x40342228u64 => "
      GLCDC.gr1_clut0()[138],
      GLCDC.gr1_clut1()[138],
      GLCDC.gr2_clut0()[138],
      GLCDC.gr2_clut1()[138],
    ",
  0x4034222cu64 => "
      GLCDC.gr1_clut0()[139],
      GLCDC.gr1_clut1()[139],
      GLCDC.gr2_clut0()[139],
      GLCDC.gr2_clut1()[139],
    ",
  0x40342230u64 => "
      GLCDC.gr1_clut0()[140],
      GLCDC.gr1_clut1()[140],
      GLCDC.gr2_clut0()[140],
      GLCDC.gr2_clut1()[140],
    ",
  0x40342234u64 => "
      GLCDC.gr1_clut0()[141],
      GLCDC.gr1_clut1()[141],
      GLCDC.gr2_clut0()[141],
      GLCDC.gr2_clut1()[141],
    ",
  0x40342238u64 => "
      GLCDC.gr1_clut0()[142],
      GLCDC.gr1_clut1()[142],
      GLCDC.gr2_clut0()[142],
      GLCDC.gr2_clut1()[142],
    ",
  0x4034223cu64 => "
      GLCDC.gr1_clut0()[143],
      GLCDC.gr1_clut1()[143],
      GLCDC.gr2_clut0()[143],
      GLCDC.gr2_clut1()[143],
    ",
  0x40342240u64 => "
      GLCDC.gr1_clut0()[144],
      GLCDC.gr1_clut1()[144],
      GLCDC.gr2_clut0()[144],
      GLCDC.gr2_clut1()[144],
    ",
  0x40342244u64 => "
      GLCDC.gr1_clut0()[145],
      GLCDC.gr1_clut1()[145],
      GLCDC.gr2_clut0()[145],
      GLCDC.gr2_clut1()[145],
    ",
  0x40342248u64 => "
      GLCDC.gr1_clut0()[146],
      GLCDC.gr1_clut1()[146],
      GLCDC.gr2_clut0()[146],
      GLCDC.gr2_clut1()[146],
    ",
  0x4034224cu64 => "
      GLCDC.gr1_clut0()[147],
      GLCDC.gr1_clut1()[147],
      GLCDC.gr2_clut0()[147],
      GLCDC.gr2_clut1()[147],
    ",
  0x40342250u64 => "
      GLCDC.gr1_clut0()[148],
      GLCDC.gr1_clut1()[148],
      GLCDC.gr2_clut0()[148],
      GLCDC.gr2_clut1()[148],
    ",
  0x40342254u64 => "
      GLCDC.gr1_clut0()[149],
      GLCDC.gr1_clut1()[149],
      GLCDC.gr2_clut0()[149],
      GLCDC.gr2_clut1()[149],
    ",
  0x40342258u64 => "
      GLCDC.gr1_clut0()[150],
      GLCDC.gr1_clut1()[150],
      GLCDC.gr2_clut0()[150],
      GLCDC.gr2_clut1()[150],
    ",
  0x4034225cu64 => "
      GLCDC.gr1_clut0()[151],
      GLCDC.gr1_clut1()[151],
      GLCDC.gr2_clut0()[151],
      GLCDC.gr2_clut1()[151],
    ",
  0x40342260u64 => "
      GLCDC.gr1_clut0()[152],
      GLCDC.gr1_clut1()[152],
      GLCDC.gr2_clut0()[152],
      GLCDC.gr2_clut1()[152],
    ",
  0x40342264u64 => "
      GLCDC.gr1_clut0()[153],
      GLCDC.gr1_clut1()[153],
      GLCDC.gr2_clut0()[153],
      GLCDC.gr2_clut1()[153],
    ",
  0x40342268u64 => "
      GLCDC.gr1_clut0()[154],
      GLCDC.gr1_clut1()[154],
      GLCDC.gr2_clut0()[154],
      GLCDC.gr2_clut1()[154],
    ",
  0x4034226cu64 => "
      GLCDC.gr1_clut0()[155],
      GLCDC.gr1_clut1()[155],
      GLCDC.gr2_clut0()[155],
      GLCDC.gr2_clut1()[155],
    ",
  0x40342270u64 => "
      GLCDC.gr1_clut0()[156],
      GLCDC.gr1_clut1()[156],
      GLCDC.gr2_clut0()[156],
      GLCDC.gr2_clut1()[156],
    ",
  0x40342274u64 => "
      GLCDC.gr1_clut0()[157],
      GLCDC.gr1_clut1()[157],
      GLCDC.gr2_clut0()[157],
      GLCDC.gr2_clut1()[157],
    ",
  0x40342278u64 => "
      GLCDC.gr1_clut0()[158],
      GLCDC.gr1_clut1()[158],
      GLCDC.gr2_clut0()[158],
      GLCDC.gr2_clut1()[158],
    ",
  0x4034227cu64 => "
      GLCDC.gr1_clut0()[159],
      GLCDC.gr1_clut1()[159],
      GLCDC.gr2_clut0()[159],
      GLCDC.gr2_clut1()[159],
    ",
  0x40342280u64 => "
      GLCDC.gr1_clut0()[160],
      GLCDC.gr1_clut1()[160],
      GLCDC.gr2_clut0()[160],
      GLCDC.gr2_clut1()[160],
    ",
  0x40342284u64 => "
      GLCDC.gr1_clut0()[161],
      GLCDC.gr1_clut1()[161],
      GLCDC.gr2_clut0()[161],
      GLCDC.gr2_clut1()[161],
    ",
  0x40342288u64 => "
      GLCDC.gr1_clut0()[162],
      GLCDC.gr1_clut1()[162],
      GLCDC.gr2_clut0()[162],
      GLCDC.gr2_clut1()[162],
    ",
  0x4034228cu64 => "
      GLCDC.gr1_clut0()[163],
      GLCDC.gr1_clut1()[163],
      GLCDC.gr2_clut0()[163],
      GLCDC.gr2_clut1()[163],
    ",
  0x40342290u64 => "
      GLCDC.gr1_clut0()[164],
      GLCDC.gr1_clut1()[164],
      GLCDC.gr2_clut0()[164],
      GLCDC.gr2_clut1()[164],
    ",
  0x40342294u64 => "
      GLCDC.gr1_clut0()[165],
      GLCDC.gr1_clut1()[165],
      GLCDC.gr2_clut0()[165],
      GLCDC.gr2_clut1()[165],
    ",
  0x40342298u64 => "
      GLCDC.gr1_clut0()[166],
      GLCDC.gr1_clut1()[166],
      GLCDC.gr2_clut0()[166],
      GLCDC.gr2_clut1()[166],
    ",
  0x4034229cu64 => "
      GLCDC.gr1_clut0()[167],
      GLCDC.gr1_clut1()[167],
      GLCDC.gr2_clut0()[167],
      GLCDC.gr2_clut1()[167],
    ",
  0x403422a0u64 => "
      GLCDC.gr1_clut0()[168],
      GLCDC.gr1_clut1()[168],
      GLCDC.gr2_clut0()[168],
      GLCDC.gr2_clut1()[168],
    ",
  0x403422a4u64 => "
      GLCDC.gr1_clut0()[169],
      GLCDC.gr1_clut1()[169],
      GLCDC.gr2_clut0()[169],
      GLCDC.gr2_clut1()[169],
    ",
  0x403422a8u64 => "
      GLCDC.gr1_clut0()[170],
      GLCDC.gr1_clut1()[170],
      GLCDC.gr2_clut0()[170],
      GLCDC.gr2_clut1()[170],
    ",
  0x403422acu64 => "
      GLCDC.gr1_clut0()[171],
      GLCDC.gr1_clut1()[171],
      GLCDC.gr2_clut0()[171],
      GLCDC.gr2_clut1()[171],
    ",
  0x403422b0u64 => "
      GLCDC.gr1_clut0()[172],
      GLCDC.gr1_clut1()[172],
      GLCDC.gr2_clut0()[172],
      GLCDC.gr2_clut1()[172],
    ",
  0x403422b4u64 => "
      GLCDC.gr1_clut0()[173],
      GLCDC.gr1_clut1()[173],
      GLCDC.gr2_clut0()[173],
      GLCDC.gr2_clut1()[173],
    ",
  0x403422b8u64 => "
      GLCDC.gr1_clut0()[174],
      GLCDC.gr1_clut1()[174],
      GLCDC.gr2_clut0()[174],
      GLCDC.gr2_clut1()[174],
    ",
  0x403422bcu64 => "
      GLCDC.gr1_clut0()[175],
      GLCDC.gr1_clut1()[175],
      GLCDC.gr2_clut0()[175],
      GLCDC.gr2_clut1()[175],
    ",
  0x403422c0u64 => "
      GLCDC.gr1_clut0()[176],
      GLCDC.gr1_clut1()[176],
      GLCDC.gr2_clut0()[176],
      GLCDC.gr2_clut1()[176],
    ",
  0x403422c4u64 => "
      GLCDC.gr1_clut0()[177],
      GLCDC.gr1_clut1()[177],
      GLCDC.gr2_clut0()[177],
      GLCDC.gr2_clut1()[177],
    ",
  0x403422c8u64 => "
      GLCDC.gr1_clut0()[178],
      GLCDC.gr1_clut1()[178],
      GLCDC.gr2_clut0()[178],
      GLCDC.gr2_clut1()[178],
    ",
  0x403422ccu64 => "
      GLCDC.gr1_clut0()[179],
      GLCDC.gr1_clut1()[179],
      GLCDC.gr2_clut0()[179],
      GLCDC.gr2_clut1()[179],
    ",
  0x403422d0u64 => "
      GLCDC.gr1_clut0()[180],
      GLCDC.gr1_clut1()[180],
      GLCDC.gr2_clut0()[180],
      GLCDC.gr2_clut1()[180],
    ",
  0x403422d4u64 => "
      GLCDC.gr1_clut0()[181],
      GLCDC.gr1_clut1()[181],
      GLCDC.gr2_clut0()[181],
      GLCDC.gr2_clut1()[181],
    ",
  0x403422d8u64 => "
      GLCDC.gr1_clut0()[182],
      GLCDC.gr1_clut1()[182],
      GLCDC.gr2_clut0()[182],
      GLCDC.gr2_clut1()[182],
    ",
  0x403422dcu64 => "
      GLCDC.gr1_clut0()[183],
      GLCDC.gr1_clut1()[183],
      GLCDC.gr2_clut0()[183],
      GLCDC.gr2_clut1()[183],
    ",
  0x403422e0u64 => "
      GLCDC.gr1_clut0()[184],
      GLCDC.gr1_clut1()[184],
      GLCDC.gr2_clut0()[184],
      GLCDC.gr2_clut1()[184],
    ",
  0x403422e4u64 => "
      GLCDC.gr1_clut0()[185],
      GLCDC.gr1_clut1()[185],
      GLCDC.gr2_clut0()[185],
      GLCDC.gr2_clut1()[185],
    ",
  0x403422e8u64 => "
      GLCDC.gr1_clut0()[186],
      GLCDC.gr1_clut1()[186],
      GLCDC.gr2_clut0()[186],
      GLCDC.gr2_clut1()[186],
    ",
  0x403422ecu64 => "
      GLCDC.gr1_clut0()[187],
      GLCDC.gr1_clut1()[187],
      GLCDC.gr2_clut0()[187],
      GLCDC.gr2_clut1()[187],
    ",
  0x403422f0u64 => "
      GLCDC.gr1_clut0()[188],
      GLCDC.gr1_clut1()[188],
      GLCDC.gr2_clut0()[188],
      GLCDC.gr2_clut1()[188],
    ",
  0x403422f4u64 => "
      GLCDC.gr1_clut0()[189],
      GLCDC.gr1_clut1()[189],
      GLCDC.gr2_clut0()[189],
      GLCDC.gr2_clut1()[189],
    ",
  0x403422f8u64 => "
      GLCDC.gr1_clut0()[190],
      GLCDC.gr1_clut1()[190],
      GLCDC.gr2_clut0()[190],
      GLCDC.gr2_clut1()[190],
    ",
  0x403422fcu64 => "
      GLCDC.gr1_clut0()[191],
      GLCDC.gr1_clut1()[191],
      GLCDC.gr2_clut0()[191],
      GLCDC.gr2_clut1()[191],
    ",
  0x40342300u64 => "
      GLCDC.gr1_clut0()[192],
      GLCDC.gr1_clut1()[192],
      GLCDC.gr2_clut0()[192],
      GLCDC.gr2_clut1()[192],
    ",
  0x40342304u64 => "
      GLCDC.gr1_clut0()[193],
      GLCDC.gr1_clut1()[193],
      GLCDC.gr2_clut0()[193],
      GLCDC.gr2_clut1()[193],
    ",
  0x40342308u64 => "
      GLCDC.gr1_clut0()[194],
      GLCDC.gr1_clut1()[194],
      GLCDC.gr2_clut0()[194],
      GLCDC.gr2_clut1()[194],
    ",
  0x4034230cu64 => "
      GLCDC.gr1_clut0()[195],
      GLCDC.gr1_clut1()[195],
      GLCDC.gr2_clut0()[195],
      GLCDC.gr2_clut1()[195],
    ",
  0x40342310u64 => "
      GLCDC.gr1_clut0()[196],
      GLCDC.gr1_clut1()[196],
      GLCDC.gr2_clut0()[196],
      GLCDC.gr2_clut1()[196],
    ",
  0x40342314u64 => "
      GLCDC.gr1_clut0()[197],
      GLCDC.gr1_clut1()[197],
      GLCDC.gr2_clut0()[197],
      GLCDC.gr2_clut1()[197],
    ",
  0x40342318u64 => "
      GLCDC.gr1_clut0()[198],
      GLCDC.gr1_clut1()[198],
      GLCDC.gr2_clut0()[198],
      GLCDC.gr2_clut1()[198],
    ",
  0x4034231cu64 => "
      GLCDC.gr1_clut0()[199],
      GLCDC.gr1_clut1()[199],
      GLCDC.gr2_clut0()[199],
      GLCDC.gr2_clut1()[199],
    ",
  0x40342320u64 => "
      GLCDC.gr1_clut0()[200],
      GLCDC.gr1_clut1()[200],
      GLCDC.gr2_clut0()[200],
      GLCDC.gr2_clut1()[200],
    ",
  0x40342324u64 => "
      GLCDC.gr1_clut0()[201],
      GLCDC.gr1_clut1()[201],
      GLCDC.gr2_clut0()[201],
      GLCDC.gr2_clut1()[201],
    ",
  0x40342328u64 => "
      GLCDC.gr1_clut0()[202],
      GLCDC.gr1_clut1()[202],
      GLCDC.gr2_clut0()[202],
      GLCDC.gr2_clut1()[202],
    ",
  0x4034232cu64 => "
      GLCDC.gr1_clut0()[203],
      GLCDC.gr1_clut1()[203],
      GLCDC.gr2_clut0()[203],
      GLCDC.gr2_clut1()[203],
    ",
  0x40342330u64 => "
      GLCDC.gr1_clut0()[204],
      GLCDC.gr1_clut1()[204],
      GLCDC.gr2_clut0()[204],
      GLCDC.gr2_clut1()[204],
    ",
  0x40342334u64 => "
      GLCDC.gr1_clut0()[205],
      GLCDC.gr1_clut1()[205],
      GLCDC.gr2_clut0()[205],
      GLCDC.gr2_clut1()[205],
    ",
  0x40342338u64 => "
      GLCDC.gr1_clut0()[206],
      GLCDC.gr1_clut1()[206],
      GLCDC.gr2_clut0()[206],
      GLCDC.gr2_clut1()[206],
    ",
  0x4034233cu64 => "
      GLCDC.gr1_clut0()[207],
      GLCDC.gr1_clut1()[207],
      GLCDC.gr2_clut0()[207],
      GLCDC.gr2_clut1()[207],
    ",
  0x40342340u64 => "
      GLCDC.gr1_clut0()[208],
      GLCDC.gr1_clut1()[208],
      GLCDC.gr2_clut0()[208],
      GLCDC.gr2_clut1()[208],
    ",
  0x40342344u64 => "
      GLCDC.gr1_clut0()[209],
      GLCDC.gr1_clut1()[209],
      GLCDC.gr2_clut0()[209],
      GLCDC.gr2_clut1()[209],
    ",
  0x40342348u64 => "
      GLCDC.gr1_clut0()[210],
      GLCDC.gr1_clut1()[210],
      GLCDC.gr2_clut0()[210],
      GLCDC.gr2_clut1()[210],
    ",
  0x4034234cu64 => "
      GLCDC.gr1_clut0()[211],
      GLCDC.gr1_clut1()[211],
      GLCDC.gr2_clut0()[211],
      GLCDC.gr2_clut1()[211],
    ",
  0x40342350u64 => "
      GLCDC.gr1_clut0()[212],
      GLCDC.gr1_clut1()[212],
      GLCDC.gr2_clut0()[212],
      GLCDC.gr2_clut1()[212],
    ",
  0x40342354u64 => "
      GLCDC.gr1_clut0()[213],
      GLCDC.gr1_clut1()[213],
      GLCDC.gr2_clut0()[213],
      GLCDC.gr2_clut1()[213],
    ",
  0x40342358u64 => "
      GLCDC.gr1_clut0()[214],
      GLCDC.gr1_clut1()[214],
      GLCDC.gr2_clut0()[214],
      GLCDC.gr2_clut1()[214],
    ",
  0x4034235cu64 => "
      GLCDC.gr1_clut0()[215],
      GLCDC.gr1_clut1()[215],
      GLCDC.gr2_clut0()[215],
      GLCDC.gr2_clut1()[215],
    ",
  0x40342360u64 => "
      GLCDC.gr1_clut0()[216],
      GLCDC.gr1_clut1()[216],
      GLCDC.gr2_clut0()[216],
      GLCDC.gr2_clut1()[216],
    ",
  0x40342364u64 => "
      GLCDC.gr1_clut0()[217],
      GLCDC.gr1_clut1()[217],
      GLCDC.gr2_clut0()[217],
      GLCDC.gr2_clut1()[217],
    ",
  0x40342368u64 => "
      GLCDC.gr1_clut0()[218],
      GLCDC.gr1_clut1()[218],
      GLCDC.gr2_clut0()[218],
      GLCDC.gr2_clut1()[218],
    ",
  0x4034236cu64 => "
      GLCDC.gr1_clut0()[219],
      GLCDC.gr1_clut1()[219],
      GLCDC.gr2_clut0()[219],
      GLCDC.gr2_clut1()[219],
    ",
  0x40342370u64 => "
      GLCDC.gr1_clut0()[220],
      GLCDC.gr1_clut1()[220],
      GLCDC.gr2_clut0()[220],
      GLCDC.gr2_clut1()[220],
    ",
  0x40342374u64 => "
      GLCDC.gr1_clut0()[221],
      GLCDC.gr1_clut1()[221],
      GLCDC.gr2_clut0()[221],
      GLCDC.gr2_clut1()[221],
    ",
  0x40342378u64 => "
      GLCDC.gr1_clut0()[222],
      GLCDC.gr1_clut1()[222],
      GLCDC.gr2_clut0()[222],
      GLCDC.gr2_clut1()[222],
    ",
  0x4034237cu64 => "
      GLCDC.gr1_clut0()[223],
      GLCDC.gr1_clut1()[223],
      GLCDC.gr2_clut0()[223],
      GLCDC.gr2_clut1()[223],
    ",
  0x40342380u64 => "
      GLCDC.gr1_clut0()[224],
      GLCDC.gr1_clut1()[224],
      GLCDC.gr2_clut0()[224],
      GLCDC.gr2_clut1()[224],
    ",
  0x40342384u64 => "
      GLCDC.gr1_clut0()[225],
      GLCDC.gr1_clut1()[225],
      GLCDC.gr2_clut0()[225],
      GLCDC.gr2_clut1()[225],
    ",
  0x40342388u64 => "
      GLCDC.gr1_clut0()[226],
      GLCDC.gr1_clut1()[226],
      GLCDC.gr2_clut0()[226],
      GLCDC.gr2_clut1()[226],
    ",
  0x4034238cu64 => "
      GLCDC.gr1_clut0()[227],
      GLCDC.gr1_clut1()[227],
      GLCDC.gr2_clut0()[227],
      GLCDC.gr2_clut1()[227],
    ",
  0x40342390u64 => "
      GLCDC.gr1_clut0()[228],
      GLCDC.gr1_clut1()[228],
      GLCDC.gr2_clut0()[228],
      GLCDC.gr2_clut1()[228],
    ",
  0x40342394u64 => "
      GLCDC.gr1_clut0()[229],
      GLCDC.gr1_clut1()[229],
      GLCDC.gr2_clut0()[229],
      GLCDC.gr2_clut1()[229],
    ",
  0x40342398u64 => "
      GLCDC.gr1_clut0()[230],
      GLCDC.gr1_clut1()[230],
      GLCDC.gr2_clut0()[230],
      GLCDC.gr2_clut1()[230],
    ",
  0x4034239cu64 => "
      GLCDC.gr1_clut0()[231],
      GLCDC.gr1_clut1()[231],
      GLCDC.gr2_clut0()[231],
      GLCDC.gr2_clut1()[231],
    ",
  0x403423a0u64 => "
      GLCDC.gr1_clut0()[232],
      GLCDC.gr1_clut1()[232],
      GLCDC.gr2_clut0()[232],
      GLCDC.gr2_clut1()[232],
    ",
  0x403423a4u64 => "
      GLCDC.gr1_clut0()[233],
      GLCDC.gr1_clut1()[233],
      GLCDC.gr2_clut0()[233],
      GLCDC.gr2_clut1()[233],
    ",
  0x403423a8u64 => "
      GLCDC.gr1_clut0()[234],
      GLCDC.gr1_clut1()[234],
      GLCDC.gr2_clut0()[234],
      GLCDC.gr2_clut1()[234],
    ",
  0x403423acu64 => "
      GLCDC.gr1_clut0()[235],
      GLCDC.gr1_clut1()[235],
      GLCDC.gr2_clut0()[235],
      GLCDC.gr2_clut1()[235],
    ",
  0x403423b0u64 => "
      GLCDC.gr1_clut0()[236],
      GLCDC.gr1_clut1()[236],
      GLCDC.gr2_clut0()[236],
      GLCDC.gr2_clut1()[236],
    ",
  0x403423b4u64 => "
      GLCDC.gr1_clut0()[237],
      GLCDC.gr1_clut1()[237],
      GLCDC.gr2_clut0()[237],
      GLCDC.gr2_clut1()[237],
    ",
  0x403423b8u64 => "
      GLCDC.gr1_clut0()[238],
      GLCDC.gr1_clut1()[238],
      GLCDC.gr2_clut0()[238],
      GLCDC.gr2_clut1()[238],
    ",
  0x403423bcu64 => "
      GLCDC.gr1_clut0()[239],
      GLCDC.gr1_clut1()[239],
      GLCDC.gr2_clut0()[239],
      GLCDC.gr2_clut1()[239],
    ",
  0x403423c0u64 => "
      GLCDC.gr1_clut0()[240],
      GLCDC.gr1_clut1()[240],
      GLCDC.gr2_clut0()[240],
      GLCDC.gr2_clut1()[240],
    ",
  0x403423c4u64 => "
      GLCDC.gr1_clut0()[241],
      GLCDC.gr1_clut1()[241],
      GLCDC.gr2_clut0()[241],
      GLCDC.gr2_clut1()[241],
    ",
  0x403423c8u64 => "
      GLCDC.gr1_clut0()[242],
      GLCDC.gr1_clut1()[242],
      GLCDC.gr2_clut0()[242],
      GLCDC.gr2_clut1()[242],
    ",
  0x403423ccu64 => "
      GLCDC.gr1_clut0()[243],
      GLCDC.gr1_clut1()[243],
      GLCDC.gr2_clut0()[243],
      GLCDC.gr2_clut1()[243],
    ",
  0x403423d0u64 => "
      GLCDC.gr1_clut0()[244],
      GLCDC.gr1_clut1()[244],
      GLCDC.gr2_clut0()[244],
      GLCDC.gr2_clut1()[244],
    ",
  0x403423d4u64 => "
      GLCDC.gr1_clut0()[245],
      GLCDC.gr1_clut1()[245],
      GLCDC.gr2_clut0()[245],
      GLCDC.gr2_clut1()[245],
    ",
  0x403423d8u64 => "
      GLCDC.gr1_clut0()[246],
      GLCDC.gr1_clut1()[246],
      GLCDC.gr2_clut0()[246],
      GLCDC.gr2_clut1()[246],
    ",
  0x403423dcu64 => "
      GLCDC.gr1_clut0()[247],
      GLCDC.gr1_clut1()[247],
      GLCDC.gr2_clut0()[247],
      GLCDC.gr2_clut1()[247],
    ",
  0x403423e0u64 => "
      GLCDC.gr1_clut0()[248],
      GLCDC.gr1_clut1()[248],
      GLCDC.gr2_clut0()[248],
      GLCDC.gr2_clut1()[248],
    ",
  0x403423e4u64 => "
      GLCDC.gr1_clut0()[249],
      GLCDC.gr1_clut1()[249],
      GLCDC.gr2_clut0()[249],
      GLCDC.gr2_clut1()[249],
    ",
  0x403423e8u64 => "
      GLCDC.gr1_clut0()[250],
      GLCDC.gr1_clut1()[250],
      GLCDC.gr2_clut0()[250],
      GLCDC.gr2_clut1()[250],
    ",
  0x403423ecu64 => "
      GLCDC.gr1_clut0()[251],
      GLCDC.gr1_clut1()[251],
      GLCDC.gr2_clut0()[251],
      GLCDC.gr2_clut1()[251],
    ",
  0x403423f0u64 => "
      GLCDC.gr1_clut0()[252],
      GLCDC.gr1_clut1()[252],
      GLCDC.gr2_clut0()[252],
      GLCDC.gr2_clut1()[252],
    ",
  0x403423f4u64 => "
      GLCDC.gr1_clut0()[253],
      GLCDC.gr1_clut1()[253],
      GLCDC.gr2_clut0()[253],
      GLCDC.gr2_clut1()[253],
    ",
  0x403423f8u64 => "
      GLCDC.gr1_clut0()[254],
      GLCDC.gr1_clut1()[254],
      GLCDC.gr2_clut0()[254],
      GLCDC.gr2_clut1()[254],
    ",
  0x403423fcu64 => "
      GLCDC.gr1_clut0()[255],
      GLCDC.gr1_clut1()[255],
      GLCDC.gr2_clut0()[255],
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
  0x40346000u64 => "
      MIPI_DSI.isr(),
    ",
  0x40346010u64 => "
      MIPI_DSI.linksr(),
    ",
  0x40346100u64 => "
      MIPI_DSI.txsetr(),
    ",
  0x40346104u64 => "
      MIPI_DSI.hsclksetr(),
    ",
  0x40346108u64 => "
      MIPI_DSI.ulpssetr(),
    ",
  0x4034610cu64 => "
      MIPI_DSI.ulpscr(),
    ",
  0x40346110u64 => "
      MIPI_DSI.rstcr(),
    ",
  0x40346114u64 => "
      MIPI_DSI.rstsr(),
    ",
  0x40346120u64 => "
      MIPI_DSI.dsisetr(),
    ",
  0x40346160u64 => "
      MIPI_DSI.txppd0r(),
    ",
  0x40346164u64 => "
      MIPI_DSI.txppd1r(),
    ",
  0x40346168u64 => "
      MIPI_DSI.txppd2r(),
    ",
  0x4034616cu64 => "
      MIPI_DSI.txppd3r(),
    ",
  0x40346200u64 => "
      MIPI_DSI.rxsr(),
    ",
  0x40346204u64 => "
      MIPI_DSI.rxscr(),
    ",
  0x40346208u64 => "
      MIPI_DSI.rxier(),
    ",
  0x40346210u64 => "
      MIPI_DSI.presptobtasetr(),
    ",
  0x40346214u64 => "
      MIPI_DSI.presptolpsetr(),
    ",
  0x40346218u64 => "
      MIPI_DSI.presptohssetr(),
    ",
  0x40346220u64 => "
      MIPI_DSI.akeplatir(),
    ",
  0x40346224u64 => "
      MIPI_DSI.akepacmsr(),
    ",
  0x40346228u64 => "
      MIPI_DSI.akepscr(),
    ",
  0x40346230u64 => "
      MIPI_DSI.rxrssr(),
    ",
  0x40346234u64 => "
      MIPI_DSI.rxrsscr(),
    ",
  0x40346238u64 => "
      MIPI_DSI.rxrinfoowsr(),
    ",
  0x4034623cu64 => "
      MIPI_DSI.rxrinfoowscr(),
    ",
  0x40346240u64 => "
      MIPI_DSI.rxrssr()[0],
    ",
  0x40346244u64 => "
      MIPI_DSI.rxrssr()[1],
    ",
  0x40346248u64 => "
      MIPI_DSI.rxrssr()[2],
    ",
  0x4034624cu64 => "
      MIPI_DSI.rxrssr()[3],
    ",
  0x403462c0u64 => "
      MIPI_DSI.rxppd0r(),
    ",
  0x403462c4u64 => "
      MIPI_DSI.rxppd1r(),
    ",
  0x403462c8u64 => "
      MIPI_DSI.rxppd2r(),
    ",
  0x403462ccu64 => "
      MIPI_DSI.rxppd3r(),
    ",
  0x403462e0u64 => "
      MIPI_DSI.hstxtosetr(),
    ",
  0x403462e4u64 => "
      MIPI_DSI.lrxhtosetr(),
    ",
  0x403462e8u64 => "
      MIPI_DSI.tatosetr(),
    ",
  0x40346300u64 => "
      MIPI_DSI.ferrsr(),
    ",
  0x40346304u64 => "
      MIPI_DSI.ferrscr(),
    ",
  0x40346308u64 => "
      MIPI_DSI.ferrier(),
    ",
  0x40346314u64 => "
      MIPI_DSI.clstptsetr(),
    ",
  0x40346318u64 => "
      MIPI_DSI.lptrnstsetr(),
    ",
  0x40346320u64 => "
      MIPI_DSI.plsr(),
    ",
  0x40346324u64 => "
      MIPI_DSI.plscr(),
    ",
  0x40346328u64 => "
      MIPI_DSI.plier(),
    ",
  0x40346400u64 => "
      MIPI_DSI.vmset0r(),
    ",
  0x40346404u64 => "
      MIPI_DSI.vmset1r(),
    ",
  0x40346410u64 => "
      MIPI_DSI.vmsr(),
    ",
  0x40346414u64 => "
      MIPI_DSI.vmscr(),
    ",
  0x40346418u64 => "
      MIPI_DSI.vmier(),
    ",
  0x40346420u64 => "
      MIPI_DSI.vmppsetr(),
    ",
  0x40346428u64 => "
      MIPI_DSI.vmvssetr(),
    ",
  0x4034642cu64 => "
      MIPI_DSI.vmvpsetr(),
    ",
  0x40346430u64 => "
      MIPI_DSI.vmhssetr(),
    ",
  0x40346434u64 => "
      MIPI_DSI.vmhpsetr(),
    ",
  0x403465c0u64 => "
      MIPI_DSI.sqch0set0r(),
    ",
  0x403465d0u64 => "
      MIPI_DSI.sqch0sr(),
    ",
  0x403465d4u64 => "
      MIPI_DSI.sqch0scr(),
    ",
  0x403465d8u64 => "
      MIPI_DSI.sqch0ier(),
    ",
  0x40346600u64 => "
      MIPI_DSI.sqch1set0r(),
    ",
  0x40346610u64 => "
      MIPI_DSI.sqch1sr(),
    ",
  0x40346614u64 => "
      MIPI_DSI.sqch1scr(),
    ",
  0x40346618u64 => "
      MIPI_DSI.sqch1ier(),
    ",
  0x40346780u64 => "
      MIPI_DSI.sqch0dscar()[0],
    ",
  0x40346790u64 => "
      MIPI_DSI.sqch0dscar()[1],
    ",
  0x403467a0u64 => "
      MIPI_DSI.sqch0dscar()[2],
    ",
  0x403467b0u64 => "
      MIPI_DSI.sqch0dscar()[3],
    ",
  0x403467c0u64 => "
      MIPI_DSI.sqch0dscar()[4],
    ",
  0x403467d0u64 => "
      MIPI_DSI.sqch0dscar()[5],
    ",
  0x403467e0u64 => "
      MIPI_DSI.sqch0dscar()[6],
    ",
  0x403467f0u64 => "
      MIPI_DSI.sqch0dscar()[7],
    ",
  0x40346784u64 => "
      MIPI_DSI.sqch0dscbr()[0],
    ",
  0x40346794u64 => "
      MIPI_DSI.sqch0dscbr()[1],
    ",
  0x403467a4u64 => "
      MIPI_DSI.sqch0dscbr()[2],
    ",
  0x403467b4u64 => "
      MIPI_DSI.sqch0dscbr()[3],
    ",
  0x403467c4u64 => "
      MIPI_DSI.sqch0dscbr()[4],
    ",
  0x403467d4u64 => "
      MIPI_DSI.sqch0dscbr()[5],
    ",
  0x403467e4u64 => "
      MIPI_DSI.sqch0dscbr()[6],
    ",
  0x403467f4u64 => "
      MIPI_DSI.sqch0dscbr()[7],
    ",
  0x40346788u64 => "
      MIPI_DSI.sqch0dsccr()[0],
    ",
  0x40346798u64 => "
      MIPI_DSI.sqch0dsccr()[1],
    ",
  0x403467a8u64 => "
      MIPI_DSI.sqch0dsccr()[2],
    ",
  0x403467b8u64 => "
      MIPI_DSI.sqch0dsccr()[3],
    ",
  0x403467c8u64 => "
      MIPI_DSI.sqch0dsccr()[4],
    ",
  0x403467d8u64 => "
      MIPI_DSI.sqch0dsccr()[5],
    ",
  0x403467e8u64 => "
      MIPI_DSI.sqch0dsccr()[6],
    ",
  0x403467f8u64 => "
      MIPI_DSI.sqch0dsccr()[7],
    ",
  0x4034678cu64 => "
      MIPI_DSI.sqch0dscdr()[0],
    ",
  0x4034679cu64 => "
      MIPI_DSI.sqch0dscdr()[1],
    ",
  0x403467acu64 => "
      MIPI_DSI.sqch0dscdr()[2],
    ",
  0x403467bcu64 => "
      MIPI_DSI.sqch0dscdr()[3],
    ",
  0x403467ccu64 => "
      MIPI_DSI.sqch0dscdr()[4],
    ",
  0x403467dcu64 => "
      MIPI_DSI.sqch0dscdr()[5],
    ",
  0x403467ecu64 => "
      MIPI_DSI.sqch0dscdr()[6],
    ",
  0x403467fcu64 => "
      MIPI_DSI.sqch0dscdr()[7],
    ",
  0x40346800u64 => "
      MIPI_DSI.sqch1dscar()[0],
    ",
  0x40346810u64 => "
      MIPI_DSI.sqch1dscar()[1],
    ",
  0x40346820u64 => "
      MIPI_DSI.sqch1dscar()[2],
    ",
  0x40346830u64 => "
      MIPI_DSI.sqch1dscar()[3],
    ",
  0x40346840u64 => "
      MIPI_DSI.sqch1dscar()[4],
    ",
  0x40346850u64 => "
      MIPI_DSI.sqch1dscar()[5],
    ",
  0x40346860u64 => "
      MIPI_DSI.sqch1dscar()[6],
    ",
  0x40346870u64 => "
      MIPI_DSI.sqch1dscar()[7],
    ",
  0x40346804u64 => "
      MIPI_DSI.sqch1dscbr()[0],
    ",
  0x40346814u64 => "
      MIPI_DSI.sqch1dscbr()[1],
    ",
  0x40346824u64 => "
      MIPI_DSI.sqch1dscbr()[2],
    ",
  0x40346834u64 => "
      MIPI_DSI.sqch1dscbr()[3],
    ",
  0x40346844u64 => "
      MIPI_DSI.sqch1dscbr()[4],
    ",
  0x40346854u64 => "
      MIPI_DSI.sqch1dscbr()[5],
    ",
  0x40346864u64 => "
      MIPI_DSI.sqch1dscbr()[6],
    ",
  0x40346874u64 => "
      MIPI_DSI.sqch1dscbr()[7],
    ",
  0x40346808u64 => "
      MIPI_DSI.sqch1dsccr()[0],
    ",
  0x40346818u64 => "
      MIPI_DSI.sqch1dsccr()[1],
    ",
  0x40346828u64 => "
      MIPI_DSI.sqch1dsccr()[2],
    ",
  0x40346838u64 => "
      MIPI_DSI.sqch1dsccr()[3],
    ",
  0x40346848u64 => "
      MIPI_DSI.sqch1dsccr()[4],
    ",
  0x40346858u64 => "
      MIPI_DSI.sqch1dsccr()[5],
    ",
  0x40346868u64 => "
      MIPI_DSI.sqch1dsccr()[6],
    ",
  0x40346878u64 => "
      MIPI_DSI.sqch1dsccr()[7],
    ",
  0x4034680cu64 => "
      MIPI_DSI.sqch1dscdr()[0],
    ",
  0x4034681cu64 => "
      MIPI_DSI.sqch1dscdr()[1],
    ",
  0x4034682cu64 => "
      MIPI_DSI.sqch1dscdr()[2],
    ",
  0x4034683cu64 => "
      MIPI_DSI.sqch1dscdr()[3],
    ",
  0x4034684cu64 => "
      MIPI_DSI.sqch1dscdr()[4],
    ",
  0x4034685cu64 => "
      MIPI_DSI.sqch1dscdr()[5],
    ",
  0x4034686cu64 => "
      MIPI_DSI.sqch1dscdr()[6],
    ",
  0x4034687cu64 => "
      MIPI_DSI.sqch1dscdr()[7],
    ",
  0x40346c00u64 => "
      MIPI_PHY_0.dphyrefcr(),
    ",
  0x40346c04u64 => "
      MIPI_PHY_0.dphyplfcr(),
    ",
  0x40346c08u64 => "
      MIPI_PHY_0.dphyplocr(),
    ",
  0x40346c0cu64 => "
      MIPI_PHY_0.dphyesccr(),
    ",
  0x40346c10u64 => "
      MIPI_PHY_0.dphypwrcr(),
    ",
  0x40346c1cu64 => "
      MIPI_PHY_0.dphysfr(),
    ",
  0x40346c20u64 => "
      MIPI_PHY_0.dphyocr(),
    ",
  0x40346c24u64 => "
      MIPI_PHY_0.dphytim1(),
    ",
  0x40346c28u64 => "
      MIPI_PHY_0.dphytim2(),
    ",
  0x40346c2cu64 => "
      MIPI_PHY_0.dphytim3(),
    ",
  0x40346c30u64 => "
      MIPI_PHY_0.dphytim4(),
    ",
  0x40346c34u64 => "
      MIPI_PHY_0.dphytim5(),
    ",
  0x40346c38u64 => "
      MIPI_PHY_0.dphytim6(),
    ",
  0x40346c48u64 => "
      MIPI_PHY_0.dphymdc(),
    ",
  0x40347000u64 => "
      MIPI_CSI_0.mcg(),
    ",
  0x40347010u64 => "
      MIPI_CSI_0.mct0(),
    ",
  0x40347018u64 => "
      MIPI_CSI_0.mct2(),
    ",
  0x4034701cu64 => "
      MIPI_CSI_0.mct3(),
    ",
  0x40347028u64 => "
      MIPI_CSI_0.rtct(),
    ",
  0x4034702cu64 => "
      MIPI_CSI_0.rtst(),
    ",
  0x40347040u64 => "
      MIPI_CSI_0.epct(),
    ",
  0x40347044u64 => "
      MIPI_CSI_0.emct(),
    ",
  0x40347050u64 => "
      MIPI_CSI_0.mist(),
    ",
  0x40347060u64 => "
      MIPI_CSI_0.dtel(),
    ",
  0x40347064u64 => "
      MIPI_CSI_0.dteh(),
    ",
  0x40347070u64 => "
      MIPI_CSI_0.rxst(),
    ",
  0x40347074u64 => "
      MIPI_CSI_0.rxsc(),
    ",
  0x40347078u64 => "
      MIPI_CSI_0.rxie(),
    ",
  0x40347080u64 => "
      MIPI_CSI_0.dlst()[0],
    ",
  0x40347090u64 => "
      MIPI_CSI_0.dlst()[1],
    ",
  0x40347084u64 => "
      MIPI_CSI_0.dlsc()[0],
    ",
  0x40347094u64 => "
      MIPI_CSI_0.dlsc()[1],
    ",
  0x40347088u64 => "
      MIPI_CSI_0.dlie()[0],
    ",
  0x40347098u64 => "
      MIPI_CSI_0.dlie()[1],
    ",
  0x40347100u64 => "
      MIPI_CSI_0.vcst()[0],
    ",
  0x40347110u64 => "
      MIPI_CSI_0.vcst()[1],
    ",
  0x40347120u64 => "
      MIPI_CSI_0.vcst()[2],
    ",
  0x40347130u64 => "
      MIPI_CSI_0.vcst()[3],
    ",
  0x40347140u64 => "
      MIPI_CSI_0.vcst()[4],
    ",
  0x40347150u64 => "
      MIPI_CSI_0.vcst()[5],
    ",
  0x40347160u64 => "
      MIPI_CSI_0.vcst()[6],
    ",
  0x40347170u64 => "
      MIPI_CSI_0.vcst()[7],
    ",
  0x40347180u64 => "
      MIPI_CSI_0.vcst()[8],
    ",
  0x40347190u64 => "
      MIPI_CSI_0.vcst()[9],
    ",
  0x403471a0u64 => "
      MIPI_CSI_0.vcst()[10],
    ",
  0x403471b0u64 => "
      MIPI_CSI_0.vcst()[11],
    ",
  0x403471c0u64 => "
      MIPI_CSI_0.vcst()[12],
    ",
  0x403471d0u64 => "
      MIPI_CSI_0.vcst()[13],
    ",
  0x403471e0u64 => "
      MIPI_CSI_0.vcst()[14],
    ",
  0x403471f0u64 => "
      MIPI_CSI_0.vcst()[15],
    ",
  0x40347104u64 => "
      MIPI_CSI_0.vcsc()[0],
    ",
  0x40347114u64 => "
      MIPI_CSI_0.vcsc()[1],
    ",
  0x40347124u64 => "
      MIPI_CSI_0.vcsc()[2],
    ",
  0x40347134u64 => "
      MIPI_CSI_0.vcsc()[3],
    ",
  0x40347144u64 => "
      MIPI_CSI_0.vcsc()[4],
    ",
  0x40347154u64 => "
      MIPI_CSI_0.vcsc()[5],
    ",
  0x40347164u64 => "
      MIPI_CSI_0.vcsc()[6],
    ",
  0x40347174u64 => "
      MIPI_CSI_0.vcsc()[7],
    ",
  0x40347184u64 => "
      MIPI_CSI_0.vcsc()[8],
    ",
  0x40347194u64 => "
      MIPI_CSI_0.vcsc()[9],
    ",
  0x403471a4u64 => "
      MIPI_CSI_0.vcsc()[10],
    ",
  0x403471b4u64 => "
      MIPI_CSI_0.vcsc()[11],
    ",
  0x403471c4u64 => "
      MIPI_CSI_0.vcsc()[12],
    ",
  0x403471d4u64 => "
      MIPI_CSI_0.vcsc()[13],
    ",
  0x403471e4u64 => "
      MIPI_CSI_0.vcsc()[14],
    ",
  0x403471f4u64 => "
      MIPI_CSI_0.vcsc()[15],
    ",
  0x40347108u64 => "
      MIPI_CSI_0.vcie()[0],
    ",
  0x40347118u64 => "
      MIPI_CSI_0.vcie()[1],
    ",
  0x40347128u64 => "
      MIPI_CSI_0.vcie()[2],
    ",
  0x40347138u64 => "
      MIPI_CSI_0.vcie()[3],
    ",
  0x40347148u64 => "
      MIPI_CSI_0.vcie()[4],
    ",
  0x40347158u64 => "
      MIPI_CSI_0.vcie()[5],
    ",
  0x40347168u64 => "
      MIPI_CSI_0.vcie()[6],
    ",
  0x40347178u64 => "
      MIPI_CSI_0.vcie()[7],
    ",
  0x40347188u64 => "
      MIPI_CSI_0.vcie()[8],
    ",
  0x40347198u64 => "
      MIPI_CSI_0.vcie()[9],
    ",
  0x403471a8u64 => "
      MIPI_CSI_0.vcie()[10],
    ",
  0x403471b8u64 => "
      MIPI_CSI_0.vcie()[11],
    ",
  0x403471c8u64 => "
      MIPI_CSI_0.vcie()[12],
    ",
  0x403471d8u64 => "
      MIPI_CSI_0.vcie()[13],
    ",
  0x403471e8u64 => "
      MIPI_CSI_0.vcie()[14],
    ",
  0x403471f8u64 => "
      MIPI_CSI_0.vcie()[15],
    ",
  0x40347200u64 => "
      MIPI_CSI_0.pmst(),
    ",
  0x40347204u64 => "
      MIPI_CSI_0.pmsc(),
    ",
  0x40347208u64 => "
      MIPI_CSI_0.pmie(),
    ",
  0x40347280u64 => "
      MIPI_CSI_0.gsct(),
    ",
  0x40347284u64 => "
      MIPI_CSI_0.gsst(),
    ",
  0x40347288u64 => "
      MIPI_CSI_0.gssc(),
    ",
  0x4034728cu64 => "
      MIPI_CSI_0.gsie(),
    ",
  0x40347290u64 => "
      MIPI_CSI_0.gsht(),
    ",
  0x40347294u64 => "
      MIPI_CSI_0.gsiu(),
    ",
  0x40347400u64 => "
      VIN_0.mc(),
    ",
  0x40347404u64 => "
      VIN_0.ms(),
    ",
  0x40347408u64 => "
      VIN_0.fc(),
    ",
  0x4034740cu64 => "
      VIN_0.slprc(),
    ",
  0x40347410u64 => "
      VIN_0.elprc(),
    ",
  0x40347414u64 => "
      VIN_0.spprc(),
    ",
  0x40347418u64 => "
      VIN_0.epprc(),
    ",
  0x40347420u64 => "
      VIN_0.csi_ifmd(),
    ",
  0x40347424u64 => "
      VIN_0.csifld(),
    ",
  0x4034742cu64 => "
      VIN_0.is(),
    ",
  0x40347430u64 => "
      VIN_0.mb1(),
    ",
  0x40347434u64 => "
      VIN_0.mb2(),
    ",
  0x40347438u64 => "
      VIN_0.mb3(),
    ",
  0x4034743cu64 => "
      VIN_0.lc(),
    ",
  0x40347440u64 => "
      VIN_0.ie(),
    ",
  0x40347444u64 => "
      VIN_0.ints(),
    ",
  0x40347448u64 => "
      VIN_0.si(),
    ",
  0x40347454u64 => "
      VIN_0.mtcstop(),
    ",
  0x40347458u64 => "
      VIN_0.dmr(),
    ",
  0x40347460u64 => "
      VIN_0.uvaof(),
    ",
  0x40347480u64 => "
      VIN_0.uds_ctrl(),
    ",
  0x40347484u64 => "
      VIN_0.uds_scale(),
    ",
  0x40347490u64 => "
      VIN_0.uds_pass_bwidth(),
    ",
  0x403474a4u64 => "
      VIN_0.uds_clip_size(),
    ",
  0x40347500u64 => "
      VIN_0.lutp(),
    ",
  0x40347504u64 => "
      VIN_0.lutd(),
    ",
  0x40347628u64 => "
      VIN_0.yccr1(),
    ",
  0x4034762cu64 => "
      VIN_0.yccr2(),
    ",
  0x40347630u64 => "
      VIN_0.yccr3(),
    ",
  0x40347634u64 => "
      VIN_0.cbccr1(),
    ",
  0x40347638u64 => "
      VIN_0.cbccr2(),
    ",
  0x4034763cu64 => "
      VIN_0.cbccr3(),
    ",
  0x40347640u64 => "
      VIN_0.crccr1(),
    ",
  0x40347644u64 => "
      VIN_0.crccr2(),
    ",
  0x40347648u64 => "
      VIN_0.crccr3(),
    ",
  0x40347700u64 => "
      VIN_0.csce1(),
    ",
  0x40347704u64 => "
      VIN_0.csce2(),
    ",
  0x40347708u64 => "
      VIN_0.csce3(),
    ",
  0x4034770cu64 => "
      VIN_0.csce4(),
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
      USBHS.dfifo()[0],
      USBHS.dfifol()[0],
      USBHS.dfifoll()[0],
    ",
  0x4035101cu64 => "
      USBHS.dfifo()[1],
      USBHS.dfifol()[1],
      USBHS.dfifoll()[1],
    ",
  0x4035101au64 => "
      USBHS.d0fifoh(),
      USBHS.d1fifoh(),
    ",
  0x4035101bu64 => "
      USBHS.d0fifohh(),
      USBHS.d1fifohh(),
    ",
  0x40351020u64 => "
      USBHS.cfifosel(),
    ",
  0x40351022u64 => "
      USBHS.cfifoctr(),
    ",
  0x40351028u64 => "
      USBHS.dfifosel()[0],
    ",
  0x4035102cu64 => "
      USBHS.dfifosel()[1],
    ",
  0x4035102au64 => "
      USBHS.dfifoctr()[0],
    ",
  0x4035102eu64 => "
      USBHS.dfifoctr()[1],
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
  0x40351074u64 => "
      USBHS.pipectr()[1],
    ",
  0x40351078u64 => "
      USBHS.pipectr()[2],
    ",
  0x4035107cu64 => "
      USBHS.pipectr()[3],
    ",
  0x40351080u64 => "
      USBHS.pipectr()[4],
    ",
  0x40351084u64 => "
      USBHS.pipectr()[5],
    ",
  0x40351088u64 => "
      USBHS.pipectr()[6],
    ",
  0x4035108cu64 => "
      USBHS.pipectr()[7],
    ",
  0x40351090u64 => "
      USBHS.pipectr()[8],
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
  0x40358000u64 => "
      SCI_0_B.rdr(),
    ",
  0x40358004u64 => "
      SCI_0_B.tdr(),
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
  0x4035f000u64 => "
      I_3_C.prts(),
    ",
  0x4035f010u64 => "
      I_3_C.cectl(),
    ",
  0x4035f014u64 => "
      I_3_C.bctl(),
    ",
  0x4035f018u64 => "
      I_3_C.msdvad(),
    ",
  0x4035f020u64 => "
      I_3_C.rstctl(),
    ",
  0x4035f024u64 => "
      I_3_C.prsst(),
    ",
  0x4035f030u64 => "
      I_3_C.inst(),
    ",
  0x4035f034u64 => "
      I_3_C.inste(),
    ",
  0x4035f038u64 => "
      I_3_C.inie(),
    ",
  0x4035f03cu64 => "
      I_3_C.instfc(),
    ",
  0x4035f044u64 => "
      I_3_C.dvct(),
    ",
  0x4035f058u64 => "
      I_3_C.ibinctl(),
    ",
  0x4035f060u64 => "
      I_3_C.bfctl(),
    ",
  0x4035f064u64 => "
      I_3_C.svctl(),
    ",
  0x4035f070u64 => "
      I_3_C.refckctl(),
    ",
  0x4035f074u64 => "
      I_3_C.stdbr(),
    ",
  0x4035f078u64 => "
      I_3_C.extbr(),
    ",
  0x4035f07cu64 => "
      I_3_C.bfrecdt(),
    ",
  0x4035f080u64 => "
      I_3_C.bavlcdt(),
    ",
  0x4035f084u64 => "
      I_3_C.bidlcdt(),
    ",
  0x4035f088u64 => "
      I_3_C.outctl(),
    ",
  0x4035f08cu64 => "
      I_3_C.inctl(),
    ",
  0x4035f090u64 => "
      I_3_C.tmoctl(),
    ",
  0x4035f098u64 => "
      I_3_C.wuctl(),
    ",
  0x4035f0a0u64 => "
      I_3_C.ackctl(),
    ",
  0x4035f0a4u64 => "
      I_3_C.scstrctl(),
    ",
  0x4035f0b0u64 => "
      I_3_C.scstlctl(),
    ",
  0x4035f0c0u64 => "
      I_3_C.svtdlg0(),
    ",
  0x4035f120u64 => "
      I_3_C.stctl(),
    ",
  0x4035f124u64 => "
      I_3_C.atctl(),
    ",
  0x4035f128u64 => "
      I_3_C.attrg(),
    ",
  0x4035f12cu64 => "
      I_3_C.atccnte(),
    ",
  0x4035f140u64 => "
      I_3_C.cndctl(),
    ",
  0x4035f150u64 => "
      I_3_C.ncmdqp(),
    ",
  0x4035f154u64 => "
      I_3_C.nrspqp(),
    ",
  0x4035f158u64 => "
      I_3_C.ntdtbp0(),
      I_3_C.ntdtbp0_by(),
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
    ",
  0x4035f194u64 => "
      I_3_C.ntbthctl0(),
    ",
  0x4035f1c0u64 => "
      I_3_C.nrqthctl(),
    ",
  0x4035f1c4u64 => "
      I_3_C.hqthctl(),
    ",
  0x4035f1c8u64 => "
      I_3_C.htbthctl(),
    ",
  0x4035f1d0u64 => "
      I_3_C.bst(),
    ",
  0x4035f1d4u64 => "
      I_3_C.bste(),
    ",
  0x4035f1d8u64 => "
      I_3_C.bie(),
    ",
  0x4035f1dcu64 => "
      I_3_C.bstfc(),
    ",
  0x4035f1e0u64 => "
      I_3_C.ntst(),
    ",
  0x4035f1e4u64 => "
      I_3_C.ntste(),
    ",
  0x4035f1e8u64 => "
      I_3_C.ntie(),
    ",
  0x4035f1ecu64 => "
      I_3_C.ntstfc(),
    ",
  0x4035f200u64 => "
      I_3_C.htst(),
    ",
  0x4035f204u64 => "
      I_3_C.htste(),
    ",
  0x4035f208u64 => "
      I_3_C.htie(),
    ",
  0x4035f20cu64 => "
      I_3_C.htstfc(),
    ",
  0x4035f210u64 => "
      I_3_C.bcst(),
    ",
  0x4035f214u64 => "
      I_3_C.svst(),
    ",
  0x4035f218u64 => "
      I_3_C.wust(),
    ",
  0x4035f21cu64 => "
      I_3_C.mrccpt(),
    ",
  0x4035f224u64 => "
      I_3_C.datbas()[0],
    ",
  0x4035f22cu64 => "
      I_3_C.datbas()[1],
    ",
  0x4035f234u64 => "
      I_3_C.datbas()[2],
    ",
  0x4035f23cu64 => "
      I_3_C.datbas()[3],
    ",
  0x4035f244u64 => "
      I_3_C.datbas()[4],
    ",
  0x4035f24cu64 => "
      I_3_C.datbas()[5],
    ",
  0x4035f254u64 => "
      I_3_C.datbas()[6],
    ",
  0x4035f25cu64 => "
      I_3_C.datbas()[7],
    ",
  0x4035f2a0u64 => "
      I_3_C.exdatbas(),
    ",
  0x4035f2b0u64 => "
      I_3_C.sdatbas0(),
      I_3_C.sdatbas1(),
      I_3_C.sdatbas2(),
    ",
  0x4035f2d0u64 => "
      I_3_C.msdct()[0],
    ",
  0x4035f2d4u64 => "
      I_3_C.msdct()[1],
    ",
  0x4035f2d8u64 => "
      I_3_C.msdct()[2],
    ",
  0x4035f2dcu64 => "
      I_3_C.msdct()[3],
    ",
  0x4035f2e0u64 => "
      I_3_C.msdct()[4],
    ",
  0x4035f2e4u64 => "
      I_3_C.msdct()[5],
    ",
  0x4035f2e8u64 => "
      I_3_C.msdct()[6],
    ",
  0x4035f2ecu64 => "
      I_3_C.msdct()[7],
    ",
  0x4035f320u64 => "
      I_3_C.svdct(),
    ",
  0x4035f324u64 => "
      I_3_C.sdctpidl(),
    ",
  0x4035f328u64 => "
      I_3_C.sdctpidh(),
    ",
  0x4035f330u64 => "
      I_3_C.svdvad()[0],
    ",
  0x4035f334u64 => "
      I_3_C.svdvad()[1],
    ",
  0x4035f338u64 => "
      I_3_C.svdvad()[2],
    ",
  0x4035f350u64 => "
      I_3_C.csecmd(),
    ",
  0x4035f354u64 => "
      I_3_C.ceactst(),
    ",
  0x4035f358u64 => "
      I_3_C.cmwlg(),
    ",
  0x4035f35cu64 => "
      I_3_C.cmrlg(),
    ",
  0x4035f360u64 => "
      I_3_C.cetstmd(),
    ",
  0x4035f364u64 => "
      I_3_C.cgdvst(),
    ",
  0x4035f368u64 => "
      I_3_C.cmdspw(),
    ",
  0x4035f36cu64 => "
      I_3_C.cmdspr(),
    ",
  0x4035f370u64 => "
      I_3_C.cmdspt(),
    ",
  0x4035f374u64 => "
      I_3_C.cetsm(),
    ",
  0x4035f378u64 => "
      I_3_C.cetss(),
    ",
  0x4035f37cu64 => "
      I_3_C.cghdrcap(),
    ",
  0x4035f380u64 => "
      I_3_C.bitcnt(),
    ",
  0x4035f394u64 => "
      I_3_C.nqstlv(),
    ",
  0x4035f398u64 => "
      I_3_C.ndbstlv0(),
    ",
  0x4035f3c0u64 => "
      I_3_C.nrsqstlv(),
    ",
  0x4035f3c4u64 => "
      I_3_C.hqstlv(),
    ",
  0x4035f3c8u64 => "
      I_3_C.hdbstlv(),
    ",
  0x4035f3ccu64 => "
      I_3_C.prstdbg(),
    ",
  0x4035f3d0u64 => "
      I_3_C.mserrcnt(),
    ",
  0x4035f3e0u64 => "
      I_3_C.sc1cpt(),
    ",
  0x4035f3e4u64 => "
      I_3_C.sc2cpt(),
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
  0x403a0000u64 => "
      ESC.r#type(),
    ",
  0x403a0001u64 => "
      ESC.revision(),
    ",
  0x403a0002u64 => "
      ESC.build(),
    ",
  0x403a0004u64 => "
      ESC.fmmu_num(),
    ",
  0x403a0005u64 => "
      ESC.sync_manager(),
    ",
  0x403a0006u64 => "
      ESC.ram_size(),
    ",
  0x403a0007u64 => "
      ESC.port_desc(),
    ",
  0x403a0008u64 => "
      ESC.feature(),
    ",
  0x403a0010u64 => "
      ESC.station_adr(),
    ",
  0x403a0012u64 => "
      ESC.station_alias(),
    ",
  0x403a0020u64 => "
      ESC.wr_reg_enable(),
    ",
  0x403a0021u64 => "
      ESC.wr_reg_protect(),
    ",
  0x403a0030u64 => "
      ESC.esc_wr_enable(),
    ",
  0x403a0031u64 => "
      ESC.esc_wr_protect(),
    ",
  0x403a0040u64 => "
      ESC.esc_reset_ecat_r(),
      ESC.esc_reset_ecat_w(),
    ",
  0x403a0041u64 => "
      ESC.esc_reset_pdi_r(),
      ESC.esc_reset_pdi_w(),
    ",
  0x403a0100u64 => "
      ESC.esc_dl_control(),
    ",
  0x403a0108u64 => "
      ESC.physical_rw_offset(),
    ",
  0x403a0110u64 => "
      ESC.esc_dl_status(),
    ",
  0x403a0120u64 => "
      ESC.al_control(),
    ",
  0x403a0130u64 => "
      ESC.al_status(),
    ",
  0x403a0134u64 => "
      ESC.al_status_code(),
    ",
  0x403a0138u64 => "
      ESC.run_led_override(),
    ",
  0x403a0139u64 => "
      ESC.err_led_override(),
    ",
  0x403a0140u64 => "
      ESC.pdi_control(),
    ",
  0x403a0141u64 => "
      ESC.esc_config(),
    ",
  0x403a0150u64 => "
      ESC.pdi_config(),
    ",
  0x403a0151u64 => "
      ESC.sync_latch_config(),
    ",
  0x403a0152u64 => "
      ESC.ext_pdi_config(),
    ",
  0x403a0200u64 => "
      ESC.ecat_event_mask(),
    ",
  0x403a0204u64 => "
      ESC.al_event_mask(),
    ",
  0x403a0210u64 => "
      ESC.ecat_event_req(),
    ",
  0x403a0220u64 => "
      ESC.al_event_req(),
    ",
  0x403a0300u64 => "
      ESC.rx_err_count()[0],
    ",
  0x403a0302u64 => "
      ESC.rx_err_count()[1],
    ",
  0x403a0308u64 => "
      ESC.fwd_rx_err_count()[0],
    ",
  0x403a0309u64 => "
      ESC.fwd_rx_err_count()[1],
    ",
  0x403a030cu64 => "
      ESC.ecat_proc_err_count(),
    ",
  0x403a030du64 => "
      ESC.pdi_err_count(),
    ",
  0x403a0310u64 => "
      ESC.lost_link_count()[0],
    ",
  0x403a0311u64 => "
      ESC.lost_link_count()[1],
    ",
  0x403a0400u64 => "
      ESC.wd_divide(),
    ",
  0x403a0410u64 => "
      ESC.wdt_pdi(),
    ",
  0x403a0420u64 => "
      ESC.wdt_data(),
    ",
  0x403a0440u64 => "
      ESC.wds_data(),
    ",
  0x403a0442u64 => "
      ESC.wdc_data(),
    ",
  0x403a0443u64 => "
      ESC.wdc_pdi(),
    ",
  0x403a0500u64 => "
      ESC.eep_conf(),
    ",
  0x403a0501u64 => "
      ESC.eep_state(),
    ",
  0x403a0502u64 => "
      ESC.eep_cont_stat(),
    ",
  0x403a0504u64 => "
      ESC.eep_adr(),
    ",
  0x403a0508u64 => "
      ESC.eep_data(),
    ",
  0x403a0510u64 => "
      ESC.mii_cont_stat(),
    ",
  0x403a0512u64 => "
      ESC.phy_adr(),
    ",
  0x403a0513u64 => "
      ESC.phy_reg_adr(),
    ",
  0x403a0514u64 => "
      ESC.phy_data(),
    ",
  0x403a0516u64 => "
      ESC.mii_ecat_acs_stat(),
    ",
  0x403a0517u64 => "
      ESC.mii_pdi_acs_stat(),
    ",
  0x403a0600u64 => "
      ESC.fmmu_l_start_adr()[0],
    ",
  0x403a0610u64 => "
      ESC.fmmu_l_start_adr()[1],
    ",
  0x403a0620u64 => "
      ESC.fmmu_l_start_adr()[2],
    ",
  0x403a0630u64 => "
      ESC.fmmu_l_start_adr()[3],
    ",
  0x403a0640u64 => "
      ESC.fmmu_l_start_adr()[4],
    ",
  0x403a0650u64 => "
      ESC.fmmu_l_start_adr()[5],
    ",
  0x403a0660u64 => "
      ESC.fmmu_l_start_adr()[6],
    ",
  0x403a0670u64 => "
      ESC.fmmu_l_start_adr()[7],
    ",
  0x403a0604u64 => "
      ESC.fmmu_len()[0],
    ",
  0x403a0614u64 => "
      ESC.fmmu_len()[1],
    ",
  0x403a0624u64 => "
      ESC.fmmu_len()[2],
    ",
  0x403a0634u64 => "
      ESC.fmmu_len()[3],
    ",
  0x403a0644u64 => "
      ESC.fmmu_len()[4],
    ",
  0x403a0654u64 => "
      ESC.fmmu_len()[5],
    ",
  0x403a0664u64 => "
      ESC.fmmu_len()[6],
    ",
  0x403a0674u64 => "
      ESC.fmmu_len()[7],
    ",
  0x403a0606u64 => "
      ESC.fmmu_l_start_bit()[0],
    ",
  0x403a0616u64 => "
      ESC.fmmu_l_start_bit()[1],
    ",
  0x403a0626u64 => "
      ESC.fmmu_l_start_bit()[2],
    ",
  0x403a0636u64 => "
      ESC.fmmu_l_start_bit()[3],
    ",
  0x403a0646u64 => "
      ESC.fmmu_l_start_bit()[4],
    ",
  0x403a0656u64 => "
      ESC.fmmu_l_start_bit()[5],
    ",
  0x403a0666u64 => "
      ESC.fmmu_l_start_bit()[6],
    ",
  0x403a0676u64 => "
      ESC.fmmu_l_start_bit()[7],
    ",
  0x403a0607u64 => "
      ESC.fmmu_l_stop_bit()[0],
    ",
  0x403a0617u64 => "
      ESC.fmmu_l_stop_bit()[1],
    ",
  0x403a0627u64 => "
      ESC.fmmu_l_stop_bit()[2],
    ",
  0x403a0637u64 => "
      ESC.fmmu_l_stop_bit()[3],
    ",
  0x403a0647u64 => "
      ESC.fmmu_l_stop_bit()[4],
    ",
  0x403a0657u64 => "
      ESC.fmmu_l_stop_bit()[5],
    ",
  0x403a0667u64 => "
      ESC.fmmu_l_stop_bit()[6],
    ",
  0x403a0677u64 => "
      ESC.fmmu_l_stop_bit()[7],
    ",
  0x403a0608u64 => "
      ESC.fmmu_p_start_adr()[0],
    ",
  0x403a0618u64 => "
      ESC.fmmu_p_start_adr()[1],
    ",
  0x403a0628u64 => "
      ESC.fmmu_p_start_adr()[2],
    ",
  0x403a0638u64 => "
      ESC.fmmu_p_start_adr()[3],
    ",
  0x403a0648u64 => "
      ESC.fmmu_p_start_adr()[4],
    ",
  0x403a0658u64 => "
      ESC.fmmu_p_start_adr()[5],
    ",
  0x403a0668u64 => "
      ESC.fmmu_p_start_adr()[6],
    ",
  0x403a0678u64 => "
      ESC.fmmu_p_start_adr()[7],
    ",
  0x403a060au64 => "
      ESC.fmmu_p_start_bit()[0],
    ",
  0x403a061au64 => "
      ESC.fmmu_p_start_bit()[1],
    ",
  0x403a062au64 => "
      ESC.fmmu_p_start_bit()[2],
    ",
  0x403a063au64 => "
      ESC.fmmu_p_start_bit()[3],
    ",
  0x403a064au64 => "
      ESC.fmmu_p_start_bit()[4],
    ",
  0x403a065au64 => "
      ESC.fmmu_p_start_bit()[5],
    ",
  0x403a066au64 => "
      ESC.fmmu_p_start_bit()[6],
    ",
  0x403a067au64 => "
      ESC.fmmu_p_start_bit()[7],
    ",
  0x403a060bu64 => "
      ESC.fmmu_type()[0],
    ",
  0x403a061bu64 => "
      ESC.fmmu_type()[1],
    ",
  0x403a062bu64 => "
      ESC.fmmu_type()[2],
    ",
  0x403a063bu64 => "
      ESC.fmmu_type()[3],
    ",
  0x403a064bu64 => "
      ESC.fmmu_type()[4],
    ",
  0x403a065bu64 => "
      ESC.fmmu_type()[5],
    ",
  0x403a066bu64 => "
      ESC.fmmu_type()[6],
    ",
  0x403a067bu64 => "
      ESC.fmmu_type()[7],
    ",
  0x403a060cu64 => "
      ESC.fmmu_act()[0],
    ",
  0x403a061cu64 => "
      ESC.fmmu_act()[1],
    ",
  0x403a062cu64 => "
      ESC.fmmu_act()[2],
    ",
  0x403a063cu64 => "
      ESC.fmmu_act()[3],
    ",
  0x403a064cu64 => "
      ESC.fmmu_act()[4],
    ",
  0x403a065cu64 => "
      ESC.fmmu_act()[5],
    ",
  0x403a066cu64 => "
      ESC.fmmu_act()[6],
    ",
  0x403a067cu64 => "
      ESC.fmmu_act()[7],
    ",
  0x403a0800u64 => "
      ESC.sm_p_start_adr()[0],
    ",
  0x403a0808u64 => "
      ESC.sm_p_start_adr()[1],
    ",
  0x403a0810u64 => "
      ESC.sm_p_start_adr()[2],
    ",
  0x403a0818u64 => "
      ESC.sm_p_start_adr()[3],
    ",
  0x403a0820u64 => "
      ESC.sm_p_start_adr()[4],
    ",
  0x403a0828u64 => "
      ESC.sm_p_start_adr()[5],
    ",
  0x403a0830u64 => "
      ESC.sm_p_start_adr()[6],
    ",
  0x403a0838u64 => "
      ESC.sm_p_start_adr()[7],
    ",
  0x403a0802u64 => "
      ESC.sm_len()[0],
    ",
  0x403a080au64 => "
      ESC.sm_len()[1],
    ",
  0x403a0812u64 => "
      ESC.sm_len()[2],
    ",
  0x403a081au64 => "
      ESC.sm_len()[3],
    ",
  0x403a0822u64 => "
      ESC.sm_len()[4],
    ",
  0x403a082au64 => "
      ESC.sm_len()[5],
    ",
  0x403a0832u64 => "
      ESC.sm_len()[6],
    ",
  0x403a083au64 => "
      ESC.sm_len()[7],
    ",
  0x403a0804u64 => "
      ESC.sm_control()[0],
    ",
  0x403a080cu64 => "
      ESC.sm_control()[1],
    ",
  0x403a0814u64 => "
      ESC.sm_control()[2],
    ",
  0x403a081cu64 => "
      ESC.sm_control()[3],
    ",
  0x403a0824u64 => "
      ESC.sm_control()[4],
    ",
  0x403a082cu64 => "
      ESC.sm_control()[5],
    ",
  0x403a0834u64 => "
      ESC.sm_control()[6],
    ",
  0x403a083cu64 => "
      ESC.sm_control()[7],
    ",
  0x403a0805u64 => "
      ESC.sm_status()[0],
    ",
  0x403a080du64 => "
      ESC.sm_status()[1],
    ",
  0x403a0815u64 => "
      ESC.sm_status()[2],
    ",
  0x403a081du64 => "
      ESC.sm_status()[3],
    ",
  0x403a0825u64 => "
      ESC.sm_status()[4],
    ",
  0x403a082du64 => "
      ESC.sm_status()[5],
    ",
  0x403a0835u64 => "
      ESC.sm_status()[6],
    ",
  0x403a083du64 => "
      ESC.sm_status()[7],
    ",
  0x403a0806u64 => "
      ESC.sm_act()[0],
    ",
  0x403a080eu64 => "
      ESC.sm_act()[1],
    ",
  0x403a0816u64 => "
      ESC.sm_act()[2],
    ",
  0x403a081eu64 => "
      ESC.sm_act()[3],
    ",
  0x403a0826u64 => "
      ESC.sm_act()[4],
    ",
  0x403a082eu64 => "
      ESC.sm_act()[5],
    ",
  0x403a0836u64 => "
      ESC.sm_act()[6],
    ",
  0x403a083eu64 => "
      ESC.sm_act()[7],
    ",
  0x403a0807u64 => "
      ESC.sm_pdi_cont()[0],
    ",
  0x403a080fu64 => "
      ESC.sm_pdi_cont()[1],
    ",
  0x403a0817u64 => "
      ESC.sm_pdi_cont()[2],
    ",
  0x403a081fu64 => "
      ESC.sm_pdi_cont()[3],
    ",
  0x403a0827u64 => "
      ESC.sm_pdi_cont()[4],
    ",
  0x403a082fu64 => "
      ESC.sm_pdi_cont()[5],
    ",
  0x403a0837u64 => "
      ESC.sm_pdi_cont()[6],
    ",
  0x403a083fu64 => "
      ESC.sm_pdi_cont()[7],
    ",
  0x403a0900u64 => "
      ESC.dc_rcv_time_port0(),
    ",
  0x403a0904u64 => "
      ESC.dc_rcv_time_port1(),
    ",
  0x403a0908u64 => "
      ESC.dc_rcv_time_port2(),
    ",
  0x403a0910u64 => "
      ESC.dc_sys_time_l(),
    ",
  0x403a0914u64 => "
      ESC.dc_sys_time_h(),
    ",
  0x403a0918u64 => "
      ESC.dc_rcv_time_unit_l(),
    ",
  0x403a091cu64 => "
      ESC.dc_rcv_time_unit_h(),
    ",
  0x403a0920u64 => "
      ESC.dc_sys_time_offset_l(),
    ",
  0x403a0924u64 => "
      ESC.dc_sys_time_offset_h(),
    ",
  0x403a0928u64 => "
      ESC.dc_sys_time_delay(),
    ",
  0x403a092cu64 => "
      ESC.dc_sys_time_diff(),
    ",
  0x403a0930u64 => "
      ESC.dc_speed_count_start(),
    ",
  0x403a0932u64 => "
      ESC.dc_speed_count_diff(),
    ",
  0x403a0934u64 => "
      ESC.dc_sys_time_diff_fil_depth(),
    ",
  0x403a0935u64 => "
      ESC.dc_speed_count_fil_depth(),
    ",
  0x403a0980u64 => "
      ESC.dc_cyc_cont(),
    ",
  0x403a0981u64 => "
      ESC.dc_act(),
    ",
  0x403a0982u64 => "
      ESC.dc_pulse_len(),
    ",
  0x403a0984u64 => "
      ESC.dc_act_stat(),
    ",
  0x403a098eu64 => "
      ESC.dc_sync0_stat(),
    ",
  0x403a098fu64 => "
      ESC.dc_sync1_stat(),
    ",
  0x403a0990u64 => "
      ESC.dc_cyc_start_time_l(),
    ",
  0x403a0994u64 => "
      ESC.dc_cyc_start_time_h(),
    ",
  0x403a0998u64 => "
      ESC.dc_next_sync1_pulse_l(),
    ",
  0x403a099cu64 => "
      ESC.dc_next_sync1_pulse_h(),
    ",
  0x403a09a0u64 => "
      ESC.dc_sync0_cyc_time(),
    ",
  0x403a09a4u64 => "
      ESC.dc_sync1_cyc_time(),
    ",
  0x403a09a8u64 => "
      ESC.dc_latch0_cont(),
    ",
  0x403a09a9u64 => "
      ESC.dc_latch1_cont(),
    ",
  0x403a09aeu64 => "
      ESC.dc_latch0_stat(),
    ",
  0x403a09afu64 => "
      ESC.dc_latch1_stat(),
    ",
  0x403a09b0u64 => "
      ESC.dc_latch0_time_pos_l(),
    ",
  0x403a09b4u64 => "
      ESC.dc_latch0_time_pos_h(),
    ",
  0x403a09b8u64 => "
      ESC.dc_latch0_time_neg_l(),
    ",
  0x403a09bcu64 => "
      ESC.dc_latch0_time_neg_h(),
    ",
  0x403a09c0u64 => "
      ESC.dc_latch1_time_pos_l(),
    ",
  0x403a09c4u64 => "
      ESC.dc_latch1_time_pos_h(),
    ",
  0x403a09c8u64 => "
      ESC.dc_latch1_time_neg_l(),
    ",
  0x403a09ccu64 => "
      ESC.dc_latch1_time_neg_h(),
    ",
  0x403a09f0u64 => "
      ESC.dc_ecat_cng_ev_time(),
    ",
  0x403a09f8u64 => "
      ESC.dc_pdi_start_ev_time(),
    ",
  0x403a09fcu64 => "
      ESC.dc_pdi_cng_ev_time(),
    ",
  0x403a0e00u64 => "
      ESC.product_id_l(),
    ",
  0x403a0e04u64 => "
      ESC.product_id_h(),
    ",
  0x403a0e08u64 => "
      ESC.vendor_id_l(),
    ",
  0x403a4000u64 => "
      ESC_INI.escrst(),
    ",
  0x403a4010u64 => "
      ESC_INI.phylink(),
    ",
  0x403a4014u64 => "
      ESC_INI.escicr(),
    ",
  0x403a4018u64 => "
      ESC_INI.ecatoffadr(),
    ",
  0x403a401cu64 => "
      ESC_INI.ecatopmod(),
    ",
  0x403a4020u64 => "
      ESC_INI.ecatdbgc(),
    ",
  0x403c0000u64 => "
      MFWD.fwgc(),
    ",
  0x403c0010u64 => "
      MFWD.fwttc0(),
    ",
  0x403c0014u64 => "
      MFWD.fwttc1(),
    ",
  0x403c0020u64 => "
      MFWD.fwceptc(),
    ",
  0x403c0024u64 => "
      MFWD.fwceprc0(),
    ",
  0x403c0028u64 => "
      MFWD.fwceprc1(),
    ",
  0x403c002cu64 => "
      MFWD.fwceprc2(),
    ",
  0x403c0030u64 => "
      MFWD.fwclptc(),
    ",
  0x403c0034u64 => "
      MFWD.fwclprc(),
    ",
  0x403c0040u64 => "
      MFWD.fwcmptc(),
    ",
  0x403c0044u64 => "
      MFWD.fwemptc(),
    ",
  0x403c0050u64 => "
      MFWD.fwsdmptc(),
    ",
  0x403c0054u64 => "
      MFWD.fwsdmpvc(),
    ",
  0x403c0080u64 => "
      MFWD.fwlbwmc()[0],
    ",
  0x403c0084u64 => "
      MFWD.fwlbwmc()[1],
    ",
  0x403c0088u64 => "
      MFWD.fwlbwmc()[2],
    ",
  0x403c0100u64 => "
      MFWD.fwpc0()[0],
    ",
  0x403c0110u64 => "
      MFWD.fwpc0()[1],
    ",
  0x403c0120u64 => "
      MFWD.fwpc0()[2],
    ",
  0x403c0104u64 => "
      MFWD.fwpc1()[0],
    ",
  0x403c0114u64 => "
      MFWD.fwpc1()[1],
    ",
  0x403c0124u64 => "
      MFWD.fwpc1()[2],
    ",
  0x403c0108u64 => "
      MFWD.fwpc2()[0],
    ",
  0x403c0118u64 => "
      MFWD.fwpc2()[1],
    ",
  0x403c0128u64 => "
      MFWD.fwpc2()[2],
    ",
  0x403c0400u64 => "
      MFWD.fwctgc0()[0],
    ",
  0x403c0440u64 => "
      MFWD.fwctgc0()[1],
    ",
  0x403c0480u64 => "
      MFWD.fwctgc0()[2],
    ",
  0x403c04c0u64 => "
      MFWD.fwctgc0()[3],
    ",
  0x403c0500u64 => "
      MFWD.fwctgc0()[4],
    ",
  0x403c0540u64 => "
      MFWD.fwctgc0()[5],
    ",
  0x403c0580u64 => "
      MFWD.fwctgc0()[6],
    ",
  0x403c05c0u64 => "
      MFWD.fwctgc0()[7],
    ",
  0x403c0404u64 => "
      MFWD.fwctgc1()[0],
    ",
  0x403c0444u64 => "
      MFWD.fwctgc1()[1],
    ",
  0x403c0484u64 => "
      MFWD.fwctgc1()[2],
    ",
  0x403c04c4u64 => "
      MFWD.fwctgc1()[3],
    ",
  0x403c0504u64 => "
      MFWD.fwctgc1()[4],
    ",
  0x403c0544u64 => "
      MFWD.fwctgc1()[5],
    ",
  0x403c0584u64 => "
      MFWD.fwctgc1()[6],
    ",
  0x403c05c4u64 => "
      MFWD.fwctgc1()[7],
    ",
  0x403c0408u64 => "
      MFWD.fwcttc0()[0],
    ",
  0x403c0448u64 => "
      MFWD.fwcttc0()[1],
    ",
  0x403c0488u64 => "
      MFWD.fwcttc0()[2],
    ",
  0x403c04c8u64 => "
      MFWD.fwcttc0()[3],
    ",
  0x403c0508u64 => "
      MFWD.fwcttc0()[4],
    ",
  0x403c0548u64 => "
      MFWD.fwcttc0()[5],
    ",
  0x403c0588u64 => "
      MFWD.fwcttc0()[6],
    ",
  0x403c05c8u64 => "
      MFWD.fwcttc0()[7],
    ",
  0x403c040cu64 => "
      MFWD.fwcttc1()[0],
    ",
  0x403c044cu64 => "
      MFWD.fwcttc1()[1],
    ",
  0x403c048cu64 => "
      MFWD.fwcttc1()[2],
    ",
  0x403c04ccu64 => "
      MFWD.fwcttc1()[3],
    ",
  0x403c050cu64 => "
      MFWD.fwcttc1()[4],
    ",
  0x403c054cu64 => "
      MFWD.fwcttc1()[5],
    ",
  0x403c058cu64 => "
      MFWD.fwcttc1()[6],
    ",
  0x403c05ccu64 => "
      MFWD.fwcttc1()[7],
    ",
  0x403c0410u64 => "
      MFWD.fwcttc20()[0],
    ",
  0x403c0450u64 => "
      MFWD.fwcttc20()[1],
    ",
  0x403c0490u64 => "
      MFWD.fwcttc20()[2],
    ",
  0x403c04d0u64 => "
      MFWD.fwcttc20()[3],
    ",
  0x403c0510u64 => "
      MFWD.fwcttc20()[4],
    ",
  0x403c0550u64 => "
      MFWD.fwcttc20()[5],
    ",
  0x403c0590u64 => "
      MFWD.fwcttc20()[6],
    ",
  0x403c05d0u64 => "
      MFWD.fwcttc20()[7],
    ",
  0x403c0420u64 => "
      MFWD.fwctsc0()[0],
    ",
  0x403c0460u64 => "
      MFWD.fwctsc0()[1],
    ",
  0x403c04a0u64 => "
      MFWD.fwctsc0()[2],
    ",
  0x403c04e0u64 => "
      MFWD.fwctsc0()[3],
    ",
  0x403c0520u64 => "
      MFWD.fwctsc0()[4],
    ",
  0x403c0560u64 => "
      MFWD.fwctsc0()[5],
    ",
  0x403c05a0u64 => "
      MFWD.fwctsc0()[6],
    ",
  0x403c05e0u64 => "
      MFWD.fwctsc0()[7],
    ",
  0x403c0424u64 => "
      MFWD.fwctsc1()[0],
    ",
  0x403c0464u64 => "
      MFWD.fwctsc1()[1],
    ",
  0x403c04a4u64 => "
      MFWD.fwctsc1()[2],
    ",
  0x403c04e4u64 => "
      MFWD.fwctsc1()[3],
    ",
  0x403c0524u64 => "
      MFWD.fwctsc1()[4],
    ",
  0x403c0564u64 => "
      MFWD.fwctsc1()[5],
    ",
  0x403c05a4u64 => "
      MFWD.fwctsc1()[6],
    ",
  0x403c05e4u64 => "
      MFWD.fwctsc1()[7],
    ",
  0x403c0428u64 => "
      MFWD.fwctsc2()[0],
    ",
  0x403c0468u64 => "
      MFWD.fwctsc2()[1],
    ",
  0x403c04a8u64 => "
      MFWD.fwctsc2()[2],
    ",
  0x403c04e8u64 => "
      MFWD.fwctsc2()[3],
    ",
  0x403c0528u64 => "
      MFWD.fwctsc2()[4],
    ",
  0x403c0568u64 => "
      MFWD.fwctsc2()[5],
    ",
  0x403c05a8u64 => "
      MFWD.fwctsc2()[6],
    ",
  0x403c05e8u64 => "
      MFWD.fwctsc2()[7],
    ",
  0x403c042cu64 => "
      MFWD.fwctsc3()[0],
    ",
  0x403c046cu64 => "
      MFWD.fwctsc3()[1],
    ",
  0x403c04acu64 => "
      MFWD.fwctsc3()[2],
    ",
  0x403c04ecu64 => "
      MFWD.fwctsc3()[3],
    ",
  0x403c052cu64 => "
      MFWD.fwctsc3()[4],
    ",
  0x403c056cu64 => "
      MFWD.fwctsc3()[5],
    ",
  0x403c05acu64 => "
      MFWD.fwctsc3()[6],
    ",
  0x403c05ecu64 => "
      MFWD.fwctsc3()[7],
    ",
  0x403c0430u64 => "
      MFWD.fwctsc4()[0],
    ",
  0x403c0470u64 => "
      MFWD.fwctsc4()[1],
    ",
  0x403c04b0u64 => "
      MFWD.fwctsc4()[2],
    ",
  0x403c04f0u64 => "
      MFWD.fwctsc4()[3],
    ",
  0x403c0530u64 => "
      MFWD.fwctsc4()[4],
    ",
  0x403c0570u64 => "
      MFWD.fwctsc4()[5],
    ",
  0x403c05b0u64 => "
      MFWD.fwctsc4()[6],
    ",
  0x403c05f0u64 => "
      MFWD.fwctsc4()[7],
    ",
  0x403c1000u64 => "
      MFWD.fwtwbfc()[0],
    ",
  0x403c1010u64 => "
      MFWD.fwtwbfc()[1],
    ",
  0x403c1020u64 => "
      MFWD.fwtwbfc()[2],
    ",
  0x403c1030u64 => "
      MFWD.fwtwbfc()[3],
    ",
  0x403c1040u64 => "
      MFWD.fwtwbfc()[4],
    ",
  0x403c1050u64 => "
      MFWD.fwtwbfc()[5],
    ",
  0x403c1060u64 => "
      MFWD.fwtwbfc()[6],
    ",
  0x403c1070u64 => "
      MFWD.fwtwbfc()[7],
    ",
  0x403c1080u64 => "
      MFWD.fwtwbfc()[8],
    ",
  0x403c1090u64 => "
      MFWD.fwtwbfc()[9],
    ",
  0x403c10a0u64 => "
      MFWD.fwtwbfc()[10],
    ",
  0x403c10b0u64 => "
      MFWD.fwtwbfc()[11],
    ",
  0x403c10c0u64 => "
      MFWD.fwtwbfc()[12],
    ",
  0x403c10d0u64 => "
      MFWD.fwtwbfc()[13],
    ",
  0x403c10e0u64 => "
      MFWD.fwtwbfc()[14],
    ",
  0x403c10f0u64 => "
      MFWD.fwtwbfc()[15],
    ",
  0x403c1004u64 => "
      MFWD.fwtwbfvc()[0],
    ",
  0x403c1014u64 => "
      MFWD.fwtwbfvc()[1],
    ",
  0x403c1024u64 => "
      MFWD.fwtwbfvc()[2],
    ",
  0x403c1034u64 => "
      MFWD.fwtwbfvc()[3],
    ",
  0x403c1044u64 => "
      MFWD.fwtwbfvc()[4],
    ",
  0x403c1054u64 => "
      MFWD.fwtwbfvc()[5],
    ",
  0x403c1064u64 => "
      MFWD.fwtwbfvc()[6],
    ",
  0x403c1074u64 => "
      MFWD.fwtwbfvc()[7],
    ",
  0x403c1084u64 => "
      MFWD.fwtwbfvc()[8],
    ",
  0x403c1094u64 => "
      MFWD.fwtwbfvc()[9],
    ",
  0x403c10a4u64 => "
      MFWD.fwtwbfvc()[10],
    ",
  0x403c10b4u64 => "
      MFWD.fwtwbfvc()[11],
    ",
  0x403c10c4u64 => "
      MFWD.fwtwbfvc()[12],
    ",
  0x403c10d4u64 => "
      MFWD.fwtwbfvc()[13],
    ",
  0x403c10e4u64 => "
      MFWD.fwtwbfvc()[14],
    ",
  0x403c10f4u64 => "
      MFWD.fwtwbfvc()[15],
    ",
  0x403c1400u64 => "
      MFWD.fwthbfc()[0],
    ",
  0x403c1410u64 => "
      MFWD.fwthbfc()[1],
    ",
  0x403c1420u64 => "
      MFWD.fwthbfc()[2],
    ",
  0x403c1430u64 => "
      MFWD.fwthbfc()[3],
    ",
  0x403c1440u64 => "
      MFWD.fwthbfc()[4],
    ",
  0x403c1450u64 => "
      MFWD.fwthbfc()[5],
    ",
  0x403c1460u64 => "
      MFWD.fwthbfc()[6],
    ",
  0x403c1470u64 => "
      MFWD.fwthbfc()[7],
    ",
  0x403c1480u64 => "
      MFWD.fwthbfc()[8],
    ",
  0x403c1490u64 => "
      MFWD.fwthbfc()[9],
    ",
  0x403c14a0u64 => "
      MFWD.fwthbfc()[10],
    ",
  0x403c14b0u64 => "
      MFWD.fwthbfc()[11],
    ",
  0x403c14c0u64 => "
      MFWD.fwthbfc()[12],
    ",
  0x403c14d0u64 => "
      MFWD.fwthbfc()[13],
    ",
  0x403c14e0u64 => "
      MFWD.fwthbfc()[14],
    ",
  0x403c14f0u64 => "
      MFWD.fwthbfc()[15],
    ",
  0x403c1404u64 => "
      MFWD.fwthbfv0c()[0],
    ",
  0x403c1414u64 => "
      MFWD.fwthbfv0c()[1],
    ",
  0x403c1424u64 => "
      MFWD.fwthbfv0c()[2],
    ",
  0x403c1434u64 => "
      MFWD.fwthbfv0c()[3],
    ",
  0x403c1444u64 => "
      MFWD.fwthbfv0c()[4],
    ",
  0x403c1454u64 => "
      MFWD.fwthbfv0c()[5],
    ",
  0x403c1464u64 => "
      MFWD.fwthbfv0c()[6],
    ",
  0x403c1474u64 => "
      MFWD.fwthbfv0c()[7],
    ",
  0x403c1484u64 => "
      MFWD.fwthbfv0c()[8],
    ",
  0x403c1494u64 => "
      MFWD.fwthbfv0c()[9],
    ",
  0x403c14a4u64 => "
      MFWD.fwthbfv0c()[10],
    ",
  0x403c14b4u64 => "
      MFWD.fwthbfv0c()[11],
    ",
  0x403c14c4u64 => "
      MFWD.fwthbfv0c()[12],
    ",
  0x403c14d4u64 => "
      MFWD.fwthbfv0c()[13],
    ",
  0x403c14e4u64 => "
      MFWD.fwthbfv0c()[14],
    ",
  0x403c14f4u64 => "
      MFWD.fwthbfv0c()[15],
    ",
  0x403c1408u64 => "
      MFWD.fwthbfv1c()[0],
    ",
  0x403c1418u64 => "
      MFWD.fwthbfv1c()[1],
    ",
  0x403c1428u64 => "
      MFWD.fwthbfv1c()[2],
    ",
  0x403c1438u64 => "
      MFWD.fwthbfv1c()[3],
    ",
  0x403c1448u64 => "
      MFWD.fwthbfv1c()[4],
    ",
  0x403c1458u64 => "
      MFWD.fwthbfv1c()[5],
    ",
  0x403c1468u64 => "
      MFWD.fwthbfv1c()[6],
    ",
  0x403c1478u64 => "
      MFWD.fwthbfv1c()[7],
    ",
  0x403c1488u64 => "
      MFWD.fwthbfv1c()[8],
    ",
  0x403c1498u64 => "
      MFWD.fwthbfv1c()[9],
    ",
  0x403c14a8u64 => "
      MFWD.fwthbfv1c()[10],
    ",
  0x403c14b8u64 => "
      MFWD.fwthbfv1c()[11],
    ",
  0x403c14c8u64 => "
      MFWD.fwthbfv1c()[12],
    ",
  0x403c14d8u64 => "
      MFWD.fwthbfv1c()[13],
    ",
  0x403c14e8u64 => "
      MFWD.fwthbfv1c()[14],
    ",
  0x403c14f8u64 => "
      MFWD.fwthbfv1c()[15],
    ",
  0x403c1800u64 => "
      MFWD.fwfobfc()[0],
    ",
  0x403c1810u64 => "
      MFWD.fwfobfc()[1],
    ",
  0x403c1820u64 => "
      MFWD.fwfobfc()[2],
    ",
  0x403c1830u64 => "
      MFWD.fwfobfc()[3],
    ",
  0x403c1840u64 => "
      MFWD.fwfobfc()[4],
    ",
  0x403c1850u64 => "
      MFWD.fwfobfc()[5],
    ",
  0x403c1860u64 => "
      MFWD.fwfobfc()[6],
    ",
  0x403c1870u64 => "
      MFWD.fwfobfc()[7],
    ",
  0x403c1880u64 => "
      MFWD.fwfobfc()[8],
    ",
  0x403c1890u64 => "
      MFWD.fwfobfc()[9],
    ",
  0x403c18a0u64 => "
      MFWD.fwfobfc()[10],
    ",
  0x403c18b0u64 => "
      MFWD.fwfobfc()[11],
    ",
  0x403c18c0u64 => "
      MFWD.fwfobfc()[12],
    ",
  0x403c18d0u64 => "
      MFWD.fwfobfc()[13],
    ",
  0x403c18e0u64 => "
      MFWD.fwfobfc()[14],
    ",
  0x403c18f0u64 => "
      MFWD.fwfobfc()[15],
    ",
  0x403c1804u64 => "
      MFWD.fwfobfv0c()[0],
    ",
  0x403c1814u64 => "
      MFWD.fwfobfv0c()[1],
    ",
  0x403c1824u64 => "
      MFWD.fwfobfv0c()[2],
    ",
  0x403c1834u64 => "
      MFWD.fwfobfv0c()[3],
    ",
  0x403c1844u64 => "
      MFWD.fwfobfv0c()[4],
    ",
  0x403c1854u64 => "
      MFWD.fwfobfv0c()[5],
    ",
  0x403c1864u64 => "
      MFWD.fwfobfv0c()[6],
    ",
  0x403c1874u64 => "
      MFWD.fwfobfv0c()[7],
    ",
  0x403c1884u64 => "
      MFWD.fwfobfv0c()[8],
    ",
  0x403c1894u64 => "
      MFWD.fwfobfv0c()[9],
    ",
  0x403c18a4u64 => "
      MFWD.fwfobfv0c()[10],
    ",
  0x403c18b4u64 => "
      MFWD.fwfobfv0c()[11],
    ",
  0x403c18c4u64 => "
      MFWD.fwfobfv0c()[12],
    ",
  0x403c18d4u64 => "
      MFWD.fwfobfv0c()[13],
    ",
  0x403c18e4u64 => "
      MFWD.fwfobfv0c()[14],
    ",
  0x403c18f4u64 => "
      MFWD.fwfobfv0c()[15],
    ",
  0x403c1808u64 => "
      MFWD.fwfobfv1c()[0],
    ",
  0x403c1818u64 => "
      MFWD.fwfobfv1c()[1],
    ",
  0x403c1828u64 => "
      MFWD.fwfobfv1c()[2],
    ",
  0x403c1838u64 => "
      MFWD.fwfobfv1c()[3],
    ",
  0x403c1848u64 => "
      MFWD.fwfobfv1c()[4],
    ",
  0x403c1858u64 => "
      MFWD.fwfobfv1c()[5],
    ",
  0x403c1868u64 => "
      MFWD.fwfobfv1c()[6],
    ",
  0x403c1878u64 => "
      MFWD.fwfobfv1c()[7],
    ",
  0x403c1888u64 => "
      MFWD.fwfobfv1c()[8],
    ",
  0x403c1898u64 => "
      MFWD.fwfobfv1c()[9],
    ",
  0x403c18a8u64 => "
      MFWD.fwfobfv1c()[10],
    ",
  0x403c18b8u64 => "
      MFWD.fwfobfv1c()[11],
    ",
  0x403c18c8u64 => "
      MFWD.fwfobfv1c()[12],
    ",
  0x403c18d8u64 => "
      MFWD.fwfobfv1c()[13],
    ",
  0x403c18e8u64 => "
      MFWD.fwfobfv1c()[14],
    ",
  0x403c18f8u64 => "
      MFWD.fwfobfv1c()[15],
    ",
  0x403c1c00u64 => "
      MFWD.fwrfc()[0],
    ",
  0x403c1c10u64 => "
      MFWD.fwrfc()[1],
    ",
  0x403c1c20u64 => "
      MFWD.fwrfc()[2],
    ",
  0x403c1c30u64 => "
      MFWD.fwrfc()[3],
    ",
  0x403c1c40u64 => "
      MFWD.fwrfc()[4],
    ",
  0x403c1c50u64 => "
      MFWD.fwrfc()[5],
    ",
  0x403c1c60u64 => "
      MFWD.fwrfc()[6],
    ",
  0x403c1c70u64 => "
      MFWD.fwrfc()[7],
    ",
  0x403c1c80u64 => "
      MFWD.fwrfc()[8],
    ",
  0x403c1c90u64 => "
      MFWD.fwrfc()[9],
    ",
  0x403c1ca0u64 => "
      MFWD.fwrfc()[10],
    ",
  0x403c1cb0u64 => "
      MFWD.fwrfc()[11],
    ",
  0x403c1cc0u64 => "
      MFWD.fwrfc()[12],
    ",
  0x403c1cd0u64 => "
      MFWD.fwrfc()[13],
    ",
  0x403c1ce0u64 => "
      MFWD.fwrfc()[14],
    ",
  0x403c1cf0u64 => "
      MFWD.fwrfc()[15],
    ",
  0x403c1c04u64 => "
      MFWD.fwrfvc()[0],
    ",
  0x403c1c14u64 => "
      MFWD.fwrfvc()[1],
    ",
  0x403c1c24u64 => "
      MFWD.fwrfvc()[2],
    ",
  0x403c1c34u64 => "
      MFWD.fwrfvc()[3],
    ",
  0x403c1c44u64 => "
      MFWD.fwrfvc()[4],
    ",
  0x403c1c54u64 => "
      MFWD.fwrfvc()[5],
    ",
  0x403c1c64u64 => "
      MFWD.fwrfvc()[6],
    ",
  0x403c1c74u64 => "
      MFWD.fwrfvc()[7],
    ",
  0x403c1c84u64 => "
      MFWD.fwrfvc()[8],
    ",
  0x403c1c94u64 => "
      MFWD.fwrfvc()[9],
    ",
  0x403c1ca4u64 => "
      MFWD.fwrfvc()[10],
    ",
  0x403c1cb4u64 => "
      MFWD.fwrfvc()[11],
    ",
  0x403c1cc4u64 => "
      MFWD.fwrfvc()[12],
    ",
  0x403c1cd4u64 => "
      MFWD.fwrfvc()[13],
    ",
  0x403c1ce4u64 => "
      MFWD.fwrfvc()[14],
    ",
  0x403c1cf4u64 => "
      MFWD.fwrfvc()[15],
    ",
  0x403c2000u64 => "
      MFWD.fwcfc()[0],
    ",
  0x403c2040u64 => "
      MFWD.fwcfc()[1],
    ",
  0x403c2080u64 => "
      MFWD.fwcfc()[2],
    ",
  0x403c20c0u64 => "
      MFWD.fwcfc()[3],
    ",
  0x403c2100u64 => "
      MFWD.fwcfc()[4],
    ",
  0x403c2140u64 => "
      MFWD.fwcfc()[5],
    ",
  0x403c2180u64 => "
      MFWD.fwcfc()[6],
    ",
  0x403c21c0u64 => "
      MFWD.fwcfc()[7],
    ",
  0x403c2200u64 => "
      MFWD.fwcfc()[8],
    ",
  0x403c2240u64 => "
      MFWD.fwcfc()[9],
    ",
  0x403c2280u64 => "
      MFWD.fwcfc()[10],
    ",
  0x403c22c0u64 => "
      MFWD.fwcfc()[11],
    ",
  0x403c2300u64 => "
      MFWD.fwcfc()[12],
    ",
  0x403c2340u64 => "
      MFWD.fwcfc()[13],
    ",
  0x403c2380u64 => "
      MFWD.fwcfc()[14],
    ",
  0x403c23c0u64 => "
      MFWD.fwcfc()[15],
    ",
  0x403c2004u64 => "
      MFWD.fwcfmc0()[0],
    ",
  0x403c2044u64 => "
      MFWD.fwcfmc0()[1],
    ",
  0x403c2084u64 => "
      MFWD.fwcfmc0()[2],
    ",
  0x403c20c4u64 => "
      MFWD.fwcfmc0()[3],
    ",
  0x403c2104u64 => "
      MFWD.fwcfmc0()[4],
    ",
  0x403c2144u64 => "
      MFWD.fwcfmc0()[5],
    ",
  0x403c2184u64 => "
      MFWD.fwcfmc0()[6],
    ",
  0x403c21c4u64 => "
      MFWD.fwcfmc0()[7],
    ",
  0x403c2204u64 => "
      MFWD.fwcfmc0()[8],
    ",
  0x403c2244u64 => "
      MFWD.fwcfmc0()[9],
    ",
  0x403c2284u64 => "
      MFWD.fwcfmc0()[10],
    ",
  0x403c22c4u64 => "
      MFWD.fwcfmc0()[11],
    ",
  0x403c2304u64 => "
      MFWD.fwcfmc0()[12],
    ",
  0x403c2344u64 => "
      MFWD.fwcfmc0()[13],
    ",
  0x403c2384u64 => "
      MFWD.fwcfmc0()[14],
    ",
  0x403c23c4u64 => "
      MFWD.fwcfmc0()[15],
    ",
  0x403c2008u64 => "
      MFWD.fwcfmc1()[0],
    ",
  0x403c2048u64 => "
      MFWD.fwcfmc1()[1],
    ",
  0x403c2088u64 => "
      MFWD.fwcfmc1()[2],
    ",
  0x403c20c8u64 => "
      MFWD.fwcfmc1()[3],
    ",
  0x403c2108u64 => "
      MFWD.fwcfmc1()[4],
    ",
  0x403c2148u64 => "
      MFWD.fwcfmc1()[5],
    ",
  0x403c2188u64 => "
      MFWD.fwcfmc1()[6],
    ",
  0x403c21c8u64 => "
      MFWD.fwcfmc1()[7],
    ",
  0x403c2208u64 => "
      MFWD.fwcfmc1()[8],
    ",
  0x403c2248u64 => "
      MFWD.fwcfmc1()[9],
    ",
  0x403c2288u64 => "
      MFWD.fwcfmc1()[10],
    ",
  0x403c22c8u64 => "
      MFWD.fwcfmc1()[11],
    ",
  0x403c2308u64 => "
      MFWD.fwcfmc1()[12],
    ",
  0x403c2348u64 => "
      MFWD.fwcfmc1()[13],
    ",
  0x403c2388u64 => "
      MFWD.fwcfmc1()[14],
    ",
  0x403c23c8u64 => "
      MFWD.fwcfmc1()[15],
    ",
  0x403c200cu64 => "
      MFWD.fwcfmc2()[0],
    ",
  0x403c204cu64 => "
      MFWD.fwcfmc2()[1],
    ",
  0x403c208cu64 => "
      MFWD.fwcfmc2()[2],
    ",
  0x403c20ccu64 => "
      MFWD.fwcfmc2()[3],
    ",
  0x403c210cu64 => "
      MFWD.fwcfmc2()[4],
    ",
  0x403c214cu64 => "
      MFWD.fwcfmc2()[5],
    ",
  0x403c218cu64 => "
      MFWD.fwcfmc2()[6],
    ",
  0x403c21ccu64 => "
      MFWD.fwcfmc2()[7],
    ",
  0x403c220cu64 => "
      MFWD.fwcfmc2()[8],
    ",
  0x403c224cu64 => "
      MFWD.fwcfmc2()[9],
    ",
  0x403c228cu64 => "
      MFWD.fwcfmc2()[10],
    ",
  0x403c22ccu64 => "
      MFWD.fwcfmc2()[11],
    ",
  0x403c230cu64 => "
      MFWD.fwcfmc2()[12],
    ",
  0x403c234cu64 => "
      MFWD.fwcfmc2()[13],
    ",
  0x403c238cu64 => "
      MFWD.fwcfmc2()[14],
    ",
  0x403c23ccu64 => "
      MFWD.fwcfmc2()[15],
    ",
  0x403c2010u64 => "
      MFWD.fwcfmc3()[0],
    ",
  0x403c2050u64 => "
      MFWD.fwcfmc3()[1],
    ",
  0x403c2090u64 => "
      MFWD.fwcfmc3()[2],
    ",
  0x403c20d0u64 => "
      MFWD.fwcfmc3()[3],
    ",
  0x403c2110u64 => "
      MFWD.fwcfmc3()[4],
    ",
  0x403c2150u64 => "
      MFWD.fwcfmc3()[5],
    ",
  0x403c2190u64 => "
      MFWD.fwcfmc3()[6],
    ",
  0x403c21d0u64 => "
      MFWD.fwcfmc3()[7],
    ",
  0x403c2210u64 => "
      MFWD.fwcfmc3()[8],
    ",
  0x403c2250u64 => "
      MFWD.fwcfmc3()[9],
    ",
  0x403c2290u64 => "
      MFWD.fwcfmc3()[10],
    ",
  0x403c22d0u64 => "
      MFWD.fwcfmc3()[11],
    ",
  0x403c2310u64 => "
      MFWD.fwcfmc3()[12],
    ",
  0x403c2350u64 => "
      MFWD.fwcfmc3()[13],
    ",
  0x403c2390u64 => "
      MFWD.fwcfmc3()[14],
    ",
  0x403c23d0u64 => "
      MFWD.fwcfmc3()[15],
    ",
  0x403c2014u64 => "
      MFWD.fwcfmc4()[0],
    ",
  0x403c2054u64 => "
      MFWD.fwcfmc4()[1],
    ",
  0x403c2094u64 => "
      MFWD.fwcfmc4()[2],
    ",
  0x403c20d4u64 => "
      MFWD.fwcfmc4()[3],
    ",
  0x403c2114u64 => "
      MFWD.fwcfmc4()[4],
    ",
  0x403c2154u64 => "
      MFWD.fwcfmc4()[5],
    ",
  0x403c2194u64 => "
      MFWD.fwcfmc4()[6],
    ",
  0x403c21d4u64 => "
      MFWD.fwcfmc4()[7],
    ",
  0x403c2214u64 => "
      MFWD.fwcfmc4()[8],
    ",
  0x403c2254u64 => "
      MFWD.fwcfmc4()[9],
    ",
  0x403c2294u64 => "
      MFWD.fwcfmc4()[10],
    ",
  0x403c22d4u64 => "
      MFWD.fwcfmc4()[11],
    ",
  0x403c2314u64 => "
      MFWD.fwcfmc4()[12],
    ",
  0x403c2354u64 => "
      MFWD.fwcfmc4()[13],
    ",
  0x403c2394u64 => "
      MFWD.fwcfmc4()[14],
    ",
  0x403c23d4u64 => "
      MFWD.fwcfmc4()[15],
    ",
  0x403c2018u64 => "
      MFWD.fwcfmc5()[0],
    ",
  0x403c2058u64 => "
      MFWD.fwcfmc5()[1],
    ",
  0x403c2098u64 => "
      MFWD.fwcfmc5()[2],
    ",
  0x403c20d8u64 => "
      MFWD.fwcfmc5()[3],
    ",
  0x403c2118u64 => "
      MFWD.fwcfmc5()[4],
    ",
  0x403c2158u64 => "
      MFWD.fwcfmc5()[5],
    ",
  0x403c2198u64 => "
      MFWD.fwcfmc5()[6],
    ",
  0x403c21d8u64 => "
      MFWD.fwcfmc5()[7],
    ",
  0x403c2218u64 => "
      MFWD.fwcfmc5()[8],
    ",
  0x403c2258u64 => "
      MFWD.fwcfmc5()[9],
    ",
  0x403c2298u64 => "
      MFWD.fwcfmc5()[10],
    ",
  0x403c22d8u64 => "
      MFWD.fwcfmc5()[11],
    ",
  0x403c2318u64 => "
      MFWD.fwcfmc5()[12],
    ",
  0x403c2358u64 => "
      MFWD.fwcfmc5()[13],
    ",
  0x403c2398u64 => "
      MFWD.fwcfmc5()[14],
    ",
  0x403c23d8u64 => "
      MFWD.fwcfmc5()[15],
    ",
  0x403c201cu64 => "
      MFWD.fwcfmc6()[0],
    ",
  0x403c205cu64 => "
      MFWD.fwcfmc6()[1],
    ",
  0x403c209cu64 => "
      MFWD.fwcfmc6()[2],
    ",
  0x403c20dcu64 => "
      MFWD.fwcfmc6()[3],
    ",
  0x403c211cu64 => "
      MFWD.fwcfmc6()[4],
    ",
  0x403c215cu64 => "
      MFWD.fwcfmc6()[5],
    ",
  0x403c219cu64 => "
      MFWD.fwcfmc6()[6],
    ",
  0x403c21dcu64 => "
      MFWD.fwcfmc6()[7],
    ",
  0x403c221cu64 => "
      MFWD.fwcfmc6()[8],
    ",
  0x403c225cu64 => "
      MFWD.fwcfmc6()[9],
    ",
  0x403c229cu64 => "
      MFWD.fwcfmc6()[10],
    ",
  0x403c22dcu64 => "
      MFWD.fwcfmc6()[11],
    ",
  0x403c231cu64 => "
      MFWD.fwcfmc6()[12],
    ",
  0x403c235cu64 => "
      MFWD.fwcfmc6()[13],
    ",
  0x403c239cu64 => "
      MFWD.fwcfmc6()[14],
    ",
  0x403c23dcu64 => "
      MFWD.fwcfmc6()[15],
    ",
  0x403c4008u64 => "
      MFWD.fwip4sc(),
    ",
  0x403c4018u64 => "
      MFWD.fwip6sc(),
    ",
  0x403c401cu64 => "
      MFWD.fwip6oc(),
    ",
  0x403c4020u64 => "
      MFWD.fwl2sc(),
    ",
  0x403c4030u64 => "
      MFWD.fwsfhec(),
    ",
  0x403c4040u64 => "
      MFWD.fwshcr0(),
    ",
  0x403c4044u64 => "
      MFWD.fwshcr1(),
    ",
  0x403c4048u64 => "
      MFWD.fwshcr2(),
    ",
  0x403c404cu64 => "
      MFWD.fwshcr3(),
    ",
  0x403c4050u64 => "
      MFWD.fwshcr4(),
    ",
  0x403c4054u64 => "
      MFWD.fwshcr5(),
    ",
  0x403c4058u64 => "
      MFWD.fwshcr6(),
    ",
  0x403c405cu64 => "
      MFWD.fwshcr7(),
    ",
  0x403c4060u64 => "
      MFWD.fwshcr8(),
    ",
  0x403c4064u64 => "
      MFWD.fwshcr9(),
    ",
  0x403c4068u64 => "
      MFWD.fwshcr10(),
    ",
  0x403c406cu64 => "
      MFWD.fwshcr11(),
    ",
  0x403c4070u64 => "
      MFWD.fwshcr12(),
    ",
  0x403c4074u64 => "
      MFWD.fwshcr13(),
    ",
  0x403c4078u64 => "
      MFWD.fwshcrr(),
    ",
  0x403c4090u64 => "
      MFWD.fwlthhec(),
    ",
  0x403c4094u64 => "
      MFWD.fwlthhc(),
    ",
  0x403c40a0u64 => "
      MFWD.fwlthtl0(),
    ",
  0x403c40a4u64 => "
      MFWD.fwlthtl1(),
    ",
  0x403c40a8u64 => "
      MFWD.fwlthtl2(),
    ",
  0x403c40acu64 => "
      MFWD.fwlthtl3(),
    ",
  0x403c40b0u64 => "
      MFWD.fwlthtl4(),
    ",
  0x403c40b4u64 => "
      MFWD.fwlthtl5(),
    ",
  0x403c40b8u64 => "
      MFWD.fwlthtl6(),
    ",
  0x403c40bcu64 => "
      MFWD.fwlthtl7(),
    ",
  0x403c40c0u64 => "
      MFWD.fwlthtl80(),
    ",
  0x403c40d0u64 => "
      MFWD.fwlthtl9(),
    ",
  0x403c40d4u64 => "
      MFWD.fwlthtlr(),
    ",
  0x403c40e0u64 => "
      MFWD.fwlthtim(),
    ",
  0x403c40e4u64 => "
      MFWD.fwlthtem(),
    ",
  0x403c4100u64 => "
      MFWD.fwlthts0(),
    ",
  0x403c4104u64 => "
      MFWD.fwlthts1(),
    ",
  0x403c4108u64 => "
      MFWD.fwlthts2(),
    ",
  0x403c410cu64 => "
      MFWD.fwlthts3(),
    ",
  0x403c4110u64 => "
      MFWD.fwlthts4(),
    ",
  0x403c4120u64 => "
      MFWD.fwlthtsr0(),
    ",
  0x403c4124u64 => "
      MFWD.fwlthtsr1(),
    ",
  0x403c4128u64 => "
      MFWD.fwlthtsr2(),
    ",
  0x403c412cu64 => "
      MFWD.fwlthtsr3(),
    ",
  0x403c4130u64 => "
      MFWD.fwlthtsr40(),
    ",
  0x403c4140u64 => "
      MFWD.fwlthtsr5(),
    ",
  0x403c4150u64 => "
      MFWD.fwlthtr(),
    ",
  0x403c4154u64 => "
      MFWD.fwlthtrr0(),
    ",
  0x403c4158u64 => "
      MFWD.fwlthtrr1(),
    ",
  0x403c415cu64 => "
      MFWD.fwlthtrr2(),
    ",
  0x403c4160u64 => "
      MFWD.fwlthtrr3(),
    ",
  0x403c4164u64 => "
      MFWD.fwlthtrr4(),
    ",
  0x403c4168u64 => "
      MFWD.fwlthtrr5(),
    ",
  0x403c416cu64 => "
      MFWD.fwlthtrr6(),
    ",
  0x403c4170u64 => "
      MFWD.fwlthtrr7(),
    ",
  0x403c4174u64 => "
      MFWD.fwlthtrr8(),
    ",
  0x403c4180u64 => "
      MFWD.fwlthtrr90(),
    ",
  0x403c4190u64 => "
      MFWD.fwlthtrr10(),
    ",
  0x403c4620u64 => "
      MFWD.fwmachec(),
    ",
  0x403c4624u64 => "
      MFWD.fwmachc(),
    ",
  0x403c4630u64 => "
      MFWD.fwmactl0(),
    ",
  0x403c4634u64 => "
      MFWD.fwmactl1(),
    ",
  0x403c4638u64 => "
      MFWD.fwmactl2(),
    ",
  0x403c463cu64 => "
      MFWD.fwmactl3(),
    ",
  0x403c4640u64 => "
      MFWD.fwmactl40(),
    ",
  0x403c4650u64 => "
      MFWD.fwmactl5(),
    ",
  0x403c4654u64 => "
      MFWD.fwmactlr(),
    ",
  0x403c4660u64 => "
      MFWD.fwmactim(),
    ",
  0x403c4664u64 => "
      MFWD.fwmactem(),
    ",
  0x403c4670u64 => "
      MFWD.fwmacts0(),
    ",
  0x403c4674u64 => "
      MFWD.fwmacts1(),
    ",
  0x403c4678u64 => "
      MFWD.fwmactsr0(),
    ",
  0x403c467cu64 => "
      MFWD.fwmactsr1(),
    ",
  0x403c4680u64 => "
      MFWD.fwmactsr20(),
    ",
  0x403c4690u64 => "
      MFWD.fwmactsr3(),
    ",
  0x403c46a0u64 => "
      MFWD.fwmactr(),
    ",
  0x403c46a4u64 => "
      MFWD.fwmactrr0(),
    ",
  0x403c46a8u64 => "
      MFWD.fwmactrr1(),
    ",
  0x403c46acu64 => "
      MFWD.fwmactrr2(),
    ",
  0x403c46b0u64 => "
      MFWD.fwmactrr3(),
    ",
  0x403c46b4u64 => "
      MFWD.fwmactrr4(),
    ",
  0x403c46c0u64 => "
      MFWD.fwmactrr50(),
    ",
  0x403c46d0u64 => "
      MFWD.fwmactrr6(),
    ",
  0x403c4880u64 => "
      MFWD.fwmacaguspc(),
    ",
  0x403c4884u64 => "
      MFWD.fwmacagc(),
    ",
  0x403c4888u64 => "
      MFWD.fwmacagm0(),
    ",
  0x403c488cu64 => "
      MFWD.fwmacagm1(),
    ",
  0x403c4900u64 => "
      MFWD.fwvlantec(),
    ",
  0x403c4910u64 => "
      MFWD.fwvlantl0(),
    ",
  0x403c4914u64 => "
      MFWD.fwvlantl1(),
    ",
  0x403c4918u64 => "
      MFWD.fwvlantl2(),
    ",
  0x403c4920u64 => "
      MFWD.fwvlantl30(),
    ",
  0x403c4930u64 => "
      MFWD.fwvlantl4(),
    ",
  0x403c4934u64 => "
      MFWD.fwvlantlr(),
    ",
  0x403c4940u64 => "
      MFWD.fwvlantim(),
    ",
  0x403c4944u64 => "
      MFWD.fwvlantem(),
    ",
  0x403c4950u64 => "
      MFWD.fwvlants(),
    ",
  0x403c4954u64 => "
      MFWD.fwvlantsr0(),
    ",
  0x403c4958u64 => "
      MFWD.fwvlantsr1(),
    ",
  0x403c4960u64 => "
      MFWD.fwvlantsr20(),
    ",
  0x403c4970u64 => "
      MFWD.fwvlantsr3(),
    ",
  0x403c4a00u64 => "
      MFWD.fwpbfc()[0],
    ",
  0x403c4a10u64 => "
      MFWD.fwpbfc()[1],
    ",
  0x403c4a20u64 => "
      MFWD.fwpbfc()[2],
    ",
  0x403c4a04u64 => "
      MFWD.fwpbfcsdc0()[0],
    ",
  0x403c4a14u64 => "
      MFWD.fwpbfcsdc0()[1],
    ",
  0x403c4a24u64 => "
      MFWD.fwpbfcsdc0()[2],
    ",
  0x403c4e00u64 => "
      MFWD.fwl23url0(),
    ",
  0x403c4e04u64 => "
      MFWD.fwl23url1(),
    ",
  0x403c4e08u64 => "
      MFWD.fwl23url2(),
    ",
  0x403c4e0cu64 => "
      MFWD.fwl23url3(),
    ",
  0x403c4e10u64 => "
      MFWD.fwl23urlr(),
    ",
  0x403c4e20u64 => "
      MFWD.fwl23utim(),
    ",
  0x403c4e30u64 => "
      MFWD.fwl23urr(),
    ",
  0x403c4e34u64 => "
      MFWD.fwl23urrr0(),
    ",
  0x403c4e38u64 => "
      MFWD.fwl23urrr1(),
    ",
  0x403c4e3cu64 => "
      MFWD.fwl23urrr2(),
    ",
  0x403c4e40u64 => "
      MFWD.fwl23urrr3(),
    ",
  0x403c4f00u64 => "
      MFWD.fwl23urmc()[0],
    ",
  0x403c4f04u64 => "
      MFWD.fwl23urmc()[1],
    ",
  0x403c4f08u64 => "
      MFWD.fwl23urmc()[2],
    ",
  0x403c4f0cu64 => "
      MFWD.fwl23urmc()[3],
    ",
  0x403c4f10u64 => "
      MFWD.fwl23urmc()[4],
    ",
  0x403c4f14u64 => "
      MFWD.fwl23urmc()[5],
    ",
  0x403c4f18u64 => "
      MFWD.fwl23urmc()[6],
    ",
  0x403c4f1cu64 => "
      MFWD.fwl23urmc()[7],
    ",
  0x403c4f20u64 => "
      MFWD.fwl23urmc()[8],
    ",
  0x403c4f24u64 => "
      MFWD.fwl23urmc()[9],
    ",
  0x403c4f28u64 => "
      MFWD.fwl23urmc()[10],
    ",
  0x403c4f2cu64 => "
      MFWD.fwl23urmc()[11],
    ",
  0x403c4f30u64 => "
      MFWD.fwl23urmc()[12],
    ",
  0x403c4f34u64 => "
      MFWD.fwl23urmc()[13],
    ",
  0x403c4f38u64 => "
      MFWD.fwl23urmc()[14],
    ",
  0x403c4f3cu64 => "
      MFWD.fwl23urmc()[15],
    ",
  0x403c4f40u64 => "
      MFWD.fwl23urmc()[16],
    ",
  0x403c4f44u64 => "
      MFWD.fwl23urmc()[17],
    ",
  0x403c4f48u64 => "
      MFWD.fwl23urmc()[18],
    ",
  0x403c4f4cu64 => "
      MFWD.fwl23urmc()[19],
    ",
  0x403c4f50u64 => "
      MFWD.fwl23urmc()[20],
    ",
  0x403c4f54u64 => "
      MFWD.fwl23urmc()[21],
    ",
  0x403c4f58u64 => "
      MFWD.fwl23urmc()[22],
    ",
  0x403c4f5cu64 => "
      MFWD.fwl23urmc()[23],
    ",
  0x403c4f60u64 => "
      MFWD.fwl23urmc()[24],
    ",
  0x403c4f64u64 => "
      MFWD.fwl23urmc()[25],
    ",
  0x403c4f68u64 => "
      MFWD.fwl23urmc()[26],
    ",
  0x403c4f6cu64 => "
      MFWD.fwl23urmc()[27],
    ",
  0x403c4f70u64 => "
      MFWD.fwl23urmc()[28],
    ",
  0x403c4f74u64 => "
      MFWD.fwl23urmc()[29],
    ",
  0x403c4f78u64 => "
      MFWD.fwl23urmc()[30],
    ",
  0x403c4f7cu64 => "
      MFWD.fwl23urmc()[31],
    ",
  0x403c5000u64 => "
      MFWD.fwpmfgc()[0],
    ",
  0x403c5004u64 => "
      MFWD.fwpmfgc()[1],
    ",
  0x403c5008u64 => "
      MFWD.fwpmfgc()[2],
    ",
  0x403c500cu64 => "
      MFWD.fwpmfgc()[3],
    ",
  0x403c5010u64 => "
      MFWD.fwpmfgc()[4],
    ",
  0x403c5014u64 => "
      MFWD.fwpmfgc()[5],
    ",
  0x403c5018u64 => "
      MFWD.fwpmfgc()[6],
    ",
  0x403c501cu64 => "
      MFWD.fwpmfgc()[7],
    ",
  0x403c5020u64 => "
      MFWD.fwpmfgc()[8],
    ",
  0x403c5024u64 => "
      MFWD.fwpmfgc()[9],
    ",
  0x403c5028u64 => "
      MFWD.fwpmfgc()[10],
    ",
  0x403c502cu64 => "
      MFWD.fwpmfgc()[11],
    ",
  0x403c5030u64 => "
      MFWD.fwpmfgc()[12],
    ",
  0x403c5034u64 => "
      MFWD.fwpmfgc()[13],
    ",
  0x403c5038u64 => "
      MFWD.fwpmfgc()[14],
    ",
  0x403c503cu64 => "
      MFWD.fwpmfgc()[15],
    ",
  0x403c5600u64 => "
      MFWD.fwpmtrfc()[0],
    ",
  0x403c5620u64 => "
      MFWD.fwpmtrfc()[1],
    ",
  0x403c5640u64 => "
      MFWD.fwpmtrfc()[2],
    ",
  0x403c5660u64 => "
      MFWD.fwpmtrfc()[3],
    ",
  0x403c5680u64 => "
      MFWD.fwpmtrfc()[4],
    ",
  0x403c56a0u64 => "
      MFWD.fwpmtrfc()[5],
    ",
  0x403c56c0u64 => "
      MFWD.fwpmtrfc()[6],
    ",
  0x403c56e0u64 => "
      MFWD.fwpmtrfc()[7],
    ",
  0x403c5700u64 => "
      MFWD.fwpmtrfc()[8],
    ",
  0x403c5720u64 => "
      MFWD.fwpmtrfc()[9],
    ",
  0x403c5740u64 => "
      MFWD.fwpmtrfc()[10],
    ",
  0x403c5760u64 => "
      MFWD.fwpmtrfc()[11],
    ",
  0x403c5780u64 => "
      MFWD.fwpmtrfc()[12],
    ",
  0x403c57a0u64 => "
      MFWD.fwpmtrfc()[13],
    ",
  0x403c57c0u64 => "
      MFWD.fwpmtrfc()[14],
    ",
  0x403c57e0u64 => "
      MFWD.fwpmtrfc()[15],
    ",
  0x403c5800u64 => "
      MFWD.fwpmtrfc()[16],
    ",
  0x403c5820u64 => "
      MFWD.fwpmtrfc()[17],
    ",
  0x403c5840u64 => "
      MFWD.fwpmtrfc()[18],
    ",
  0x403c5860u64 => "
      MFWD.fwpmtrfc()[19],
    ",
  0x403c5880u64 => "
      MFWD.fwpmtrfc()[20],
    ",
  0x403c58a0u64 => "
      MFWD.fwpmtrfc()[21],
    ",
  0x403c58c0u64 => "
      MFWD.fwpmtrfc()[22],
    ",
  0x403c58e0u64 => "
      MFWD.fwpmtrfc()[23],
    ",
  0x403c5900u64 => "
      MFWD.fwpmtrfc()[24],
    ",
  0x403c5920u64 => "
      MFWD.fwpmtrfc()[25],
    ",
  0x403c5940u64 => "
      MFWD.fwpmtrfc()[26],
    ",
  0x403c5960u64 => "
      MFWD.fwpmtrfc()[27],
    ",
  0x403c5980u64 => "
      MFWD.fwpmtrfc()[28],
    ",
  0x403c59a0u64 => "
      MFWD.fwpmtrfc()[29],
    ",
  0x403c59c0u64 => "
      MFWD.fwpmtrfc()[30],
    ",
  0x403c59e0u64 => "
      MFWD.fwpmtrfc()[31],
    ",
  0x403c5604u64 => "
      MFWD.fwpmtrcbsc()[0],
    ",
  0x403c5624u64 => "
      MFWD.fwpmtrcbsc()[1],
    ",
  0x403c5644u64 => "
      MFWD.fwpmtrcbsc()[2],
    ",
  0x403c5664u64 => "
      MFWD.fwpmtrcbsc()[3],
    ",
  0x403c5684u64 => "
      MFWD.fwpmtrcbsc()[4],
    ",
  0x403c56a4u64 => "
      MFWD.fwpmtrcbsc()[5],
    ",
  0x403c56c4u64 => "
      MFWD.fwpmtrcbsc()[6],
    ",
  0x403c56e4u64 => "
      MFWD.fwpmtrcbsc()[7],
    ",
  0x403c5704u64 => "
      MFWD.fwpmtrcbsc()[8],
    ",
  0x403c5724u64 => "
      MFWD.fwpmtrcbsc()[9],
    ",
  0x403c5744u64 => "
      MFWD.fwpmtrcbsc()[10],
    ",
  0x403c5764u64 => "
      MFWD.fwpmtrcbsc()[11],
    ",
  0x403c5784u64 => "
      MFWD.fwpmtrcbsc()[12],
    ",
  0x403c57a4u64 => "
      MFWD.fwpmtrcbsc()[13],
    ",
  0x403c57c4u64 => "
      MFWD.fwpmtrcbsc()[14],
    ",
  0x403c57e4u64 => "
      MFWD.fwpmtrcbsc()[15],
    ",
  0x403c5804u64 => "
      MFWD.fwpmtrcbsc()[16],
    ",
  0x403c5824u64 => "
      MFWD.fwpmtrcbsc()[17],
    ",
  0x403c5844u64 => "
      MFWD.fwpmtrcbsc()[18],
    ",
  0x403c5864u64 => "
      MFWD.fwpmtrcbsc()[19],
    ",
  0x403c5884u64 => "
      MFWD.fwpmtrcbsc()[20],
    ",
  0x403c58a4u64 => "
      MFWD.fwpmtrcbsc()[21],
    ",
  0x403c58c4u64 => "
      MFWD.fwpmtrcbsc()[22],
    ",
  0x403c58e4u64 => "
      MFWD.fwpmtrcbsc()[23],
    ",
  0x403c5904u64 => "
      MFWD.fwpmtrcbsc()[24],
    ",
  0x403c5924u64 => "
      MFWD.fwpmtrcbsc()[25],
    ",
  0x403c5944u64 => "
      MFWD.fwpmtrcbsc()[26],
    ",
  0x403c5964u64 => "
      MFWD.fwpmtrcbsc()[27],
    ",
  0x403c5984u64 => "
      MFWD.fwpmtrcbsc()[28],
    ",
  0x403c59a4u64 => "
      MFWD.fwpmtrcbsc()[29],
    ",
  0x403c59c4u64 => "
      MFWD.fwpmtrcbsc()[30],
    ",
  0x403c59e4u64 => "
      MFWD.fwpmtrcbsc()[31],
    ",
  0x403c5608u64 => "
      MFWD.fwpmtrcirc()[0],
    ",
  0x403c5628u64 => "
      MFWD.fwpmtrcirc()[1],
    ",
  0x403c5648u64 => "
      MFWD.fwpmtrcirc()[2],
    ",
  0x403c5668u64 => "
      MFWD.fwpmtrcirc()[3],
    ",
  0x403c5688u64 => "
      MFWD.fwpmtrcirc()[4],
    ",
  0x403c56a8u64 => "
      MFWD.fwpmtrcirc()[5],
    ",
  0x403c56c8u64 => "
      MFWD.fwpmtrcirc()[6],
    ",
  0x403c56e8u64 => "
      MFWD.fwpmtrcirc()[7],
    ",
  0x403c5708u64 => "
      MFWD.fwpmtrcirc()[8],
    ",
  0x403c5728u64 => "
      MFWD.fwpmtrcirc()[9],
    ",
  0x403c5748u64 => "
      MFWD.fwpmtrcirc()[10],
    ",
  0x403c5768u64 => "
      MFWD.fwpmtrcirc()[11],
    ",
  0x403c5788u64 => "
      MFWD.fwpmtrcirc()[12],
    ",
  0x403c57a8u64 => "
      MFWD.fwpmtrcirc()[13],
    ",
  0x403c57c8u64 => "
      MFWD.fwpmtrcirc()[14],
    ",
  0x403c57e8u64 => "
      MFWD.fwpmtrcirc()[15],
    ",
  0x403c5808u64 => "
      MFWD.fwpmtrcirc()[16],
    ",
  0x403c5828u64 => "
      MFWD.fwpmtrcirc()[17],
    ",
  0x403c5848u64 => "
      MFWD.fwpmtrcirc()[18],
    ",
  0x403c5868u64 => "
      MFWD.fwpmtrcirc()[19],
    ",
  0x403c5888u64 => "
      MFWD.fwpmtrcirc()[20],
    ",
  0x403c58a8u64 => "
      MFWD.fwpmtrcirc()[21],
    ",
  0x403c58c8u64 => "
      MFWD.fwpmtrcirc()[22],
    ",
  0x403c58e8u64 => "
      MFWD.fwpmtrcirc()[23],
    ",
  0x403c5908u64 => "
      MFWD.fwpmtrcirc()[24],
    ",
  0x403c5928u64 => "
      MFWD.fwpmtrcirc()[25],
    ",
  0x403c5948u64 => "
      MFWD.fwpmtrcirc()[26],
    ",
  0x403c5968u64 => "
      MFWD.fwpmtrcirc()[27],
    ",
  0x403c5988u64 => "
      MFWD.fwpmtrcirc()[28],
    ",
  0x403c59a8u64 => "
      MFWD.fwpmtrcirc()[29],
    ",
  0x403c59c8u64 => "
      MFWD.fwpmtrcirc()[30],
    ",
  0x403c59e8u64 => "
      MFWD.fwpmtrcirc()[31],
    ",
  0x403c560cu64 => "
      MFWD.fwpmtrebsc()[0],
    ",
  0x403c562cu64 => "
      MFWD.fwpmtrebsc()[1],
    ",
  0x403c564cu64 => "
      MFWD.fwpmtrebsc()[2],
    ",
  0x403c566cu64 => "
      MFWD.fwpmtrebsc()[3],
    ",
  0x403c568cu64 => "
      MFWD.fwpmtrebsc()[4],
    ",
  0x403c56acu64 => "
      MFWD.fwpmtrebsc()[5],
    ",
  0x403c56ccu64 => "
      MFWD.fwpmtrebsc()[6],
    ",
  0x403c56ecu64 => "
      MFWD.fwpmtrebsc()[7],
    ",
  0x403c5610u64 => "
      MFWD.fwpmtreirc()[0],
    ",
  0x403c5630u64 => "
      MFWD.fwpmtreirc()[1],
    ",
  0x403c5650u64 => "
      MFWD.fwpmtreirc()[2],
    ",
  0x403c5670u64 => "
      MFWD.fwpmtreirc()[3],
    ",
  0x403c5690u64 => "
      MFWD.fwpmtreirc()[4],
    ",
  0x403c56b0u64 => "
      MFWD.fwpmtreirc()[5],
    ",
  0x403c56d0u64 => "
      MFWD.fwpmtreirc()[6],
    ",
  0x403c56f0u64 => "
      MFWD.fwpmtreirc()[7],
    ",
  0x403c5614u64 => "
      MFWD.fwpmtrfm()[0],
    ",
  0x403c5634u64 => "
      MFWD.fwpmtrfm()[1],
    ",
  0x403c5654u64 => "
      MFWD.fwpmtrfm()[2],
    ",
  0x403c5674u64 => "
      MFWD.fwpmtrfm()[3],
    ",
  0x403c5694u64 => "
      MFWD.fwpmtrfm()[4],
    ",
  0x403c56b4u64 => "
      MFWD.fwpmtrfm()[5],
    ",
  0x403c56d4u64 => "
      MFWD.fwpmtrfm()[6],
    ",
  0x403c56f4u64 => "
      MFWD.fwpmtrfm()[7],
    ",
  0x403c5714u64 => "
      MFWD.fwpmtrfm()[8],
    ",
  0x403c5734u64 => "
      MFWD.fwpmtrfm()[9],
    ",
  0x403c5754u64 => "
      MFWD.fwpmtrfm()[10],
    ",
  0x403c5774u64 => "
      MFWD.fwpmtrfm()[11],
    ",
  0x403c5794u64 => "
      MFWD.fwpmtrfm()[12],
    ",
  0x403c57b4u64 => "
      MFWD.fwpmtrfm()[13],
    ",
  0x403c57d4u64 => "
      MFWD.fwpmtrfm()[14],
    ",
  0x403c57f4u64 => "
      MFWD.fwpmtrfm()[15],
    ",
  0x403c5814u64 => "
      MFWD.fwpmtrfm()[16],
    ",
  0x403c5834u64 => "
      MFWD.fwpmtrfm()[17],
    ",
  0x403c5854u64 => "
      MFWD.fwpmtrfm()[18],
    ",
  0x403c5874u64 => "
      MFWD.fwpmtrfm()[19],
    ",
  0x403c5894u64 => "
      MFWD.fwpmtrfm()[20],
    ",
  0x403c58b4u64 => "
      MFWD.fwpmtrfm()[21],
    ",
  0x403c58d4u64 => "
      MFWD.fwpmtrfm()[22],
    ",
  0x403c58f4u64 => "
      MFWD.fwpmtrfm()[23],
    ",
  0x403c5914u64 => "
      MFWD.fwpmtrfm()[24],
    ",
  0x403c5934u64 => "
      MFWD.fwpmtrfm()[25],
    ",
  0x403c5954u64 => "
      MFWD.fwpmtrfm()[26],
    ",
  0x403c5974u64 => "
      MFWD.fwpmtrfm()[27],
    ",
  0x403c5994u64 => "
      MFWD.fwpmtrfm()[28],
    ",
  0x403c59b4u64 => "
      MFWD.fwpmtrfm()[29],
    ",
  0x403c59d4u64 => "
      MFWD.fwpmtrfm()[30],
    ",
  0x403c59f4u64 => "
      MFWD.fwpmtrfm()[31],
    ",
  0x403c6000u64 => "
      MFWD.fwftl0(),
    ",
  0x403c6004u64 => "
      MFWD.fwftl1(),
    ",
  0x403c6008u64 => "
      MFWD.fwftlr(),
    ",
  0x403c6010u64 => "
      MFWD.fwftoc(),
    ",
  0x403c6014u64 => "
      MFWD.fwftopc(),
    ",
  0x403c6020u64 => "
      MFWD.fwftim(),
    ",
  0x403c6030u64 => "
      MFWD.fwftr(),
    ",
  0x403c6034u64 => "
      MFWD.fwftrr0(),
    ",
  0x403c6038u64 => "
      MFWD.fwftrr1(),
    ",
  0x403c603cu64 => "
      MFWD.fwftrr2(),
    ",
  0x403c6100u64 => "
      MFWD.fwseqngc()[0],
    ",
  0x403c6108u64 => "
      MFWD.fwseqngc()[1],
    ",
  0x403c6110u64 => "
      MFWD.fwseqngc()[2],
    ",
  0x403c6118u64 => "
      MFWD.fwseqngc()[3],
    ",
  0x403c6120u64 => "
      MFWD.fwseqngc()[4],
    ",
  0x403c6128u64 => "
      MFWD.fwseqngc()[5],
    ",
  0x403c6130u64 => "
      MFWD.fwseqngc()[6],
    ",
  0x403c6138u64 => "
      MFWD.fwseqngc()[7],
    ",
  0x403c6140u64 => "
      MFWD.fwseqngc()[8],
    ",
  0x403c6148u64 => "
      MFWD.fwseqngc()[9],
    ",
  0x403c6150u64 => "
      MFWD.fwseqngc()[10],
    ",
  0x403c6158u64 => "
      MFWD.fwseqngc()[11],
    ",
  0x403c6160u64 => "
      MFWD.fwseqngc()[12],
    ",
  0x403c6168u64 => "
      MFWD.fwseqngc()[13],
    ",
  0x403c6170u64 => "
      MFWD.fwseqngc()[14],
    ",
  0x403c6178u64 => "
      MFWD.fwseqngc()[15],
    ",
  0x403c6180u64 => "
      MFWD.fwseqngc()[16],
    ",
  0x403c6188u64 => "
      MFWD.fwseqngc()[17],
    ",
  0x403c6190u64 => "
      MFWD.fwseqngc()[18],
    ",
  0x403c6198u64 => "
      MFWD.fwseqngc()[19],
    ",
  0x403c61a0u64 => "
      MFWD.fwseqngc()[20],
    ",
  0x403c61a8u64 => "
      MFWD.fwseqngc()[21],
    ",
  0x403c61b0u64 => "
      MFWD.fwseqngc()[22],
    ",
  0x403c61b8u64 => "
      MFWD.fwseqngc()[23],
    ",
  0x403c61c0u64 => "
      MFWD.fwseqngc()[24],
    ",
  0x403c61c8u64 => "
      MFWD.fwseqngc()[25],
    ",
  0x403c61d0u64 => "
      MFWD.fwseqngc()[26],
    ",
  0x403c61d8u64 => "
      MFWD.fwseqngc()[27],
    ",
  0x403c61e0u64 => "
      MFWD.fwseqngc()[28],
    ",
  0x403c61e8u64 => "
      MFWD.fwseqngc()[29],
    ",
  0x403c61f0u64 => "
      MFWD.fwseqngc()[30],
    ",
  0x403c61f8u64 => "
      MFWD.fwseqngc()[31],
    ",
  0x403c6104u64 => "
      MFWD.fwseqngm()[0],
    ",
  0x403c610cu64 => "
      MFWD.fwseqngm()[1],
    ",
  0x403c6114u64 => "
      MFWD.fwseqngm()[2],
    ",
  0x403c611cu64 => "
      MFWD.fwseqngm()[3],
    ",
  0x403c6124u64 => "
      MFWD.fwseqngm()[4],
    ",
  0x403c612cu64 => "
      MFWD.fwseqngm()[5],
    ",
  0x403c6134u64 => "
      MFWD.fwseqngm()[6],
    ",
  0x403c613cu64 => "
      MFWD.fwseqngm()[7],
    ",
  0x403c6144u64 => "
      MFWD.fwseqngm()[8],
    ",
  0x403c614cu64 => "
      MFWD.fwseqngm()[9],
    ",
  0x403c6154u64 => "
      MFWD.fwseqngm()[10],
    ",
  0x403c615cu64 => "
      MFWD.fwseqngm()[11],
    ",
  0x403c6164u64 => "
      MFWD.fwseqngm()[12],
    ",
  0x403c616cu64 => "
      MFWD.fwseqngm()[13],
    ",
  0x403c6174u64 => "
      MFWD.fwseqngm()[14],
    ",
  0x403c617cu64 => "
      MFWD.fwseqngm()[15],
    ",
  0x403c6184u64 => "
      MFWD.fwseqngm()[16],
    ",
  0x403c618cu64 => "
      MFWD.fwseqngm()[17],
    ",
  0x403c6194u64 => "
      MFWD.fwseqngm()[18],
    ",
  0x403c619cu64 => "
      MFWD.fwseqngm()[19],
    ",
  0x403c61a4u64 => "
      MFWD.fwseqngm()[20],
    ",
  0x403c61acu64 => "
      MFWD.fwseqngm()[21],
    ",
  0x403c61b4u64 => "
      MFWD.fwseqngm()[22],
    ",
  0x403c61bcu64 => "
      MFWD.fwseqngm()[23],
    ",
  0x403c61c4u64 => "
      MFWD.fwseqngm()[24],
    ",
  0x403c61ccu64 => "
      MFWD.fwseqngm()[25],
    ",
  0x403c61d4u64 => "
      MFWD.fwseqngm()[26],
    ",
  0x403c61dcu64 => "
      MFWD.fwseqngm()[27],
    ",
  0x403c61e4u64 => "
      MFWD.fwseqngm()[28],
    ",
  0x403c61ecu64 => "
      MFWD.fwseqngm()[29],
    ",
  0x403c61f4u64 => "
      MFWD.fwseqngm()[30],
    ",
  0x403c61fcu64 => "
      MFWD.fwseqngm()[31],
    ",
  0x403c6200u64 => "
      MFWD.fwseqnrc(),
    ",
  0x403c6300u64 => "
      MFWD.fwctfdcn()[0],
    ",
  0x403c6320u64 => "
      MFWD.fwctfdcn()[1],
    ",
  0x403c6304u64 => "
      MFWD.fwlthfdcn()[0],
    ",
  0x403c6324u64 => "
      MFWD.fwlthfdcn()[1],
    ",
  0x403c6344u64 => "
      MFWD.fwlthfdcn()[2],
    ",
  0x403c630cu64 => "
      MFWD.fwltwfdcn()[0],
    ",
  0x403c632cu64 => "
      MFWD.fwltwfdcn()[1],
    ",
  0x403c634cu64 => "
      MFWD.fwltwfdcn()[2],
    ",
  0x403c6310u64 => "
      MFWD.fwpbfdcn()[0],
    ",
  0x403c6330u64 => "
      MFWD.fwpbfdcn()[1],
    ",
  0x403c6350u64 => "
      MFWD.fwpbfdcn()[2],
    ",
  0x403c6314u64 => "
      MFWD.fwmhlcn()[0],
    ",
  0x403c6334u64 => "
      MFWD.fwmhlcn()[1],
    ",
  0x403c6354u64 => "
      MFWD.fwmhlcn()[2],
    ",
  0x403c6340u64 => "
      MFWD.fwddfdcn2(),
    ",
  0x403c6504u64 => "
      MFWD.fwwmrdcn()[0],
    ",
  0x403c6524u64 => "
      MFWD.fwwmrdcn()[1],
    ",
  0x403c6544u64 => "
      MFWD.fwwmrdcn()[2],
    ",
  0x403c6508u64 => "
      MFWD.fwctrdcn()[0],
    ",
  0x403c6528u64 => "
      MFWD.fwctrdcn()[1],
    ",
  0x403c650cu64 => "
      MFWD.fwlthrdcn()[0],
    ",
  0x403c652cu64 => "
      MFWD.fwlthrdcn()[1],
    ",
  0x403c654cu64 => "
      MFWD.fwlthrdcn()[2],
    ",
  0x403c6514u64 => "
      MFWD.fwltwrdcn()[0],
    ",
  0x403c6534u64 => "
      MFWD.fwltwrdcn()[1],
    ",
  0x403c6554u64 => "
      MFWD.fwltwrdcn()[2],
    ",
  0x403c6518u64 => "
      MFWD.fwpbrdcn()[0],
    ",
  0x403c6538u64 => "
      MFWD.fwpbrdcn()[1],
    ",
  0x403c6558u64 => "
      MFWD.fwpbrdcn()[2],
    ",
  0x403c6548u64 => "
      MFWD.fwddrdcn2(),
    ",
  0x403c6700u64 => "
      MFWD.fwpmfdcn()[0],
    ",
  0x403c6704u64 => "
      MFWD.fwpmfdcn()[1],
    ",
  0x403c6708u64 => "
      MFWD.fwpmfdcn()[2],
    ",
  0x403c670cu64 => "
      MFWD.fwpmfdcn()[3],
    ",
  0x403c6710u64 => "
      MFWD.fwpmfdcn()[4],
    ",
  0x403c6714u64 => "
      MFWD.fwpmfdcn()[5],
    ",
  0x403c6718u64 => "
      MFWD.fwpmfdcn()[6],
    ",
  0x403c671cu64 => "
      MFWD.fwpmfdcn()[7],
    ",
  0x403c6720u64 => "
      MFWD.fwpmfdcn()[8],
    ",
  0x403c6724u64 => "
      MFWD.fwpmfdcn()[9],
    ",
  0x403c6728u64 => "
      MFWD.fwpmfdcn()[10],
    ",
  0x403c672cu64 => "
      MFWD.fwpmfdcn()[11],
    ",
  0x403c6730u64 => "
      MFWD.fwpmfdcn()[12],
    ",
  0x403c6734u64 => "
      MFWD.fwpmfdcn()[13],
    ",
  0x403c6738u64 => "
      MFWD.fwpmfdcn()[14],
    ",
  0x403c673cu64 => "
      MFWD.fwpmfdcn()[15],
    ",
  0x403c6800u64 => "
      MFWD.fwpmgdcn()[0],
    ",
  0x403c6810u64 => "
      MFWD.fwpmgdcn()[1],
    ",
  0x403c6820u64 => "
      MFWD.fwpmgdcn()[2],
    ",
  0x403c6830u64 => "
      MFWD.fwpmgdcn()[3],
    ",
  0x403c6840u64 => "
      MFWD.fwpmgdcn()[4],
    ",
  0x403c6850u64 => "
      MFWD.fwpmgdcn()[5],
    ",
  0x403c6860u64 => "
      MFWD.fwpmgdcn()[6],
    ",
  0x403c6870u64 => "
      MFWD.fwpmgdcn()[7],
    ",
  0x403c6880u64 => "
      MFWD.fwpmgdcn()[8],
    ",
  0x403c6890u64 => "
      MFWD.fwpmgdcn()[9],
    ",
  0x403c68a0u64 => "
      MFWD.fwpmgdcn()[10],
    ",
  0x403c68b0u64 => "
      MFWD.fwpmgdcn()[11],
    ",
  0x403c68c0u64 => "
      MFWD.fwpmgdcn()[12],
    ",
  0x403c68d0u64 => "
      MFWD.fwpmgdcn()[13],
    ",
  0x403c68e0u64 => "
      MFWD.fwpmgdcn()[14],
    ",
  0x403c68f0u64 => "
      MFWD.fwpmgdcn()[15],
    ",
  0x403c6900u64 => "
      MFWD.fwpmgdcn()[16],
    ",
  0x403c6910u64 => "
      MFWD.fwpmgdcn()[17],
    ",
  0x403c6920u64 => "
      MFWD.fwpmgdcn()[18],
    ",
  0x403c6930u64 => "
      MFWD.fwpmgdcn()[19],
    ",
  0x403c6940u64 => "
      MFWD.fwpmgdcn()[20],
    ",
  0x403c6950u64 => "
      MFWD.fwpmgdcn()[21],
    ",
  0x403c6960u64 => "
      MFWD.fwpmgdcn()[22],
    ",
  0x403c6970u64 => "
      MFWD.fwpmgdcn()[23],
    ",
  0x403c6980u64 => "
      MFWD.fwpmgdcn()[24],
    ",
  0x403c6990u64 => "
      MFWD.fwpmgdcn()[25],
    ",
  0x403c69a0u64 => "
      MFWD.fwpmgdcn()[26],
    ",
  0x403c69b0u64 => "
      MFWD.fwpmgdcn()[27],
    ",
  0x403c69c0u64 => "
      MFWD.fwpmgdcn()[28],
    ",
  0x403c69d0u64 => "
      MFWD.fwpmgdcn()[29],
    ",
  0x403c69e0u64 => "
      MFWD.fwpmgdcn()[30],
    ",
  0x403c69f0u64 => "
      MFWD.fwpmgdcn()[31],
    ",
  0x403c6804u64 => "
      MFWD.fwpmydcn()[0],
    ",
  0x403c6814u64 => "
      MFWD.fwpmydcn()[1],
    ",
  0x403c6824u64 => "
      MFWD.fwpmydcn()[2],
    ",
  0x403c6834u64 => "
      MFWD.fwpmydcn()[3],
    ",
  0x403c6844u64 => "
      MFWD.fwpmydcn()[4],
    ",
  0x403c6854u64 => "
      MFWD.fwpmydcn()[5],
    ",
  0x403c6864u64 => "
      MFWD.fwpmydcn()[6],
    ",
  0x403c6874u64 => "
      MFWD.fwpmydcn()[7],
    ",
  0x403c6808u64 => "
      MFWD.fwpmrdcn()[0],
    ",
  0x403c6818u64 => "
      MFWD.fwpmrdcn()[1],
    ",
  0x403c6828u64 => "
      MFWD.fwpmrdcn()[2],
    ",
  0x403c6838u64 => "
      MFWD.fwpmrdcn()[3],
    ",
  0x403c6848u64 => "
      MFWD.fwpmrdcn()[4],
    ",
  0x403c6858u64 => "
      MFWD.fwpmrdcn()[5],
    ",
  0x403c6868u64 => "
      MFWD.fwpmrdcn()[6],
    ",
  0x403c6878u64 => "
      MFWD.fwpmrdcn()[7],
    ",
  0x403c6888u64 => "
      MFWD.fwpmrdcn()[8],
    ",
  0x403c6898u64 => "
      MFWD.fwpmrdcn()[9],
    ",
  0x403c68a8u64 => "
      MFWD.fwpmrdcn()[10],
    ",
  0x403c68b8u64 => "
      MFWD.fwpmrdcn()[11],
    ",
  0x403c68c8u64 => "
      MFWD.fwpmrdcn()[12],
    ",
  0x403c68d8u64 => "
      MFWD.fwpmrdcn()[13],
    ",
  0x403c68e8u64 => "
      MFWD.fwpmrdcn()[14],
    ",
  0x403c68f8u64 => "
      MFWD.fwpmrdcn()[15],
    ",
  0x403c6908u64 => "
      MFWD.fwpmrdcn()[16],
    ",
  0x403c6918u64 => "
      MFWD.fwpmrdcn()[17],
    ",
  0x403c6928u64 => "
      MFWD.fwpmrdcn()[18],
    ",
  0x403c6938u64 => "
      MFWD.fwpmrdcn()[19],
    ",
  0x403c6948u64 => "
      MFWD.fwpmrdcn()[20],
    ",
  0x403c6958u64 => "
      MFWD.fwpmrdcn()[21],
    ",
  0x403c6968u64 => "
      MFWD.fwpmrdcn()[22],
    ",
  0x403c6978u64 => "
      MFWD.fwpmrdcn()[23],
    ",
  0x403c6988u64 => "
      MFWD.fwpmrdcn()[24],
    ",
  0x403c6998u64 => "
      MFWD.fwpmrdcn()[25],
    ",
  0x403c69a8u64 => "
      MFWD.fwpmrdcn()[26],
    ",
  0x403c69b8u64 => "
      MFWD.fwpmrdcn()[27],
    ",
  0x403c69c8u64 => "
      MFWD.fwpmrdcn()[28],
    ",
  0x403c69d8u64 => "
      MFWD.fwpmrdcn()[29],
    ",
  0x403c69e8u64 => "
      MFWD.fwpmrdcn()[30],
    ",
  0x403c69f8u64 => "
      MFWD.fwpmrdcn()[31],
    ",
  0x403c6a00u64 => "
      MFWD.fwfrppcn()[0],
    ",
  0x403c6a08u64 => "
      MFWD.fwfrppcn()[1],
    ",
  0x403c6a10u64 => "
      MFWD.fwfrppcn()[2],
    ",
  0x403c6a18u64 => "
      MFWD.fwfrppcn()[3],
    ",
  0x403c6a20u64 => "
      MFWD.fwfrppcn()[4],
    ",
  0x403c6a28u64 => "
      MFWD.fwfrppcn()[5],
    ",
  0x403c6a30u64 => "
      MFWD.fwfrppcn()[6],
    ",
  0x403c6a38u64 => "
      MFWD.fwfrppcn()[7],
    ",
  0x403c6a40u64 => "
      MFWD.fwfrppcn()[8],
    ",
  0x403c6a48u64 => "
      MFWD.fwfrppcn()[9],
    ",
  0x403c6a50u64 => "
      MFWD.fwfrppcn()[10],
    ",
  0x403c6a58u64 => "
      MFWD.fwfrppcn()[11],
    ",
  0x403c6a60u64 => "
      MFWD.fwfrppcn()[12],
    ",
  0x403c6a68u64 => "
      MFWD.fwfrppcn()[13],
    ",
  0x403c6a70u64 => "
      MFWD.fwfrppcn()[14],
    ",
  0x403c6a78u64 => "
      MFWD.fwfrppcn()[15],
    ",
  0x403c6a80u64 => "
      MFWD.fwfrppcn()[16],
    ",
  0x403c6a88u64 => "
      MFWD.fwfrppcn()[17],
    ",
  0x403c6a90u64 => "
      MFWD.fwfrppcn()[18],
    ",
  0x403c6a98u64 => "
      MFWD.fwfrppcn()[19],
    ",
  0x403c6aa0u64 => "
      MFWD.fwfrppcn()[20],
    ",
  0x403c6aa8u64 => "
      MFWD.fwfrppcn()[21],
    ",
  0x403c6ab0u64 => "
      MFWD.fwfrppcn()[22],
    ",
  0x403c6ab8u64 => "
      MFWD.fwfrppcn()[23],
    ",
  0x403c6ac0u64 => "
      MFWD.fwfrppcn()[24],
    ",
  0x403c6ac8u64 => "
      MFWD.fwfrppcn()[25],
    ",
  0x403c6ad0u64 => "
      MFWD.fwfrppcn()[26],
    ",
  0x403c6ad8u64 => "
      MFWD.fwfrppcn()[27],
    ",
  0x403c6ae0u64 => "
      MFWD.fwfrppcn()[28],
    ",
  0x403c6ae8u64 => "
      MFWD.fwfrppcn()[29],
    ",
  0x403c6af0u64 => "
      MFWD.fwfrppcn()[30],
    ",
  0x403c6af8u64 => "
      MFWD.fwfrppcn()[31],
    ",
  0x403c6b00u64 => "
      MFWD.fwfrppcn()[32],
    ",
  0x403c6b08u64 => "
      MFWD.fwfrppcn()[33],
    ",
  0x403c6b10u64 => "
      MFWD.fwfrppcn()[34],
    ",
  0x403c6b18u64 => "
      MFWD.fwfrppcn()[35],
    ",
  0x403c6b20u64 => "
      MFWD.fwfrppcn()[36],
    ",
  0x403c6b28u64 => "
      MFWD.fwfrppcn()[37],
    ",
  0x403c6b30u64 => "
      MFWD.fwfrppcn()[38],
    ",
  0x403c6b38u64 => "
      MFWD.fwfrppcn()[39],
    ",
  0x403c6b40u64 => "
      MFWD.fwfrppcn()[40],
    ",
  0x403c6b48u64 => "
      MFWD.fwfrppcn()[41],
    ",
  0x403c6b50u64 => "
      MFWD.fwfrppcn()[42],
    ",
  0x403c6b58u64 => "
      MFWD.fwfrppcn()[43],
    ",
  0x403c6b60u64 => "
      MFWD.fwfrppcn()[44],
    ",
  0x403c6b68u64 => "
      MFWD.fwfrppcn()[45],
    ",
  0x403c6b70u64 => "
      MFWD.fwfrppcn()[46],
    ",
  0x403c6b78u64 => "
      MFWD.fwfrppcn()[47],
    ",
  0x403c6b80u64 => "
      MFWD.fwfrppcn()[48],
    ",
  0x403c6b88u64 => "
      MFWD.fwfrppcn()[49],
    ",
  0x403c6b90u64 => "
      MFWD.fwfrppcn()[50],
    ",
  0x403c6b98u64 => "
      MFWD.fwfrppcn()[51],
    ",
  0x403c6ba0u64 => "
      MFWD.fwfrppcn()[52],
    ",
  0x403c6ba8u64 => "
      MFWD.fwfrppcn()[53],
    ",
  0x403c6bb0u64 => "
      MFWD.fwfrppcn()[54],
    ",
  0x403c6bb8u64 => "
      MFWD.fwfrppcn()[55],
    ",
  0x403c6bc0u64 => "
      MFWD.fwfrppcn()[56],
    ",
  0x403c6bc8u64 => "
      MFWD.fwfrppcn()[57],
    ",
  0x403c6bd0u64 => "
      MFWD.fwfrppcn()[58],
    ",
  0x403c6bd8u64 => "
      MFWD.fwfrppcn()[59],
    ",
  0x403c6be0u64 => "
      MFWD.fwfrppcn()[60],
    ",
  0x403c6be8u64 => "
      MFWD.fwfrppcn()[61],
    ",
  0x403c6bf0u64 => "
      MFWD.fwfrppcn()[62],
    ",
  0x403c6bf8u64 => "
      MFWD.fwfrppcn()[63],
    ",
  0x403c6c00u64 => "
      MFWD.fwfrppcn()[64],
    ",
  0x403c6c08u64 => "
      MFWD.fwfrppcn()[65],
    ",
  0x403c6c10u64 => "
      MFWD.fwfrppcn()[66],
    ",
  0x403c6c18u64 => "
      MFWD.fwfrppcn()[67],
    ",
  0x403c6c20u64 => "
      MFWD.fwfrppcn()[68],
    ",
  0x403c6c28u64 => "
      MFWD.fwfrppcn()[69],
    ",
  0x403c6c30u64 => "
      MFWD.fwfrppcn()[70],
    ",
  0x403c6c38u64 => "
      MFWD.fwfrppcn()[71],
    ",
  0x403c6c40u64 => "
      MFWD.fwfrppcn()[72],
    ",
  0x403c6c48u64 => "
      MFWD.fwfrppcn()[73],
    ",
  0x403c6c50u64 => "
      MFWD.fwfrppcn()[74],
    ",
  0x403c6c58u64 => "
      MFWD.fwfrppcn()[75],
    ",
  0x403c6c60u64 => "
      MFWD.fwfrppcn()[76],
    ",
  0x403c6c68u64 => "
      MFWD.fwfrppcn()[77],
    ",
  0x403c6c70u64 => "
      MFWD.fwfrppcn()[78],
    ",
  0x403c6c78u64 => "
      MFWD.fwfrppcn()[79],
    ",
  0x403c6c80u64 => "
      MFWD.fwfrppcn()[80],
    ",
  0x403c6c88u64 => "
      MFWD.fwfrppcn()[81],
    ",
  0x403c6c90u64 => "
      MFWD.fwfrppcn()[82],
    ",
  0x403c6c98u64 => "
      MFWD.fwfrppcn()[83],
    ",
  0x403c6ca0u64 => "
      MFWD.fwfrppcn()[84],
    ",
  0x403c6ca8u64 => "
      MFWD.fwfrppcn()[85],
    ",
  0x403c6cb0u64 => "
      MFWD.fwfrppcn()[86],
    ",
  0x403c6cb8u64 => "
      MFWD.fwfrppcn()[87],
    ",
  0x403c6cc0u64 => "
      MFWD.fwfrppcn()[88],
    ",
  0x403c6cc8u64 => "
      MFWD.fwfrppcn()[89],
    ",
  0x403c6cd0u64 => "
      MFWD.fwfrppcn()[90],
    ",
  0x403c6cd8u64 => "
      MFWD.fwfrppcn()[91],
    ",
  0x403c6ce0u64 => "
      MFWD.fwfrppcn()[92],
    ",
  0x403c6ce8u64 => "
      MFWD.fwfrppcn()[93],
    ",
  0x403c6cf0u64 => "
      MFWD.fwfrppcn()[94],
    ",
  0x403c6cf8u64 => "
      MFWD.fwfrppcn()[95],
    ",
  0x403c6d00u64 => "
      MFWD.fwfrppcn()[96],
    ",
  0x403c6d08u64 => "
      MFWD.fwfrppcn()[97],
    ",
  0x403c6d10u64 => "
      MFWD.fwfrppcn()[98],
    ",
  0x403c6d18u64 => "
      MFWD.fwfrppcn()[99],
    ",
  0x403c6d20u64 => "
      MFWD.fwfrppcn()[100],
    ",
  0x403c6d28u64 => "
      MFWD.fwfrppcn()[101],
    ",
  0x403c6d30u64 => "
      MFWD.fwfrppcn()[102],
    ",
  0x403c6d38u64 => "
      MFWD.fwfrppcn()[103],
    ",
  0x403c6d40u64 => "
      MFWD.fwfrppcn()[104],
    ",
  0x403c6d48u64 => "
      MFWD.fwfrppcn()[105],
    ",
  0x403c6d50u64 => "
      MFWD.fwfrppcn()[106],
    ",
  0x403c6d58u64 => "
      MFWD.fwfrppcn()[107],
    ",
  0x403c6d60u64 => "
      MFWD.fwfrppcn()[108],
    ",
  0x403c6d68u64 => "
      MFWD.fwfrppcn()[109],
    ",
  0x403c6d70u64 => "
      MFWD.fwfrppcn()[110],
    ",
  0x403c6d78u64 => "
      MFWD.fwfrppcn()[111],
    ",
  0x403c6d80u64 => "
      MFWD.fwfrppcn()[112],
    ",
  0x403c6d88u64 => "
      MFWD.fwfrppcn()[113],
    ",
  0x403c6d90u64 => "
      MFWD.fwfrppcn()[114],
    ",
  0x403c6d98u64 => "
      MFWD.fwfrppcn()[115],
    ",
  0x403c6da0u64 => "
      MFWD.fwfrppcn()[116],
    ",
  0x403c6da8u64 => "
      MFWD.fwfrppcn()[117],
    ",
  0x403c6db0u64 => "
      MFWD.fwfrppcn()[118],
    ",
  0x403c6db8u64 => "
      MFWD.fwfrppcn()[119],
    ",
  0x403c6dc0u64 => "
      MFWD.fwfrppcn()[120],
    ",
  0x403c6dc8u64 => "
      MFWD.fwfrppcn()[121],
    ",
  0x403c6dd0u64 => "
      MFWD.fwfrppcn()[122],
    ",
  0x403c6dd8u64 => "
      MFWD.fwfrppcn()[123],
    ",
  0x403c6de0u64 => "
      MFWD.fwfrppcn()[124],
    ",
  0x403c6de8u64 => "
      MFWD.fwfrppcn()[125],
    ",
  0x403c6df0u64 => "
      MFWD.fwfrppcn()[126],
    ",
  0x403c6df8u64 => "
      MFWD.fwfrppcn()[127],
    ",
  0x403c6a04u64 => "
      MFWD.fwfrdpcn()[0],
    ",
  0x403c6a0cu64 => "
      MFWD.fwfrdpcn()[1],
    ",
  0x403c6a14u64 => "
      MFWD.fwfrdpcn()[2],
    ",
  0x403c6a1cu64 => "
      MFWD.fwfrdpcn()[3],
    ",
  0x403c6a24u64 => "
      MFWD.fwfrdpcn()[4],
    ",
  0x403c6a2cu64 => "
      MFWD.fwfrdpcn()[5],
    ",
  0x403c6a34u64 => "
      MFWD.fwfrdpcn()[6],
    ",
  0x403c6a3cu64 => "
      MFWD.fwfrdpcn()[7],
    ",
  0x403c6a44u64 => "
      MFWD.fwfrdpcn()[8],
    ",
  0x403c6a4cu64 => "
      MFWD.fwfrdpcn()[9],
    ",
  0x403c6a54u64 => "
      MFWD.fwfrdpcn()[10],
    ",
  0x403c6a5cu64 => "
      MFWD.fwfrdpcn()[11],
    ",
  0x403c6a64u64 => "
      MFWD.fwfrdpcn()[12],
    ",
  0x403c6a6cu64 => "
      MFWD.fwfrdpcn()[13],
    ",
  0x403c6a74u64 => "
      MFWD.fwfrdpcn()[14],
    ",
  0x403c6a7cu64 => "
      MFWD.fwfrdpcn()[15],
    ",
  0x403c6a84u64 => "
      MFWD.fwfrdpcn()[16],
    ",
  0x403c6a8cu64 => "
      MFWD.fwfrdpcn()[17],
    ",
  0x403c6a94u64 => "
      MFWD.fwfrdpcn()[18],
    ",
  0x403c6a9cu64 => "
      MFWD.fwfrdpcn()[19],
    ",
  0x403c6aa4u64 => "
      MFWD.fwfrdpcn()[20],
    ",
  0x403c6aacu64 => "
      MFWD.fwfrdpcn()[21],
    ",
  0x403c6ab4u64 => "
      MFWD.fwfrdpcn()[22],
    ",
  0x403c6abcu64 => "
      MFWD.fwfrdpcn()[23],
    ",
  0x403c6ac4u64 => "
      MFWD.fwfrdpcn()[24],
    ",
  0x403c6accu64 => "
      MFWD.fwfrdpcn()[25],
    ",
  0x403c6ad4u64 => "
      MFWD.fwfrdpcn()[26],
    ",
  0x403c6adcu64 => "
      MFWD.fwfrdpcn()[27],
    ",
  0x403c6ae4u64 => "
      MFWD.fwfrdpcn()[28],
    ",
  0x403c6aecu64 => "
      MFWD.fwfrdpcn()[29],
    ",
  0x403c6af4u64 => "
      MFWD.fwfrdpcn()[30],
    ",
  0x403c6afcu64 => "
      MFWD.fwfrdpcn()[31],
    ",
  0x403c6b04u64 => "
      MFWD.fwfrdpcn()[32],
    ",
  0x403c6b0cu64 => "
      MFWD.fwfrdpcn()[33],
    ",
  0x403c6b14u64 => "
      MFWD.fwfrdpcn()[34],
    ",
  0x403c6b1cu64 => "
      MFWD.fwfrdpcn()[35],
    ",
  0x403c6b24u64 => "
      MFWD.fwfrdpcn()[36],
    ",
  0x403c6b2cu64 => "
      MFWD.fwfrdpcn()[37],
    ",
  0x403c6b34u64 => "
      MFWD.fwfrdpcn()[38],
    ",
  0x403c6b3cu64 => "
      MFWD.fwfrdpcn()[39],
    ",
  0x403c6b44u64 => "
      MFWD.fwfrdpcn()[40],
    ",
  0x403c6b4cu64 => "
      MFWD.fwfrdpcn()[41],
    ",
  0x403c6b54u64 => "
      MFWD.fwfrdpcn()[42],
    ",
  0x403c6b5cu64 => "
      MFWD.fwfrdpcn()[43],
    ",
  0x403c6b64u64 => "
      MFWD.fwfrdpcn()[44],
    ",
  0x403c6b6cu64 => "
      MFWD.fwfrdpcn()[45],
    ",
  0x403c6b74u64 => "
      MFWD.fwfrdpcn()[46],
    ",
  0x403c6b7cu64 => "
      MFWD.fwfrdpcn()[47],
    ",
  0x403c6b84u64 => "
      MFWD.fwfrdpcn()[48],
    ",
  0x403c6b8cu64 => "
      MFWD.fwfrdpcn()[49],
    ",
  0x403c6b94u64 => "
      MFWD.fwfrdpcn()[50],
    ",
  0x403c6b9cu64 => "
      MFWD.fwfrdpcn()[51],
    ",
  0x403c6ba4u64 => "
      MFWD.fwfrdpcn()[52],
    ",
  0x403c6bacu64 => "
      MFWD.fwfrdpcn()[53],
    ",
  0x403c6bb4u64 => "
      MFWD.fwfrdpcn()[54],
    ",
  0x403c6bbcu64 => "
      MFWD.fwfrdpcn()[55],
    ",
  0x403c6bc4u64 => "
      MFWD.fwfrdpcn()[56],
    ",
  0x403c6bccu64 => "
      MFWD.fwfrdpcn()[57],
    ",
  0x403c6bd4u64 => "
      MFWD.fwfrdpcn()[58],
    ",
  0x403c6bdcu64 => "
      MFWD.fwfrdpcn()[59],
    ",
  0x403c6be4u64 => "
      MFWD.fwfrdpcn()[60],
    ",
  0x403c6becu64 => "
      MFWD.fwfrdpcn()[61],
    ",
  0x403c6bf4u64 => "
      MFWD.fwfrdpcn()[62],
    ",
  0x403c6bfcu64 => "
      MFWD.fwfrdpcn()[63],
    ",
  0x403c6c04u64 => "
      MFWD.fwfrdpcn()[64],
    ",
  0x403c6c0cu64 => "
      MFWD.fwfrdpcn()[65],
    ",
  0x403c6c14u64 => "
      MFWD.fwfrdpcn()[66],
    ",
  0x403c6c1cu64 => "
      MFWD.fwfrdpcn()[67],
    ",
  0x403c6c24u64 => "
      MFWD.fwfrdpcn()[68],
    ",
  0x403c6c2cu64 => "
      MFWD.fwfrdpcn()[69],
    ",
  0x403c6c34u64 => "
      MFWD.fwfrdpcn()[70],
    ",
  0x403c6c3cu64 => "
      MFWD.fwfrdpcn()[71],
    ",
  0x403c6c44u64 => "
      MFWD.fwfrdpcn()[72],
    ",
  0x403c6c4cu64 => "
      MFWD.fwfrdpcn()[73],
    ",
  0x403c6c54u64 => "
      MFWD.fwfrdpcn()[74],
    ",
  0x403c6c5cu64 => "
      MFWD.fwfrdpcn()[75],
    ",
  0x403c6c64u64 => "
      MFWD.fwfrdpcn()[76],
    ",
  0x403c6c6cu64 => "
      MFWD.fwfrdpcn()[77],
    ",
  0x403c6c74u64 => "
      MFWD.fwfrdpcn()[78],
    ",
  0x403c6c7cu64 => "
      MFWD.fwfrdpcn()[79],
    ",
  0x403c6c84u64 => "
      MFWD.fwfrdpcn()[80],
    ",
  0x403c6c8cu64 => "
      MFWD.fwfrdpcn()[81],
    ",
  0x403c6c94u64 => "
      MFWD.fwfrdpcn()[82],
    ",
  0x403c6c9cu64 => "
      MFWD.fwfrdpcn()[83],
    ",
  0x403c6ca4u64 => "
      MFWD.fwfrdpcn()[84],
    ",
  0x403c6cacu64 => "
      MFWD.fwfrdpcn()[85],
    ",
  0x403c6cb4u64 => "
      MFWD.fwfrdpcn()[86],
    ",
  0x403c6cbcu64 => "
      MFWD.fwfrdpcn()[87],
    ",
  0x403c6cc4u64 => "
      MFWD.fwfrdpcn()[88],
    ",
  0x403c6cccu64 => "
      MFWD.fwfrdpcn()[89],
    ",
  0x403c6cd4u64 => "
      MFWD.fwfrdpcn()[90],
    ",
  0x403c6cdcu64 => "
      MFWD.fwfrdpcn()[91],
    ",
  0x403c6ce4u64 => "
      MFWD.fwfrdpcn()[92],
    ",
  0x403c6cecu64 => "
      MFWD.fwfrdpcn()[93],
    ",
  0x403c6cf4u64 => "
      MFWD.fwfrdpcn()[94],
    ",
  0x403c6cfcu64 => "
      MFWD.fwfrdpcn()[95],
    ",
  0x403c6d04u64 => "
      MFWD.fwfrdpcn()[96],
    ",
  0x403c6d0cu64 => "
      MFWD.fwfrdpcn()[97],
    ",
  0x403c6d14u64 => "
      MFWD.fwfrdpcn()[98],
    ",
  0x403c6d1cu64 => "
      MFWD.fwfrdpcn()[99],
    ",
  0x403c6d24u64 => "
      MFWD.fwfrdpcn()[100],
    ",
  0x403c6d2cu64 => "
      MFWD.fwfrdpcn()[101],
    ",
  0x403c6d34u64 => "
      MFWD.fwfrdpcn()[102],
    ",
  0x403c6d3cu64 => "
      MFWD.fwfrdpcn()[103],
    ",
  0x403c6d44u64 => "
      MFWD.fwfrdpcn()[104],
    ",
  0x403c6d4cu64 => "
      MFWD.fwfrdpcn()[105],
    ",
  0x403c6d54u64 => "
      MFWD.fwfrdpcn()[106],
    ",
  0x403c6d5cu64 => "
      MFWD.fwfrdpcn()[107],
    ",
  0x403c6d64u64 => "
      MFWD.fwfrdpcn()[108],
    ",
  0x403c6d6cu64 => "
      MFWD.fwfrdpcn()[109],
    ",
  0x403c6d74u64 => "
      MFWD.fwfrdpcn()[110],
    ",
  0x403c6d7cu64 => "
      MFWD.fwfrdpcn()[111],
    ",
  0x403c6d84u64 => "
      MFWD.fwfrdpcn()[112],
    ",
  0x403c6d8cu64 => "
      MFWD.fwfrdpcn()[113],
    ",
  0x403c6d94u64 => "
      MFWD.fwfrdpcn()[114],
    ",
  0x403c6d9cu64 => "
      MFWD.fwfrdpcn()[115],
    ",
  0x403c6da4u64 => "
      MFWD.fwfrdpcn()[116],
    ",
  0x403c6dacu64 => "
      MFWD.fwfrdpcn()[117],
    ",
  0x403c6db4u64 => "
      MFWD.fwfrdpcn()[118],
    ",
  0x403c6dbcu64 => "
      MFWD.fwfrdpcn()[119],
    ",
  0x403c6dc4u64 => "
      MFWD.fwfrdpcn()[120],
    ",
  0x403c6dccu64 => "
      MFWD.fwfrdpcn()[121],
    ",
  0x403c6dd4u64 => "
      MFWD.fwfrdpcn()[122],
    ",
  0x403c6ddcu64 => "
      MFWD.fwfrdpcn()[123],
    ",
  0x403c6de4u64 => "
      MFWD.fwfrdpcn()[124],
    ",
  0x403c6decu64 => "
      MFWD.fwfrdpcn()[125],
    ",
  0x403c6df4u64 => "
      MFWD.fwfrdpcn()[126],
    ",
  0x403c6dfcu64 => "
      MFWD.fwfrdpcn()[127],
    ",
  0x403c7900u64 => "
      MFWD.fweis0()[0],
    ",
  0x403c7910u64 => "
      MFWD.fweis0()[1],
    ",
  0x403c7920u64 => "
      MFWD.fweis0()[2],
    ",
  0x403c7904u64 => "
      MFWD.fweie0()[0],
    ",
  0x403c7914u64 => "
      MFWD.fweie0()[1],
    ",
  0x403c7924u64 => "
      MFWD.fweie0()[2],
    ",
  0x403c7908u64 => "
      MFWD.fweid0()[0],
    ",
  0x403c7918u64 => "
      MFWD.fweid0()[1],
    ",
  0x403c7928u64 => "
      MFWD.fweid0()[2],
    ",
  0x403c7a00u64 => "
      MFWD.fweis1(),
    ",
  0x403c7a04u64 => "
      MFWD.fweie1(),
    ",
  0x403c7a08u64 => "
      MFWD.fweid1(),
    ",
  0x403c7a10u64 => "
      MFWD.fweis2(),
    ",
  0x403c7a14u64 => "
      MFWD.fweie2(),
    ",
  0x403c7a18u64 => "
      MFWD.fweid2(),
    ",
  0x403c7a40u64 => "
      MFWD.fweis5(),
    ",
  0x403c7a44u64 => "
      MFWD.fweie5(),
    ",
  0x403c7a48u64 => "
      MFWD.fweid5(),
    ",
  0x403c7a50u64 => "
      MFWD.fweis60(),
    ",
  0x403c7a54u64 => "
      MFWD.fweie60(),
    ",
  0x403c7a58u64 => "
      MFWD.fweid60(),
    ",
  0x403c7a60u64 => "
      MFWD.fweis61(),
    ",
  0x403c7a64u64 => "
      MFWD.fweie61(),
    ",
  0x403c7a68u64 => "
      MFWD.fweid61(),
    ",
  0x403c7a70u64 => "
      MFWD.fweis62(),
    ",
  0x403c7a74u64 => "
      MFWD.fweie62(),
    ",
  0x403c7a78u64 => "
      MFWD.fweid62(),
    ",
  0x403c7a80u64 => "
      MFWD.fweis63(),
    ",
  0x403c7a84u64 => "
      MFWD.fweie63(),
    ",
  0x403c7a88u64 => "
      MFWD.fweid63(),
    ",
  0x403c7a90u64 => "
      MFWD.fweis70(),
    ",
  0x403c7a94u64 => "
      MFWD.fweie70(),
    ",
  0x403c7a98u64 => "
      MFWD.fweid70(),
    ",
  0x403c7aa0u64 => "
      MFWD.fweis71(),
    ",
  0x403c7aa4u64 => "
      MFWD.fweie71(),
    ",
  0x403c7aa8u64 => "
      MFWD.fweid71(),
    ",
  0x403c7ab0u64 => "
      MFWD.fweis72(),
    ",
  0x403c7ab4u64 => "
      MFWD.fweie72(),
    ",
  0x403c7ab8u64 => "
      MFWD.fweid72(),
    ",
  0x403c7ac0u64 => "
      MFWD.fweis73(),
    ",
  0x403c7ac4u64 => "
      MFWD.fweie73(),
    ",
  0x403c7ac8u64 => "
      MFWD.fweid73(),
    ",
  0x403c7ad0u64 => "
      MFWD.fweis80(),
    ",
  0x403c7ad4u64 => "
      MFWD.fweie80(),
    ",
  0x403c7ad8u64 => "
      MFWD.fweid80(),
    ",
  0x403c7ae0u64 => "
      MFWD.fweis81(),
    ",
  0x403c7ae4u64 => "
      MFWD.fweie81(),
    ",
  0x403c7ae8u64 => "
      MFWD.fweid81(),
    ",
  0x403c7af0u64 => "
      MFWD.fweis82(),
    ",
  0x403c7af4u64 => "
      MFWD.fweie82(),
    ",
  0x403c7af8u64 => "
      MFWD.fweid82(),
    ",
  0x403c7b00u64 => "
      MFWD.fweis83(),
    ",
  0x403c7b04u64 => "
      MFWD.fweie83(),
    ",
  0x403c7b08u64 => "
      MFWD.fweid83(),
    ",
  0x403c7c00u64 => "
      MFWD.fwmis0(),
    ",
  0x403c7c04u64 => "
      MFWD.fwmie0(),
    ",
  0x403c7c08u64 => "
      MFWD.fwmid0(),
    ",
  0x403c8000u64 => "
      ESWM.tpemimc0(),
    ",
  0x403c8004u64 => "
      ESWM.tpemimc1(),
    ",
  0x403c8008u64 => "
      ESWM.tpemimc2(),
    ",
  0x403c800cu64 => "
      ESWM.tpemimc3(),
    ",
  0x403c8010u64 => "
      ESWM.tpemimc4(),
    ",
  0x403c8080u64 => "
      ESWM.tpemimc6()[0],
    ",
  0x403c8084u64 => "
      ESWM.tpemimc6()[1],
    ",
  0x403c8088u64 => "
      ESWM.tpemimc6()[2],
    ",
  0x403c808cu64 => "
      ESWM.tpemimc6()[3],
    ",
  0x403c8090u64 => "
      ESWM.tpemimc6()[4],
    ",
  0x403c8100u64 => "
      ESWM.tpemimc7()[0],
    ",
  0x403c8104u64 => "
      ESWM.tpemimc7()[1],
    ",
  0x403c8108u64 => "
      ESWM.tpemimc7()[2],
    ",
  0x403c810cu64 => "
      ESWM.tpemimc7()[3],
    ",
  0x403c8110u64 => "
      ESWM.tpemimc7()[4],
    ",
  0x403c8700u64 => "
      ESWM.tsim(),
    ",
  0x403c8704u64 => "
      ESWM.tfim(),
    ",
  0x403c8708u64 => "
      ESWM.tcim(),
    ",
  0x403c8710u64 => "
      ESWM.tgim0(),
    ",
  0x403c8720u64 => "
      ESWM.teim0(),
    ",
  0x403c8724u64 => "
      ESWM.teim1(),
    ",
  0x403e1400u64 => "
      ESWM.miirr(),
    ",
  0x403e1404u64 => "
      ESWM.miicr0(),
    ",
  0x403e1408u64 => "
      ESWM.miicr1(),
    ",
  0x403e1410u64 => "
      ESWM.mccesr(),
    ",
  0x403e1420u64 => "
      ESWM.tasstsr(),
    ",
  0x403c9000u64 => "
      COMA.ripv(),
    ",
  0x403c9004u64 => "
      COMA.rrc(),
    ",
  0x403c9008u64 => "
      COMA.rcec(),
    ",
  0x403c900cu64 => "
      COMA.rcdc(),
    ",
  0x403c9020u64 => "
      COMA.cabpibwmc()[0],
    ",
  0x403c9024u64 => "
      COMA.cabpibwmc()[1],
    ",
  0x403c9028u64 => "
      COMA.cabpibwmc()[2],
    ",
  0x403c902cu64 => "
      COMA.cabpibwmc()[3],
    ",
  0x403c9030u64 => "
      COMA.cabpibwmc()[4],
    ",
  0x403c9034u64 => "
      COMA.cabpibwmc()[5],
    ",
  0x403c9038u64 => "
      COMA.cabpibwmc()[6],
    ",
  0x403c903cu64 => "
      COMA.cabpibwmc()[7],
    ",
  0x403c9040u64 => "
      COMA.cabpwmlc(),
    ",
  0x403c9050u64 => "
      COMA.cabppflci(),
    ",
  0x403c9060u64 => "
      COMA.cabppwmlc()[0],
    ",
  0x403c9064u64 => "
      COMA.cabppwmlc()[1],
    ",
  0x403c9068u64 => "
      COMA.cabppwmlc()[2],
    ",
  0x403c90a0u64 => "
      COMA.cabpppflc0()[0],
    ",
  0x403c90a4u64 => "
      COMA.cabpppflc0()[1],
    ",
  0x403c90a8u64 => "
      COMA.cabpppflc1()[0],
    ",
  0x403c90acu64 => "
      COMA.cabpppflc1()[1],
    ",
  0x403c90b0u64 => "
      COMA.cabpppflc2()[0],
    ",
  0x403c90b4u64 => "
      COMA.cabpppflc2()[1],
    ",
  0x403c9100u64 => "
      COMA.cabpulc()[0],
    ",
  0x403c9104u64 => "
      COMA.cabpulc()[1],
    ",
  0x403c9108u64 => "
      COMA.cabpulc()[2],
    ",
  0x403c9140u64 => "
      COMA.cabpirm(),
    ",
  0x403c9144u64 => "
      COMA.cabppcm(),
    ",
  0x403c9148u64 => "
      COMA.cabplcm(),
    ",
  0x403c9180u64 => "
      COMA.cabpcpm()[0],
    ",
  0x403c9184u64 => "
      COMA.cabpcpm()[1],
    ",
  0x403c9188u64 => "
      COMA.cabpcpm()[2],
    ",
  0x403c9200u64 => "
      COMA.cabpmcpm()[0],
    ",
  0x403c9204u64 => "
      COMA.cabpmcpm()[1],
    ",
  0x403c9208u64 => "
      COMA.cabpmcpm()[2],
    ",
  0x403c9300u64 => "
      COMA.cardnm(),
    ",
  0x403c9304u64 => "
      COMA.cardmnm(),
    ",
  0x403c9310u64 => "
      COMA.cardcn(),
    ",
  0x403c9400u64 => "
      COMA.caeis0(),
    ",
  0x403c9404u64 => "
      COMA.caeie0(),
    ",
  0x403c9408u64 => "
      COMA.caeid0(),
    ",
  0x403c9410u64 => "
      COMA.caeis1(),
    ",
  0x403c9414u64 => "
      COMA.caeie1(),
    ",
  0x403c9418u64 => "
      COMA.caeid1(),
    ",
  0x403c9440u64 => "
      COMA.camis0(),
    ",
  0x403c9444u64 => "
      COMA.camie0(),
    ",
  0x403c9448u64 => "
      COMA.camid0(),
    ",
  0x403c9450u64 => "
      COMA.camis1(),
    ",
  0x403c9454u64 => "
      COMA.camie1(),
    ",
  0x403c9458u64 => "
      COMA.camid1(),
    ",
  0x403cb000u64 => "
      RMAC_0.mpsm(),
    ",
  0x403cb004u64 => "
      RMAC_0.mpic(),
    ",
  0x403cb008u64 => "
      RMAC_0.mpim(),
    ",
  0x403cb010u64 => "
      RMAC_0.mioc(),
    ",
  0x403cb020u64 => "
      RMAC_0.mtffc(),
    ",
  0x403cb024u64 => "
      RMAC_0.mtpfc(),
    ",
  0x403cb028u64 => "
      RMAC_0.mtpfc2(),
    ",
  0x403cb030u64 => "
      RMAC_0.mtpfc3t(),
    ",
  0x403cb080u64 => "
      RMAC_0.mrgc(),
    ",
  0x403cb084u64 => "
      RMAC_0.mrmac0(),
    ",
  0x403cb088u64 => "
      RMAC_0.mrmac1(),
    ",
  0x403cb08cu64 => "
      RMAC_0.mrafc(),
    ",
  0x403cb090u64 => "
      RMAC_0.mrsce(),
    ",
  0x403cb094u64 => "
      RMAC_0.mrscp(),
    ",
  0x403cb098u64 => "
      RMAC_0.mrscc(),
    ",
  0x403cb09cu64 => "
      RMAC_0.mrfsce(),
    ",
  0x403cb0a0u64 => "
      RMAC_0.mrfscp(),
    ",
  0x403cb0a4u64 => "
      RMAC_0.mtrc(),
    ",
  0x403cb0acu64 => "
      RMAC_0.mrpfm(),
    ",
  0x403cb100u64 => "
      RMAC_0.mpfc()[0],
    ",
  0x403cb104u64 => "
      RMAC_0.mpfc()[1],
    ",
  0x403cb108u64 => "
      RMAC_0.mpfc()[2],
    ",
  0x403cb10cu64 => "
      RMAC_0.mpfc()[3],
    ",
  0x403cb110u64 => "
      RMAC_0.mpfc()[4],
    ",
  0x403cb114u64 => "
      RMAC_0.mpfc()[5],
    ",
  0x403cb118u64 => "
      RMAC_0.mpfc()[6],
    ",
  0x403cb11cu64 => "
      RMAC_0.mpfc()[7],
    ",
  0x403cb120u64 => "
      RMAC_0.mpfc()[8],
    ",
  0x403cb124u64 => "
      RMAC_0.mpfc()[9],
    ",
  0x403cb128u64 => "
      RMAC_0.mpfc()[10],
    ",
  0x403cb12cu64 => "
      RMAC_0.mpfc()[11],
    ",
  0x403cb130u64 => "
      RMAC_0.mpfc()[12],
    ",
  0x403cb134u64 => "
      RMAC_0.mpfc()[13],
    ",
  0x403cb138u64 => "
      RMAC_0.mpfc()[14],
    ",
  0x403cb13cu64 => "
      RMAC_0.mpfc()[15],
    ",
  0x403cb180u64 => "
      RMAC_0.mlvc(),
    ",
  0x403cb184u64 => "
      RMAC_0.meeec(),
    ",
  0x403cb188u64 => "
      RMAC_0.mlbc(),
    ",
  0x403cb200u64 => "
      RMAC_0.meis(),
    ",
  0x403cb204u64 => "
      RMAC_0.meie(),
    ",
  0x403cb208u64 => "
      RMAC_0.meid(),
    ",
  0x403cb210u64 => "
      RMAC_0.mmis0(),
    ",
  0x403cb214u64 => "
      RMAC_0.mmie0(),
    ",
  0x403cb218u64 => "
      RMAC_0.mmid0(),
    ",
  0x403cb220u64 => "
      RMAC_0.mmis1(),
    ",
  0x403cb224u64 => "
      RMAC_0.mmie1(),
    ",
  0x403cb228u64 => "
      RMAC_0.mmid1(),
    ",
  0x403cb230u64 => "
      RMAC_0.mmis2(),
    ",
  0x403cb234u64 => "
      RMAC_0.mmie2(),
    ",
  0x403cb238u64 => "
      RMAC_0.mmid2(),
    ",
  0x403cb300u64 => "
      RMAC_0.mmpftct(),
    ",
  0x403cb304u64 => "
      RMAC_0.mapftct(),
    ",
  0x403cb308u64 => "
      RMAC_0.mpfrct(),
    ",
  0x403cb30cu64 => "
      RMAC_0.mfcict(),
    ",
  0x403cb310u64 => "
      RMAC_0.meeect(),
    ",
  0x403cb320u64 => "
      RMAC_0.mmpcftct()[0],
    ",
  0x403cb324u64 => "
      RMAC_0.mmpcftct()[1],
    ",
  0x403cb330u64 => "
      RMAC_0.mapcftct()[0],
    ",
  0x403cb334u64 => "
      RMAC_0.mapcftct()[1],
    ",
  0x403cb340u64 => "
      RMAC_0.mpcfrct()[0],
    ",
  0x403cb344u64 => "
      RMAC_0.mpcfrct()[1],
    ",
  0x403cb360u64 => "
      RMAC_0.mrovfc(),
    ",
  0x403cb408u64 => "
      RMAC_0.mrgfce(),
    ",
  0x403cb40cu64 => "
      RMAC_0.mrgfcp(),
    ",
  0x403cb410u64 => "
      RMAC_0.mrbfc(),
    ",
  0x403cb414u64 => "
      RMAC_0.mrmfc(),
    ",
  0x403cb418u64 => "
      RMAC_0.mrufc(),
    ",
  0x403cb41cu64 => "
      RMAC_0.mrpefc(),
    ",
  0x403cb420u64 => "
      RMAC_0.mrnefc(),
    ",
  0x403cb424u64 => "
      RMAC_0.mrfmefc(),
    ",
  0x403cb428u64 => "
      RMAC_0.mrffmefc(),
    ",
  0x403cb42cu64 => "
      RMAC_0.mrcfcefc(),
    ",
  0x403cb430u64 => "
      RMAC_0.mrfcefc(),
    ",
  0x403cb434u64 => "
      RMAC_0.mrrcfefc(),
    ",
  0x403cb438u64 => "
      RMAC_0.mrfc(),
    ",
  0x403cb43cu64 => "
      RMAC_0.mrguefc(),
    ",
  0x403cb440u64 => "
      RMAC_0.mrbuefc(),
    ",
  0x403cb444u64 => "
      RMAC_0.mrgoefc(),
    ",
  0x403cb448u64 => "
      RMAC_0.mrboefc(),
    ",
  0x403cb44cu64 => "
      RMAC_0.mrxbceu(),
    ",
  0x403cb450u64 => "
      RMAC_0.mrxbcel(),
    ",
  0x403cb454u64 => "
      RMAC_0.mrxbcpu(),
    ",
  0x403cb458u64 => "
      RMAC_0.mrxbcpl(),
    ",
  0x403cb508u64 => "
      RMAC_0.mtgfce(),
    ",
  0x403cb50cu64 => "
      RMAC_0.mtgfcp(),
    ",
  0x403cb510u64 => "
      RMAC_0.mtbfc(),
    ",
  0x403cb514u64 => "
      RMAC_0.mtmfc(),
    ",
  0x403cb518u64 => "
      RMAC_0.mtufc(),
    ",
  0x403cb51cu64 => "
      RMAC_0.mtefc(),
    ",
  0x403cb520u64 => "
      RMAC_0.mtxbceu(),
    ",
  0x403cb524u64 => "
      RMAC_0.mtxbcel(),
    ",
  0x403cb528u64 => "
      RMAC_0.mtxbcpu(),
    ",
  0x403cb52cu64 => "
      RMAC_0.mtxbcpl(),
    ",
  0x403ce000u64 => "
      GWCA_0.gwmc(),
    ",
  0x403ce004u64 => "
      GWCA_0.gwms(),
    ",
  0x403ce010u64 => "
      GWCA_0.gwirc(),
    ",
  0x403ce014u64 => "
      GWCA_0.gwrdqsc(),
    ",
  0x403ce018u64 => "
      GWCA_0.gwrdqc(),
    ",
  0x403ce01cu64 => "
      GWCA_0.gwrdqac(),
    ",
  0x403ce020u64 => "
      GWCA_0.gwrgc(),
    ",
  0x403ce040u64 => "
      GWCA_0.gwrmfsc()[0],
    ",
  0x403ce044u64 => "
      GWCA_0.gwrmfsc()[1],
    ",
  0x403ce048u64 => "
      GWCA_0.gwrmfsc()[2],
    ",
  0x403ce04cu64 => "
      GWCA_0.gwrmfsc()[3],
    ",
  0x403ce050u64 => "
      GWCA_0.gwrmfsc()[4],
    ",
  0x403ce054u64 => "
      GWCA_0.gwrmfsc()[5],
    ",
  0x403ce058u64 => "
      GWCA_0.gwrmfsc()[6],
    ",
  0x403ce05cu64 => "
      GWCA_0.gwrmfsc()[7],
    ",
  0x403ce060u64 => "
      GWCA_0.gwrdqdc()[0],
    ",
  0x403ce064u64 => "
      GWCA_0.gwrdqdc()[1],
    ",
  0x403ce068u64 => "
      GWCA_0.gwrdqdc()[2],
    ",
  0x403ce06cu64 => "
      GWCA_0.gwrdqdc()[3],
    ",
  0x403ce070u64 => "
      GWCA_0.gwrdqdc()[4],
    ",
  0x403ce074u64 => "
      GWCA_0.gwrdqdc()[5],
    ",
  0x403ce078u64 => "
      GWCA_0.gwrdqdc()[6],
    ",
  0x403ce07cu64 => "
      GWCA_0.gwrdqdc()[7],
    ",
  0x403ce080u64 => "
      GWCA_0.gwrdqm()[0],
    ",
  0x403ce084u64 => "
      GWCA_0.gwrdqm()[1],
    ",
  0x403ce088u64 => "
      GWCA_0.gwrdqm()[2],
    ",
  0x403ce08cu64 => "
      GWCA_0.gwrdqm()[3],
    ",
  0x403ce090u64 => "
      GWCA_0.gwrdqm()[4],
    ",
  0x403ce094u64 => "
      GWCA_0.gwrdqm()[5],
    ",
  0x403ce098u64 => "
      GWCA_0.gwrdqm()[6],
    ",
  0x403ce09cu64 => "
      GWCA_0.gwrdqm()[7],
    ",
  0x403ce0a0u64 => "
      GWCA_0.gwrdqmlm()[0],
    ",
  0x403ce0a4u64 => "
      GWCA_0.gwrdqmlm()[1],
    ",
  0x403ce0a8u64 => "
      GWCA_0.gwrdqmlm()[2],
    ",
  0x403ce0acu64 => "
      GWCA_0.gwrdqmlm()[3],
    ",
  0x403ce0b0u64 => "
      GWCA_0.gwrdqmlm()[4],
    ",
  0x403ce0b4u64 => "
      GWCA_0.gwrdqmlm()[5],
    ",
  0x403ce0b8u64 => "
      GWCA_0.gwrdqmlm()[6],
    ",
  0x403ce0bcu64 => "
      GWCA_0.gwrdqmlm()[7],
    ",
  0x403ce100u64 => "
      GWCA_0.gwmtirm(),
    ",
  0x403ce104u64 => "
      GWCA_0.gwmstls(),
    ",
  0x403ce108u64 => "
      GWCA_0.gwmstlr(),
    ",
  0x403ce10cu64 => "
      GWCA_0.gwmstss(),
    ",
  0x403ce110u64 => "
      GWCA_0.gwmstsr(),
    ",
  0x403ce120u64 => "
      GWCA_0.gwmac0(),
    ",
  0x403ce124u64 => "
      GWCA_0.gwmac1(),
    ",
  0x403ce130u64 => "
      GWCA_0.gwvcc(),
    ",
  0x403ce134u64 => "
      GWCA_0.gwvtc(),
    ",
  0x403ce138u64 => "
      GWCA_0.gwttfc(),
    ",
  0x403ce140u64 => "
      GWCA_0.gwtdcac0()[0],
    ",
  0x403ce148u64 => "
      GWCA_0.gwtdcac0()[1],
    ",
  0x403ce144u64 => "
      GWCA_0.gwtdcac1()[0],
    ",
  0x403ce14cu64 => "
      GWCA_0.gwtdcac1()[1],
    ",
  0x403ce160u64 => "
      GWCA_0.gwtsdcc()[0],
    ",
  0x403ce164u64 => "
      GWCA_0.gwtsdcc()[1],
    ",
  0x403ce180u64 => "
      GWCA_0.gwtsnm(),
    ",
  0x403ce184u64 => "
      GWCA_0.gwtsmnm(),
    ",
  0x403ce190u64 => "
      GWCA_0.gwac(),
    ",
  0x403ce194u64 => "
      GWCA_0.gwdcbac0(),
    ",
  0x403ce198u64 => "
      GWCA_0.gwdcbac1(),
    ",
  0x403ce1a0u64 => "
      GWCA_0.gwmdnc(),
    ",
  0x403ce200u64 => "
      GWCA_0.gwtrc0(),
    ",
  0x403ce204u64 => "
      GWCA_0.gwtrc1(),
    ",
  0x403ce300u64 => "
      GWCA_0.gwtpcp(),
    ",
  0x403ce380u64 => "
      GWCA_0.gwarirm(),
    ",
  0x403ce400u64 => "
      GWCA_0.gwdcc()[0],
    ",
  0x403ce404u64 => "
      GWCA_0.gwdcc()[1],
    ",
  0x403ce408u64 => "
      GWCA_0.gwdcc()[2],
    ",
  0x403ce40cu64 => "
      GWCA_0.gwdcc()[3],
    ",
  0x403ce410u64 => "
      GWCA_0.gwdcc()[4],
    ",
  0x403ce414u64 => "
      GWCA_0.gwdcc()[5],
    ",
  0x403ce418u64 => "
      GWCA_0.gwdcc()[6],
    ",
  0x403ce41cu64 => "
      GWCA_0.gwdcc()[7],
    ",
  0x403ce420u64 => "
      GWCA_0.gwdcc()[8],
    ",
  0x403ce424u64 => "
      GWCA_0.gwdcc()[9],
    ",
  0x403ce428u64 => "
      GWCA_0.gwdcc()[10],
    ",
  0x403ce42cu64 => "
      GWCA_0.gwdcc()[11],
    ",
  0x403ce430u64 => "
      GWCA_0.gwdcc()[12],
    ",
  0x403ce434u64 => "
      GWCA_0.gwdcc()[13],
    ",
  0x403ce438u64 => "
      GWCA_0.gwdcc()[14],
    ",
  0x403ce43cu64 => "
      GWCA_0.gwdcc()[15],
    ",
  0x403ce440u64 => "
      GWCA_0.gwdcc()[16],
    ",
  0x403ce444u64 => "
      GWCA_0.gwdcc()[17],
    ",
  0x403ce448u64 => "
      GWCA_0.gwdcc()[18],
    ",
  0x403ce44cu64 => "
      GWCA_0.gwdcc()[19],
    ",
  0x403ce450u64 => "
      GWCA_0.gwdcc()[20],
    ",
  0x403ce454u64 => "
      GWCA_0.gwdcc()[21],
    ",
  0x403ce458u64 => "
      GWCA_0.gwdcc()[22],
    ",
  0x403ce45cu64 => "
      GWCA_0.gwdcc()[23],
    ",
  0x403ce460u64 => "
      GWCA_0.gwdcc()[24],
    ",
  0x403ce464u64 => "
      GWCA_0.gwdcc()[25],
    ",
  0x403ce468u64 => "
      GWCA_0.gwdcc()[26],
    ",
  0x403ce46cu64 => "
      GWCA_0.gwdcc()[27],
    ",
  0x403ce470u64 => "
      GWCA_0.gwdcc()[28],
    ",
  0x403ce474u64 => "
      GWCA_0.gwdcc()[29],
    ",
  0x403ce478u64 => "
      GWCA_0.gwdcc()[30],
    ",
  0x403ce47cu64 => "
      GWCA_0.gwdcc()[31],
    ",
  0x403ce480u64 => "
      GWCA_0.gwdcc()[32],
    ",
  0x403ce484u64 => "
      GWCA_0.gwdcc()[33],
    ",
  0x403ce488u64 => "
      GWCA_0.gwdcc()[34],
    ",
  0x403ce48cu64 => "
      GWCA_0.gwdcc()[35],
    ",
  0x403ce490u64 => "
      GWCA_0.gwdcc()[36],
    ",
  0x403ce494u64 => "
      GWCA_0.gwdcc()[37],
    ",
  0x403ce498u64 => "
      GWCA_0.gwdcc()[38],
    ",
  0x403ce49cu64 => "
      GWCA_0.gwdcc()[39],
    ",
  0x403ce4a0u64 => "
      GWCA_0.gwdcc()[40],
    ",
  0x403ce4a4u64 => "
      GWCA_0.gwdcc()[41],
    ",
  0x403ce4a8u64 => "
      GWCA_0.gwdcc()[42],
    ",
  0x403ce4acu64 => "
      GWCA_0.gwdcc()[43],
    ",
  0x403ce4b0u64 => "
      GWCA_0.gwdcc()[44],
    ",
  0x403ce4b4u64 => "
      GWCA_0.gwdcc()[45],
    ",
  0x403ce4b8u64 => "
      GWCA_0.gwdcc()[46],
    ",
  0x403ce4bcu64 => "
      GWCA_0.gwdcc()[47],
    ",
  0x403ce4c0u64 => "
      GWCA_0.gwdcc()[48],
    ",
  0x403ce4c4u64 => "
      GWCA_0.gwdcc()[49],
    ",
  0x403ce4c8u64 => "
      GWCA_0.gwdcc()[50],
    ",
  0x403ce4ccu64 => "
      GWCA_0.gwdcc()[51],
    ",
  0x403ce4d0u64 => "
      GWCA_0.gwdcc()[52],
    ",
  0x403ce4d4u64 => "
      GWCA_0.gwdcc()[53],
    ",
  0x403ce4d8u64 => "
      GWCA_0.gwdcc()[54],
    ",
  0x403ce4dcu64 => "
      GWCA_0.gwdcc()[55],
    ",
  0x403ce4e0u64 => "
      GWCA_0.gwdcc()[56],
    ",
  0x403ce4e4u64 => "
      GWCA_0.gwdcc()[57],
    ",
  0x403ce4e8u64 => "
      GWCA_0.gwdcc()[58],
    ",
  0x403ce4ecu64 => "
      GWCA_0.gwdcc()[59],
    ",
  0x403ce4f0u64 => "
      GWCA_0.gwdcc()[60],
    ",
  0x403ce4f4u64 => "
      GWCA_0.gwdcc()[61],
    ",
  0x403ce4f8u64 => "
      GWCA_0.gwdcc()[62],
    ",
  0x403ce4fcu64 => "
      GWCA_0.gwdcc()[63],
    ",
  0x403ce800u64 => "
      GWCA_0.gwaarss(),
    ",
  0x403ce804u64 => "
      GWCA_0.gwaarsr0(),
    ",
  0x403ce808u64 => "
      GWCA_0.gwaarsr1(),
    ",
  0x403ce840u64 => "
      GWCA_0.gwidauas()[0],
    ",
  0x403ce844u64 => "
      GWCA_0.gwidauas()[1],
    ",
  0x403ce848u64 => "
      GWCA_0.gwidauas()[2],
    ",
  0x403ce84cu64 => "
      GWCA_0.gwidauas()[3],
    ",
  0x403ce880u64 => "
      GWCA_0.gwidasm()[0],
    ",
  0x403ce884u64 => "
      GWCA_0.gwidasm()[1],
    ",
  0x403ce888u64 => "
      GWCA_0.gwidasm()[2],
    ",
  0x403ce88cu64 => "
      GWCA_0.gwidasm()[3],
    ",
  0x403ce900u64 => "
      GWCA_0.gwidasam0()[0],
    ",
  0x403ce908u64 => "
      GWCA_0.gwidasam0()[1],
    ",
  0x403ce910u64 => "
      GWCA_0.gwidasam0()[2],
    ",
  0x403ce918u64 => "
      GWCA_0.gwidasam0()[3],
    ",
  0x403ce904u64 => "
      GWCA_0.gwidasam1()[0],
    ",
  0x403ce90cu64 => "
      GWCA_0.gwidasam1()[1],
    ",
  0x403ce914u64 => "
      GWCA_0.gwidasam1()[2],
    ",
  0x403ce91cu64 => "
      GWCA_0.gwidasam1()[3],
    ",
  0x403ce980u64 => "
      GWCA_0.gwidacam0()[0],
    ",
  0x403ce988u64 => "
      GWCA_0.gwidacam0()[1],
    ",
  0x403ce990u64 => "
      GWCA_0.gwidacam0()[2],
    ",
  0x403ce998u64 => "
      GWCA_0.gwidacam0()[3],
    ",
  0x403ce984u64 => "
      GWCA_0.gwidacam1()[0],
    ",
  0x403ce98cu64 => "
      GWCA_0.gwidacam1()[1],
    ",
  0x403ce994u64 => "
      GWCA_0.gwidacam1()[2],
    ",
  0x403ce99cu64 => "
      GWCA_0.gwidacam1()[3],
    ",
  0x403cea00u64 => "
      GWCA_0.gwgrlc(),
    ",
  0x403cea04u64 => "
      GWCA_0.gwgrlulc(),
    ",
  0x403cea80u64 => "
      GWCA_0.gwrlc()[0],
    ",
  0x403cea88u64 => "
      GWCA_0.gwrlc()[1],
    ",
  0x403cea90u64 => "
      GWCA_0.gwrlc()[2],
    ",
  0x403cea98u64 => "
      GWCA_0.gwrlc()[3],
    ",
  0x403ceaa0u64 => "
      GWCA_0.gwrlc()[4],
    ",
  0x403ceaa8u64 => "
      GWCA_0.gwrlc()[5],
    ",
  0x403ceab0u64 => "
      GWCA_0.gwrlc()[6],
    ",
  0x403ceab8u64 => "
      GWCA_0.gwrlc()[7],
    ",
  0x403cea84u64 => "
      GWCA_0.gwrlulc()[0],
    ",
  0x403cea8cu64 => "
      GWCA_0.gwrlulc()[1],
    ",
  0x403cea94u64 => "
      GWCA_0.gwrlulc()[2],
    ",
  0x403cea9cu64 => "
      GWCA_0.gwrlulc()[3],
    ",
  0x403ceaa4u64 => "
      GWCA_0.gwrlulc()[4],
    ",
  0x403ceaacu64 => "
      GWCA_0.gwrlulc()[5],
    ",
  0x403ceab4u64 => "
      GWCA_0.gwrlulc()[6],
    ",
  0x403ceabcu64 => "
      GWCA_0.gwrlulc()[7],
    ",
  0x403ceb80u64 => "
      GWCA_0.gwidpc(),
    ",
  0x403cf000u64 => "
      GWCA_0.gwrdcn(),
    ",
  0x403cf004u64 => "
      GWCA_0.gwtdcn(),
    ",
  0x403cf008u64 => "
      GWCA_0.gwtscn(),
    ",
  0x403cf00cu64 => "
      GWCA_0.gwtsovfecn(),
    ",
  0x403cf010u64 => "
      GWCA_0.gwusmfsecn(),
    ",
  0x403cf014u64 => "
      GWCA_0.gwtfecn(),
    ",
  0x403cf018u64 => "
      GWCA_0.gwseqecn(),
    ",
  0x403cf020u64 => "
      GWCA_0.gwtxdnecn(),
    ",
  0x403cf024u64 => "
      GWCA_0.gwfsecn(),
    ",
  0x403cf028u64 => "
      GWCA_0.gwtdfecn(),
    ",
  0x403cf02cu64 => "
      GWCA_0.gwtsdnecn(),
    ",
  0x403cf030u64 => "
      GWCA_0.gwdqoecn(),
    ",
  0x403cf034u64 => "
      GWCA_0.gwdqsecn(),
    ",
  0x403cf038u64 => "
      GWCA_0.gwdfecn(),
    ",
  0x403cf03cu64 => "
      GWCA_0.gwdsecn(),
    ",
  0x403cf040u64 => "
      GWCA_0.gwdszecn(),
    ",
  0x403cf044u64 => "
      GWCA_0.gwdctecn(),
    ",
  0x403cf048u64 => "
      GWCA_0.gwrxdnecn(),
    ",
  0x403cf100u64 => "
      GWCA_0.gwdis0(),
    ",
  0x403cf104u64 => "
      GWCA_0.gwdie0(),
    ",
  0x403cf108u64 => "
      GWCA_0.gwdid0(),
    ",
  0x403cf10cu64 => "
      GWCA_0.gwdids0(),
    ",
  0x403cf110u64 => "
      GWCA_0.gwdis1(),
    ",
  0x403cf114u64 => "
      GWCA_0.gwdie1(),
    ",
  0x403cf118u64 => "
      GWCA_0.gwdid1(),
    ",
  0x403cf11cu64 => "
      GWCA_0.gwdids1(),
    ",
  0x403cf180u64 => "
      GWCA_0.gwtsdis(),
    ",
  0x403cf184u64 => "
      GWCA_0.gwtsdie(),
    ",
  0x403cf188u64 => "
      GWCA_0.gwtsdid(),
    ",
  0x403cf190u64 => "
      GWCA_0.gweis0(),
    ",
  0x403cf194u64 => "
      GWCA_0.gweie0(),
    ",
  0x403cf198u64 => "
      GWCA_0.gweid0(),
    ",
  0x403cf1a0u64 => "
      GWCA_0.gweis1(),
    ",
  0x403cf1a4u64 => "
      GWCA_0.gweie1(),
    ",
  0x403cf1a8u64 => "
      GWCA_0.gweid1(),
    ",
  0x403cf200u64 => "
      GWCA_0.gweis20(),
    ",
  0x403cf204u64 => "
      GWCA_0.gweie20(),
    ",
  0x403cf208u64 => "
      GWCA_0.gweid20(),
    ",
  0x403cf210u64 => "
      GWCA_0.gweis21(),
    ",
  0x403cf214u64 => "
      GWCA_0.gweie21(),
    ",
  0x403cf218u64 => "
      GWCA_0.gweid21(),
    ",
  0x403cf280u64 => "
      GWCA_0.gweis3(),
    ",
  0x403cf284u64 => "
      GWCA_0.gweie3(),
    ",
  0x403cf288u64 => "
      GWCA_0.gweid3(),
    ",
  0x403cf290u64 => "
      GWCA_0.gweis4(),
    ",
  0x403cf294u64 => "
      GWCA_0.gweie4(),
    ",
  0x403cf298u64 => "
      GWCA_0.gweid4(),
    ",
  0x403cf2a0u64 => "
      GWCA_0.gweis5(),
    ",
  0x403cf2a4u64 => "
      GWCA_0.gweie5(),
    ",
  0x403cf2a8u64 => "
      GWCA_0.gweid5(),
    ",
  0x403da000u64 => "
      GWCA_0.gwidc()[0],
    ",
  0x403da004u64 => "
      GWCA_0.gwidc()[1],
    ",
  0x403da008u64 => "
      GWCA_0.gwidc()[2],
    ",
  0x403da00cu64 => "
      GWCA_0.gwidc()[3],
    ",
  0x403da010u64 => "
      GWCA_0.gwidc()[4],
    ",
  0x403da014u64 => "
      GWCA_0.gwidc()[5],
    ",
  0x403da018u64 => "
      GWCA_0.gwidc()[6],
    ",
  0x403da01cu64 => "
      GWCA_0.gwidc()[7],
    ",
  0x403da020u64 => "
      GWCA_0.gwidc()[8],
    ",
  0x403da024u64 => "
      GWCA_0.gwidc()[9],
    ",
  0x403da028u64 => "
      GWCA_0.gwidc()[10],
    ",
  0x403da02cu64 => "
      GWCA_0.gwidc()[11],
    ",
  0x403da030u64 => "
      GWCA_0.gwidc()[12],
    ",
  0x403da034u64 => "
      GWCA_0.gwidc()[13],
    ",
  0x403da038u64 => "
      GWCA_0.gwidc()[14],
    ",
  0x403da03cu64 => "
      GWCA_0.gwidc()[15],
    ",
  0x403da040u64 => "
      GWCA_0.gwidc()[16],
    ",
  0x403da044u64 => "
      GWCA_0.gwidc()[17],
    ",
  0x403da048u64 => "
      GWCA_0.gwidc()[18],
    ",
  0x403da04cu64 => "
      GWCA_0.gwidc()[19],
    ",
  0x403da050u64 => "
      GWCA_0.gwidc()[20],
    ",
  0x403da054u64 => "
      GWCA_0.gwidc()[21],
    ",
  0x403da058u64 => "
      GWCA_0.gwidc()[22],
    ",
  0x403da05cu64 => "
      GWCA_0.gwidc()[23],
    ",
  0x403da060u64 => "
      GWCA_0.gwidc()[24],
    ",
  0x403da064u64 => "
      GWCA_0.gwidc()[25],
    ",
  0x403da068u64 => "
      GWCA_0.gwidc()[26],
    ",
  0x403da06cu64 => "
      GWCA_0.gwidc()[27],
    ",
  0x403da070u64 => "
      GWCA_0.gwidc()[28],
    ",
  0x403da074u64 => "
      GWCA_0.gwidc()[29],
    ",
  0x403da078u64 => "
      GWCA_0.gwidc()[30],
    ",
  0x403da07cu64 => "
      GWCA_0.gwidc()[31],
    ",
  0x403da080u64 => "
      GWCA_0.gwidc()[32],
    ",
  0x403da084u64 => "
      GWCA_0.gwidc()[33],
    ",
  0x403da088u64 => "
      GWCA_0.gwidc()[34],
    ",
  0x403da08cu64 => "
      GWCA_0.gwidc()[35],
    ",
  0x403da090u64 => "
      GWCA_0.gwidc()[36],
    ",
  0x403da094u64 => "
      GWCA_0.gwidc()[37],
    ",
  0x403da098u64 => "
      GWCA_0.gwidc()[38],
    ",
  0x403da09cu64 => "
      GWCA_0.gwidc()[39],
    ",
  0x403da0a0u64 => "
      GWCA_0.gwidc()[40],
    ",
  0x403da0a4u64 => "
      GWCA_0.gwidc()[41],
    ",
  0x403da0a8u64 => "
      GWCA_0.gwidc()[42],
    ",
  0x403da0acu64 => "
      GWCA_0.gwidc()[43],
    ",
  0x403da0b0u64 => "
      GWCA_0.gwidc()[44],
    ",
  0x403da0b4u64 => "
      GWCA_0.gwidc()[45],
    ",
  0x403da0b8u64 => "
      GWCA_0.gwidc()[46],
    ",
  0x403da0bcu64 => "
      GWCA_0.gwidc()[47],
    ",
  0x403da0c0u64 => "
      GWCA_0.gwidc()[48],
    ",
  0x403da0c4u64 => "
      GWCA_0.gwidc()[49],
    ",
  0x403da0c8u64 => "
      GWCA_0.gwidc()[50],
    ",
  0x403da0ccu64 => "
      GWCA_0.gwidc()[51],
    ",
  0x403da0d0u64 => "
      GWCA_0.gwidc()[52],
    ",
  0x403da0d4u64 => "
      GWCA_0.gwidc()[53],
    ",
  0x403da0d8u64 => "
      GWCA_0.gwidc()[54],
    ",
  0x403da0dcu64 => "
      GWCA_0.gwidc()[55],
    ",
  0x403da0e0u64 => "
      GWCA_0.gwidc()[56],
    ",
  0x403da0e4u64 => "
      GWCA_0.gwidc()[57],
    ",
  0x403da0e8u64 => "
      GWCA_0.gwidc()[58],
    ",
  0x403da0ecu64 => "
      GWCA_0.gwidc()[59],
    ",
  0x403da0f0u64 => "
      GWCA_0.gwidc()[60],
    ",
  0x403da0f4u64 => "
      GWCA_0.gwidc()[61],
    ",
  0x403da0f8u64 => "
      GWCA_0.gwidc()[62],
    ",
  0x403da0fcu64 => "
      GWCA_0.gwidc()[63],
    ",
  0x403e0000u64 => "
      GPTP.ptpipv(),
    ",
  0x403e0010u64 => "
      GPTP.ptptmec(),
    ",
  0x403e0014u64 => "
      GPTP.ptptmdc(),
    ",
  0x403e0020u64 => "
      GPTP.ptptivc()[0],
    ",
  0x403e0060u64 => "
      GPTP.ptptivc()[1],
    ",
  0x403e0030u64 => "
      GPTP.ptptovcl()[0],
    ",
  0x403e0070u64 => "
      GPTP.ptptovcl()[1],
    ",
  0x403e0034u64 => "
      GPTP.ptptovcm()[0],
    ",
  0x403e0074u64 => "
      GPTP.ptptovcm()[1],
    ",
  0x403e0038u64 => "
      GPTP.ptptovcu()[0],
    ",
  0x403e0078u64 => "
      GPTP.ptptovcu()[1],
    ",
  0x403e0040u64 => "
      GPTP.ptpavtptml()[0],
    ",
  0x403e0080u64 => "
      GPTP.ptpavtptml()[1],
    ",
  0x403e0044u64 => "
      GPTP.ptpavtptmu()[0],
    ",
  0x403e0084u64 => "
      GPTP.ptpavtptmu()[1],
    ",
  0x403e0050u64 => "
      GPTP.ptpgptptml()[0],
    ",
  0x403e0090u64 => "
      GPTP.ptpgptptml()[1],
    ",
  0x403e0054u64 => "
      GPTP.ptpgptptmm()[0],
    ",
  0x403e0094u64 => "
      GPTP.ptpgptptmm()[1],
    ",
  0x403e0058u64 => "
      GPTP.ptpgptptmu()[0],
    ",
  0x403e0098u64 => "
      GPTP.ptpgptptmu()[1],
    ",
  0x403e0200u64 => "
      GPTP.ptpmccc()[0],
    ",
  0x403e0210u64 => "
      GPTP.ptpmccc()[1],
    ",
  0x403e0204u64 => "
      GPTP.ptpmccml()[0],
    ",
  0x403e0214u64 => "
      GPTP.ptpmccml()[1],
    ",
  0x403e0208u64 => "
      GPTP.ptpmccmm()[0],
    ",
  0x403e0218u64 => "
      GPTP.ptpmccmm()[1],
    ",
  0x403e020cu64 => "
      GPTP.ptpmccmu()[0],
    ",
  0x403e021cu64 => "
      GPTP.ptpmccmu()[1],
    ",
  0x403e0300u64 => "
      GPTP.ptpmcrc()[0],
    ",
  0x403e0310u64 => "
      GPTP.ptpmcrc()[1],
    ",
  0x403e0304u64 => "
      GPTP.ptpmcrtcl()[0],
    ",
  0x403e0314u64 => "
      GPTP.ptpmcrtcl()[1],
    ",
  0x403e0308u64 => "
      GPTP.ptpmcrtcm()[0],
    ",
  0x403e0318u64 => "
      GPTP.ptpmcrtcm()[1],
    ",
  0x403e030cu64 => "
      GPTP.ptpmcrtcu()[0],
    ",
  0x403e031cu64 => "
      GPTP.ptpmcrtcu()[1],
    ",
  0x403e0400u64 => "
      GPTP.ptpmcpc()[0],
    ",
  0x403e0404u64 => "
      GPTP.ptpmcpc()[1],
    ",
  0x403e0500u64 => "
      GPTP.ptpccc0()[0],
    ",
  0x403e0508u64 => "
      GPTP.ptpccc0()[1],
    ",
  0x403e0510u64 => "
      GPTP.ptpccc0()[2],
    ",
  0x403e0518u64 => "
      GPTP.ptpccc0()[3],
    ",
  0x403e0520u64 => "
      GPTP.ptpccc0()[4],
    ",
  0x403e0528u64 => "
      GPTP.ptpccc0()[5],
    ",
  0x403e0530u64 => "
      GPTP.ptpccc0()[6],
    ",
  0x403e0538u64 => "
      GPTP.ptpccc0()[7],
    ",
  0x403e0504u64 => "
      GPTP.ptpccc1()[0],
    ",
  0x403e050cu64 => "
      GPTP.ptpccc1()[1],
    ",
  0x403e0514u64 => "
      GPTP.ptpccc1()[2],
    ",
  0x403e051cu64 => "
      GPTP.ptpccc1()[3],
    ",
  0x403e0524u64 => "
      GPTP.ptpccc1()[4],
    ",
  0x403e052cu64 => "
      GPTP.ptpccc1()[5],
    ",
  0x403e0534u64 => "
      GPTP.ptpccc1()[6],
    ",
  0x403e053cu64 => "
      GPTP.ptpccc1()[7],
    ",
  0x403e0700u64 => "
      GPTP.ptpis0(),
    ",
  0x403e0704u64 => "
      GPTP.ptpie0(),
    ",
  0x403e0708u64 => "
      GPTP.ptpid0(),
    ",
  0x403e0710u64 => "
      GPTP.ptpis1(),
    ",
  0x403e0714u64 => "
      GPTP.ptpie1(),
    ",
  0x403e0718u64 => "
      GPTP.ptpid1(),
    ",
  0x403e1000u64 => "
      GPTP.potcfgr(),
    ",
  0x403e1004u64 => "
      GPTP.potcprl()[0],
      GPTP.potcprm()[0],
      GPTP.potcpru()[0],
      GPTP.potcr()[0],
      GPTP.potperl()[0],
      GPTP.potperm()[0],
      GPTP.potpwr()[0],
      GPTP.potstrl()[0],
      GPTP.potstrm()[0],
    ",
  0x403e1034u64 => "
      GPTP.potcprl()[1],
      GPTP.potcprm()[1],
      GPTP.potcpru()[1],
      GPTP.potcr()[1],
      GPTP.potperl()[1],
      GPTP.potperm()[1],
      GPTP.potpwr()[1],
      GPTP.potstrl()[1],
      GPTP.potstrm()[1],
    ",
  0x403e1064u64 => "
      GPTP.potcprl()[2],
      GPTP.potcprm()[2],
      GPTP.potcpru()[2],
      GPTP.potcr()[2],
      GPTP.potperl()[2],
      GPTP.potperm()[2],
      GPTP.potpwr()[2],
      GPTP.potstrl()[2],
      GPTP.potstrm()[2],
    ",
  0x403e1094u64 => "
      GPTP.potcprl()[3],
      GPTP.potcprm()[3],
      GPTP.potcpru()[3],
      GPTP.potcr()[3],
      GPTP.potperl()[3],
      GPTP.potperm()[3],
      GPTP.potpwr()[3],
      GPTP.potstrl()[3],
      GPTP.potstrm()[3],
    ",
  0x403e1008u64 => "
      GPTP.potperu()[0],
      GPTP.potstru()[0],
    ",
  0x403e1038u64 => "
      GPTP.potperu()[1],
      GPTP.potstru()[1],
    ",
  0x403e1068u64 => "
      GPTP.potperu()[2],
      GPTP.potstru()[2],
    ",
  0x403e1098u64 => "
      GPTP.potperu()[3],
      GPTP.potstru()[3],
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
      PORT_1.eidr(),
      PORT_1.pidr(),
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
  0x40400146u64 => "
      PORTA.eidr(),
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
  0x40400166u64 => "
      PORTB.eidr(),
    ",
  0x40400168u64 => "
      PORTB.pcntr3(),
      PORTB.posr(),
    ",
  0x4040016au64 => "
      PORTB.porr(),
    ",
  0x40400180u64 => "
      PORTC.pcntr1(),
      PORTC.pdr(),
    ",
  0x40400182u64 => "
      PORTC.podr(),
    ",
  0x40400184u64 => "
      PORTC.pcntr2(),
      PORTC.pidr(),
    ",
  0x40400186u64 => "
      PORTC.eidr(),
    ",
  0x40400188u64 => "
      PORTC.pcntr3(),
      PORTC.posr(),
    ",
  0x4040018au64 => "
      PORTC.porr(),
    ",
  0x404001a0u64 => "
      PORTD.pcntr1(),
      PORTD.pdr(),
    ",
  0x404001a2u64 => "
      PORTD.podr(),
    ",
  0x404001a4u64 => "
      PORTD.pcntr2(),
      PORTD.pidr(),
    ",
  0x404001a6u64 => "
      PORTD.eidr(),
    ",
  0x404001a8u64 => "
      PORTD.pcntr3(),
      PORTD.posr(),
    ",
  0x404001aau64 => "
      PORTD.porr(),
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
  0x40400860u64 => "
      PFS.p10pfs()[8],
      PFS.p10pfs_ha()[8],
      PFS.p10pfs_by()[8],
    ",
  0x40400864u64 => "
      PFS.p10pfs()[9],
      PFS.p10pfs_ha()[9],
      PFS.p10pfs_by()[9],
    ",
  0x40400868u64 => "
      PFS.p1pfs()[0],
      PFS.p1pfs_ha()[0],
      PFS.p1pfs_by()[0],
    ",
  0x4040086cu64 => "
      PFS.p1pfs()[1],
      PFS.p1pfs_ha()[1],
      PFS.p1pfs_by()[1],
    ",
  0x40400870u64 => "
      PFS.p1pfs()[2],
      PFS.p1pfs_ha()[2],
      PFS.p1pfs_by()[2],
    ",
  0x40400874u64 => "
      PFS.p1pfs()[3],
      PFS.p1pfs_ha()[3],
      PFS.p1pfs_by()[3],
    ",
  0x40400878u64 => "
      PFS.p1pfs()[4],
      PFS.p1pfs_ha()[4],
      PFS.p1pfs_by()[4],
    ",
  0x4040087cu64 => "
      PFS.p1pfs()[5],
      PFS.p1pfs_ha()[5],
      PFS.p1pfs_by()[5],
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
      PFS.p2pfs()[0],
      PFS.p2pfs_ha()[0],
      PFS.p2pfs_by()[0],
    ",
  0x404008d0u64 => "
      PFS.p30pfs()[4],
      PFS.p30pfs_ha()[4],
      PFS.p30pfs_by()[4],
      PFS.p2pfs()[1],
      PFS.p2pfs_ha()[1],
      PFS.p2pfs_by()[1],
    ",
  0x404008d4u64 => "
      PFS.p30pfs()[5],
      PFS.p30pfs_ha()[5],
      PFS.p30pfs_by()[5],
      PFS.p2pfs()[2],
      PFS.p2pfs_ha()[2],
      PFS.p2pfs_by()[2],
    ",
  0x404008d8u64 => "
      PFS.p30pfs()[6],
      PFS.p30pfs_ha()[6],
      PFS.p30pfs_by()[6],
      PFS.p2pfs()[3],
      PFS.p2pfs_ha()[3],
      PFS.p2pfs_by()[3],
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
      PFS.p814pfs(),
      PFS.p814pfs_ha(),
      PFS.p814pfs_by(),
    ",
  0x40400a3cu64 => "
      PFS.p815pfs(),
      PFS.p815pfs_ha(),
      PFS.p815pfs_by(),
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
  0x40400b00u64 => "
      PFS.pc0pfs()[0],
      PFS.pc0pfs_ha()[0],
      PFS.pc0pfs_by()[0],
    ",
  0x40400b04u64 => "
      PFS.pc0pfs()[1],
      PFS.pc0pfs_ha()[1],
      PFS.pc0pfs_by()[1],
    ",
  0x40400b08u64 => "
      PFS.pc0pfs()[2],
      PFS.pc0pfs_ha()[2],
      PFS.pc0pfs_by()[2],
    ",
  0x40400b0cu64 => "
      PFS.pc0pfs()[3],
      PFS.pc0pfs_ha()[3],
      PFS.pc0pfs_by()[3],
    ",
  0x40400b10u64 => "
      PFS.pc0pfs()[4],
      PFS.pc0pfs_ha()[4],
      PFS.pc0pfs_by()[4],
    ",
  0x40400b14u64 => "
      PFS.pc0pfs()[5],
      PFS.pc0pfs_ha()[5],
      PFS.pc0pfs_by()[5],
    ",
  0x40400b18u64 => "
      PFS.pc0pfs()[6],
      PFS.pc0pfs_ha()[6],
      PFS.pc0pfs_by()[6],
    ",
  0x40400b1cu64 => "
      PFS.pc0pfs()[7],
      PFS.pc0pfs_ha()[7],
      PFS.pc0pfs_by()[7],
    ",
  0x40400b20u64 => "
      PFS.pc0pfs()[8],
      PFS.pc0pfs_ha()[8],
      PFS.pc0pfs_by()[8],
    ",
  0x40400b24u64 => "
      PFS.pc0pfs()[9],
      PFS.pc0pfs_ha()[9],
      PFS.pc0pfs_by()[9],
    ",
  0x40400b28u64 => "
      PFS.pcpfs()[0],
      PFS.pcpfs_ha()[0],
      PFS.pcpfs_by()[0],
    ",
  0x40400b2cu64 => "
      PFS.pcpfs()[1],
      PFS.pcpfs_ha()[1],
      PFS.pcpfs_by()[1],
    ",
  0x40400b30u64 => "
      PFS.pcpfs()[2],
      PFS.pcpfs_ha()[2],
      PFS.pcpfs_by()[2],
    ",
  0x40400b34u64 => "
      PFS.pcpfs()[3],
      PFS.pcpfs_ha()[3],
      PFS.pcpfs_by()[3],
    ",
  0x40400b38u64 => "
      PFS.pcpfs()[4],
      PFS.pcpfs_ha()[4],
      PFS.pcpfs_by()[4],
    ",
  0x40400b3cu64 => "
      PFS.pcpfs()[5],
      PFS.pcpfs_ha()[5],
      PFS.pcpfs_by()[5],
    ",
  0x40400b40u64 => "
      PFS.pd0pfs()[0],
      PFS.pd0pfs_ha()[0],
      PFS.pd0pfs_by()[0],
    ",
  0x40400b44u64 => "
      PFS.pd0pfs()[1],
      PFS.pd0pfs_ha()[1],
      PFS.pd0pfs_by()[1],
    ",
  0x40400b48u64 => "
      PFS.pd0pfs()[2],
      PFS.pd0pfs_ha()[2],
      PFS.pd0pfs_by()[2],
    ",
  0x40400b4cu64 => "
      PFS.pd0pfs()[3],
      PFS.pd0pfs_ha()[3],
      PFS.pd0pfs_by()[3],
    ",
  0x40400b50u64 => "
      PFS.pd0pfs()[4],
      PFS.pd0pfs_ha()[4],
      PFS.pd0pfs_by()[4],
    ",
  0x40400b54u64 => "
      PFS.pd0pfs()[5],
      PFS.pd0pfs_ha()[5],
      PFS.pd0pfs_by()[5],
    ",
  0x40400b58u64 => "
      PFS.pd0pfs()[6],
      PFS.pd0pfs_ha()[6],
      PFS.pd0pfs_by()[6],
    ",
  0x40400b5cu64 => "
      PFS.pd0pfs()[7],
      PFS.pd0pfs_ha()[7],
      PFS.pd0pfs_by()[7],
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
  0x40444000u64 => "
      DRW.control(),
      DRW.status(),
    ",
  0x40444004u64 => "
      DRW.control2(),
      DRW.hwrevision(),
    ",
  0x40444010u64 => "
      DRW.lstart()[0],
    ",
  0x40444014u64 => "
      DRW.lstart()[1],
    ",
  0x40444018u64 => "
      DRW.lstart()[2],
    ",
  0x4044401cu64 => "
      DRW.lstart()[3],
    ",
  0x40444020u64 => "
      DRW.lstart()[4],
    ",
  0x40444024u64 => "
      DRW.lstart()[5],
    ",
  0x40444028u64 => "
      DRW.lxadd()[0],
    ",
  0x4044402cu64 => "
      DRW.lxadd()[1],
    ",
  0x40444030u64 => "
      DRW.lxadd()[2],
    ",
  0x40444034u64 => "
      DRW.lxadd()[3],
    ",
  0x40444038u64 => "
      DRW.lxadd()[4],
    ",
  0x4044403cu64 => "
      DRW.lxadd()[5],
    ",
  0x40444040u64 => "
      DRW.lyadd()[0],
    ",
  0x40444044u64 => "
      DRW.lyadd()[1],
    ",
  0x40444048u64 => "
      DRW.lyadd()[2],
    ",
  0x4044404cu64 => "
      DRW.lyadd()[3],
    ",
  0x40444050u64 => "
      DRW.lyadd()[4],
    ",
  0x40444054u64 => "
      DRW.lyadd()[5],
    ",
  0x40444058u64 => "
      DRW.lband()[0],
    ",
  0x4044405cu64 => "
      DRW.lband()[1],
    ",
  0x40444064u64 => "
      DRW.color1(),
    ",
  0x40444068u64 => "
      DRW.color2(),
    ",
  0x40444074u64 => "
      DRW.pattern(),
    ",
  0x40444078u64 => "
      DRW.size(),
    ",
  0x4044407cu64 => "
      DRW.pitch(),
    ",
  0x40444080u64 => "
      DRW.origin(),
    ",
  0x40444090u64 => "
      DRW.lustart(),
    ",
  0x40444094u64 => "
      DRW.luxadd(),
    ",
  0x40444098u64 => "
      DRW.luyadd(),
    ",
  0x4044409cu64 => "
      DRW.lvstarti(),
    ",
  0x404440a0u64 => "
      DRW.lvstartf(),
    ",
  0x404440a4u64 => "
      DRW.lvxaddi(),
    ",
  0x404440a8u64 => "
      DRW.lvyaddi(),
    ",
  0x404440acu64 => "
      DRW.lvyxaddf(),
    ",
  0x404440b4u64 => "
      DRW.texpitch(),
    ",
  0x404440b8u64 => "
      DRW.texmask(),
    ",
  0x404440bcu64 => "
      DRW.texorigin(),
    ",
  0x404440c0u64 => "
      DRW.irqctl(),
    ",
  0x404440c4u64 => "
      DRW.cachectl(),
    ",
  0x404440c8u64 => "
      DRW.dliststart(),
    ",
  0x404440ccu64 => "
      DRW.perfcount1(),
      DRW.perfcount2(),
    ",
  0x404440d4u64 => "
      DRW.perftrigger(),
    ",
  0x404440dcu64 => "
      DRW.texcladdr(),
    ",
  0x404440e0u64 => "
      DRW.texcldata(),
    ",
  0x404440e4u64 => "
      DRW.texcloffset(),
    ",
  0x404440e8u64 => "
      DRW.colkey(),
    ",
  0x40444100u64 => "
      DRW.dbwer(),
    ",
  0x50000000u64 => "
      RMPU_NS.mmpuoad(),
    ",
  0x50000004u64 => "
      RMPU_NS.mmpuoadpt(),
    ",
  0x50000100u64 => "
      RMPU_NS.mmpuendmac()[0],
    ",
  0x50000300u64 => "
      RMPU_NS.mmpuendmac()[1],
    ",
  0x50000104u64 => "
      RMPU_NS.mmpuenptdmac()[0],
    ",
  0x50000304u64 => "
      RMPU_NS.mmpuenptdmac()[1],
    ",
  0x5000010cu64 => "
      RMPU_NS.mmpurptdmac()[0],
    ",
  0x5000030cu64 => "
      RMPU_NS.mmpurptdmac()[1],
    ",
  0x50000200u64 => "
      RMPU_NS.mmpuacdmac0()[0],
    ",
  0x50000210u64 => "
      RMPU_NS.mmpuacdmac0()[1],
    ",
  0x50000220u64 => "
      RMPU_NS.mmpuacdmac0()[2],
    ",
  0x50000230u64 => "
      RMPU_NS.mmpuacdmac0()[3],
    ",
  0x50000240u64 => "
      RMPU_NS.mmpuacdmac0()[4],
    ",
  0x50000250u64 => "
      RMPU_NS.mmpuacdmac0()[5],
    ",
  0x50000260u64 => "
      RMPU_NS.mmpuacdmac0()[6],
    ",
  0x50000270u64 => "
      RMPU_NS.mmpuacdmac0()[7],
    ",
  0x50000204u64 => "
      RMPU_NS.mmpusdmac0()[0],
    ",
  0x50000214u64 => "
      RMPU_NS.mmpusdmac0()[1],
    ",
  0x50000224u64 => "
      RMPU_NS.mmpusdmac0()[2],
    ",
  0x50000234u64 => "
      RMPU_NS.mmpusdmac0()[3],
    ",
  0x50000244u64 => "
      RMPU_NS.mmpusdmac0()[4],
    ",
  0x50000254u64 => "
      RMPU_NS.mmpusdmac0()[5],
    ",
  0x50000264u64 => "
      RMPU_NS.mmpusdmac0()[6],
    ",
  0x50000274u64 => "
      RMPU_NS.mmpusdmac0()[7],
    ",
  0x50000208u64 => "
      RMPU_NS.mmpuedmac0()[0],
    ",
  0x50000218u64 => "
      RMPU_NS.mmpuedmac0()[1],
    ",
  0x50000228u64 => "
      RMPU_NS.mmpuedmac0()[2],
    ",
  0x50000238u64 => "
      RMPU_NS.mmpuedmac0()[3],
    ",
  0x50000248u64 => "
      RMPU_NS.mmpuedmac0()[4],
    ",
  0x50000258u64 => "
      RMPU_NS.mmpuedmac0()[5],
    ",
  0x50000268u64 => "
      RMPU_NS.mmpuedmac0()[6],
    ",
  0x50000278u64 => "
      RMPU_NS.mmpuedmac0()[7],
    ",
  0x50000400u64 => "
      RMPU_NS.mmpuacdmac1()[0],
    ",
  0x50000410u64 => "
      RMPU_NS.mmpuacdmac1()[1],
    ",
  0x50000420u64 => "
      RMPU_NS.mmpuacdmac1()[2],
    ",
  0x50000430u64 => "
      RMPU_NS.mmpuacdmac1()[3],
    ",
  0x50000440u64 => "
      RMPU_NS.mmpuacdmac1()[4],
    ",
  0x50000450u64 => "
      RMPU_NS.mmpuacdmac1()[5],
    ",
  0x50000460u64 => "
      RMPU_NS.mmpuacdmac1()[6],
    ",
  0x50000470u64 => "
      RMPU_NS.mmpuacdmac1()[7],
    ",
  0x50000404u64 => "
      RMPU_NS.mmpusdmac1()[0],
    ",
  0x50000414u64 => "
      RMPU_NS.mmpusdmac1()[1],
    ",
  0x50000424u64 => "
      RMPU_NS.mmpusdmac1()[2],
    ",
  0x50000434u64 => "
      RMPU_NS.mmpusdmac1()[3],
    ",
  0x50000444u64 => "
      RMPU_NS.mmpusdmac1()[4],
    ",
  0x50000454u64 => "
      RMPU_NS.mmpusdmac1()[5],
    ",
  0x50000464u64 => "
      RMPU_NS.mmpusdmac1()[6],
    ",
  0x50000474u64 => "
      RMPU_NS.mmpusdmac1()[7],
    ",
  0x50000408u64 => "
      RMPU_NS.mmpuedmac1()[0],
    ",
  0x50000418u64 => "
      RMPU_NS.mmpuedmac1()[1],
    ",
  0x50000428u64 => "
      RMPU_NS.mmpuedmac1()[2],
    ",
  0x50000438u64 => "
      RMPU_NS.mmpuedmac1()[3],
    ",
  0x50000448u64 => "
      RMPU_NS.mmpuedmac1()[4],
    ",
  0x50000458u64 => "
      RMPU_NS.mmpuedmac1()[5],
    ",
  0x50000468u64 => "
      RMPU_NS.mmpuedmac1()[6],
    ",
  0x50000478u64 => "
      RMPU_NS.mmpuedmac1()[7],
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
  0x50000640u64 => "
      RMPU_NS.mmpuacedmac()[4],
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
  0x50000644u64 => "
      RMPU_NS.mmpusedmac()[4],
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
  0x50000648u64 => "
      RMPU_NS.mmpueedmac()[4],
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
  0x50000b00u64 => "
      RMPU_NS.mmpuenmipid(),
    ",
  0x50000b04u64 => "
      RMPU_NS.mmpuenptmipid(),
    ",
  0x50000b08u64 => "
      RMPU_NS.mmpurptmipid(),
    ",
  0x50000c00u64 => "
      RMPU_NS.mmpuacmipid(),
    ",
  0x50000c04u64 => "
      RMPU_NS.mmpusmipid(),
    ",
  0x50000c08u64 => "
      RMPU_NS.mmpuemipid(),
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
  0x50000f00u64 => "
      RMPU_NS.mmpuenmipic(),
    ",
  0x50000f04u64 => "
      RMPU_NS.mmpuenptmipic(),
    ",
  0x50000f08u64 => "
      RMPU_NS.mmpurptmipic(),
    ",
  0x50001000u64 => "
      RMPU_NS.mmpuacmipic()[0],
    ",
  0x50001010u64 => "
      RMPU_NS.mmpuacmipic()[1],
    ",
  0x50001020u64 => "
      RMPU_NS.mmpuacmipic()[2],
    ",
  0x50001004u64 => "
      RMPU_NS.mmpusmipic()[0],
    ",
  0x50001014u64 => "
      RMPU_NS.mmpusmipic()[1],
    ",
  0x50001024u64 => "
      RMPU_NS.mmpusmipic()[2],
    ",
  0x50001008u64 => "
      RMPU_NS.mmpuemipic()[0],
    ",
  0x50001018u64 => "
      RMPU_NS.mmpuemipic()[1],
    ",
  0x50001028u64 => "
      RMPU_NS.mmpuemipic()[2],
    ",
  0x50001100u64 => "
      RMPU_NS.mmpuennpu(),
    ",
  0x50001104u64 => "
      RMPU_NS.mmpuenptnpu(),
    ",
  0x50001108u64 => "
      RMPU_NS.mmpurptnpu(),
    ",
  0x50001200u64 => "
      RMPU_NS.mmpuacnpu()[0],
    ",
  0x50001210u64 => "
      RMPU_NS.mmpuacnpu()[1],
    ",
  0x50001220u64 => "
      RMPU_NS.mmpuacnpu()[2],
    ",
  0x50001230u64 => "
      RMPU_NS.mmpuacnpu()[3],
    ",
  0x50001240u64 => "
      RMPU_NS.mmpuacnpu()[4],
    ",
  0x50001204u64 => "
      RMPU_NS.mmpusnpu()[0],
    ",
  0x50001214u64 => "
      RMPU_NS.mmpusnpu()[1],
    ",
  0x50001224u64 => "
      RMPU_NS.mmpusnpu()[2],
    ",
  0x50001234u64 => "
      RMPU_NS.mmpusnpu()[3],
    ",
  0x50001244u64 => "
      RMPU_NS.mmpusnpu()[4],
    ",
  0x50001208u64 => "
      RMPU_NS.mmpuenpu()[0],
    ",
  0x50001218u64 => "
      RMPU_NS.mmpuenpu()[1],
    ",
  0x50001228u64 => "
      RMPU_NS.mmpuenpu()[2],
    ",
  0x50001238u64 => "
      RMPU_NS.mmpuenpu()[3],
    ",
  0x50001248u64 => "
      RMPU_NS.mmpuenpu()[4],
    ",
  0x50002004u64 => "
      SRAM_NS.sramprcr_ns(),
    ",
  0x50002008u64 => "
      SRAM_NS.sramwtsc(),
    ",
  0x50002010u64 => "
      SRAM_NS.sramcr()[0],
    ",
  0x50002014u64 => "
      SRAM_NS.sramcr()[1],
    ",
  0x50002018u64 => "
      SRAM_NS.sramcr()[2],
    ",
  0x5000201cu64 => "
      SRAM_NS.sramcr()[3],
    ",
  0x50002030u64 => "
      SRAM_NS.srameccrgn0(),
    ",
  0x50002034u64 => "
      SRAM_NS.srameccrgn1(),
    ",
  0x50002038u64 => "
      SRAM_NS.srameccrgn2(),
    ",
  0x5000203cu64 => "
      SRAM_NS.srameccrgn3(),
    ",
  0x50002040u64 => "
      SRAM_NS.sramesr(),
    ",
  0x50002048u64 => "
      SRAM_NS.sramesclr(),
    ",
  0x50002050u64 => "
      SRAM_NS.sramear0()[0],
    ",
  0x50002060u64 => "
      SRAM_NS.sramear0()[1],
    ",
  0x50002070u64 => "
      SRAM_NS.sramear0()[2],
    ",
  0x50002080u64 => "
      SRAM_NS.sramear0()[3],
    ",
  0x50002054u64 => "
      SRAM_NS.sramear1()[0],
    ",
  0x50002064u64 => "
      SRAM_NS.sramear1()[1],
    ",
  0x50002074u64 => "
      SRAM_NS.sramear1()[2],
    ",
  0x50002084u64 => "
      SRAM_NS.sramear1()[3],
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
      BUS_NS.busmabtgraphbi(),
    ",
  0x50004200u64 => "
      BUS_NS.bussabt1mrc0bi(),
    ",
  0x50004208u64 => "
      BUS_NS.bussabt0mre0bi(),
      BUS_NS.bussabt0s0bi(),
      BUS_NS.bussabt0s1bi(),
      BUS_NS.bussabt0s2bi(),
    ",
  0x50004238u64 => "
      BUS_NS.bussabt0ecbi(),
      BUS_NS.bussabt0ospi0bi(),
      BUS_NS.bussabt0ospi1bi(),
      BUS_NS.bussabt0s3bi(),
    ",
  0x50004268u64 => "
      BUS_NS.bussabt0cpu0sahbi(),
      BUS_NS.bussabt0cpu1tcmbi(),
      BUS_NS.bussabt0pabi(),
      BUS_NS.bussabt0pbbi(),
    ",
  0x50004288u64 => "
      BUS_NS.bussabt0pibi(),
      BUS_NS.bussabt0psbi(),
    ",
  0x50004a00u64 => "
      BUS_NS.buserrstatcpu0(),
    ",
  0x50004a04u64 => "
      BUS_NS.buserrclrcpu0(),
    ",
  0x50004a08u64 => "
      BUS_NS.busirqencpu0(),
    ",
  0x50004a10u64 => "
      BUS_NS.buserrstatcpu1(),
    ",
  0x50004a14u64 => "
      BUS_NS.buserrclrcpu1(),
    ",
  0x50004a18u64 => "
      BUS_NS.busirqencpu1(),
    ",
  0x50004a20u64 => "
      BUS_NS.buserrstatdmac0(),
    ",
  0x50004a24u64 => "
      BUS_NS.buserrclrdmac0(),
    ",
  0x50004a28u64 => "
      BUS_NS.busirqendmac0(),
    ",
  0x50004a30u64 => "
      BUS_NS.buserrstatdmac1(),
    ",
  0x50004a34u64 => "
      BUS_NS.buserrclrdmac1(),
    ",
  0x50004a38u64 => "
      BUS_NS.busirqendmac1(),
    ",
  0x50004a40u64 => "
      BUS_NS.buserrstatnpu(),
    ",
  0x50004a44u64 => "
      BUS_NS.buserrclrnpu(),
    ",
  0x50004a48u64 => "
      BUS_NS.busirqennpu(),
    ",
  0x50004a50u64 => "
      BUS_NS.buserrstatedmac(),
    ",
  0x50004a54u64 => "
      BUS_NS.buserrclredmac(),
    ",
  0x50004a58u64 => "
      BUS_NS.busirqenedmac(),
    ",
  0x50004a60u64 => "
      BUS_NS.buserrstatglcdc(),
    ",
  0x50004a64u64 => "
      BUS_NS.buserrclrglcdc(),
    ",
  0x50004a68u64 => "
      BUS_NS.busirqenglcdc(),
    ",
  0x50004a70u64 => "
      BUS_NS.buserrstattdrw(),
    ",
  0x50004a74u64 => "
      BUS_NS.buserrclrtdrw(),
    ",
  0x50004a78u64 => "
      BUS_NS.busirqentdrw(),
    ",
  0x50004a80u64 => "
      BUS_NS.buserrstatmipi0(),
    ",
  0x50004a84u64 => "
      BUS_NS.buserrclrmipi0(),
    ",
  0x50004a88u64 => "
      BUS_NS.busirqenmipi0(),
    ",
  0x50004a90u64 => "
      BUS_NS.buserrstatmipi1(),
    ",
  0x50004a94u64 => "
      BUS_NS.buserrclrmipi1(),
    ",
  0x50004a98u64 => "
      BUS_NS.busirqenmipi1(),
    ",
  0x50004aa0u64 => "
      BUS_NS.buserrstatceu(),
    ",
  0x50004aa4u64 => "
      BUS_NS.buserrclrceu(),
    ",
  0x50004aa8u64 => "
      BUS_NS.busirqenceu(),
    ",
  0x50004b00u64 => "
      BUS_NS.mbwerrstat(),
    ",
  0x50004b08u64 => "
      BUS_NS.mbwerrclr(),
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
      ICU_COMMON_NS.irqcr()[16],
      ICU_COMMON_NS.nmicr(),
    ",
  0x50006011u64 => "
      ICU_COMMON_NS.irqcr()[17],
    ",
  0x50006012u64 => "
      ICU_COMMON_NS.irqcr()[18],
    ",
  0x50006013u64 => "
      ICU_COMMON_NS.irqcr()[19],
    ",
  0x50006014u64 => "
      ICU_COMMON_NS.irqcr()[20],
    ",
  0x50006015u64 => "
      ICU_COMMON_NS.irqcr()[21],
    ",
  0x50006016u64 => "
      ICU_COMMON_NS.irqcr()[22],
    ",
  0x50006017u64 => "
      ICU_COMMON_NS.irqcr()[23],
    ",
  0x50006018u64 => "
      ICU_COMMON_NS.irqcr()[24],
    ",
  0x50006019u64 => "
      ICU_COMMON_NS.irqcr()[25],
    ",
  0x5000601au64 => "
      ICU_COMMON_NS.irqcr()[26],
    ",
  0x5000601bu64 => "
      ICU_COMMON_NS.irqcr()[27],
    ",
  0x5000601cu64 => "
      ICU_COMMON_NS.irqcr()[28],
    ",
  0x5000601du64 => "
      ICU_COMMON_NS.irqcr()[29],
    ",
  0x5000601eu64 => "
      ICU_COMMON_NS.irqcr()[30],
    ",
  0x5000601fu64 => "
      ICU_COMMON_NS.irqcr()[31],
    ",
  0x50006040u64 => "
      ICU_COMMON_NS.intselr()[0],
    ",
  0x50006044u64 => "
      ICU_COMMON_NS.intselr()[1],
    ",
  0x50006048u64 => "
      ICU_COMMON_NS.intselr()[2],
    ",
  0x5000604cu64 => "
      ICU_COMMON_NS.intselr()[3],
    ",
  0x50006050u64 => "
      ICU_COMMON_NS.intselr()[4],
    ",
  0x50006054u64 => "
      ICU_COMMON_NS.intselr()[5],
    ",
  0x50006058u64 => "
      ICU_COMMON_NS.intselr()[6],
    ",
  0x5000605cu64 => "
      ICU_COMMON_NS.intselr()[7],
    ",
  0x50006060u64 => "
      ICU_COMMON_NS.intselr()[8],
    ",
  0x50006064u64 => "
      ICU_COMMON_NS.intselr()[9],
    ",
  0x50006068u64 => "
      ICU_COMMON_NS.intselr()[10],
    ",
  0x5000606cu64 => "
      ICU_COMMON_NS.intselr()[11],
    ",
  0x50006070u64 => "
      ICU_COMMON_NS.intselr()[12],
    ",
  0x50006074u64 => "
      ICU_COMMON_NS.intselr()[13],
    ",
  0x50006078u64 => "
      ICU_COMMON_NS.intselr()[14],
    ",
  0x5000607cu64 => "
      ICU_COMMON_NS.intselr()[15],
    ",
  0x50006080u64 => "
      ICU_COMMON_NS.intselr()[16],
    ",
  0x50006084u64 => "
      ICU_COMMON_NS.intselr()[17],
    ",
  0x50006088u64 => "
      ICU_COMMON_NS.intselr()[18],
    ",
  0x5000608cu64 => "
      ICU_COMMON_NS.intselr()[19],
    ",
  0x50006090u64 => "
      ICU_COMMON_NS.intselr()[20],
    ",
  0x50006094u64 => "
      ICU_COMMON_NS.intselr()[21],
    ",
  0x50006098u64 => "
      ICU_COMMON_NS.intselr()[22],
    ",
  0x5000609cu64 => "
      ICU_COMMON_NS.intselr()[23],
    ",
  0x500060a0u64 => "
      ICU_COMMON_NS.intselr()[24],
    ",
  0x500060a4u64 => "
      ICU_COMMON_NS.intselr()[25],
    ",
  0x500060a8u64 => "
      ICU_COMMON_NS.intselr()[26],
    ",
  0x500060acu64 => "
      ICU_COMMON_NS.intselr()[27],
    ",
  0x500060b0u64 => "
      ICU_COMMON_NS.intselr()[28],
    ",
  0x500060b4u64 => "
      ICU_COMMON_NS.intselr()[29],
    ",
  0x500060b8u64 => "
      ICU_COMMON_NS.intselr()[30],
    ",
  0x500060bcu64 => "
      ICU_COMMON_NS.intselr()[31],
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
  0x5000807cu64 => "
      CPSCU_NS.icusarj(),
    ",
  0x50008080u64 => "
      CPSCU_NS.icusark(),
    ",
  0x50008084u64 => "
      CPSCU_NS.icusarl(),
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
  0x5000840cu64 => "
      CPSCU_NS.sramsabar()[3],
    ",
  0x50008500u64 => "
      CPSCU_NS.cachesar(),
    ",
  0x50008510u64 => "
      CPSCU_NS.sramesar(),
    ",
  0x50008600u64 => "
      CPSCU_NS.tevtrcr(),
    ",
  0x50008610u64 => "
      CPSCU_NS.ipcsar(),
    ",
  0x50008614u64 => "
      CPSCU_NS.ipcpar(),
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
  0x5000a810u64 => "
      DMA_0_NS.dmctl(),
    ",
  0x5000a840u64 => "
      DMA_0_NS.dmechr(),
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
  0x5000ac18u64 => "
      DTC_0_NS.dtcdisp(),
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
  0x5000c214u64 => "
      ICU_NS.dslpwupirqen0(),
    ",
  0x5000c218u64 => "
      ICU_NS.dslpwupirqen1(),
    ",
  0x5000c21cu64 => "
      ICU_NS.dslpwupirqen2(),
    ",
  0x5000c280u64 => "
      ICU_NS.delsrm(),
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
      CPU_CTRL_NS.cpulckupcr()[0],
    ",
  0x5000f034u64 => "
      CPU_CTRL_NS.cpulckupcr()[1],
    ",
  0x5000f040u64 => "
      CPU_CTRL_NS.cpuinitvtor()[0],
    ",
  0x5000f044u64 => "
      CPU_CTRL_NS.cpuinitvtor()[1],
    ",
  0x5000f500u64 => "
      CPU_CTRL_NS.cpulockcrns()[0],
    ",
  0x5000f504u64 => "
      CPU_CTRL_NS.cpulockcrns()[1],
    ",
  0x5000f840u64 => "
      CPU_CTRL_NS.cpucrpt()[0],
    ",
  0x5000f844u64 => "
      CPU_CTRL_NS.cpucrpt()[1],
    ",
  0x5001c000u64 => "
      CACHE_NS.ccactl(),
    ",
  0x5001c004u64 => "
      CACHE_NS.ccafct(),
    ",
  0x5001c00cu64 => "
      CACHE_NS.ccawta(),
    ",
  0x5001c010u64 => "
      CACHE_NS.ccaedst(),
    ",
  0x5001c014u64 => "
      CACHE_NS.ccataa(),
    ",
  0x5001c018u64 => "
      CACHE_NS.ccatad_data(),
      CACHE_NS.ccatad_ecc(),
      CACHE_NS.ccatad_lru(),
      CACHE_NS.ccatad_tag(),
      CACHE_NS.ccatad_tagecc(),
    ",
  0x5001c040u64 => "
      CACHE_NS.scactl(),
    ",
  0x5001c044u64 => "
      CACHE_NS.scafct(),
    ",
  0x5001c04cu64 => "
      CACHE_NS.scawta(),
    ",
  0x5001c050u64 => "
      CACHE_NS.scaedst(),
    ",
  0x5001c054u64 => "
      CACHE_NS.scataa(),
    ",
  0x5001c058u64 => "
      CACHE_NS.scatad_data(),
      CACHE_NS.scatad_ecc(),
      CACHE_NS.scatad_lru(),
      CACHE_NS.scatad_tag(),
    ",
  0x5001c200u64 => "
      CACHE_NS.capoad(),
    ",
  0x5001c204u64 => "
      CACHE_NS.caprcr(),
    ",
  0x5001c140u64 => "
      FCACHE_NS.fsar(),
    ",
  0x5001c804u64 => "
      TCM_NS.tcmprcr_ns(),
    ",
  0x5001c810u64 => "
      TCM_NS.tcmcrc(),
      TCM_NS.tcmcrs(),
    ",
  0x5001c840u64 => "
      TCM_NS.tcmesr(),
    ",
  0x5001c848u64 => "
      TCM_NS.tcmesclr(),
    ",
  0x5001c850u64 => "
      TCM_NS.tcmearc0(),
      TCM_NS.tcmearc1(),
      TCM_NS.tcmears0(),
      TCM_NS.tcmears1(),
    ",
  0x5001e00cu64 => "
      SYSC_NS.sbycr(),
    ",
  0x5001e014u64 => "
      SYSC_NS.vscr(),
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
  0x5001e05au64 => "
      SYSC_NS.adcckdivcr(),
    ",
  0x5001e05bu64 => "
      SYSC_NS.adcckcr(),
    ",
  0x5001e05cu64 => "
      SYSC_NS.gptckdivcr(),
    ",
  0x5001e05du64 => "
      SYSC_NS.gptckcr(),
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
  0x5001e06fu64 => "
      SYSC_NS.usb60ckdivcr(),
    ",
  0x5001e070u64 => "
      SYSC_NS.i3cckdivcr(),
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
  0x5001e077u64 => "
      SYSC_NS.usb60ckcr(),
    ",
  0x5001e078u64 => "
      SYSC_NS.i3cckcr(),
    ",
  0x5001e07cu64 => "
      SYSC_NS.moscscr(),
    ",
  0x5001e07du64 => "
      SYSC_NS.hocoscr(),
    ",
  0x5001e084u64 => "
      SYSC_NS.mocoscr(),
    ",
  0x5001e0a0u64 => "
      SYSC_NS.opccr(),
    ",
  0x5001e0a2u64 => "
      SYSC_NS.moscwtcr(),
    ",
  0x5001e0acu64 => "
      SYSC_NS.pllccr(),
    ",
  0x5001e0c0u64 => "
      SYSC_NS.rstsr1(),
    ",
  0x5001e0c8u64 => "
      SYSC_NS.pll2ccr(),
    ",
  0x5001e0ccu64 => "
      SYSC_NS.syraccr(),
    ",
  0x5001e0d4u64 => "
      SYSC_NS.bckadivcr(),
    ",
  0x5001e0d5u64 => "
      SYSC_NS.eswckdivcr(),
    ",
  0x5001e0d6u64 => "
      SYSC_NS.eswpckdivcr(),
    ",
  0x5001e0d8u64 => "
      SYSC_NS.ethpckdivcr(),
    ",
  0x5001e0dau64 => "
      SYSC_NS.bckacr(),
    ",
  0x5001e0dbu64 => "
      SYSC_NS.eswckcr(),
    ",
  0x5001e0dcu64 => "
      SYSC_NS.eswpckcr(),
    ",
  0x5001e0deu64 => "
      SYSC_NS.ethpckcr(),
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
  0x5001e100u64 => "
      SYSC_NS.cpudscr(),
    ",
  0x5001e104u64 => "
      SYSC_NS.pgscr(),
    ",
  0x5001e110u64 => "
      SYSC_NS.pdctrgd(),
    ",
  0x5001e114u64 => "
      SYSC_NS.pdctrnpu(),
    ",
  0x5001e118u64 => "
      SYSC_NS.pdctreswm(),
    ",
  0x5001e140u64 => "
      SYSC_NS.pdramscr0(),
    ",
  0x5001e142u64 => "
      SYSC_NS.pdramscr1(),
    ",
  0x5001e210u64 => "
      SYSC_NS.psstcr()[0],
    ",
  0x5001e212u64 => "
      SYSC_NS.psstcr()[1],
    ",
  0x5001e214u64 => "
      SYSC_NS.psstcr()[2],
    ",
  0x5001e216u64 => "
      SYSC_NS.psstcr()[3],
    ",
  0x5001e218u64 => "
      SYSC_NS.psstcr()[4],
    ",
  0x5001e21au64 => "
      SYSC_NS.psstcr()[5],
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
  0x5001e3e8u64 => "
      SYSC_NS.dpfsar1(),
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
  0x5001ea34u64 => "
      SYSC_NS.dpsiegr3(),
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
  0x5001ea48u64 => "
      SYSC_NS.rstsr3(),
    ",
  0x5001ea50u64 => "
      SYSC_NS.momcr(),
    ",
  0x5001ea54u64 => "
      SYSC_NS.fwepror(),
    ",
  0x5001ea64u64 => "
      SYSC_NS.pvdcmpcr()[0],
    ",
  0x5001ea68u64 => "
      SYSC_NS.pvdcmpcr()[1],
    ",
  0x5001ea7cu64 => "
      SYSC_NS.pvdcr0()[0],
    ",
  0x5001ea80u64 => "
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
  0x5001ea9cu64 => "
      SYSC_NS.svscr(),
    ",
  0x5001eab0u64 => "
      SYSC_NS.lvocr(),
    ",
  0x5001eab4u64 => "
      SYSC_NS.mwmcr(),
    ",
  0x5001ead0u64 => "
      SYSC_NS.syrstmsk0(),
    ",
  0x5001ead4u64 => "
      SYSC_NS.syrstmsk1(),
    ",
  0x5001ead8u64 => "
      SYSC_NS.syrstmsk2(),
    ",
  0x5001eadcu64 => "
      SYSC_NS.temprcr(),
    ",
  0x5001eae0u64 => "
      SYSC_NS.temprlr(),
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
  0x5001eb2cu64 => "
      SYSC_NS.pvdfcr()[0],
    ",
  0x5001eb30u64 => "
      SYSC_NS.pvdfcr()[1],
    ",
  0x5001eb34u64 => "
      SYSC_NS.pvdlr(),
    ",
  0x5001eb40u64 => "
      SYSC_NS.dpsier4(),
    ",
  0x5001eb44u64 => "
      SYSC_NS.dpsier5(),
    ",
  0x5001eb48u64 => "
      SYSC_NS.dpsifr4(),
    ",
  0x5001eb4cu64 => "
      SYSC_NS.dpsifr5(),
    ",
  0x5001eb50u64 => "
      SYSC_NS.dpsiegr4(),
    ",
  0x5001ec00u64 => "
      SYSC_NS.sosccr(),
    ",
  0x5001ec01u64 => "
      SYSC_NS.somcr(),
    ",
  0x5001ec04u64 => "
      SYSC_NS.sostdcr(),
    ",
  0x5001ec05u64 => "
      SYSC_NS.sostdsr(),
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
  0x5001ec50u64 => "
      SYSC_NS.vbtncwcr(),
    ",
  0x5001ec54u64 => "
      SYSC_NS.vbtadcr3(),
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
  0x50020000u64 => "
      IPC_NS.ipcsem()[0],
    ",
  0x50020004u64 => "
      IPC_NS.ipcsem()[1],
    ",
  0x50020008u64 => "
      IPC_NS.ipcsem()[2],
    ",
  0x5002000cu64 => "
      IPC_NS.ipcsem()[3],
    ",
  0x50020010u64 => "
      IPC_NS.ipcsem()[4],
    ",
  0x50020014u64 => "
      IPC_NS.ipcsem()[5],
    ",
  0x50020018u64 => "
      IPC_NS.ipcsem()[6],
    ",
  0x5002001cu64 => "
      IPC_NS.ipcsem()[7],
    ",
  0x50020020u64 => "
      IPC_NS.ipcsem()[8],
    ",
  0x50020024u64 => "
      IPC_NS.ipcsem()[9],
    ",
  0x50020028u64 => "
      IPC_NS.ipcsem()[10],
    ",
  0x5002002cu64 => "
      IPC_NS.ipcsem()[11],
    ",
  0x50020030u64 => "
      IPC_NS.ipcsem()[12],
    ",
  0x50020034u64 => "
      IPC_NS.ipcsem()[13],
    ",
  0x50020038u64 => "
      IPC_NS.ipcsem()[14],
    ",
  0x5002003cu64 => "
      IPC_NS.ipcsem()[15],
    ",
  0x50020080u64 => "
      IPC_NS.ipc0nmista(),
    ",
  0x50020084u64 => "
      IPC_NS.ipc0nmiset(),
    ",
  0x50020088u64 => "
      IPC_NS.ipc0nmiclr(),
    ",
  0x50020090u64 => "
      IPC_NS.ipc1nmista(),
    ",
  0x50020094u64 => "
      IPC_NS.ipc1nmiset(),
    ",
  0x50020098u64 => "
      IPC_NS.ipc1nmiclr(),
    ",
  0x500200c0u64 => "
      IPC_NS.ipc0sta0(),
    ",
  0x500200c4u64 => "
      IPC_NS.ipc0iset0(),
    ",
  0x500200c8u64 => "
      IPC_NS.ipc0txd0(),
    ",
  0x500200ccu64 => "
      IPC_NS.ipc0rxd0(),
    ",
  0x500200d0u64 => "
      IPC_NS.ipc0clr0(),
    ",
  0x500200e0u64 => "
      IPC_NS.ipc0sta1(),
    ",
  0x500200e4u64 => "
      IPC_NS.ipc0iset1(),
    ",
  0x500200e8u64 => "
      IPC_NS.ipc0txd1(),
    ",
  0x500200ecu64 => "
      IPC_NS.ipc0rxd1(),
    ",
  0x500200f0u64 => "
      IPC_NS.ipc0clr1(),
    ",
  0x50020100u64 => "
      IPC_NS.ipc1sta0(),
    ",
  0x50020104u64 => "
      IPC_NS.ipc1iset0(),
    ",
  0x50020108u64 => "
      IPC_NS.ipc1txd0(),
    ",
  0x5002010cu64 => "
      IPC_NS.ipc1rxd0(),
    ",
  0x50020110u64 => "
      IPC_NS.ipc1clr0(),
    ",
  0x50020120u64 => "
      IPC_NS.ipc1sta1(),
    ",
  0x50020124u64 => "
      IPC_NS.ipc1iset1(),
    ",
  0x50020128u64 => "
      IPC_NS.ipc1txd1(),
    ",
  0x5002012cu64 => "
      IPC_NS.ipc1rxd1(),
    ",
  0x50020130u64 => "
      IPC_NS.ipc1clr1(),
    ",
  0x5013c000u64 => "
      MRAM_NS.mrcpfb(),
    ",
  0x5013c004u64 => "
      MRAM_NS.mrcfreq(),
    ",
  0x5013c008u64 => "
      MRAM_NS.mrefreq(),
    ",
  0x5013c010u64 => "
      MRAM_NS.mrcdecc(),
    ",
  0x5013c014u64 => "
      MRAM_NS.mrcraeint(),
    ",
  0x5013c018u64 => "
      MRAM_NS.mrcraes(),
    ",
  0x5013c01cu64 => "
      MRAM_NS.mrcrtea(),
    ",
  0x5013c020u64 => "
      MRAM_NS.mrcrdea(),
    ",
  0x5013c034u64 => "
      MRAM_NS.mreraeint(),
    ",
  0x5013c038u64 => "
      MRAM_NS.mreraes(),
    ",
  0x5013c03cu64 => "
      MRAM_NS.mrertea(),
    ",
  0x5013c040u64 => "
      MRAM_NS.mrerdea(),
    ",
  0x5013c100u64 => "
      MRAM_NS.msar(),
    ",
  0x5013c400u64 => "
      MRAM_NS.mrezs(),
    ",
  0x5013c404u64 => "
      MRAM_NS.mrezc(),
    ",
  0x5013e010u64 => "
      MRAM_NS.mastat(),
    ",
  0x5013e014u64 => "
      MRAM_NS.mpaeint(),
    ",
  0x5013e018u64 => "
      MRAM_NS.mrdyie(),
    ",
  0x5013e030u64 => "
      MRAM_NS.msaddr(),
    ",
  0x5013e048u64 => "
      MRAM_NS.mcntselr(),
    ",
  0x5013e04cu64 => "
      MRAM_NS.mcntdtr()[0],
    ",
  0x5013e050u64 => "
      MRAM_NS.mcntdtr()[1],
    ",
  0x5013e060u64 => "
      MRAM_NS.mctrcntr(),
    ",
  0x5013e064u64 => "
      MRAM_NS.mctrlsr(),
    ",
  0x5013e06cu64 => "
      MRAM_NS.mctrstatr(),
    ",
  0x5013e080u64 => "
      MRAM_NS.mstatr(),
    ",
  0x5013e084u64 => "
      MRAM_NS.mentryr(),
    ",
  0x5013e08cu64 => "
      MRAM_NS.msuinitr(),
    ",
  0x5013e0a0u64 => "
      MRAM_NS.mcmdr(),
    ",
  0x5013e0dcu64 => "
      MRAM_NS.msuasmon(),
    ",
  0x5013e0e8u64 => "
      MRAM_NS.msuacr(),
    ",
  0x5013e800u64 => "
      MRAM_NS.mrpsc(),
    ",
  0x5013f000u64 => "
      MRAM_NS.mrcpc0(),
    ",
  0x5013f004u64 => "
      MRAM_NS.mrcpc1(),
    ",
  0x5013f008u64 => "
      MRAM_NS.mrcbprot0(),
    ",
  0x5013f00cu64 => "
      MRAM_NS.mrcbprot1(),
    ",
  0x5013f010u64 => "
      MRAM_NS.mrcps(),
    ",
  0x5013f014u64 => "
      MRAM_NS.mrcpaeint(),
    ",
  0x5013f018u64 => "
      MRAM_NS.mrcpea(),
    ",
  0x5013f030u64 => "
      MRAM_NS.mrcflr(),
    ",
  0x5013f804u64 => "
      MRAM_NS.mrceecc(),
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
  0x5020100cu64 => "
      ELC_NS.elsegr()[2],
    ",
  0x50201010u64 => "
      ELC_NS.elsegr()[3],
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
  0x50201068u64 => "
      ELC_NS.elsr()[18],
    ",
  0x5020106cu64 => "
      ELC_NS.elsr()[19],
    ",
  0x50201070u64 => "
      ELC_NS.elsr()[20],
    ",
  0x50201074u64 => "
      ELC_NS.elsr()[21],
    ",
  0x50201078u64 => "
      ELC_NS.elsr()[22],
    ",
  0x5020107cu64 => "
      ELC_NS.elsr()[23],
    ",
  0x50201080u64 => "
      ELC_NS.elsr()[24],
    ",
  0x50201084u64 => "
      ELC_NS.elsr()[25],
    ",
  0x50201088u64 => "
      ELC_NS.elsr()[26],
    ",
  0x5020108cu64 => "
      ELC_NS.elsr()[27],
    ",
  0x50201090u64 => "
      ELC_NS.elsr()[28],
    ",
  0x50201094u64 => "
      ELC_NS.elsr()[29],
    ",
  0x50201098u64 => "
      ELC_NS.elsr()[30],
    ",
  0x5020109cu64 => "
      ELC_NS.elsr()[31],
    ",
  0x502010a0u64 => "
      ELC_NS.elsr()[32],
    ",
  0x502010a4u64 => "
      ELC_NS.elsr()[33],
    ",
  0x502010a8u64 => "
      ELC_NS.elsr()[34],
    ",
  0x502010acu64 => "
      ELC_NS.elsr()[35],
    ",
  0x502010b0u64 => "
      ELC_NS.elsr()[36],
    ",
  0x502010b4u64 => "
      ELC_NS.elsr()[37],
    ",
  0x502010b8u64 => "
      ELC_NS.elsr()[38],
    ",
  0x502010bcu64 => "
      ELC_NS.elsr()[39],
    ",
  0x502010c0u64 => "
      ELC_NS.elsr()[40],
    ",
  0x502010c4u64 => "
      ELC_NS.elsr()[41],
    ",
  0x502010c8u64 => "
      ELC_NS.elsr()[42],
    ",
  0x502010ccu64 => "
      ELC_NS.elsr()[43],
    ",
  0x502010d0u64 => "
      ELC_NS.elsr()[44],
    ",
  0x502010d4u64 => "
      ELC_NS.elsr()[45],
    ",
  0x502010d8u64 => "
      ELC_NS.elsr()[46],
    ",
  0x502010dcu64 => "
      ELC_NS.elsr()[47],
    ",
  0x502010e0u64 => "
      ELC_NS.elsr()[48],
    ",
  0x502010e4u64 => "
      ELC_NS.elsr()[49],
    ",
  0x502010e8u64 => "
      ELC_NS.elsr()[50],
    ",
  0x502010ecu64 => "
      ELC_NS.elsr()[51],
    ",
  0x502010f0u64 => "
      ELC_NS.elsr()[52],
    ",
  0x50201100u64 => "
      ELC_NS.elcsara(),
    ",
  0x50201104u64 => "
      ELC_NS.elcsarb(),
    ",
  0x50201108u64 => "
      ELC_NS.elcsarc(),
    ",
  0x50201110u64 => "
      ELC_NS.elcpara(),
    ",
  0x50201114u64 => "
      ELC_NS.elcparb(),
    ",
  0x50201118u64 => "
      ELC_NS.elcparc(),
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
      PSCU_NS.cmsamon(),
    ",
  0x50204038u64 => "
      PSCU_NS.dlmmon(),
    ",
  0x5020403cu64 => "
      PSCU_NS.sfsamon(),
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
  0x50233000u64 => "
      DAC_120_NS.dadr(),
    ",
  0x50233004u64 => "
      DAC_120_NS.dacr0(),
    ",
  0x50233008u64 => "
      DAC_120_NS.dacr1(),
    ",
  0x5023300cu64 => "
      DAC_120_NS.dacr2(),
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
  0x50252000u64 => "
      SDHI_0_NS.sd_cmd(),
    ",
  0x50252008u64 => "
      SDHI_0_NS.sd_arg(),
    ",
  0x5025200cu64 => "
      SDHI_0_NS.sd_arg1(),
    ",
  0x50252010u64 => "
      SDHI_0_NS.sd_stop(),
    ",
  0x50252014u64 => "
      SDHI_0_NS.sd_seccnt(),
    ",
  0x50252018u64 => "
      SDHI_0_NS.sd_rsp10(),
    ",
  0x5025201cu64 => "
      SDHI_0_NS.sd_rsp1(),
    ",
  0x50252020u64 => "
      SDHI_0_NS.sd_rsp32(),
    ",
  0x50252024u64 => "
      SDHI_0_NS.sd_rsp3(),
    ",
  0x50252028u64 => "
      SDHI_0_NS.sd_rsp54(),
    ",
  0x5025202cu64 => "
      SDHI_0_NS.sd_rsp5(),
    ",
  0x50252030u64 => "
      SDHI_0_NS.sd_rsp76(),
    ",
  0x50252034u64 => "
      SDHI_0_NS.sd_rsp7(),
    ",
  0x50252038u64 => "
      SDHI_0_NS.sd_info1(),
    ",
  0x5025203cu64 => "
      SDHI_0_NS.sd_info2(),
    ",
  0x50252040u64 => "
      SDHI_0_NS.sd_info1_mask(),
    ",
  0x50252044u64 => "
      SDHI_0_NS.sd_info2_mask(),
    ",
  0x50252048u64 => "
      SDHI_0_NS.sd_clk_ctrl(),
    ",
  0x5025204cu64 => "
      SDHI_0_NS.sd_size(),
    ",
  0x50252050u64 => "
      SDHI_0_NS.sd_option(),
    ",
  0x50252058u64 => "
      SDHI_0_NS.sd_err_sts1(),
    ",
  0x5025205cu64 => "
      SDHI_0_NS.sd_err_sts2(),
    ",
  0x50252060u64 => "
      SDHI_0_NS.sd_buf0(),
    ",
  0x50252068u64 => "
      SDHI_0_NS.sdio_mode(),
    ",
  0x5025206cu64 => "
      SDHI_0_NS.sdio_info1(),
    ",
  0x50252070u64 => "
      SDHI_0_NS.sdio_info1_mask(),
    ",
  0x502521b0u64 => "
      SDHI_0_NS.sd_dmaen(),
    ",
  0x502521c0u64 => "
      SDHI_0_NS.soft_rst(),
    ",
  0x502521ccu64 => "
      SDHI_0_NS.sdif_mode(),
    ",
  0x502521e0u64 => "
      SDHI_0_NS.ext_swap(),
    ",
  0x50256000u64 => "
      PDMIF_NS.pdcstrtr(),
    ",
  0x50256004u64 => "
      PDMIF_NS.pdcstptr(),
    ",
  0x50256008u64 => "
      PDMIF_NS.pdcchgtr(),
    ",
  0x5025600cu64 => "
      PDMIF_NS.pdcicr(),
    ",
  0x50256010u64 => "
      PDMIF_NS.pdcsr(),
    ",
  0x50256014u64 => "
      PDMIF_NS.pdcscr(),
    ",
  0x50256020u64 => "
      PDMIF_NS.pdcsdcr(),
    ",
  0x50256024u64 => "
      PDMIF_NS.pdcdrcr(),
    ",
  0x50256028u64 => "
      PDMIF_NS.pdcdcr(),
    ",
  0x50256080u64 => "
      PDMIF_NS.pdvr(),
    ",
  0x50256100u64 => "
      PDMIF_NS.pdstrtrch()[0],
    ",
  0x50256200u64 => "
      PDMIF_NS.pdstrtrch()[1],
    ",
  0x50256300u64 => "
      PDMIF_NS.pdstrtrch()[2],
    ",
  0x50256104u64 => "
      PDMIF_NS.pdstptrch()[0],
    ",
  0x50256204u64 => "
      PDMIF_NS.pdstptrch()[1],
    ",
  0x50256304u64 => "
      PDMIF_NS.pdstptrch()[2],
    ",
  0x50256108u64 => "
      PDMIF_NS.pdchgtrch()[0],
    ",
  0x50256208u64 => "
      PDMIF_NS.pdchgtrch()[1],
    ",
  0x50256308u64 => "
      PDMIF_NS.pdchgtrch()[2],
    ",
  0x5025610cu64 => "
      PDMIF_NS.pdicrch()[0],
    ",
  0x5025620cu64 => "
      PDMIF_NS.pdicrch()[1],
    ",
  0x5025630cu64 => "
      PDMIF_NS.pdicrch()[2],
    ",
  0x50256110u64 => "
      PDMIF_NS.pdsdcrch()[0],
    ",
  0x50256210u64 => "
      PDMIF_NS.pdsdcrch()[1],
    ",
  0x50256310u64 => "
      PDMIF_NS.pdsdcrch()[2],
    ",
  0x50256114u64 => "
      PDMIF_NS.pdsrch()[0],
    ",
  0x50256214u64 => "
      PDMIF_NS.pdsrch()[1],
    ",
  0x50256314u64 => "
      PDMIF_NS.pdsrch()[2],
    ",
  0x50256118u64 => "
      PDMIF_NS.pdscrch()[0],
    ",
  0x50256218u64 => "
      PDMIF_NS.pdscrch()[1],
    ",
  0x50256318u64 => "
      PDMIF_NS.pdscrch()[2],
    ",
  0x50256120u64 => "
      PDMIF_NS.pdmdsrch()[0],
    ",
  0x50256220u64 => "
      PDMIF_NS.pdmdsrch()[1],
    ",
  0x50256320u64 => "
      PDMIF_NS.pdmdsrch()[2],
    ",
  0x50256124u64 => "
      PDMIF_NS.pdsfcrch()[0],
    ",
  0x50256224u64 => "
      PDMIF_NS.pdsfcrch()[1],
    ",
  0x50256324u64 => "
      PDMIF_NS.pdsfcrch()[2],
    ",
  0x50256128u64 => "
      PDMIF_NS.pdhfcs0rch()[0],
    ",
  0x50256228u64 => "
      PDMIF_NS.pdhfcs0rch()[1],
    ",
  0x50256328u64 => "
      PDMIF_NS.pdhfcs0rch()[2],
    ",
  0x5025612cu64 => "
      PDMIF_NS.pdhfck1rch()[0],
    ",
  0x5025622cu64 => "
      PDMIF_NS.pdhfck1rch()[1],
    ",
  0x5025632cu64 => "
      PDMIF_NS.pdhfck1rch()[2],
    ",
  0x50256130u64 => "
      PDMIF_NS.pdhfch0rch()[0],
    ",
  0x50256230u64 => "
      PDMIF_NS.pdhfch0rch()[1],
    ",
  0x50256330u64 => "
      PDMIF_NS.pdhfch0rch()[2],
    ",
  0x50256134u64 => "
      PDMIF_NS.pdhfch1rch()[0],
    ",
  0x50256234u64 => "
      PDMIF_NS.pdhfch1rch()[1],
    ",
  0x50256334u64 => "
      PDMIF_NS.pdhfch1rch()[2],
    ",
  0x50256138u64 => "
      PDMIF_NS.pdcfch00rch()[0],
    ",
  0x50256238u64 => "
      PDMIF_NS.pdcfch00rch()[1],
    ",
  0x50256338u64 => "
      PDMIF_NS.pdcfch00rch()[2],
    ",
  0x5025613cu64 => "
      PDMIF_NS.pdcfch01rch()[0],
    ",
  0x5025623cu64 => "
      PDMIF_NS.pdcfch01rch()[1],
    ",
  0x5025633cu64 => "
      PDMIF_NS.pdcfch01rch()[2],
    ",
  0x50256140u64 => "
      PDMIF_NS.pdcfch02rch()[0],
    ",
  0x50256240u64 => "
      PDMIF_NS.pdcfch02rch()[1],
    ",
  0x50256340u64 => "
      PDMIF_NS.pdcfch02rch()[2],
    ",
  0x50256144u64 => "
      PDMIF_NS.pdcfch03rch()[0],
    ",
  0x50256244u64 => "
      PDMIF_NS.pdcfch03rch()[1],
    ",
  0x50256344u64 => "
      PDMIF_NS.pdcfch03rch()[2],
    ",
  0x50256148u64 => "
      PDMIF_NS.pdcfch04rch()[0],
    ",
  0x50256248u64 => "
      PDMIF_NS.pdcfch04rch()[1],
    ",
  0x50256348u64 => "
      PDMIF_NS.pdcfch04rch()[2],
    ",
  0x5025614cu64 => "
      PDMIF_NS.pdcfch05rch()[0],
    ",
  0x5025624cu64 => "
      PDMIF_NS.pdcfch05rch()[1],
    ",
  0x5025634cu64 => "
      PDMIF_NS.pdcfch05rch()[2],
    ",
  0x50256150u64 => "
      PDMIF_NS.pdcfch06rchn(),
    ",
  0x50256154u64 => "
      PDMIF_NS.pdcfch07rch()[0],
    ",
  0x50256254u64 => "
      PDMIF_NS.pdcfch07rch()[1],
    ",
  0x50256354u64 => "
      PDMIF_NS.pdcfch07rch()[2],
    ",
  0x50256158u64 => "
      PDMIF_NS.pdcfch08rch()[0],
    ",
  0x50256258u64 => "
      PDMIF_NS.pdcfch08rch()[1],
    ",
  0x50256358u64 => "
      PDMIF_NS.pdcfch08rch()[2],
    ",
  0x5025615cu64 => "
      PDMIF_NS.pdcfch09rch()[0],
    ",
  0x5025625cu64 => "
      PDMIF_NS.pdcfch09rch()[1],
    ",
  0x5025635cu64 => "
      PDMIF_NS.pdcfch09rch()[2],
    ",
  0x50256160u64 => "
      PDMIF_NS.pdcfch10rch()[0],
    ",
  0x50256260u64 => "
      PDMIF_NS.pdcfch10rch()[1],
    ",
  0x50256360u64 => "
      PDMIF_NS.pdcfch10rch()[2],
    ",
  0x50256164u64 => "
      PDMIF_NS.pdlfch010rch()[0],
    ",
  0x50256264u64 => "
      PDMIF_NS.pdlfch010rch()[1],
    ",
  0x50256364u64 => "
      PDMIF_NS.pdlfch010rch()[2],
    ",
  0x50256168u64 => "
      PDMIF_NS.pdlfch100rch()[0],
    ",
  0x50256268u64 => "
      PDMIF_NS.pdlfch100rch()[1],
    ",
  0x50256368u64 => "
      PDMIF_NS.pdlfch100rch()[2],
    ",
  0x5025616cu64 => "
      PDMIF_NS.pdlfch101rch()[0],
    ",
  0x5025626cu64 => "
      PDMIF_NS.pdlfch101rch()[1],
    ",
  0x5025636cu64 => "
      PDMIF_NS.pdlfch101rch()[2],
    ",
  0x50256170u64 => "
      PDMIF_NS.pdlfch102rch()[0],
    ",
  0x50256270u64 => "
      PDMIF_NS.pdlfch102rch()[1],
    ",
  0x50256370u64 => "
      PDMIF_NS.pdlfch102rch()[2],
    ",
  0x50256174u64 => "
      PDMIF_NS.pdlfch103rch()[0],
    ",
  0x50256274u64 => "
      PDMIF_NS.pdlfch103rch()[1],
    ",
  0x50256374u64 => "
      PDMIF_NS.pdlfch103rch()[2],
    ",
  0x50256178u64 => "
      PDMIF_NS.pdlfch104rch()[0],
    ",
  0x50256278u64 => "
      PDMIF_NS.pdlfch104rch()[1],
    ",
  0x50256378u64 => "
      PDMIF_NS.pdlfch104rch()[2],
    ",
  0x5025617cu64 => "
      PDMIF_NS.pdlfch105rch()[0],
    ",
  0x5025627cu64 => "
      PDMIF_NS.pdlfch105rch()[1],
    ",
  0x5025637cu64 => "
      PDMIF_NS.pdlfch105rch()[2],
    ",
  0x50256180u64 => "
      PDMIF_NS.pdlfch106rch()[0],
    ",
  0x50256280u64 => "
      PDMIF_NS.pdlfch106rch()[1],
    ",
  0x50256380u64 => "
      PDMIF_NS.pdlfch106rch()[2],
    ",
  0x50256184u64 => "
      PDMIF_NS.pdlfch107rch()[0],
    ",
  0x50256284u64 => "
      PDMIF_NS.pdlfch107rch()[1],
    ",
  0x50256384u64 => "
      PDMIF_NS.pdlfch107rch()[2],
    ",
  0x50256188u64 => "
      PDMIF_NS.pdlfch108rch()[0],
    ",
  0x50256288u64 => "
      PDMIF_NS.pdlfch108rch()[1],
    ",
  0x50256388u64 => "
      PDMIF_NS.pdlfch108rch()[2],
    ",
  0x5025618cu64 => "
      PDMIF_NS.pdlfch109rch()[0],
    ",
  0x5025628cu64 => "
      PDMIF_NS.pdlfch109rch()[1],
    ",
  0x5025638cu64 => "
      PDMIF_NS.pdlfch109rch()[2],
    ",
  0x50256190u64 => "
      PDMIF_NS.pdlfch110rch()[0],
    ",
  0x50256290u64 => "
      PDMIF_NS.pdlfch110rch()[1],
    ",
  0x50256390u64 => "
      PDMIF_NS.pdlfch110rch()[2],
    ",
  0x50256194u64 => "
      PDMIF_NS.pdlfch111rch()[0],
    ",
  0x50256294u64 => "
      PDMIF_NS.pdlfch111rch()[1],
    ",
  0x50256394u64 => "
      PDMIF_NS.pdlfch111rch()[2],
    ",
  0x50256198u64 => "
      PDMIF_NS.pdlfch112rch()[0],
    ",
  0x50256298u64 => "
      PDMIF_NS.pdlfch112rch()[1],
    ",
  0x50256398u64 => "
      PDMIF_NS.pdlfch112rch()[2],
    ",
  0x5025619cu64 => "
      PDMIF_NS.pdlfch113rch()[0],
    ",
  0x5025629cu64 => "
      PDMIF_NS.pdlfch113rch()[1],
    ",
  0x5025639cu64 => "
      PDMIF_NS.pdlfch113rch()[2],
    ",
  0x502561a0u64 => "
      PDMIF_NS.pdlfch114rch()[0],
    ",
  0x502562a0u64 => "
      PDMIF_NS.pdlfch114rch()[1],
    ",
  0x502563a0u64 => "
      PDMIF_NS.pdlfch114rch()[2],
    ",
  0x502561a4u64 => "
      PDMIF_NS.pdlfch115rch()[0],
    ",
  0x502562a4u64 => "
      PDMIF_NS.pdlfch115rch()[1],
    ",
  0x502563a4u64 => "
      PDMIF_NS.pdlfch115rch()[2],
    ",
  0x502561a8u64 => "
      PDMIF_NS.pdlfch116rch()[0],
    ",
  0x502562a8u64 => "
      PDMIF_NS.pdlfch116rch()[1],
    ",
  0x502563a8u64 => "
      PDMIF_NS.pdlfch116rch()[2],
    ",
  0x502561acu64 => "
      PDMIF_NS.pdlfch117rch()[0],
    ",
  0x502562acu64 => "
      PDMIF_NS.pdlfch117rch()[1],
    ",
  0x502563acu64 => "
      PDMIF_NS.pdlfch117rch()[2],
    ",
  0x502561b0u64 => "
      PDMIF_NS.pdlfch118rch()[0],
    ",
  0x502562b0u64 => "
      PDMIF_NS.pdlfch118rch()[1],
    ",
  0x502563b0u64 => "
      PDMIF_NS.pdlfch118rch()[2],
    ",
  0x502561b4u64 => "
      PDMIF_NS.pdlfch119rch()[0],
    ",
  0x502562b4u64 => "
      PDMIF_NS.pdlfch119rch()[1],
    ",
  0x502563b4u64 => "
      PDMIF_NS.pdlfch119rch()[2],
    ",
  0x502561b8u64 => "
      PDMIF_NS.pdsdltrch()[0],
    ",
  0x502562b8u64 => "
      PDMIF_NS.pdsdltrch()[1],
    ",
  0x502563b8u64 => "
      PDMIF_NS.pdsdltrch()[2],
    ",
  0x502561bcu64 => "
      PDMIF_NS.pdsdutrch()[0],
    ",
  0x502562bcu64 => "
      PDMIF_NS.pdsdutrch()[1],
    ",
  0x502563bcu64 => "
      PDMIF_NS.pdsdutrch()[2],
    ",
  0x502561c0u64 => "
      PDMIF_NS.pddbcrch()[0],
    ",
  0x502562c0u64 => "
      PDMIF_NS.pddbcrch()[1],
    ",
  0x502563c0u64 => "
      PDMIF_NS.pddbcrch()[2],
    ",
  0x502561c4u64 => "
      PDMIF_NS.pdsctsrch()[0],
    ",
  0x502562c4u64 => "
      PDMIF_NS.pdsctsrch()[1],
    ",
  0x502563c4u64 => "
      PDMIF_NS.pdsctsrch()[2],
    ",
  0x502561c8u64 => "
      PDMIF_NS.pdovltrch()[0],
    ",
  0x502562c8u64 => "
      PDMIF_NS.pdovltrch()[1],
    ",
  0x502563c8u64 => "
      PDMIF_NS.pdovltrch()[2],
    ",
  0x502561ccu64 => "
      PDMIF_NS.pdovutrch()[0],
    ",
  0x502562ccu64 => "
      PDMIF_NS.pdovutrch()[1],
    ",
  0x502563ccu64 => "
      PDMIF_NS.pdovutrch()[2],
    ",
  0x502561e0u64 => "
      PDMIF_NS.pddrcrch()[0],
    ",
  0x502562e0u64 => "
      PDMIF_NS.pddrcrch()[1],
    ",
  0x502563e0u64 => "
      PDMIF_NS.pddrcrch()[2],
    ",
  0x502561e4u64 => "
      PDMIF_NS.pddcrch()[0],
    ",
  0x502562e4u64 => "
      PDMIF_NS.pddcrch()[1],
    ",
  0x502563e4u64 => "
      PDMIF_NS.pddcrch()[2],
    ",
  0x502561e8u64 => "
      PDMIF_NS.pddrrch()[0],
    ",
  0x502562e8u64 => "
      PDMIF_NS.pddrrch()[1],
    ",
  0x502563e8u64 => "
      PDMIF_NS.pddrrch()[2],
    ",
  0x502561ecu64 => "
      PDMIF_NS.pddsrch()[0],
    ",
  0x502562ecu64 => "
      PDMIF_NS.pddsrch()[1],
    ",
  0x502563ecu64 => "
      PDMIF_NS.pddsrch()[2],
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
  0x50268800u64 => "
      DOTF_0_NS.convareast(),
    ",
  0x50268804u64 => "
      DOTF_0_NS.convaread(),
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
  0x50322044u64 => "
      GPT_320_NS.gtitc(),
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
  0x5032206cu64 => "
      GPT_320_NS.gtpdbr(),
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
  0x50322090u64 => "
      GPT_320_NS.gtdvd(),
    ",
  0x50322094u64 => "
      GPT_320_NS.gtdbu(),
    ",
  0x50322098u64 => "
      GPT_320_NS.gtdbd(),
    ",
  0x5032209cu64 => "
      GPT_320_NS.gtsos(),
    ",
  0x503220a0u64 => "
      GPT_320_NS.gtsotr(),
    ",
  0x503220a4u64 => "
      GPT_320_NS.gtadsmr(),
    ",
  0x503220a8u64 => "
      GPT_320_NS.gteitc(),
    ",
  0x503220acu64 => "
      GPT_320_NS.gteitli1(),
    ",
  0x503220b0u64 => "
      GPT_320_NS.gteitli2(),
    ",
  0x503220b4u64 => "
      GPT_320_NS.gteitlb(),
    ",
  0x503220b8u64 => "
      GPT_320_NS.gticlf(),
    ",
  0x503220bcu64 => "
      GPT_320_NS.gtpc(),
    ",
  0x503220c0u64 => "
      GPT_320_NS.gtadcmsc(),
    ",
  0x503220c4u64 => "
      GPT_320_NS.gtadcmss(),
    ",
  0x503220d0u64 => "
      GPT_320_NS.gtsecsr(),
    ",
  0x503220d4u64 => "
      GPT_320_NS.gtsecr(),
    ",
  0x503220e0u64 => "
      GPT_320_NS.gtber2(),
    ",
  0x503220e4u64 => "
      GPT_320_NS.gtolbr(),
    ",
  0x503220ecu64 => "
      GPT_320_NS.gticcr(),
    ",
  0x50323f00u64 => "
      GPT_OPS_NS.opscr(),
    ",
  0x50323f10u64 => "
      GPT_GTCLK_NS.gtclkcr(),
    ",
  0x50324000u64 => "
      PDG_NS.gtdlycr(),
    ",
  0x50324002u64 => "
      PDG_NS.gtdlycr2(),
    ",
  0x50324018u64 => "
      PDG_NS.gtdlyra()[0],
    ",
  0x5032401cu64 => "
      PDG_NS.gtdlyra()[1],
    ",
  0x50324020u64 => "
      PDG_NS.gtdlyra()[2],
    ",
  0x50324024u64 => "
      PDG_NS.gtdlyra()[3],
    ",
  0x5032401au64 => "
      PDG_NS.gtdlyrb()[0],
    ",
  0x5032401eu64 => "
      PDG_NS.gtdlyrb()[1],
    ",
  0x50324022u64 => "
      PDG_NS.gtdlyrb()[2],
    ",
  0x50324026u64 => "
      PDG_NS.gtdlyrb()[3],
    ",
  0x50324028u64 => "
      PDG_NS.gtdlyfa()[0],
    ",
  0x5032402cu64 => "
      PDG_NS.gtdlyfa()[1],
    ",
  0x50324030u64 => "
      PDG_NS.gtdlyfa()[2],
    ",
  0x50324034u64 => "
      PDG_NS.gtdlyfa()[3],
    ",
  0x5032402au64 => "
      PDG_NS.gtdlyfb()[0],
    ",
  0x5032402eu64 => "
      PDG_NS.gtdlyfb()[1],
    ",
  0x50324032u64 => "
      PDG_NS.gtdlyfb()[2],
    ",
  0x50324036u64 => "
      PDG_NS.gtdlyfb()[3],
    ",
  0x50338000u64 => "
      ADC_B_NS.adclkenr(),
    ",
  0x50338004u64 => "
      ADC_B_NS.adclksr(),
    ",
  0x50338008u64 => "
      ADC_B_NS.adclkcr(),
    ",
  0x5033800cu64 => "
      ADC_B_NS.adsycr(),
    ",
  0x50338010u64 => "
      ADC_B_NS.aduslpcr0(),
    ",
  0x50338014u64 => "
      ADC_B_NS.aduslpcr1(),
    ",
  0x50338020u64 => "
      ADC_B_NS.aderintcr(),
    ",
  0x50338024u64 => "
      ADC_B_NS.adovfintcr(),
    ",
  0x50338028u64 => "
      ADC_B_NS.adcalintcr(),
    ",
  0x50338040u64 => "
      ADC_B_NS.admdr(),
    ",
  0x50338044u64 => "
      ADC_B_NS.adgspcr(),
    ",
  0x50338048u64 => "
      ADC_B_NS.adsger(),
    ",
  0x5033804cu64 => "
      ADC_B_NS.adsgcr0(),
    ",
  0x50338050u64 => "
      ADC_B_NS.adsgcr1(),
    ",
  0x50338054u64 => "
      ADC_B_NS.adsgcr2(),
    ",
  0x5033805cu64 => "
      ADC_B_NS.adintcr(),
    ",
  0x50338060u64 => "
      ADC_B_NS.adswnr0(),
    ",
  0x50338064u64 => "
      ADC_B_NS.adswnr1(),
    ",
  0x50338080u64 => "
      ADC_B_NS.addeccr(),
    ",
  0x50338084u64 => "
      ADC_B_NS.adacmdr(),
    ",
  0x503380c0u64 => "
      ADC_B_NS.adtrgext()[0],
    ",
  0x503380d0u64 => "
      ADC_B_NS.adtrgext()[1],
    ",
  0x503380e0u64 => "
      ADC_B_NS.adtrgext()[2],
    ",
  0x503380f0u64 => "
      ADC_B_NS.adtrgext()[3],
    ",
  0x50338100u64 => "
      ADC_B_NS.adtrgext()[4],
    ",
  0x50338110u64 => "
      ADC_B_NS.adtrgext()[5],
    ",
  0x50338120u64 => "
      ADC_B_NS.adtrgext()[6],
    ",
  0x50338130u64 => "
      ADC_B_NS.adtrgext()[7],
    ",
  0x50338140u64 => "
      ADC_B_NS.adtrgext()[8],
    ",
  0x503380c4u64 => "
      ADC_B_NS.adtrgelc()[0],
    ",
  0x503380d4u64 => "
      ADC_B_NS.adtrgelc()[1],
    ",
  0x503380e4u64 => "
      ADC_B_NS.adtrgelc()[2],
    ",
  0x503380f4u64 => "
      ADC_B_NS.adtrgelc()[3],
    ",
  0x50338104u64 => "
      ADC_B_NS.adtrgelc()[4],
    ",
  0x50338114u64 => "
      ADC_B_NS.adtrgelc()[5],
    ",
  0x50338124u64 => "
      ADC_B_NS.adtrgelc()[6],
    ",
  0x50338134u64 => "
      ADC_B_NS.adtrgelc()[7],
    ",
  0x50338144u64 => "
      ADC_B_NS.adtrgelc()[8],
    ",
  0x503380c8u64 => "
      ADC_B_NS.adtrggpt()[0],
    ",
  0x503380d8u64 => "
      ADC_B_NS.adtrggpt()[1],
    ",
  0x503380e8u64 => "
      ADC_B_NS.adtrggpt()[2],
    ",
  0x503380f8u64 => "
      ADC_B_NS.adtrggpt()[3],
    ",
  0x50338108u64 => "
      ADC_B_NS.adtrggpt()[4],
    ",
  0x50338118u64 => "
      ADC_B_NS.adtrggpt()[5],
    ",
  0x50338128u64 => "
      ADC_B_NS.adtrggpt()[6],
    ",
  0x50338138u64 => "
      ADC_B_NS.adtrggpt()[7],
    ",
  0x50338148u64 => "
      ADC_B_NS.adtrggpt()[8],
    ",
  0x503381c0u64 => "
      ADC_B_NS.adtrgdlr0(),
    ",
  0x503381c4u64 => "
      ADC_B_NS.adtrgdlr1(),
    ",
  0x503381c8u64 => "
      ADC_B_NS.adtrgdlr2(),
    ",
  0x503381ccu64 => "
      ADC_B_NS.adtrgdlr3(),
    ",
  0x503381d0u64 => "
      ADC_B_NS.adtrgdlr4(),
    ",
  0x50338200u64 => "
      ADC_B_NS.adsgdcr()[0],
    ",
  0x50338204u64 => "
      ADC_B_NS.adsgdcr()[1],
    ",
  0x50338208u64 => "
      ADC_B_NS.adsgdcr()[2],
    ",
  0x5033820cu64 => "
      ADC_B_NS.adsgdcr()[3],
    ",
  0x50338210u64 => "
      ADC_B_NS.adsgdcr()[4],
    ",
  0x50338214u64 => "
      ADC_B_NS.adsgdcr()[5],
    ",
  0x50338218u64 => "
      ADC_B_NS.adsgdcr()[6],
    ",
  0x5033821cu64 => "
      ADC_B_NS.adsgdcr()[7],
    ",
  0x50338220u64 => "
      ADC_B_NS.adsgdcr()[8],
    ",
  0x50338240u64 => "
      ADC_B_NS.adsstr0(),
    ",
  0x50338244u64 => "
      ADC_B_NS.adsstr1(),
    ",
  0x50338248u64 => "
      ADC_B_NS.adsstr2(),
    ",
  0x5033824cu64 => "
      ADC_B_NS.adsstr3(),
    ",
  0x50338250u64 => "
      ADC_B_NS.adsstr4(),
    ",
  0x50338254u64 => "
      ADC_B_NS.adsstr5(),
    ",
  0x50338258u64 => "
      ADC_B_NS.adsstr6(),
    ",
  0x5033825cu64 => "
      ADC_B_NS.adsstr7(),
    ",
  0x50338260u64 => "
      ADC_B_NS.adcnvstr(),
    ",
  0x50338264u64 => "
      ADC_B_NS.adcalstcr(),
    ",
  0x50338280u64 => "
      ADC_B_NS.adshcr0(),
    ",
  0x50338284u64 => "
      ADC_B_NS.adshdcr0(),
    ",
  0x50338288u64 => "
      ADC_B_NS.adshstr0(),
    ",
  0x5033828cu64 => "
      ADC_B_NS.adshcr1(),
    ",
  0x50338290u64 => "
      ADC_B_NS.adshdcr1(),
    ",
  0x50338294u64 => "
      ADC_B_NS.adshstr1(),
    ",
  0x503382b0u64 => "
      ADC_B_NS.adcalshcr(),
    ",
  0x50338310u64 => "
      ADC_B_NS.adshsbpcr(),
    ",
  0x50338314u64 => "
      ADC_B_NS.adshdbpcr(),
    ",
  0x50338318u64 => "
      ADC_B_NS.adshsdcr0(),
    ",
  0x5033831cu64 => "
      ADC_B_NS.adshsdcr1(),
    ",
  0x50338320u64 => "
      ADC_B_NS.adrefcr(),
    ",
  0x50338340u64 => "
      ADC_B_NS.addfsr()[0],
    ",
  0x50338344u64 => "
      ADC_B_NS.addfsr()[1],
    ",
  0x50338360u64 => "
      ADC_B_NS.aduoftr()[0],
    ",
  0x50338364u64 => "
      ADC_B_NS.aduoftr()[1],
    ",
  0x50338368u64 => "
      ADC_B_NS.aduoftr()[2],
    ",
  0x5033836cu64 => "
      ADC_B_NS.aduoftr()[3],
    ",
  0x50338370u64 => "
      ADC_B_NS.aduoftr()[4],
    ",
  0x50338374u64 => "
      ADC_B_NS.aduoftr()[5],
    ",
  0x50338378u64 => "
      ADC_B_NS.aduoftr()[6],
    ",
  0x5033837cu64 => "
      ADC_B_NS.aduoftr()[7],
    ",
  0x50338380u64 => "
      ADC_B_NS.adugtr()[0],
    ",
  0x50338384u64 => "
      ADC_B_NS.adugtr()[1],
    ",
  0x50338388u64 => "
      ADC_B_NS.adugtr()[2],
    ",
  0x5033838cu64 => "
      ADC_B_NS.adugtr()[3],
    ",
  0x50338390u64 => "
      ADC_B_NS.adugtr()[4],
    ",
  0x50338394u64 => "
      ADC_B_NS.adugtr()[5],
    ",
  0x50338398u64 => "
      ADC_B_NS.adugtr()[6],
    ",
  0x5033839cu64 => "
      ADC_B_NS.adugtr()[7],
    ",
  0x503383a0u64 => "
      ADC_B_NS.adlimintcr(),
    ",
  0x503383a4u64 => "
      ADC_B_NS.adlimtr()[0],
    ",
  0x503383a8u64 => "
      ADC_B_NS.adlimtr()[1],
    ",
  0x503383acu64 => "
      ADC_B_NS.adlimtr()[2],
    ",
  0x503383b0u64 => "
      ADC_B_NS.adlimtr()[3],
    ",
  0x503383b4u64 => "
      ADC_B_NS.adlimtr()[4],
    ",
  0x503383b8u64 => "
      ADC_B_NS.adlimtr()[5],
    ",
  0x503383bcu64 => "
      ADC_B_NS.adlimtr()[6],
    ",
  0x503383c0u64 => "
      ADC_B_NS.adlimtr()[7],
    ",
  0x50338400u64 => "
      ADC_B_NS.adcmpenr(),
    ",
  0x50338404u64 => "
      ADC_B_NS.adcmpintcr(),
    ",
  0x50338408u64 => "
      ADC_B_NS.adccmpcr()[0],
    ",
  0x5033840cu64 => "
      ADC_B_NS.adccmpcr()[1],
    ",
  0x50338448u64 => "
      ADC_B_NS.adcmpmdr0(),
    ",
  0x5033844cu64 => "
      ADC_B_NS.adcmpmdr1(),
    ",
  0x50338458u64 => "
      ADC_B_NS.adcmptbr()[0],
    ",
  0x5033845cu64 => "
      ADC_B_NS.adcmptbr()[1],
    ",
  0x50338460u64 => "
      ADC_B_NS.adcmptbr()[2],
    ",
  0x50338464u64 => "
      ADC_B_NS.adcmptbr()[3],
    ",
  0x50338468u64 => "
      ADC_B_NS.adcmptbr()[4],
    ",
  0x5033846cu64 => "
      ADC_B_NS.adcmptbr()[5],
    ",
  0x50338470u64 => "
      ADC_B_NS.adcmptbr()[6],
    ",
  0x50338474u64 => "
      ADC_B_NS.adcmptbr()[7],
    ",
  0x503384c4u64 => "
      ADC_B_NS.adfifointcr(),
    ",
  0x503384c8u64 => "
      ADC_B_NS.adfifointlr0(),
    ",
  0x503384ccu64 => "
      ADC_B_NS.adfifointlr1(),
    ",
  0x503384d0u64 => "
      ADC_B_NS.adfifointlr2(),
    ",
  0x503384d4u64 => "
      ADC_B_NS.adfifointlr3(),
    ",
  0x503384d8u64 => "
      ADC_B_NS.adfifointlr4(),
    ",
  0x50338600u64 => "
      ADC_B_NS.adchcr()[0],
    ",
  0x50338610u64 => "
      ADC_B_NS.adchcr()[1],
    ",
  0x50338620u64 => "
      ADC_B_NS.adchcr()[2],
    ",
  0x50338630u64 => "
      ADC_B_NS.adchcr()[3],
    ",
  0x50338640u64 => "
      ADC_B_NS.adchcr()[4],
    ",
  0x50338650u64 => "
      ADC_B_NS.adchcr()[5],
    ",
  0x50338660u64 => "
      ADC_B_NS.adchcr()[6],
    ",
  0x50338670u64 => "
      ADC_B_NS.adchcr()[7],
    ",
  0x50338680u64 => "
      ADC_B_NS.adchcr()[8],
    ",
  0x50338690u64 => "
      ADC_B_NS.adchcr()[9],
    ",
  0x503386a0u64 => "
      ADC_B_NS.adchcr()[10],
    ",
  0x503386b0u64 => "
      ADC_B_NS.adchcr()[11],
    ",
  0x503386c0u64 => "
      ADC_B_NS.adchcr()[12],
    ",
  0x503386d0u64 => "
      ADC_B_NS.adchcr()[13],
    ",
  0x503386e0u64 => "
      ADC_B_NS.adchcr()[14],
    ",
  0x503386f0u64 => "
      ADC_B_NS.adchcr()[15],
    ",
  0x50338700u64 => "
      ADC_B_NS.adchcr()[16],
    ",
  0x50338710u64 => "
      ADC_B_NS.adchcr()[17],
    ",
  0x50338720u64 => "
      ADC_B_NS.adchcr()[18],
    ",
  0x50338730u64 => "
      ADC_B_NS.adchcr()[19],
    ",
  0x50338740u64 => "
      ADC_B_NS.adchcr()[20],
    ",
  0x50338750u64 => "
      ADC_B_NS.adchcr()[21],
    ",
  0x50338760u64 => "
      ADC_B_NS.adchcr()[22],
    ",
  0x50338770u64 => "
      ADC_B_NS.adchcr()[23],
    ",
  0x50338780u64 => "
      ADC_B_NS.adchcr()[24],
    ",
  0x50338790u64 => "
      ADC_B_NS.adchcr()[25],
    ",
  0x503387a0u64 => "
      ADC_B_NS.adchcr()[26],
    ",
  0x503387b0u64 => "
      ADC_B_NS.adchcr()[27],
    ",
  0x503387c0u64 => "
      ADC_B_NS.adchcr()[28],
    ",
  0x503387d0u64 => "
      ADC_B_NS.adchcr()[29],
    ",
  0x503387e0u64 => "
      ADC_B_NS.adchcr()[30],
    ",
  0x503387f0u64 => "
      ADC_B_NS.adchcr()[31],
    ",
  0x50338800u64 => "
      ADC_B_NS.adchcr()[32],
    ",
  0x50338604u64 => "
      ADC_B_NS.addopcra()[0],
    ",
  0x50338614u64 => "
      ADC_B_NS.addopcra()[1],
    ",
  0x50338624u64 => "
      ADC_B_NS.addopcra()[2],
    ",
  0x50338634u64 => "
      ADC_B_NS.addopcra()[3],
    ",
  0x50338644u64 => "
      ADC_B_NS.addopcra()[4],
    ",
  0x50338654u64 => "
      ADC_B_NS.addopcra()[5],
    ",
  0x50338664u64 => "
      ADC_B_NS.addopcra()[6],
    ",
  0x50338674u64 => "
      ADC_B_NS.addopcra()[7],
    ",
  0x50338684u64 => "
      ADC_B_NS.addopcra()[8],
    ",
  0x50338694u64 => "
      ADC_B_NS.addopcra()[9],
    ",
  0x503386a4u64 => "
      ADC_B_NS.addopcra()[10],
    ",
  0x503386b4u64 => "
      ADC_B_NS.addopcra()[11],
    ",
  0x503386c4u64 => "
      ADC_B_NS.addopcra()[12],
    ",
  0x503386d4u64 => "
      ADC_B_NS.addopcra()[13],
    ",
  0x503386e4u64 => "
      ADC_B_NS.addopcra()[14],
    ",
  0x503386f4u64 => "
      ADC_B_NS.addopcra()[15],
    ",
  0x50338704u64 => "
      ADC_B_NS.addopcra()[16],
    ",
  0x50338714u64 => "
      ADC_B_NS.addopcra()[17],
    ",
  0x50338724u64 => "
      ADC_B_NS.addopcra()[18],
    ",
  0x50338734u64 => "
      ADC_B_NS.addopcra()[19],
    ",
  0x50338744u64 => "
      ADC_B_NS.addopcra()[20],
    ",
  0x50338754u64 => "
      ADC_B_NS.addopcra()[21],
    ",
  0x50338764u64 => "
      ADC_B_NS.addopcra()[22],
    ",
  0x50338774u64 => "
      ADC_B_NS.addopcra()[23],
    ",
  0x50338784u64 => "
      ADC_B_NS.addopcra()[24],
    ",
  0x50338794u64 => "
      ADC_B_NS.addopcra()[25],
    ",
  0x503387a4u64 => "
      ADC_B_NS.addopcra()[26],
    ",
  0x503387b4u64 => "
      ADC_B_NS.addopcra()[27],
    ",
  0x503387c4u64 => "
      ADC_B_NS.addopcra()[28],
    ",
  0x503387d4u64 => "
      ADC_B_NS.addopcra()[29],
    ",
  0x503387e4u64 => "
      ADC_B_NS.addopcra()[30],
    ",
  0x503387f4u64 => "
      ADC_B_NS.addopcra()[31],
    ",
  0x50338804u64 => "
      ADC_B_NS.addopcra()[32],
    ",
  0x50338608u64 => "
      ADC_B_NS.addopcrb()[0],
    ",
  0x50338618u64 => "
      ADC_B_NS.addopcrb()[1],
    ",
  0x50338628u64 => "
      ADC_B_NS.addopcrb()[2],
    ",
  0x50338638u64 => "
      ADC_B_NS.addopcrb()[3],
    ",
  0x50338648u64 => "
      ADC_B_NS.addopcrb()[4],
    ",
  0x50338658u64 => "
      ADC_B_NS.addopcrb()[5],
    ",
  0x50338668u64 => "
      ADC_B_NS.addopcrb()[6],
    ",
  0x50338678u64 => "
      ADC_B_NS.addopcrb()[7],
    ",
  0x50338688u64 => "
      ADC_B_NS.addopcrb()[8],
    ",
  0x50338698u64 => "
      ADC_B_NS.addopcrb()[9],
    ",
  0x503386a8u64 => "
      ADC_B_NS.addopcrb()[10],
    ",
  0x503386b8u64 => "
      ADC_B_NS.addopcrb()[11],
    ",
  0x503386c8u64 => "
      ADC_B_NS.addopcrb()[12],
    ",
  0x503386d8u64 => "
      ADC_B_NS.addopcrb()[13],
    ",
  0x503386e8u64 => "
      ADC_B_NS.addopcrb()[14],
    ",
  0x503386f8u64 => "
      ADC_B_NS.addopcrb()[15],
    ",
  0x50338708u64 => "
      ADC_B_NS.addopcrb()[16],
    ",
  0x50338718u64 => "
      ADC_B_NS.addopcrb()[17],
    ",
  0x50338728u64 => "
      ADC_B_NS.addopcrb()[18],
    ",
  0x50338738u64 => "
      ADC_B_NS.addopcrb()[19],
    ",
  0x50338748u64 => "
      ADC_B_NS.addopcrb()[20],
    ",
  0x50338758u64 => "
      ADC_B_NS.addopcrb()[21],
    ",
  0x50338768u64 => "
      ADC_B_NS.addopcrb()[22],
    ",
  0x50338778u64 => "
      ADC_B_NS.addopcrb()[23],
    ",
  0x50338788u64 => "
      ADC_B_NS.addopcrb()[24],
    ",
  0x50338798u64 => "
      ADC_B_NS.addopcrb()[25],
    ",
  0x503387a8u64 => "
      ADC_B_NS.addopcrb()[26],
    ",
  0x503387b8u64 => "
      ADC_B_NS.addopcrb()[27],
    ",
  0x503387c8u64 => "
      ADC_B_NS.addopcrb()[28],
    ",
  0x503387d8u64 => "
      ADC_B_NS.addopcrb()[29],
    ",
  0x503387e8u64 => "
      ADC_B_NS.addopcrb()[30],
    ",
  0x503387f8u64 => "
      ADC_B_NS.addopcrb()[31],
    ",
  0x50338808u64 => "
      ADC_B_NS.addopcrb()[32],
    ",
  0x5033860cu64 => "
      ADC_B_NS.addopcrc()[0],
    ",
  0x5033861cu64 => "
      ADC_B_NS.addopcrc()[1],
    ",
  0x5033862cu64 => "
      ADC_B_NS.addopcrc()[2],
    ",
  0x5033863cu64 => "
      ADC_B_NS.addopcrc()[3],
    ",
  0x5033864cu64 => "
      ADC_B_NS.addopcrc()[4],
    ",
  0x5033865cu64 => "
      ADC_B_NS.addopcrc()[5],
    ",
  0x5033866cu64 => "
      ADC_B_NS.addopcrc()[6],
    ",
  0x5033867cu64 => "
      ADC_B_NS.addopcrc()[7],
    ",
  0x5033868cu64 => "
      ADC_B_NS.addopcrc()[8],
    ",
  0x5033869cu64 => "
      ADC_B_NS.addopcrc()[9],
    ",
  0x503386acu64 => "
      ADC_B_NS.addopcrc()[10],
    ",
  0x503386bcu64 => "
      ADC_B_NS.addopcrc()[11],
    ",
  0x503386ccu64 => "
      ADC_B_NS.addopcrc()[12],
    ",
  0x503386dcu64 => "
      ADC_B_NS.addopcrc()[13],
    ",
  0x503386ecu64 => "
      ADC_B_NS.addopcrc()[14],
    ",
  0x503386fcu64 => "
      ADC_B_NS.addopcrc()[15],
    ",
  0x5033870cu64 => "
      ADC_B_NS.addopcrc()[16],
    ",
  0x5033871cu64 => "
      ADC_B_NS.addopcrc()[17],
    ",
  0x5033872cu64 => "
      ADC_B_NS.addopcrc()[18],
    ",
  0x5033873cu64 => "
      ADC_B_NS.addopcrc()[19],
    ",
  0x5033874cu64 => "
      ADC_B_NS.addopcrc()[20],
    ",
  0x5033875cu64 => "
      ADC_B_NS.addopcrc()[21],
    ",
  0x5033876cu64 => "
      ADC_B_NS.addopcrc()[22],
    ",
  0x5033877cu64 => "
      ADC_B_NS.addopcrc()[23],
    ",
  0x5033878cu64 => "
      ADC_B_NS.addopcrc()[24],
    ",
  0x5033879cu64 => "
      ADC_B_NS.addopcrc()[25],
    ",
  0x503387acu64 => "
      ADC_B_NS.addopcrc()[26],
    ",
  0x503387bcu64 => "
      ADC_B_NS.addopcrc()[27],
    ",
  0x503387ccu64 => "
      ADC_B_NS.addopcrc()[28],
    ",
  0x503387dcu64 => "
      ADC_B_NS.addopcrc()[29],
    ",
  0x503387ecu64 => "
      ADC_B_NS.addopcrc()[30],
    ",
  0x503387fcu64 => "
      ADC_B_NS.addopcrc()[31],
    ",
  0x5033880cu64 => "
      ADC_B_NS.addopcrc()[32],
    ",
  0x50338c00u64 => "
      ADC_B_NS.adcalstr(),
    ",
  0x50338c04u64 => "
      ADC_B_NS.adshcscr(),
    ",
  0x50338c08u64 => "
      ADC_B_NS.adtrgenr(),
    ",
  0x50338c10u64 => "
      ADC_B_NS.adsystr(),
    ",
  0x50338c20u64 => "
      ADC_B_NS.adstr()[0],
    ",
  0x50338c24u64 => "
      ADC_B_NS.adstr()[1],
    ",
  0x50338c28u64 => "
      ADC_B_NS.adstr()[2],
    ",
  0x50338c2cu64 => "
      ADC_B_NS.adstr()[3],
    ",
  0x50338c30u64 => "
      ADC_B_NS.adstr()[4],
    ",
  0x50338c34u64 => "
      ADC_B_NS.adstr()[5],
    ",
  0x50338c38u64 => "
      ADC_B_NS.adstr()[6],
    ",
  0x50338c3cu64 => "
      ADC_B_NS.adstr()[7],
    ",
  0x50338c40u64 => "
      ADC_B_NS.adstr()[8],
    ",
  0x50338c60u64 => "
      ADC_B_NS.adstopr(),
    ",
  0x50338c80u64 => "
      ADC_B_NS.adsr(),
    ",
  0x50338c84u64 => "
      ADC_B_NS.adgrsr(),
    ",
  0x50338c88u64 => "
      ADC_B_NS.adersr(),
    ",
  0x50338c8cu64 => "
      ADC_B_NS.aderscr(),
    ",
  0x50338c98u64 => "
      ADC_B_NS.adcalendsr(),
    ",
  0x50338c9cu64 => "
      ADC_B_NS.adcalendscr(),
    ",
  0x50338ca0u64 => "
      ADC_B_NS.adovfersr(),
    ",
  0x50338ca4u64 => "
      ADC_B_NS.adovfchsr0(),
    ",
  0x50338cb0u64 => "
      ADC_B_NS.adovfexsr(),
    ",
  0x50338cb4u64 => "
      ADC_B_NS.adovferscr(),
    ",
  0x50338cb8u64 => "
      ADC_B_NS.adovfchscr0(),
    ",
  0x50338cc4u64 => "
      ADC_B_NS.adovfexscr(),
    ",
  0x50338cd0u64 => "
      ADC_B_NS.adfifosr0(),
    ",
  0x50338cd4u64 => "
      ADC_B_NS.adfifosr1(),
    ",
  0x50338cd8u64 => "
      ADC_B_NS.adfifosr2(),
    ",
  0x50338cdcu64 => "
      ADC_B_NS.adfifosr3(),
    ",
  0x50338ce0u64 => "
      ADC_B_NS.adfifosr4(),
    ",
  0x50338cf0u64 => "
      ADC_B_NS.adfifodcr(),
    ",
  0x50338cf4u64 => "
      ADC_B_NS.adfifoersr(),
    ",
  0x50338cf8u64 => "
      ADC_B_NS.adfifoerscr(),
    ",
  0x50338d00u64 => "
      ADC_B_NS.adcmptbsr(),
    ",
  0x50338d04u64 => "
      ADC_B_NS.adcmptbscr(),
    ",
  0x50338d08u64 => "
      ADC_B_NS.adcmpchsr0(),
    ",
  0x50338d14u64 => "
      ADC_B_NS.adcmpexsr(),
    ",
  0x50338d18u64 => "
      ADC_B_NS.adcmpchscr0(),
    ",
  0x50338d24u64 => "
      ADC_B_NS.adcmpexscr(),
    ",
  0x50338d28u64 => "
      ADC_B_NS.adlimgrsr(),
    ",
  0x50338d2cu64 => "
      ADC_B_NS.adlimchsr0(),
    ",
  0x50338d38u64 => "
      ADC_B_NS.adlimexsr(),
    ",
  0x50338d3cu64 => "
      ADC_B_NS.adlimgrscr(),
    ",
  0x50338d40u64 => "
      ADC_B_NS.adlimchscr0(),
    ",
  0x50338d4cu64 => "
      ADC_B_NS.adlimexscr(),
    ",
  0x50338d50u64 => "
      ADC_B_NS.adscanendsr(),
    ",
  0x50338d54u64 => "
      ADC_B_NS.adscanendscr(),
    ",
  0x5033a000u64 => "
      ADC_B_NS.addr()[0],
    ",
  0x5033a004u64 => "
      ADC_B_NS.addr()[1],
    ",
  0x5033a008u64 => "
      ADC_B_NS.addr()[2],
    ",
  0x5033a00cu64 => "
      ADC_B_NS.addr()[3],
    ",
  0x5033a010u64 => "
      ADC_B_NS.addr()[4],
    ",
  0x5033a014u64 => "
      ADC_B_NS.addr()[5],
    ",
  0x5033a018u64 => "
      ADC_B_NS.addr()[6],
    ",
  0x5033a01cu64 => "
      ADC_B_NS.addr()[7],
    ",
  0x5033a020u64 => "
      ADC_B_NS.addr()[8],
    ",
  0x5033a024u64 => "
      ADC_B_NS.addr()[9],
    ",
  0x5033a028u64 => "
      ADC_B_NS.addr()[10],
    ",
  0x5033a02cu64 => "
      ADC_B_NS.addr()[11],
    ",
  0x5033a030u64 => "
      ADC_B_NS.addr()[12],
    ",
  0x5033a034u64 => "
      ADC_B_NS.addr()[13],
    ",
  0x5033a038u64 => "
      ADC_B_NS.addr()[14],
    ",
  0x5033a03cu64 => "
      ADC_B_NS.addr()[15],
    ",
  0x5033a040u64 => "
      ADC_B_NS.addr()[16],
    ",
  0x5033a044u64 => "
      ADC_B_NS.addr()[17],
    ",
  0x5033a048u64 => "
      ADC_B_NS.addr()[18],
    ",
  0x5033a04cu64 => "
      ADC_B_NS.addr()[19],
    ",
  0x5033a050u64 => "
      ADC_B_NS.addr()[20],
    ",
  0x5033a054u64 => "
      ADC_B_NS.addr()[21],
    ",
  0x5033a058u64 => "
      ADC_B_NS.addr()[22],
    ",
  0x5033a200u64 => "
      ADC_B_NS.adfifodr()[0],
    ",
  0x5033a204u64 => "
      ADC_B_NS.adfifodr()[1],
    ",
  0x5033a208u64 => "
      ADC_B_NS.adfifodr()[2],
    ",
  0x5033a20cu64 => "
      ADC_B_NS.adfifodr()[3],
    ",
  0x5033a210u64 => "
      ADC_B_NS.adfifodr()[4],
    ",
  0x5033a214u64 => "
      ADC_B_NS.adfifodr()[5],
    ",
  0x5033a218u64 => "
      ADC_B_NS.adfifodr()[6],
    ",
  0x5033a21cu64 => "
      ADC_B_NS.adfifodr()[7],
    ",
  0x5033a220u64 => "
      ADC_B_NS.adfifodr()[8],
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
  0x50346000u64 => "
      MIPI_DSI_NS.isr(),
    ",
  0x50346010u64 => "
      MIPI_DSI_NS.linksr(),
    ",
  0x50346100u64 => "
      MIPI_DSI_NS.txsetr(),
    ",
  0x50346104u64 => "
      MIPI_DSI_NS.hsclksetr(),
    ",
  0x50346108u64 => "
      MIPI_DSI_NS.ulpssetr(),
    ",
  0x5034610cu64 => "
      MIPI_DSI_NS.ulpscr(),
    ",
  0x50346110u64 => "
      MIPI_DSI_NS.rstcr(),
    ",
  0x50346114u64 => "
      MIPI_DSI_NS.rstsr(),
    ",
  0x50346120u64 => "
      MIPI_DSI_NS.dsisetr(),
    ",
  0x50346160u64 => "
      MIPI_DSI_NS.txppd0r(),
    ",
  0x50346164u64 => "
      MIPI_DSI_NS.txppd1r(),
    ",
  0x50346168u64 => "
      MIPI_DSI_NS.txppd2r(),
    ",
  0x5034616cu64 => "
      MIPI_DSI_NS.txppd3r(),
    ",
  0x50346200u64 => "
      MIPI_DSI_NS.rxsr(),
    ",
  0x50346204u64 => "
      MIPI_DSI_NS.rxscr(),
    ",
  0x50346208u64 => "
      MIPI_DSI_NS.rxier(),
    ",
  0x50346210u64 => "
      MIPI_DSI_NS.presptobtasetr(),
    ",
  0x50346214u64 => "
      MIPI_DSI_NS.presptolpsetr(),
    ",
  0x50346218u64 => "
      MIPI_DSI_NS.presptohssetr(),
    ",
  0x50346220u64 => "
      MIPI_DSI_NS.akeplatir(),
    ",
  0x50346224u64 => "
      MIPI_DSI_NS.akepacmsr(),
    ",
  0x50346228u64 => "
      MIPI_DSI_NS.akepscr(),
    ",
  0x50346230u64 => "
      MIPI_DSI_NS.rxrssr(),
    ",
  0x50346234u64 => "
      MIPI_DSI_NS.rxrsscr(),
    ",
  0x50346238u64 => "
      MIPI_DSI_NS.rxrinfoowsr(),
    ",
  0x5034623cu64 => "
      MIPI_DSI_NS.rxrinfoowscr(),
    ",
  0x50346240u64 => "
      MIPI_DSI_NS.rxrssr()[0],
    ",
  0x50346244u64 => "
      MIPI_DSI_NS.rxrssr()[1],
    ",
  0x50346248u64 => "
      MIPI_DSI_NS.rxrssr()[2],
    ",
  0x5034624cu64 => "
      MIPI_DSI_NS.rxrssr()[3],
    ",
  0x503462c0u64 => "
      MIPI_DSI_NS.rxppd0r(),
    ",
  0x503462c4u64 => "
      MIPI_DSI_NS.rxppd1r(),
    ",
  0x503462c8u64 => "
      MIPI_DSI_NS.rxppd2r(),
    ",
  0x503462ccu64 => "
      MIPI_DSI_NS.rxppd3r(),
    ",
  0x503462e0u64 => "
      MIPI_DSI_NS.hstxtosetr(),
    ",
  0x503462e4u64 => "
      MIPI_DSI_NS.lrxhtosetr(),
    ",
  0x503462e8u64 => "
      MIPI_DSI_NS.tatosetr(),
    ",
  0x50346300u64 => "
      MIPI_DSI_NS.ferrsr(),
    ",
  0x50346304u64 => "
      MIPI_DSI_NS.ferrscr(),
    ",
  0x50346308u64 => "
      MIPI_DSI_NS.ferrier(),
    ",
  0x50346314u64 => "
      MIPI_DSI_NS.clstptsetr(),
    ",
  0x50346318u64 => "
      MIPI_DSI_NS.lptrnstsetr(),
    ",
  0x50346320u64 => "
      MIPI_DSI_NS.plsr(),
    ",
  0x50346324u64 => "
      MIPI_DSI_NS.plscr(),
    ",
  0x50346328u64 => "
      MIPI_DSI_NS.plier(),
    ",
  0x50346400u64 => "
      MIPI_DSI_NS.vmset0r(),
    ",
  0x50346404u64 => "
      MIPI_DSI_NS.vmset1r(),
    ",
  0x50346410u64 => "
      MIPI_DSI_NS.vmsr(),
    ",
  0x50346414u64 => "
      MIPI_DSI_NS.vmscr(),
    ",
  0x50346418u64 => "
      MIPI_DSI_NS.vmier(),
    ",
  0x50346420u64 => "
      MIPI_DSI_NS.vmppsetr(),
    ",
  0x50346428u64 => "
      MIPI_DSI_NS.vmvssetr(),
    ",
  0x5034642cu64 => "
      MIPI_DSI_NS.vmvpsetr(),
    ",
  0x50346430u64 => "
      MIPI_DSI_NS.vmhssetr(),
    ",
  0x50346434u64 => "
      MIPI_DSI_NS.vmhpsetr(),
    ",
  0x503465c0u64 => "
      MIPI_DSI_NS.sqch0set0r(),
    ",
  0x503465d0u64 => "
      MIPI_DSI_NS.sqch0sr(),
    ",
  0x503465d4u64 => "
      MIPI_DSI_NS.sqch0scr(),
    ",
  0x503465d8u64 => "
      MIPI_DSI_NS.sqch0ier(),
    ",
  0x50346600u64 => "
      MIPI_DSI_NS.sqch1set0r(),
    ",
  0x50346610u64 => "
      MIPI_DSI_NS.sqch1sr(),
    ",
  0x50346614u64 => "
      MIPI_DSI_NS.sqch1scr(),
    ",
  0x50346618u64 => "
      MIPI_DSI_NS.sqch1ier(),
    ",
  0x50346780u64 => "
      MIPI_DSI_NS.sqch0dscar()[0],
    ",
  0x50346790u64 => "
      MIPI_DSI_NS.sqch0dscar()[1],
    ",
  0x503467a0u64 => "
      MIPI_DSI_NS.sqch0dscar()[2],
    ",
  0x503467b0u64 => "
      MIPI_DSI_NS.sqch0dscar()[3],
    ",
  0x503467c0u64 => "
      MIPI_DSI_NS.sqch0dscar()[4],
    ",
  0x503467d0u64 => "
      MIPI_DSI_NS.sqch0dscar()[5],
    ",
  0x503467e0u64 => "
      MIPI_DSI_NS.sqch0dscar()[6],
    ",
  0x503467f0u64 => "
      MIPI_DSI_NS.sqch0dscar()[7],
    ",
  0x50346784u64 => "
      MIPI_DSI_NS.sqch0dscbr()[0],
    ",
  0x50346794u64 => "
      MIPI_DSI_NS.sqch0dscbr()[1],
    ",
  0x503467a4u64 => "
      MIPI_DSI_NS.sqch0dscbr()[2],
    ",
  0x503467b4u64 => "
      MIPI_DSI_NS.sqch0dscbr()[3],
    ",
  0x503467c4u64 => "
      MIPI_DSI_NS.sqch0dscbr()[4],
    ",
  0x503467d4u64 => "
      MIPI_DSI_NS.sqch0dscbr()[5],
    ",
  0x503467e4u64 => "
      MIPI_DSI_NS.sqch0dscbr()[6],
    ",
  0x503467f4u64 => "
      MIPI_DSI_NS.sqch0dscbr()[7],
    ",
  0x50346788u64 => "
      MIPI_DSI_NS.sqch0dsccr()[0],
    ",
  0x50346798u64 => "
      MIPI_DSI_NS.sqch0dsccr()[1],
    ",
  0x503467a8u64 => "
      MIPI_DSI_NS.sqch0dsccr()[2],
    ",
  0x503467b8u64 => "
      MIPI_DSI_NS.sqch0dsccr()[3],
    ",
  0x503467c8u64 => "
      MIPI_DSI_NS.sqch0dsccr()[4],
    ",
  0x503467d8u64 => "
      MIPI_DSI_NS.sqch0dsccr()[5],
    ",
  0x503467e8u64 => "
      MIPI_DSI_NS.sqch0dsccr()[6],
    ",
  0x503467f8u64 => "
      MIPI_DSI_NS.sqch0dsccr()[7],
    ",
  0x5034678cu64 => "
      MIPI_DSI_NS.sqch0dscdr()[0],
    ",
  0x5034679cu64 => "
      MIPI_DSI_NS.sqch0dscdr()[1],
    ",
  0x503467acu64 => "
      MIPI_DSI_NS.sqch0dscdr()[2],
    ",
  0x503467bcu64 => "
      MIPI_DSI_NS.sqch0dscdr()[3],
    ",
  0x503467ccu64 => "
      MIPI_DSI_NS.sqch0dscdr()[4],
    ",
  0x503467dcu64 => "
      MIPI_DSI_NS.sqch0dscdr()[5],
    ",
  0x503467ecu64 => "
      MIPI_DSI_NS.sqch0dscdr()[6],
    ",
  0x503467fcu64 => "
      MIPI_DSI_NS.sqch0dscdr()[7],
    ",
  0x50346800u64 => "
      MIPI_DSI_NS.sqch1dscar()[0],
    ",
  0x50346810u64 => "
      MIPI_DSI_NS.sqch1dscar()[1],
    ",
  0x50346820u64 => "
      MIPI_DSI_NS.sqch1dscar()[2],
    ",
  0x50346830u64 => "
      MIPI_DSI_NS.sqch1dscar()[3],
    ",
  0x50346840u64 => "
      MIPI_DSI_NS.sqch1dscar()[4],
    ",
  0x50346850u64 => "
      MIPI_DSI_NS.sqch1dscar()[5],
    ",
  0x50346860u64 => "
      MIPI_DSI_NS.sqch1dscar()[6],
    ",
  0x50346870u64 => "
      MIPI_DSI_NS.sqch1dscar()[7],
    ",
  0x50346804u64 => "
      MIPI_DSI_NS.sqch1dscbr()[0],
    ",
  0x50346814u64 => "
      MIPI_DSI_NS.sqch1dscbr()[1],
    ",
  0x50346824u64 => "
      MIPI_DSI_NS.sqch1dscbr()[2],
    ",
  0x50346834u64 => "
      MIPI_DSI_NS.sqch1dscbr()[3],
    ",
  0x50346844u64 => "
      MIPI_DSI_NS.sqch1dscbr()[4],
    ",
  0x50346854u64 => "
      MIPI_DSI_NS.sqch1dscbr()[5],
    ",
  0x50346864u64 => "
      MIPI_DSI_NS.sqch1dscbr()[6],
    ",
  0x50346874u64 => "
      MIPI_DSI_NS.sqch1dscbr()[7],
    ",
  0x50346808u64 => "
      MIPI_DSI_NS.sqch1dsccr()[0],
    ",
  0x50346818u64 => "
      MIPI_DSI_NS.sqch1dsccr()[1],
    ",
  0x50346828u64 => "
      MIPI_DSI_NS.sqch1dsccr()[2],
    ",
  0x50346838u64 => "
      MIPI_DSI_NS.sqch1dsccr()[3],
    ",
  0x50346848u64 => "
      MIPI_DSI_NS.sqch1dsccr()[4],
    ",
  0x50346858u64 => "
      MIPI_DSI_NS.sqch1dsccr()[5],
    ",
  0x50346868u64 => "
      MIPI_DSI_NS.sqch1dsccr()[6],
    ",
  0x50346878u64 => "
      MIPI_DSI_NS.sqch1dsccr()[7],
    ",
  0x5034680cu64 => "
      MIPI_DSI_NS.sqch1dscdr()[0],
    ",
  0x5034681cu64 => "
      MIPI_DSI_NS.sqch1dscdr()[1],
    ",
  0x5034682cu64 => "
      MIPI_DSI_NS.sqch1dscdr()[2],
    ",
  0x5034683cu64 => "
      MIPI_DSI_NS.sqch1dscdr()[3],
    ",
  0x5034684cu64 => "
      MIPI_DSI_NS.sqch1dscdr()[4],
    ",
  0x5034685cu64 => "
      MIPI_DSI_NS.sqch1dscdr()[5],
    ",
  0x5034686cu64 => "
      MIPI_DSI_NS.sqch1dscdr()[6],
    ",
  0x5034687cu64 => "
      MIPI_DSI_NS.sqch1dscdr()[7],
    ",
  0x50346c00u64 => "
      MIPI_PHY_0_NS.dphyrefcr(),
    ",
  0x50346c04u64 => "
      MIPI_PHY_0_NS.dphyplfcr(),
    ",
  0x50346c08u64 => "
      MIPI_PHY_0_NS.dphyplocr(),
    ",
  0x50346c0cu64 => "
      MIPI_PHY_0_NS.dphyesccr(),
    ",
  0x50346c10u64 => "
      MIPI_PHY_0_NS.dphypwrcr(),
    ",
  0x50346c1cu64 => "
      MIPI_PHY_0_NS.dphysfr(),
    ",
  0x50346c20u64 => "
      MIPI_PHY_0_NS.dphyocr(),
    ",
  0x50346c24u64 => "
      MIPI_PHY_0_NS.dphytim1(),
    ",
  0x50346c28u64 => "
      MIPI_PHY_0_NS.dphytim2(),
    ",
  0x50346c2cu64 => "
      MIPI_PHY_0_NS.dphytim3(),
    ",
  0x50346c30u64 => "
      MIPI_PHY_0_NS.dphytim4(),
    ",
  0x50346c34u64 => "
      MIPI_PHY_0_NS.dphytim5(),
    ",
  0x50346c38u64 => "
      MIPI_PHY_0_NS.dphytim6(),
    ",
  0x50346c48u64 => "
      MIPI_PHY_0_NS.dphymdc(),
    ",
  0x50347000u64 => "
      MIPI_CSI_0_NS.mcg(),
    ",
  0x50347010u64 => "
      MIPI_CSI_0_NS.mct0(),
    ",
  0x50347018u64 => "
      MIPI_CSI_0_NS.mct2(),
    ",
  0x5034701cu64 => "
      MIPI_CSI_0_NS.mct3(),
    ",
  0x50347028u64 => "
      MIPI_CSI_0_NS.rtct(),
    ",
  0x5034702cu64 => "
      MIPI_CSI_0_NS.rtst(),
    ",
  0x50347040u64 => "
      MIPI_CSI_0_NS.epct(),
    ",
  0x50347044u64 => "
      MIPI_CSI_0_NS.emct(),
    ",
  0x50347050u64 => "
      MIPI_CSI_0_NS.mist(),
    ",
  0x50347060u64 => "
      MIPI_CSI_0_NS.dtel(),
    ",
  0x50347064u64 => "
      MIPI_CSI_0_NS.dteh(),
    ",
  0x50347070u64 => "
      MIPI_CSI_0_NS.rxst(),
    ",
  0x50347074u64 => "
      MIPI_CSI_0_NS.rxsc(),
    ",
  0x50347078u64 => "
      MIPI_CSI_0_NS.rxie(),
    ",
  0x50347080u64 => "
      MIPI_CSI_0_NS.dlst()[0],
    ",
  0x50347090u64 => "
      MIPI_CSI_0_NS.dlst()[1],
    ",
  0x50347084u64 => "
      MIPI_CSI_0_NS.dlsc()[0],
    ",
  0x50347094u64 => "
      MIPI_CSI_0_NS.dlsc()[1],
    ",
  0x50347088u64 => "
      MIPI_CSI_0_NS.dlie()[0],
    ",
  0x50347098u64 => "
      MIPI_CSI_0_NS.dlie()[1],
    ",
  0x50347100u64 => "
      MIPI_CSI_0_NS.vcst()[0],
    ",
  0x50347110u64 => "
      MIPI_CSI_0_NS.vcst()[1],
    ",
  0x50347120u64 => "
      MIPI_CSI_0_NS.vcst()[2],
    ",
  0x50347130u64 => "
      MIPI_CSI_0_NS.vcst()[3],
    ",
  0x50347140u64 => "
      MIPI_CSI_0_NS.vcst()[4],
    ",
  0x50347150u64 => "
      MIPI_CSI_0_NS.vcst()[5],
    ",
  0x50347160u64 => "
      MIPI_CSI_0_NS.vcst()[6],
    ",
  0x50347170u64 => "
      MIPI_CSI_0_NS.vcst()[7],
    ",
  0x50347180u64 => "
      MIPI_CSI_0_NS.vcst()[8],
    ",
  0x50347190u64 => "
      MIPI_CSI_0_NS.vcst()[9],
    ",
  0x503471a0u64 => "
      MIPI_CSI_0_NS.vcst()[10],
    ",
  0x503471b0u64 => "
      MIPI_CSI_0_NS.vcst()[11],
    ",
  0x503471c0u64 => "
      MIPI_CSI_0_NS.vcst()[12],
    ",
  0x503471d0u64 => "
      MIPI_CSI_0_NS.vcst()[13],
    ",
  0x503471e0u64 => "
      MIPI_CSI_0_NS.vcst()[14],
    ",
  0x503471f0u64 => "
      MIPI_CSI_0_NS.vcst()[15],
    ",
  0x50347104u64 => "
      MIPI_CSI_0_NS.vcsc()[0],
    ",
  0x50347114u64 => "
      MIPI_CSI_0_NS.vcsc()[1],
    ",
  0x50347124u64 => "
      MIPI_CSI_0_NS.vcsc()[2],
    ",
  0x50347134u64 => "
      MIPI_CSI_0_NS.vcsc()[3],
    ",
  0x50347144u64 => "
      MIPI_CSI_0_NS.vcsc()[4],
    ",
  0x50347154u64 => "
      MIPI_CSI_0_NS.vcsc()[5],
    ",
  0x50347164u64 => "
      MIPI_CSI_0_NS.vcsc()[6],
    ",
  0x50347174u64 => "
      MIPI_CSI_0_NS.vcsc()[7],
    ",
  0x50347184u64 => "
      MIPI_CSI_0_NS.vcsc()[8],
    ",
  0x50347194u64 => "
      MIPI_CSI_0_NS.vcsc()[9],
    ",
  0x503471a4u64 => "
      MIPI_CSI_0_NS.vcsc()[10],
    ",
  0x503471b4u64 => "
      MIPI_CSI_0_NS.vcsc()[11],
    ",
  0x503471c4u64 => "
      MIPI_CSI_0_NS.vcsc()[12],
    ",
  0x503471d4u64 => "
      MIPI_CSI_0_NS.vcsc()[13],
    ",
  0x503471e4u64 => "
      MIPI_CSI_0_NS.vcsc()[14],
    ",
  0x503471f4u64 => "
      MIPI_CSI_0_NS.vcsc()[15],
    ",
  0x50347108u64 => "
      MIPI_CSI_0_NS.vcie()[0],
    ",
  0x50347118u64 => "
      MIPI_CSI_0_NS.vcie()[1],
    ",
  0x50347128u64 => "
      MIPI_CSI_0_NS.vcie()[2],
    ",
  0x50347138u64 => "
      MIPI_CSI_0_NS.vcie()[3],
    ",
  0x50347148u64 => "
      MIPI_CSI_0_NS.vcie()[4],
    ",
  0x50347158u64 => "
      MIPI_CSI_0_NS.vcie()[5],
    ",
  0x50347168u64 => "
      MIPI_CSI_0_NS.vcie()[6],
    ",
  0x50347178u64 => "
      MIPI_CSI_0_NS.vcie()[7],
    ",
  0x50347188u64 => "
      MIPI_CSI_0_NS.vcie()[8],
    ",
  0x50347198u64 => "
      MIPI_CSI_0_NS.vcie()[9],
    ",
  0x503471a8u64 => "
      MIPI_CSI_0_NS.vcie()[10],
    ",
  0x503471b8u64 => "
      MIPI_CSI_0_NS.vcie()[11],
    ",
  0x503471c8u64 => "
      MIPI_CSI_0_NS.vcie()[12],
    ",
  0x503471d8u64 => "
      MIPI_CSI_0_NS.vcie()[13],
    ",
  0x503471e8u64 => "
      MIPI_CSI_0_NS.vcie()[14],
    ",
  0x503471f8u64 => "
      MIPI_CSI_0_NS.vcie()[15],
    ",
  0x50347200u64 => "
      MIPI_CSI_0_NS.pmst(),
    ",
  0x50347204u64 => "
      MIPI_CSI_0_NS.pmsc(),
    ",
  0x50347208u64 => "
      MIPI_CSI_0_NS.pmie(),
    ",
  0x50347280u64 => "
      MIPI_CSI_0_NS.gsct(),
    ",
  0x50347284u64 => "
      MIPI_CSI_0_NS.gsst(),
    ",
  0x50347288u64 => "
      MIPI_CSI_0_NS.gssc(),
    ",
  0x5034728cu64 => "
      MIPI_CSI_0_NS.gsie(),
    ",
  0x50347290u64 => "
      MIPI_CSI_0_NS.gsht(),
    ",
  0x50347294u64 => "
      MIPI_CSI_0_NS.gsiu(),
    ",
  0x50347400u64 => "
      VIN_0_NS.mc(),
    ",
  0x50347404u64 => "
      VIN_0_NS.ms(),
    ",
  0x50347408u64 => "
      VIN_0_NS.fc(),
    ",
  0x5034740cu64 => "
      VIN_0_NS.slprc(),
    ",
  0x50347410u64 => "
      VIN_0_NS.elprc(),
    ",
  0x50347414u64 => "
      VIN_0_NS.spprc(),
    ",
  0x50347418u64 => "
      VIN_0_NS.epprc(),
    ",
  0x50347420u64 => "
      VIN_0_NS.csi_ifmd(),
    ",
  0x50347424u64 => "
      VIN_0_NS.csifld(),
    ",
  0x5034742cu64 => "
      VIN_0_NS.is(),
    ",
  0x50347430u64 => "
      VIN_0_NS.mb1(),
    ",
  0x50347434u64 => "
      VIN_0_NS.mb2(),
    ",
  0x50347438u64 => "
      VIN_0_NS.mb3(),
    ",
  0x5034743cu64 => "
      VIN_0_NS.lc(),
    ",
  0x50347440u64 => "
      VIN_0_NS.ie(),
    ",
  0x50347444u64 => "
      VIN_0_NS.ints(),
    ",
  0x50347448u64 => "
      VIN_0_NS.si(),
    ",
  0x50347454u64 => "
      VIN_0_NS.mtcstop(),
    ",
  0x50347458u64 => "
      VIN_0_NS.dmr(),
    ",
  0x50347460u64 => "
      VIN_0_NS.uvaof(),
    ",
  0x50347480u64 => "
      VIN_0_NS.uds_ctrl(),
    ",
  0x50347484u64 => "
      VIN_0_NS.uds_scale(),
    ",
  0x50347490u64 => "
      VIN_0_NS.uds_pass_bwidth(),
    ",
  0x503474a4u64 => "
      VIN_0_NS.uds_clip_size(),
    ",
  0x50347500u64 => "
      VIN_0_NS.lutp(),
    ",
  0x50347504u64 => "
      VIN_0_NS.lutd(),
    ",
  0x50347628u64 => "
      VIN_0_NS.yccr1(),
    ",
  0x5034762cu64 => "
      VIN_0_NS.yccr2(),
    ",
  0x50347630u64 => "
      VIN_0_NS.yccr3(),
    ",
  0x50347634u64 => "
      VIN_0_NS.cbccr1(),
    ",
  0x50347638u64 => "
      VIN_0_NS.cbccr2(),
    ",
  0x5034763cu64 => "
      VIN_0_NS.cbccr3(),
    ",
  0x50347640u64 => "
      VIN_0_NS.crccr1(),
    ",
  0x50347644u64 => "
      VIN_0_NS.crccr2(),
    ",
  0x50347648u64 => "
      VIN_0_NS.crccr3(),
    ",
  0x50347700u64 => "
      VIN_0_NS.csce1(),
    ",
  0x50347704u64 => "
      VIN_0_NS.csce2(),
    ",
  0x50347708u64 => "
      VIN_0_NS.csce3(),
    ",
  0x5034770cu64 => "
      VIN_0_NS.csce4(),
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
  0x50351000u64 => "
      USBHS_NS.syscfg(),
    ",
  0x50351002u64 => "
      USBHS_NS.buswait(),
    ",
  0x50351004u64 => "
      USBHS_NS.syssts0(),
    ",
  0x50351006u64 => "
      USBHS_NS.pllsta(),
    ",
  0x50351008u64 => "
      USBHS_NS.dvstctr0(),
    ",
  0x5035100cu64 => "
      USBHS_NS.testmode(),
    ",
  0x50351014u64 => "
      USBHS_NS.cfifo(),
      USBHS_NS.cfifol(),
      USBHS_NS.cfifoll(),
    ",
  0x50351016u64 => "
      USBHS_NS.cfifoh(),
    ",
  0x50351017u64 => "
      USBHS_NS.cfifohh(),
    ",
  0x50351018u64 => "
      USBHS_NS.dfifo()[0],
      USBHS_NS.dfifol()[0],
      USBHS_NS.dfifoll()[0],
    ",
  0x5035101cu64 => "
      USBHS_NS.dfifo()[1],
      USBHS_NS.dfifol()[1],
      USBHS_NS.dfifoll()[1],
    ",
  0x5035101au64 => "
      USBHS_NS.d0fifoh(),
      USBHS_NS.d1fifoh(),
    ",
  0x5035101bu64 => "
      USBHS_NS.d0fifohh(),
      USBHS_NS.d1fifohh(),
    ",
  0x50351020u64 => "
      USBHS_NS.cfifosel(),
    ",
  0x50351022u64 => "
      USBHS_NS.cfifoctr(),
    ",
  0x50351028u64 => "
      USBHS_NS.dfifosel()[0],
    ",
  0x5035102cu64 => "
      USBHS_NS.dfifosel()[1],
    ",
  0x5035102au64 => "
      USBHS_NS.dfifoctr()[0],
    ",
  0x5035102eu64 => "
      USBHS_NS.dfifoctr()[1],
    ",
  0x50351030u64 => "
      USBHS_NS.intenb0(),
    ",
  0x50351032u64 => "
      USBHS_NS.intenb1(),
    ",
  0x50351036u64 => "
      USBHS_NS.brdyenb(),
    ",
  0x50351038u64 => "
      USBHS_NS.nrdyenb(),
    ",
  0x5035103au64 => "
      USBHS_NS.bempenb(),
    ",
  0x5035103cu64 => "
      USBHS_NS.sofcfg(),
    ",
  0x5035103eu64 => "
      USBHS_NS.physet(),
    ",
  0x50351040u64 => "
      USBHS_NS.intsts0(),
    ",
  0x50351042u64 => "
      USBHS_NS.intsts1(),
    ",
  0x50351046u64 => "
      USBHS_NS.brdysts(),
    ",
  0x50351048u64 => "
      USBHS_NS.nrdysts(),
    ",
  0x5035104au64 => "
      USBHS_NS.bempsts(),
    ",
  0x5035104cu64 => "
      USBHS_NS.frmnum(),
    ",
  0x5035104eu64 => "
      USBHS_NS.ufrmnum(),
    ",
  0x50351050u64 => "
      USBHS_NS.usbaddr(),
    ",
  0x50351054u64 => "
      USBHS_NS.usbreq(),
    ",
  0x50351056u64 => "
      USBHS_NS.usbval(),
    ",
  0x50351058u64 => "
      USBHS_NS.usbindx(),
    ",
  0x5035105au64 => "
      USBHS_NS.usbleng(),
    ",
  0x5035105cu64 => "
      USBHS_NS.dcpcfg(),
    ",
  0x5035105eu64 => "
      USBHS_NS.dcpmaxp(),
    ",
  0x50351060u64 => "
      USBHS_NS.dcpctr(),
    ",
  0x50351064u64 => "
      USBHS_NS.pipesel(),
    ",
  0x50351068u64 => "
      USBHS_NS.pipecfg(),
    ",
  0x5035106au64 => "
      USBHS_NS.pipebuf(),
    ",
  0x5035106cu64 => "
      USBHS_NS.pipemaxp(),
    ",
  0x5035106eu64 => "
      USBHS_NS.pipeperi(),
    ",
  0x50351070u64 => "
      USBHS_NS.pipectr()[0],
    ",
  0x50351074u64 => "
      USBHS_NS.pipectr()[1],
    ",
  0x50351078u64 => "
      USBHS_NS.pipectr()[2],
    ",
  0x5035107cu64 => "
      USBHS_NS.pipectr()[3],
    ",
  0x50351080u64 => "
      USBHS_NS.pipectr()[4],
    ",
  0x50351084u64 => "
      USBHS_NS.pipectr()[5],
    ",
  0x50351088u64 => "
      USBHS_NS.pipectr()[6],
    ",
  0x5035108cu64 => "
      USBHS_NS.pipectr()[7],
    ",
  0x50351090u64 => "
      USBHS_NS.pipectr()[8],
      USBHS_NS.pipetre()[0],
    ",
  0x50351094u64 => "
      USBHS_NS.pipetre()[1],
    ",
  0x50351098u64 => "
      USBHS_NS.pipetre()[2],
    ",
  0x5035109cu64 => "
      USBHS_NS.pipetre()[3],
    ",
  0x503510a0u64 => "
      USBHS_NS.pipetre()[4],
    ",
  0x50351092u64 => "
      USBHS_NS.pipetrn()[0],
    ",
  0x50351096u64 => "
      USBHS_NS.pipetrn()[1],
    ",
  0x5035109au64 => "
      USBHS_NS.pipetrn()[2],
    ",
  0x5035109eu64 => "
      USBHS_NS.pipetrn()[3],
    ",
  0x503510a2u64 => "
      USBHS_NS.pipetrn()[4],
    ",
  0x503510d0u64 => "
      USBHS_NS.devadd()[0],
    ",
  0x503510d2u64 => "
      USBHS_NS.devadd()[1],
    ",
  0x503510d4u64 => "
      USBHS_NS.devadd()[2],
    ",
  0x503510d6u64 => "
      USBHS_NS.devadd()[3],
    ",
  0x503510d8u64 => "
      USBHS_NS.devadd()[4],
    ",
  0x503510dau64 => "
      USBHS_NS.devadd()[5],
    ",
  0x503510dcu64 => "
      USBHS_NS.devadd()[6],
    ",
  0x503510deu64 => "
      USBHS_NS.devadd()[7],
    ",
  0x503510e0u64 => "
      USBHS_NS.devadd()[8],
    ",
  0x503510e2u64 => "
      USBHS_NS.devadd()[9],
    ",
  0x503510e4u64 => "
      USBHS_NS.devadda(),
    ",
  0x50351100u64 => "
      USBHS_NS.lpctrl(),
    ",
  0x50351102u64 => "
      USBHS_NS.lpsts(),
    ",
  0x50351140u64 => "
      USBHS_NS.bcctrl(),
    ",
  0x50351144u64 => "
      USBHS_NS.pl1ctrl1(),
    ",
  0x50351146u64 => "
      USBHS_NS.pl1ctrl2(),
    ",
  0x50351148u64 => "
      USBHS_NS.hl1ctrl1(),
    ",
  0x5035114au64 => "
      USBHS_NS.hl1ctrl2(),
    ",
  0x50351160u64 => "
      USBHS_NS.dpusr0r(),
    ",
  0x50351164u64 => "
      USBHS_NS.dpusr1r(),
    ",
  0x50351168u64 => "
      USBHS_NS.dpusr2r(),
    ",
  0x5035116au64 => "
      USBHS_NS.dpusrcr(),
    ",
  0x50358000u64 => "
      SCI_0_B_NS.rdr(),
    ",
  0x50358004u64 => "
      SCI_0_B_NS.tdr(),
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
  0x5035f000u64 => "
      I_3_C_NS.prts(),
    ",
  0x5035f010u64 => "
      I_3_C_NS.cectl(),
    ",
  0x5035f014u64 => "
      I_3_C_NS.bctl(),
    ",
  0x5035f018u64 => "
      I_3_C_NS.msdvad(),
    ",
  0x5035f020u64 => "
      I_3_C_NS.rstctl(),
    ",
  0x5035f024u64 => "
      I_3_C_NS.prsst(),
    ",
  0x5035f030u64 => "
      I_3_C_NS.inst(),
    ",
  0x5035f034u64 => "
      I_3_C_NS.inste(),
    ",
  0x5035f038u64 => "
      I_3_C_NS.inie(),
    ",
  0x5035f03cu64 => "
      I_3_C_NS.instfc(),
    ",
  0x5035f044u64 => "
      I_3_C_NS.dvct(),
    ",
  0x5035f058u64 => "
      I_3_C_NS.ibinctl(),
    ",
  0x5035f060u64 => "
      I_3_C_NS.bfctl(),
    ",
  0x5035f064u64 => "
      I_3_C_NS.svctl(),
    ",
  0x5035f070u64 => "
      I_3_C_NS.refckctl(),
    ",
  0x5035f074u64 => "
      I_3_C_NS.stdbr(),
    ",
  0x5035f078u64 => "
      I_3_C_NS.extbr(),
    ",
  0x5035f07cu64 => "
      I_3_C_NS.bfrecdt(),
    ",
  0x5035f080u64 => "
      I_3_C_NS.bavlcdt(),
    ",
  0x5035f084u64 => "
      I_3_C_NS.bidlcdt(),
    ",
  0x5035f088u64 => "
      I_3_C_NS.outctl(),
    ",
  0x5035f08cu64 => "
      I_3_C_NS.inctl(),
    ",
  0x5035f090u64 => "
      I_3_C_NS.tmoctl(),
    ",
  0x5035f098u64 => "
      I_3_C_NS.wuctl(),
    ",
  0x5035f0a0u64 => "
      I_3_C_NS.ackctl(),
    ",
  0x5035f0a4u64 => "
      I_3_C_NS.scstrctl(),
    ",
  0x5035f0b0u64 => "
      I_3_C_NS.scstlctl(),
    ",
  0x5035f0c0u64 => "
      I_3_C_NS.svtdlg0(),
    ",
  0x5035f120u64 => "
      I_3_C_NS.stctl(),
    ",
  0x5035f124u64 => "
      I_3_C_NS.atctl(),
    ",
  0x5035f128u64 => "
      I_3_C_NS.attrg(),
    ",
  0x5035f12cu64 => "
      I_3_C_NS.atccnte(),
    ",
  0x5035f140u64 => "
      I_3_C_NS.cndctl(),
    ",
  0x5035f150u64 => "
      I_3_C_NS.ncmdqp(),
    ",
  0x5035f154u64 => "
      I_3_C_NS.nrspqp(),
    ",
  0x5035f158u64 => "
      I_3_C_NS.ntdtbp0(),
      I_3_C_NS.ntdtbp0_by(),
    ",
  0x5035f17cu64 => "
      I_3_C_NS.nibiqp(),
    ",
  0x5035f180u64 => "
      I_3_C_NS.nrsqp(),
    ",
  0x5035f184u64 => "
      I_3_C_NS.hcmdqp(),
    ",
  0x5035f188u64 => "
      I_3_C_NS.hrspqp(),
    ",
  0x5035f18cu64 => "
      I_3_C_NS.htdtbp(),
    ",
  0x5035f190u64 => "
      I_3_C_NS.nqthctl(),
    ",
  0x5035f194u64 => "
      I_3_C_NS.ntbthctl0(),
    ",
  0x5035f1c0u64 => "
      I_3_C_NS.nrqthctl(),
    ",
  0x5035f1c4u64 => "
      I_3_C_NS.hqthctl(),
    ",
  0x5035f1c8u64 => "
      I_3_C_NS.htbthctl(),
    ",
  0x5035f1d0u64 => "
      I_3_C_NS.bst(),
    ",
  0x5035f1d4u64 => "
      I_3_C_NS.bste(),
    ",
  0x5035f1d8u64 => "
      I_3_C_NS.bie(),
    ",
  0x5035f1dcu64 => "
      I_3_C_NS.bstfc(),
    ",
  0x5035f1e0u64 => "
      I_3_C_NS.ntst(),
    ",
  0x5035f1e4u64 => "
      I_3_C_NS.ntste(),
    ",
  0x5035f1e8u64 => "
      I_3_C_NS.ntie(),
    ",
  0x5035f1ecu64 => "
      I_3_C_NS.ntstfc(),
    ",
  0x5035f200u64 => "
      I_3_C_NS.htst(),
    ",
  0x5035f204u64 => "
      I_3_C_NS.htste(),
    ",
  0x5035f208u64 => "
      I_3_C_NS.htie(),
    ",
  0x5035f20cu64 => "
      I_3_C_NS.htstfc(),
    ",
  0x5035f210u64 => "
      I_3_C_NS.bcst(),
    ",
  0x5035f214u64 => "
      I_3_C_NS.svst(),
    ",
  0x5035f218u64 => "
      I_3_C_NS.wust(),
    ",
  0x5035f21cu64 => "
      I_3_C_NS.mrccpt(),
    ",
  0x5035f224u64 => "
      I_3_C_NS.datbas()[0],
    ",
  0x5035f22cu64 => "
      I_3_C_NS.datbas()[1],
    ",
  0x5035f234u64 => "
      I_3_C_NS.datbas()[2],
    ",
  0x5035f23cu64 => "
      I_3_C_NS.datbas()[3],
    ",
  0x5035f244u64 => "
      I_3_C_NS.datbas()[4],
    ",
  0x5035f24cu64 => "
      I_3_C_NS.datbas()[5],
    ",
  0x5035f254u64 => "
      I_3_C_NS.datbas()[6],
    ",
  0x5035f25cu64 => "
      I_3_C_NS.datbas()[7],
    ",
  0x5035f2a0u64 => "
      I_3_C_NS.exdatbas(),
    ",
  0x5035f2b0u64 => "
      I_3_C_NS.sdatbas0(),
      I_3_C_NS.sdatbas1(),
      I_3_C_NS.sdatbas2(),
    ",
  0x5035f2d0u64 => "
      I_3_C_NS.msdct()[0],
    ",
  0x5035f2d4u64 => "
      I_3_C_NS.msdct()[1],
    ",
  0x5035f2d8u64 => "
      I_3_C_NS.msdct()[2],
    ",
  0x5035f2dcu64 => "
      I_3_C_NS.msdct()[3],
    ",
  0x5035f2e0u64 => "
      I_3_C_NS.msdct()[4],
    ",
  0x5035f2e4u64 => "
      I_3_C_NS.msdct()[5],
    ",
  0x5035f2e8u64 => "
      I_3_C_NS.msdct()[6],
    ",
  0x5035f2ecu64 => "
      I_3_C_NS.msdct()[7],
    ",
  0x5035f320u64 => "
      I_3_C_NS.svdct(),
    ",
  0x5035f324u64 => "
      I_3_C_NS.sdctpidl(),
    ",
  0x5035f328u64 => "
      I_3_C_NS.sdctpidh(),
    ",
  0x5035f330u64 => "
      I_3_C_NS.svdvad()[0],
    ",
  0x5035f334u64 => "
      I_3_C_NS.svdvad()[1],
    ",
  0x5035f338u64 => "
      I_3_C_NS.svdvad()[2],
    ",
  0x5035f350u64 => "
      I_3_C_NS.csecmd(),
    ",
  0x5035f354u64 => "
      I_3_C_NS.ceactst(),
    ",
  0x5035f358u64 => "
      I_3_C_NS.cmwlg(),
    ",
  0x5035f35cu64 => "
      I_3_C_NS.cmrlg(),
    ",
  0x5035f360u64 => "
      I_3_C_NS.cetstmd(),
    ",
  0x5035f364u64 => "
      I_3_C_NS.cgdvst(),
    ",
  0x5035f368u64 => "
      I_3_C_NS.cmdspw(),
    ",
  0x5035f36cu64 => "
      I_3_C_NS.cmdspr(),
    ",
  0x5035f370u64 => "
      I_3_C_NS.cmdspt(),
    ",
  0x5035f374u64 => "
      I_3_C_NS.cetsm(),
    ",
  0x5035f378u64 => "
      I_3_C_NS.cetss(),
    ",
  0x5035f37cu64 => "
      I_3_C_NS.cghdrcap(),
    ",
  0x5035f380u64 => "
      I_3_C_NS.bitcnt(),
    ",
  0x5035f394u64 => "
      I_3_C_NS.nqstlv(),
    ",
  0x5035f398u64 => "
      I_3_C_NS.ndbstlv0(),
    ",
  0x5035f3c0u64 => "
      I_3_C_NS.nrsqstlv(),
    ",
  0x5035f3c4u64 => "
      I_3_C_NS.hqstlv(),
    ",
  0x5035f3c8u64 => "
      I_3_C_NS.hdbstlv(),
    ",
  0x5035f3ccu64 => "
      I_3_C_NS.prstdbg(),
    ",
  0x5035f3d0u64 => "
      I_3_C_NS.mserrcnt(),
    ",
  0x5035f3e0u64 => "
      I_3_C_NS.sc1cpt(),
    ",
  0x5035f3e4u64 => "
      I_3_C_NS.sc2cpt(),
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
  0x503a0000u64 => "
      ESC_NS.r#type(),
    ",
  0x503a0001u64 => "
      ESC_NS.revision(),
    ",
  0x503a0002u64 => "
      ESC_NS.build(),
    ",
  0x503a0004u64 => "
      ESC_NS.fmmu_num(),
    ",
  0x503a0005u64 => "
      ESC_NS.sync_manager(),
    ",
  0x503a0006u64 => "
      ESC_NS.ram_size(),
    ",
  0x503a0007u64 => "
      ESC_NS.port_desc(),
    ",
  0x503a0008u64 => "
      ESC_NS.feature(),
    ",
  0x503a0010u64 => "
      ESC_NS.station_adr(),
    ",
  0x503a0012u64 => "
      ESC_NS.station_alias(),
    ",
  0x503a0020u64 => "
      ESC_NS.wr_reg_enable(),
    ",
  0x503a0021u64 => "
      ESC_NS.wr_reg_protect(),
    ",
  0x503a0030u64 => "
      ESC_NS.esc_wr_enable(),
    ",
  0x503a0031u64 => "
      ESC_NS.esc_wr_protect(),
    ",
  0x503a0040u64 => "
      ESC_NS.esc_reset_ecat_r(),
      ESC_NS.esc_reset_ecat_w(),
    ",
  0x503a0041u64 => "
      ESC_NS.esc_reset_pdi_r(),
      ESC_NS.esc_reset_pdi_w(),
    ",
  0x503a0100u64 => "
      ESC_NS.esc_dl_control(),
    ",
  0x503a0108u64 => "
      ESC_NS.physical_rw_offset(),
    ",
  0x503a0110u64 => "
      ESC_NS.esc_dl_status(),
    ",
  0x503a0120u64 => "
      ESC_NS.al_control(),
    ",
  0x503a0130u64 => "
      ESC_NS.al_status(),
    ",
  0x503a0134u64 => "
      ESC_NS.al_status_code(),
    ",
  0x503a0138u64 => "
      ESC_NS.run_led_override(),
    ",
  0x503a0139u64 => "
      ESC_NS.err_led_override(),
    ",
  0x503a0140u64 => "
      ESC_NS.pdi_control(),
    ",
  0x503a0141u64 => "
      ESC_NS.esc_config(),
    ",
  0x503a0150u64 => "
      ESC_NS.pdi_config(),
    ",
  0x503a0151u64 => "
      ESC_NS.sync_latch_config(),
    ",
  0x503a0152u64 => "
      ESC_NS.ext_pdi_config(),
    ",
  0x503a0200u64 => "
      ESC_NS.ecat_event_mask(),
    ",
  0x503a0204u64 => "
      ESC_NS.al_event_mask(),
    ",
  0x503a0210u64 => "
      ESC_NS.ecat_event_req(),
    ",
  0x503a0220u64 => "
      ESC_NS.al_event_req(),
    ",
  0x503a0300u64 => "
      ESC_NS.rx_err_count()[0],
    ",
  0x503a0302u64 => "
      ESC_NS.rx_err_count()[1],
    ",
  0x503a0308u64 => "
      ESC_NS.fwd_rx_err_count()[0],
    ",
  0x503a0309u64 => "
      ESC_NS.fwd_rx_err_count()[1],
    ",
  0x503a030cu64 => "
      ESC_NS.ecat_proc_err_count(),
    ",
  0x503a030du64 => "
      ESC_NS.pdi_err_count(),
    ",
  0x503a0310u64 => "
      ESC_NS.lost_link_count()[0],
    ",
  0x503a0311u64 => "
      ESC_NS.lost_link_count()[1],
    ",
  0x503a0400u64 => "
      ESC_NS.wd_divide(),
    ",
  0x503a0410u64 => "
      ESC_NS.wdt_pdi(),
    ",
  0x503a0420u64 => "
      ESC_NS.wdt_data(),
    ",
  0x503a0440u64 => "
      ESC_NS.wds_data(),
    ",
  0x503a0442u64 => "
      ESC_NS.wdc_data(),
    ",
  0x503a0443u64 => "
      ESC_NS.wdc_pdi(),
    ",
  0x503a0500u64 => "
      ESC_NS.eep_conf(),
    ",
  0x503a0501u64 => "
      ESC_NS.eep_state(),
    ",
  0x503a0502u64 => "
      ESC_NS.eep_cont_stat(),
    ",
  0x503a0504u64 => "
      ESC_NS.eep_adr(),
    ",
  0x503a0508u64 => "
      ESC_NS.eep_data(),
    ",
  0x503a0510u64 => "
      ESC_NS.mii_cont_stat(),
    ",
  0x503a0512u64 => "
      ESC_NS.phy_adr(),
    ",
  0x503a0513u64 => "
      ESC_NS.phy_reg_adr(),
    ",
  0x503a0514u64 => "
      ESC_NS.phy_data(),
    ",
  0x503a0516u64 => "
      ESC_NS.mii_ecat_acs_stat(),
    ",
  0x503a0517u64 => "
      ESC_NS.mii_pdi_acs_stat(),
    ",
  0x503a0600u64 => "
      ESC_NS.fmmu_l_start_adr()[0],
    ",
  0x503a0610u64 => "
      ESC_NS.fmmu_l_start_adr()[1],
    ",
  0x503a0620u64 => "
      ESC_NS.fmmu_l_start_adr()[2],
    ",
  0x503a0630u64 => "
      ESC_NS.fmmu_l_start_adr()[3],
    ",
  0x503a0640u64 => "
      ESC_NS.fmmu_l_start_adr()[4],
    ",
  0x503a0650u64 => "
      ESC_NS.fmmu_l_start_adr()[5],
    ",
  0x503a0660u64 => "
      ESC_NS.fmmu_l_start_adr()[6],
    ",
  0x503a0670u64 => "
      ESC_NS.fmmu_l_start_adr()[7],
    ",
  0x503a0604u64 => "
      ESC_NS.fmmu_len()[0],
    ",
  0x503a0614u64 => "
      ESC_NS.fmmu_len()[1],
    ",
  0x503a0624u64 => "
      ESC_NS.fmmu_len()[2],
    ",
  0x503a0634u64 => "
      ESC_NS.fmmu_len()[3],
    ",
  0x503a0644u64 => "
      ESC_NS.fmmu_len()[4],
    ",
  0x503a0654u64 => "
      ESC_NS.fmmu_len()[5],
    ",
  0x503a0664u64 => "
      ESC_NS.fmmu_len()[6],
    ",
  0x503a0674u64 => "
      ESC_NS.fmmu_len()[7],
    ",
  0x503a0606u64 => "
      ESC_NS.fmmu_l_start_bit()[0],
    ",
  0x503a0616u64 => "
      ESC_NS.fmmu_l_start_bit()[1],
    ",
  0x503a0626u64 => "
      ESC_NS.fmmu_l_start_bit()[2],
    ",
  0x503a0636u64 => "
      ESC_NS.fmmu_l_start_bit()[3],
    ",
  0x503a0646u64 => "
      ESC_NS.fmmu_l_start_bit()[4],
    ",
  0x503a0656u64 => "
      ESC_NS.fmmu_l_start_bit()[5],
    ",
  0x503a0666u64 => "
      ESC_NS.fmmu_l_start_bit()[6],
    ",
  0x503a0676u64 => "
      ESC_NS.fmmu_l_start_bit()[7],
    ",
  0x503a0607u64 => "
      ESC_NS.fmmu_l_stop_bit()[0],
    ",
  0x503a0617u64 => "
      ESC_NS.fmmu_l_stop_bit()[1],
    ",
  0x503a0627u64 => "
      ESC_NS.fmmu_l_stop_bit()[2],
    ",
  0x503a0637u64 => "
      ESC_NS.fmmu_l_stop_bit()[3],
    ",
  0x503a0647u64 => "
      ESC_NS.fmmu_l_stop_bit()[4],
    ",
  0x503a0657u64 => "
      ESC_NS.fmmu_l_stop_bit()[5],
    ",
  0x503a0667u64 => "
      ESC_NS.fmmu_l_stop_bit()[6],
    ",
  0x503a0677u64 => "
      ESC_NS.fmmu_l_stop_bit()[7],
    ",
  0x503a0608u64 => "
      ESC_NS.fmmu_p_start_adr()[0],
    ",
  0x503a0618u64 => "
      ESC_NS.fmmu_p_start_adr()[1],
    ",
  0x503a0628u64 => "
      ESC_NS.fmmu_p_start_adr()[2],
    ",
  0x503a0638u64 => "
      ESC_NS.fmmu_p_start_adr()[3],
    ",
  0x503a0648u64 => "
      ESC_NS.fmmu_p_start_adr()[4],
    ",
  0x503a0658u64 => "
      ESC_NS.fmmu_p_start_adr()[5],
    ",
  0x503a0668u64 => "
      ESC_NS.fmmu_p_start_adr()[6],
    ",
  0x503a0678u64 => "
      ESC_NS.fmmu_p_start_adr()[7],
    ",
  0x503a060au64 => "
      ESC_NS.fmmu_p_start_bit()[0],
    ",
  0x503a061au64 => "
      ESC_NS.fmmu_p_start_bit()[1],
    ",
  0x503a062au64 => "
      ESC_NS.fmmu_p_start_bit()[2],
    ",
  0x503a063au64 => "
      ESC_NS.fmmu_p_start_bit()[3],
    ",
  0x503a064au64 => "
      ESC_NS.fmmu_p_start_bit()[4],
    ",
  0x503a065au64 => "
      ESC_NS.fmmu_p_start_bit()[5],
    ",
  0x503a066au64 => "
      ESC_NS.fmmu_p_start_bit()[6],
    ",
  0x503a067au64 => "
      ESC_NS.fmmu_p_start_bit()[7],
    ",
  0x503a060bu64 => "
      ESC_NS.fmmu_type()[0],
    ",
  0x503a061bu64 => "
      ESC_NS.fmmu_type()[1],
    ",
  0x503a062bu64 => "
      ESC_NS.fmmu_type()[2],
    ",
  0x503a063bu64 => "
      ESC_NS.fmmu_type()[3],
    ",
  0x503a064bu64 => "
      ESC_NS.fmmu_type()[4],
    ",
  0x503a065bu64 => "
      ESC_NS.fmmu_type()[5],
    ",
  0x503a066bu64 => "
      ESC_NS.fmmu_type()[6],
    ",
  0x503a067bu64 => "
      ESC_NS.fmmu_type()[7],
    ",
  0x503a060cu64 => "
      ESC_NS.fmmu_act()[0],
    ",
  0x503a061cu64 => "
      ESC_NS.fmmu_act()[1],
    ",
  0x503a062cu64 => "
      ESC_NS.fmmu_act()[2],
    ",
  0x503a063cu64 => "
      ESC_NS.fmmu_act()[3],
    ",
  0x503a064cu64 => "
      ESC_NS.fmmu_act()[4],
    ",
  0x503a065cu64 => "
      ESC_NS.fmmu_act()[5],
    ",
  0x503a066cu64 => "
      ESC_NS.fmmu_act()[6],
    ",
  0x503a067cu64 => "
      ESC_NS.fmmu_act()[7],
    ",
  0x503a0800u64 => "
      ESC_NS.sm_p_start_adr()[0],
    ",
  0x503a0808u64 => "
      ESC_NS.sm_p_start_adr()[1],
    ",
  0x503a0810u64 => "
      ESC_NS.sm_p_start_adr()[2],
    ",
  0x503a0818u64 => "
      ESC_NS.sm_p_start_adr()[3],
    ",
  0x503a0820u64 => "
      ESC_NS.sm_p_start_adr()[4],
    ",
  0x503a0828u64 => "
      ESC_NS.sm_p_start_adr()[5],
    ",
  0x503a0830u64 => "
      ESC_NS.sm_p_start_adr()[6],
    ",
  0x503a0838u64 => "
      ESC_NS.sm_p_start_adr()[7],
    ",
  0x503a0802u64 => "
      ESC_NS.sm_len()[0],
    ",
  0x503a080au64 => "
      ESC_NS.sm_len()[1],
    ",
  0x503a0812u64 => "
      ESC_NS.sm_len()[2],
    ",
  0x503a081au64 => "
      ESC_NS.sm_len()[3],
    ",
  0x503a0822u64 => "
      ESC_NS.sm_len()[4],
    ",
  0x503a082au64 => "
      ESC_NS.sm_len()[5],
    ",
  0x503a0832u64 => "
      ESC_NS.sm_len()[6],
    ",
  0x503a083au64 => "
      ESC_NS.sm_len()[7],
    ",
  0x503a0804u64 => "
      ESC_NS.sm_control()[0],
    ",
  0x503a080cu64 => "
      ESC_NS.sm_control()[1],
    ",
  0x503a0814u64 => "
      ESC_NS.sm_control()[2],
    ",
  0x503a081cu64 => "
      ESC_NS.sm_control()[3],
    ",
  0x503a0824u64 => "
      ESC_NS.sm_control()[4],
    ",
  0x503a082cu64 => "
      ESC_NS.sm_control()[5],
    ",
  0x503a0834u64 => "
      ESC_NS.sm_control()[6],
    ",
  0x503a083cu64 => "
      ESC_NS.sm_control()[7],
    ",
  0x503a0805u64 => "
      ESC_NS.sm_status()[0],
    ",
  0x503a080du64 => "
      ESC_NS.sm_status()[1],
    ",
  0x503a0815u64 => "
      ESC_NS.sm_status()[2],
    ",
  0x503a081du64 => "
      ESC_NS.sm_status()[3],
    ",
  0x503a0825u64 => "
      ESC_NS.sm_status()[4],
    ",
  0x503a082du64 => "
      ESC_NS.sm_status()[5],
    ",
  0x503a0835u64 => "
      ESC_NS.sm_status()[6],
    ",
  0x503a083du64 => "
      ESC_NS.sm_status()[7],
    ",
  0x503a0806u64 => "
      ESC_NS.sm_act()[0],
    ",
  0x503a080eu64 => "
      ESC_NS.sm_act()[1],
    ",
  0x503a0816u64 => "
      ESC_NS.sm_act()[2],
    ",
  0x503a081eu64 => "
      ESC_NS.sm_act()[3],
    ",
  0x503a0826u64 => "
      ESC_NS.sm_act()[4],
    ",
  0x503a082eu64 => "
      ESC_NS.sm_act()[5],
    ",
  0x503a0836u64 => "
      ESC_NS.sm_act()[6],
    ",
  0x503a083eu64 => "
      ESC_NS.sm_act()[7],
    ",
  0x503a0807u64 => "
      ESC_NS.sm_pdi_cont()[0],
    ",
  0x503a080fu64 => "
      ESC_NS.sm_pdi_cont()[1],
    ",
  0x503a0817u64 => "
      ESC_NS.sm_pdi_cont()[2],
    ",
  0x503a081fu64 => "
      ESC_NS.sm_pdi_cont()[3],
    ",
  0x503a0827u64 => "
      ESC_NS.sm_pdi_cont()[4],
    ",
  0x503a082fu64 => "
      ESC_NS.sm_pdi_cont()[5],
    ",
  0x503a0837u64 => "
      ESC_NS.sm_pdi_cont()[6],
    ",
  0x503a083fu64 => "
      ESC_NS.sm_pdi_cont()[7],
    ",
  0x503a0900u64 => "
      ESC_NS.dc_rcv_time_port0(),
    ",
  0x503a0904u64 => "
      ESC_NS.dc_rcv_time_port1(),
    ",
  0x503a0908u64 => "
      ESC_NS.dc_rcv_time_port2(),
    ",
  0x503a0910u64 => "
      ESC_NS.dc_sys_time_l(),
    ",
  0x503a0914u64 => "
      ESC_NS.dc_sys_time_h(),
    ",
  0x503a0918u64 => "
      ESC_NS.dc_rcv_time_unit_l(),
    ",
  0x503a091cu64 => "
      ESC_NS.dc_rcv_time_unit_h(),
    ",
  0x503a0920u64 => "
      ESC_NS.dc_sys_time_offset_l(),
    ",
  0x503a0924u64 => "
      ESC_NS.dc_sys_time_offset_h(),
    ",
  0x503a0928u64 => "
      ESC_NS.dc_sys_time_delay(),
    ",
  0x503a092cu64 => "
      ESC_NS.dc_sys_time_diff(),
    ",
  0x503a0930u64 => "
      ESC_NS.dc_speed_count_start(),
    ",
  0x503a0932u64 => "
      ESC_NS.dc_speed_count_diff(),
    ",
  0x503a0934u64 => "
      ESC_NS.dc_sys_time_diff_fil_depth(),
    ",
  0x503a0935u64 => "
      ESC_NS.dc_speed_count_fil_depth(),
    ",
  0x503a0980u64 => "
      ESC_NS.dc_cyc_cont(),
    ",
  0x503a0981u64 => "
      ESC_NS.dc_act(),
    ",
  0x503a0982u64 => "
      ESC_NS.dc_pulse_len(),
    ",
  0x503a0984u64 => "
      ESC_NS.dc_act_stat(),
    ",
  0x503a098eu64 => "
      ESC_NS.dc_sync0_stat(),
    ",
  0x503a098fu64 => "
      ESC_NS.dc_sync1_stat(),
    ",
  0x503a0990u64 => "
      ESC_NS.dc_cyc_start_time_l(),
    ",
  0x503a0994u64 => "
      ESC_NS.dc_cyc_start_time_h(),
    ",
  0x503a0998u64 => "
      ESC_NS.dc_next_sync1_pulse_l(),
    ",
  0x503a099cu64 => "
      ESC_NS.dc_next_sync1_pulse_h(),
    ",
  0x503a09a0u64 => "
      ESC_NS.dc_sync0_cyc_time(),
    ",
  0x503a09a4u64 => "
      ESC_NS.dc_sync1_cyc_time(),
    ",
  0x503a09a8u64 => "
      ESC_NS.dc_latch0_cont(),
    ",
  0x503a09a9u64 => "
      ESC_NS.dc_latch1_cont(),
    ",
  0x503a09aeu64 => "
      ESC_NS.dc_latch0_stat(),
    ",
  0x503a09afu64 => "
      ESC_NS.dc_latch1_stat(),
    ",
  0x503a09b0u64 => "
      ESC_NS.dc_latch0_time_pos_l(),
    ",
  0x503a09b4u64 => "
      ESC_NS.dc_latch0_time_pos_h(),
    ",
  0x503a09b8u64 => "
      ESC_NS.dc_latch0_time_neg_l(),
    ",
  0x503a09bcu64 => "
      ESC_NS.dc_latch0_time_neg_h(),
    ",
  0x503a09c0u64 => "
      ESC_NS.dc_latch1_time_pos_l(),
    ",
  0x503a09c4u64 => "
      ESC_NS.dc_latch1_time_pos_h(),
    ",
  0x503a09c8u64 => "
      ESC_NS.dc_latch1_time_neg_l(),
    ",
  0x503a09ccu64 => "
      ESC_NS.dc_latch1_time_neg_h(),
    ",
  0x503a09f0u64 => "
      ESC_NS.dc_ecat_cng_ev_time(),
    ",
  0x503a09f8u64 => "
      ESC_NS.dc_pdi_start_ev_time(),
    ",
  0x503a09fcu64 => "
      ESC_NS.dc_pdi_cng_ev_time(),
    ",
  0x503a0e00u64 => "
      ESC_NS.product_id_l(),
    ",
  0x503a0e04u64 => "
      ESC_NS.product_id_h(),
    ",
  0x503a0e08u64 => "
      ESC_NS.vendor_id_l(),
    ",
  0x503a4000u64 => "
      ESC_INI_NS.escrst(),
    ",
  0x503a4010u64 => "
      ESC_INI_NS.phylink(),
    ",
  0x503a4014u64 => "
      ESC_INI_NS.escicr(),
    ",
  0x503a4018u64 => "
      ESC_INI_NS.ecatoffadr(),
    ",
  0x503a401cu64 => "
      ESC_INI_NS.ecatopmod(),
    ",
  0x503a4020u64 => "
      ESC_INI_NS.ecatdbgc(),
    ",
  0x503c0000u64 => "
      MFWD_NS.fwgc(),
    ",
  0x503c0010u64 => "
      MFWD_NS.fwttc0(),
    ",
  0x503c0014u64 => "
      MFWD_NS.fwttc1(),
    ",
  0x503c0020u64 => "
      MFWD_NS.fwceptc(),
    ",
  0x503c0024u64 => "
      MFWD_NS.fwceprc0(),
    ",
  0x503c0028u64 => "
      MFWD_NS.fwceprc1(),
    ",
  0x503c002cu64 => "
      MFWD_NS.fwceprc2(),
    ",
  0x503c0030u64 => "
      MFWD_NS.fwclptc(),
    ",
  0x503c0034u64 => "
      MFWD_NS.fwclprc(),
    ",
  0x503c0040u64 => "
      MFWD_NS.fwcmptc(),
    ",
  0x503c0044u64 => "
      MFWD_NS.fwemptc(),
    ",
  0x503c0050u64 => "
      MFWD_NS.fwsdmptc(),
    ",
  0x503c0054u64 => "
      MFWD_NS.fwsdmpvc(),
    ",
  0x503c0080u64 => "
      MFWD_NS.fwlbwmc()[0],
    ",
  0x503c0084u64 => "
      MFWD_NS.fwlbwmc()[1],
    ",
  0x503c0088u64 => "
      MFWD_NS.fwlbwmc()[2],
    ",
  0x503c0100u64 => "
      MFWD_NS.fwpc0()[0],
    ",
  0x503c0110u64 => "
      MFWD_NS.fwpc0()[1],
    ",
  0x503c0120u64 => "
      MFWD_NS.fwpc0()[2],
    ",
  0x503c0104u64 => "
      MFWD_NS.fwpc1()[0],
    ",
  0x503c0114u64 => "
      MFWD_NS.fwpc1()[1],
    ",
  0x503c0124u64 => "
      MFWD_NS.fwpc1()[2],
    ",
  0x503c0108u64 => "
      MFWD_NS.fwpc2()[0],
    ",
  0x503c0118u64 => "
      MFWD_NS.fwpc2()[1],
    ",
  0x503c0128u64 => "
      MFWD_NS.fwpc2()[2],
    ",
  0x503c0400u64 => "
      MFWD_NS.fwctgc0()[0],
    ",
  0x503c0440u64 => "
      MFWD_NS.fwctgc0()[1],
    ",
  0x503c0480u64 => "
      MFWD_NS.fwctgc0()[2],
    ",
  0x503c04c0u64 => "
      MFWD_NS.fwctgc0()[3],
    ",
  0x503c0500u64 => "
      MFWD_NS.fwctgc0()[4],
    ",
  0x503c0540u64 => "
      MFWD_NS.fwctgc0()[5],
    ",
  0x503c0580u64 => "
      MFWD_NS.fwctgc0()[6],
    ",
  0x503c05c0u64 => "
      MFWD_NS.fwctgc0()[7],
    ",
  0x503c0404u64 => "
      MFWD_NS.fwctgc1()[0],
    ",
  0x503c0444u64 => "
      MFWD_NS.fwctgc1()[1],
    ",
  0x503c0484u64 => "
      MFWD_NS.fwctgc1()[2],
    ",
  0x503c04c4u64 => "
      MFWD_NS.fwctgc1()[3],
    ",
  0x503c0504u64 => "
      MFWD_NS.fwctgc1()[4],
    ",
  0x503c0544u64 => "
      MFWD_NS.fwctgc1()[5],
    ",
  0x503c0584u64 => "
      MFWD_NS.fwctgc1()[6],
    ",
  0x503c05c4u64 => "
      MFWD_NS.fwctgc1()[7],
    ",
  0x503c0408u64 => "
      MFWD_NS.fwcttc0()[0],
    ",
  0x503c0448u64 => "
      MFWD_NS.fwcttc0()[1],
    ",
  0x503c0488u64 => "
      MFWD_NS.fwcttc0()[2],
    ",
  0x503c04c8u64 => "
      MFWD_NS.fwcttc0()[3],
    ",
  0x503c0508u64 => "
      MFWD_NS.fwcttc0()[4],
    ",
  0x503c0548u64 => "
      MFWD_NS.fwcttc0()[5],
    ",
  0x503c0588u64 => "
      MFWD_NS.fwcttc0()[6],
    ",
  0x503c05c8u64 => "
      MFWD_NS.fwcttc0()[7],
    ",
  0x503c040cu64 => "
      MFWD_NS.fwcttc1()[0],
    ",
  0x503c044cu64 => "
      MFWD_NS.fwcttc1()[1],
    ",
  0x503c048cu64 => "
      MFWD_NS.fwcttc1()[2],
    ",
  0x503c04ccu64 => "
      MFWD_NS.fwcttc1()[3],
    ",
  0x503c050cu64 => "
      MFWD_NS.fwcttc1()[4],
    ",
  0x503c054cu64 => "
      MFWD_NS.fwcttc1()[5],
    ",
  0x503c058cu64 => "
      MFWD_NS.fwcttc1()[6],
    ",
  0x503c05ccu64 => "
      MFWD_NS.fwcttc1()[7],
    ",
  0x503c0410u64 => "
      MFWD_NS.fwcttc20()[0],
    ",
  0x503c0450u64 => "
      MFWD_NS.fwcttc20()[1],
    ",
  0x503c0490u64 => "
      MFWD_NS.fwcttc20()[2],
    ",
  0x503c04d0u64 => "
      MFWD_NS.fwcttc20()[3],
    ",
  0x503c0510u64 => "
      MFWD_NS.fwcttc20()[4],
    ",
  0x503c0550u64 => "
      MFWD_NS.fwcttc20()[5],
    ",
  0x503c0590u64 => "
      MFWD_NS.fwcttc20()[6],
    ",
  0x503c05d0u64 => "
      MFWD_NS.fwcttc20()[7],
    ",
  0x503c0420u64 => "
      MFWD_NS.fwctsc0()[0],
    ",
  0x503c0460u64 => "
      MFWD_NS.fwctsc0()[1],
    ",
  0x503c04a0u64 => "
      MFWD_NS.fwctsc0()[2],
    ",
  0x503c04e0u64 => "
      MFWD_NS.fwctsc0()[3],
    ",
  0x503c0520u64 => "
      MFWD_NS.fwctsc0()[4],
    ",
  0x503c0560u64 => "
      MFWD_NS.fwctsc0()[5],
    ",
  0x503c05a0u64 => "
      MFWD_NS.fwctsc0()[6],
    ",
  0x503c05e0u64 => "
      MFWD_NS.fwctsc0()[7],
    ",
  0x503c0424u64 => "
      MFWD_NS.fwctsc1()[0],
    ",
  0x503c0464u64 => "
      MFWD_NS.fwctsc1()[1],
    ",
  0x503c04a4u64 => "
      MFWD_NS.fwctsc1()[2],
    ",
  0x503c04e4u64 => "
      MFWD_NS.fwctsc1()[3],
    ",
  0x503c0524u64 => "
      MFWD_NS.fwctsc1()[4],
    ",
  0x503c0564u64 => "
      MFWD_NS.fwctsc1()[5],
    ",
  0x503c05a4u64 => "
      MFWD_NS.fwctsc1()[6],
    ",
  0x503c05e4u64 => "
      MFWD_NS.fwctsc1()[7],
    ",
  0x503c0428u64 => "
      MFWD_NS.fwctsc2()[0],
    ",
  0x503c0468u64 => "
      MFWD_NS.fwctsc2()[1],
    ",
  0x503c04a8u64 => "
      MFWD_NS.fwctsc2()[2],
    ",
  0x503c04e8u64 => "
      MFWD_NS.fwctsc2()[3],
    ",
  0x503c0528u64 => "
      MFWD_NS.fwctsc2()[4],
    ",
  0x503c0568u64 => "
      MFWD_NS.fwctsc2()[5],
    ",
  0x503c05a8u64 => "
      MFWD_NS.fwctsc2()[6],
    ",
  0x503c05e8u64 => "
      MFWD_NS.fwctsc2()[7],
    ",
  0x503c042cu64 => "
      MFWD_NS.fwctsc3()[0],
    ",
  0x503c046cu64 => "
      MFWD_NS.fwctsc3()[1],
    ",
  0x503c04acu64 => "
      MFWD_NS.fwctsc3()[2],
    ",
  0x503c04ecu64 => "
      MFWD_NS.fwctsc3()[3],
    ",
  0x503c052cu64 => "
      MFWD_NS.fwctsc3()[4],
    ",
  0x503c056cu64 => "
      MFWD_NS.fwctsc3()[5],
    ",
  0x503c05acu64 => "
      MFWD_NS.fwctsc3()[6],
    ",
  0x503c05ecu64 => "
      MFWD_NS.fwctsc3()[7],
    ",
  0x503c0430u64 => "
      MFWD_NS.fwctsc4()[0],
    ",
  0x503c0470u64 => "
      MFWD_NS.fwctsc4()[1],
    ",
  0x503c04b0u64 => "
      MFWD_NS.fwctsc4()[2],
    ",
  0x503c04f0u64 => "
      MFWD_NS.fwctsc4()[3],
    ",
  0x503c0530u64 => "
      MFWD_NS.fwctsc4()[4],
    ",
  0x503c0570u64 => "
      MFWD_NS.fwctsc4()[5],
    ",
  0x503c05b0u64 => "
      MFWD_NS.fwctsc4()[6],
    ",
  0x503c05f0u64 => "
      MFWD_NS.fwctsc4()[7],
    ",
  0x503c1000u64 => "
      MFWD_NS.fwtwbfc()[0],
    ",
  0x503c1010u64 => "
      MFWD_NS.fwtwbfc()[1],
    ",
  0x503c1020u64 => "
      MFWD_NS.fwtwbfc()[2],
    ",
  0x503c1030u64 => "
      MFWD_NS.fwtwbfc()[3],
    ",
  0x503c1040u64 => "
      MFWD_NS.fwtwbfc()[4],
    ",
  0x503c1050u64 => "
      MFWD_NS.fwtwbfc()[5],
    ",
  0x503c1060u64 => "
      MFWD_NS.fwtwbfc()[6],
    ",
  0x503c1070u64 => "
      MFWD_NS.fwtwbfc()[7],
    ",
  0x503c1080u64 => "
      MFWD_NS.fwtwbfc()[8],
    ",
  0x503c1090u64 => "
      MFWD_NS.fwtwbfc()[9],
    ",
  0x503c10a0u64 => "
      MFWD_NS.fwtwbfc()[10],
    ",
  0x503c10b0u64 => "
      MFWD_NS.fwtwbfc()[11],
    ",
  0x503c10c0u64 => "
      MFWD_NS.fwtwbfc()[12],
    ",
  0x503c10d0u64 => "
      MFWD_NS.fwtwbfc()[13],
    ",
  0x503c10e0u64 => "
      MFWD_NS.fwtwbfc()[14],
    ",
  0x503c10f0u64 => "
      MFWD_NS.fwtwbfc()[15],
    ",
  0x503c1004u64 => "
      MFWD_NS.fwtwbfvc()[0],
    ",
  0x503c1014u64 => "
      MFWD_NS.fwtwbfvc()[1],
    ",
  0x503c1024u64 => "
      MFWD_NS.fwtwbfvc()[2],
    ",
  0x503c1034u64 => "
      MFWD_NS.fwtwbfvc()[3],
    ",
  0x503c1044u64 => "
      MFWD_NS.fwtwbfvc()[4],
    ",
  0x503c1054u64 => "
      MFWD_NS.fwtwbfvc()[5],
    ",
  0x503c1064u64 => "
      MFWD_NS.fwtwbfvc()[6],
    ",
  0x503c1074u64 => "
      MFWD_NS.fwtwbfvc()[7],
    ",
  0x503c1084u64 => "
      MFWD_NS.fwtwbfvc()[8],
    ",
  0x503c1094u64 => "
      MFWD_NS.fwtwbfvc()[9],
    ",
  0x503c10a4u64 => "
      MFWD_NS.fwtwbfvc()[10],
    ",
  0x503c10b4u64 => "
      MFWD_NS.fwtwbfvc()[11],
    ",
  0x503c10c4u64 => "
      MFWD_NS.fwtwbfvc()[12],
    ",
  0x503c10d4u64 => "
      MFWD_NS.fwtwbfvc()[13],
    ",
  0x503c10e4u64 => "
      MFWD_NS.fwtwbfvc()[14],
    ",
  0x503c10f4u64 => "
      MFWD_NS.fwtwbfvc()[15],
    ",
  0x503c1400u64 => "
      MFWD_NS.fwthbfc()[0],
    ",
  0x503c1410u64 => "
      MFWD_NS.fwthbfc()[1],
    ",
  0x503c1420u64 => "
      MFWD_NS.fwthbfc()[2],
    ",
  0x503c1430u64 => "
      MFWD_NS.fwthbfc()[3],
    ",
  0x503c1440u64 => "
      MFWD_NS.fwthbfc()[4],
    ",
  0x503c1450u64 => "
      MFWD_NS.fwthbfc()[5],
    ",
  0x503c1460u64 => "
      MFWD_NS.fwthbfc()[6],
    ",
  0x503c1470u64 => "
      MFWD_NS.fwthbfc()[7],
    ",
  0x503c1480u64 => "
      MFWD_NS.fwthbfc()[8],
    ",
  0x503c1490u64 => "
      MFWD_NS.fwthbfc()[9],
    ",
  0x503c14a0u64 => "
      MFWD_NS.fwthbfc()[10],
    ",
  0x503c14b0u64 => "
      MFWD_NS.fwthbfc()[11],
    ",
  0x503c14c0u64 => "
      MFWD_NS.fwthbfc()[12],
    ",
  0x503c14d0u64 => "
      MFWD_NS.fwthbfc()[13],
    ",
  0x503c14e0u64 => "
      MFWD_NS.fwthbfc()[14],
    ",
  0x503c14f0u64 => "
      MFWD_NS.fwthbfc()[15],
    ",
  0x503c1404u64 => "
      MFWD_NS.fwthbfv0c()[0],
    ",
  0x503c1414u64 => "
      MFWD_NS.fwthbfv0c()[1],
    ",
  0x503c1424u64 => "
      MFWD_NS.fwthbfv0c()[2],
    ",
  0x503c1434u64 => "
      MFWD_NS.fwthbfv0c()[3],
    ",
  0x503c1444u64 => "
      MFWD_NS.fwthbfv0c()[4],
    ",
  0x503c1454u64 => "
      MFWD_NS.fwthbfv0c()[5],
    ",
  0x503c1464u64 => "
      MFWD_NS.fwthbfv0c()[6],
    ",
  0x503c1474u64 => "
      MFWD_NS.fwthbfv0c()[7],
    ",
  0x503c1484u64 => "
      MFWD_NS.fwthbfv0c()[8],
    ",
  0x503c1494u64 => "
      MFWD_NS.fwthbfv0c()[9],
    ",
  0x503c14a4u64 => "
      MFWD_NS.fwthbfv0c()[10],
    ",
  0x503c14b4u64 => "
      MFWD_NS.fwthbfv0c()[11],
    ",
  0x503c14c4u64 => "
      MFWD_NS.fwthbfv0c()[12],
    ",
  0x503c14d4u64 => "
      MFWD_NS.fwthbfv0c()[13],
    ",
  0x503c14e4u64 => "
      MFWD_NS.fwthbfv0c()[14],
    ",
  0x503c14f4u64 => "
      MFWD_NS.fwthbfv0c()[15],
    ",
  0x503c1408u64 => "
      MFWD_NS.fwthbfv1c()[0],
    ",
  0x503c1418u64 => "
      MFWD_NS.fwthbfv1c()[1],
    ",
  0x503c1428u64 => "
      MFWD_NS.fwthbfv1c()[2],
    ",
  0x503c1438u64 => "
      MFWD_NS.fwthbfv1c()[3],
    ",
  0x503c1448u64 => "
      MFWD_NS.fwthbfv1c()[4],
    ",
  0x503c1458u64 => "
      MFWD_NS.fwthbfv1c()[5],
    ",
  0x503c1468u64 => "
      MFWD_NS.fwthbfv1c()[6],
    ",
  0x503c1478u64 => "
      MFWD_NS.fwthbfv1c()[7],
    ",
  0x503c1488u64 => "
      MFWD_NS.fwthbfv1c()[8],
    ",
  0x503c1498u64 => "
      MFWD_NS.fwthbfv1c()[9],
    ",
  0x503c14a8u64 => "
      MFWD_NS.fwthbfv1c()[10],
    ",
  0x503c14b8u64 => "
      MFWD_NS.fwthbfv1c()[11],
    ",
  0x503c14c8u64 => "
      MFWD_NS.fwthbfv1c()[12],
    ",
  0x503c14d8u64 => "
      MFWD_NS.fwthbfv1c()[13],
    ",
  0x503c14e8u64 => "
      MFWD_NS.fwthbfv1c()[14],
    ",
  0x503c14f8u64 => "
      MFWD_NS.fwthbfv1c()[15],
    ",
  0x503c1800u64 => "
      MFWD_NS.fwfobfc()[0],
    ",
  0x503c1810u64 => "
      MFWD_NS.fwfobfc()[1],
    ",
  0x503c1820u64 => "
      MFWD_NS.fwfobfc()[2],
    ",
  0x503c1830u64 => "
      MFWD_NS.fwfobfc()[3],
    ",
  0x503c1840u64 => "
      MFWD_NS.fwfobfc()[4],
    ",
  0x503c1850u64 => "
      MFWD_NS.fwfobfc()[5],
    ",
  0x503c1860u64 => "
      MFWD_NS.fwfobfc()[6],
    ",
  0x503c1870u64 => "
      MFWD_NS.fwfobfc()[7],
    ",
  0x503c1880u64 => "
      MFWD_NS.fwfobfc()[8],
    ",
  0x503c1890u64 => "
      MFWD_NS.fwfobfc()[9],
    ",
  0x503c18a0u64 => "
      MFWD_NS.fwfobfc()[10],
    ",
  0x503c18b0u64 => "
      MFWD_NS.fwfobfc()[11],
    ",
  0x503c18c0u64 => "
      MFWD_NS.fwfobfc()[12],
    ",
  0x503c18d0u64 => "
      MFWD_NS.fwfobfc()[13],
    ",
  0x503c18e0u64 => "
      MFWD_NS.fwfobfc()[14],
    ",
  0x503c18f0u64 => "
      MFWD_NS.fwfobfc()[15],
    ",
  0x503c1804u64 => "
      MFWD_NS.fwfobfv0c()[0],
    ",
  0x503c1814u64 => "
      MFWD_NS.fwfobfv0c()[1],
    ",
  0x503c1824u64 => "
      MFWD_NS.fwfobfv0c()[2],
    ",
  0x503c1834u64 => "
      MFWD_NS.fwfobfv0c()[3],
    ",
  0x503c1844u64 => "
      MFWD_NS.fwfobfv0c()[4],
    ",
  0x503c1854u64 => "
      MFWD_NS.fwfobfv0c()[5],
    ",
  0x503c1864u64 => "
      MFWD_NS.fwfobfv0c()[6],
    ",
  0x503c1874u64 => "
      MFWD_NS.fwfobfv0c()[7],
    ",
  0x503c1884u64 => "
      MFWD_NS.fwfobfv0c()[8],
    ",
  0x503c1894u64 => "
      MFWD_NS.fwfobfv0c()[9],
    ",
  0x503c18a4u64 => "
      MFWD_NS.fwfobfv0c()[10],
    ",
  0x503c18b4u64 => "
      MFWD_NS.fwfobfv0c()[11],
    ",
  0x503c18c4u64 => "
      MFWD_NS.fwfobfv0c()[12],
    ",
  0x503c18d4u64 => "
      MFWD_NS.fwfobfv0c()[13],
    ",
  0x503c18e4u64 => "
      MFWD_NS.fwfobfv0c()[14],
    ",
  0x503c18f4u64 => "
      MFWD_NS.fwfobfv0c()[15],
    ",
  0x503c1808u64 => "
      MFWD_NS.fwfobfv1c()[0],
    ",
  0x503c1818u64 => "
      MFWD_NS.fwfobfv1c()[1],
    ",
  0x503c1828u64 => "
      MFWD_NS.fwfobfv1c()[2],
    ",
  0x503c1838u64 => "
      MFWD_NS.fwfobfv1c()[3],
    ",
  0x503c1848u64 => "
      MFWD_NS.fwfobfv1c()[4],
    ",
  0x503c1858u64 => "
      MFWD_NS.fwfobfv1c()[5],
    ",
  0x503c1868u64 => "
      MFWD_NS.fwfobfv1c()[6],
    ",
  0x503c1878u64 => "
      MFWD_NS.fwfobfv1c()[7],
    ",
  0x503c1888u64 => "
      MFWD_NS.fwfobfv1c()[8],
    ",
  0x503c1898u64 => "
      MFWD_NS.fwfobfv1c()[9],
    ",
  0x503c18a8u64 => "
      MFWD_NS.fwfobfv1c()[10],
    ",
  0x503c18b8u64 => "
      MFWD_NS.fwfobfv1c()[11],
    ",
  0x503c18c8u64 => "
      MFWD_NS.fwfobfv1c()[12],
    ",
  0x503c18d8u64 => "
      MFWD_NS.fwfobfv1c()[13],
    ",
  0x503c18e8u64 => "
      MFWD_NS.fwfobfv1c()[14],
    ",
  0x503c18f8u64 => "
      MFWD_NS.fwfobfv1c()[15],
    ",
  0x503c1c00u64 => "
      MFWD_NS.fwrfc()[0],
    ",
  0x503c1c10u64 => "
      MFWD_NS.fwrfc()[1],
    ",
  0x503c1c20u64 => "
      MFWD_NS.fwrfc()[2],
    ",
  0x503c1c30u64 => "
      MFWD_NS.fwrfc()[3],
    ",
  0x503c1c40u64 => "
      MFWD_NS.fwrfc()[4],
    ",
  0x503c1c50u64 => "
      MFWD_NS.fwrfc()[5],
    ",
  0x503c1c60u64 => "
      MFWD_NS.fwrfc()[6],
    ",
  0x503c1c70u64 => "
      MFWD_NS.fwrfc()[7],
    ",
  0x503c1c80u64 => "
      MFWD_NS.fwrfc()[8],
    ",
  0x503c1c90u64 => "
      MFWD_NS.fwrfc()[9],
    ",
  0x503c1ca0u64 => "
      MFWD_NS.fwrfc()[10],
    ",
  0x503c1cb0u64 => "
      MFWD_NS.fwrfc()[11],
    ",
  0x503c1cc0u64 => "
      MFWD_NS.fwrfc()[12],
    ",
  0x503c1cd0u64 => "
      MFWD_NS.fwrfc()[13],
    ",
  0x503c1ce0u64 => "
      MFWD_NS.fwrfc()[14],
    ",
  0x503c1cf0u64 => "
      MFWD_NS.fwrfc()[15],
    ",
  0x503c1c04u64 => "
      MFWD_NS.fwrfvc()[0],
    ",
  0x503c1c14u64 => "
      MFWD_NS.fwrfvc()[1],
    ",
  0x503c1c24u64 => "
      MFWD_NS.fwrfvc()[2],
    ",
  0x503c1c34u64 => "
      MFWD_NS.fwrfvc()[3],
    ",
  0x503c1c44u64 => "
      MFWD_NS.fwrfvc()[4],
    ",
  0x503c1c54u64 => "
      MFWD_NS.fwrfvc()[5],
    ",
  0x503c1c64u64 => "
      MFWD_NS.fwrfvc()[6],
    ",
  0x503c1c74u64 => "
      MFWD_NS.fwrfvc()[7],
    ",
  0x503c1c84u64 => "
      MFWD_NS.fwrfvc()[8],
    ",
  0x503c1c94u64 => "
      MFWD_NS.fwrfvc()[9],
    ",
  0x503c1ca4u64 => "
      MFWD_NS.fwrfvc()[10],
    ",
  0x503c1cb4u64 => "
      MFWD_NS.fwrfvc()[11],
    ",
  0x503c1cc4u64 => "
      MFWD_NS.fwrfvc()[12],
    ",
  0x503c1cd4u64 => "
      MFWD_NS.fwrfvc()[13],
    ",
  0x503c1ce4u64 => "
      MFWD_NS.fwrfvc()[14],
    ",
  0x503c1cf4u64 => "
      MFWD_NS.fwrfvc()[15],
    ",
  0x503c2000u64 => "
      MFWD_NS.fwcfc()[0],
    ",
  0x503c2040u64 => "
      MFWD_NS.fwcfc()[1],
    ",
  0x503c2080u64 => "
      MFWD_NS.fwcfc()[2],
    ",
  0x503c20c0u64 => "
      MFWD_NS.fwcfc()[3],
    ",
  0x503c2100u64 => "
      MFWD_NS.fwcfc()[4],
    ",
  0x503c2140u64 => "
      MFWD_NS.fwcfc()[5],
    ",
  0x503c2180u64 => "
      MFWD_NS.fwcfc()[6],
    ",
  0x503c21c0u64 => "
      MFWD_NS.fwcfc()[7],
    ",
  0x503c2200u64 => "
      MFWD_NS.fwcfc()[8],
    ",
  0x503c2240u64 => "
      MFWD_NS.fwcfc()[9],
    ",
  0x503c2280u64 => "
      MFWD_NS.fwcfc()[10],
    ",
  0x503c22c0u64 => "
      MFWD_NS.fwcfc()[11],
    ",
  0x503c2300u64 => "
      MFWD_NS.fwcfc()[12],
    ",
  0x503c2340u64 => "
      MFWD_NS.fwcfc()[13],
    ",
  0x503c2380u64 => "
      MFWD_NS.fwcfc()[14],
    ",
  0x503c23c0u64 => "
      MFWD_NS.fwcfc()[15],
    ",
  0x503c2004u64 => "
      MFWD_NS.fwcfmc0()[0],
    ",
  0x503c2044u64 => "
      MFWD_NS.fwcfmc0()[1],
    ",
  0x503c2084u64 => "
      MFWD_NS.fwcfmc0()[2],
    ",
  0x503c20c4u64 => "
      MFWD_NS.fwcfmc0()[3],
    ",
  0x503c2104u64 => "
      MFWD_NS.fwcfmc0()[4],
    ",
  0x503c2144u64 => "
      MFWD_NS.fwcfmc0()[5],
    ",
  0x503c2184u64 => "
      MFWD_NS.fwcfmc0()[6],
    ",
  0x503c21c4u64 => "
      MFWD_NS.fwcfmc0()[7],
    ",
  0x503c2204u64 => "
      MFWD_NS.fwcfmc0()[8],
    ",
  0x503c2244u64 => "
      MFWD_NS.fwcfmc0()[9],
    ",
  0x503c2284u64 => "
      MFWD_NS.fwcfmc0()[10],
    ",
  0x503c22c4u64 => "
      MFWD_NS.fwcfmc0()[11],
    ",
  0x503c2304u64 => "
      MFWD_NS.fwcfmc0()[12],
    ",
  0x503c2344u64 => "
      MFWD_NS.fwcfmc0()[13],
    ",
  0x503c2384u64 => "
      MFWD_NS.fwcfmc0()[14],
    ",
  0x503c23c4u64 => "
      MFWD_NS.fwcfmc0()[15],
    ",
  0x503c2008u64 => "
      MFWD_NS.fwcfmc1()[0],
    ",
  0x503c2048u64 => "
      MFWD_NS.fwcfmc1()[1],
    ",
  0x503c2088u64 => "
      MFWD_NS.fwcfmc1()[2],
    ",
  0x503c20c8u64 => "
      MFWD_NS.fwcfmc1()[3],
    ",
  0x503c2108u64 => "
      MFWD_NS.fwcfmc1()[4],
    ",
  0x503c2148u64 => "
      MFWD_NS.fwcfmc1()[5],
    ",
  0x503c2188u64 => "
      MFWD_NS.fwcfmc1()[6],
    ",
  0x503c21c8u64 => "
      MFWD_NS.fwcfmc1()[7],
    ",
  0x503c2208u64 => "
      MFWD_NS.fwcfmc1()[8],
    ",
  0x503c2248u64 => "
      MFWD_NS.fwcfmc1()[9],
    ",
  0x503c2288u64 => "
      MFWD_NS.fwcfmc1()[10],
    ",
  0x503c22c8u64 => "
      MFWD_NS.fwcfmc1()[11],
    ",
  0x503c2308u64 => "
      MFWD_NS.fwcfmc1()[12],
    ",
  0x503c2348u64 => "
      MFWD_NS.fwcfmc1()[13],
    ",
  0x503c2388u64 => "
      MFWD_NS.fwcfmc1()[14],
    ",
  0x503c23c8u64 => "
      MFWD_NS.fwcfmc1()[15],
    ",
  0x503c200cu64 => "
      MFWD_NS.fwcfmc2()[0],
    ",
  0x503c204cu64 => "
      MFWD_NS.fwcfmc2()[1],
    ",
  0x503c208cu64 => "
      MFWD_NS.fwcfmc2()[2],
    ",
  0x503c20ccu64 => "
      MFWD_NS.fwcfmc2()[3],
    ",
  0x503c210cu64 => "
      MFWD_NS.fwcfmc2()[4],
    ",
  0x503c214cu64 => "
      MFWD_NS.fwcfmc2()[5],
    ",
  0x503c218cu64 => "
      MFWD_NS.fwcfmc2()[6],
    ",
  0x503c21ccu64 => "
      MFWD_NS.fwcfmc2()[7],
    ",
  0x503c220cu64 => "
      MFWD_NS.fwcfmc2()[8],
    ",
  0x503c224cu64 => "
      MFWD_NS.fwcfmc2()[9],
    ",
  0x503c228cu64 => "
      MFWD_NS.fwcfmc2()[10],
    ",
  0x503c22ccu64 => "
      MFWD_NS.fwcfmc2()[11],
    ",
  0x503c230cu64 => "
      MFWD_NS.fwcfmc2()[12],
    ",
  0x503c234cu64 => "
      MFWD_NS.fwcfmc2()[13],
    ",
  0x503c238cu64 => "
      MFWD_NS.fwcfmc2()[14],
    ",
  0x503c23ccu64 => "
      MFWD_NS.fwcfmc2()[15],
    ",
  0x503c2010u64 => "
      MFWD_NS.fwcfmc3()[0],
    ",
  0x503c2050u64 => "
      MFWD_NS.fwcfmc3()[1],
    ",
  0x503c2090u64 => "
      MFWD_NS.fwcfmc3()[2],
    ",
  0x503c20d0u64 => "
      MFWD_NS.fwcfmc3()[3],
    ",
  0x503c2110u64 => "
      MFWD_NS.fwcfmc3()[4],
    ",
  0x503c2150u64 => "
      MFWD_NS.fwcfmc3()[5],
    ",
  0x503c2190u64 => "
      MFWD_NS.fwcfmc3()[6],
    ",
  0x503c21d0u64 => "
      MFWD_NS.fwcfmc3()[7],
    ",
  0x503c2210u64 => "
      MFWD_NS.fwcfmc3()[8],
    ",
  0x503c2250u64 => "
      MFWD_NS.fwcfmc3()[9],
    ",
  0x503c2290u64 => "
      MFWD_NS.fwcfmc3()[10],
    ",
  0x503c22d0u64 => "
      MFWD_NS.fwcfmc3()[11],
    ",
  0x503c2310u64 => "
      MFWD_NS.fwcfmc3()[12],
    ",
  0x503c2350u64 => "
      MFWD_NS.fwcfmc3()[13],
    ",
  0x503c2390u64 => "
      MFWD_NS.fwcfmc3()[14],
    ",
  0x503c23d0u64 => "
      MFWD_NS.fwcfmc3()[15],
    ",
  0x503c2014u64 => "
      MFWD_NS.fwcfmc4()[0],
    ",
  0x503c2054u64 => "
      MFWD_NS.fwcfmc4()[1],
    ",
  0x503c2094u64 => "
      MFWD_NS.fwcfmc4()[2],
    ",
  0x503c20d4u64 => "
      MFWD_NS.fwcfmc4()[3],
    ",
  0x503c2114u64 => "
      MFWD_NS.fwcfmc4()[4],
    ",
  0x503c2154u64 => "
      MFWD_NS.fwcfmc4()[5],
    ",
  0x503c2194u64 => "
      MFWD_NS.fwcfmc4()[6],
    ",
  0x503c21d4u64 => "
      MFWD_NS.fwcfmc4()[7],
    ",
  0x503c2214u64 => "
      MFWD_NS.fwcfmc4()[8],
    ",
  0x503c2254u64 => "
      MFWD_NS.fwcfmc4()[9],
    ",
  0x503c2294u64 => "
      MFWD_NS.fwcfmc4()[10],
    ",
  0x503c22d4u64 => "
      MFWD_NS.fwcfmc4()[11],
    ",
  0x503c2314u64 => "
      MFWD_NS.fwcfmc4()[12],
    ",
  0x503c2354u64 => "
      MFWD_NS.fwcfmc4()[13],
    ",
  0x503c2394u64 => "
      MFWD_NS.fwcfmc4()[14],
    ",
  0x503c23d4u64 => "
      MFWD_NS.fwcfmc4()[15],
    ",
  0x503c2018u64 => "
      MFWD_NS.fwcfmc5()[0],
    ",
  0x503c2058u64 => "
      MFWD_NS.fwcfmc5()[1],
    ",
  0x503c2098u64 => "
      MFWD_NS.fwcfmc5()[2],
    ",
  0x503c20d8u64 => "
      MFWD_NS.fwcfmc5()[3],
    ",
  0x503c2118u64 => "
      MFWD_NS.fwcfmc5()[4],
    ",
  0x503c2158u64 => "
      MFWD_NS.fwcfmc5()[5],
    ",
  0x503c2198u64 => "
      MFWD_NS.fwcfmc5()[6],
    ",
  0x503c21d8u64 => "
      MFWD_NS.fwcfmc5()[7],
    ",
  0x503c2218u64 => "
      MFWD_NS.fwcfmc5()[8],
    ",
  0x503c2258u64 => "
      MFWD_NS.fwcfmc5()[9],
    ",
  0x503c2298u64 => "
      MFWD_NS.fwcfmc5()[10],
    ",
  0x503c22d8u64 => "
      MFWD_NS.fwcfmc5()[11],
    ",
  0x503c2318u64 => "
      MFWD_NS.fwcfmc5()[12],
    ",
  0x503c2358u64 => "
      MFWD_NS.fwcfmc5()[13],
    ",
  0x503c2398u64 => "
      MFWD_NS.fwcfmc5()[14],
    ",
  0x503c23d8u64 => "
      MFWD_NS.fwcfmc5()[15],
    ",
  0x503c201cu64 => "
      MFWD_NS.fwcfmc6()[0],
    ",
  0x503c205cu64 => "
      MFWD_NS.fwcfmc6()[1],
    ",
  0x503c209cu64 => "
      MFWD_NS.fwcfmc6()[2],
    ",
  0x503c20dcu64 => "
      MFWD_NS.fwcfmc6()[3],
    ",
  0x503c211cu64 => "
      MFWD_NS.fwcfmc6()[4],
    ",
  0x503c215cu64 => "
      MFWD_NS.fwcfmc6()[5],
    ",
  0x503c219cu64 => "
      MFWD_NS.fwcfmc6()[6],
    ",
  0x503c21dcu64 => "
      MFWD_NS.fwcfmc6()[7],
    ",
  0x503c221cu64 => "
      MFWD_NS.fwcfmc6()[8],
    ",
  0x503c225cu64 => "
      MFWD_NS.fwcfmc6()[9],
    ",
  0x503c229cu64 => "
      MFWD_NS.fwcfmc6()[10],
    ",
  0x503c22dcu64 => "
      MFWD_NS.fwcfmc6()[11],
    ",
  0x503c231cu64 => "
      MFWD_NS.fwcfmc6()[12],
    ",
  0x503c235cu64 => "
      MFWD_NS.fwcfmc6()[13],
    ",
  0x503c239cu64 => "
      MFWD_NS.fwcfmc6()[14],
    ",
  0x503c23dcu64 => "
      MFWD_NS.fwcfmc6()[15],
    ",
  0x503c4008u64 => "
      MFWD_NS.fwip4sc(),
    ",
  0x503c4018u64 => "
      MFWD_NS.fwip6sc(),
    ",
  0x503c401cu64 => "
      MFWD_NS.fwip6oc(),
    ",
  0x503c4020u64 => "
      MFWD_NS.fwl2sc(),
    ",
  0x503c4030u64 => "
      MFWD_NS.fwsfhec(),
    ",
  0x503c4040u64 => "
      MFWD_NS.fwshcr0(),
    ",
  0x503c4044u64 => "
      MFWD_NS.fwshcr1(),
    ",
  0x503c4048u64 => "
      MFWD_NS.fwshcr2(),
    ",
  0x503c404cu64 => "
      MFWD_NS.fwshcr3(),
    ",
  0x503c4050u64 => "
      MFWD_NS.fwshcr4(),
    ",
  0x503c4054u64 => "
      MFWD_NS.fwshcr5(),
    ",
  0x503c4058u64 => "
      MFWD_NS.fwshcr6(),
    ",
  0x503c405cu64 => "
      MFWD_NS.fwshcr7(),
    ",
  0x503c4060u64 => "
      MFWD_NS.fwshcr8(),
    ",
  0x503c4064u64 => "
      MFWD_NS.fwshcr9(),
    ",
  0x503c4068u64 => "
      MFWD_NS.fwshcr10(),
    ",
  0x503c406cu64 => "
      MFWD_NS.fwshcr11(),
    ",
  0x503c4070u64 => "
      MFWD_NS.fwshcr12(),
    ",
  0x503c4074u64 => "
      MFWD_NS.fwshcr13(),
    ",
  0x503c4078u64 => "
      MFWD_NS.fwshcrr(),
    ",
  0x503c4090u64 => "
      MFWD_NS.fwlthhec(),
    ",
  0x503c4094u64 => "
      MFWD_NS.fwlthhc(),
    ",
  0x503c40a0u64 => "
      MFWD_NS.fwlthtl0(),
    ",
  0x503c40a4u64 => "
      MFWD_NS.fwlthtl1(),
    ",
  0x503c40a8u64 => "
      MFWD_NS.fwlthtl2(),
    ",
  0x503c40acu64 => "
      MFWD_NS.fwlthtl3(),
    ",
  0x503c40b0u64 => "
      MFWD_NS.fwlthtl4(),
    ",
  0x503c40b4u64 => "
      MFWD_NS.fwlthtl5(),
    ",
  0x503c40b8u64 => "
      MFWD_NS.fwlthtl6(),
    ",
  0x503c40bcu64 => "
      MFWD_NS.fwlthtl7(),
    ",
  0x503c40c0u64 => "
      MFWD_NS.fwlthtl80(),
    ",
  0x503c40d0u64 => "
      MFWD_NS.fwlthtl9(),
    ",
  0x503c40d4u64 => "
      MFWD_NS.fwlthtlr(),
    ",
  0x503c40e0u64 => "
      MFWD_NS.fwlthtim(),
    ",
  0x503c40e4u64 => "
      MFWD_NS.fwlthtem(),
    ",
  0x503c4100u64 => "
      MFWD_NS.fwlthts0(),
    ",
  0x503c4104u64 => "
      MFWD_NS.fwlthts1(),
    ",
  0x503c4108u64 => "
      MFWD_NS.fwlthts2(),
    ",
  0x503c410cu64 => "
      MFWD_NS.fwlthts3(),
    ",
  0x503c4110u64 => "
      MFWD_NS.fwlthts4(),
    ",
  0x503c4120u64 => "
      MFWD_NS.fwlthtsr0(),
    ",
  0x503c4124u64 => "
      MFWD_NS.fwlthtsr1(),
    ",
  0x503c4128u64 => "
      MFWD_NS.fwlthtsr2(),
    ",
  0x503c412cu64 => "
      MFWD_NS.fwlthtsr3(),
    ",
  0x503c4130u64 => "
      MFWD_NS.fwlthtsr40(),
    ",
  0x503c4140u64 => "
      MFWD_NS.fwlthtsr5(),
    ",
  0x503c4150u64 => "
      MFWD_NS.fwlthtr(),
    ",
  0x503c4154u64 => "
      MFWD_NS.fwlthtrr0(),
    ",
  0x503c4158u64 => "
      MFWD_NS.fwlthtrr1(),
    ",
  0x503c415cu64 => "
      MFWD_NS.fwlthtrr2(),
    ",
  0x503c4160u64 => "
      MFWD_NS.fwlthtrr3(),
    ",
  0x503c4164u64 => "
      MFWD_NS.fwlthtrr4(),
    ",
  0x503c4168u64 => "
      MFWD_NS.fwlthtrr5(),
    ",
  0x503c416cu64 => "
      MFWD_NS.fwlthtrr6(),
    ",
  0x503c4170u64 => "
      MFWD_NS.fwlthtrr7(),
    ",
  0x503c4174u64 => "
      MFWD_NS.fwlthtrr8(),
    ",
  0x503c4180u64 => "
      MFWD_NS.fwlthtrr90(),
    ",
  0x503c4190u64 => "
      MFWD_NS.fwlthtrr10(),
    ",
  0x503c4620u64 => "
      MFWD_NS.fwmachec(),
    ",
  0x503c4624u64 => "
      MFWD_NS.fwmachc(),
    ",
  0x503c4630u64 => "
      MFWD_NS.fwmactl0(),
    ",
  0x503c4634u64 => "
      MFWD_NS.fwmactl1(),
    ",
  0x503c4638u64 => "
      MFWD_NS.fwmactl2(),
    ",
  0x503c463cu64 => "
      MFWD_NS.fwmactl3(),
    ",
  0x503c4640u64 => "
      MFWD_NS.fwmactl40(),
    ",
  0x503c4650u64 => "
      MFWD_NS.fwmactl5(),
    ",
  0x503c4654u64 => "
      MFWD_NS.fwmactlr(),
    ",
  0x503c4660u64 => "
      MFWD_NS.fwmactim(),
    ",
  0x503c4664u64 => "
      MFWD_NS.fwmactem(),
    ",
  0x503c4670u64 => "
      MFWD_NS.fwmacts0(),
    ",
  0x503c4674u64 => "
      MFWD_NS.fwmacts1(),
    ",
  0x503c4678u64 => "
      MFWD_NS.fwmactsr0(),
    ",
  0x503c467cu64 => "
      MFWD_NS.fwmactsr1(),
    ",
  0x503c4680u64 => "
      MFWD_NS.fwmactsr20(),
    ",
  0x503c4690u64 => "
      MFWD_NS.fwmactsr3(),
    ",
  0x503c46a0u64 => "
      MFWD_NS.fwmactr(),
    ",
  0x503c46a4u64 => "
      MFWD_NS.fwmactrr0(),
    ",
  0x503c46a8u64 => "
      MFWD_NS.fwmactrr1(),
    ",
  0x503c46acu64 => "
      MFWD_NS.fwmactrr2(),
    ",
  0x503c46b0u64 => "
      MFWD_NS.fwmactrr3(),
    ",
  0x503c46b4u64 => "
      MFWD_NS.fwmactrr4(),
    ",
  0x503c46c0u64 => "
      MFWD_NS.fwmactrr50(),
    ",
  0x503c46d0u64 => "
      MFWD_NS.fwmactrr6(),
    ",
  0x503c4880u64 => "
      MFWD_NS.fwmacaguspc(),
    ",
  0x503c4884u64 => "
      MFWD_NS.fwmacagc(),
    ",
  0x503c4888u64 => "
      MFWD_NS.fwmacagm0(),
    ",
  0x503c488cu64 => "
      MFWD_NS.fwmacagm1(),
    ",
  0x503c4900u64 => "
      MFWD_NS.fwvlantec(),
    ",
  0x503c4910u64 => "
      MFWD_NS.fwvlantl0(),
    ",
  0x503c4914u64 => "
      MFWD_NS.fwvlantl1(),
    ",
  0x503c4918u64 => "
      MFWD_NS.fwvlantl2(),
    ",
  0x503c4920u64 => "
      MFWD_NS.fwvlantl30(),
    ",
  0x503c4930u64 => "
      MFWD_NS.fwvlantl4(),
    ",
  0x503c4934u64 => "
      MFWD_NS.fwvlantlr(),
    ",
  0x503c4940u64 => "
      MFWD_NS.fwvlantim(),
    ",
  0x503c4944u64 => "
      MFWD_NS.fwvlantem(),
    ",
  0x503c4950u64 => "
      MFWD_NS.fwvlants(),
    ",
  0x503c4954u64 => "
      MFWD_NS.fwvlantsr0(),
    ",
  0x503c4958u64 => "
      MFWD_NS.fwvlantsr1(),
    ",
  0x503c4960u64 => "
      MFWD_NS.fwvlantsr20(),
    ",
  0x503c4970u64 => "
      MFWD_NS.fwvlantsr3(),
    ",
  0x503c4a00u64 => "
      MFWD_NS.fwpbfc()[0],
    ",
  0x503c4a10u64 => "
      MFWD_NS.fwpbfc()[1],
    ",
  0x503c4a20u64 => "
      MFWD_NS.fwpbfc()[2],
    ",
  0x503c4a04u64 => "
      MFWD_NS.fwpbfcsdc0()[0],
    ",
  0x503c4a14u64 => "
      MFWD_NS.fwpbfcsdc0()[1],
    ",
  0x503c4a24u64 => "
      MFWD_NS.fwpbfcsdc0()[2],
    ",
  0x503c4e00u64 => "
      MFWD_NS.fwl23url0(),
    ",
  0x503c4e04u64 => "
      MFWD_NS.fwl23url1(),
    ",
  0x503c4e08u64 => "
      MFWD_NS.fwl23url2(),
    ",
  0x503c4e0cu64 => "
      MFWD_NS.fwl23url3(),
    ",
  0x503c4e10u64 => "
      MFWD_NS.fwl23urlr(),
    ",
  0x503c4e20u64 => "
      MFWD_NS.fwl23utim(),
    ",
  0x503c4e30u64 => "
      MFWD_NS.fwl23urr(),
    ",
  0x503c4e34u64 => "
      MFWD_NS.fwl23urrr0(),
    ",
  0x503c4e38u64 => "
      MFWD_NS.fwl23urrr1(),
    ",
  0x503c4e3cu64 => "
      MFWD_NS.fwl23urrr2(),
    ",
  0x503c4e40u64 => "
      MFWD_NS.fwl23urrr3(),
    ",
  0x503c4f00u64 => "
      MFWD_NS.fwl23urmc()[0],
    ",
  0x503c4f04u64 => "
      MFWD_NS.fwl23urmc()[1],
    ",
  0x503c4f08u64 => "
      MFWD_NS.fwl23urmc()[2],
    ",
  0x503c4f0cu64 => "
      MFWD_NS.fwl23urmc()[3],
    ",
  0x503c4f10u64 => "
      MFWD_NS.fwl23urmc()[4],
    ",
  0x503c4f14u64 => "
      MFWD_NS.fwl23urmc()[5],
    ",
  0x503c4f18u64 => "
      MFWD_NS.fwl23urmc()[6],
    ",
  0x503c4f1cu64 => "
      MFWD_NS.fwl23urmc()[7],
    ",
  0x503c4f20u64 => "
      MFWD_NS.fwl23urmc()[8],
    ",
  0x503c4f24u64 => "
      MFWD_NS.fwl23urmc()[9],
    ",
  0x503c4f28u64 => "
      MFWD_NS.fwl23urmc()[10],
    ",
  0x503c4f2cu64 => "
      MFWD_NS.fwl23urmc()[11],
    ",
  0x503c4f30u64 => "
      MFWD_NS.fwl23urmc()[12],
    ",
  0x503c4f34u64 => "
      MFWD_NS.fwl23urmc()[13],
    ",
  0x503c4f38u64 => "
      MFWD_NS.fwl23urmc()[14],
    ",
  0x503c4f3cu64 => "
      MFWD_NS.fwl23urmc()[15],
    ",
  0x503c4f40u64 => "
      MFWD_NS.fwl23urmc()[16],
    ",
  0x503c4f44u64 => "
      MFWD_NS.fwl23urmc()[17],
    ",
  0x503c4f48u64 => "
      MFWD_NS.fwl23urmc()[18],
    ",
  0x503c4f4cu64 => "
      MFWD_NS.fwl23urmc()[19],
    ",
  0x503c4f50u64 => "
      MFWD_NS.fwl23urmc()[20],
    ",
  0x503c4f54u64 => "
      MFWD_NS.fwl23urmc()[21],
    ",
  0x503c4f58u64 => "
      MFWD_NS.fwl23urmc()[22],
    ",
  0x503c4f5cu64 => "
      MFWD_NS.fwl23urmc()[23],
    ",
  0x503c4f60u64 => "
      MFWD_NS.fwl23urmc()[24],
    ",
  0x503c4f64u64 => "
      MFWD_NS.fwl23urmc()[25],
    ",
  0x503c4f68u64 => "
      MFWD_NS.fwl23urmc()[26],
    ",
  0x503c4f6cu64 => "
      MFWD_NS.fwl23urmc()[27],
    ",
  0x503c4f70u64 => "
      MFWD_NS.fwl23urmc()[28],
    ",
  0x503c4f74u64 => "
      MFWD_NS.fwl23urmc()[29],
    ",
  0x503c4f78u64 => "
      MFWD_NS.fwl23urmc()[30],
    ",
  0x503c4f7cu64 => "
      MFWD_NS.fwl23urmc()[31],
    ",
  0x503c5000u64 => "
      MFWD_NS.fwpmfgc()[0],
    ",
  0x503c5004u64 => "
      MFWD_NS.fwpmfgc()[1],
    ",
  0x503c5008u64 => "
      MFWD_NS.fwpmfgc()[2],
    ",
  0x503c500cu64 => "
      MFWD_NS.fwpmfgc()[3],
    ",
  0x503c5010u64 => "
      MFWD_NS.fwpmfgc()[4],
    ",
  0x503c5014u64 => "
      MFWD_NS.fwpmfgc()[5],
    ",
  0x503c5018u64 => "
      MFWD_NS.fwpmfgc()[6],
    ",
  0x503c501cu64 => "
      MFWD_NS.fwpmfgc()[7],
    ",
  0x503c5020u64 => "
      MFWD_NS.fwpmfgc()[8],
    ",
  0x503c5024u64 => "
      MFWD_NS.fwpmfgc()[9],
    ",
  0x503c5028u64 => "
      MFWD_NS.fwpmfgc()[10],
    ",
  0x503c502cu64 => "
      MFWD_NS.fwpmfgc()[11],
    ",
  0x503c5030u64 => "
      MFWD_NS.fwpmfgc()[12],
    ",
  0x503c5034u64 => "
      MFWD_NS.fwpmfgc()[13],
    ",
  0x503c5038u64 => "
      MFWD_NS.fwpmfgc()[14],
    ",
  0x503c503cu64 => "
      MFWD_NS.fwpmfgc()[15],
    ",
  0x503c5600u64 => "
      MFWD_NS.fwpmtrfc()[0],
    ",
  0x503c5620u64 => "
      MFWD_NS.fwpmtrfc()[1],
    ",
  0x503c5640u64 => "
      MFWD_NS.fwpmtrfc()[2],
    ",
  0x503c5660u64 => "
      MFWD_NS.fwpmtrfc()[3],
    ",
  0x503c5680u64 => "
      MFWD_NS.fwpmtrfc()[4],
    ",
  0x503c56a0u64 => "
      MFWD_NS.fwpmtrfc()[5],
    ",
  0x503c56c0u64 => "
      MFWD_NS.fwpmtrfc()[6],
    ",
  0x503c56e0u64 => "
      MFWD_NS.fwpmtrfc()[7],
    ",
  0x503c5700u64 => "
      MFWD_NS.fwpmtrfc()[8],
    ",
  0x503c5720u64 => "
      MFWD_NS.fwpmtrfc()[9],
    ",
  0x503c5740u64 => "
      MFWD_NS.fwpmtrfc()[10],
    ",
  0x503c5760u64 => "
      MFWD_NS.fwpmtrfc()[11],
    ",
  0x503c5780u64 => "
      MFWD_NS.fwpmtrfc()[12],
    ",
  0x503c57a0u64 => "
      MFWD_NS.fwpmtrfc()[13],
    ",
  0x503c57c0u64 => "
      MFWD_NS.fwpmtrfc()[14],
    ",
  0x503c57e0u64 => "
      MFWD_NS.fwpmtrfc()[15],
    ",
  0x503c5800u64 => "
      MFWD_NS.fwpmtrfc()[16],
    ",
  0x503c5820u64 => "
      MFWD_NS.fwpmtrfc()[17],
    ",
  0x503c5840u64 => "
      MFWD_NS.fwpmtrfc()[18],
    ",
  0x503c5860u64 => "
      MFWD_NS.fwpmtrfc()[19],
    ",
  0x503c5880u64 => "
      MFWD_NS.fwpmtrfc()[20],
    ",
  0x503c58a0u64 => "
      MFWD_NS.fwpmtrfc()[21],
    ",
  0x503c58c0u64 => "
      MFWD_NS.fwpmtrfc()[22],
    ",
  0x503c58e0u64 => "
      MFWD_NS.fwpmtrfc()[23],
    ",
  0x503c5900u64 => "
      MFWD_NS.fwpmtrfc()[24],
    ",
  0x503c5920u64 => "
      MFWD_NS.fwpmtrfc()[25],
    ",
  0x503c5940u64 => "
      MFWD_NS.fwpmtrfc()[26],
    ",
  0x503c5960u64 => "
      MFWD_NS.fwpmtrfc()[27],
    ",
  0x503c5980u64 => "
      MFWD_NS.fwpmtrfc()[28],
    ",
  0x503c59a0u64 => "
      MFWD_NS.fwpmtrfc()[29],
    ",
  0x503c59c0u64 => "
      MFWD_NS.fwpmtrfc()[30],
    ",
  0x503c59e0u64 => "
      MFWD_NS.fwpmtrfc()[31],
    ",
  0x503c5604u64 => "
      MFWD_NS.fwpmtrcbsc()[0],
    ",
  0x503c5624u64 => "
      MFWD_NS.fwpmtrcbsc()[1],
    ",
  0x503c5644u64 => "
      MFWD_NS.fwpmtrcbsc()[2],
    ",
  0x503c5664u64 => "
      MFWD_NS.fwpmtrcbsc()[3],
    ",
  0x503c5684u64 => "
      MFWD_NS.fwpmtrcbsc()[4],
    ",
  0x503c56a4u64 => "
      MFWD_NS.fwpmtrcbsc()[5],
    ",
  0x503c56c4u64 => "
      MFWD_NS.fwpmtrcbsc()[6],
    ",
  0x503c56e4u64 => "
      MFWD_NS.fwpmtrcbsc()[7],
    ",
  0x503c5704u64 => "
      MFWD_NS.fwpmtrcbsc()[8],
    ",
  0x503c5724u64 => "
      MFWD_NS.fwpmtrcbsc()[9],
    ",
  0x503c5744u64 => "
      MFWD_NS.fwpmtrcbsc()[10],
    ",
  0x503c5764u64 => "
      MFWD_NS.fwpmtrcbsc()[11],
    ",
  0x503c5784u64 => "
      MFWD_NS.fwpmtrcbsc()[12],
    ",
  0x503c57a4u64 => "
      MFWD_NS.fwpmtrcbsc()[13],
    ",
  0x503c57c4u64 => "
      MFWD_NS.fwpmtrcbsc()[14],
    ",
  0x503c57e4u64 => "
      MFWD_NS.fwpmtrcbsc()[15],
    ",
  0x503c5804u64 => "
      MFWD_NS.fwpmtrcbsc()[16],
    ",
  0x503c5824u64 => "
      MFWD_NS.fwpmtrcbsc()[17],
    ",
  0x503c5844u64 => "
      MFWD_NS.fwpmtrcbsc()[18],
    ",
  0x503c5864u64 => "
      MFWD_NS.fwpmtrcbsc()[19],
    ",
  0x503c5884u64 => "
      MFWD_NS.fwpmtrcbsc()[20],
    ",
  0x503c58a4u64 => "
      MFWD_NS.fwpmtrcbsc()[21],
    ",
  0x503c58c4u64 => "
      MFWD_NS.fwpmtrcbsc()[22],
    ",
  0x503c58e4u64 => "
      MFWD_NS.fwpmtrcbsc()[23],
    ",
  0x503c5904u64 => "
      MFWD_NS.fwpmtrcbsc()[24],
    ",
  0x503c5924u64 => "
      MFWD_NS.fwpmtrcbsc()[25],
    ",
  0x503c5944u64 => "
      MFWD_NS.fwpmtrcbsc()[26],
    ",
  0x503c5964u64 => "
      MFWD_NS.fwpmtrcbsc()[27],
    ",
  0x503c5984u64 => "
      MFWD_NS.fwpmtrcbsc()[28],
    ",
  0x503c59a4u64 => "
      MFWD_NS.fwpmtrcbsc()[29],
    ",
  0x503c59c4u64 => "
      MFWD_NS.fwpmtrcbsc()[30],
    ",
  0x503c59e4u64 => "
      MFWD_NS.fwpmtrcbsc()[31],
    ",
  0x503c5608u64 => "
      MFWD_NS.fwpmtrcirc()[0],
    ",
  0x503c5628u64 => "
      MFWD_NS.fwpmtrcirc()[1],
    ",
  0x503c5648u64 => "
      MFWD_NS.fwpmtrcirc()[2],
    ",
  0x503c5668u64 => "
      MFWD_NS.fwpmtrcirc()[3],
    ",
  0x503c5688u64 => "
      MFWD_NS.fwpmtrcirc()[4],
    ",
  0x503c56a8u64 => "
      MFWD_NS.fwpmtrcirc()[5],
    ",
  0x503c56c8u64 => "
      MFWD_NS.fwpmtrcirc()[6],
    ",
  0x503c56e8u64 => "
      MFWD_NS.fwpmtrcirc()[7],
    ",
  0x503c5708u64 => "
      MFWD_NS.fwpmtrcirc()[8],
    ",
  0x503c5728u64 => "
      MFWD_NS.fwpmtrcirc()[9],
    ",
  0x503c5748u64 => "
      MFWD_NS.fwpmtrcirc()[10],
    ",
  0x503c5768u64 => "
      MFWD_NS.fwpmtrcirc()[11],
    ",
  0x503c5788u64 => "
      MFWD_NS.fwpmtrcirc()[12],
    ",
  0x503c57a8u64 => "
      MFWD_NS.fwpmtrcirc()[13],
    ",
  0x503c57c8u64 => "
      MFWD_NS.fwpmtrcirc()[14],
    ",
  0x503c57e8u64 => "
      MFWD_NS.fwpmtrcirc()[15],
    ",
  0x503c5808u64 => "
      MFWD_NS.fwpmtrcirc()[16],
    ",
  0x503c5828u64 => "
      MFWD_NS.fwpmtrcirc()[17],
    ",
  0x503c5848u64 => "
      MFWD_NS.fwpmtrcirc()[18],
    ",
  0x503c5868u64 => "
      MFWD_NS.fwpmtrcirc()[19],
    ",
  0x503c5888u64 => "
      MFWD_NS.fwpmtrcirc()[20],
    ",
  0x503c58a8u64 => "
      MFWD_NS.fwpmtrcirc()[21],
    ",
  0x503c58c8u64 => "
      MFWD_NS.fwpmtrcirc()[22],
    ",
  0x503c58e8u64 => "
      MFWD_NS.fwpmtrcirc()[23],
    ",
  0x503c5908u64 => "
      MFWD_NS.fwpmtrcirc()[24],
    ",
  0x503c5928u64 => "
      MFWD_NS.fwpmtrcirc()[25],
    ",
  0x503c5948u64 => "
      MFWD_NS.fwpmtrcirc()[26],
    ",
  0x503c5968u64 => "
      MFWD_NS.fwpmtrcirc()[27],
    ",
  0x503c5988u64 => "
      MFWD_NS.fwpmtrcirc()[28],
    ",
  0x503c59a8u64 => "
      MFWD_NS.fwpmtrcirc()[29],
    ",
  0x503c59c8u64 => "
      MFWD_NS.fwpmtrcirc()[30],
    ",
  0x503c59e8u64 => "
      MFWD_NS.fwpmtrcirc()[31],
    ",
  0x503c560cu64 => "
      MFWD_NS.fwpmtrebsc()[0],
    ",
  0x503c562cu64 => "
      MFWD_NS.fwpmtrebsc()[1],
    ",
  0x503c564cu64 => "
      MFWD_NS.fwpmtrebsc()[2],
    ",
  0x503c566cu64 => "
      MFWD_NS.fwpmtrebsc()[3],
    ",
  0x503c568cu64 => "
      MFWD_NS.fwpmtrebsc()[4],
    ",
  0x503c56acu64 => "
      MFWD_NS.fwpmtrebsc()[5],
    ",
  0x503c56ccu64 => "
      MFWD_NS.fwpmtrebsc()[6],
    ",
  0x503c56ecu64 => "
      MFWD_NS.fwpmtrebsc()[7],
    ",
  0x503c5610u64 => "
      MFWD_NS.fwpmtreirc()[0],
    ",
  0x503c5630u64 => "
      MFWD_NS.fwpmtreirc()[1],
    ",
  0x503c5650u64 => "
      MFWD_NS.fwpmtreirc()[2],
    ",
  0x503c5670u64 => "
      MFWD_NS.fwpmtreirc()[3],
    ",
  0x503c5690u64 => "
      MFWD_NS.fwpmtreirc()[4],
    ",
  0x503c56b0u64 => "
      MFWD_NS.fwpmtreirc()[5],
    ",
  0x503c56d0u64 => "
      MFWD_NS.fwpmtreirc()[6],
    ",
  0x503c56f0u64 => "
      MFWD_NS.fwpmtreirc()[7],
    ",
  0x503c5614u64 => "
      MFWD_NS.fwpmtrfm()[0],
    ",
  0x503c5634u64 => "
      MFWD_NS.fwpmtrfm()[1],
    ",
  0x503c5654u64 => "
      MFWD_NS.fwpmtrfm()[2],
    ",
  0x503c5674u64 => "
      MFWD_NS.fwpmtrfm()[3],
    ",
  0x503c5694u64 => "
      MFWD_NS.fwpmtrfm()[4],
    ",
  0x503c56b4u64 => "
      MFWD_NS.fwpmtrfm()[5],
    ",
  0x503c56d4u64 => "
      MFWD_NS.fwpmtrfm()[6],
    ",
  0x503c56f4u64 => "
      MFWD_NS.fwpmtrfm()[7],
    ",
  0x503c5714u64 => "
      MFWD_NS.fwpmtrfm()[8],
    ",
  0x503c5734u64 => "
      MFWD_NS.fwpmtrfm()[9],
    ",
  0x503c5754u64 => "
      MFWD_NS.fwpmtrfm()[10],
    ",
  0x503c5774u64 => "
      MFWD_NS.fwpmtrfm()[11],
    ",
  0x503c5794u64 => "
      MFWD_NS.fwpmtrfm()[12],
    ",
  0x503c57b4u64 => "
      MFWD_NS.fwpmtrfm()[13],
    ",
  0x503c57d4u64 => "
      MFWD_NS.fwpmtrfm()[14],
    ",
  0x503c57f4u64 => "
      MFWD_NS.fwpmtrfm()[15],
    ",
  0x503c5814u64 => "
      MFWD_NS.fwpmtrfm()[16],
    ",
  0x503c5834u64 => "
      MFWD_NS.fwpmtrfm()[17],
    ",
  0x503c5854u64 => "
      MFWD_NS.fwpmtrfm()[18],
    ",
  0x503c5874u64 => "
      MFWD_NS.fwpmtrfm()[19],
    ",
  0x503c5894u64 => "
      MFWD_NS.fwpmtrfm()[20],
    ",
  0x503c58b4u64 => "
      MFWD_NS.fwpmtrfm()[21],
    ",
  0x503c58d4u64 => "
      MFWD_NS.fwpmtrfm()[22],
    ",
  0x503c58f4u64 => "
      MFWD_NS.fwpmtrfm()[23],
    ",
  0x503c5914u64 => "
      MFWD_NS.fwpmtrfm()[24],
    ",
  0x503c5934u64 => "
      MFWD_NS.fwpmtrfm()[25],
    ",
  0x503c5954u64 => "
      MFWD_NS.fwpmtrfm()[26],
    ",
  0x503c5974u64 => "
      MFWD_NS.fwpmtrfm()[27],
    ",
  0x503c5994u64 => "
      MFWD_NS.fwpmtrfm()[28],
    ",
  0x503c59b4u64 => "
      MFWD_NS.fwpmtrfm()[29],
    ",
  0x503c59d4u64 => "
      MFWD_NS.fwpmtrfm()[30],
    ",
  0x503c59f4u64 => "
      MFWD_NS.fwpmtrfm()[31],
    ",
  0x503c6000u64 => "
      MFWD_NS.fwftl0(),
    ",
  0x503c6004u64 => "
      MFWD_NS.fwftl1(),
    ",
  0x503c6008u64 => "
      MFWD_NS.fwftlr(),
    ",
  0x503c6010u64 => "
      MFWD_NS.fwftoc(),
    ",
  0x503c6014u64 => "
      MFWD_NS.fwftopc(),
    ",
  0x503c6020u64 => "
      MFWD_NS.fwftim(),
    ",
  0x503c6030u64 => "
      MFWD_NS.fwftr(),
    ",
  0x503c6034u64 => "
      MFWD_NS.fwftrr0(),
    ",
  0x503c6038u64 => "
      MFWD_NS.fwftrr1(),
    ",
  0x503c603cu64 => "
      MFWD_NS.fwftrr2(),
    ",
  0x503c6100u64 => "
      MFWD_NS.fwseqngc()[0],
    ",
  0x503c6108u64 => "
      MFWD_NS.fwseqngc()[1],
    ",
  0x503c6110u64 => "
      MFWD_NS.fwseqngc()[2],
    ",
  0x503c6118u64 => "
      MFWD_NS.fwseqngc()[3],
    ",
  0x503c6120u64 => "
      MFWD_NS.fwseqngc()[4],
    ",
  0x503c6128u64 => "
      MFWD_NS.fwseqngc()[5],
    ",
  0x503c6130u64 => "
      MFWD_NS.fwseqngc()[6],
    ",
  0x503c6138u64 => "
      MFWD_NS.fwseqngc()[7],
    ",
  0x503c6140u64 => "
      MFWD_NS.fwseqngc()[8],
    ",
  0x503c6148u64 => "
      MFWD_NS.fwseqngc()[9],
    ",
  0x503c6150u64 => "
      MFWD_NS.fwseqngc()[10],
    ",
  0x503c6158u64 => "
      MFWD_NS.fwseqngc()[11],
    ",
  0x503c6160u64 => "
      MFWD_NS.fwseqngc()[12],
    ",
  0x503c6168u64 => "
      MFWD_NS.fwseqngc()[13],
    ",
  0x503c6170u64 => "
      MFWD_NS.fwseqngc()[14],
    ",
  0x503c6178u64 => "
      MFWD_NS.fwseqngc()[15],
    ",
  0x503c6180u64 => "
      MFWD_NS.fwseqngc()[16],
    ",
  0x503c6188u64 => "
      MFWD_NS.fwseqngc()[17],
    ",
  0x503c6190u64 => "
      MFWD_NS.fwseqngc()[18],
    ",
  0x503c6198u64 => "
      MFWD_NS.fwseqngc()[19],
    ",
  0x503c61a0u64 => "
      MFWD_NS.fwseqngc()[20],
    ",
  0x503c61a8u64 => "
      MFWD_NS.fwseqngc()[21],
    ",
  0x503c61b0u64 => "
      MFWD_NS.fwseqngc()[22],
    ",
  0x503c61b8u64 => "
      MFWD_NS.fwseqngc()[23],
    ",
  0x503c61c0u64 => "
      MFWD_NS.fwseqngc()[24],
    ",
  0x503c61c8u64 => "
      MFWD_NS.fwseqngc()[25],
    ",
  0x503c61d0u64 => "
      MFWD_NS.fwseqngc()[26],
    ",
  0x503c61d8u64 => "
      MFWD_NS.fwseqngc()[27],
    ",
  0x503c61e0u64 => "
      MFWD_NS.fwseqngc()[28],
    ",
  0x503c61e8u64 => "
      MFWD_NS.fwseqngc()[29],
    ",
  0x503c61f0u64 => "
      MFWD_NS.fwseqngc()[30],
    ",
  0x503c61f8u64 => "
      MFWD_NS.fwseqngc()[31],
    ",
  0x503c6104u64 => "
      MFWD_NS.fwseqngm()[0],
    ",
  0x503c610cu64 => "
      MFWD_NS.fwseqngm()[1],
    ",
  0x503c6114u64 => "
      MFWD_NS.fwseqngm()[2],
    ",
  0x503c611cu64 => "
      MFWD_NS.fwseqngm()[3],
    ",
  0x503c6124u64 => "
      MFWD_NS.fwseqngm()[4],
    ",
  0x503c612cu64 => "
      MFWD_NS.fwseqngm()[5],
    ",
  0x503c6134u64 => "
      MFWD_NS.fwseqngm()[6],
    ",
  0x503c613cu64 => "
      MFWD_NS.fwseqngm()[7],
    ",
  0x503c6144u64 => "
      MFWD_NS.fwseqngm()[8],
    ",
  0x503c614cu64 => "
      MFWD_NS.fwseqngm()[9],
    ",
  0x503c6154u64 => "
      MFWD_NS.fwseqngm()[10],
    ",
  0x503c615cu64 => "
      MFWD_NS.fwseqngm()[11],
    ",
  0x503c6164u64 => "
      MFWD_NS.fwseqngm()[12],
    ",
  0x503c616cu64 => "
      MFWD_NS.fwseqngm()[13],
    ",
  0x503c6174u64 => "
      MFWD_NS.fwseqngm()[14],
    ",
  0x503c617cu64 => "
      MFWD_NS.fwseqngm()[15],
    ",
  0x503c6184u64 => "
      MFWD_NS.fwseqngm()[16],
    ",
  0x503c618cu64 => "
      MFWD_NS.fwseqngm()[17],
    ",
  0x503c6194u64 => "
      MFWD_NS.fwseqngm()[18],
    ",
  0x503c619cu64 => "
      MFWD_NS.fwseqngm()[19],
    ",
  0x503c61a4u64 => "
      MFWD_NS.fwseqngm()[20],
    ",
  0x503c61acu64 => "
      MFWD_NS.fwseqngm()[21],
    ",
  0x503c61b4u64 => "
      MFWD_NS.fwseqngm()[22],
    ",
  0x503c61bcu64 => "
      MFWD_NS.fwseqngm()[23],
    ",
  0x503c61c4u64 => "
      MFWD_NS.fwseqngm()[24],
    ",
  0x503c61ccu64 => "
      MFWD_NS.fwseqngm()[25],
    ",
  0x503c61d4u64 => "
      MFWD_NS.fwseqngm()[26],
    ",
  0x503c61dcu64 => "
      MFWD_NS.fwseqngm()[27],
    ",
  0x503c61e4u64 => "
      MFWD_NS.fwseqngm()[28],
    ",
  0x503c61ecu64 => "
      MFWD_NS.fwseqngm()[29],
    ",
  0x503c61f4u64 => "
      MFWD_NS.fwseqngm()[30],
    ",
  0x503c61fcu64 => "
      MFWD_NS.fwseqngm()[31],
    ",
  0x503c6200u64 => "
      MFWD_NS.fwseqnrc(),
    ",
  0x503c6300u64 => "
      MFWD_NS.fwctfdcn()[0],
    ",
  0x503c6320u64 => "
      MFWD_NS.fwctfdcn()[1],
    ",
  0x503c6304u64 => "
      MFWD_NS.fwlthfdcn()[0],
    ",
  0x503c6324u64 => "
      MFWD_NS.fwlthfdcn()[1],
    ",
  0x503c6344u64 => "
      MFWD_NS.fwlthfdcn()[2],
    ",
  0x503c630cu64 => "
      MFWD_NS.fwltwfdcn()[0],
    ",
  0x503c632cu64 => "
      MFWD_NS.fwltwfdcn()[1],
    ",
  0x503c634cu64 => "
      MFWD_NS.fwltwfdcn()[2],
    ",
  0x503c6310u64 => "
      MFWD_NS.fwpbfdcn()[0],
    ",
  0x503c6330u64 => "
      MFWD_NS.fwpbfdcn()[1],
    ",
  0x503c6350u64 => "
      MFWD_NS.fwpbfdcn()[2],
    ",
  0x503c6314u64 => "
      MFWD_NS.fwmhlcn()[0],
    ",
  0x503c6334u64 => "
      MFWD_NS.fwmhlcn()[1],
    ",
  0x503c6354u64 => "
      MFWD_NS.fwmhlcn()[2],
    ",
  0x503c6340u64 => "
      MFWD_NS.fwddfdcn2(),
    ",
  0x503c6504u64 => "
      MFWD_NS.fwwmrdcn()[0],
    ",
  0x503c6524u64 => "
      MFWD_NS.fwwmrdcn()[1],
    ",
  0x503c6544u64 => "
      MFWD_NS.fwwmrdcn()[2],
    ",
  0x503c6508u64 => "
      MFWD_NS.fwctrdcn()[0],
    ",
  0x503c6528u64 => "
      MFWD_NS.fwctrdcn()[1],
    ",
  0x503c650cu64 => "
      MFWD_NS.fwlthrdcn()[0],
    ",
  0x503c652cu64 => "
      MFWD_NS.fwlthrdcn()[1],
    ",
  0x503c654cu64 => "
      MFWD_NS.fwlthrdcn()[2],
    ",
  0x503c6514u64 => "
      MFWD_NS.fwltwrdcn()[0],
    ",
  0x503c6534u64 => "
      MFWD_NS.fwltwrdcn()[1],
    ",
  0x503c6554u64 => "
      MFWD_NS.fwltwrdcn()[2],
    ",
  0x503c6518u64 => "
      MFWD_NS.fwpbrdcn()[0],
    ",
  0x503c6538u64 => "
      MFWD_NS.fwpbrdcn()[1],
    ",
  0x503c6558u64 => "
      MFWD_NS.fwpbrdcn()[2],
    ",
  0x503c6548u64 => "
      MFWD_NS.fwddrdcn2(),
    ",
  0x503c6700u64 => "
      MFWD_NS.fwpmfdcn()[0],
    ",
  0x503c6704u64 => "
      MFWD_NS.fwpmfdcn()[1],
    ",
  0x503c6708u64 => "
      MFWD_NS.fwpmfdcn()[2],
    ",
  0x503c670cu64 => "
      MFWD_NS.fwpmfdcn()[3],
    ",
  0x503c6710u64 => "
      MFWD_NS.fwpmfdcn()[4],
    ",
  0x503c6714u64 => "
      MFWD_NS.fwpmfdcn()[5],
    ",
  0x503c6718u64 => "
      MFWD_NS.fwpmfdcn()[6],
    ",
  0x503c671cu64 => "
      MFWD_NS.fwpmfdcn()[7],
    ",
  0x503c6720u64 => "
      MFWD_NS.fwpmfdcn()[8],
    ",
  0x503c6724u64 => "
      MFWD_NS.fwpmfdcn()[9],
    ",
  0x503c6728u64 => "
      MFWD_NS.fwpmfdcn()[10],
    ",
  0x503c672cu64 => "
      MFWD_NS.fwpmfdcn()[11],
    ",
  0x503c6730u64 => "
      MFWD_NS.fwpmfdcn()[12],
    ",
  0x503c6734u64 => "
      MFWD_NS.fwpmfdcn()[13],
    ",
  0x503c6738u64 => "
      MFWD_NS.fwpmfdcn()[14],
    ",
  0x503c673cu64 => "
      MFWD_NS.fwpmfdcn()[15],
    ",
  0x503c6800u64 => "
      MFWD_NS.fwpmgdcn()[0],
    ",
  0x503c6810u64 => "
      MFWD_NS.fwpmgdcn()[1],
    ",
  0x503c6820u64 => "
      MFWD_NS.fwpmgdcn()[2],
    ",
  0x503c6830u64 => "
      MFWD_NS.fwpmgdcn()[3],
    ",
  0x503c6840u64 => "
      MFWD_NS.fwpmgdcn()[4],
    ",
  0x503c6850u64 => "
      MFWD_NS.fwpmgdcn()[5],
    ",
  0x503c6860u64 => "
      MFWD_NS.fwpmgdcn()[6],
    ",
  0x503c6870u64 => "
      MFWD_NS.fwpmgdcn()[7],
    ",
  0x503c6880u64 => "
      MFWD_NS.fwpmgdcn()[8],
    ",
  0x503c6890u64 => "
      MFWD_NS.fwpmgdcn()[9],
    ",
  0x503c68a0u64 => "
      MFWD_NS.fwpmgdcn()[10],
    ",
  0x503c68b0u64 => "
      MFWD_NS.fwpmgdcn()[11],
    ",
  0x503c68c0u64 => "
      MFWD_NS.fwpmgdcn()[12],
    ",
  0x503c68d0u64 => "
      MFWD_NS.fwpmgdcn()[13],
    ",
  0x503c68e0u64 => "
      MFWD_NS.fwpmgdcn()[14],
    ",
  0x503c68f0u64 => "
      MFWD_NS.fwpmgdcn()[15],
    ",
  0x503c6900u64 => "
      MFWD_NS.fwpmgdcn()[16],
    ",
  0x503c6910u64 => "
      MFWD_NS.fwpmgdcn()[17],
    ",
  0x503c6920u64 => "
      MFWD_NS.fwpmgdcn()[18],
    ",
  0x503c6930u64 => "
      MFWD_NS.fwpmgdcn()[19],
    ",
  0x503c6940u64 => "
      MFWD_NS.fwpmgdcn()[20],
    ",
  0x503c6950u64 => "
      MFWD_NS.fwpmgdcn()[21],
    ",
  0x503c6960u64 => "
      MFWD_NS.fwpmgdcn()[22],
    ",
  0x503c6970u64 => "
      MFWD_NS.fwpmgdcn()[23],
    ",
  0x503c6980u64 => "
      MFWD_NS.fwpmgdcn()[24],
    ",
  0x503c6990u64 => "
      MFWD_NS.fwpmgdcn()[25],
    ",
  0x503c69a0u64 => "
      MFWD_NS.fwpmgdcn()[26],
    ",
  0x503c69b0u64 => "
      MFWD_NS.fwpmgdcn()[27],
    ",
  0x503c69c0u64 => "
      MFWD_NS.fwpmgdcn()[28],
    ",
  0x503c69d0u64 => "
      MFWD_NS.fwpmgdcn()[29],
    ",
  0x503c69e0u64 => "
      MFWD_NS.fwpmgdcn()[30],
    ",
  0x503c69f0u64 => "
      MFWD_NS.fwpmgdcn()[31],
    ",
  0x503c6804u64 => "
      MFWD_NS.fwpmydcn()[0],
    ",
  0x503c6814u64 => "
      MFWD_NS.fwpmydcn()[1],
    ",
  0x503c6824u64 => "
      MFWD_NS.fwpmydcn()[2],
    ",
  0x503c6834u64 => "
      MFWD_NS.fwpmydcn()[3],
    ",
  0x503c6844u64 => "
      MFWD_NS.fwpmydcn()[4],
    ",
  0x503c6854u64 => "
      MFWD_NS.fwpmydcn()[5],
    ",
  0x503c6864u64 => "
      MFWD_NS.fwpmydcn()[6],
    ",
  0x503c6874u64 => "
      MFWD_NS.fwpmydcn()[7],
    ",
  0x503c6808u64 => "
      MFWD_NS.fwpmrdcn()[0],
    ",
  0x503c6818u64 => "
      MFWD_NS.fwpmrdcn()[1],
    ",
  0x503c6828u64 => "
      MFWD_NS.fwpmrdcn()[2],
    ",
  0x503c6838u64 => "
      MFWD_NS.fwpmrdcn()[3],
    ",
  0x503c6848u64 => "
      MFWD_NS.fwpmrdcn()[4],
    ",
  0x503c6858u64 => "
      MFWD_NS.fwpmrdcn()[5],
    ",
  0x503c6868u64 => "
      MFWD_NS.fwpmrdcn()[6],
    ",
  0x503c6878u64 => "
      MFWD_NS.fwpmrdcn()[7],
    ",
  0x503c6888u64 => "
      MFWD_NS.fwpmrdcn()[8],
    ",
  0x503c6898u64 => "
      MFWD_NS.fwpmrdcn()[9],
    ",
  0x503c68a8u64 => "
      MFWD_NS.fwpmrdcn()[10],
    ",
  0x503c68b8u64 => "
      MFWD_NS.fwpmrdcn()[11],
    ",
  0x503c68c8u64 => "
      MFWD_NS.fwpmrdcn()[12],
    ",
  0x503c68d8u64 => "
      MFWD_NS.fwpmrdcn()[13],
    ",
  0x503c68e8u64 => "
      MFWD_NS.fwpmrdcn()[14],
    ",
  0x503c68f8u64 => "
      MFWD_NS.fwpmrdcn()[15],
    ",
  0x503c6908u64 => "
      MFWD_NS.fwpmrdcn()[16],
    ",
  0x503c6918u64 => "
      MFWD_NS.fwpmrdcn()[17],
    ",
  0x503c6928u64 => "
      MFWD_NS.fwpmrdcn()[18],
    ",
  0x503c6938u64 => "
      MFWD_NS.fwpmrdcn()[19],
    ",
  0x503c6948u64 => "
      MFWD_NS.fwpmrdcn()[20],
    ",
  0x503c6958u64 => "
      MFWD_NS.fwpmrdcn()[21],
    ",
  0x503c6968u64 => "
      MFWD_NS.fwpmrdcn()[22],
    ",
  0x503c6978u64 => "
      MFWD_NS.fwpmrdcn()[23],
    ",
  0x503c6988u64 => "
      MFWD_NS.fwpmrdcn()[24],
    ",
  0x503c6998u64 => "
      MFWD_NS.fwpmrdcn()[25],
    ",
  0x503c69a8u64 => "
      MFWD_NS.fwpmrdcn()[26],
    ",
  0x503c69b8u64 => "
      MFWD_NS.fwpmrdcn()[27],
    ",
  0x503c69c8u64 => "
      MFWD_NS.fwpmrdcn()[28],
    ",
  0x503c69d8u64 => "
      MFWD_NS.fwpmrdcn()[29],
    ",
  0x503c69e8u64 => "
      MFWD_NS.fwpmrdcn()[30],
    ",
  0x503c69f8u64 => "
      MFWD_NS.fwpmrdcn()[31],
    ",
  0x503c6a00u64 => "
      MFWD_NS.fwfrppcn()[0],
    ",
  0x503c6a08u64 => "
      MFWD_NS.fwfrppcn()[1],
    ",
  0x503c6a10u64 => "
      MFWD_NS.fwfrppcn()[2],
    ",
  0x503c6a18u64 => "
      MFWD_NS.fwfrppcn()[3],
    ",
  0x503c6a20u64 => "
      MFWD_NS.fwfrppcn()[4],
    ",
  0x503c6a28u64 => "
      MFWD_NS.fwfrppcn()[5],
    ",
  0x503c6a30u64 => "
      MFWD_NS.fwfrppcn()[6],
    ",
  0x503c6a38u64 => "
      MFWD_NS.fwfrppcn()[7],
    ",
  0x503c6a40u64 => "
      MFWD_NS.fwfrppcn()[8],
    ",
  0x503c6a48u64 => "
      MFWD_NS.fwfrppcn()[9],
    ",
  0x503c6a50u64 => "
      MFWD_NS.fwfrppcn()[10],
    ",
  0x503c6a58u64 => "
      MFWD_NS.fwfrppcn()[11],
    ",
  0x503c6a60u64 => "
      MFWD_NS.fwfrppcn()[12],
    ",
  0x503c6a68u64 => "
      MFWD_NS.fwfrppcn()[13],
    ",
  0x503c6a70u64 => "
      MFWD_NS.fwfrppcn()[14],
    ",
  0x503c6a78u64 => "
      MFWD_NS.fwfrppcn()[15],
    ",
  0x503c6a80u64 => "
      MFWD_NS.fwfrppcn()[16],
    ",
  0x503c6a88u64 => "
      MFWD_NS.fwfrppcn()[17],
    ",
  0x503c6a90u64 => "
      MFWD_NS.fwfrppcn()[18],
    ",
  0x503c6a98u64 => "
      MFWD_NS.fwfrppcn()[19],
    ",
  0x503c6aa0u64 => "
      MFWD_NS.fwfrppcn()[20],
    ",
  0x503c6aa8u64 => "
      MFWD_NS.fwfrppcn()[21],
    ",
  0x503c6ab0u64 => "
      MFWD_NS.fwfrppcn()[22],
    ",
  0x503c6ab8u64 => "
      MFWD_NS.fwfrppcn()[23],
    ",
  0x503c6ac0u64 => "
      MFWD_NS.fwfrppcn()[24],
    ",
  0x503c6ac8u64 => "
      MFWD_NS.fwfrppcn()[25],
    ",
  0x503c6ad0u64 => "
      MFWD_NS.fwfrppcn()[26],
    ",
  0x503c6ad8u64 => "
      MFWD_NS.fwfrppcn()[27],
    ",
  0x503c6ae0u64 => "
      MFWD_NS.fwfrppcn()[28],
    ",
  0x503c6ae8u64 => "
      MFWD_NS.fwfrppcn()[29],
    ",
  0x503c6af0u64 => "
      MFWD_NS.fwfrppcn()[30],
    ",
  0x503c6af8u64 => "
      MFWD_NS.fwfrppcn()[31],
    ",
  0x503c6b00u64 => "
      MFWD_NS.fwfrppcn()[32],
    ",
  0x503c6b08u64 => "
      MFWD_NS.fwfrppcn()[33],
    ",
  0x503c6b10u64 => "
      MFWD_NS.fwfrppcn()[34],
    ",
  0x503c6b18u64 => "
      MFWD_NS.fwfrppcn()[35],
    ",
  0x503c6b20u64 => "
      MFWD_NS.fwfrppcn()[36],
    ",
  0x503c6b28u64 => "
      MFWD_NS.fwfrppcn()[37],
    ",
  0x503c6b30u64 => "
      MFWD_NS.fwfrppcn()[38],
    ",
  0x503c6b38u64 => "
      MFWD_NS.fwfrppcn()[39],
    ",
  0x503c6b40u64 => "
      MFWD_NS.fwfrppcn()[40],
    ",
  0x503c6b48u64 => "
      MFWD_NS.fwfrppcn()[41],
    ",
  0x503c6b50u64 => "
      MFWD_NS.fwfrppcn()[42],
    ",
  0x503c6b58u64 => "
      MFWD_NS.fwfrppcn()[43],
    ",
  0x503c6b60u64 => "
      MFWD_NS.fwfrppcn()[44],
    ",
  0x503c6b68u64 => "
      MFWD_NS.fwfrppcn()[45],
    ",
  0x503c6b70u64 => "
      MFWD_NS.fwfrppcn()[46],
    ",
  0x503c6b78u64 => "
      MFWD_NS.fwfrppcn()[47],
    ",
  0x503c6b80u64 => "
      MFWD_NS.fwfrppcn()[48],
    ",
  0x503c6b88u64 => "
      MFWD_NS.fwfrppcn()[49],
    ",
  0x503c6b90u64 => "
      MFWD_NS.fwfrppcn()[50],
    ",
  0x503c6b98u64 => "
      MFWD_NS.fwfrppcn()[51],
    ",
  0x503c6ba0u64 => "
      MFWD_NS.fwfrppcn()[52],
    ",
  0x503c6ba8u64 => "
      MFWD_NS.fwfrppcn()[53],
    ",
  0x503c6bb0u64 => "
      MFWD_NS.fwfrppcn()[54],
    ",
  0x503c6bb8u64 => "
      MFWD_NS.fwfrppcn()[55],
    ",
  0x503c6bc0u64 => "
      MFWD_NS.fwfrppcn()[56],
    ",
  0x503c6bc8u64 => "
      MFWD_NS.fwfrppcn()[57],
    ",
  0x503c6bd0u64 => "
      MFWD_NS.fwfrppcn()[58],
    ",
  0x503c6bd8u64 => "
      MFWD_NS.fwfrppcn()[59],
    ",
  0x503c6be0u64 => "
      MFWD_NS.fwfrppcn()[60],
    ",
  0x503c6be8u64 => "
      MFWD_NS.fwfrppcn()[61],
    ",
  0x503c6bf0u64 => "
      MFWD_NS.fwfrppcn()[62],
    ",
  0x503c6bf8u64 => "
      MFWD_NS.fwfrppcn()[63],
    ",
  0x503c6c00u64 => "
      MFWD_NS.fwfrppcn()[64],
    ",
  0x503c6c08u64 => "
      MFWD_NS.fwfrppcn()[65],
    ",
  0x503c6c10u64 => "
      MFWD_NS.fwfrppcn()[66],
    ",
  0x503c6c18u64 => "
      MFWD_NS.fwfrppcn()[67],
    ",
  0x503c6c20u64 => "
      MFWD_NS.fwfrppcn()[68],
    ",
  0x503c6c28u64 => "
      MFWD_NS.fwfrppcn()[69],
    ",
  0x503c6c30u64 => "
      MFWD_NS.fwfrppcn()[70],
    ",
  0x503c6c38u64 => "
      MFWD_NS.fwfrppcn()[71],
    ",
  0x503c6c40u64 => "
      MFWD_NS.fwfrppcn()[72],
    ",
  0x503c6c48u64 => "
      MFWD_NS.fwfrppcn()[73],
    ",
  0x503c6c50u64 => "
      MFWD_NS.fwfrppcn()[74],
    ",
  0x503c6c58u64 => "
      MFWD_NS.fwfrppcn()[75],
    ",
  0x503c6c60u64 => "
      MFWD_NS.fwfrppcn()[76],
    ",
  0x503c6c68u64 => "
      MFWD_NS.fwfrppcn()[77],
    ",
  0x503c6c70u64 => "
      MFWD_NS.fwfrppcn()[78],
    ",
  0x503c6c78u64 => "
      MFWD_NS.fwfrppcn()[79],
    ",
  0x503c6c80u64 => "
      MFWD_NS.fwfrppcn()[80],
    ",
  0x503c6c88u64 => "
      MFWD_NS.fwfrppcn()[81],
    ",
  0x503c6c90u64 => "
      MFWD_NS.fwfrppcn()[82],
    ",
  0x503c6c98u64 => "
      MFWD_NS.fwfrppcn()[83],
    ",
  0x503c6ca0u64 => "
      MFWD_NS.fwfrppcn()[84],
    ",
  0x503c6ca8u64 => "
      MFWD_NS.fwfrppcn()[85],
    ",
  0x503c6cb0u64 => "
      MFWD_NS.fwfrppcn()[86],
    ",
  0x503c6cb8u64 => "
      MFWD_NS.fwfrppcn()[87],
    ",
  0x503c6cc0u64 => "
      MFWD_NS.fwfrppcn()[88],
    ",
  0x503c6cc8u64 => "
      MFWD_NS.fwfrppcn()[89],
    ",
  0x503c6cd0u64 => "
      MFWD_NS.fwfrppcn()[90],
    ",
  0x503c6cd8u64 => "
      MFWD_NS.fwfrppcn()[91],
    ",
  0x503c6ce0u64 => "
      MFWD_NS.fwfrppcn()[92],
    ",
  0x503c6ce8u64 => "
      MFWD_NS.fwfrppcn()[93],
    ",
  0x503c6cf0u64 => "
      MFWD_NS.fwfrppcn()[94],
    ",
  0x503c6cf8u64 => "
      MFWD_NS.fwfrppcn()[95],
    ",
  0x503c6d00u64 => "
      MFWD_NS.fwfrppcn()[96],
    ",
  0x503c6d08u64 => "
      MFWD_NS.fwfrppcn()[97],
    ",
  0x503c6d10u64 => "
      MFWD_NS.fwfrppcn()[98],
    ",
  0x503c6d18u64 => "
      MFWD_NS.fwfrppcn()[99],
    ",
  0x503c6d20u64 => "
      MFWD_NS.fwfrppcn()[100],
    ",
  0x503c6d28u64 => "
      MFWD_NS.fwfrppcn()[101],
    ",
  0x503c6d30u64 => "
      MFWD_NS.fwfrppcn()[102],
    ",
  0x503c6d38u64 => "
      MFWD_NS.fwfrppcn()[103],
    ",
  0x503c6d40u64 => "
      MFWD_NS.fwfrppcn()[104],
    ",
  0x503c6d48u64 => "
      MFWD_NS.fwfrppcn()[105],
    ",
  0x503c6d50u64 => "
      MFWD_NS.fwfrppcn()[106],
    ",
  0x503c6d58u64 => "
      MFWD_NS.fwfrppcn()[107],
    ",
  0x503c6d60u64 => "
      MFWD_NS.fwfrppcn()[108],
    ",
  0x503c6d68u64 => "
      MFWD_NS.fwfrppcn()[109],
    ",
  0x503c6d70u64 => "
      MFWD_NS.fwfrppcn()[110],
    ",
  0x503c6d78u64 => "
      MFWD_NS.fwfrppcn()[111],
    ",
  0x503c6d80u64 => "
      MFWD_NS.fwfrppcn()[112],
    ",
  0x503c6d88u64 => "
      MFWD_NS.fwfrppcn()[113],
    ",
  0x503c6d90u64 => "
      MFWD_NS.fwfrppcn()[114],
    ",
  0x503c6d98u64 => "
      MFWD_NS.fwfrppcn()[115],
    ",
  0x503c6da0u64 => "
      MFWD_NS.fwfrppcn()[116],
    ",
  0x503c6da8u64 => "
      MFWD_NS.fwfrppcn()[117],
    ",
  0x503c6db0u64 => "
      MFWD_NS.fwfrppcn()[118],
    ",
  0x503c6db8u64 => "
      MFWD_NS.fwfrppcn()[119],
    ",
  0x503c6dc0u64 => "
      MFWD_NS.fwfrppcn()[120],
    ",
  0x503c6dc8u64 => "
      MFWD_NS.fwfrppcn()[121],
    ",
  0x503c6dd0u64 => "
      MFWD_NS.fwfrppcn()[122],
    ",
  0x503c6dd8u64 => "
      MFWD_NS.fwfrppcn()[123],
    ",
  0x503c6de0u64 => "
      MFWD_NS.fwfrppcn()[124],
    ",
  0x503c6de8u64 => "
      MFWD_NS.fwfrppcn()[125],
    ",
  0x503c6df0u64 => "
      MFWD_NS.fwfrppcn()[126],
    ",
  0x503c6df8u64 => "
      MFWD_NS.fwfrppcn()[127],
    ",
  0x503c6a04u64 => "
      MFWD_NS.fwfrdpcn()[0],
    ",
  0x503c6a0cu64 => "
      MFWD_NS.fwfrdpcn()[1],
    ",
  0x503c6a14u64 => "
      MFWD_NS.fwfrdpcn()[2],
    ",
  0x503c6a1cu64 => "
      MFWD_NS.fwfrdpcn()[3],
    ",
  0x503c6a24u64 => "
      MFWD_NS.fwfrdpcn()[4],
    ",
  0x503c6a2cu64 => "
      MFWD_NS.fwfrdpcn()[5],
    ",
  0x503c6a34u64 => "
      MFWD_NS.fwfrdpcn()[6],
    ",
  0x503c6a3cu64 => "
      MFWD_NS.fwfrdpcn()[7],
    ",
  0x503c6a44u64 => "
      MFWD_NS.fwfrdpcn()[8],
    ",
  0x503c6a4cu64 => "
      MFWD_NS.fwfrdpcn()[9],
    ",
  0x503c6a54u64 => "
      MFWD_NS.fwfrdpcn()[10],
    ",
  0x503c6a5cu64 => "
      MFWD_NS.fwfrdpcn()[11],
    ",
  0x503c6a64u64 => "
      MFWD_NS.fwfrdpcn()[12],
    ",
  0x503c6a6cu64 => "
      MFWD_NS.fwfrdpcn()[13],
    ",
  0x503c6a74u64 => "
      MFWD_NS.fwfrdpcn()[14],
    ",
  0x503c6a7cu64 => "
      MFWD_NS.fwfrdpcn()[15],
    ",
  0x503c6a84u64 => "
      MFWD_NS.fwfrdpcn()[16],
    ",
  0x503c6a8cu64 => "
      MFWD_NS.fwfrdpcn()[17],
    ",
  0x503c6a94u64 => "
      MFWD_NS.fwfrdpcn()[18],
    ",
  0x503c6a9cu64 => "
      MFWD_NS.fwfrdpcn()[19],
    ",
  0x503c6aa4u64 => "
      MFWD_NS.fwfrdpcn()[20],
    ",
  0x503c6aacu64 => "
      MFWD_NS.fwfrdpcn()[21],
    ",
  0x503c6ab4u64 => "
      MFWD_NS.fwfrdpcn()[22],
    ",
  0x503c6abcu64 => "
      MFWD_NS.fwfrdpcn()[23],
    ",
  0x503c6ac4u64 => "
      MFWD_NS.fwfrdpcn()[24],
    ",
  0x503c6accu64 => "
      MFWD_NS.fwfrdpcn()[25],
    ",
  0x503c6ad4u64 => "
      MFWD_NS.fwfrdpcn()[26],
    ",
  0x503c6adcu64 => "
      MFWD_NS.fwfrdpcn()[27],
    ",
  0x503c6ae4u64 => "
      MFWD_NS.fwfrdpcn()[28],
    ",
  0x503c6aecu64 => "
      MFWD_NS.fwfrdpcn()[29],
    ",
  0x503c6af4u64 => "
      MFWD_NS.fwfrdpcn()[30],
    ",
  0x503c6afcu64 => "
      MFWD_NS.fwfrdpcn()[31],
    ",
  0x503c6b04u64 => "
      MFWD_NS.fwfrdpcn()[32],
    ",
  0x503c6b0cu64 => "
      MFWD_NS.fwfrdpcn()[33],
    ",
  0x503c6b14u64 => "
      MFWD_NS.fwfrdpcn()[34],
    ",
  0x503c6b1cu64 => "
      MFWD_NS.fwfrdpcn()[35],
    ",
  0x503c6b24u64 => "
      MFWD_NS.fwfrdpcn()[36],
    ",
  0x503c6b2cu64 => "
      MFWD_NS.fwfrdpcn()[37],
    ",
  0x503c6b34u64 => "
      MFWD_NS.fwfrdpcn()[38],
    ",
  0x503c6b3cu64 => "
      MFWD_NS.fwfrdpcn()[39],
    ",
  0x503c6b44u64 => "
      MFWD_NS.fwfrdpcn()[40],
    ",
  0x503c6b4cu64 => "
      MFWD_NS.fwfrdpcn()[41],
    ",
  0x503c6b54u64 => "
      MFWD_NS.fwfrdpcn()[42],
    ",
  0x503c6b5cu64 => "
      MFWD_NS.fwfrdpcn()[43],
    ",
  0x503c6b64u64 => "
      MFWD_NS.fwfrdpcn()[44],
    ",
  0x503c6b6cu64 => "
      MFWD_NS.fwfrdpcn()[45],
    ",
  0x503c6b74u64 => "
      MFWD_NS.fwfrdpcn()[46],
    ",
  0x503c6b7cu64 => "
      MFWD_NS.fwfrdpcn()[47],
    ",
  0x503c6b84u64 => "
      MFWD_NS.fwfrdpcn()[48],
    ",
  0x503c6b8cu64 => "
      MFWD_NS.fwfrdpcn()[49],
    ",
  0x503c6b94u64 => "
      MFWD_NS.fwfrdpcn()[50],
    ",
  0x503c6b9cu64 => "
      MFWD_NS.fwfrdpcn()[51],
    ",
  0x503c6ba4u64 => "
      MFWD_NS.fwfrdpcn()[52],
    ",
  0x503c6bacu64 => "
      MFWD_NS.fwfrdpcn()[53],
    ",
  0x503c6bb4u64 => "
      MFWD_NS.fwfrdpcn()[54],
    ",
  0x503c6bbcu64 => "
      MFWD_NS.fwfrdpcn()[55],
    ",
  0x503c6bc4u64 => "
      MFWD_NS.fwfrdpcn()[56],
    ",
  0x503c6bccu64 => "
      MFWD_NS.fwfrdpcn()[57],
    ",
  0x503c6bd4u64 => "
      MFWD_NS.fwfrdpcn()[58],
    ",
  0x503c6bdcu64 => "
      MFWD_NS.fwfrdpcn()[59],
    ",
  0x503c6be4u64 => "
      MFWD_NS.fwfrdpcn()[60],
    ",
  0x503c6becu64 => "
      MFWD_NS.fwfrdpcn()[61],
    ",
  0x503c6bf4u64 => "
      MFWD_NS.fwfrdpcn()[62],
    ",
  0x503c6bfcu64 => "
      MFWD_NS.fwfrdpcn()[63],
    ",
  0x503c6c04u64 => "
      MFWD_NS.fwfrdpcn()[64],
    ",
  0x503c6c0cu64 => "
      MFWD_NS.fwfrdpcn()[65],
    ",
  0x503c6c14u64 => "
      MFWD_NS.fwfrdpcn()[66],
    ",
  0x503c6c1cu64 => "
      MFWD_NS.fwfrdpcn()[67],
    ",
  0x503c6c24u64 => "
      MFWD_NS.fwfrdpcn()[68],
    ",
  0x503c6c2cu64 => "
      MFWD_NS.fwfrdpcn()[69],
    ",
  0x503c6c34u64 => "
      MFWD_NS.fwfrdpcn()[70],
    ",
  0x503c6c3cu64 => "
      MFWD_NS.fwfrdpcn()[71],
    ",
  0x503c6c44u64 => "
      MFWD_NS.fwfrdpcn()[72],
    ",
  0x503c6c4cu64 => "
      MFWD_NS.fwfrdpcn()[73],
    ",
  0x503c6c54u64 => "
      MFWD_NS.fwfrdpcn()[74],
    ",
  0x503c6c5cu64 => "
      MFWD_NS.fwfrdpcn()[75],
    ",
  0x503c6c64u64 => "
      MFWD_NS.fwfrdpcn()[76],
    ",
  0x503c6c6cu64 => "
      MFWD_NS.fwfrdpcn()[77],
    ",
  0x503c6c74u64 => "
      MFWD_NS.fwfrdpcn()[78],
    ",
  0x503c6c7cu64 => "
      MFWD_NS.fwfrdpcn()[79],
    ",
  0x503c6c84u64 => "
      MFWD_NS.fwfrdpcn()[80],
    ",
  0x503c6c8cu64 => "
      MFWD_NS.fwfrdpcn()[81],
    ",
  0x503c6c94u64 => "
      MFWD_NS.fwfrdpcn()[82],
    ",
  0x503c6c9cu64 => "
      MFWD_NS.fwfrdpcn()[83],
    ",
  0x503c6ca4u64 => "
      MFWD_NS.fwfrdpcn()[84],
    ",
  0x503c6cacu64 => "
      MFWD_NS.fwfrdpcn()[85],
    ",
  0x503c6cb4u64 => "
      MFWD_NS.fwfrdpcn()[86],
    ",
  0x503c6cbcu64 => "
      MFWD_NS.fwfrdpcn()[87],
    ",
  0x503c6cc4u64 => "
      MFWD_NS.fwfrdpcn()[88],
    ",
  0x503c6cccu64 => "
      MFWD_NS.fwfrdpcn()[89],
    ",
  0x503c6cd4u64 => "
      MFWD_NS.fwfrdpcn()[90],
    ",
  0x503c6cdcu64 => "
      MFWD_NS.fwfrdpcn()[91],
    ",
  0x503c6ce4u64 => "
      MFWD_NS.fwfrdpcn()[92],
    ",
  0x503c6cecu64 => "
      MFWD_NS.fwfrdpcn()[93],
    ",
  0x503c6cf4u64 => "
      MFWD_NS.fwfrdpcn()[94],
    ",
  0x503c6cfcu64 => "
      MFWD_NS.fwfrdpcn()[95],
    ",
  0x503c6d04u64 => "
      MFWD_NS.fwfrdpcn()[96],
    ",
  0x503c6d0cu64 => "
      MFWD_NS.fwfrdpcn()[97],
    ",
  0x503c6d14u64 => "
      MFWD_NS.fwfrdpcn()[98],
    ",
  0x503c6d1cu64 => "
      MFWD_NS.fwfrdpcn()[99],
    ",
  0x503c6d24u64 => "
      MFWD_NS.fwfrdpcn()[100],
    ",
  0x503c6d2cu64 => "
      MFWD_NS.fwfrdpcn()[101],
    ",
  0x503c6d34u64 => "
      MFWD_NS.fwfrdpcn()[102],
    ",
  0x503c6d3cu64 => "
      MFWD_NS.fwfrdpcn()[103],
    ",
  0x503c6d44u64 => "
      MFWD_NS.fwfrdpcn()[104],
    ",
  0x503c6d4cu64 => "
      MFWD_NS.fwfrdpcn()[105],
    ",
  0x503c6d54u64 => "
      MFWD_NS.fwfrdpcn()[106],
    ",
  0x503c6d5cu64 => "
      MFWD_NS.fwfrdpcn()[107],
    ",
  0x503c6d64u64 => "
      MFWD_NS.fwfrdpcn()[108],
    ",
  0x503c6d6cu64 => "
      MFWD_NS.fwfrdpcn()[109],
    ",
  0x503c6d74u64 => "
      MFWD_NS.fwfrdpcn()[110],
    ",
  0x503c6d7cu64 => "
      MFWD_NS.fwfrdpcn()[111],
    ",
  0x503c6d84u64 => "
      MFWD_NS.fwfrdpcn()[112],
    ",
  0x503c6d8cu64 => "
      MFWD_NS.fwfrdpcn()[113],
    ",
  0x503c6d94u64 => "
      MFWD_NS.fwfrdpcn()[114],
    ",
  0x503c6d9cu64 => "
      MFWD_NS.fwfrdpcn()[115],
    ",
  0x503c6da4u64 => "
      MFWD_NS.fwfrdpcn()[116],
    ",
  0x503c6dacu64 => "
      MFWD_NS.fwfrdpcn()[117],
    ",
  0x503c6db4u64 => "
      MFWD_NS.fwfrdpcn()[118],
    ",
  0x503c6dbcu64 => "
      MFWD_NS.fwfrdpcn()[119],
    ",
  0x503c6dc4u64 => "
      MFWD_NS.fwfrdpcn()[120],
    ",
  0x503c6dccu64 => "
      MFWD_NS.fwfrdpcn()[121],
    ",
  0x503c6dd4u64 => "
      MFWD_NS.fwfrdpcn()[122],
    ",
  0x503c6ddcu64 => "
      MFWD_NS.fwfrdpcn()[123],
    ",
  0x503c6de4u64 => "
      MFWD_NS.fwfrdpcn()[124],
    ",
  0x503c6decu64 => "
      MFWD_NS.fwfrdpcn()[125],
    ",
  0x503c6df4u64 => "
      MFWD_NS.fwfrdpcn()[126],
    ",
  0x503c6dfcu64 => "
      MFWD_NS.fwfrdpcn()[127],
    ",
  0x503c7900u64 => "
      MFWD_NS.fweis0()[0],
    ",
  0x503c7910u64 => "
      MFWD_NS.fweis0()[1],
    ",
  0x503c7920u64 => "
      MFWD_NS.fweis0()[2],
    ",
  0x503c7904u64 => "
      MFWD_NS.fweie0()[0],
    ",
  0x503c7914u64 => "
      MFWD_NS.fweie0()[1],
    ",
  0x503c7924u64 => "
      MFWD_NS.fweie0()[2],
    ",
  0x503c7908u64 => "
      MFWD_NS.fweid0()[0],
    ",
  0x503c7918u64 => "
      MFWD_NS.fweid0()[1],
    ",
  0x503c7928u64 => "
      MFWD_NS.fweid0()[2],
    ",
  0x503c7a00u64 => "
      MFWD_NS.fweis1(),
    ",
  0x503c7a04u64 => "
      MFWD_NS.fweie1(),
    ",
  0x503c7a08u64 => "
      MFWD_NS.fweid1(),
    ",
  0x503c7a10u64 => "
      MFWD_NS.fweis2(),
    ",
  0x503c7a14u64 => "
      MFWD_NS.fweie2(),
    ",
  0x503c7a18u64 => "
      MFWD_NS.fweid2(),
    ",
  0x503c7a40u64 => "
      MFWD_NS.fweis5(),
    ",
  0x503c7a44u64 => "
      MFWD_NS.fweie5(),
    ",
  0x503c7a48u64 => "
      MFWD_NS.fweid5(),
    ",
  0x503c7a50u64 => "
      MFWD_NS.fweis60(),
    ",
  0x503c7a54u64 => "
      MFWD_NS.fweie60(),
    ",
  0x503c7a58u64 => "
      MFWD_NS.fweid60(),
    ",
  0x503c7a60u64 => "
      MFWD_NS.fweis61(),
    ",
  0x503c7a64u64 => "
      MFWD_NS.fweie61(),
    ",
  0x503c7a68u64 => "
      MFWD_NS.fweid61(),
    ",
  0x503c7a70u64 => "
      MFWD_NS.fweis62(),
    ",
  0x503c7a74u64 => "
      MFWD_NS.fweie62(),
    ",
  0x503c7a78u64 => "
      MFWD_NS.fweid62(),
    ",
  0x503c7a80u64 => "
      MFWD_NS.fweis63(),
    ",
  0x503c7a84u64 => "
      MFWD_NS.fweie63(),
    ",
  0x503c7a88u64 => "
      MFWD_NS.fweid63(),
    ",
  0x503c7a90u64 => "
      MFWD_NS.fweis70(),
    ",
  0x503c7a94u64 => "
      MFWD_NS.fweie70(),
    ",
  0x503c7a98u64 => "
      MFWD_NS.fweid70(),
    ",
  0x503c7aa0u64 => "
      MFWD_NS.fweis71(),
    ",
  0x503c7aa4u64 => "
      MFWD_NS.fweie71(),
    ",
  0x503c7aa8u64 => "
      MFWD_NS.fweid71(),
    ",
  0x503c7ab0u64 => "
      MFWD_NS.fweis72(),
    ",
  0x503c7ab4u64 => "
      MFWD_NS.fweie72(),
    ",
  0x503c7ab8u64 => "
      MFWD_NS.fweid72(),
    ",
  0x503c7ac0u64 => "
      MFWD_NS.fweis73(),
    ",
  0x503c7ac4u64 => "
      MFWD_NS.fweie73(),
    ",
  0x503c7ac8u64 => "
      MFWD_NS.fweid73(),
    ",
  0x503c7ad0u64 => "
      MFWD_NS.fweis80(),
    ",
  0x503c7ad4u64 => "
      MFWD_NS.fweie80(),
    ",
  0x503c7ad8u64 => "
      MFWD_NS.fweid80(),
    ",
  0x503c7ae0u64 => "
      MFWD_NS.fweis81(),
    ",
  0x503c7ae4u64 => "
      MFWD_NS.fweie81(),
    ",
  0x503c7ae8u64 => "
      MFWD_NS.fweid81(),
    ",
  0x503c7af0u64 => "
      MFWD_NS.fweis82(),
    ",
  0x503c7af4u64 => "
      MFWD_NS.fweie82(),
    ",
  0x503c7af8u64 => "
      MFWD_NS.fweid82(),
    ",
  0x503c7b00u64 => "
      MFWD_NS.fweis83(),
    ",
  0x503c7b04u64 => "
      MFWD_NS.fweie83(),
    ",
  0x503c7b08u64 => "
      MFWD_NS.fweid83(),
    ",
  0x503c7c00u64 => "
      MFWD_NS.fwmis0(),
    ",
  0x503c7c04u64 => "
      MFWD_NS.fwmie0(),
    ",
  0x503c7c08u64 => "
      MFWD_NS.fwmid0(),
    ",
  0x503c8000u64 => "
      ESWM_NS.tpemimc0(),
    ",
  0x503c8004u64 => "
      ESWM_NS.tpemimc1(),
    ",
  0x503c8008u64 => "
      ESWM_NS.tpemimc2(),
    ",
  0x503c800cu64 => "
      ESWM_NS.tpemimc3(),
    ",
  0x503c8010u64 => "
      ESWM_NS.tpemimc4(),
    ",
  0x503c8080u64 => "
      ESWM_NS.tpemimc6()[0],
    ",
  0x503c8084u64 => "
      ESWM_NS.tpemimc6()[1],
    ",
  0x503c8088u64 => "
      ESWM_NS.tpemimc6()[2],
    ",
  0x503c808cu64 => "
      ESWM_NS.tpemimc6()[3],
    ",
  0x503c8090u64 => "
      ESWM_NS.tpemimc6()[4],
    ",
  0x503c8100u64 => "
      ESWM_NS.tpemimc7()[0],
    ",
  0x503c8104u64 => "
      ESWM_NS.tpemimc7()[1],
    ",
  0x503c8108u64 => "
      ESWM_NS.tpemimc7()[2],
    ",
  0x503c810cu64 => "
      ESWM_NS.tpemimc7()[3],
    ",
  0x503c8110u64 => "
      ESWM_NS.tpemimc7()[4],
    ",
  0x503c8700u64 => "
      ESWM_NS.tsim(),
    ",
  0x503c8704u64 => "
      ESWM_NS.tfim(),
    ",
  0x503c8708u64 => "
      ESWM_NS.tcim(),
    ",
  0x503c8710u64 => "
      ESWM_NS.tgim0(),
    ",
  0x503c8720u64 => "
      ESWM_NS.teim0(),
    ",
  0x503c8724u64 => "
      ESWM_NS.teim1(),
    ",
  0x503e1400u64 => "
      ESWM_NS.miirr(),
    ",
  0x503e1404u64 => "
      ESWM_NS.miicr0(),
    ",
  0x503e1408u64 => "
      ESWM_NS.miicr1(),
    ",
  0x503e1410u64 => "
      ESWM_NS.mccesr(),
    ",
  0x503e1420u64 => "
      ESWM_NS.tasstsr(),
    ",
  0x503c9000u64 => "
      COMA_NS.ripv(),
    ",
  0x503c9004u64 => "
      COMA_NS.rrc(),
    ",
  0x503c9008u64 => "
      COMA_NS.rcec(),
    ",
  0x503c900cu64 => "
      COMA_NS.rcdc(),
    ",
  0x503c9020u64 => "
      COMA_NS.cabpibwmc()[0],
    ",
  0x503c9024u64 => "
      COMA_NS.cabpibwmc()[1],
    ",
  0x503c9028u64 => "
      COMA_NS.cabpibwmc()[2],
    ",
  0x503c902cu64 => "
      COMA_NS.cabpibwmc()[3],
    ",
  0x503c9030u64 => "
      COMA_NS.cabpibwmc()[4],
    ",
  0x503c9034u64 => "
      COMA_NS.cabpibwmc()[5],
    ",
  0x503c9038u64 => "
      COMA_NS.cabpibwmc()[6],
    ",
  0x503c903cu64 => "
      COMA_NS.cabpibwmc()[7],
    ",
  0x503c9040u64 => "
      COMA_NS.cabpwmlc(),
    ",
  0x503c9050u64 => "
      COMA_NS.cabppflci(),
    ",
  0x503c9060u64 => "
      COMA_NS.cabppwmlc()[0],
    ",
  0x503c9064u64 => "
      COMA_NS.cabppwmlc()[1],
    ",
  0x503c9068u64 => "
      COMA_NS.cabppwmlc()[2],
    ",
  0x503c90a0u64 => "
      COMA_NS.cabpppflc0()[0],
    ",
  0x503c90a4u64 => "
      COMA_NS.cabpppflc0()[1],
    ",
  0x503c90a8u64 => "
      COMA_NS.cabpppflc1()[0],
    ",
  0x503c90acu64 => "
      COMA_NS.cabpppflc1()[1],
    ",
  0x503c90b0u64 => "
      COMA_NS.cabpppflc2()[0],
    ",
  0x503c90b4u64 => "
      COMA_NS.cabpppflc2()[1],
    ",
  0x503c9100u64 => "
      COMA_NS.cabpulc()[0],
    ",
  0x503c9104u64 => "
      COMA_NS.cabpulc()[1],
    ",
  0x503c9108u64 => "
      COMA_NS.cabpulc()[2],
    ",
  0x503c9140u64 => "
      COMA_NS.cabpirm(),
    ",
  0x503c9144u64 => "
      COMA_NS.cabppcm(),
    ",
  0x503c9148u64 => "
      COMA_NS.cabplcm(),
    ",
  0x503c9180u64 => "
      COMA_NS.cabpcpm()[0],
    ",
  0x503c9184u64 => "
      COMA_NS.cabpcpm()[1],
    ",
  0x503c9188u64 => "
      COMA_NS.cabpcpm()[2],
    ",
  0x503c9200u64 => "
      COMA_NS.cabpmcpm()[0],
    ",
  0x503c9204u64 => "
      COMA_NS.cabpmcpm()[1],
    ",
  0x503c9208u64 => "
      COMA_NS.cabpmcpm()[2],
    ",
  0x503c9300u64 => "
      COMA_NS.cardnm(),
    ",
  0x503c9304u64 => "
      COMA_NS.cardmnm(),
    ",
  0x503c9310u64 => "
      COMA_NS.cardcn(),
    ",
  0x503c9400u64 => "
      COMA_NS.caeis0(),
    ",
  0x503c9404u64 => "
      COMA_NS.caeie0(),
    ",
  0x503c9408u64 => "
      COMA_NS.caeid0(),
    ",
  0x503c9410u64 => "
      COMA_NS.caeis1(),
    ",
  0x503c9414u64 => "
      COMA_NS.caeie1(),
    ",
  0x503c9418u64 => "
      COMA_NS.caeid1(),
    ",
  0x503c9440u64 => "
      COMA_NS.camis0(),
    ",
  0x503c9444u64 => "
      COMA_NS.camie0(),
    ",
  0x503c9448u64 => "
      COMA_NS.camid0(),
    ",
  0x503c9450u64 => "
      COMA_NS.camis1(),
    ",
  0x503c9454u64 => "
      COMA_NS.camie1(),
    ",
  0x503c9458u64 => "
      COMA_NS.camid1(),
    ",
  0x503ca000u64 => "
      ETHA_0_NS.eamc(),
    ",
  0x503ca004u64 => "
      ETHA_0_NS.eams(),
    ",
  0x503ca010u64 => "
      ETHA_0_NS.eairc(),
    ",
  0x503ca014u64 => "
      ETHA_0_NS.eatdqsc(),
    ",
  0x503ca018u64 => "
      ETHA_0_NS.eatdqc(),
    ",
  0x503ca01cu64 => "
      ETHA_0_NS.eatdqac(),
    ",
  0x503ca020u64 => "
      ETHA_0_NS.eatpec(),
    ",
  0x503ca040u64 => "
      ETHA_0_NS.eatmfsc()[0],
    ",
  0x503ca044u64 => "
      ETHA_0_NS.eatmfsc()[1],
    ",
  0x503ca048u64 => "
      ETHA_0_NS.eatmfsc()[2],
    ",
  0x503ca04cu64 => "
      ETHA_0_NS.eatmfsc()[3],
    ",
  0x503ca050u64 => "
      ETHA_0_NS.eatmfsc()[4],
    ",
  0x503ca054u64 => "
      ETHA_0_NS.eatmfsc()[5],
    ",
  0x503ca058u64 => "
      ETHA_0_NS.eatmfsc()[6],
    ",
  0x503ca05cu64 => "
      ETHA_0_NS.eatmfsc()[7],
    ",
  0x503ca060u64 => "
      ETHA_0_NS.eatdqdc()[0],
    ",
  0x503ca064u64 => "
      ETHA_0_NS.eatdqdc()[1],
    ",
  0x503ca068u64 => "
      ETHA_0_NS.eatdqdc()[2],
    ",
  0x503ca06cu64 => "
      ETHA_0_NS.eatdqdc()[3],
    ",
  0x503ca070u64 => "
      ETHA_0_NS.eatdqdc()[4],
    ",
  0x503ca074u64 => "
      ETHA_0_NS.eatdqdc()[5],
    ",
  0x503ca078u64 => "
      ETHA_0_NS.eatdqdc()[6],
    ",
  0x503ca07cu64 => "
      ETHA_0_NS.eatdqdc()[7],
    ",
  0x503ca080u64 => "
      ETHA_0_NS.eatdqm()[0],
    ",
  0x503ca084u64 => "
      ETHA_0_NS.eatdqm()[1],
    ",
  0x503ca088u64 => "
      ETHA_0_NS.eatdqm()[2],
    ",
  0x503ca08cu64 => "
      ETHA_0_NS.eatdqm()[3],
    ",
  0x503ca090u64 => "
      ETHA_0_NS.eatdqm()[4],
    ",
  0x503ca094u64 => "
      ETHA_0_NS.eatdqm()[5],
    ",
  0x503ca098u64 => "
      ETHA_0_NS.eatdqm()[6],
    ",
  0x503ca09cu64 => "
      ETHA_0_NS.eatdqm()[7],
    ",
  0x503ca0a0u64 => "
      ETHA_0_NS.eatdqmlm()[0],
    ",
  0x503ca0a4u64 => "
      ETHA_0_NS.eatdqmlm()[1],
    ",
  0x503ca0a8u64 => "
      ETHA_0_NS.eatdqmlm()[2],
    ",
  0x503ca0acu64 => "
      ETHA_0_NS.eatdqmlm()[3],
    ",
  0x503ca0b0u64 => "
      ETHA_0_NS.eatdqmlm()[4],
    ",
  0x503ca0b4u64 => "
      ETHA_0_NS.eatdqmlm()[5],
    ",
  0x503ca0b8u64 => "
      ETHA_0_NS.eatdqmlm()[6],
    ",
  0x503ca0bcu64 => "
      ETHA_0_NS.eatdqmlm()[7],
    ",
  0x503ca100u64 => "
      ETHA_0_NS.eactqc(),
    ",
  0x503ca104u64 => "
      ETHA_0_NS.eactdqdc(),
    ",
  0x503ca108u64 => "
      ETHA_0_NS.eactdqm(),
    ",
  0x503ca10cu64 => "
      ETHA_0_NS.eactdqmlm(),
    ",
  0x503ca130u64 => "
      ETHA_0_NS.eavcc(),
    ",
  0x503ca134u64 => "
      ETHA_0_NS.eavtc(),
    ",
  0x503ca138u64 => "
      ETHA_0_NS.eartfc(),
    ",
  0x503ca200u64 => "
      ETHA_0_NS.eacaec(),
    ",
  0x503ca204u64 => "
      ETHA_0_NS.eacc(),
    ",
  0x503ca220u64 => "
      ETHA_0_NS.eacaivc()[0],
    ",
  0x503ca224u64 => "
      ETHA_0_NS.eacaivc()[1],
    ",
  0x503ca228u64 => "
      ETHA_0_NS.eacaivc()[2],
    ",
  0x503ca22cu64 => "
      ETHA_0_NS.eacaivc()[3],
    ",
  0x503ca230u64 => "
      ETHA_0_NS.eacaivc()[4],
    ",
  0x503ca234u64 => "
      ETHA_0_NS.eacaivc()[5],
    ",
  0x503ca238u64 => "
      ETHA_0_NS.eacaivc()[6],
    ",
  0x503ca23cu64 => "
      ETHA_0_NS.eacaivc()[7],
    ",
  0x503ca240u64 => "
      ETHA_0_NS.eacaulcq(),
    ",
  0x503ca260u64 => "
      ETHA_0_NS.eacoem(),
    ",
  0x503ca280u64 => "
      ETHA_0_NS.eacoivm()[0],
    ",
  0x503ca284u64 => "
      ETHA_0_NS.eacoivm()[1],
    ",
  0x503ca288u64 => "
      ETHA_0_NS.eacoivm()[2],
    ",
  0x503ca28cu64 => "
      ETHA_0_NS.eacoivm()[3],
    ",
  0x503ca290u64 => "
      ETHA_0_NS.eacoivm()[4],
    ",
  0x503ca294u64 => "
      ETHA_0_NS.eacoivm()[5],
    ",
  0x503ca298u64 => "
      ETHA_0_NS.eacoivm()[6],
    ",
  0x503ca29cu64 => "
      ETHA_0_NS.eacoivm()[7],
    ",
  0x503ca2a0u64 => "
      ETHA_0_NS.eacoulm()[0],
    ",
  0x503ca2a4u64 => "
      ETHA_0_NS.eacoulm()[1],
    ",
  0x503ca2a8u64 => "
      ETHA_0_NS.eacoulm()[2],
    ",
  0x503ca2acu64 => "
      ETHA_0_NS.eacoulm()[3],
    ",
  0x503ca2b0u64 => "
      ETHA_0_NS.eacoulm()[4],
    ",
  0x503ca2b4u64 => "
      ETHA_0_NS.eacoulm()[5],
    ",
  0x503ca2b8u64 => "
      ETHA_0_NS.eacoulm()[6],
    ",
  0x503ca2bcu64 => "
      ETHA_0_NS.eacoulm()[7],
    ",
  0x503ca2c0u64 => "
      ETHA_0_NS.eacgsm(),
    ",
  0x503ca300u64 => "
      ETHA_0_NS.eatasc(),
    ",
  0x503ca304u64 => "
      ETHA_0_NS.eatasigsc(),
    ",
  0x503ca320u64 => "
      ETHA_0_NS.eatasenc()[0],
    ",
  0x503ca324u64 => "
      ETHA_0_NS.eatasenc()[1],
    ",
  0x503ca328u64 => "
      ETHA_0_NS.eatasenc()[2],
    ",
  0x503ca32cu64 => "
      ETHA_0_NS.eatasenc()[3],
    ",
  0x503ca330u64 => "
      ETHA_0_NS.eatasenc()[4],
    ",
  0x503ca334u64 => "
      ETHA_0_NS.eatasenc()[5],
    ",
  0x503ca338u64 => "
      ETHA_0_NS.eatasenc()[6],
    ",
  0x503ca33cu64 => "
      ETHA_0_NS.eatasenc()[7],
    ",
  0x503ca340u64 => "
      ETHA_0_NS.eatasenc()[8],
      ETHA_0_NS.eatasctenc(),
    ",
  0x503ca360u64 => "
      ETHA_0_NS.eatasenm()[0],
    ",
  0x503ca364u64 => "
      ETHA_0_NS.eatasenm()[1],
    ",
  0x503ca368u64 => "
      ETHA_0_NS.eatasenm()[2],
    ",
  0x503ca36cu64 => "
      ETHA_0_NS.eatasenm()[3],
    ",
  0x503ca370u64 => "
      ETHA_0_NS.eatasenm()[4],
    ",
  0x503ca374u64 => "
      ETHA_0_NS.eatasenm()[5],
    ",
  0x503ca378u64 => "
      ETHA_0_NS.eatasenm()[6],
    ",
  0x503ca37cu64 => "
      ETHA_0_NS.eatasenm()[7],
    ",
  0x503ca380u64 => "
      ETHA_0_NS.eatasenm()[8],
      ETHA_0_NS.eatasctenm(),
    ",
  0x503ca3a0u64 => "
      ETHA_0_NS.eatascstc0(),
    ",
  0x503ca3a4u64 => "
      ETHA_0_NS.eatascstc1(),
    ",
  0x503ca3a8u64 => "
      ETHA_0_NS.eatascstm0(),
    ",
  0x503ca3acu64 => "
      ETHA_0_NS.eatascstm1(),
    ",
  0x503ca3b0u64 => "
      ETHA_0_NS.eatasctc(),
    ",
  0x503ca3b4u64 => "
      ETHA_0_NS.eatasctm(),
    ",
  0x503ca3c0u64 => "
      ETHA_0_NS.eatasgl0(),
    ",
  0x503ca3c4u64 => "
      ETHA_0_NS.eatasgl1(),
    ",
  0x503ca3c8u64 => "
      ETHA_0_NS.eatasglr(),
    ",
  0x503ca3d0u64 => "
      ETHA_0_NS.eatasgr(),
    ",
  0x503ca3d4u64 => "
      ETHA_0_NS.eatasgrr(),
    ",
  0x503ca3e0u64 => "
      ETHA_0_NS.eatashcc(),
    ",
  0x503ca3e4u64 => "
      ETHA_0_NS.eatasrirm(),
    ",
  0x503ca3e8u64 => "
      ETHA_0_NS.eatassm(),
    ",
  0x503ca400u64 => "
      ETHA_0_NS.eausmfsecn(),
    ",
  0x503ca404u64 => "
      ETHA_0_NS.eatfecn(),
    ",
  0x503ca408u64 => "
      ETHA_0_NS.eafsecn(),
    ",
  0x503ca40cu64 => "
      ETHA_0_NS.eadqoecn(),
    ",
  0x503ca410u64 => "
      ETHA_0_NS.eadqsecn(),
    ",
  0x503ca500u64 => "
      ETHA_0_NS.eaeis0(),
    ",
  0x503ca504u64 => "
      ETHA_0_NS.eaeie0(),
    ",
  0x503ca508u64 => "
      ETHA_0_NS.eaeid0(),
    ",
  0x503ca510u64 => "
      ETHA_0_NS.eaeis1(),
    ",
  0x503ca514u64 => "
      ETHA_0_NS.eaeie1(),
    ",
  0x503ca518u64 => "
      ETHA_0_NS.eaeid1(),
    ",
  0x503ca520u64 => "
      ETHA_0_NS.eaeis2(),
    ",
  0x503ca524u64 => "
      ETHA_0_NS.eaeie2(),
    ",
  0x503ca528u64 => "
      ETHA_0_NS.eaeid2(),
    ",
  0x503cb000u64 => "
      RMAC_0_NS.mpsm(),
    ",
  0x503cb004u64 => "
      RMAC_0_NS.mpic(),
    ",
  0x503cb008u64 => "
      RMAC_0_NS.mpim(),
    ",
  0x503cb010u64 => "
      RMAC_0_NS.mioc(),
    ",
  0x503cb020u64 => "
      RMAC_0_NS.mtffc(),
    ",
  0x503cb024u64 => "
      RMAC_0_NS.mtpfc(),
    ",
  0x503cb028u64 => "
      RMAC_0_NS.mtpfc2(),
    ",
  0x503cb030u64 => "
      RMAC_0_NS.mtpfc3t(),
    ",
  0x503cb080u64 => "
      RMAC_0_NS.mrgc(),
    ",
  0x503cb084u64 => "
      RMAC_0_NS.mrmac0(),
    ",
  0x503cb088u64 => "
      RMAC_0_NS.mrmac1(),
    ",
  0x503cb08cu64 => "
      RMAC_0_NS.mrafc(),
    ",
  0x503cb090u64 => "
      RMAC_0_NS.mrsce(),
    ",
  0x503cb094u64 => "
      RMAC_0_NS.mrscp(),
    ",
  0x503cb098u64 => "
      RMAC_0_NS.mrscc(),
    ",
  0x503cb09cu64 => "
      RMAC_0_NS.mrfsce(),
    ",
  0x503cb0a0u64 => "
      RMAC_0_NS.mrfscp(),
    ",
  0x503cb0a4u64 => "
      RMAC_0_NS.mtrc(),
    ",
  0x503cb0acu64 => "
      RMAC_0_NS.mrpfm(),
    ",
  0x503cb100u64 => "
      RMAC_0_NS.mpfc()[0],
    ",
  0x503cb104u64 => "
      RMAC_0_NS.mpfc()[1],
    ",
  0x503cb108u64 => "
      RMAC_0_NS.mpfc()[2],
    ",
  0x503cb10cu64 => "
      RMAC_0_NS.mpfc()[3],
    ",
  0x503cb110u64 => "
      RMAC_0_NS.mpfc()[4],
    ",
  0x503cb114u64 => "
      RMAC_0_NS.mpfc()[5],
    ",
  0x503cb118u64 => "
      RMAC_0_NS.mpfc()[6],
    ",
  0x503cb11cu64 => "
      RMAC_0_NS.mpfc()[7],
    ",
  0x503cb120u64 => "
      RMAC_0_NS.mpfc()[8],
    ",
  0x503cb124u64 => "
      RMAC_0_NS.mpfc()[9],
    ",
  0x503cb128u64 => "
      RMAC_0_NS.mpfc()[10],
    ",
  0x503cb12cu64 => "
      RMAC_0_NS.mpfc()[11],
    ",
  0x503cb130u64 => "
      RMAC_0_NS.mpfc()[12],
    ",
  0x503cb134u64 => "
      RMAC_0_NS.mpfc()[13],
    ",
  0x503cb138u64 => "
      RMAC_0_NS.mpfc()[14],
    ",
  0x503cb13cu64 => "
      RMAC_0_NS.mpfc()[15],
    ",
  0x503cb180u64 => "
      RMAC_0_NS.mlvc(),
    ",
  0x503cb184u64 => "
      RMAC_0_NS.meeec(),
    ",
  0x503cb188u64 => "
      RMAC_0_NS.mlbc(),
    ",
  0x503cb200u64 => "
      RMAC_0_NS.meis(),
    ",
  0x503cb204u64 => "
      RMAC_0_NS.meie(),
    ",
  0x503cb208u64 => "
      RMAC_0_NS.meid(),
    ",
  0x503cb210u64 => "
      RMAC_0_NS.mmis0(),
    ",
  0x503cb214u64 => "
      RMAC_0_NS.mmie0(),
    ",
  0x503cb218u64 => "
      RMAC_0_NS.mmid0(),
    ",
  0x503cb220u64 => "
      RMAC_0_NS.mmis1(),
    ",
  0x503cb224u64 => "
      RMAC_0_NS.mmie1(),
    ",
  0x503cb228u64 => "
      RMAC_0_NS.mmid1(),
    ",
  0x503cb230u64 => "
      RMAC_0_NS.mmis2(),
    ",
  0x503cb234u64 => "
      RMAC_0_NS.mmie2(),
    ",
  0x503cb238u64 => "
      RMAC_0_NS.mmid2(),
    ",
  0x503cb300u64 => "
      RMAC_0_NS.mmpftct(),
    ",
  0x503cb304u64 => "
      RMAC_0_NS.mapftct(),
    ",
  0x503cb308u64 => "
      RMAC_0_NS.mpfrct(),
    ",
  0x503cb30cu64 => "
      RMAC_0_NS.mfcict(),
    ",
  0x503cb310u64 => "
      RMAC_0_NS.meeect(),
    ",
  0x503cb320u64 => "
      RMAC_0_NS.mmpcftct()[0],
    ",
  0x503cb324u64 => "
      RMAC_0_NS.mmpcftct()[1],
    ",
  0x503cb330u64 => "
      RMAC_0_NS.mapcftct()[0],
    ",
  0x503cb334u64 => "
      RMAC_0_NS.mapcftct()[1],
    ",
  0x503cb340u64 => "
      RMAC_0_NS.mpcfrct()[0],
    ",
  0x503cb344u64 => "
      RMAC_0_NS.mpcfrct()[1],
    ",
  0x503cb360u64 => "
      RMAC_0_NS.mrovfc(),
    ",
  0x503cb408u64 => "
      RMAC_0_NS.mrgfce(),
    ",
  0x503cb40cu64 => "
      RMAC_0_NS.mrgfcp(),
    ",
  0x503cb410u64 => "
      RMAC_0_NS.mrbfc(),
    ",
  0x503cb414u64 => "
      RMAC_0_NS.mrmfc(),
    ",
  0x503cb418u64 => "
      RMAC_0_NS.mrufc(),
    ",
  0x503cb41cu64 => "
      RMAC_0_NS.mrpefc(),
    ",
  0x503cb420u64 => "
      RMAC_0_NS.mrnefc(),
    ",
  0x503cb424u64 => "
      RMAC_0_NS.mrfmefc(),
    ",
  0x503cb428u64 => "
      RMAC_0_NS.mrffmefc(),
    ",
  0x503cb42cu64 => "
      RMAC_0_NS.mrcfcefc(),
    ",
  0x503cb430u64 => "
      RMAC_0_NS.mrfcefc(),
    ",
  0x503cb434u64 => "
      RMAC_0_NS.mrrcfefc(),
    ",
  0x503cb438u64 => "
      RMAC_0_NS.mrfc(),
    ",
  0x503cb43cu64 => "
      RMAC_0_NS.mrguefc(),
    ",
  0x503cb440u64 => "
      RMAC_0_NS.mrbuefc(),
    ",
  0x503cb444u64 => "
      RMAC_0_NS.mrgoefc(),
    ",
  0x503cb448u64 => "
      RMAC_0_NS.mrboefc(),
    ",
  0x503cb44cu64 => "
      RMAC_0_NS.mrxbceu(),
    ",
  0x503cb450u64 => "
      RMAC_0_NS.mrxbcel(),
    ",
  0x503cb454u64 => "
      RMAC_0_NS.mrxbcpu(),
    ",
  0x503cb458u64 => "
      RMAC_0_NS.mrxbcpl(),
    ",
  0x503cb508u64 => "
      RMAC_0_NS.mtgfce(),
    ",
  0x503cb50cu64 => "
      RMAC_0_NS.mtgfcp(),
    ",
  0x503cb510u64 => "
      RMAC_0_NS.mtbfc(),
    ",
  0x503cb514u64 => "
      RMAC_0_NS.mtmfc(),
    ",
  0x503cb518u64 => "
      RMAC_0_NS.mtufc(),
    ",
  0x503cb51cu64 => "
      RMAC_0_NS.mtefc(),
    ",
  0x503cb520u64 => "
      RMAC_0_NS.mtxbceu(),
    ",
  0x503cb524u64 => "
      RMAC_0_NS.mtxbcel(),
    ",
  0x503cb528u64 => "
      RMAC_0_NS.mtxbcpu(),
    ",
  0x503cb52cu64 => "
      RMAC_0_NS.mtxbcpl(),
    ",
  0x503ce000u64 => "
      GWCA_0_NS.gwmc(),
    ",
  0x503ce004u64 => "
      GWCA_0_NS.gwms(),
    ",
  0x503ce010u64 => "
      GWCA_0_NS.gwirc(),
    ",
  0x503ce014u64 => "
      GWCA_0_NS.gwrdqsc(),
    ",
  0x503ce018u64 => "
      GWCA_0_NS.gwrdqc(),
    ",
  0x503ce01cu64 => "
      GWCA_0_NS.gwrdqac(),
    ",
  0x503ce020u64 => "
      GWCA_0_NS.gwrgc(),
    ",
  0x503ce040u64 => "
      GWCA_0_NS.gwrmfsc()[0],
    ",
  0x503ce044u64 => "
      GWCA_0_NS.gwrmfsc()[1],
    ",
  0x503ce048u64 => "
      GWCA_0_NS.gwrmfsc()[2],
    ",
  0x503ce04cu64 => "
      GWCA_0_NS.gwrmfsc()[3],
    ",
  0x503ce050u64 => "
      GWCA_0_NS.gwrmfsc()[4],
    ",
  0x503ce054u64 => "
      GWCA_0_NS.gwrmfsc()[5],
    ",
  0x503ce058u64 => "
      GWCA_0_NS.gwrmfsc()[6],
    ",
  0x503ce05cu64 => "
      GWCA_0_NS.gwrmfsc()[7],
    ",
  0x503ce060u64 => "
      GWCA_0_NS.gwrdqdc()[0],
    ",
  0x503ce064u64 => "
      GWCA_0_NS.gwrdqdc()[1],
    ",
  0x503ce068u64 => "
      GWCA_0_NS.gwrdqdc()[2],
    ",
  0x503ce06cu64 => "
      GWCA_0_NS.gwrdqdc()[3],
    ",
  0x503ce070u64 => "
      GWCA_0_NS.gwrdqdc()[4],
    ",
  0x503ce074u64 => "
      GWCA_0_NS.gwrdqdc()[5],
    ",
  0x503ce078u64 => "
      GWCA_0_NS.gwrdqdc()[6],
    ",
  0x503ce07cu64 => "
      GWCA_0_NS.gwrdqdc()[7],
    ",
  0x503ce080u64 => "
      GWCA_0_NS.gwrdqm()[0],
    ",
  0x503ce084u64 => "
      GWCA_0_NS.gwrdqm()[1],
    ",
  0x503ce088u64 => "
      GWCA_0_NS.gwrdqm()[2],
    ",
  0x503ce08cu64 => "
      GWCA_0_NS.gwrdqm()[3],
    ",
  0x503ce090u64 => "
      GWCA_0_NS.gwrdqm()[4],
    ",
  0x503ce094u64 => "
      GWCA_0_NS.gwrdqm()[5],
    ",
  0x503ce098u64 => "
      GWCA_0_NS.gwrdqm()[6],
    ",
  0x503ce09cu64 => "
      GWCA_0_NS.gwrdqm()[7],
    ",
  0x503ce0a0u64 => "
      GWCA_0_NS.gwrdqmlm()[0],
    ",
  0x503ce0a4u64 => "
      GWCA_0_NS.gwrdqmlm()[1],
    ",
  0x503ce0a8u64 => "
      GWCA_0_NS.gwrdqmlm()[2],
    ",
  0x503ce0acu64 => "
      GWCA_0_NS.gwrdqmlm()[3],
    ",
  0x503ce0b0u64 => "
      GWCA_0_NS.gwrdqmlm()[4],
    ",
  0x503ce0b4u64 => "
      GWCA_0_NS.gwrdqmlm()[5],
    ",
  0x503ce0b8u64 => "
      GWCA_0_NS.gwrdqmlm()[6],
    ",
  0x503ce0bcu64 => "
      GWCA_0_NS.gwrdqmlm()[7],
    ",
  0x503ce100u64 => "
      GWCA_0_NS.gwmtirm(),
    ",
  0x503ce104u64 => "
      GWCA_0_NS.gwmstls(),
    ",
  0x503ce108u64 => "
      GWCA_0_NS.gwmstlr(),
    ",
  0x503ce10cu64 => "
      GWCA_0_NS.gwmstss(),
    ",
  0x503ce110u64 => "
      GWCA_0_NS.gwmstsr(),
    ",
  0x503ce120u64 => "
      GWCA_0_NS.gwmac0(),
    ",
  0x503ce124u64 => "
      GWCA_0_NS.gwmac1(),
    ",
  0x503ce130u64 => "
      GWCA_0_NS.gwvcc(),
    ",
  0x503ce134u64 => "
      GWCA_0_NS.gwvtc(),
    ",
  0x503ce138u64 => "
      GWCA_0_NS.gwttfc(),
    ",
  0x503ce140u64 => "
      GWCA_0_NS.gwtdcac0()[0],
    ",
  0x503ce148u64 => "
      GWCA_0_NS.gwtdcac0()[1],
    ",
  0x503ce144u64 => "
      GWCA_0_NS.gwtdcac1()[0],
    ",
  0x503ce14cu64 => "
      GWCA_0_NS.gwtdcac1()[1],
    ",
  0x503ce160u64 => "
      GWCA_0_NS.gwtsdcc()[0],
    ",
  0x503ce164u64 => "
      GWCA_0_NS.gwtsdcc()[1],
    ",
  0x503ce180u64 => "
      GWCA_0_NS.gwtsnm(),
    ",
  0x503ce184u64 => "
      GWCA_0_NS.gwtsmnm(),
    ",
  0x503ce190u64 => "
      GWCA_0_NS.gwac(),
    ",
  0x503ce194u64 => "
      GWCA_0_NS.gwdcbac0(),
    ",
  0x503ce198u64 => "
      GWCA_0_NS.gwdcbac1(),
    ",
  0x503ce1a0u64 => "
      GWCA_0_NS.gwmdnc(),
    ",
  0x503ce200u64 => "
      GWCA_0_NS.gwtrc0(),
    ",
  0x503ce204u64 => "
      GWCA_0_NS.gwtrc1(),
    ",
  0x503ce300u64 => "
      GWCA_0_NS.gwtpcp(),
    ",
  0x503ce380u64 => "
      GWCA_0_NS.gwarirm(),
    ",
  0x503ce400u64 => "
      GWCA_0_NS.gwdcc()[0],
    ",
  0x503ce404u64 => "
      GWCA_0_NS.gwdcc()[1],
    ",
  0x503ce408u64 => "
      GWCA_0_NS.gwdcc()[2],
    ",
  0x503ce40cu64 => "
      GWCA_0_NS.gwdcc()[3],
    ",
  0x503ce410u64 => "
      GWCA_0_NS.gwdcc()[4],
    ",
  0x503ce414u64 => "
      GWCA_0_NS.gwdcc()[5],
    ",
  0x503ce418u64 => "
      GWCA_0_NS.gwdcc()[6],
    ",
  0x503ce41cu64 => "
      GWCA_0_NS.gwdcc()[7],
    ",
  0x503ce420u64 => "
      GWCA_0_NS.gwdcc()[8],
    ",
  0x503ce424u64 => "
      GWCA_0_NS.gwdcc()[9],
    ",
  0x503ce428u64 => "
      GWCA_0_NS.gwdcc()[10],
    ",
  0x503ce42cu64 => "
      GWCA_0_NS.gwdcc()[11],
    ",
  0x503ce430u64 => "
      GWCA_0_NS.gwdcc()[12],
    ",
  0x503ce434u64 => "
      GWCA_0_NS.gwdcc()[13],
    ",
  0x503ce438u64 => "
      GWCA_0_NS.gwdcc()[14],
    ",
  0x503ce43cu64 => "
      GWCA_0_NS.gwdcc()[15],
    ",
  0x503ce440u64 => "
      GWCA_0_NS.gwdcc()[16],
    ",
  0x503ce444u64 => "
      GWCA_0_NS.gwdcc()[17],
    ",
  0x503ce448u64 => "
      GWCA_0_NS.gwdcc()[18],
    ",
  0x503ce44cu64 => "
      GWCA_0_NS.gwdcc()[19],
    ",
  0x503ce450u64 => "
      GWCA_0_NS.gwdcc()[20],
    ",
  0x503ce454u64 => "
      GWCA_0_NS.gwdcc()[21],
    ",
  0x503ce458u64 => "
      GWCA_0_NS.gwdcc()[22],
    ",
  0x503ce45cu64 => "
      GWCA_0_NS.gwdcc()[23],
    ",
  0x503ce460u64 => "
      GWCA_0_NS.gwdcc()[24],
    ",
  0x503ce464u64 => "
      GWCA_0_NS.gwdcc()[25],
    ",
  0x503ce468u64 => "
      GWCA_0_NS.gwdcc()[26],
    ",
  0x503ce46cu64 => "
      GWCA_0_NS.gwdcc()[27],
    ",
  0x503ce470u64 => "
      GWCA_0_NS.gwdcc()[28],
    ",
  0x503ce474u64 => "
      GWCA_0_NS.gwdcc()[29],
    ",
  0x503ce478u64 => "
      GWCA_0_NS.gwdcc()[30],
    ",
  0x503ce47cu64 => "
      GWCA_0_NS.gwdcc()[31],
    ",
  0x503ce480u64 => "
      GWCA_0_NS.gwdcc()[32],
    ",
  0x503ce484u64 => "
      GWCA_0_NS.gwdcc()[33],
    ",
  0x503ce488u64 => "
      GWCA_0_NS.gwdcc()[34],
    ",
  0x503ce48cu64 => "
      GWCA_0_NS.gwdcc()[35],
    ",
  0x503ce490u64 => "
      GWCA_0_NS.gwdcc()[36],
    ",
  0x503ce494u64 => "
      GWCA_0_NS.gwdcc()[37],
    ",
  0x503ce498u64 => "
      GWCA_0_NS.gwdcc()[38],
    ",
  0x503ce49cu64 => "
      GWCA_0_NS.gwdcc()[39],
    ",
  0x503ce4a0u64 => "
      GWCA_0_NS.gwdcc()[40],
    ",
  0x503ce4a4u64 => "
      GWCA_0_NS.gwdcc()[41],
    ",
  0x503ce4a8u64 => "
      GWCA_0_NS.gwdcc()[42],
    ",
  0x503ce4acu64 => "
      GWCA_0_NS.gwdcc()[43],
    ",
  0x503ce4b0u64 => "
      GWCA_0_NS.gwdcc()[44],
    ",
  0x503ce4b4u64 => "
      GWCA_0_NS.gwdcc()[45],
    ",
  0x503ce4b8u64 => "
      GWCA_0_NS.gwdcc()[46],
    ",
  0x503ce4bcu64 => "
      GWCA_0_NS.gwdcc()[47],
    ",
  0x503ce4c0u64 => "
      GWCA_0_NS.gwdcc()[48],
    ",
  0x503ce4c4u64 => "
      GWCA_0_NS.gwdcc()[49],
    ",
  0x503ce4c8u64 => "
      GWCA_0_NS.gwdcc()[50],
    ",
  0x503ce4ccu64 => "
      GWCA_0_NS.gwdcc()[51],
    ",
  0x503ce4d0u64 => "
      GWCA_0_NS.gwdcc()[52],
    ",
  0x503ce4d4u64 => "
      GWCA_0_NS.gwdcc()[53],
    ",
  0x503ce4d8u64 => "
      GWCA_0_NS.gwdcc()[54],
    ",
  0x503ce4dcu64 => "
      GWCA_0_NS.gwdcc()[55],
    ",
  0x503ce4e0u64 => "
      GWCA_0_NS.gwdcc()[56],
    ",
  0x503ce4e4u64 => "
      GWCA_0_NS.gwdcc()[57],
    ",
  0x503ce4e8u64 => "
      GWCA_0_NS.gwdcc()[58],
    ",
  0x503ce4ecu64 => "
      GWCA_0_NS.gwdcc()[59],
    ",
  0x503ce4f0u64 => "
      GWCA_0_NS.gwdcc()[60],
    ",
  0x503ce4f4u64 => "
      GWCA_0_NS.gwdcc()[61],
    ",
  0x503ce4f8u64 => "
      GWCA_0_NS.gwdcc()[62],
    ",
  0x503ce4fcu64 => "
      GWCA_0_NS.gwdcc()[63],
    ",
  0x503ce800u64 => "
      GWCA_0_NS.gwaarss(),
    ",
  0x503ce804u64 => "
      GWCA_0_NS.gwaarsr0(),
    ",
  0x503ce808u64 => "
      GWCA_0_NS.gwaarsr1(),
    ",
  0x503ce840u64 => "
      GWCA_0_NS.gwidauas()[0],
    ",
  0x503ce844u64 => "
      GWCA_0_NS.gwidauas()[1],
    ",
  0x503ce848u64 => "
      GWCA_0_NS.gwidauas()[2],
    ",
  0x503ce84cu64 => "
      GWCA_0_NS.gwidauas()[3],
    ",
  0x503ce880u64 => "
      GWCA_0_NS.gwidasm()[0],
    ",
  0x503ce884u64 => "
      GWCA_0_NS.gwidasm()[1],
    ",
  0x503ce888u64 => "
      GWCA_0_NS.gwidasm()[2],
    ",
  0x503ce88cu64 => "
      GWCA_0_NS.gwidasm()[3],
    ",
  0x503ce900u64 => "
      GWCA_0_NS.gwidasam0()[0],
    ",
  0x503ce908u64 => "
      GWCA_0_NS.gwidasam0()[1],
    ",
  0x503ce910u64 => "
      GWCA_0_NS.gwidasam0()[2],
    ",
  0x503ce918u64 => "
      GWCA_0_NS.gwidasam0()[3],
    ",
  0x503ce904u64 => "
      GWCA_0_NS.gwidasam1()[0],
    ",
  0x503ce90cu64 => "
      GWCA_0_NS.gwidasam1()[1],
    ",
  0x503ce914u64 => "
      GWCA_0_NS.gwidasam1()[2],
    ",
  0x503ce91cu64 => "
      GWCA_0_NS.gwidasam1()[3],
    ",
  0x503ce980u64 => "
      GWCA_0_NS.gwidacam0()[0],
    ",
  0x503ce988u64 => "
      GWCA_0_NS.gwidacam0()[1],
    ",
  0x503ce990u64 => "
      GWCA_0_NS.gwidacam0()[2],
    ",
  0x503ce998u64 => "
      GWCA_0_NS.gwidacam0()[3],
    ",
  0x503ce984u64 => "
      GWCA_0_NS.gwidacam1()[0],
    ",
  0x503ce98cu64 => "
      GWCA_0_NS.gwidacam1()[1],
    ",
  0x503ce994u64 => "
      GWCA_0_NS.gwidacam1()[2],
    ",
  0x503ce99cu64 => "
      GWCA_0_NS.gwidacam1()[3],
    ",
  0x503cea00u64 => "
      GWCA_0_NS.gwgrlc(),
    ",
  0x503cea04u64 => "
      GWCA_0_NS.gwgrlulc(),
    ",
  0x503cea80u64 => "
      GWCA_0_NS.gwrlc()[0],
    ",
  0x503cea88u64 => "
      GWCA_0_NS.gwrlc()[1],
    ",
  0x503cea90u64 => "
      GWCA_0_NS.gwrlc()[2],
    ",
  0x503cea98u64 => "
      GWCA_0_NS.gwrlc()[3],
    ",
  0x503ceaa0u64 => "
      GWCA_0_NS.gwrlc()[4],
    ",
  0x503ceaa8u64 => "
      GWCA_0_NS.gwrlc()[5],
    ",
  0x503ceab0u64 => "
      GWCA_0_NS.gwrlc()[6],
    ",
  0x503ceab8u64 => "
      GWCA_0_NS.gwrlc()[7],
    ",
  0x503cea84u64 => "
      GWCA_0_NS.gwrlulc()[0],
    ",
  0x503cea8cu64 => "
      GWCA_0_NS.gwrlulc()[1],
    ",
  0x503cea94u64 => "
      GWCA_0_NS.gwrlulc()[2],
    ",
  0x503cea9cu64 => "
      GWCA_0_NS.gwrlulc()[3],
    ",
  0x503ceaa4u64 => "
      GWCA_0_NS.gwrlulc()[4],
    ",
  0x503ceaacu64 => "
      GWCA_0_NS.gwrlulc()[5],
    ",
  0x503ceab4u64 => "
      GWCA_0_NS.gwrlulc()[6],
    ",
  0x503ceabcu64 => "
      GWCA_0_NS.gwrlulc()[7],
    ",
  0x503ceb80u64 => "
      GWCA_0_NS.gwidpc(),
    ",
  0x503cf000u64 => "
      GWCA_0_NS.gwrdcn(),
    ",
  0x503cf004u64 => "
      GWCA_0_NS.gwtdcn(),
    ",
  0x503cf008u64 => "
      GWCA_0_NS.gwtscn(),
    ",
  0x503cf00cu64 => "
      GWCA_0_NS.gwtsovfecn(),
    ",
  0x503cf010u64 => "
      GWCA_0_NS.gwusmfsecn(),
    ",
  0x503cf014u64 => "
      GWCA_0_NS.gwtfecn(),
    ",
  0x503cf018u64 => "
      GWCA_0_NS.gwseqecn(),
    ",
  0x503cf020u64 => "
      GWCA_0_NS.gwtxdnecn(),
    ",
  0x503cf024u64 => "
      GWCA_0_NS.gwfsecn(),
    ",
  0x503cf028u64 => "
      GWCA_0_NS.gwtdfecn(),
    ",
  0x503cf02cu64 => "
      GWCA_0_NS.gwtsdnecn(),
    ",
  0x503cf030u64 => "
      GWCA_0_NS.gwdqoecn(),
    ",
  0x503cf034u64 => "
      GWCA_0_NS.gwdqsecn(),
    ",
  0x503cf038u64 => "
      GWCA_0_NS.gwdfecn(),
    ",
  0x503cf03cu64 => "
      GWCA_0_NS.gwdsecn(),
    ",
  0x503cf040u64 => "
      GWCA_0_NS.gwdszecn(),
    ",
  0x503cf044u64 => "
      GWCA_0_NS.gwdctecn(),
    ",
  0x503cf048u64 => "
      GWCA_0_NS.gwrxdnecn(),
    ",
  0x503cf100u64 => "
      GWCA_0_NS.gwdis0(),
    ",
  0x503cf104u64 => "
      GWCA_0_NS.gwdie0(),
    ",
  0x503cf108u64 => "
      GWCA_0_NS.gwdid0(),
    ",
  0x503cf10cu64 => "
      GWCA_0_NS.gwdids0(),
    ",
  0x503cf110u64 => "
      GWCA_0_NS.gwdis1(),
    ",
  0x503cf114u64 => "
      GWCA_0_NS.gwdie1(),
    ",
  0x503cf118u64 => "
      GWCA_0_NS.gwdid1(),
    ",
  0x503cf11cu64 => "
      GWCA_0_NS.gwdids1(),
    ",
  0x503cf180u64 => "
      GWCA_0_NS.gwtsdis(),
    ",
  0x503cf184u64 => "
      GWCA_0_NS.gwtsdie(),
    ",
  0x503cf188u64 => "
      GWCA_0_NS.gwtsdid(),
    ",
  0x503cf190u64 => "
      GWCA_0_NS.gweis0(),
    ",
  0x503cf194u64 => "
      GWCA_0_NS.gweie0(),
    ",
  0x503cf198u64 => "
      GWCA_0_NS.gweid0(),
    ",
  0x503cf1a0u64 => "
      GWCA_0_NS.gweis1(),
    ",
  0x503cf1a4u64 => "
      GWCA_0_NS.gweie1(),
    ",
  0x503cf1a8u64 => "
      GWCA_0_NS.gweid1(),
    ",
  0x503cf200u64 => "
      GWCA_0_NS.gweis20(),
    ",
  0x503cf204u64 => "
      GWCA_0_NS.gweie20(),
    ",
  0x503cf208u64 => "
      GWCA_0_NS.gweid20(),
    ",
  0x503cf210u64 => "
      GWCA_0_NS.gweis21(),
    ",
  0x503cf214u64 => "
      GWCA_0_NS.gweie21(),
    ",
  0x503cf218u64 => "
      GWCA_0_NS.gweid21(),
    ",
  0x503cf280u64 => "
      GWCA_0_NS.gweis3(),
    ",
  0x503cf284u64 => "
      GWCA_0_NS.gweie3(),
    ",
  0x503cf288u64 => "
      GWCA_0_NS.gweid3(),
    ",
  0x503cf290u64 => "
      GWCA_0_NS.gweis4(),
    ",
  0x503cf294u64 => "
      GWCA_0_NS.gweie4(),
    ",
  0x503cf298u64 => "
      GWCA_0_NS.gweid4(),
    ",
  0x503cf2a0u64 => "
      GWCA_0_NS.gweis5(),
    ",
  0x503cf2a4u64 => "
      GWCA_0_NS.gweie5(),
    ",
  0x503cf2a8u64 => "
      GWCA_0_NS.gweid5(),
    ",
  0x503da000u64 => "
      GWCA_0_NS.gwidc()[0],
    ",
  0x503da004u64 => "
      GWCA_0_NS.gwidc()[1],
    ",
  0x503da008u64 => "
      GWCA_0_NS.gwidc()[2],
    ",
  0x503da00cu64 => "
      GWCA_0_NS.gwidc()[3],
    ",
  0x503da010u64 => "
      GWCA_0_NS.gwidc()[4],
    ",
  0x503da014u64 => "
      GWCA_0_NS.gwidc()[5],
    ",
  0x503da018u64 => "
      GWCA_0_NS.gwidc()[6],
    ",
  0x503da01cu64 => "
      GWCA_0_NS.gwidc()[7],
    ",
  0x503da020u64 => "
      GWCA_0_NS.gwidc()[8],
    ",
  0x503da024u64 => "
      GWCA_0_NS.gwidc()[9],
    ",
  0x503da028u64 => "
      GWCA_0_NS.gwidc()[10],
    ",
  0x503da02cu64 => "
      GWCA_0_NS.gwidc()[11],
    ",
  0x503da030u64 => "
      GWCA_0_NS.gwidc()[12],
    ",
  0x503da034u64 => "
      GWCA_0_NS.gwidc()[13],
    ",
  0x503da038u64 => "
      GWCA_0_NS.gwidc()[14],
    ",
  0x503da03cu64 => "
      GWCA_0_NS.gwidc()[15],
    ",
  0x503da040u64 => "
      GWCA_0_NS.gwidc()[16],
    ",
  0x503da044u64 => "
      GWCA_0_NS.gwidc()[17],
    ",
  0x503da048u64 => "
      GWCA_0_NS.gwidc()[18],
    ",
  0x503da04cu64 => "
      GWCA_0_NS.gwidc()[19],
    ",
  0x503da050u64 => "
      GWCA_0_NS.gwidc()[20],
    ",
  0x503da054u64 => "
      GWCA_0_NS.gwidc()[21],
    ",
  0x503da058u64 => "
      GWCA_0_NS.gwidc()[22],
    ",
  0x503da05cu64 => "
      GWCA_0_NS.gwidc()[23],
    ",
  0x503da060u64 => "
      GWCA_0_NS.gwidc()[24],
    ",
  0x503da064u64 => "
      GWCA_0_NS.gwidc()[25],
    ",
  0x503da068u64 => "
      GWCA_0_NS.gwidc()[26],
    ",
  0x503da06cu64 => "
      GWCA_0_NS.gwidc()[27],
    ",
  0x503da070u64 => "
      GWCA_0_NS.gwidc()[28],
    ",
  0x503da074u64 => "
      GWCA_0_NS.gwidc()[29],
    ",
  0x503da078u64 => "
      GWCA_0_NS.gwidc()[30],
    ",
  0x503da07cu64 => "
      GWCA_0_NS.gwidc()[31],
    ",
  0x503da080u64 => "
      GWCA_0_NS.gwidc()[32],
    ",
  0x503da084u64 => "
      GWCA_0_NS.gwidc()[33],
    ",
  0x503da088u64 => "
      GWCA_0_NS.gwidc()[34],
    ",
  0x503da08cu64 => "
      GWCA_0_NS.gwidc()[35],
    ",
  0x503da090u64 => "
      GWCA_0_NS.gwidc()[36],
    ",
  0x503da094u64 => "
      GWCA_0_NS.gwidc()[37],
    ",
  0x503da098u64 => "
      GWCA_0_NS.gwidc()[38],
    ",
  0x503da09cu64 => "
      GWCA_0_NS.gwidc()[39],
    ",
  0x503da0a0u64 => "
      GWCA_0_NS.gwidc()[40],
    ",
  0x503da0a4u64 => "
      GWCA_0_NS.gwidc()[41],
    ",
  0x503da0a8u64 => "
      GWCA_0_NS.gwidc()[42],
    ",
  0x503da0acu64 => "
      GWCA_0_NS.gwidc()[43],
    ",
  0x503da0b0u64 => "
      GWCA_0_NS.gwidc()[44],
    ",
  0x503da0b4u64 => "
      GWCA_0_NS.gwidc()[45],
    ",
  0x503da0b8u64 => "
      GWCA_0_NS.gwidc()[46],
    ",
  0x503da0bcu64 => "
      GWCA_0_NS.gwidc()[47],
    ",
  0x503da0c0u64 => "
      GWCA_0_NS.gwidc()[48],
    ",
  0x503da0c4u64 => "
      GWCA_0_NS.gwidc()[49],
    ",
  0x503da0c8u64 => "
      GWCA_0_NS.gwidc()[50],
    ",
  0x503da0ccu64 => "
      GWCA_0_NS.gwidc()[51],
    ",
  0x503da0d0u64 => "
      GWCA_0_NS.gwidc()[52],
    ",
  0x503da0d4u64 => "
      GWCA_0_NS.gwidc()[53],
    ",
  0x503da0d8u64 => "
      GWCA_0_NS.gwidc()[54],
    ",
  0x503da0dcu64 => "
      GWCA_0_NS.gwidc()[55],
    ",
  0x503da0e0u64 => "
      GWCA_0_NS.gwidc()[56],
    ",
  0x503da0e4u64 => "
      GWCA_0_NS.gwidc()[57],
    ",
  0x503da0e8u64 => "
      GWCA_0_NS.gwidc()[58],
    ",
  0x503da0ecu64 => "
      GWCA_0_NS.gwidc()[59],
    ",
  0x503da0f0u64 => "
      GWCA_0_NS.gwidc()[60],
    ",
  0x503da0f4u64 => "
      GWCA_0_NS.gwidc()[61],
    ",
  0x503da0f8u64 => "
      GWCA_0_NS.gwidc()[62],
    ",
  0x503da0fcu64 => "
      GWCA_0_NS.gwidc()[63],
    ",
  0x503e0000u64 => "
      GPTP_NS.ptpipv(),
    ",
  0x503e0010u64 => "
      GPTP_NS.ptptmec(),
    ",
  0x503e0014u64 => "
      GPTP_NS.ptptmdc(),
    ",
  0x503e0020u64 => "
      GPTP_NS.ptptivc()[0],
    ",
  0x503e0060u64 => "
      GPTP_NS.ptptivc()[1],
    ",
  0x503e0030u64 => "
      GPTP_NS.ptptovcl()[0],
    ",
  0x503e0070u64 => "
      GPTP_NS.ptptovcl()[1],
    ",
  0x503e0034u64 => "
      GPTP_NS.ptptovcm()[0],
    ",
  0x503e0074u64 => "
      GPTP_NS.ptptovcm()[1],
    ",
  0x503e0038u64 => "
      GPTP_NS.ptptovcu()[0],
    ",
  0x503e0078u64 => "
      GPTP_NS.ptptovcu()[1],
    ",
  0x503e0040u64 => "
      GPTP_NS.ptpavtptml()[0],
    ",
  0x503e0080u64 => "
      GPTP_NS.ptpavtptml()[1],
    ",
  0x503e0044u64 => "
      GPTP_NS.ptpavtptmu()[0],
    ",
  0x503e0084u64 => "
      GPTP_NS.ptpavtptmu()[1],
    ",
  0x503e0050u64 => "
      GPTP_NS.ptpgptptml()[0],
    ",
  0x503e0090u64 => "
      GPTP_NS.ptpgptptml()[1],
    ",
  0x503e0054u64 => "
      GPTP_NS.ptpgptptmm()[0],
    ",
  0x503e0094u64 => "
      GPTP_NS.ptpgptptmm()[1],
    ",
  0x503e0058u64 => "
      GPTP_NS.ptpgptptmu()[0],
    ",
  0x503e0098u64 => "
      GPTP_NS.ptpgptptmu()[1],
    ",
  0x503e0200u64 => "
      GPTP_NS.ptpmccc()[0],
    ",
  0x503e0210u64 => "
      GPTP_NS.ptpmccc()[1],
    ",
  0x503e0204u64 => "
      GPTP_NS.ptpmccml()[0],
    ",
  0x503e0214u64 => "
      GPTP_NS.ptpmccml()[1],
    ",
  0x503e0208u64 => "
      GPTP_NS.ptpmccmm()[0],
    ",
  0x503e0218u64 => "
      GPTP_NS.ptpmccmm()[1],
    ",
  0x503e020cu64 => "
      GPTP_NS.ptpmccmu()[0],
    ",
  0x503e021cu64 => "
      GPTP_NS.ptpmccmu()[1],
    ",
  0x503e0300u64 => "
      GPTP_NS.ptpmcrc()[0],
    ",
  0x503e0310u64 => "
      GPTP_NS.ptpmcrc()[1],
    ",
  0x503e0304u64 => "
      GPTP_NS.ptpmcrtcl()[0],
    ",
  0x503e0314u64 => "
      GPTP_NS.ptpmcrtcl()[1],
    ",
  0x503e0308u64 => "
      GPTP_NS.ptpmcrtcm()[0],
    ",
  0x503e0318u64 => "
      GPTP_NS.ptpmcrtcm()[1],
    ",
  0x503e030cu64 => "
      GPTP_NS.ptpmcrtcu()[0],
    ",
  0x503e031cu64 => "
      GPTP_NS.ptpmcrtcu()[1],
    ",
  0x503e0400u64 => "
      GPTP_NS.ptpmcpc()[0],
    ",
  0x503e0404u64 => "
      GPTP_NS.ptpmcpc()[1],
    ",
  0x503e0500u64 => "
      GPTP_NS.ptpccc0()[0],
    ",
  0x503e0508u64 => "
      GPTP_NS.ptpccc0()[1],
    ",
  0x503e0510u64 => "
      GPTP_NS.ptpccc0()[2],
    ",
  0x503e0518u64 => "
      GPTP_NS.ptpccc0()[3],
    ",
  0x503e0520u64 => "
      GPTP_NS.ptpccc0()[4],
    ",
  0x503e0528u64 => "
      GPTP_NS.ptpccc0()[5],
    ",
  0x503e0530u64 => "
      GPTP_NS.ptpccc0()[6],
    ",
  0x503e0538u64 => "
      GPTP_NS.ptpccc0()[7],
    ",
  0x503e0504u64 => "
      GPTP_NS.ptpccc1()[0],
    ",
  0x503e050cu64 => "
      GPTP_NS.ptpccc1()[1],
    ",
  0x503e0514u64 => "
      GPTP_NS.ptpccc1()[2],
    ",
  0x503e051cu64 => "
      GPTP_NS.ptpccc1()[3],
    ",
  0x503e0524u64 => "
      GPTP_NS.ptpccc1()[4],
    ",
  0x503e052cu64 => "
      GPTP_NS.ptpccc1()[5],
    ",
  0x503e0534u64 => "
      GPTP_NS.ptpccc1()[6],
    ",
  0x503e053cu64 => "
      GPTP_NS.ptpccc1()[7],
    ",
  0x503e0700u64 => "
      GPTP_NS.ptpis0(),
    ",
  0x503e0704u64 => "
      GPTP_NS.ptpie0(),
    ",
  0x503e0708u64 => "
      GPTP_NS.ptpid0(),
    ",
  0x503e0710u64 => "
      GPTP_NS.ptpis1(),
    ",
  0x503e0714u64 => "
      GPTP_NS.ptpie1(),
    ",
  0x503e0718u64 => "
      GPTP_NS.ptpid1(),
    ",
  0x503e1000u64 => "
      GPTP_NS.potcfgr(),
    ",
  0x503e1004u64 => "
      GPTP_NS.potcprl()[0],
      GPTP_NS.potcprm()[0],
      GPTP_NS.potcpru()[0],
      GPTP_NS.potcr()[0],
      GPTP_NS.potperl()[0],
      GPTP_NS.potperm()[0],
      GPTP_NS.potpwr()[0],
      GPTP_NS.potstrl()[0],
      GPTP_NS.potstrm()[0],
    ",
  0x503e1034u64 => "
      GPTP_NS.potcprl()[1],
      GPTP_NS.potcprm()[1],
      GPTP_NS.potcpru()[1],
      GPTP_NS.potcr()[1],
      GPTP_NS.potperl()[1],
      GPTP_NS.potperm()[1],
      GPTP_NS.potpwr()[1],
      GPTP_NS.potstrl()[1],
      GPTP_NS.potstrm()[1],
    ",
  0x503e1064u64 => "
      GPTP_NS.potcprl()[2],
      GPTP_NS.potcprm()[2],
      GPTP_NS.potcpru()[2],
      GPTP_NS.potcr()[2],
      GPTP_NS.potperl()[2],
      GPTP_NS.potperm()[2],
      GPTP_NS.potpwr()[2],
      GPTP_NS.potstrl()[2],
      GPTP_NS.potstrm()[2],
    ",
  0x503e1094u64 => "
      GPTP_NS.potcprl()[3],
      GPTP_NS.potcprm()[3],
      GPTP_NS.potcpru()[3],
      GPTP_NS.potcr()[3],
      GPTP_NS.potperl()[3],
      GPTP_NS.potperm()[3],
      GPTP_NS.potpwr()[3],
      GPTP_NS.potstrl()[3],
      GPTP_NS.potstrm()[3],
    ",
  0x503e1008u64 => "
      GPTP_NS.potperu()[0],
      GPTP_NS.potstru()[0],
    ",
  0x503e1038u64 => "
      GPTP_NS.potperu()[1],
      GPTP_NS.potstru()[1],
    ",
  0x503e1068u64 => "
      GPTP_NS.potperu()[2],
      GPTP_NS.potstru()[2],
    ",
  0x503e1098u64 => "
      GPTP_NS.potperu()[3],
      GPTP_NS.potstru()[3],
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
  0x50400006u64 => "
      PORT_0_NS.eidr(),
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
      PORT_1_NS.eidr(),
      PORT_1_NS.pidr(),
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
  0x50400140u64 => "
      PORTA_NS.pcntr1(),
      PORTA_NS.pdr(),
    ",
  0x50400142u64 => "
      PORTA_NS.podr(),
    ",
  0x50400144u64 => "
      PORTA_NS.pcntr2(),
      PORTA_NS.pidr(),
    ",
  0x50400146u64 => "
      PORTA_NS.eidr(),
    ",
  0x50400148u64 => "
      PORTA_NS.pcntr3(),
      PORTA_NS.posr(),
    ",
  0x5040014au64 => "
      PORTA_NS.porr(),
    ",
  0x50400160u64 => "
      PORTB_NS.pcntr1(),
      PORTB_NS.pdr(),
    ",
  0x50400162u64 => "
      PORTB_NS.podr(),
    ",
  0x50400164u64 => "
      PORTB_NS.pcntr2(),
      PORTB_NS.pidr(),
    ",
  0x50400166u64 => "
      PORTB_NS.eidr(),
    ",
  0x50400168u64 => "
      PORTB_NS.pcntr3(),
      PORTB_NS.posr(),
    ",
  0x5040016au64 => "
      PORTB_NS.porr(),
    ",
  0x50400180u64 => "
      PORTC_NS.pcntr1(),
      PORTC_NS.pdr(),
    ",
  0x50400182u64 => "
      PORTC_NS.podr(),
    ",
  0x50400184u64 => "
      PORTC_NS.pcntr2(),
      PORTC_NS.pidr(),
    ",
  0x50400186u64 => "
      PORTC_NS.eidr(),
    ",
  0x50400188u64 => "
      PORTC_NS.pcntr3(),
      PORTC_NS.posr(),
    ",
  0x5040018au64 => "
      PORTC_NS.porr(),
    ",
  0x504001a0u64 => "
      PORTD_NS.pcntr1(),
      PORTD_NS.pdr(),
    ",
  0x504001a2u64 => "
      PORTD_NS.podr(),
    ",
  0x504001a4u64 => "
      PORTD_NS.pcntr2(),
      PORTD_NS.pidr(),
    ",
  0x504001a6u64 => "
      PORTD_NS.eidr(),
    ",
  0x504001a8u64 => "
      PORTD_NS.pcntr3(),
      PORTD_NS.posr(),
    ",
  0x504001aau64 => "
      PORTD_NS.porr(),
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
  0x50400828u64 => "
      PFS_NS.p0pfs()[0],
      PFS_NS.p0pfs_ha()[0],
      PFS_NS.p0pfs_by()[0],
    ",
  0x5040082cu64 => "
      PFS_NS.p0pfs()[1],
      PFS_NS.p0pfs_ha()[1],
      PFS_NS.p0pfs_by()[1],
    ",
  0x50400830u64 => "
      PFS_NS.p0pfs()[2],
      PFS_NS.p0pfs_ha()[2],
      PFS_NS.p0pfs_by()[2],
    ",
  0x50400834u64 => "
      PFS_NS.p0pfs()[3],
      PFS_NS.p0pfs_ha()[3],
      PFS_NS.p0pfs_by()[3],
    ",
  0x50400838u64 => "
      PFS_NS.p0pfs()[4],
      PFS_NS.p0pfs_ha()[4],
      PFS_NS.p0pfs_by()[4],
    ",
  0x5040083cu64 => "
      PFS_NS.p0pfs()[5],
      PFS_NS.p0pfs_ha()[5],
      PFS_NS.p0pfs_by()[5],
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
  0x50400860u64 => "
      PFS_NS.p10pfs()[8],
      PFS_NS.p10pfs_ha()[8],
      PFS_NS.p10pfs_by()[8],
    ",
  0x50400864u64 => "
      PFS_NS.p10pfs()[9],
      PFS_NS.p10pfs_ha()[9],
      PFS_NS.p10pfs_by()[9],
    ",
  0x50400868u64 => "
      PFS_NS.p1pfs()[0],
      PFS_NS.p1pfs_ha()[0],
      PFS_NS.p1pfs_by()[0],
    ",
  0x5040086cu64 => "
      PFS_NS.p1pfs()[1],
      PFS_NS.p1pfs_ha()[1],
      PFS_NS.p1pfs_by()[1],
    ",
  0x50400870u64 => "
      PFS_NS.p1pfs()[2],
      PFS_NS.p1pfs_ha()[2],
      PFS_NS.p1pfs_by()[2],
    ",
  0x50400874u64 => "
      PFS_NS.p1pfs()[3],
      PFS_NS.p1pfs_ha()[3],
      PFS_NS.p1pfs_by()[3],
    ",
  0x50400878u64 => "
      PFS_NS.p1pfs()[4],
      PFS_NS.p1pfs_ha()[4],
      PFS_NS.p1pfs_by()[4],
    ",
  0x5040087cu64 => "
      PFS_NS.p1pfs()[5],
      PFS_NS.p1pfs_ha()[5],
      PFS_NS.p1pfs_by()[5],
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
      PFS_NS.p2pfs()[0],
      PFS_NS.p2pfs_ha()[0],
      PFS_NS.p2pfs_by()[0],
    ",
  0x504008d0u64 => "
      PFS_NS.p30pfs()[4],
      PFS_NS.p30pfs_ha()[4],
      PFS_NS.p30pfs_by()[4],
      PFS_NS.p2pfs()[1],
      PFS_NS.p2pfs_ha()[1],
      PFS_NS.p2pfs_by()[1],
    ",
  0x504008d4u64 => "
      PFS_NS.p30pfs()[5],
      PFS_NS.p30pfs_ha()[5],
      PFS_NS.p30pfs_by()[5],
      PFS_NS.p2pfs()[2],
      PFS_NS.p2pfs_ha()[2],
      PFS_NS.p2pfs_by()[2],
    ",
  0x504008d8u64 => "
      PFS_NS.p30pfs()[6],
      PFS_NS.p30pfs_ha()[6],
      PFS_NS.p30pfs_by()[6],
      PFS_NS.p2pfs()[3],
      PFS_NS.p2pfs_ha()[3],
      PFS_NS.p2pfs_by()[3],
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
  0x50400a28u64 => "
      PFS_NS.p8pfs()[0],
      PFS_NS.p8pfs_ha()[0],
      PFS_NS.p8pfs_by()[0],
    ",
  0x50400a2cu64 => "
      PFS_NS.p8pfs()[1],
      PFS_NS.p8pfs_ha()[1],
      PFS_NS.p8pfs_by()[1],
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
      PFS_NS.p814pfs(),
      PFS_NS.p814pfs_ha(),
      PFS_NS.p814pfs_by(),
    ",
  0x50400a3cu64 => "
      PFS_NS.p815pfs(),
      PFS_NS.p815pfs_ha(),
      PFS_NS.p815pfs_by(),
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
  0x50400b00u64 => "
      PFS_NS.pc0pfs()[0],
      PFS_NS.pc0pfs_ha()[0],
      PFS_NS.pc0pfs_by()[0],
    ",
  0x50400b04u64 => "
      PFS_NS.pc0pfs()[1],
      PFS_NS.pc0pfs_ha()[1],
      PFS_NS.pc0pfs_by()[1],
    ",
  0x50400b08u64 => "
      PFS_NS.pc0pfs()[2],
      PFS_NS.pc0pfs_ha()[2],
      PFS_NS.pc0pfs_by()[2],
    ",
  0x50400b0cu64 => "
      PFS_NS.pc0pfs()[3],
      PFS_NS.pc0pfs_ha()[3],
      PFS_NS.pc0pfs_by()[3],
    ",
  0x50400b10u64 => "
      PFS_NS.pc0pfs()[4],
      PFS_NS.pc0pfs_ha()[4],
      PFS_NS.pc0pfs_by()[4],
    ",
  0x50400b14u64 => "
      PFS_NS.pc0pfs()[5],
      PFS_NS.pc0pfs_ha()[5],
      PFS_NS.pc0pfs_by()[5],
    ",
  0x50400b18u64 => "
      PFS_NS.pc0pfs()[6],
      PFS_NS.pc0pfs_ha()[6],
      PFS_NS.pc0pfs_by()[6],
    ",
  0x50400b1cu64 => "
      PFS_NS.pc0pfs()[7],
      PFS_NS.pc0pfs_ha()[7],
      PFS_NS.pc0pfs_by()[7],
    ",
  0x50400b20u64 => "
      PFS_NS.pc0pfs()[8],
      PFS_NS.pc0pfs_ha()[8],
      PFS_NS.pc0pfs_by()[8],
    ",
  0x50400b24u64 => "
      PFS_NS.pc0pfs()[9],
      PFS_NS.pc0pfs_ha()[9],
      PFS_NS.pc0pfs_by()[9],
    ",
  0x50400b28u64 => "
      PFS_NS.pcpfs()[0],
      PFS_NS.pcpfs_ha()[0],
      PFS_NS.pcpfs_by()[0],
    ",
  0x50400b2cu64 => "
      PFS_NS.pcpfs()[1],
      PFS_NS.pcpfs_ha()[1],
      PFS_NS.pcpfs_by()[1],
    ",
  0x50400b30u64 => "
      PFS_NS.pcpfs()[2],
      PFS_NS.pcpfs_ha()[2],
      PFS_NS.pcpfs_by()[2],
    ",
  0x50400b34u64 => "
      PFS_NS.pcpfs()[3],
      PFS_NS.pcpfs_ha()[3],
      PFS_NS.pcpfs_by()[3],
    ",
  0x50400b38u64 => "
      PFS_NS.pcpfs()[4],
      PFS_NS.pcpfs_ha()[4],
      PFS_NS.pcpfs_by()[4],
    ",
  0x50400b3cu64 => "
      PFS_NS.pcpfs()[5],
      PFS_NS.pcpfs_ha()[5],
      PFS_NS.pcpfs_by()[5],
    ",
  0x50400b40u64 => "
      PFS_NS.pd0pfs()[0],
      PFS_NS.pd0pfs_ha()[0],
      PFS_NS.pd0pfs_by()[0],
    ",
  0x50400b44u64 => "
      PFS_NS.pd0pfs()[1],
      PFS_NS.pd0pfs_ha()[1],
      PFS_NS.pd0pfs_by()[1],
    ",
  0x50400b48u64 => "
      PFS_NS.pd0pfs()[2],
      PFS_NS.pd0pfs_ha()[2],
      PFS_NS.pd0pfs_by()[2],
    ",
  0x50400b4cu64 => "
      PFS_NS.pd0pfs()[3],
      PFS_NS.pd0pfs_ha()[3],
      PFS_NS.pd0pfs_by()[3],
    ",
  0x50400b50u64 => "
      PFS_NS.pd0pfs()[4],
      PFS_NS.pd0pfs_ha()[4],
      PFS_NS.pd0pfs_by()[4],
    ",
  0x50400b54u64 => "
      PFS_NS.pd0pfs()[5],
      PFS_NS.pd0pfs_ha()[5],
      PFS_NS.pd0pfs_by()[5],
    ",
  0x50400b58u64 => "
      PFS_NS.pd0pfs()[6],
      PFS_NS.pd0pfs_ha()[6],
      PFS_NS.pd0pfs_by()[6],
    ",
  0x50400b5cu64 => "
      PFS_NS.pd0pfs()[7],
      PFS_NS.pd0pfs_ha()[7],
      PFS_NS.pd0pfs_by()[7],
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
  0x50444000u64 => "
      DRW_NS.control(),
      DRW_NS.status(),
    ",
  0x50444004u64 => "
      DRW_NS.control2(),
      DRW_NS.hwrevision(),
    ",
  0x50444010u64 => "
      DRW_NS.lstart()[0],
    ",
  0x50444014u64 => "
      DRW_NS.lstart()[1],
    ",
  0x50444018u64 => "
      DRW_NS.lstart()[2],
    ",
  0x5044401cu64 => "
      DRW_NS.lstart()[3],
    ",
  0x50444020u64 => "
      DRW_NS.lstart()[4],
    ",
  0x50444024u64 => "
      DRW_NS.lstart()[5],
    ",
  0x50444028u64 => "
      DRW_NS.lxadd()[0],
    ",
  0x5044402cu64 => "
      DRW_NS.lxadd()[1],
    ",
  0x50444030u64 => "
      DRW_NS.lxadd()[2],
    ",
  0x50444034u64 => "
      DRW_NS.lxadd()[3],
    ",
  0x50444038u64 => "
      DRW_NS.lxadd()[4],
    ",
  0x5044403cu64 => "
      DRW_NS.lxadd()[5],
    ",
  0x50444040u64 => "
      DRW_NS.lyadd()[0],
    ",
  0x50444044u64 => "
      DRW_NS.lyadd()[1],
    ",
  0x50444048u64 => "
      DRW_NS.lyadd()[2],
    ",
  0x5044404cu64 => "
      DRW_NS.lyadd()[3],
    ",
  0x50444050u64 => "
      DRW_NS.lyadd()[4],
    ",
  0x50444054u64 => "
      DRW_NS.lyadd()[5],
    ",
  0x50444058u64 => "
      DRW_NS.lband()[0],
    ",
  0x5044405cu64 => "
      DRW_NS.lband()[1],
    ",
  0x50444064u64 => "
      DRW_NS.color1(),
    ",
  0x50444068u64 => "
      DRW_NS.color2(),
    ",
  0x50444074u64 => "
      DRW_NS.pattern(),
    ",
  0x50444078u64 => "
      DRW_NS.size(),
    ",
  0x5044407cu64 => "
      DRW_NS.pitch(),
    ",
  0x50444080u64 => "
      DRW_NS.origin(),
    ",
  0x50444090u64 => "
      DRW_NS.lustart(),
    ",
  0x50444094u64 => "
      DRW_NS.luxadd(),
    ",
  0x50444098u64 => "
      DRW_NS.luyadd(),
    ",
  0x5044409cu64 => "
      DRW_NS.lvstarti(),
    ",
  0x504440a0u64 => "
      DRW_NS.lvstartf(),
    ",
  0x504440a4u64 => "
      DRW_NS.lvxaddi(),
    ",
  0x504440a8u64 => "
      DRW_NS.lvyaddi(),
    ",
  0x504440acu64 => "
      DRW_NS.lvyxaddf(),
    ",
  0x504440b4u64 => "
      DRW_NS.texpitch(),
    ",
  0x504440b8u64 => "
      DRW_NS.texmask(),
    ",
  0x504440bcu64 => "
      DRW_NS.texorigin(),
    ",
  0x504440c0u64 => "
      DRW_NS.irqctl(),
    ",
  0x504440c4u64 => "
      DRW_NS.cachectl(),
    ",
  0x504440c8u64 => "
      DRW_NS.dliststart(),
    ",
  0x504440ccu64 => "
      DRW_NS.perfcount1(),
      DRW_NS.perfcount2(),
    ",
  0x504440d4u64 => "
      DRW_NS.perftrigger(),
    ",
  0x504440dcu64 => "
      DRW_NS.texcladdr(),
    ",
  0x504440e0u64 => "
      DRW_NS.texcldata(),
    ",
  0x504440e4u64 => "
      DRW_NS.texcloffset(),
    ",
  0x504440e8u64 => "
      DRW_NS.colkey(),
    ",
  0x50444100u64 => "
      DRW_NS.dbwer(),
    ",
};
