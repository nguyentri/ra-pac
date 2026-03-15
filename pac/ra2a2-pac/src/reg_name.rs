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
// Generated from SVD 1.20.02, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:01:00 +0000

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
  0x40001000u64 => "
      MMF.mmsfr(),
    ",
  0x40001004u64 => "
      MMF.mmen(),
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
  0x400061c0u64 => "
      ICU.ielen(),
    ",
  0x40006200u64 => "
      ICU.selsr0(),
    ",
  0x40006380u64 => "
      ICU.ielsr()[0],
    ",
  0x40006384u64 => "
      ICU.ielsr()[1],
    ",
  0x40006388u64 => "
      ICU.ielsr()[2],
    ",
  0x4000638cu64 => "
      ICU.ielsr()[3],
    ",
  0x40006390u64 => "
      ICU.ielsr()[4],
    ",
  0x40006394u64 => "
      ICU.ielsr()[5],
    ",
  0x40006398u64 => "
      ICU.ielsr()[6],
    ",
  0x4000639cu64 => "
      ICU.ielsr()[7],
    ",
  0x400063a0u64 => "
      ICU.ielsr()[8],
    ",
  0x400063a4u64 => "
      ICU.ielsr()[9],
    ",
  0x400063a8u64 => "
      ICU.ielsr()[10],
    ",
  0x400063acu64 => "
      ICU.ielsr()[11],
    ",
  0x400063b0u64 => "
      ICU.ielsr()[12],
    ",
  0x400063b4u64 => "
      ICU.ielsr()[13],
    ",
  0x400063b8u64 => "
      ICU.ielsr()[14],
    ",
  0x400063bcu64 => "
      ICU.ielsr()[15],
    ",
  0x400063c0u64 => "
      ICU.ielsr()[16],
    ",
  0x400063c4u64 => "
      ICU.ielsr()[17],
    ",
  0x400063c8u64 => "
      ICU.ielsr()[18],
    ",
  0x400063ccu64 => "
      ICU.ielsr()[19],
    ",
  0x400063d0u64 => "
      ICU.ielsr()[20],
    ",
  0x400063d4u64 => "
      ICU.ielsr()[21],
    ",
  0x400063d8u64 => "
      ICU.ielsr()[22],
    ",
  0x400063dcu64 => "
      ICU.ielsr()[23],
    ",
  0x400063e0u64 => "
      ICU.ielsr()[24],
    ",
  0x400063e4u64 => "
      ICU.ielsr()[25],
    ",
  0x400063e8u64 => "
      ICU.ielsr()[26],
    ",
  0x400063ecu64 => "
      ICU.ielsr()[27],
    ",
  0x400063f0u64 => "
      ICU.ielsr()[28],
    ",
  0x400063f4u64 => "
      ICU.ielsr()[29],
    ",
  0x400063f8u64 => "
      ICU.ielsr()[30],
    ",
  0x400063fcu64 => "
      ICU.ielsr()[31],
    ",
  0x40006400u64 => "
      ICU.ielsr()[32],
    ",
  0x40006404u64 => "
      ICU.ielsr()[33],
    ",
  0x40006408u64 => "
      ICU.ielsr()[34],
    ",
  0x4000640cu64 => "
      ICU.ielsr()[35],
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
  0x4001e050u64 => "
      SYSC.slcdsckcr(),
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
  0x4001e0a5u64 => "
      SYSC.hocowtcr(),
    ",
  0x4001e0aau64 => "
      SYSC.sopccr(),
    ",
  0x4001e0c0u64 => "
      SYSC.rstsr1(),
    ",
  0x4001e0d1u64 => "
      SYSC.sdadcckcr(),
    ",
  0x4001e0d8u64 => "
      SYSC.sostd(),
    ",
  0x4001e0dau64 => "
      SYSC.mostd(),
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
  0x4001e4c0u64 => "
      SYSC.vbtlvdcr(),
    ",
  0x4001e4c1u64 => "
      SYSC.vbtlvdsr(),
    ",
  0x4001e4c2u64 => "
      SYSC.vbtcmpcr(),
    ",
  0x4001e4c3u64 => "
      SYSC.vbtlvdicr(),
    ",
  0x4001e4c4u64 => "
      SYSC.vrtlvdcr(),
    ",
  0x4001e4c5u64 => "
      SYSC.vrtsr(),
    ",
  0x4001e4c6u64 => "
      SYSC.vrtcmpcr(),
    ",
  0x4001e4c7u64 => "
      SYSC.vrtlvdicr(),
    ",
  0x4001e4c8u64 => "
      SYSC.exlvdcr(),
    ",
  0x4001e4c9u64 => "
      SYSC.exlvdsr(),
    ",
  0x4001e4cau64 => "
      SYSC.exlvdcmpcr(),
    ",
  0x4001e4cbu64 => "
      SYSC.exlvdicr(),
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
      PORT_0.eidr(),
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
  0x40040810u64 => "
      PFS.p00pfs()[0],
    ",
  0x40040814u64 => "
      PFS.p00pfs()[1],
    ",
  0x40040818u64 => "
      PFS.p00pfs()[2],
    ",
  0x4004081cu64 => "
      PFS.p00pfs()[3],
    ",
  0x40040820u64 => "
      PFS.p00pfs()[4],
    ",
  0x40040824u64 => "
      PFS.p00pfs()[5],
    ",
  0x40040812u64 => "
      PFS.p00pfs_ha()[0],
    ",
  0x40040816u64 => "
      PFS.p00pfs_ha()[1],
    ",
  0x4004081au64 => "
      PFS.p00pfs_ha()[2],
    ",
  0x4004081eu64 => "
      PFS.p00pfs_ha()[3],
    ",
  0x40040822u64 => "
      PFS.p00pfs_ha()[4],
    ",
  0x40040826u64 => "
      PFS.p00pfs_ha()[5],
    ",
  0x40040813u64 => "
      PFS.p00pfs_by()[0],
    ",
  0x40040817u64 => "
      PFS.p00pfs_by()[1],
    ",
  0x4004081bu64 => "
      PFS.p00pfs_by()[2],
    ",
  0x4004081fu64 => "
      PFS.p00pfs_by()[3],
    ",
  0x40040823u64 => "
      PFS.p00pfs_by()[4],
    ",
  0x40040827u64 => "
      PFS.p00pfs_by()[5],
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
  0x4004088cu64 => "
      PFS.p20pfs()[0],
    ",
  0x40040890u64 => "
      PFS.p20pfs()[1],
    ",
  0x40040894u64 => "
      PFS.p20pfs()[2],
    ",
  0x40040898u64 => "
      PFS.p20pfs()[3],
    ",
  0x4004089cu64 => "
      PFS.p20pfs()[4],
    ",
  0x400408a0u64 => "
      PFS.p20pfs()[5],
    ",
  0x400408a4u64 => "
      PFS.p20pfs()[6],
    ",
  0x4004088eu64 => "
      PFS.p20pfs_ha()[0],
    ",
  0x40040892u64 => "
      PFS.p20pfs_ha()[1],
    ",
  0x40040896u64 => "
      PFS.p20pfs_ha()[2],
    ",
  0x4004089au64 => "
      PFS.p20pfs_ha()[3],
    ",
  0x4004089eu64 => "
      PFS.p20pfs_ha()[4],
    ",
  0x400408a2u64 => "
      PFS.p20pfs_ha()[5],
    ",
  0x400408a6u64 => "
      PFS.p20pfs_ha()[6],
    ",
  0x4004088fu64 => "
      PFS.p20pfs_by()[0],
    ",
  0x40040893u64 => "
      PFS.p20pfs_by()[1],
    ",
  0x40040897u64 => "
      PFS.p20pfs_by()[2],
    ",
  0x4004089bu64 => "
      PFS.p20pfs_by()[3],
    ",
  0x4004089fu64 => "
      PFS.p20pfs_by()[4],
    ",
  0x400408a3u64 => "
      PFS.p20pfs_by()[5],
    ",
  0x400408a7u64 => "
      PFS.p20pfs_by()[6],
    ",
  0x400408a8u64 => "
      PFS.p2pfs()[0],
    ",
  0x400408acu64 => "
      PFS.p2pfs()[1],
    ",
  0x400408b0u64 => "
      PFS.p2pfs()[2],
    ",
  0x400408b4u64 => "
      PFS.p2pfs()[3],
    ",
  0x400408b8u64 => "
      PFS.p2pfs()[4],
    ",
  0x400408bcu64 => "
      PFS.p2pfs()[5],
    ",
  0x400408aau64 => "
      PFS.p2pfs_ha()[0],
    ",
  0x400408aeu64 => "
      PFS.p2pfs_ha()[1],
    ",
  0x400408b2u64 => "
      PFS.p2pfs_ha()[2],
    ",
  0x400408b6u64 => "
      PFS.p2pfs_ha()[3],
    ",
  0x400408bau64 => "
      PFS.p2pfs_ha()[4],
    ",
  0x400408beu64 => "
      PFS.p2pfs_ha()[5],
    ",
  0x400408abu64 => "
      PFS.p2pfs_by()[0],
    ",
  0x400408afu64 => "
      PFS.p2pfs_by()[1],
    ",
  0x400408b3u64 => "
      PFS.p2pfs_by()[2],
    ",
  0x400408b7u64 => "
      PFS.p2pfs_by()[3],
    ",
  0x400408bbu64 => "
      PFS.p2pfs_by()[4],
    ",
  0x400408bfu64 => "
      PFS.p2pfs_by()[5],
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
  0x400408e0u64 => "
      PFS.p30pfs()[7],
    ",
  0x400408e4u64 => "
      PFS.p30pfs()[8],
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
  0x400408e2u64 => "
      PFS.p30pfs_ha()[7],
    ",
  0x400408e6u64 => "
      PFS.p30pfs_ha()[8],
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
  0x400408e3u64 => "
      PFS.p30pfs_by()[7],
    ",
  0x400408e7u64 => "
      PFS.p30pfs_by()[8],
    ",
  0x400408e8u64 => "
      PFS.p3pfs()[0],
    ",
  0x400408ecu64 => "
      PFS.p3pfs()[1],
    ",
  0x400408f0u64 => "
      PFS.p3pfs()[2],
    ",
  0x400408f4u64 => "
      PFS.p3pfs()[3],
    ",
  0x400408eau64 => "
      PFS.p3pfs_ha()[0],
    ",
  0x400408eeu64 => "
      PFS.p3pfs_ha()[1],
    ",
  0x400408f2u64 => "
      PFS.p3pfs_ha()[2],
    ",
  0x400408f6u64 => "
      PFS.p3pfs_ha()[3],
    ",
  0x400408ebu64 => "
      PFS.p3pfs_by()[0],
    ",
  0x400408efu64 => "
      PFS.p3pfs_by()[1],
    ",
  0x400408f3u64 => "
      PFS.p3pfs_by()[2],
    ",
  0x400408f7u64 => "
      PFS.p3pfs_by()[3],
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
  0x40040920u64 => "
      PFS.p40pfs()[0],
    ",
  0x40040924u64 => "
      PFS.p40pfs()[1],
    ",
  0x40040922u64 => "
      PFS.p40pfs_ha()[0],
    ",
  0x40040926u64 => "
      PFS.p40pfs_ha()[1],
    ",
  0x40040923u64 => "
      PFS.p40pfs_by()[0],
    ",
  0x40040927u64 => "
      PFS.p40pfs_by()[1],
    ",
  0x40040928u64 => "
      PFS.p4pfs()[0],
    ",
  0x4004092cu64 => "
      PFS.p4pfs()[1],
    ",
  0x4004092au64 => "
      PFS.p4pfs_ha()[0],
    ",
  0x4004092eu64 => "
      PFS.p4pfs_ha()[1],
    ",
  0x4004092bu64 => "
      PFS.p4pfs_by()[0],
    ",
  0x4004092fu64 => "
      PFS.p4pfs_by()[1],
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
  0x40040958u64 => "
      PFS.p50pfs()[6],
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
  0x4004095au64 => "
      PFS.p50pfs_ha()[6],
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
  0x4004095bu64 => "
      PFS.p50pfs_by()[6],
    ",
  0x40040980u64 => "
      PFS.p600pfs(),
    ",
  0x40040982u64 => "
      PFS.p600pfs_ha(),
    ",
  0x40040983u64 => "
      PFS.p600pfs_by(),
    ",
  0x40040985u64 => "
      PFS.vl1sel(),
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
  0x40041048u64 => "
      ELC.elsr()[0],
    ",
  0x4004104cu64 => "
      ELC.elsr()[1],
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
      RTC.bcnt0ar()[0],
      RTC.rsecar()[0],
    ",
  0x40044030u64 => "
      RTC.bcnt0ar()[1],
      RTC.rsecar()[1],
    ",
  0x40044012u64 => "
      RTC.bcnt1ar()[0],
      RTC.rminar()[0],
    ",
  0x40044032u64 => "
      RTC.bcnt1ar()[1],
      RTC.rminar()[1],
    ",
  0x40044014u64 => "
      RTC.bcnt2ar()[0],
      RTC.rhrar()[0],
    ",
  0x40044034u64 => "
      RTC.bcnt2ar()[1],
      RTC.rhrar()[1],
    ",
  0x40044016u64 => "
      RTC.bcnt3ar()[0],
      RTC.rwkar()[0],
    ",
  0x40044036u64 => "
      RTC.bcnt3ar()[1],
      RTC.rwkar()[1],
    ",
  0x40044018u64 => "
      RTC.bcnt0aer()[0],
      RTC.rdayar()[0],
    ",
  0x40044038u64 => "
      RTC.bcnt0aer()[1],
      RTC.rdayar()[1],
    ",
  0x4004401au64 => "
      RTC.bcnt1aer()[0],
      RTC.rmonar()[0],
    ",
  0x4004403au64 => "
      RTC.bcnt1aer()[1],
      RTC.rmonar()[1],
    ",
  0x4004401cu64 => "
      RTC.bcnt2aer()[0],
      RTC.ryrar()[0],
    ",
  0x4004403cu64 => "
      RTC.bcnt2aer()[1],
      RTC.ryrar()[1],
    ",
  0x4004401eu64 => "
      RTC.bcnt3aer()[0],
      RTC.ryraren()[0],
    ",
  0x4004403eu64 => "
      RTC.bcnt3aer()[1],
      RTC.ryraren()[1],
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
  0x4004402eu64 => "
      RTC.radj(),
    ",
  0x40044040u64 => "
      RTC.rtccr()[0],
    ",
  0x40044042u64 => "
      RTC.rtccr()[1],
    ",
  0x40044044u64 => "
      RTC.rtccr()[2],
    ",
  0x40044052u64 => "
      RTC.bcnt0cp()[0],
      RTC.rseccp()[0],
    ",
  0x40044062u64 => "
      RTC.bcnt0cp()[1],
      RTC.rseccp()[1],
    ",
  0x40044072u64 => "
      RTC.bcnt0cp()[2],
      RTC.rseccp()[2],
    ",
  0x40044054u64 => "
      RTC.bcnt1cp()[0],
      RTC.rmincp()[0],
    ",
  0x40044064u64 => "
      RTC.bcnt1cp()[1],
      RTC.rmincp()[1],
    ",
  0x40044074u64 => "
      RTC.bcnt1cp()[2],
      RTC.rmincp()[2],
    ",
  0x40044056u64 => "
      RTC.bcnt2cp()[0],
      RTC.rhrcp()[0],
    ",
  0x40044066u64 => "
      RTC.bcnt2cp()[1],
      RTC.rhrcp()[1],
    ",
  0x40044076u64 => "
      RTC.bcnt2cp()[2],
      RTC.rhrcp()[2],
    ",
  0x4004405au64 => "
      RTC.bcnt3cp()[0],
      RTC.rdaycp()[0],
    ",
  0x4004406au64 => "
      RTC.bcnt3cp()[1],
      RTC.rdaycp()[1],
    ",
  0x4004407au64 => "
      RTC.bcnt3cp()[2],
      RTC.rdaycp()[2],
    ",
  0x4004405cu64 => "
      RTC.rmoncp()[0],
    ",
  0x4004406cu64 => "
      RTC.rmoncp()[1],
    ",
  0x4004407cu64 => "
      RTC.rmoncp()[2],
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
  0x4005c008u64 => "
      ADC_120.adads0(),
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
  0x4005c020u64 => "
      ADC_120.addr()[0],
    ",
  0x4005c022u64 => "
      ADC_120.addr()[1],
    ",
  0x4005c024u64 => "
      ADC_120.addr()[2],
    ",
  0x4005c026u64 => "
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
  0x4005c098u64 => "
      ADC_120.adcmplr0(),
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
  0x40082000u64 => "
      SLCDC.lcdm0(),
    ",
  0x40082001u64 => "
      SLCDC.lcdm1(),
    ",
  0x40082002u64 => "
      SLCDC.lcdc0(),
    ",
  0x40082003u64 => "
      SLCDC.vlcd(),
    ",
  0x40082100u64 => "
      SLCDC.seg()[0],
    ",
  0x40082101u64 => "
      SLCDC.seg()[1],
    ",
  0x40082102u64 => "
      SLCDC.seg()[2],
    ",
  0x40082103u64 => "
      SLCDC.seg()[3],
    ",
  0x40082104u64 => "
      SLCDC.seg()[4],
    ",
  0x40082105u64 => "
      SLCDC.seg()[5],
    ",
  0x40082106u64 => "
      SLCDC.seg()[6],
    ",
  0x40082107u64 => "
      SLCDC.seg()[7],
    ",
  0x40082108u64 => "
      SLCDC.seg()[8],
    ",
  0x40082109u64 => "
      SLCDC.seg()[9],
    ",
  0x4008210au64 => "
      SLCDC.seg()[10],
    ",
  0x4008210bu64 => "
      SLCDC.seg()[11],
    ",
  0x4008210cu64 => "
      SLCDC.seg()[12],
    ",
  0x4008210du64 => "
      SLCDC.seg()[13],
    ",
  0x4008210eu64 => "
      SLCDC.seg()[14],
    ",
  0x4008210fu64 => "
      SLCDC.seg()[15],
    ",
  0x40082110u64 => "
      SLCDC.seg()[16],
    ",
  0x40082111u64 => "
      SLCDC.seg()[17],
    ",
  0x40082112u64 => "
      SLCDC.seg()[18],
    ",
  0x40082113u64 => "
      SLCDC.seg()[19],
    ",
  0x40082114u64 => "
      SLCDC.seg()[20],
    ",
  0x40082115u64 => "
      SLCDC.seg()[21],
    ",
  0x40082116u64 => "
      SLCDC.seg()[22],
    ",
  0x40082117u64 => "
      SLCDC.seg()[23],
    ",
  0x40082118u64 => "
      SLCDC.seg()[24],
    ",
  0x40082119u64 => "
      SLCDC.seg()[25],
    ",
  0x4008211au64 => "
      SLCDC.seg()[26],
    ",
  0x4008211bu64 => "
      SLCDC.seg()[27],
    ",
  0x4008211cu64 => "
      SLCDC.seg()[28],
    ",
  0x4008211du64 => "
      SLCDC.seg()[29],
    ",
  0x4008211eu64 => "
      SLCDC.seg()[30],
    ",
  0x4008211fu64 => "
      SLCDC.seg()[31],
    ",
  0x40082120u64 => "
      SLCDC.seg()[32],
    ",
  0x40082121u64 => "
      SLCDC.seg()[33],
    ",
  0x40082122u64 => "
      SLCDC.seg()[34],
    ",
  0x40082123u64 => "
      SLCDC.seg()[35],
    ",
  0x40082124u64 => "
      SLCDC.seg()[36],
    ",
  0x40082125u64 => "
      SLCDC.seg()[37],
    ",
  0x40082126u64 => "
      SLCDC.seg()[38],
    ",
  0x40082127u64 => "
      SLCDC.seg()[39],
    ",
  0x40082128u64 => "
      SLCDC.seg()[40],
    ",
  0x40082129u64 => "
      SLCDC.seg()[41],
    ",
  0x4008212au64 => "
      SLCDC.seg()[42],
    ",
  0x4008212bu64 => "
      SLCDC.seg()[43],
    ",
  0x4008212cu64 => "
      SLCDC.seg()[44],
    ",
  0x40084000u64 => "
      AGTW_0.agt(),
    ",
  0x40084004u64 => "
      AGTW_0.agtcma(),
    ",
  0x40084008u64 => "
      AGTW_0.agtcmb(),
    ",
  0x4008400cu64 => "
      AGTW_0.agtcr(),
    ",
  0x4008400du64 => "
      AGTW_0.agtmr1(),
    ",
  0x4008400eu64 => "
      AGTW_0.agtmr2(),
    ",
  0x4008400fu64 => "
      AGTW_0.agtiosel(),
    ",
  0x40084010u64 => "
      AGTW_0.agtioc(),
    ",
  0x40084011u64 => "
      AGTW_0.agtisr(),
    ",
  0x40084012u64 => "
      AGTW_0.agtcmsr(),
    ",
  0x40084200u64 => "
      AGT_0.agt(),
    ",
  0x40084202u64 => "
      AGT_0.agtcma(),
    ",
  0x40084204u64 => "
      AGT_0.agtcmb(),
    ",
  0x40084208u64 => "
      AGT_0.agtcr(),
    ",
  0x40084209u64 => "
      AGT_0.agtmr1(),
    ",
  0x4008420au64 => "
      AGT_0.agtmr2(),
    ",
  0x4008420cu64 => "
      AGT_0.agtioc(),
    ",
  0x4008420du64 => "
      AGT_0.agtisr(),
    ",
  0x4008420eu64 => "
      AGT_0.agtcmsr(),
    ",
  0x4008420fu64 => "
      AGT_0.agtiosel(),
    ",
  0x4009c000u64 => "
      SDADC_24_B.sdadccr(),
    ",
  0x4009c004u64 => "
      SDADC_24_B.sdadmr(),
    ",
  0x4009c00cu64 => "
      SDADC_24_B.sdadrr(),
    ",
  0x4009c010u64 => "
      SDADC_24_B.sdadgcr(),
    ",
  0x4009c018u64 => "
      SDADC_24_B.sdadhpfcr(),
    ",
  0x4009c020u64 => "
      SDADC_24_B.sdadicr(),
    ",
  0x4009c024u64 => "
      SDADC_24_B.sdadiclr(),
    ",
  0x4009c028u64 => "
      SDADC_24_B.sdadisr(),
    ",
  0x4009c040u64 => "
      SDADC_24_B.sdadphcr()[0],
    ",
  0x4009c044u64 => "
      SDADC_24_B.sdadphcr()[1],
    ",
  0x4009c048u64 => "
      SDADC_24_B.sdadphcr()[2],
    ",
  0x4009c04cu64 => "
      SDADC_24_B.sdadphcr()[3],
    ",
  0x4009c050u64 => "
      SDADC_24_B.sdadphcr()[4],
    ",
  0x4009c054u64 => "
      SDADC_24_B.sdadphcr()[5],
    ",
  0x4009c058u64 => "
      SDADC_24_B.sdadphcr()[6],
    ",
  0x4009c05cu64 => "
      SDADC_24_B.sdadphcr()[7],
    ",
  0x4009c080u64 => "
      SDADC_24_B.sdadcr()[0],
    ",
  0x4009c084u64 => "
      SDADC_24_B.sdadcr()[1],
    ",
  0x4009c088u64 => "
      SDADC_24_B.sdadcr()[2],
    ",
  0x4009c08cu64 => "
      SDADC_24_B.sdadcr()[3],
    ",
  0x4009c090u64 => "
      SDADC_24_B.sdadcr()[4],
    ",
  0x4009c094u64 => "
      SDADC_24_B.sdadcr()[5],
    ",
  0x4009c098u64 => "
      SDADC_24_B.sdadcr()[6],
    ",
  0x4009c0a0u64 => "
      SDADC_24_B.sdadcrt2()[0],
    ",
  0x4009c0a4u64 => "
      SDADC_24_B.sdadcrt2()[1],
    ",
  0x4009c0a8u64 => "
      SDADC_24_B.sdadcrt2()[2],
    ",
  0x4009c0acu64 => "
      SDADC_24_B.sdadcrt2()[3],
    ",
  0x4009c0c0u64 => "
      SDADC_24_B.sdadcrlpf()[0],
    ",
  0x4009c0c4u64 => "
      SDADC_24_B.sdadcrlpf()[1],
    ",
  0x4009c0c8u64 => "
      SDADC_24_B.sdadcrlpf()[2],
    ",
  0x4009c0ccu64 => "
      SDADC_24_B.sdadcrlpf()[3],
    ",
  0x4009c0d0u64 => "
      SDADC_24_B.sdadcrlpf()[4],
    ",
  0x4009c0d4u64 => "
      SDADC_24_B.sdadcrlpf()[5],
    ",
  0x4009c0d8u64 => "
      SDADC_24_B.sdadcrlpf()[6],
    ",
  0x4009c0e0u64 => "
      SDADC_24_B.sdadcrlpft2()[0],
    ",
  0x4009c0e4u64 => "
      SDADC_24_B.sdadcrlpft2()[1],
    ",
  0x4009c0e8u64 => "
      SDADC_24_B.sdadcrlpft2()[2],
    ",
  0x4009c0ecu64 => "
      SDADC_24_B.sdadcrlpft2()[3],
    ",
  0x400a0000u64 => "
      MACL.mulb()[0],
    ",
  0x400a0008u64 => "
      MACL.mulb()[1],
    ",
  0x400a0010u64 => "
      MACL.mulb()[2],
    ",
  0x400a0018u64 => "
      MACL.mulb()[3],
    ",
  0x400a0020u64 => "
      MACL.mulb()[4],
    ",
  0x400a0028u64 => "
      MACL.mulb()[5],
    ",
  0x400a0030u64 => "
      MACL.mulb()[6],
    ",
  0x400a0038u64 => "
      MACL.mulb()[7],
    ",
  0x400a0040u64 => "
      MACL.mulb()[8],
    ",
  0x400a0048u64 => "
      MACL.mulb()[9],
    ",
  0x400a0050u64 => "
      MACL.mulb()[10],
    ",
  0x400a0058u64 => "
      MACL.mulb()[11],
    ",
  0x400a0060u64 => "
      MACL.mulb()[12],
    ",
  0x400a0068u64 => "
      MACL.mulb()[13],
    ",
  0x400a0070u64 => "
      MACL.mulb()[14],
    ",
  0x400a0078u64 => "
      MACL.mulb()[15],
    ",
  0x400a0080u64 => "
      MACL.mulb()[16],
    ",
  0x400a0088u64 => "
      MACL.mulb()[17],
    ",
  0x400a0090u64 => "
      MACL.mulb()[18],
    ",
  0x400a0098u64 => "
      MACL.mulb()[19],
    ",
  0x400a00a0u64 => "
      MACL.mulb()[20],
    ",
  0x400a00a8u64 => "
      MACL.mulb()[21],
    ",
  0x400a00b0u64 => "
      MACL.mulb()[22],
    ",
  0x400a00b8u64 => "
      MACL.mulb()[23],
    ",
  0x400a00c0u64 => "
      MACL.mulrl()[0],
    ",
  0x400a00c8u64 => "
      MACL.mulrl()[1],
    ",
  0x400a00d0u64 => "
      MACL.mulrl()[2],
    ",
  0x400a00d8u64 => "
      MACL.mulrl()[3],
    ",
  0x400a00e0u64 => "
      MACL.mulrl()[4],
    ",
  0x400a00e8u64 => "
      MACL.mulrl()[5],
    ",
  0x400a00f0u64 => "
      MACL.mulrl()[6],
    ",
  0x400a00f8u64 => "
      MACL.mulrl()[7],
    ",
  0x400a0100u64 => "
      MACL.mulrl()[8],
    ",
  0x400a0108u64 => "
      MACL.mulrl()[9],
    ",
  0x400a0110u64 => "
      MACL.mulrl()[10],
    ",
  0x400a0118u64 => "
      MACL.mulrl()[11],
    ",
  0x400a0120u64 => "
      MACL.mulrl()[12],
    ",
  0x400a0128u64 => "
      MACL.mulrl()[13],
    ",
  0x400a0130u64 => "
      MACL.mulrl()[14],
    ",
  0x400a0138u64 => "
      MACL.mulrl()[15],
    ",
  0x400a0140u64 => "
      MACL.mulrl()[16],
    ",
  0x400a0148u64 => "
      MACL.mulrl()[17],
    ",
  0x400a0150u64 => "
      MACL.mulrl()[18],
    ",
  0x400a0158u64 => "
      MACL.mulrl()[19],
    ",
  0x400a0160u64 => "
      MACL.mulrl()[20],
    ",
  0x400a0168u64 => "
      MACL.mulrl()[21],
    ",
  0x400a0170u64 => "
      MACL.mulrl()[22],
    ",
  0x400a0178u64 => "
      MACL.mulrl()[23],
    ",
  0x400a00c4u64 => "
      MACL.mulrh()[0],
    ",
  0x400a00ccu64 => "
      MACL.mulrh()[1],
    ",
  0x400a00d4u64 => "
      MACL.mulrh()[2],
    ",
  0x400a00dcu64 => "
      MACL.mulrh()[3],
    ",
  0x400a00e4u64 => "
      MACL.mulrh()[4],
    ",
  0x400a00ecu64 => "
      MACL.mulrh()[5],
    ",
  0x400a00f4u64 => "
      MACL.mulrh()[6],
    ",
  0x400a00fcu64 => "
      MACL.mulrh()[7],
    ",
  0x400a0104u64 => "
      MACL.mulrh()[8],
    ",
  0x400a010cu64 => "
      MACL.mulrh()[9],
    ",
  0x400a0114u64 => "
      MACL.mulrh()[10],
    ",
  0x400a011cu64 => "
      MACL.mulrh()[11],
    ",
  0x400a0124u64 => "
      MACL.mulrh()[12],
    ",
  0x400a012cu64 => "
      MACL.mulrh()[13],
    ",
  0x400a0134u64 => "
      MACL.mulrh()[14],
    ",
  0x400a013cu64 => "
      MACL.mulrh()[15],
    ",
  0x400a0144u64 => "
      MACL.mulrh()[16],
    ",
  0x400a014cu64 => "
      MACL.mulrh()[17],
    ",
  0x400a0154u64 => "
      MACL.mulrh()[18],
    ",
  0x400a015cu64 => "
      MACL.mulrh()[19],
    ",
  0x400a0164u64 => "
      MACL.mulrh()[20],
    ",
  0x400a016cu64 => "
      MACL.mulrh()[21],
    ",
  0x400a0174u64 => "
      MACL.mulrh()[22],
    ",
  0x400a017cu64 => "
      MACL.mulrh()[23],
    ",
  0x400a0180u64 => "
      MACL.mul32u(),
    ",
  0x400a0184u64 => "
      MACL.mul32s(),
    ",
  0x400a0188u64 => "
      MACL.mac32u(),
    ",
  0x400a018cu64 => "
      MACL.mac32s(),
    ",
  0x400a0190u64 => "
      MACL.mulc(),
    ",
  0x400a0194u64 => "
      MACL.mulrclr(),
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
  0x407ec370u64 => "
      FLCN.fctlfr(),
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
  0x407effd0u64 => "
      FLCN.fbkpgcr(),
    ",
  0x407effd4u64 => "
      FLCN.fbkswcr(),
    ",
};
