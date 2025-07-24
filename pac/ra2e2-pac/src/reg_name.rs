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
// Generated from SVD 1.40.00, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:46:37 +0000

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
  0x4001e0a5u64 => "
      SYSC.hocowtcr(),
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
  0x40040838u64 => "
      PFS.p0pfs()[0],
    ",
  0x4004083cu64 => "
      PFS.p0pfs()[1],
    ",
  0x4004083au64 => "
      PFS.p0pfs_ha()[0],
    ",
  0x4004083eu64 => "
      PFS.p0pfs_ha()[1],
    ",
  0x4004083bu64 => "
      PFS.p0pfs_by()[0],
    ",
  0x4004083fu64 => "
      PFS.p0pfs_by()[1],
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
  0x4004086au64 => "
      PFS.p1pfs_ha()[0],
    ",
  0x4004086eu64 => "
      PFS.p1pfs_ha()[1],
    ",
  0x40040872u64 => "
      PFS.p1pfs_ha()[2],
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
  0x40040894u64 => "
      PFS.p205pfs(),
    ",
  0x40040896u64 => "
      PFS.p205pfs_ha(),
    ",
  0x40040897u64 => "
      PFS.p205pfs_by(),
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
  0x40040900u64 => "
      PFS.p40pfs()[0],
    ",
  0x40040904u64 => "
      PFS.p40pfs()[1],
    ",
  0x40040902u64 => "
      PFS.p40pfs_ha()[0],
    ",
  0x40040906u64 => "
      PFS.p40pfs_ha()[1],
    ",
  0x40040903u64 => "
      PFS.p40pfs_by()[0],
    ",
  0x40040907u64 => "
      PFS.p40pfs_by()[1],
    ",
  0x40040a78u64 => "
      PFS.p914pfs(),
    ",
  0x40040a7au64 => "
      PFS.p914pfs_ha(),
    ",
  0x40040a7bu64 => "
      PFS.p914pfs_by(),
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
  0x4005c046u64 => "
      ADC_120.addr()[0],
    ",
  0x4005c048u64 => "
      ADC_120.addr()[1],
    ",
  0x4005c04au64 => "
      ADC_120.addr()[2],
    ",
  0x4005c04cu64 => "
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
  0x4005c0e9u64 => "
      ADC_120.adsstr()[0],
    ",
  0x4005c0eau64 => "
      ADC_120.adsstr()[1],
    ",
  0x40070120u64 => "
      SCI_9.smr(),
      SCI_9.smr_smci(),
    ",
  0x40070121u64 => "
      SCI_9.brr(),
    ",
  0x40070122u64 => "
      SCI_9.scr(),
      SCI_9.scr_smci(),
    ",
  0x40070123u64 => "
      SCI_9.tdr(),
    ",
  0x40070124u64 => "
      SCI_9.ssr(),
      SCI_9.ssr_smci(),
    ",
  0x40070125u64 => "
      SCI_9.rdr(),
    ",
  0x40070126u64 => "
      SCI_9.scmr(),
    ",
  0x40070127u64 => "
      SCI_9.semr(),
    ",
  0x40070128u64 => "
      SCI_9.snfr(),
    ",
  0x40070129u64 => "
      SCI_9.simr1(),
    ",
  0x4007012au64 => "
      SCI_9.simr2(),
    ",
  0x4007012bu64 => "
      SCI_9.simr3(),
    ",
  0x4007012cu64 => "
      SCI_9.sisr(),
    ",
  0x4007012du64 => "
      SCI_9.spmr(),
    ",
  0x4007012eu64 => "
      SCI_9.tdrhl(),
    ",
  0x40070130u64 => "
      SCI_9.rdrhl(),
    ",
  0x40070132u64 => "
      SCI_9.mddr(),
    ",
  0x40070133u64 => "
      SCI_9.dccr(),
    ",
  0x4007013au64 => "
      SCI_9.cdr(),
    ",
  0x4007013cu64 => "
      SCI_9.sptr(),
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
  0x40080000u64 => "
      KINT.krctl(),
    ",
  0x40080004u64 => "
      KINT.krf(),
    ",
  0x40080008u64 => "
      KINT.krm(),
    ",
  0x40083000u64 => "
      I_3_C.prts(),
    ",
  0x40083014u64 => "
      I_3_C.bctl(),
    ",
  0x40083018u64 => "
      I_3_C.msdvad(),
    ",
  0x40083020u64 => "
      I_3_C.rstctl(),
    ",
  0x40083024u64 => "
      I_3_C.prsst(),
    ",
  0x40083030u64 => "
      I_3_C.inst(),
    ",
  0x40083034u64 => "
      I_3_C.inste(),
    ",
  0x40083038u64 => "
      I_3_C.inie(),
    ",
  0x4008303cu64 => "
      I_3_C.instfc(),
    ",
  0x40083044u64 => "
      I_3_C.dvct(),
    ",
  0x40083058u64 => "
      I_3_C.ibinctl(),
    ",
  0x40083060u64 => "
      I_3_C.bfctl(),
    ",
  0x40083064u64 => "
      I_3_C.svctl(),
    ",
  0x40083070u64 => "
      I_3_C.refckctl(),
    ",
  0x40083074u64 => "
      I_3_C.stdbr(),
    ",
  0x40083078u64 => "
      I_3_C.extbr(),
    ",
  0x4008307cu64 => "
      I_3_C.bfrecdt(),
    ",
  0x40083080u64 => "
      I_3_C.bavlcdt(),
    ",
  0x40083084u64 => "
      I_3_C.bidlcdt(),
    ",
  0x40083088u64 => "
      I_3_C.outctl(),
    ",
  0x4008308cu64 => "
      I_3_C.inctl(),
    ",
  0x40083090u64 => "
      I_3_C.tmoctl(),
    ",
  0x400830a0u64 => "
      I_3_C.ackctl(),
    ",
  0x400830a4u64 => "
      I_3_C.scstrctl(),
    ",
  0x400830b0u64 => "
      I_3_C.scstlctl(),
    ",
  0x400830c0u64 => "
      I_3_C.svtdlg0(),
    ",
  0x40083140u64 => "
      I_3_C.cndctl(),
    ",
  0x40083150u64 => "
      I_3_C.ncmdqp(),
    ",
  0x40083154u64 => "
      I_3_C.nrspqp(),
    ",
  0x40083158u64 => "
      I_3_C.ntdtbp0(),
      I_3_C.ntdtbp0_by(),
    ",
  0x4008317cu64 => "
      I_3_C.nibiqp(),
    ",
  0x40083180u64 => "
      I_3_C.nrsqp(),
    ",
  0x40083190u64 => "
      I_3_C.nqthctl(),
    ",
  0x40083194u64 => "
      I_3_C.ntbthctl0(),
    ",
  0x400831c0u64 => "
      I_3_C.nrqthctl(),
    ",
  0x400831d0u64 => "
      I_3_C.bst(),
    ",
  0x400831d4u64 => "
      I_3_C.bste(),
    ",
  0x400831d8u64 => "
      I_3_C.bie(),
    ",
  0x400831dcu64 => "
      I_3_C.bstfc(),
    ",
  0x400831e0u64 => "
      I_3_C.ntst(),
    ",
  0x400831e4u64 => "
      I_3_C.ntste(),
    ",
  0x400831e8u64 => "
      I_3_C.ntie(),
    ",
  0x400831ecu64 => "
      I_3_C.ntstfc(),
    ",
  0x40083210u64 => "
      I_3_C.bcst(),
    ",
  0x40083214u64 => "
      I_3_C.svst(),
    ",
  0x40083224u64 => "
      I_3_C.datbas()[0],
    ",
  0x4008322cu64 => "
      I_3_C.datbas()[1],
    ",
  0x40083234u64 => "
      I_3_C.datbas()[2],
    ",
  0x4008323cu64 => "
      I_3_C.datbas()[3],
    ",
  0x400832a0u64 => "
      I_3_C.exdatbas(),
    ",
  0x400832b0u64 => "
      I_3_C.sdatbas0(),
    ",
  0x400832d0u64 => "
      I_3_C.msdct()[0],
    ",
  0x400832d4u64 => "
      I_3_C.msdct()[1],
    ",
  0x400832d8u64 => "
      I_3_C.msdct()[2],
    ",
  0x400832dcu64 => "
      I_3_C.msdct()[3],
    ",
  0x40083320u64 => "
      I_3_C.svdct(),
    ",
  0x40083324u64 => "
      I_3_C.sdctpidl(),
    ",
  0x40083328u64 => "
      I_3_C.sdctpidh(),
    ",
  0x40083330u64 => "
      I_3_C.svdvad0(),
    ",
  0x40083350u64 => "
      I_3_C.csecmd(),
    ",
  0x40083354u64 => "
      I_3_C.ceactst(),
    ",
  0x40083358u64 => "
      I_3_C.cmwlg(),
    ",
  0x4008335cu64 => "
      I_3_C.cmrlg(),
    ",
  0x40083360u64 => "
      I_3_C.cetstmd(),
    ",
  0x40083364u64 => "
      I_3_C.cgdvst(),
    ",
  0x40083368u64 => "
      I_3_C.cmdspw(),
    ",
  0x4008336cu64 => "
      I_3_C.cmdspr(),
    ",
  0x40083370u64 => "
      I_3_C.cmdspt(),
    ",
  0x40083374u64 => "
      I_3_C.cetsm(),
    ",
  0x40083380u64 => "
      I_3_C.bitcnt(),
    ",
  0x40083394u64 => "
      I_3_C.nqstlv(),
    ",
  0x40083398u64 => "
      I_3_C.ndbstlv0(),
    ",
  0x400833c0u64 => "
      I_3_C.nrsqstlv(),
    ",
  0x400833ccu64 => "
      I_3_C.prstdbg(),
    ",
  0x400833d0u64 => "
      I_3_C.mserrcnt(),
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
