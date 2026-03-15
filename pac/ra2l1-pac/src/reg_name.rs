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
// Generated from SVD 1.50.00, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:03:37 +0000

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
      RMPU.mmpuctla(),
    ",
  0x40000102u64 => "
      RMPU.mmpupta(),
    ",
  0x40000200u64 => "
      RMPU.mmpuaca()[0],
    ",
  0x40000210u64 => "
      RMPU.mmpuaca()[1],
    ",
  0x40000220u64 => "
      RMPU.mmpuaca()[2],
    ",
  0x40000230u64 => "
      RMPU.mmpuaca()[3],
    ",
  0x40000204u64 => "
      RMPU.mmpusa()[0],
    ",
  0x40000214u64 => "
      RMPU.mmpusa()[1],
    ",
  0x40000224u64 => "
      RMPU.mmpusa()[2],
    ",
  0x40000234u64 => "
      RMPU.mmpusa()[3],
    ",
  0x40000208u64 => "
      RMPU.mmpuea()[0],
    ",
  0x40000218u64 => "
      RMPU.mmpuea()[1],
    ",
  0x40000228u64 => "
      RMPU.mmpuea()[2],
    ",
  0x40000238u64 => "
      RMPU.mmpuea()[3],
    ",
  0x40000c00u64 => "
      RMPU.smpuctl(),
    ",
  0x40000c10u64 => "
      RMPU.smpumbiu(),
    ",
  0x40000c14u64 => "
      RMPU.smpufbiu(),
    ",
  0x40000c18u64 => "
      RMPU.smpusram0(),
    ",
  0x40000c20u64 => "
      RMPU.smpup0biu(),
    ",
  0x40000c24u64 => "
      RMPU.smpup2biu(),
    ",
  0x40000c28u64 => "
      RMPU.smpup6biu(),
    ",
  0x40000d00u64 => "
      RMPU.mspmpuoad(),
    ",
  0x40000d04u64 => "
      RMPU.mspmpuctl(),
    ",
  0x40000d06u64 => "
      RMPU.mspmpupt(),
    ",
  0x40000d08u64 => "
      RMPU.mspmpusa(),
    ",
  0x40000d0cu64 => "
      RMPU.mspmpuea(),
    ",
  0x40000d10u64 => "
      RMPU.pspmpuoad(),
    ",
  0x40000d14u64 => "
      RMPU.pspmpuctl(),
    ",
  0x40000d16u64 => "
      RMPU.pspmpupt(),
    ",
  0x40000d18u64 => "
      RMPU.pspmpusa(),
    ",
  0x40000d1cu64 => "
      RMPU.pspmpuea(),
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
  0x40004008u64 => "
      BUS.busmcntsys(),
    ",
  0x4000400cu64 => "
      BUS.busmcntdma(),
    ",
  0x40004820u64 => "
      BUS.bus3erradd(),
    ",
  0x40004824u64 => "
      BUS.bus3errstat(),
    ",
  0x40004830u64 => "
      BUS.bus4erradd(),
    ",
  0x40004834u64 => "
      BUS.bus4errstat(),
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
      ICU.wupen(),
    ",
  0x400061c0u64 => "
      ICU.ielen(),
    ",
  0x40006200u64 => "
      ICU.selsr0(),
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
  0x4001b000u64 => "
      DBG.dbgstr(),
    ",
  0x4001b010u64 => "
      DBG.dbgstopcr(),
    ",
  0x4001e00cu64 => "
      SYSC.sbycr(),
    ",
  0x4001e01cu64 => "
      SYSC.mstpcra(),
    ",
  0x4001e020u64 => "
      SYSC.sckdivcr(),
    ",
  0x4001e026u64 => "
      SYSC.sckscr(),
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
  0x4001e04cu64 => "
      SYSC.lpopt(),
    ",
  0x4001e061u64 => "
      SYSC.mocoutcr(),
    ",
  0x4001e062u64 => "
      SYSC.hocoutcr(),
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
      SYSC.lvcmpcr(),
    ",
  0x4001e418u64 => "
      SYSC.lvdlvlr(),
    ",
  0x4001e41au64 => "
      SYSC.lvd1cr0(),
    ",
  0x4001e41bu64 => "
      SYSC.lvd2cr0(),
    ",
  0x4001e440u64 => "
      SYSC.dcdcctl(),
    ",
  0x4001e441u64 => "
      SYSC.vccsel(),
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
  0x40040000u64 => "
      PORT_0.pcntr1(),
      PORT_0.podr(),
    ",
  0x40040002u64 => "
      PORT_0.pdr(),
    ",
  0x40040004u64 => "
      PORT_0.pcntr2(),
    ",
  0x40040006u64 => "
      PORT_0.pidr(),
    ",
  0x40040008u64 => "
      PORT_0.pcntr3(),
      PORT_0.porr(),
    ",
  0x4004000au64 => "
      PORT_0.posr(),
    ",
  0x40040020u64 => "
      PORT_1.pcntr1(),
      PORT_1.podr(),
    ",
  0x40040022u64 => "
      PORT_1.pdr(),
    ",
  0x40040024u64 => "
      PORT_1.pcntr2(),
      PORT_1.eidr(),
    ",
  0x40040026u64 => "
      PORT_1.pidr(),
    ",
  0x40040028u64 => "
      PORT_1.pcntr3(),
      PORT_1.porr(),
    ",
  0x4004002au64 => "
      PORT_1.posr(),
    ",
  0x4004002cu64 => "
      PORT_1.pcntr4(),
      PORT_1.eorr(),
    ",
  0x4004002eu64 => "
      PORT_1.eosr(),
    ",
  0x40040800u64 => "
      PFS.p00pfs()[0],
    ",
  0x40040804u64 => "
      PFS.p00pfs()[1],
    ",
  0x40040808u64 => "
      PFS.p00pfs()[2],
    ",
  0x4004080cu64 => "
      PFS.p00pfs()[3],
    ",
  0x40040810u64 => "
      PFS.p00pfs()[4],
    ",
  0x40040814u64 => "
      PFS.p00pfs()[5],
    ",
  0x40040818u64 => "
      PFS.p00pfs()[6],
    ",
  0x4004081cu64 => "
      PFS.p00pfs()[7],
    ",
  0x40040820u64 => "
      PFS.p00pfs()[8],
    ",
  0x40040802u64 => "
      PFS.p00pfs_ha()[0],
    ",
  0x40040806u64 => "
      PFS.p00pfs_ha()[1],
    ",
  0x4004080au64 => "
      PFS.p00pfs_ha()[2],
    ",
  0x4004080eu64 => "
      PFS.p00pfs_ha()[3],
    ",
  0x40040812u64 => "
      PFS.p00pfs_ha()[4],
    ",
  0x40040816u64 => "
      PFS.p00pfs_ha()[5],
    ",
  0x4004081au64 => "
      PFS.p00pfs_ha()[6],
    ",
  0x4004081eu64 => "
      PFS.p00pfs_ha()[7],
    ",
  0x40040822u64 => "
      PFS.p00pfs_ha()[8],
    ",
  0x40040803u64 => "
      PFS.p00pfs_by()[0],
    ",
  0x40040807u64 => "
      PFS.p00pfs_by()[1],
    ",
  0x4004080bu64 => "
      PFS.p00pfs_by()[2],
    ",
  0x4004080fu64 => "
      PFS.p00pfs_by()[3],
    ",
  0x40040813u64 => "
      PFS.p00pfs_by()[4],
    ",
  0x40040817u64 => "
      PFS.p00pfs_by()[5],
    ",
  0x4004081bu64 => "
      PFS.p00pfs_by()[6],
    ",
  0x4004081fu64 => "
      PFS.p00pfs_by()[7],
    ",
  0x40040823u64 => "
      PFS.p00pfs_by()[8],
    ",
  0x40040828u64 => "
      PFS.p0pfs()[0],
    ",
  0x4004082cu64 => "
      PFS.p0pfs()[1],
    ",
  0x40040830u64 => "
      PFS.p0pfs()[2],
    ",
  0x40040834u64 => "
      PFS.p0pfs()[3],
    ",
  0x40040838u64 => "
      PFS.p0pfs()[4],
    ",
  0x4004083cu64 => "
      PFS.p0pfs()[5],
    ",
  0x4004082au64 => "
      PFS.p0pfs_ha()[0],
    ",
  0x4004082eu64 => "
      PFS.p0pfs_ha()[1],
    ",
  0x40040832u64 => "
      PFS.p0pfs_ha()[2],
    ",
  0x40040836u64 => "
      PFS.p0pfs_ha()[3],
    ",
  0x4004083au64 => "
      PFS.p0pfs_ha()[4],
    ",
  0x4004083eu64 => "
      PFS.p0pfs_ha()[5],
    ",
  0x4004082bu64 => "
      PFS.p0pfs_by()[0],
    ",
  0x4004082fu64 => "
      PFS.p0pfs_by()[1],
    ",
  0x40040833u64 => "
      PFS.p0pfs_by()[2],
    ",
  0x40040837u64 => "
      PFS.p0pfs_by()[3],
    ",
  0x4004083bu64 => "
      PFS.p0pfs_by()[4],
    ",
  0x4004083fu64 => "
      PFS.p0pfs_by()[5],
    ",
  0x40040840u64 => "
      PFS.p10pfs()[0],
    ",
  0x40040844u64 => "
      PFS.p10pfs()[1],
    ",
  0x40040848u64 => "
      PFS.p10pfs()[2],
    ",
  0x4004084cu64 => "
      PFS.p10pfs()[3],
    ",
  0x40040850u64 => "
      PFS.p10pfs()[4],
    ",
  0x40040854u64 => "
      PFS.p10pfs()[5],
    ",
  0x40040858u64 => "
      PFS.p10pfs()[6],
    ",
  0x4004085cu64 => "
      PFS.p10pfs()[7],
    ",
  0x40040842u64 => "
      PFS.p10pfs_ha()[0],
    ",
  0x40040846u64 => "
      PFS.p10pfs_ha()[1],
    ",
  0x4004084au64 => "
      PFS.p10pfs_ha()[2],
    ",
  0x4004084eu64 => "
      PFS.p10pfs_ha()[3],
    ",
  0x40040852u64 => "
      PFS.p10pfs_ha()[4],
    ",
  0x40040856u64 => "
      PFS.p10pfs_ha()[5],
    ",
  0x4004085au64 => "
      PFS.p10pfs_ha()[6],
    ",
  0x4004085eu64 => "
      PFS.p10pfs_ha()[7],
    ",
  0x40040843u64 => "
      PFS.p10pfs_by()[0],
    ",
  0x40040847u64 => "
      PFS.p10pfs_by()[1],
    ",
  0x4004084bu64 => "
      PFS.p10pfs_by()[2],
    ",
  0x4004084fu64 => "
      PFS.p10pfs_by()[3],
    ",
  0x40040853u64 => "
      PFS.p10pfs_by()[4],
    ",
  0x40040857u64 => "
      PFS.p10pfs_by()[5],
    ",
  0x4004085bu64 => "
      PFS.p10pfs_by()[6],
    ",
  0x4004085fu64 => "
      PFS.p10pfs_by()[7],
    ",
  0x40040860u64 => "
      PFS.p108pfs(),
    ",
  0x40040862u64 => "
      PFS.p108pfs_ha(),
    ",
  0x40040863u64 => "
      PFS.p108pfs_by(),
    ",
  0x40040864u64 => "
      PFS.p109pfs(),
    ",
  0x40040866u64 => "
      PFS.p109pfs_ha(),
    ",
  0x40040867u64 => "
      PFS.p109pfs_by(),
    ",
  0x40040868u64 => "
      PFS.p1pfs()[0],
    ",
  0x4004086cu64 => "
      PFS.p1pfs()[1],
    ",
  0x40040870u64 => "
      PFS.p1pfs()[2],
    ",
  0x40040874u64 => "
      PFS.p1pfs()[3],
    ",
  0x40040878u64 => "
      PFS.p1pfs()[4],
    ",
  0x4004087cu64 => "
      PFS.p1pfs()[5],
    ",
  0x4004086au64 => "
      PFS.p1pfs_ha()[0],
    ",
  0x4004086eu64 => "
      PFS.p1pfs_ha()[1],
    ",
  0x40040872u64 => "
      PFS.p1pfs_ha()[2],
    ",
  0x40040876u64 => "
      PFS.p1pfs_ha()[3],
    ",
  0x4004087au64 => "
      PFS.p1pfs_ha()[4],
    ",
  0x4004087eu64 => "
      PFS.p1pfs_ha()[5],
    ",
  0x4004086bu64 => "
      PFS.p1pfs_by()[0],
    ",
  0x4004086fu64 => "
      PFS.p1pfs_by()[1],
    ",
  0x40040873u64 => "
      PFS.p1pfs_by()[2],
    ",
  0x40040877u64 => "
      PFS.p1pfs_by()[3],
    ",
  0x4004087bu64 => "
      PFS.p1pfs_by()[4],
    ",
  0x4004087fu64 => "
      PFS.p1pfs_by()[5],
    ",
  0x40040880u64 => "
      PFS.p200pfs(),
    ",
  0x40040882u64 => "
      PFS.p200pfs_ha(),
    ",
  0x40040883u64 => "
      PFS.p200pfs_by(),
    ",
  0x40040884u64 => "
      PFS.p201pfs(),
    ",
  0x40040886u64 => "
      PFS.p201pfs_ha(),
    ",
  0x40040887u64 => "
      PFS.p201pfs_by(),
    ",
  0x40040888u64 => "
      PFS.p20pfs()[0],
    ",
  0x4004088cu64 => "
      PFS.p20pfs()[1],
    ",
  0x40040890u64 => "
      PFS.p20pfs()[2],
    ",
  0x40040894u64 => "
      PFS.p20pfs()[3],
    ",
  0x40040898u64 => "
      PFS.p20pfs()[4],
    ",
  0x4004089cu64 => "
      PFS.p20pfs()[5],
    ",
  0x400408a0u64 => "
      PFS.p20pfs()[6],
    ",
  0x4004088au64 => "
      PFS.p20pfs_ha()[0],
    ",
  0x4004088eu64 => "
      PFS.p20pfs_ha()[1],
    ",
  0x40040892u64 => "
      PFS.p20pfs_ha()[2],
    ",
  0x40040896u64 => "
      PFS.p20pfs_ha()[3],
    ",
  0x4004089au64 => "
      PFS.p20pfs_ha()[4],
    ",
  0x4004089eu64 => "
      PFS.p20pfs_ha()[5],
    ",
  0x400408a2u64 => "
      PFS.p20pfs_ha()[6],
    ",
  0x4004088bu64 => "
      PFS.p20pfs_by()[0],
    ",
  0x4004088fu64 => "
      PFS.p20pfs_by()[1],
    ",
  0x40040893u64 => "
      PFS.p20pfs_by()[2],
    ",
  0x40040897u64 => "
      PFS.p20pfs_by()[3],
    ",
  0x4004089bu64 => "
      PFS.p20pfs_by()[4],
    ",
  0x4004089fu64 => "
      PFS.p20pfs_by()[5],
    ",
  0x400408a3u64 => "
      PFS.p20pfs_by()[6],
    ",
  0x400408b0u64 => "
      PFS.p2pfs()[0],
    ",
  0x400408b4u64 => "
      PFS.p2pfs()[1],
    ",
  0x400408b8u64 => "
      PFS.p2pfs()[2],
    ",
  0x400408bcu64 => "
      PFS.p2pfs()[3],
    ",
  0x400408b2u64 => "
      PFS.p2pfs_ha()[0],
    ",
  0x400408b6u64 => "
      PFS.p2pfs_ha()[1],
    ",
  0x400408bau64 => "
      PFS.p2pfs_ha()[2],
    ",
  0x400408beu64 => "
      PFS.p2pfs_ha()[3],
    ",
  0x400408b3u64 => "
      PFS.p2pfs_by()[0],
    ",
  0x400408b7u64 => "
      PFS.p2pfs_by()[1],
    ",
  0x400408bbu64 => "
      PFS.p2pfs_by()[2],
    ",
  0x400408bfu64 => "
      PFS.p2pfs_by()[3],
    ",
  0x400408c0u64 => "
      PFS.p300pfs(),
    ",
  0x400408c2u64 => "
      PFS.p300pfs_ha(),
    ",
  0x400408c3u64 => "
      PFS.p300pfs_by(),
    ",
  0x400408c4u64 => "
      PFS.p30pfs()[0],
    ",
  0x400408c8u64 => "
      PFS.p30pfs()[1],
    ",
  0x400408ccu64 => "
      PFS.p30pfs()[2],
    ",
  0x400408d0u64 => "
      PFS.p30pfs()[3],
    ",
  0x400408d4u64 => "
      PFS.p30pfs()[4],
    ",
  0x400408d8u64 => "
      PFS.p30pfs()[5],
    ",
  0x400408dcu64 => "
      PFS.p30pfs()[6],
    ",
  0x400408c6u64 => "
      PFS.p30pfs_ha()[0],
    ",
  0x400408cau64 => "
      PFS.p30pfs_ha()[1],
    ",
  0x400408ceu64 => "
      PFS.p30pfs_ha()[2],
    ",
  0x400408d2u64 => "
      PFS.p30pfs_ha()[3],
    ",
  0x400408d6u64 => "
      PFS.p30pfs_ha()[4],
    ",
  0x400408dau64 => "
      PFS.p30pfs_ha()[5],
    ",
  0x400408deu64 => "
      PFS.p30pfs_ha()[6],
    ",
  0x400408c7u64 => "
      PFS.p30pfs_by()[0],
    ",
  0x400408cbu64 => "
      PFS.p30pfs_by()[1],
    ",
  0x400408cfu64 => "
      PFS.p30pfs_by()[2],
    ",
  0x400408d3u64 => "
      PFS.p30pfs_by()[3],
    ",
  0x400408d7u64 => "
      PFS.p30pfs_by()[4],
    ",
  0x400408dbu64 => "
      PFS.p30pfs_by()[5],
    ",
  0x400408dfu64 => "
      PFS.p30pfs_by()[6],
    ",
  0x40040900u64 => "
      PFS.p40pfs()[0],
    ",
  0x40040904u64 => "
      PFS.p40pfs()[1],
    ",
  0x40040908u64 => "
      PFS.p40pfs()[2],
    ",
  0x4004090cu64 => "
      PFS.p40pfs()[3],
    ",
  0x40040910u64 => "
      PFS.p40pfs()[4],
    ",
  0x40040914u64 => "
      PFS.p40pfs()[5],
    ",
  0x40040918u64 => "
      PFS.p40pfs()[6],
    ",
  0x4004091cu64 => "
      PFS.p40pfs()[7],
    ",
  0x40040920u64 => "
      PFS.p40pfs()[8],
    ",
  0x40040924u64 => "
      PFS.p40pfs()[9],
    ",
  0x40040902u64 => "
      PFS.p40pfs_ha()[0],
    ",
  0x40040906u64 => "
      PFS.p40pfs_ha()[1],
    ",
  0x4004090au64 => "
      PFS.p40pfs_ha()[2],
    ",
  0x4004090eu64 => "
      PFS.p40pfs_ha()[3],
    ",
  0x40040912u64 => "
      PFS.p40pfs_ha()[4],
    ",
  0x40040916u64 => "
      PFS.p40pfs_ha()[5],
    ",
  0x4004091au64 => "
      PFS.p40pfs_ha()[6],
    ",
  0x4004091eu64 => "
      PFS.p40pfs_ha()[7],
    ",
  0x40040922u64 => "
      PFS.p40pfs_ha()[8],
    ",
  0x40040926u64 => "
      PFS.p40pfs_ha()[9],
    ",
  0x40040903u64 => "
      PFS.p40pfs_by()[0],
    ",
  0x40040907u64 => "
      PFS.p40pfs_by()[1],
    ",
  0x4004090bu64 => "
      PFS.p40pfs_by()[2],
    ",
  0x4004090fu64 => "
      PFS.p40pfs_by()[3],
    ",
  0x40040913u64 => "
      PFS.p40pfs_by()[4],
    ",
  0x40040917u64 => "
      PFS.p40pfs_by()[5],
    ",
  0x4004091bu64 => "
      PFS.p40pfs_by()[6],
    ",
  0x4004091fu64 => "
      PFS.p40pfs_by()[7],
    ",
  0x40040923u64 => "
      PFS.p40pfs_by()[8],
    ",
  0x40040927u64 => "
      PFS.p40pfs_by()[9],
    ",
  0x40040928u64 => "
      PFS.p4pfs()[0],
    ",
  0x4004092cu64 => "
      PFS.p4pfs()[1],
    ",
  0x40040930u64 => "
      PFS.p4pfs()[2],
    ",
  0x40040934u64 => "
      PFS.p4pfs()[3],
    ",
  0x40040938u64 => "
      PFS.p4pfs()[4],
    ",
  0x4004093cu64 => "
      PFS.p4pfs()[5],
    ",
  0x4004092au64 => "
      PFS.p4pfs_ha()[0],
    ",
  0x4004092eu64 => "
      PFS.p4pfs_ha()[1],
    ",
  0x40040932u64 => "
      PFS.p4pfs_ha()[2],
    ",
  0x40040936u64 => "
      PFS.p4pfs_ha()[3],
    ",
  0x4004093au64 => "
      PFS.p4pfs_ha()[4],
    ",
  0x4004093eu64 => "
      PFS.p4pfs_ha()[5],
    ",
  0x4004092bu64 => "
      PFS.p4pfs_by()[0],
    ",
  0x4004092fu64 => "
      PFS.p4pfs_by()[1],
    ",
  0x40040933u64 => "
      PFS.p4pfs_by()[2],
    ",
  0x40040937u64 => "
      PFS.p4pfs_by()[3],
    ",
  0x4004093bu64 => "
      PFS.p4pfs_by()[4],
    ",
  0x4004093fu64 => "
      PFS.p4pfs_by()[5],
    ",
  0x40040940u64 => "
      PFS.p50pfs()[0],
    ",
  0x40040944u64 => "
      PFS.p50pfs()[1],
    ",
  0x40040948u64 => "
      PFS.p50pfs()[2],
    ",
  0x4004094cu64 => "
      PFS.p50pfs()[3],
    ",
  0x40040950u64 => "
      PFS.p50pfs()[4],
    ",
  0x40040954u64 => "
      PFS.p50pfs()[5],
    ",
  0x40040942u64 => "
      PFS.p50pfs_ha()[0],
    ",
  0x40040946u64 => "
      PFS.p50pfs_ha()[1],
    ",
  0x4004094au64 => "
      PFS.p50pfs_ha()[2],
    ",
  0x4004094eu64 => "
      PFS.p50pfs_ha()[3],
    ",
  0x40040952u64 => "
      PFS.p50pfs_ha()[4],
    ",
  0x40040956u64 => "
      PFS.p50pfs_ha()[5],
    ",
  0x40040943u64 => "
      PFS.p50pfs_by()[0],
    ",
  0x40040947u64 => "
      PFS.p50pfs_by()[1],
    ",
  0x4004094bu64 => "
      PFS.p50pfs_by()[2],
    ",
  0x4004094fu64 => "
      PFS.p50pfs_by()[3],
    ",
  0x40040953u64 => "
      PFS.p50pfs_by()[4],
    ",
  0x40040957u64 => "
      PFS.p50pfs_by()[5],
    ",
  0x40040988u64 => "
      PFS.p60pfs()[2],
    ",
  0x4004098cu64 => "
      PFS.p60pfs()[3],
    ",
  0x4004098au64 => "
      PFS.p60pfs_ha()[2],
    ",
  0x4004098eu64 => "
      PFS.p60pfs_ha()[3],
    ",
  0x4004098bu64 => "
      PFS.p60pfs_by()[2],
    ",
  0x4004098fu64 => "
      PFS.p60pfs_by()[3],
    ",
  0x400409a0u64 => "
      PFS.p60pfs()[0],
    ",
  0x400409a4u64 => "
      PFS.p60pfs()[1],
    ",
  0x400409a2u64 => "
      PFS.p60pfs_ha()[0],
    ",
  0x400409a6u64 => "
      PFS.p60pfs_ha()[1],
    ",
  0x400409a3u64 => "
      PFS.p60pfs_by()[0],
    ",
  0x400409a7u64 => "
      PFS.p60pfs_by()[1],
    ",
  0x400409a8u64 => "
      PFS.p610pfs(),
    ",
  0x400409aau64 => "
      PFS.p610pfs_ha(),
    ",
  0x400409abu64 => "
      PFS.p610pfs_by(),
    ",
  0x400409e0u64 => "
      PFS.p708pfs(),
    ",
  0x400409e2u64 => "
      PFS.p708pfs_ha(),
    ",
  0x400409e3u64 => "
      PFS.p708pfs_by(),
    ",
  0x400409f8u64 => "
      PFS.p714pfs(),
    ",
  0x400409fau64 => "
      PFS.p714pfs_ha(),
    ",
  0x400409fbu64 => "
      PFS.p714pfs_by(),
    ",
  0x40040a20u64 => "
      PFS.p80pfs()[0],
    ",
  0x40040a24u64 => "
      PFS.p80pfs()[1],
    ",
  0x40040a22u64 => "
      PFS.p80pfs_ha()[0],
    ",
  0x40040a26u64 => "
      PFS.p80pfs_ha()[1],
    ",
  0x40040a23u64 => "
      PFS.p80pfs_by()[0],
    ",
  0x40040a27u64 => "
      PFS.p80pfs_by()[1],
    ",
  0x40040d03u64 => "
      PFS.pwpr(),
    ",
  0x40040d0fu64 => "
      PFS.prwcntr(),
    ",
  0x40041000u64 => "
      ELC.elcr(),
    ",
  0x40041002u64 => "
      ELC.elsegr()[0],
    ",
  0x40041004u64 => "
      ELC.elsegr()[1],
    ",
  0x40041018u64 => "
      ELC.elsr()[2],
    ",
  0x4004101cu64 => "
      ELC.elsr()[3],
    ",
  0x40041040u64 => "
      ELC.elsr12(),
    ",
  0x40041048u64 => "
      ELC.elsr()[0],
    ",
  0x4004104cu64 => "
      ELC.elsr()[1],
    ",
  0x40041058u64 => "
      ELC.elsr18(),
    ",
  0x40042000u64 => "
      POEG.poegga(),
    ",
  0x40042100u64 => "
      POEG.poeggb(),
    ",
  0x40044000u64 => "
      RTC.r64cnt(),
    ",
  0x40044002u64 => "
      RTC.bcnt()[0],
      RTC.rseccnt(),
    ",
  0x40044004u64 => "
      RTC.bcnt()[1],
      RTC.rmincnt(),
    ",
  0x40044006u64 => "
      RTC.bcnt()[2],
      RTC.rhrcnt(),
    ",
  0x40044008u64 => "
      RTC.bcnt()[3],
      RTC.rwkcnt(),
    ",
  0x4004400au64 => "
      RTC.rdaycnt(),
    ",
  0x4004400cu64 => "
      RTC.rmoncnt(),
    ",
  0x4004400eu64 => "
      RTC.ryrcnt(),
    ",
  0x40044010u64 => "
      RTC.bcntar()[0],
      RTC.rsecar(),
    ",
  0x40044012u64 => "
      RTC.bcntar()[1],
      RTC.rminar(),
    ",
  0x40044014u64 => "
      RTC.bcntar()[2],
      RTC.rhrar(),
    ",
  0x40044016u64 => "
      RTC.bcntar()[3],
      RTC.rwkar(),
    ",
  0x40044018u64 => "
      RTC.bcntaer()[0],
      RTC.rdayar(),
    ",
  0x4004401au64 => "
      RTC.bcntaer()[1],
      RTC.rmonar(),
    ",
  0x4004401cu64 => "
      RTC.bcnt2aer(),
      RTC.ryrar(),
    ",
  0x4004401eu64 => "
      RTC.bcnt3aer(),
      RTC.ryraren(),
    ",
  0x40044022u64 => "
      RTC.rcr1(),
    ",
  0x40044024u64 => "
      RTC.rcr2(),
      RTC.rcr2_bcnt(),
    ",
  0x40044028u64 => "
      RTC.rcr4(),
    ",
  0x4004402au64 => "
      RTC.rfrh(),
    ",
  0x4004402cu64 => "
      RTC.rfrl(),
    ",
  0x4004402eu64 => "
      RTC.radj(),
    ",
  0x40044200u64 => "
      WDT.wdtrr(),
    ",
  0x40044202u64 => "
      WDT.wdtcr(),
    ",
  0x40044204u64 => "
      WDT.wdtsr(),
    ",
  0x40044206u64 => "
      WDT.wdtrcr(),
    ",
  0x40044208u64 => "
      WDT.wdtcstpr(),
    ",
  0x40044400u64 => "
      IWDT.iwdtrr(),
    ",
  0x40044404u64 => "
      IWDT.iwdtsr(),
    ",
  0x40044600u64 => "
      CAC.cacr0(),
    ",
  0x40044601u64 => "
      CAC.cacr1(),
    ",
  0x40044602u64 => "
      CAC.cacr2(),
    ",
  0x40044603u64 => "
      CAC.caicr(),
    ",
  0x40044604u64 => "
      CAC.castr(),
    ",
  0x40044606u64 => "
      CAC.caulvr(),
    ",
  0x40044608u64 => "
      CAC.callvr(),
    ",
  0x4004460au64 => "
      CAC.cacntbr(),
    ",
  0x40047000u64 => "
      MSTP.mstpcrb(),
    ",
  0x40047004u64 => "
      MSTP.mstpcrc(),
    ",
  0x40047008u64 => "
      MSTP.mstpcrd(),
    ",
  0x4004700cu64 => "
      MSTP.lsmrwdis(),
    ",
  0x40050200u64 => "
      CAN_0.mb_id()[0],
    ",
  0x40050210u64 => "
      CAN_0.mb_id()[1],
    ",
  0x40050220u64 => "
      CAN_0.mb_id()[2],
    ",
  0x40050230u64 => "
      CAN_0.mb_id()[3],
    ",
  0x40050240u64 => "
      CAN_0.mb_id()[4],
    ",
  0x40050250u64 => "
      CAN_0.mb_id()[5],
    ",
  0x40050260u64 => "
      CAN_0.mb_id()[6],
    ",
  0x40050270u64 => "
      CAN_0.mb_id()[7],
    ",
  0x40050280u64 => "
      CAN_0.mb_id()[8],
    ",
  0x40050290u64 => "
      CAN_0.mb_id()[9],
    ",
  0x400502a0u64 => "
      CAN_0.mb_id()[10],
    ",
  0x400502b0u64 => "
      CAN_0.mb_id()[11],
    ",
  0x400502c0u64 => "
      CAN_0.mb_id()[12],
    ",
  0x400502d0u64 => "
      CAN_0.mb_id()[13],
    ",
  0x400502e0u64 => "
      CAN_0.mb_id()[14],
    ",
  0x400502f0u64 => "
      CAN_0.mb_id()[15],
    ",
  0x40050300u64 => "
      CAN_0.mb_id()[16],
    ",
  0x40050310u64 => "
      CAN_0.mb_id()[17],
    ",
  0x40050320u64 => "
      CAN_0.mb_id()[18],
    ",
  0x40050330u64 => "
      CAN_0.mb_id()[19],
    ",
  0x40050340u64 => "
      CAN_0.mb_id()[20],
    ",
  0x40050350u64 => "
      CAN_0.mb_id()[21],
    ",
  0x40050360u64 => "
      CAN_0.mb_id()[22],
    ",
  0x40050370u64 => "
      CAN_0.mb_id()[23],
    ",
  0x40050380u64 => "
      CAN_0.mb_id()[24],
    ",
  0x40050390u64 => "
      CAN_0.mb_id()[25],
    ",
  0x400503a0u64 => "
      CAN_0.mb_id()[26],
    ",
  0x400503b0u64 => "
      CAN_0.mb_id()[27],
    ",
  0x400503c0u64 => "
      CAN_0.mb_id()[28],
    ",
  0x400503d0u64 => "
      CAN_0.mb_id()[29],
    ",
  0x400503e0u64 => "
      CAN_0.mb_id()[30],
    ",
  0x400503f0u64 => "
      CAN_0.mb_id()[31],
    ",
  0x40050204u64 => "
      CAN_0.mb_dl()[0],
    ",
  0x40050214u64 => "
      CAN_0.mb_dl()[1],
    ",
  0x40050224u64 => "
      CAN_0.mb_dl()[2],
    ",
  0x40050234u64 => "
      CAN_0.mb_dl()[3],
    ",
  0x40050244u64 => "
      CAN_0.mb_dl()[4],
    ",
  0x40050254u64 => "
      CAN_0.mb_dl()[5],
    ",
  0x40050264u64 => "
      CAN_0.mb_dl()[6],
    ",
  0x40050274u64 => "
      CAN_0.mb_dl()[7],
    ",
  0x40050284u64 => "
      CAN_0.mb_dl()[8],
    ",
  0x40050294u64 => "
      CAN_0.mb_dl()[9],
    ",
  0x400502a4u64 => "
      CAN_0.mb_dl()[10],
    ",
  0x400502b4u64 => "
      CAN_0.mb_dl()[11],
    ",
  0x400502c4u64 => "
      CAN_0.mb_dl()[12],
    ",
  0x400502d4u64 => "
      CAN_0.mb_dl()[13],
    ",
  0x400502e4u64 => "
      CAN_0.mb_dl()[14],
    ",
  0x400502f4u64 => "
      CAN_0.mb_dl()[15],
    ",
  0x40050304u64 => "
      CAN_0.mb_dl()[16],
    ",
  0x40050314u64 => "
      CAN_0.mb_dl()[17],
    ",
  0x40050324u64 => "
      CAN_0.mb_dl()[18],
    ",
  0x40050334u64 => "
      CAN_0.mb_dl()[19],
    ",
  0x40050344u64 => "
      CAN_0.mb_dl()[20],
    ",
  0x40050354u64 => "
      CAN_0.mb_dl()[21],
    ",
  0x40050364u64 => "
      CAN_0.mb_dl()[22],
    ",
  0x40050374u64 => "
      CAN_0.mb_dl()[23],
    ",
  0x40050384u64 => "
      CAN_0.mb_dl()[24],
    ",
  0x40050394u64 => "
      CAN_0.mb_dl()[25],
    ",
  0x400503a4u64 => "
      CAN_0.mb_dl()[26],
    ",
  0x400503b4u64 => "
      CAN_0.mb_dl()[27],
    ",
  0x400503c4u64 => "
      CAN_0.mb_dl()[28],
    ",
  0x400503d4u64 => "
      CAN_0.mb_dl()[29],
    ",
  0x400503e4u64 => "
      CAN_0.mb_dl()[30],
    ",
  0x400503f4u64 => "
      CAN_0.mb_dl()[31],
    ",
  0x40050206u64 => "
      CAN_0.mb_d0()[0],
    ",
  0x40050216u64 => "
      CAN_0.mb_d0()[1],
    ",
  0x40050226u64 => "
      CAN_0.mb_d0()[2],
    ",
  0x40050236u64 => "
      CAN_0.mb_d0()[3],
    ",
  0x40050246u64 => "
      CAN_0.mb_d0()[4],
    ",
  0x40050256u64 => "
      CAN_0.mb_d0()[5],
    ",
  0x40050266u64 => "
      CAN_0.mb_d0()[6],
    ",
  0x40050276u64 => "
      CAN_0.mb_d0()[7],
    ",
  0x40050286u64 => "
      CAN_0.mb_d0()[8],
    ",
  0x40050296u64 => "
      CAN_0.mb_d0()[9],
    ",
  0x400502a6u64 => "
      CAN_0.mb_d0()[10],
    ",
  0x400502b6u64 => "
      CAN_0.mb_d0()[11],
    ",
  0x400502c6u64 => "
      CAN_0.mb_d0()[12],
    ",
  0x400502d6u64 => "
      CAN_0.mb_d0()[13],
    ",
  0x400502e6u64 => "
      CAN_0.mb_d0()[14],
    ",
  0x400502f6u64 => "
      CAN_0.mb_d0()[15],
    ",
  0x40050306u64 => "
      CAN_0.mb_d0()[16],
    ",
  0x40050316u64 => "
      CAN_0.mb_d0()[17],
    ",
  0x40050326u64 => "
      CAN_0.mb_d0()[18],
    ",
  0x40050336u64 => "
      CAN_0.mb_d0()[19],
    ",
  0x40050346u64 => "
      CAN_0.mb_d0()[20],
    ",
  0x40050356u64 => "
      CAN_0.mb_d0()[21],
    ",
  0x40050366u64 => "
      CAN_0.mb_d0()[22],
    ",
  0x40050376u64 => "
      CAN_0.mb_d0()[23],
    ",
  0x40050386u64 => "
      CAN_0.mb_d0()[24],
    ",
  0x40050396u64 => "
      CAN_0.mb_d0()[25],
    ",
  0x400503a6u64 => "
      CAN_0.mb_d0()[26],
    ",
  0x400503b6u64 => "
      CAN_0.mb_d0()[27],
    ",
  0x400503c6u64 => "
      CAN_0.mb_d0()[28],
    ",
  0x400503d6u64 => "
      CAN_0.mb_d0()[29],
    ",
  0x400503e6u64 => "
      CAN_0.mb_d0()[30],
    ",
  0x400503f6u64 => "
      CAN_0.mb_d0()[31],
    ",
  0x40050207u64 => "
      CAN_0.mb_d1()[0],
    ",
  0x40050217u64 => "
      CAN_0.mb_d1()[1],
    ",
  0x40050227u64 => "
      CAN_0.mb_d1()[2],
    ",
  0x40050237u64 => "
      CAN_0.mb_d1()[3],
    ",
  0x40050247u64 => "
      CAN_0.mb_d1()[4],
    ",
  0x40050257u64 => "
      CAN_0.mb_d1()[5],
    ",
  0x40050267u64 => "
      CAN_0.mb_d1()[6],
    ",
  0x40050277u64 => "
      CAN_0.mb_d1()[7],
    ",
  0x40050287u64 => "
      CAN_0.mb_d1()[8],
    ",
  0x40050297u64 => "
      CAN_0.mb_d1()[9],
    ",
  0x400502a7u64 => "
      CAN_0.mb_d1()[10],
    ",
  0x400502b7u64 => "
      CAN_0.mb_d1()[11],
    ",
  0x400502c7u64 => "
      CAN_0.mb_d1()[12],
    ",
  0x400502d7u64 => "
      CAN_0.mb_d1()[13],
    ",
  0x400502e7u64 => "
      CAN_0.mb_d1()[14],
    ",
  0x400502f7u64 => "
      CAN_0.mb_d1()[15],
    ",
  0x40050307u64 => "
      CAN_0.mb_d1()[16],
    ",
  0x40050317u64 => "
      CAN_0.mb_d1()[17],
    ",
  0x40050327u64 => "
      CAN_0.mb_d1()[18],
    ",
  0x40050337u64 => "
      CAN_0.mb_d1()[19],
    ",
  0x40050347u64 => "
      CAN_0.mb_d1()[20],
    ",
  0x40050357u64 => "
      CAN_0.mb_d1()[21],
    ",
  0x40050367u64 => "
      CAN_0.mb_d1()[22],
    ",
  0x40050377u64 => "
      CAN_0.mb_d1()[23],
    ",
  0x40050387u64 => "
      CAN_0.mb_d1()[24],
    ",
  0x40050397u64 => "
      CAN_0.mb_d1()[25],
    ",
  0x400503a7u64 => "
      CAN_0.mb_d1()[26],
    ",
  0x400503b7u64 => "
      CAN_0.mb_d1()[27],
    ",
  0x400503c7u64 => "
      CAN_0.mb_d1()[28],
    ",
  0x400503d7u64 => "
      CAN_0.mb_d1()[29],
    ",
  0x400503e7u64 => "
      CAN_0.mb_d1()[30],
    ",
  0x400503f7u64 => "
      CAN_0.mb_d1()[31],
    ",
  0x40050208u64 => "
      CAN_0.mb_d2()[0],
    ",
  0x40050218u64 => "
      CAN_0.mb_d2()[1],
    ",
  0x40050228u64 => "
      CAN_0.mb_d2()[2],
    ",
  0x40050238u64 => "
      CAN_0.mb_d2()[3],
    ",
  0x40050248u64 => "
      CAN_0.mb_d2()[4],
    ",
  0x40050258u64 => "
      CAN_0.mb_d2()[5],
    ",
  0x40050268u64 => "
      CAN_0.mb_d2()[6],
    ",
  0x40050278u64 => "
      CAN_0.mb_d2()[7],
    ",
  0x40050288u64 => "
      CAN_0.mb_d2()[8],
    ",
  0x40050298u64 => "
      CAN_0.mb_d2()[9],
    ",
  0x400502a8u64 => "
      CAN_0.mb_d2()[10],
    ",
  0x400502b8u64 => "
      CAN_0.mb_d2()[11],
    ",
  0x400502c8u64 => "
      CAN_0.mb_d2()[12],
    ",
  0x400502d8u64 => "
      CAN_0.mb_d2()[13],
    ",
  0x400502e8u64 => "
      CAN_0.mb_d2()[14],
    ",
  0x400502f8u64 => "
      CAN_0.mb_d2()[15],
    ",
  0x40050308u64 => "
      CAN_0.mb_d2()[16],
    ",
  0x40050318u64 => "
      CAN_0.mb_d2()[17],
    ",
  0x40050328u64 => "
      CAN_0.mb_d2()[18],
    ",
  0x40050338u64 => "
      CAN_0.mb_d2()[19],
    ",
  0x40050348u64 => "
      CAN_0.mb_d2()[20],
    ",
  0x40050358u64 => "
      CAN_0.mb_d2()[21],
    ",
  0x40050368u64 => "
      CAN_0.mb_d2()[22],
    ",
  0x40050378u64 => "
      CAN_0.mb_d2()[23],
    ",
  0x40050388u64 => "
      CAN_0.mb_d2()[24],
    ",
  0x40050398u64 => "
      CAN_0.mb_d2()[25],
    ",
  0x400503a8u64 => "
      CAN_0.mb_d2()[26],
    ",
  0x400503b8u64 => "
      CAN_0.mb_d2()[27],
    ",
  0x400503c8u64 => "
      CAN_0.mb_d2()[28],
    ",
  0x400503d8u64 => "
      CAN_0.mb_d2()[29],
    ",
  0x400503e8u64 => "
      CAN_0.mb_d2()[30],
    ",
  0x400503f8u64 => "
      CAN_0.mb_d2()[31],
    ",
  0x40050209u64 => "
      CAN_0.mb_d3()[0],
    ",
  0x40050219u64 => "
      CAN_0.mb_d3()[1],
    ",
  0x40050229u64 => "
      CAN_0.mb_d3()[2],
    ",
  0x40050239u64 => "
      CAN_0.mb_d3()[3],
    ",
  0x40050249u64 => "
      CAN_0.mb_d3()[4],
    ",
  0x40050259u64 => "
      CAN_0.mb_d3()[5],
    ",
  0x40050269u64 => "
      CAN_0.mb_d3()[6],
    ",
  0x40050279u64 => "
      CAN_0.mb_d3()[7],
    ",
  0x40050289u64 => "
      CAN_0.mb_d3()[8],
    ",
  0x40050299u64 => "
      CAN_0.mb_d3()[9],
    ",
  0x400502a9u64 => "
      CAN_0.mb_d3()[10],
    ",
  0x400502b9u64 => "
      CAN_0.mb_d3()[11],
    ",
  0x400502c9u64 => "
      CAN_0.mb_d3()[12],
    ",
  0x400502d9u64 => "
      CAN_0.mb_d3()[13],
    ",
  0x400502e9u64 => "
      CAN_0.mb_d3()[14],
    ",
  0x400502f9u64 => "
      CAN_0.mb_d3()[15],
    ",
  0x40050309u64 => "
      CAN_0.mb_d3()[16],
    ",
  0x40050319u64 => "
      CAN_0.mb_d3()[17],
    ",
  0x40050329u64 => "
      CAN_0.mb_d3()[18],
    ",
  0x40050339u64 => "
      CAN_0.mb_d3()[19],
    ",
  0x40050349u64 => "
      CAN_0.mb_d3()[20],
    ",
  0x40050359u64 => "
      CAN_0.mb_d3()[21],
    ",
  0x40050369u64 => "
      CAN_0.mb_d3()[22],
    ",
  0x40050379u64 => "
      CAN_0.mb_d3()[23],
    ",
  0x40050389u64 => "
      CAN_0.mb_d3()[24],
    ",
  0x40050399u64 => "
      CAN_0.mb_d3()[25],
    ",
  0x400503a9u64 => "
      CAN_0.mb_d3()[26],
    ",
  0x400503b9u64 => "
      CAN_0.mb_d3()[27],
    ",
  0x400503c9u64 => "
      CAN_0.mb_d3()[28],
    ",
  0x400503d9u64 => "
      CAN_0.mb_d3()[29],
    ",
  0x400503e9u64 => "
      CAN_0.mb_d3()[30],
    ",
  0x400503f9u64 => "
      CAN_0.mb_d3()[31],
    ",
  0x4005020au64 => "
      CAN_0.mb_d4()[0],
    ",
  0x4005021au64 => "
      CAN_0.mb_d4()[1],
    ",
  0x4005022au64 => "
      CAN_0.mb_d4()[2],
    ",
  0x4005023au64 => "
      CAN_0.mb_d4()[3],
    ",
  0x4005024au64 => "
      CAN_0.mb_d4()[4],
    ",
  0x4005025au64 => "
      CAN_0.mb_d4()[5],
    ",
  0x4005026au64 => "
      CAN_0.mb_d4()[6],
    ",
  0x4005027au64 => "
      CAN_0.mb_d4()[7],
    ",
  0x4005028au64 => "
      CAN_0.mb_d4()[8],
    ",
  0x4005029au64 => "
      CAN_0.mb_d4()[9],
    ",
  0x400502aau64 => "
      CAN_0.mb_d4()[10],
    ",
  0x400502bau64 => "
      CAN_0.mb_d4()[11],
    ",
  0x400502cau64 => "
      CAN_0.mb_d4()[12],
    ",
  0x400502dau64 => "
      CAN_0.mb_d4()[13],
    ",
  0x400502eau64 => "
      CAN_0.mb_d4()[14],
    ",
  0x400502fau64 => "
      CAN_0.mb_d4()[15],
    ",
  0x4005030au64 => "
      CAN_0.mb_d4()[16],
    ",
  0x4005031au64 => "
      CAN_0.mb_d4()[17],
    ",
  0x4005032au64 => "
      CAN_0.mb_d4()[18],
    ",
  0x4005033au64 => "
      CAN_0.mb_d4()[19],
    ",
  0x4005034au64 => "
      CAN_0.mb_d4()[20],
    ",
  0x4005035au64 => "
      CAN_0.mb_d4()[21],
    ",
  0x4005036au64 => "
      CAN_0.mb_d4()[22],
    ",
  0x4005037au64 => "
      CAN_0.mb_d4()[23],
    ",
  0x4005038au64 => "
      CAN_0.mb_d4()[24],
    ",
  0x4005039au64 => "
      CAN_0.mb_d4()[25],
    ",
  0x400503aau64 => "
      CAN_0.mb_d4()[26],
    ",
  0x400503bau64 => "
      CAN_0.mb_d4()[27],
    ",
  0x400503cau64 => "
      CAN_0.mb_d4()[28],
    ",
  0x400503dau64 => "
      CAN_0.mb_d4()[29],
    ",
  0x400503eau64 => "
      CAN_0.mb_d4()[30],
    ",
  0x400503fau64 => "
      CAN_0.mb_d4()[31],
    ",
  0x4005020bu64 => "
      CAN_0.mb_d5()[0],
    ",
  0x4005021bu64 => "
      CAN_0.mb_d5()[1],
    ",
  0x4005022bu64 => "
      CAN_0.mb_d5()[2],
    ",
  0x4005023bu64 => "
      CAN_0.mb_d5()[3],
    ",
  0x4005024bu64 => "
      CAN_0.mb_d5()[4],
    ",
  0x4005025bu64 => "
      CAN_0.mb_d5()[5],
    ",
  0x4005026bu64 => "
      CAN_0.mb_d5()[6],
    ",
  0x4005027bu64 => "
      CAN_0.mb_d5()[7],
    ",
  0x4005028bu64 => "
      CAN_0.mb_d5()[8],
    ",
  0x4005029bu64 => "
      CAN_0.mb_d5()[9],
    ",
  0x400502abu64 => "
      CAN_0.mb_d5()[10],
    ",
  0x400502bbu64 => "
      CAN_0.mb_d5()[11],
    ",
  0x400502cbu64 => "
      CAN_0.mb_d5()[12],
    ",
  0x400502dbu64 => "
      CAN_0.mb_d5()[13],
    ",
  0x400502ebu64 => "
      CAN_0.mb_d5()[14],
    ",
  0x400502fbu64 => "
      CAN_0.mb_d5()[15],
    ",
  0x4005030bu64 => "
      CAN_0.mb_d5()[16],
    ",
  0x4005031bu64 => "
      CAN_0.mb_d5()[17],
    ",
  0x4005032bu64 => "
      CAN_0.mb_d5()[18],
    ",
  0x4005033bu64 => "
      CAN_0.mb_d5()[19],
    ",
  0x4005034bu64 => "
      CAN_0.mb_d5()[20],
    ",
  0x4005035bu64 => "
      CAN_0.mb_d5()[21],
    ",
  0x4005036bu64 => "
      CAN_0.mb_d5()[22],
    ",
  0x4005037bu64 => "
      CAN_0.mb_d5()[23],
    ",
  0x4005038bu64 => "
      CAN_0.mb_d5()[24],
    ",
  0x4005039bu64 => "
      CAN_0.mb_d5()[25],
    ",
  0x400503abu64 => "
      CAN_0.mb_d5()[26],
    ",
  0x400503bbu64 => "
      CAN_0.mb_d5()[27],
    ",
  0x400503cbu64 => "
      CAN_0.mb_d5()[28],
    ",
  0x400503dbu64 => "
      CAN_0.mb_d5()[29],
    ",
  0x400503ebu64 => "
      CAN_0.mb_d5()[30],
    ",
  0x400503fbu64 => "
      CAN_0.mb_d5()[31],
    ",
  0x4005020cu64 => "
      CAN_0.mb_d6()[0],
    ",
  0x4005021cu64 => "
      CAN_0.mb_d6()[1],
    ",
  0x4005022cu64 => "
      CAN_0.mb_d6()[2],
    ",
  0x4005023cu64 => "
      CAN_0.mb_d6()[3],
    ",
  0x4005024cu64 => "
      CAN_0.mb_d6()[4],
    ",
  0x4005025cu64 => "
      CAN_0.mb_d6()[5],
    ",
  0x4005026cu64 => "
      CAN_0.mb_d6()[6],
    ",
  0x4005027cu64 => "
      CAN_0.mb_d6()[7],
    ",
  0x4005028cu64 => "
      CAN_0.mb_d6()[8],
    ",
  0x4005029cu64 => "
      CAN_0.mb_d6()[9],
    ",
  0x400502acu64 => "
      CAN_0.mb_d6()[10],
    ",
  0x400502bcu64 => "
      CAN_0.mb_d6()[11],
    ",
  0x400502ccu64 => "
      CAN_0.mb_d6()[12],
    ",
  0x400502dcu64 => "
      CAN_0.mb_d6()[13],
    ",
  0x400502ecu64 => "
      CAN_0.mb_d6()[14],
    ",
  0x400502fcu64 => "
      CAN_0.mb_d6()[15],
    ",
  0x4005030cu64 => "
      CAN_0.mb_d6()[16],
    ",
  0x4005031cu64 => "
      CAN_0.mb_d6()[17],
    ",
  0x4005032cu64 => "
      CAN_0.mb_d6()[18],
    ",
  0x4005033cu64 => "
      CAN_0.mb_d6()[19],
    ",
  0x4005034cu64 => "
      CAN_0.mb_d6()[20],
    ",
  0x4005035cu64 => "
      CAN_0.mb_d6()[21],
    ",
  0x4005036cu64 => "
      CAN_0.mb_d6()[22],
    ",
  0x4005037cu64 => "
      CAN_0.mb_d6()[23],
    ",
  0x4005038cu64 => "
      CAN_0.mb_d6()[24],
    ",
  0x4005039cu64 => "
      CAN_0.mb_d6()[25],
    ",
  0x400503acu64 => "
      CAN_0.mb_d6()[26],
    ",
  0x400503bcu64 => "
      CAN_0.mb_d6()[27],
    ",
  0x400503ccu64 => "
      CAN_0.mb_d6()[28],
    ",
  0x400503dcu64 => "
      CAN_0.mb_d6()[29],
    ",
  0x400503ecu64 => "
      CAN_0.mb_d6()[30],
    ",
  0x400503fcu64 => "
      CAN_0.mb_d6()[31],
    ",
  0x4005020du64 => "
      CAN_0.mb_d7()[0],
    ",
  0x4005021du64 => "
      CAN_0.mb_d7()[1],
    ",
  0x4005022du64 => "
      CAN_0.mb_d7()[2],
    ",
  0x4005023du64 => "
      CAN_0.mb_d7()[3],
    ",
  0x4005024du64 => "
      CAN_0.mb_d7()[4],
    ",
  0x4005025du64 => "
      CAN_0.mb_d7()[5],
    ",
  0x4005026du64 => "
      CAN_0.mb_d7()[6],
    ",
  0x4005027du64 => "
      CAN_0.mb_d7()[7],
    ",
  0x4005028du64 => "
      CAN_0.mb_d7()[8],
    ",
  0x4005029du64 => "
      CAN_0.mb_d7()[9],
    ",
  0x400502adu64 => "
      CAN_0.mb_d7()[10],
    ",
  0x400502bdu64 => "
      CAN_0.mb_d7()[11],
    ",
  0x400502cdu64 => "
      CAN_0.mb_d7()[12],
    ",
  0x400502ddu64 => "
      CAN_0.mb_d7()[13],
    ",
  0x400502edu64 => "
      CAN_0.mb_d7()[14],
    ",
  0x400502fdu64 => "
      CAN_0.mb_d7()[15],
    ",
  0x4005030du64 => "
      CAN_0.mb_d7()[16],
    ",
  0x4005031du64 => "
      CAN_0.mb_d7()[17],
    ",
  0x4005032du64 => "
      CAN_0.mb_d7()[18],
    ",
  0x4005033du64 => "
      CAN_0.mb_d7()[19],
    ",
  0x4005034du64 => "
      CAN_0.mb_d7()[20],
    ",
  0x4005035du64 => "
      CAN_0.mb_d7()[21],
    ",
  0x4005036du64 => "
      CAN_0.mb_d7()[22],
    ",
  0x4005037du64 => "
      CAN_0.mb_d7()[23],
    ",
  0x4005038du64 => "
      CAN_0.mb_d7()[24],
    ",
  0x4005039du64 => "
      CAN_0.mb_d7()[25],
    ",
  0x400503adu64 => "
      CAN_0.mb_d7()[26],
    ",
  0x400503bdu64 => "
      CAN_0.mb_d7()[27],
    ",
  0x400503cdu64 => "
      CAN_0.mb_d7()[28],
    ",
  0x400503ddu64 => "
      CAN_0.mb_d7()[29],
    ",
  0x400503edu64 => "
      CAN_0.mb_d7()[30],
    ",
  0x400503fdu64 => "
      CAN_0.mb_d7()[31],
    ",
  0x4005020eu64 => "
      CAN_0.mb_ts()[0],
    ",
  0x4005021eu64 => "
      CAN_0.mb_ts()[1],
    ",
  0x4005022eu64 => "
      CAN_0.mb_ts()[2],
    ",
  0x4005023eu64 => "
      CAN_0.mb_ts()[3],
    ",
  0x4005024eu64 => "
      CAN_0.mb_ts()[4],
    ",
  0x4005025eu64 => "
      CAN_0.mb_ts()[5],
    ",
  0x4005026eu64 => "
      CAN_0.mb_ts()[6],
    ",
  0x4005027eu64 => "
      CAN_0.mb_ts()[7],
    ",
  0x4005028eu64 => "
      CAN_0.mb_ts()[8],
    ",
  0x4005029eu64 => "
      CAN_0.mb_ts()[9],
    ",
  0x400502aeu64 => "
      CAN_0.mb_ts()[10],
    ",
  0x400502beu64 => "
      CAN_0.mb_ts()[11],
    ",
  0x400502ceu64 => "
      CAN_0.mb_ts()[12],
    ",
  0x400502deu64 => "
      CAN_0.mb_ts()[13],
    ",
  0x400502eeu64 => "
      CAN_0.mb_ts()[14],
    ",
  0x400502feu64 => "
      CAN_0.mb_ts()[15],
    ",
  0x4005030eu64 => "
      CAN_0.mb_ts()[16],
    ",
  0x4005031eu64 => "
      CAN_0.mb_ts()[17],
    ",
  0x4005032eu64 => "
      CAN_0.mb_ts()[18],
    ",
  0x4005033eu64 => "
      CAN_0.mb_ts()[19],
    ",
  0x4005034eu64 => "
      CAN_0.mb_ts()[20],
    ",
  0x4005035eu64 => "
      CAN_0.mb_ts()[21],
    ",
  0x4005036eu64 => "
      CAN_0.mb_ts()[22],
    ",
  0x4005037eu64 => "
      CAN_0.mb_ts()[23],
    ",
  0x4005038eu64 => "
      CAN_0.mb_ts()[24],
    ",
  0x4005039eu64 => "
      CAN_0.mb_ts()[25],
    ",
  0x400503aeu64 => "
      CAN_0.mb_ts()[26],
    ",
  0x400503beu64 => "
      CAN_0.mb_ts()[27],
    ",
  0x400503ceu64 => "
      CAN_0.mb_ts()[28],
    ",
  0x400503deu64 => "
      CAN_0.mb_ts()[29],
    ",
  0x400503eeu64 => "
      CAN_0.mb_ts()[30],
    ",
  0x400503feu64 => "
      CAN_0.mb_ts()[31],
    ",
  0x40050400u64 => "
      CAN_0.mkr()[0],
    ",
  0x40050404u64 => "
      CAN_0.mkr()[1],
    ",
  0x40050408u64 => "
      CAN_0.mkr()[2],
    ",
  0x4005040cu64 => "
      CAN_0.mkr()[3],
    ",
  0x40050410u64 => "
      CAN_0.mkr()[4],
    ",
  0x40050414u64 => "
      CAN_0.mkr()[5],
    ",
  0x40050418u64 => "
      CAN_0.mkr()[6],
    ",
  0x4005041cu64 => "
      CAN_0.mkr()[7],
    ",
  0x40050420u64 => "
      CAN_0.fidcr()[0],
    ",
  0x40050424u64 => "
      CAN_0.fidcr()[1],
    ",
  0x40050428u64 => "
      CAN_0.mkivlr(),
    ",
  0x4005042cu64 => "
      CAN_0.mier(),
      CAN_0.mier_fifo(),
    ",
  0x40050820u64 => "
      CAN_0.mctl_rx()[0],
      CAN_0.mctl_tx()[0],
    ",
  0x40050821u64 => "
      CAN_0.mctl_rx()[1],
      CAN_0.mctl_tx()[1],
    ",
  0x40050822u64 => "
      CAN_0.mctl_rx()[2],
      CAN_0.mctl_tx()[2],
    ",
  0x40050823u64 => "
      CAN_0.mctl_rx()[3],
      CAN_0.mctl_tx()[3],
    ",
  0x40050824u64 => "
      CAN_0.mctl_rx()[4],
      CAN_0.mctl_tx()[4],
    ",
  0x40050825u64 => "
      CAN_0.mctl_rx()[5],
      CAN_0.mctl_tx()[5],
    ",
  0x40050826u64 => "
      CAN_0.mctl_rx()[6],
      CAN_0.mctl_tx()[6],
    ",
  0x40050827u64 => "
      CAN_0.mctl_rx()[7],
      CAN_0.mctl_tx()[7],
    ",
  0x40050828u64 => "
      CAN_0.mctl_rx()[8],
      CAN_0.mctl_tx()[8],
    ",
  0x40050829u64 => "
      CAN_0.mctl_rx()[9],
      CAN_0.mctl_tx()[9],
    ",
  0x4005082au64 => "
      CAN_0.mctl_rx()[10],
      CAN_0.mctl_tx()[10],
    ",
  0x4005082bu64 => "
      CAN_0.mctl_rx()[11],
      CAN_0.mctl_tx()[11],
    ",
  0x4005082cu64 => "
      CAN_0.mctl_rx()[12],
      CAN_0.mctl_tx()[12],
    ",
  0x4005082du64 => "
      CAN_0.mctl_rx()[13],
      CAN_0.mctl_tx()[13],
    ",
  0x4005082eu64 => "
      CAN_0.mctl_rx()[14],
      CAN_0.mctl_tx()[14],
    ",
  0x4005082fu64 => "
      CAN_0.mctl_rx()[15],
      CAN_0.mctl_tx()[15],
    ",
  0x40050830u64 => "
      CAN_0.mctl_rx()[16],
      CAN_0.mctl_tx()[16],
    ",
  0x40050831u64 => "
      CAN_0.mctl_rx()[17],
      CAN_0.mctl_tx()[17],
    ",
  0x40050832u64 => "
      CAN_0.mctl_rx()[18],
      CAN_0.mctl_tx()[18],
    ",
  0x40050833u64 => "
      CAN_0.mctl_rx()[19],
      CAN_0.mctl_tx()[19],
    ",
  0x40050834u64 => "
      CAN_0.mctl_rx()[20],
      CAN_0.mctl_tx()[20],
    ",
  0x40050835u64 => "
      CAN_0.mctl_rx()[21],
      CAN_0.mctl_tx()[21],
    ",
  0x40050836u64 => "
      CAN_0.mctl_rx()[22],
      CAN_0.mctl_tx()[22],
    ",
  0x40050837u64 => "
      CAN_0.mctl_rx()[23],
      CAN_0.mctl_tx()[23],
    ",
  0x40050838u64 => "
      CAN_0.mctl_rx()[24],
      CAN_0.mctl_tx()[24],
    ",
  0x40050839u64 => "
      CAN_0.mctl_rx()[25],
      CAN_0.mctl_tx()[25],
    ",
  0x4005083au64 => "
      CAN_0.mctl_rx()[26],
      CAN_0.mctl_tx()[26],
    ",
  0x4005083bu64 => "
      CAN_0.mctl_rx()[27],
      CAN_0.mctl_tx()[27],
    ",
  0x4005083cu64 => "
      CAN_0.mctl_rx()[28],
      CAN_0.mctl_tx()[28],
    ",
  0x4005083du64 => "
      CAN_0.mctl_rx()[29],
      CAN_0.mctl_tx()[29],
    ",
  0x4005083eu64 => "
      CAN_0.mctl_rx()[30],
      CAN_0.mctl_tx()[30],
    ",
  0x4005083fu64 => "
      CAN_0.mctl_rx()[31],
      CAN_0.mctl_tx()[31],
    ",
  0x40050840u64 => "
      CAN_0.ctlr(),
    ",
  0x40050842u64 => "
      CAN_0.str(),
    ",
  0x40050844u64 => "
      CAN_0.bcr(),
    ",
  0x40050848u64 => "
      CAN_0.rfcr(),
    ",
  0x40050849u64 => "
      CAN_0.rfpcr(),
    ",
  0x4005084au64 => "
      CAN_0.tfcr(),
    ",
  0x4005084bu64 => "
      CAN_0.tfpcr(),
    ",
  0x4005084cu64 => "
      CAN_0.eier(),
    ",
  0x4005084du64 => "
      CAN_0.eifr(),
    ",
  0x4005084eu64 => "
      CAN_0.recr(),
    ",
  0x4005084fu64 => "
      CAN_0.tecr(),
    ",
  0x40050850u64 => "
      CAN_0.ecsr(),
    ",
  0x40050851u64 => "
      CAN_0.cssr(),
    ",
  0x40050852u64 => "
      CAN_0.mssr(),
    ",
  0x40050853u64 => "
      CAN_0.msmr(),
    ",
  0x40050854u64 => "
      CAN_0.tsr(),
    ",
  0x40050856u64 => "
      CAN_0.afsr(),
    ",
  0x40050858u64 => "
      CAN_0.tcr(),
    ",
  0x40053000u64 => "
      IIC_0.iccr1(),
    ",
  0x40053001u64 => "
      IIC_0.iccr2(),
    ",
  0x40053002u64 => "
      IIC_0.icmr1(),
    ",
  0x40053003u64 => "
      IIC_0.icmr2(),
    ",
  0x40053004u64 => "
      IIC_0.icmr3(),
    ",
  0x40053005u64 => "
      IIC_0.icfer(),
    ",
  0x40053006u64 => "
      IIC_0.icser(),
    ",
  0x40053007u64 => "
      IIC_0.icier(),
    ",
  0x40053008u64 => "
      IIC_0.icsr1(),
    ",
  0x40053009u64 => "
      IIC_0.icsr2(),
    ",
  0x4005300au64 => "
      IIC_0.sarl()[0],
    ",
  0x4005300cu64 => "
      IIC_0.sarl()[1],
    ",
  0x4005300eu64 => "
      IIC_0.sarl()[2],
    ",
  0x4005300bu64 => "
      IIC_0.saru()[0],
    ",
  0x4005300du64 => "
      IIC_0.saru()[1],
    ",
  0x4005300fu64 => "
      IIC_0.saru()[2],
    ",
  0x40053010u64 => "
      IIC_0.icbrl(),
    ",
  0x40053011u64 => "
      IIC_0.icbrh(),
    ",
  0x40053012u64 => "
      IIC_0.icdrt(),
    ",
  0x40053013u64 => "
      IIC_0.icdrr(),
    ",
  0x40053016u64 => "
      IIC_0_WU.icwur(),
    ",
  0x40053017u64 => "
      IIC_0_WU.icwur2(),
    ",
  0x40054100u64 => "
      DOC.docr(),
    ",
  0x40054102u64 => "
      DOC.dodir(),
    ",
  0x40054104u64 => "
      DOC.dodsr(),
    ",
  0x4005c000u64 => "
      ADC_120.adcsr(),
    ",
  0x4005c004u64 => "
      ADC_120.adansa0(),
    ",
  0x4005c006u64 => "
      ADC_120.adansa1(),
    ",
  0x4005c008u64 => "
      ADC_120.adads0(),
    ",
  0x4005c00au64 => "
      ADC_120.adads1(),
    ",
  0x4005c00cu64 => "
      ADC_120.adadc(),
    ",
  0x4005c00eu64 => "
      ADC_120.adcer(),
    ",
  0x4005c010u64 => "
      ADC_120.adstrgr(),
    ",
  0x4005c012u64 => "
      ADC_120.adexicr(),
    ",
  0x4005c014u64 => "
      ADC_120.adansb0(),
    ",
  0x4005c016u64 => "
      ADC_120.adansb1(),
    ",
  0x4005c018u64 => "
      ADC_120.addbldr(),
    ",
  0x4005c01au64 => "
      ADC_120.adtsdr(),
    ",
  0x4005c01cu64 => "
      ADC_120.adocdr(),
    ",
  0x4005c01eu64 => "
      ADC_120.adrd(),
    ",
  0x4005c028u64 => "
      ADC_120.addr()[4],
    ",
  0x4005c02au64 => "
      ADC_120.addr()[5],
    ",
  0x4005c02cu64 => "
      ADC_120.addr()[6],
    ",
  0x4005c02eu64 => "
      ADC_120.addr()[7],
    ",
  0x4005c030u64 => "
      ADC_120.addr()[8],
    ",
  0x4005c032u64 => "
      ADC_120.addr()[9],
    ",
  0x4005c034u64 => "
      ADC_120.addr()[10],
    ",
  0x4005c036u64 => "
      ADC_120.addr()[11],
    ",
  0x4005c038u64 => "
      ADC_120.addr()[12],
    ",
  0x4005c03au64 => "
      ADC_120.addr()[13],
    ",
  0x4005c03cu64 => "
      ADC_120.addr()[14],
    ",
  0x4005c040u64 => "
      ADC_120.adctdr(),
    ",
  0x4005c042u64 => "
      ADC_120.addr()[0],
    ",
  0x4005c044u64 => "
      ADC_120.addr()[1],
    ",
  0x4005c046u64 => "
      ADC_120.addr()[2],
    ",
  0x4005c048u64 => "
      ADC_120.addr()[3],
    ",
  0x4005c07au64 => "
      ADC_120.addiscr(),
    ",
  0x4005c07eu64 => "
      ADC_120.adacsr(),
    ",
  0x4005c080u64 => "
      ADC_120.adgspcr(),
    ",
  0x4005c084u64 => "
      ADC_120.addbldra(),
    ",
  0x4005c086u64 => "
      ADC_120.addbldrb(),
    ",
  0x4005c08au64 => "
      ADC_120.adhvrefcnt(),
    ",
  0x4005c08cu64 => "
      ADC_120.adwinmon(),
    ",
  0x4005c090u64 => "
      ADC_120.adcmpcr(),
    ",
  0x4005c092u64 => "
      ADC_120.adcmpanser(),
    ",
  0x4005c093u64 => "
      ADC_120.adcmpler(),
    ",
  0x4005c094u64 => "
      ADC_120.adcmpansr0(),
    ",
  0x4005c096u64 => "
      ADC_120.adcmpansr1(),
    ",
  0x4005c098u64 => "
      ADC_120.adcmplr0(),
    ",
  0x4005c09au64 => "
      ADC_120.adcmplr1(),
    ",
  0x4005c09cu64 => "
      ADC_120.adcmpdr()[0],
    ",
  0x4005c09eu64 => "
      ADC_120.adcmpdr()[1],
    ",
  0x4005c0a0u64 => "
      ADC_120.adcmpsr0(),
    ",
  0x4005c0a2u64 => "
      ADC_120.adcmpsr1(),
    ",
  0x4005c0a4u64 => "
      ADC_120.adcmpser(),
    ",
  0x4005c0a6u64 => "
      ADC_120.adcmpbnsr(),
    ",
  0x4005c0a8u64 => "
      ADC_120.adwinllb(),
    ",
  0x4005c0aau64 => "
      ADC_120.adwinulb(),
    ",
  0x4005c0acu64 => "
      ADC_120.adcmpbsr(),
    ",
  0x4005c0ddu64 => "
      ADC_120.adsstrl(),
    ",
  0x4005c0deu64 => "
      ADC_120.adsstrt(),
    ",
  0x4005c0dfu64 => "
      ADC_120.adsstro(),
    ",
  0x4005c0e0u64 => "
      ADC_120.adsstr()[0],
    ",
  0x4005c0e1u64 => "
      ADC_120.adsstr()[1],
    ",
  0x4005c0e2u64 => "
      ADC_120.adsstr()[2],
    ",
  0x4005c0e3u64 => "
      ADC_120.adsstr()[3],
    ",
  0x4005c0e4u64 => "
      ADC_120.adsstr()[4],
    ",
  0x4005c0e5u64 => "
      ADC_120.adsstr()[5],
    ",
  0x4005c0e6u64 => "
      ADC_120.adsstr()[6],
    ",
  0x4005c0e7u64 => "
      ADC_120.adsstr()[7],
    ",
  0x4005c0e8u64 => "
      ADC_120.adsstr()[8],
    ",
  0x4005c0e9u64 => "
      ADC_120.adsstr()[9],
    ",
  0x4005c0eau64 => "
      ADC_120.adsstr()[10],
    ",
  0x4005c0ebu64 => "
      ADC_120.adsstr()[11],
    ",
  0x4005c0ecu64 => "
      ADC_120.adsstr()[12],
    ",
  0x4005c0edu64 => "
      ADC_120.adsstr()[13],
    ",
  0x4005c0eeu64 => "
      ADC_120.adsstr()[14],
    ",
  0x4005e000u64 => "
      DAC_12.dadr0(),
    ",
  0x4005e004u64 => "
      DAC_12.dacr(),
    ",
  0x4005e005u64 => "
      DAC_12.dadpr(),
    ",
  0x4005e006u64 => "
      DAC_12.daadscr(),
    ",
  0x4005e007u64 => "
      DAC_12.davrefcr(),
    ",
  0x40070000u64 => "
      SCI_0.smr(),
      SCI_0.smr_smci(),
    ",
  0x40070001u64 => "
      SCI_0.brr(),
    ",
  0x40070002u64 => "
      SCI_0.scr(),
      SCI_0.scr_smci(),
    ",
  0x40070003u64 => "
      SCI_0.tdr(),
    ",
  0x40070004u64 => "
      SCI_0.ssr(),
      SCI_0.ssr_fifo(),
      SCI_0.ssr_smci(),
    ",
  0x40070005u64 => "
      SCI_0.rdr(),
    ",
  0x40070006u64 => "
      SCI_0.scmr(),
    ",
  0x40070007u64 => "
      SCI_0.semr(),
    ",
  0x40070008u64 => "
      SCI_0.snfr(),
    ",
  0x40070009u64 => "
      SCI_0.simr1(),
    ",
  0x4007000au64 => "
      SCI_0.simr2(),
    ",
  0x4007000bu64 => "
      SCI_0.simr3(),
    ",
  0x4007000cu64 => "
      SCI_0.sisr(),
    ",
  0x4007000du64 => "
      SCI_0.spmr(),
    ",
  0x4007000eu64 => "
      SCI_0.ftdrhl(),
      SCI_0.tdrhl(),
      SCI_0.ftdrh(),
    ",
  0x4007000fu64 => "
      SCI_0.ftdrl(),
    ",
  0x40070010u64 => "
      SCI_0.frdrhl(),
      SCI_0.rdrhl(),
      SCI_0.frdrh(),
    ",
  0x40070011u64 => "
      SCI_0.frdrl(),
    ",
  0x40070012u64 => "
      SCI_0.mddr(),
    ",
  0x40070013u64 => "
      SCI_0.dccr(),
    ",
  0x40070014u64 => "
      SCI_0.fcr(),
    ",
  0x40070016u64 => "
      SCI_0.fdr(),
    ",
  0x40070018u64 => "
      SCI_0.lsr(),
    ",
  0x4007001au64 => "
      SCI_0.cdr(),
    ",
  0x4007001cu64 => "
      SCI_0.sptr(),
    ",
  0x40070020u64 => "
      SCI_1.smr(),
      SCI_1.smr_smci(),
    ",
  0x40070021u64 => "
      SCI_1.brr(),
    ",
  0x40070022u64 => "
      SCI_1.scr(),
      SCI_1.scr_smci(),
    ",
  0x40070023u64 => "
      SCI_1.tdr(),
    ",
  0x40070024u64 => "
      SCI_1.ssr(),
      SCI_1.ssr_smci(),
    ",
  0x40070025u64 => "
      SCI_1.rdr(),
    ",
  0x40070026u64 => "
      SCI_1.scmr(),
    ",
  0x40070027u64 => "
      SCI_1.semr(),
    ",
  0x40070028u64 => "
      SCI_1.snfr(),
    ",
  0x40070029u64 => "
      SCI_1.simr1(),
    ",
  0x4007002au64 => "
      SCI_1.simr2(),
    ",
  0x4007002bu64 => "
      SCI_1.simr3(),
    ",
  0x4007002cu64 => "
      SCI_1.sisr(),
    ",
  0x4007002du64 => "
      SCI_1.spmr(),
    ",
  0x4007002eu64 => "
      SCI_1.tdrhl(),
    ",
  0x40070030u64 => "
      SCI_1.rdrhl(),
    ",
  0x40070032u64 => "
      SCI_1.mddr(),
    ",
  0x40070033u64 => "
      SCI_1.dccr(),
    ",
  0x4007003au64 => "
      SCI_1.cdr(),
    ",
  0x4007003cu64 => "
      SCI_1.sptr(),
    ",
  0x40072000u64 => "
      SPI_0.spcr(),
    ",
  0x40072001u64 => "
      SPI_0.sslp(),
    ",
  0x40072002u64 => "
      SPI_0.sppcr(),
    ",
  0x40072003u64 => "
      SPI_0.spsr(),
    ",
  0x40072004u64 => "
      SPI_0.spdr(),
      SPI_0.spdr_ha(),
      SPI_0.spdr_by(),
    ",
  0x4007200au64 => "
      SPI_0.spbr(),
    ",
  0x4007200bu64 => "
      SPI_0.spdcr(),
    ",
  0x4007200cu64 => "
      SPI_0.spckd(),
    ",
  0x4007200du64 => "
      SPI_0.sslnd(),
    ",
  0x4007200eu64 => "
      SPI_0.spnd(),
    ",
  0x4007200fu64 => "
      SPI_0.spcr2(),
    ",
  0x40072010u64 => "
      SPI_0.spcmd0(),
    ",
  0x40074000u64 => "
      CRC.crccr0(),
    ",
  0x40074001u64 => "
      CRC.crccr1(),
    ",
  0x40074004u64 => "
      CRC.crcdir(),
      CRC.crcdir_by(),
    ",
  0x40074008u64 => "
      CRC.crcdor(),
      CRC.crcdor_ha(),
      CRC.crcdor_by(),
    ",
  0x4007400cu64 => "
      CRC.crcsar(),
    ",
  0x40078000u64 => "
      GPT_320.gtwp(),
    ",
  0x40078004u64 => "
      GPT_320.gtstr(),
    ",
  0x40078008u64 => "
      GPT_320.gtstp(),
    ",
  0x4007800cu64 => "
      GPT_320.gtclr(),
    ",
  0x40078010u64 => "
      GPT_320.gtssr(),
    ",
  0x40078014u64 => "
      GPT_320.gtpsr(),
    ",
  0x40078018u64 => "
      GPT_320.gtcsr(),
    ",
  0x4007801cu64 => "
      GPT_320.gtupsr(),
    ",
  0x40078020u64 => "
      GPT_320.gtdnsr(),
    ",
  0x40078024u64 => "
      GPT_320.gticasr(),
    ",
  0x40078028u64 => "
      GPT_320.gticbsr(),
    ",
  0x4007802cu64 => "
      GPT_320.gtcr(),
    ",
  0x40078030u64 => "
      GPT_320.gtuddtyc(),
    ",
  0x40078034u64 => "
      GPT_320.gtior(),
    ",
  0x40078038u64 => "
      GPT_320.gtintad(),
    ",
  0x4007803cu64 => "
      GPT_320.gtst(),
    ",
  0x40078040u64 => "
      GPT_320.gtber(),
    ",
  0x40078048u64 => "
      GPT_320.gtcnt(),
    ",
  0x4007804cu64 => "
      GPT_320.gtccra(),
    ",
  0x40078050u64 => "
      GPT_320.gtccrb(),
    ",
  0x40078054u64 => "
      GPT_320.gtccrc(),
    ",
  0x40078058u64 => "
      GPT_320.gtccre(),
    ",
  0x4007805cu64 => "
      GPT_320.gtccrd(),
    ",
  0x40078060u64 => "
      GPT_320.gtccrf(),
    ",
  0x40078064u64 => "
      GPT_320.gtpr(),
    ",
  0x40078068u64 => "
      GPT_320.gtpbr(),
    ",
  0x40078088u64 => "
      GPT_320.gtdtcr(),
    ",
  0x4007808cu64 => "
      GPT_320.gtdvu(),
    ",
  0x40078400u64 => "
      GPT_164.gtwp(),
    ",
  0x40078404u64 => "
      GPT_164.gtstr(),
    ",
  0x40078408u64 => "
      GPT_164.gtstp(),
    ",
  0x4007840cu64 => "
      GPT_164.gtclr(),
    ",
  0x40078410u64 => "
      GPT_164.gtssr(),
    ",
  0x40078414u64 => "
      GPT_164.gtpsr(),
    ",
  0x40078418u64 => "
      GPT_164.gtcsr(),
    ",
  0x4007841cu64 => "
      GPT_164.gtupsr(),
    ",
  0x40078420u64 => "
      GPT_164.gtdnsr(),
    ",
  0x40078424u64 => "
      GPT_164.gticasr(),
    ",
  0x40078428u64 => "
      GPT_164.gticbsr(),
    ",
  0x4007842cu64 => "
      GPT_164.gtcr(),
    ",
  0x40078430u64 => "
      GPT_164.gtuddtyc(),
    ",
  0x40078434u64 => "
      GPT_164.gtior(),
    ",
  0x40078438u64 => "
      GPT_164.gtintad(),
    ",
  0x4007843cu64 => "
      GPT_164.gtst(),
    ",
  0x40078440u64 => "
      GPT_164.gtber(),
    ",
  0x40078448u64 => "
      GPT_164.gtcnt(),
    ",
  0x4007844cu64 => "
      GPT_164.gtccra(),
    ",
  0x40078450u64 => "
      GPT_164.gtccrb(),
    ",
  0x40078454u64 => "
      GPT_164.gtccrc(),
    ",
  0x40078458u64 => "
      GPT_164.gtccre(),
    ",
  0x4007845cu64 => "
      GPT_164.gtccrd(),
    ",
  0x40078460u64 => "
      GPT_164.gtccrf(),
    ",
  0x40078464u64 => "
      GPT_164.gtpr(),
    ",
  0x40078468u64 => "
      GPT_164.gtpbr(),
    ",
  0x40078488u64 => "
      GPT_164.gtdtcr(),
    ",
  0x4007848cu64 => "
      GPT_164.gtdvu(),
    ",
  0x40078ff0u64 => "
      GPT_OPS.opscr(),
    ",
  0x40080000u64 => "
      KINT.krctl(),
    ",
  0x40080004u64 => "
      KINT.krf(),
    ",
  0x40080008u64 => "
      KINT.krm(),
    ",
  0x40082000u64 => "
      CTSU.ctsucra(),
      CTSU.ctsucral(),
      CTSU.ctsucr0(),
    ",
  0x40082001u64 => "
      CTSU.ctsucr1(),
    ",
  0x40082002u64 => "
      CTSU.ctsucrah(),
      CTSU.ctsucr2(),
    ",
  0x40082003u64 => "
      CTSU.ctsucr3(),
    ",
  0x40082004u64 => "
      CTSU.ctsucrb(),
      CTSU.ctsucrbl(),
      CTSU.ctsusdprs(),
    ",
  0x40082005u64 => "
      CTSU.ctsusst(),
    ",
  0x40082006u64 => "
      CTSU.ctsucrbh(),
    ",
  0x40082007u64 => "
      CTSU.ctsudclkc(),
    ",
  0x40082008u64 => "
      CTSU.ctsumch(),
      CTSU.ctsumchl(),
      CTSU.ctsumch0(),
    ",
  0x40082009u64 => "
      CTSU.ctsumch1(),
    ",
  0x4008200au64 => "
      CTSU.ctsumchh(),
      CTSU.ctsumfaf(),
    ",
  0x4008200cu64 => "
      CTSU.ctsuchaca(),
      CTSU.ctsuchacal(),
      CTSU.ctsuchac0(),
    ",
  0x4008200du64 => "
      CTSU.ctsuchac1(),
    ",
  0x4008200eu64 => "
      CTSU.ctsuchacah(),
      CTSU.ctsuchac2(),
    ",
  0x4008200fu64 => "
      CTSU.ctsuchac3(),
    ",
  0x40082010u64 => "
      CTSU.ctsuchacb(),
      CTSU.ctsuchacbl(),
      CTSU.ctsuchac4(),
    ",
  0x40082014u64 => "
      CTSU.ctsuchtrca(),
      CTSU.ctsuchtrcal(),
      CTSU.ctsuchtrc0(),
    ",
  0x40082015u64 => "
      CTSU.ctsuchtrc1(),
    ",
  0x40082016u64 => "
      CTSU.ctsuchtrcah(),
      CTSU.ctsuchtrc2(),
    ",
  0x40082017u64 => "
      CTSU.ctsuchtrc3(),
    ",
  0x40082018u64 => "
      CTSU.ctsuchtrcb(),
      CTSU.ctsuchtrcbl(),
      CTSU.ctsuchtrc4(),
    ",
  0x4008201cu64 => "
      CTSU.ctsusr(),
      CTSU.ctsusrl(),
      CTSU.ctsusr0(),
    ",
  0x4008201du64 => "
      CTSU.ctsust(),
    ",
  0x4008201eu64 => "
      CTSU.ctsusrh(),
      CTSU.ctsusr2(),
    ",
  0x40082020u64 => "
      CTSU.ctsuso(),
      CTSU.ctsuso0(),
    ",
  0x40082022u64 => "
      CTSU.ctsuso1(),
    ",
  0x40082024u64 => "
      CTSU.ctsuscnt(),
      CTSU.ctsusc(),
    ",
  0x40082028u64 => "
      CTSU.ctsucalib(),
      CTSU.ctsudbgr0(),
    ",
  0x4008202au64 => "
      CTSU.ctsudbgr1(),
    ",
  0x4008202cu64 => "
      CTSU.ctsusuclka(),
      CTSU.ctsusuclk0(),
    ",
  0x4008202eu64 => "
      CTSU.ctsusuclk1(),
    ",
  0x40082030u64 => "
      CTSU.ctsusuclkb(),
      CTSU.ctsusuclk2(),
    ",
  0x40082032u64 => "
      CTSU.ctsusuclk3(),
    ",
  0x40082034u64 => "
      CTSU.ctsucfccnt(),
      CTSU.ctsucfccntl(),
    ",
  0x40084000u64 => "
      AGT_0.agt(),
    ",
  0x40084002u64 => "
      AGT_0.agtcma(),
    ",
  0x40084004u64 => "
      AGT_0.agtcmb(),
    ",
  0x40084008u64 => "
      AGT_0.agtcr(),
    ",
  0x40084009u64 => "
      AGT_0.agtmr1(),
    ",
  0x4008400au64 => "
      AGT_0.agtmr2(),
    ",
  0x4008400cu64 => "
      AGT_0.agtioc(),
    ",
  0x4008400du64 => "
      AGT_0.agtisr(),
    ",
  0x4008400eu64 => "
      AGT_0.agtcmsr(),
    ",
  0x4008400fu64 => "
      AGT_0.agtiosel(),
    ",
  0x40085e00u64 => "
      ACMPLP.compmdr(),
    ",
  0x40085e01u64 => "
      ACMPLP.compfir(),
    ",
  0x40085e02u64 => "
      ACMPLP.compocr(),
    ",
  0x407ec090u64 => "
      FLCN.dflctl(),
    ",
  0x407ec100u64 => "
      FLCN.fpmcr(),
    ",
  0x407ec104u64 => "
      FLCN.fasr(),
    ",
  0x407ec108u64 => "
      FLCN.fsarl(),
    ",
  0x407ec110u64 => "
      FLCN.fsarh(),
    ",
  0x407ec114u64 => "
      FLCN.fcr(),
    ",
  0x407ec118u64 => "
      FLCN.fearl(),
    ",
  0x407ec120u64 => "
      FLCN.fearh(),
    ",
  0x407ec124u64 => "
      FLCN.fresetr(),
    ",
  0x407ec12cu64 => "
      FLCN.fstatr1(),
    ",
  0x407ec130u64 => "
      FLCN.fwbl0(),
    ",
  0x407ec138u64 => "
      FLCN.fwbh0(),
    ",
  0x407ec180u64 => "
      FLCN.fpr(),
    ",
  0x407ec184u64 => "
      FLCN.fpsr(),
    ",
  0x407ec188u64 => "
      FLCN.frbl0(),
    ",
  0x407ec190u64 => "
      FLCN.frbh0(),
    ",
  0x407ec1c0u64 => "
      FLCN.fscmr(),
    ",
  0x407ec1c8u64 => "
      FLCN.fawsmr(),
    ",
  0x407ec1d0u64 => "
      FLCN.fawemr(),
    ",
  0x407ec1d8u64 => "
      FLCN.fisr(),
    ",
  0x407ec1dcu64 => "
      FLCN.fexcr(),
    ",
  0x407ec1e0u64 => "
      FLCN.feaml(),
    ",
  0x407ec1e8u64 => "
      FLCN.feamh(),
    ",
  0x407ec1f0u64 => "
      FLCN.fstatr2(),
    ",
  0x407ec228u64 => "
      FLCN.tscdr(),
    ",
  0x407ec3a4u64 => "
      FLCN.ctsutrima(),
    ",
  0x407ec3a8u64 => "
      FLCN.ctsutrimb(),
    ",
  0x407effb0u64 => "
      FLCN.fentryr(),
    ",
  0x407effc4u64 => "
      FLCN.fldwaitr(),
    ",
  0x407effc8u64 => "
      FLCN.pfber(),
    ",
};
