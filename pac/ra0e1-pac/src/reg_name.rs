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
// Generated from SVD 1.10.00, with svd2pac 0.6.1 on Sun, 15 Mar 2026 07:00:13 +0000

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
  0x40006300u64 => "
      ICU.dtcenst0(),
    ",
  0x40006304u64 => "
      ICU.dtcenst1(),
    ",
  0x40006310u64 => "
      ICU.dtcenset0(),
    ",
  0x40006314u64 => "
      ICU.dtcenset1(),
    ",
  0x40006320u64 => "
      ICU.dtcenclr0(),
    ",
  0x40006324u64 => "
      ICU.dtcenclr1(),
    ",
  0x40006330u64 => "
      ICU.intflag0(),
    ",
  0x40006334u64 => "
      ICU.intflag1(),
    ",
  0x40006340u64 => "
      ICU.sbyedcr0(),
    ",
  0x40006344u64 => "
      ICU.sbyedcr1(),
    ",
  0x4001b000u64 => "
      DBG.dbgstr(),
    ",
  0x4001b010u64 => "
      DBG.dbgstopcr(),
    ",
  0x4001e800u64 => "
      SYSC.cmc(),
    ",
  0x4001e803u64 => "
      SYSC.somrg(),
    ",
  0x4001e804u64 => "
      SYSC.miotrm(),
    ",
  0x4001e805u64 => "
      SYSC.liotrm(),
    ",
  0x4001e808u64 => "
      SYSC.hococr(),
    ",
  0x4001e809u64 => "
      SYSC.mococr(),
    ",
  0x4001e80au64 => "
      SYSC.lococr(),
    ",
  0x4001e80bu64 => "
      SYSC.mosccr(),
    ",
  0x4001e80cu64 => "
      SYSC.sosccr(),
    ",
  0x4001e810u64 => "
      SYSC.ostc(),
    ",
  0x4001e811u64 => "
      SYSC.osts(),
    ",
  0x4001e812u64 => "
      SYSC.oscsf(),
    ",
  0x4001e818u64 => "
      SYSC.hocodiv(),
    ",
  0x4001e819u64 => "
      SYSC.mocodiv(),
    ",
  0x4001e81au64 => "
      SYSC.moscdiv(),
    ",
  0x4001e820u64 => "
      SYSC.focoscr(),
    ",
  0x4001e821u64 => "
      SYSC.fmainscr(),
    ",
  0x4001e822u64 => "
      SYSC.fsubscr(),
    ",
  0x4001e823u64 => "
      SYSC.iclkscr(),
    ",
  0x4001e824u64 => "
      SYSC.osmc(),
    ",
  0x4001e830u64 => "
      SYSC.resf(),
    ",
  0x4001e831u64 => "
      SYSC.porsr(),
    ",
  0x4001e840u64 => "
      SYSC.lvd1cr(),
    ",
  0x4001e841u64 => "
      SYSC.lvd1mkr(),
    ",
  0x4001e843u64 => "
      SYSC.lvd1sr(),
    ",
  0x4001e860u64 => "
      SYSC.sbycr(),
    ",
  0x4001e862u64 => "
      SYSC.psmcr(),
    ",
  0x4001e863u64 => "
      SYSC.syocdcr(),
    ",
  0x4001e8feu64 => "
      SYSC.prcr(),
    ",
  0x4001ec02u64 => "
      SYSC.mstpcra(),
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
  0x4004101cu64 => "
      ELC.elsr()[0],
    ",
  0x40041020u64 => "
      ELC.elsr()[1],
    ",
  0x40041024u64 => "
      ELC.elsr()[2],
    ",
  0x40041028u64 => "
      ELC.elsr()[3],
    ",
  0x4004102cu64 => "
      ELC.elsr()[4],
    ",
  0x40041030u64 => "
      ELC.elsr()[5],
    ",
  0x40044400u64 => "
      IWDT.iwdtrr(),
    ",
  0x40044404u64 => "
      IWDT.iwdtsr(),
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
  0x40074000u64 => "
      CRC.crccr0(),
    ",
  0x40074004u64 => "
      CRC.crcdir(),
      CRC.crcdir_by(),
    ",
  0x40074008u64 => "
      CRC.crcdor(),
      CRC.crcdor_ha(),
    ",
  0x400a0000u64 => "
      PORT_0.podr0(),
    ",
  0x400a0002u64 => "
      PORT_0.pdr0(),
    ",
  0x400a0006u64 => "
      PORT_0.pidr0(),
    ",
  0x400a0008u64 => "
      PORT_0.porr0(),
    ",
  0x400a000au64 => "
      PORT_0.posr0(),
    ",
  0x400a0020u64 => "
      PORT_1.podr1(),
    ",
  0x400a0022u64 => "
      PORT_1.pdr1(),
    ",
  0x400a0026u64 => "
      PORT_1.pidr1(),
    ",
  0x400a0028u64 => "
      PORT_1.porr1(),
    ",
  0x400a002au64 => "
      PORT_1.posr1(),
    ",
  0x400a002cu64 => "
      PORT_1.eorr1(),
    ",
  0x400a002eu64 => "
      PORT_1.eosr1(),
    ",
  0x400a0040u64 => "
      PORT_2.podr2(),
    ",
  0x400a0042u64 => "
      PORT_2.pdr2(),
    ",
  0x400a0046u64 => "
      PORT_2.pidr2(),
    ",
  0x400a0048u64 => "
      PORT_2.porr2(),
    ",
  0x400a004au64 => "
      PORT_2.posr2(),
    ",
  0x400a004cu64 => "
      PORT_2.eorr2(),
    ",
  0x400a004eu64 => "
      PORT_2.eosr2(),
    ",
  0x400a0060u64 => "
      PORT_3.podr3(),
    ",
  0x400a0062u64 => "
      PORT_3.pdr3(),
    ",
  0x400a0066u64 => "
      PORT_3.pidr3(),
    ",
  0x400a0068u64 => "
      PORT_3.porr3(),
    ",
  0x400a006au64 => "
      PORT_3.posr3(),
    ",
  0x400a0080u64 => "
      PORT_4.podr4(),
    ",
  0x400a0082u64 => "
      PORT_4.pdr4(),
    ",
  0x400a0086u64 => "
      PORT_4.pidr4(),
    ",
  0x400a0088u64 => "
      PORT_4.porr4(),
    ",
  0x400a008au64 => "
      PORT_4.posr4(),
    ",
  0x400a0120u64 => "
      PORT_9.podr9(),
    ",
  0x400a0122u64 => "
      PORT_9.pdr9(),
    ",
  0x400a0126u64 => "
      PORT_9.pidr9(),
    ",
  0x400a0128u64 => "
      PORT_9.porr9(),
    ",
  0x400a012au64 => "
      PORT_9.posr9(),
    ",
  0x400a0210u64 => "
      PFS_A.p00pfs_a()[0],
    ",
  0x400a0212u64 => "
      PFS_A.p00pfs_a()[1],
    ",
  0x400a0214u64 => "
      PFS_A.p0pfs_a()[0],
    ",
  0x400a0216u64 => "
      PFS_A.p0pfs_a()[1],
    ",
  0x400a0218u64 => "
      PFS_A.p0pfs_a()[2],
    ",
  0x400a021au64 => "
      PFS_A.p0pfs_a()[3],
    ",
  0x400a021cu64 => "
      PFS_A.p0pfs_a()[4],
    ",
  0x400a021eu64 => "
      PFS_A.p015pfs_a(),
    ",
  0x400a0224u64 => "
      PFS_A.p10pfs_a()[0],
    ",
  0x400a0226u64 => "
      PFS_A.p10pfs_a()[1],
    ",
  0x400a0230u64 => "
      PFS_A.p108pfs_a(),
    ",
  0x400a0232u64 => "
      PFS_A.p109pfs_a(),
    ",
  0x400a0234u64 => "
      PFS_A.p110pfs_a(),
    ",
  0x400a0238u64 => "
      PFS_A.p112pfs_a(),
    ",
  0x400a0240u64 => "
      PFS_A.p200pfs_a(),
    ",
  0x400a0242u64 => "
      PFS_A.p201pfs_a(),
    ",
  0x400a024cu64 => "
      PFS_A.p206pfs_a(),
    ",
  0x400a024eu64 => "
      PFS_A.p20pfs_a()[0],
    ",
  0x400a0250u64 => "
      PFS_A.p20pfs_a()[1],
    ",
  0x400a025cu64 => "
      PFS_A.p2pfs_a()[0],
    ",
  0x400a025eu64 => "
      PFS_A.p2pfs_a()[1],
    ",
  0x400a0260u64 => "
      PFS_A.p300pfs_a(),
    ",
  0x400a028eu64 => "
      PFS_A.p407pfs_a(),
    ",
  0x400a033au64 => "
      PFS_A.p9pfs_a()[0],
    ",
  0x400a033cu64 => "
      PFS_A.p9pfs_a()[1],
    ",
  0x400a0340u64 => "
      PFS_A.pwpr(),
    ",
  0x400a1000u64 => "
      PORGA.snfen(),
    ",
  0x400a1001u64 => "
      PORGA.tnfen(),
    ",
  0x400a1003u64 => "
      PORGA.isc(),
    ",
  0x400a1004u64 => "
      PORGA.tis0(),
    ",
  0x400a1005u64 => "
      PORGA.tis1(),
    ",
  0x400a1009u64 => "
      PORGA.ulbs(),
    ",
  0x400a1800u64 => "
      ADC_D.adm0(),
    ",
  0x400a1801u64 => "
      ADC_D.ads(),
    ",
  0x400a1802u64 => "
      ADC_D.adm1(),
    ",
  0x400a1806u64 => "
      ADC_D.adcr(),
    ",
  0x400a1807u64 => "
      ADC_D.adcrh(),
    ",
  0x400a1910u64 => "
      ADC_D.adm2(),
    ",
  0x400a1911u64 => "
      ADC_D.adul(),
    ",
  0x400a1912u64 => "
      ADC_D.adll(),
    ",
  0x400a1913u64 => "
      ADC_D.adtes(),
    ",
  0x400a1920u64 => "
      ADC_D.adcr0(),
    ",
  0x400a1921u64 => "
      ADC_D.adcr0h(),
    ",
  0x400a1922u64 => "
      ADC_D.adcr1(),
    ",
  0x400a1923u64 => "
      ADC_D.adcr1h(),
    ",
  0x400a1924u64 => "
      ADC_D.adcr2(),
    ",
  0x400a1925u64 => "
      ADC_D.adcr2h(),
    ",
  0x400a1926u64 => "
      ADC_D.adcr3(),
    ",
  0x400a1927u64 => "
      ADC_D.adcr3h(),
    ",
  0x400a2000u64 => "
      SAU_0.sdr0()[0],
    ",
  0x400a2002u64 => "
      SAU_0.sdr0()[1],
    ",
  0x400a2004u64 => "
      SAU_0.sdr0()[2],
    ",
  0x400a2006u64 => "
      SAU_0.sdr0()[3],
    ",
  0x400a2100u64 => "
      SAU_0.ssr00(),
    ",
  0x400a2102u64 => "
      SAU_0.ssr01(),
    ",
  0x400a2104u64 => "
      SAU_0.ssr02(),
    ",
  0x400a2106u64 => "
      SAU_0.ssr03(),
    ",
  0x400a2108u64 => "
      SAU_0.sir00(),
    ",
  0x400a210au64 => "
      SAU_0.sir01(),
    ",
  0x400a210cu64 => "
      SAU_0.sir02(),
    ",
  0x400a210eu64 => "
      SAU_0.sir03(),
    ",
  0x400a2110u64 => "
      SAU_0.smr00(),
    ",
  0x400a2112u64 => "
      SAU_0.smr01(),
    ",
  0x400a2114u64 => "
      SAU_0.smr02(),
    ",
  0x400a2116u64 => "
      SAU_0.smr03(),
    ",
  0x400a2118u64 => "
      SAU_0.scr00(),
    ",
  0x400a211au64 => "
      SAU_0.scr01(),
    ",
  0x400a211cu64 => "
      SAU_0.scr02(),
    ",
  0x400a211eu64 => "
      SAU_0.scr03(),
    ",
  0x400a2120u64 => "
      SAU_0.se0(),
    ",
  0x400a2122u64 => "
      SAU_0.ss0(),
    ",
  0x400a2124u64 => "
      SAU_0.st0(),
    ",
  0x400a2126u64 => "
      SAU_0.sps0(),
    ",
  0x400a2128u64 => "
      SAU_0.so0(),
    ",
  0x400a212au64 => "
      SAU_0.soe0(),
    ",
  0x400a2134u64 => "
      SAU_0.sol0(),
    ",
  0x400a2138u64 => "
      SAU_0.ssc0(),
    ",
  0x400a2200u64 => "
      SAU_1.sdr1()[0],
    ",
  0x400a2202u64 => "
      SAU_1.sdr1()[1],
    ",
  0x400a2300u64 => "
      SAU_1.ssr10(),
    ",
  0x400a2302u64 => "
      SAU_1.ssr11(),
    ",
  0x400a2308u64 => "
      SAU_1.sir10(),
    ",
  0x400a230au64 => "
      SAU_1.sir11(),
    ",
  0x400a2310u64 => "
      SAU_1.smr10(),
    ",
  0x400a2312u64 => "
      SAU_1.smr11(),
    ",
  0x400a2318u64 => "
      SAU_1.scr10(),
    ",
  0x400a231au64 => "
      SAU_1.scr11(),
    ",
  0x400a2320u64 => "
      SAU_1.se1(),
    ",
  0x400a2322u64 => "
      SAU_1.ss1(),
    ",
  0x400a2324u64 => "
      SAU_1.st1(),
    ",
  0x400a2326u64 => "
      SAU_1.sps1(),
    ",
  0x400a2328u64 => "
      SAU_1.so1(),
    ",
  0x400a232au64 => "
      SAU_1.soe1(),
    ",
  0x400a2334u64 => "
      SAU_1.sol1(),
    ",
  0x400a2600u64 => "
      TAU.tdr00(),
    ",
  0x400a2602u64 => "
      TAU.tdr01(),
      TAU.tdr01l(),
    ",
  0x400a2603u64 => "
      TAU.tdr01h(),
    ",
  0x400a2604u64 => "
      TAU.tdr02(),
    ",
  0x400a2606u64 => "
      TAU.tdr03(),
      TAU.tdr03l(),
    ",
  0x400a2607u64 => "
      TAU.tdr03h(),
    ",
  0x400a2608u64 => "
      TAU.tdr04(),
    ",
  0x400a260au64 => "
      TAU.tdr05(),
    ",
  0x400a260cu64 => "
      TAU.tdr06(),
    ",
  0x400a260eu64 => "
      TAU.tdr07(),
    ",
  0x400a2700u64 => "
      TAU.tcr0()[0],
    ",
  0x400a2702u64 => "
      TAU.tcr0()[1],
    ",
  0x400a2704u64 => "
      TAU.tcr0()[2],
    ",
  0x400a2706u64 => "
      TAU.tcr0()[3],
    ",
  0x400a2708u64 => "
      TAU.tcr0()[4],
    ",
  0x400a270au64 => "
      TAU.tcr0()[5],
    ",
  0x400a270cu64 => "
      TAU.tcr0()[6],
    ",
  0x400a270eu64 => "
      TAU.tcr0()[7],
    ",
  0x400a2710u64 => "
      TAU.tmr00(),
    ",
  0x400a2712u64 => "
      TAU.tmr01(),
    ",
  0x400a2714u64 => "
      TAU.tmr02(),
    ",
  0x400a2716u64 => "
      TAU.tmr03(),
    ",
  0x400a2718u64 => "
      TAU.tmr04(),
    ",
  0x400a271au64 => "
      TAU.tmr05(),
    ",
  0x400a271cu64 => "
      TAU.tmr06(),
    ",
  0x400a271eu64 => "
      TAU.tmr07(),
    ",
  0x400a2720u64 => "
      TAU.tsr0()[0],
    ",
  0x400a2722u64 => "
      TAU.tsr0()[1],
    ",
  0x400a2724u64 => "
      TAU.tsr0()[2],
    ",
  0x400a2726u64 => "
      TAU.tsr0()[3],
    ",
  0x400a2728u64 => "
      TAU.tsr0()[4],
    ",
  0x400a272au64 => "
      TAU.tsr0()[5],
    ",
  0x400a272cu64 => "
      TAU.tsr0()[6],
    ",
  0x400a272eu64 => "
      TAU.tsr0()[7],
    ",
  0x400a2730u64 => "
      TAU.te0(),
    ",
  0x400a2732u64 => "
      TAU.ts0(),
    ",
  0x400a2734u64 => "
      TAU.tt0(),
    ",
  0x400a2736u64 => "
      TAU.tps0(),
    ",
  0x400a2738u64 => "
      TAU.to0(),
    ",
  0x400a273au64 => "
      TAU.toe0(),
    ",
  0x400a273cu64 => "
      TAU.tol0(),
    ",
  0x400a273eu64 => "
      TAU.tom0(),
    ",
  0x400a2c00u64 => "
      RTC_C.sec(),
    ",
  0x400a2c01u64 => "
      RTC_C.min(),
    ",
  0x400a2c02u64 => "
      RTC_C.hour(),
    ",
  0x400a2c03u64 => "
      RTC_C.week(),
    ",
  0x400a2c04u64 => "
      RTC_C.day(),
    ",
  0x400a2c05u64 => "
      RTC_C.month(),
    ",
  0x400a2c06u64 => "
      RTC_C.year(),
    ",
  0x400a2c07u64 => "
      RTC_C.subcud(),
    ",
  0x400a2c08u64 => "
      RTC_C.alarmwm(),
    ",
  0x400a2c09u64 => "
      RTC_C.alarmwh(),
    ",
  0x400a2c0au64 => "
      RTC_C.alarmww(),
    ",
  0x400a2c0bu64 => "
      RTC_C.rtcc0(),
    ",
  0x400a2c0cu64 => "
      RTC_C.rtcc1(),
    ",
  0x400a3000u64 => "
      IICA.iica0(),
    ",
  0x400a3001u64 => "
      IICA.iics0(),
    ",
  0x400a3002u64 => "
      IICA.iicf0(),
    ",
  0x400a3100u64 => "
      IICA.iicctl00(),
    ",
  0x400a3101u64 => "
      IICA.iicctl01(),
    ",
  0x400a3102u64 => "
      IICA.iicwl0(),
    ",
  0x400a3103u64 => "
      IICA.iicwh0(),
    ",
  0x400a3104u64 => "
      IICA.sva0(),
    ",
  0x400a3400u64 => "
      UARTA.txba0(),
    ",
  0x400a3401u64 => "
      UARTA.rxba0(),
    ",
  0x400a3402u64 => "
      UARTA.asima00(),
    ",
  0x400a3403u64 => "
      UARTA.asima01(),
    ",
  0x400a3404u64 => "
      UARTA.brgca0(),
    ",
  0x400a3405u64 => "
      UARTA.asisa0(),
    ",
  0x400a3406u64 => "
      UARTA.ascta0(),
    ",
  0x400a3500u64 => "
      UARTA.uta0ck(),
    ",
  0x400a3800u64 => "
      TML_32.itlcmp0()[0],
      TML_32.itlcmp0_l()[0],
    ",
  0x400a3802u64 => "
      TML_32.itlcmp0()[1],
      TML_32.itlcmp0_l()[1],
    ",
  0x400a3801u64 => "
      TML_32.itlcmp0_h()[0],
    ",
  0x400a3803u64 => "
      TML_32.itlcmp0_h()[1],
    ",
  0x400a3804u64 => "
      TML_32.itlcap00(),
    ",
  0x400a3806u64 => "
      TML_32.itlctl0(),
    ",
  0x400a3807u64 => "
      TML_32.itlcsel0(),
    ",
  0x400a3808u64 => "
      TML_32.itlfdiv00(),
    ",
  0x400a3809u64 => "
      TML_32.itlfdiv01(),
    ",
  0x400a380au64 => "
      TML_32.itlcc0(),
    ",
  0x400a380bu64 => "
      TML_32.itls0(),
    ",
  0x400a380cu64 => "
      TML_32.itlmkf0(),
    ",
  0x400a3b01u64 => "
      PCLBUZ.cks0(),
    ",
  0x400d1000u64 => "
      TRNG.trngsdr(),
    ",
  0x400d1002u64 => "
      TRNG.trngscr0(),
    ",
  0x400d1003u64 => "
      TRNG.trngscr1(),
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
  0x407ec200u64 => "
      FLCN.hiotrm(),
    ",
  0x407ec20au64 => "
      FLCN.flmode(),
    ",
  0x407ec20bu64 => "
      FLCN.flmwrp(),
    ",
  0x407ec21au64 => "
      FLCN.fentryr(),
    ",
};
