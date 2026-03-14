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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:49:18 +0000

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
  0x4001e41fu64 => "
      SYSTEM.vbtcr1(),
    ",
  0x4001e4b0u64 => "
      SYSTEM.vbtcr2(),
    ",
  0x4001e4b1u64 => "
      SYSTEM.vbtsr(),
    ",
  0x4001e4b2u64 => "
      SYSTEM.vbtcmpcr(),
    ",
  0x4001e4b4u64 => "
      SYSTEM.vbtlvdicr(),
    ",
  0x4001e4b6u64 => "
      SYSTEM.vbtwctlr(),
    ",
  0x4001e4b8u64 => "
      SYSTEM.vbtwch0otsr(),
    ",
  0x4001e4b9u64 => "
      SYSTEM.vbtwch1otsr(),
    ",
  0x4001e4bau64 => "
      SYSTEM.vbtwch2otsr(),
    ",
  0x4001e4bbu64 => "
      SYSTEM.vbtictlr(),
    ",
  0x4001e4bcu64 => "
      SYSTEM.vbtoctlr(),
    ",
  0x4001e4bdu64 => "
      SYSTEM.vbtwter(),
    ",
  0x4001e4beu64 => "
      SYSTEM.vbtwegr(),
    ",
  0x4001e4bfu64 => "
      SYSTEM.vbtwfr(),
    ",
  0x4001e500u64 => "
      SYSTEM.vbtbkr()[0],
    ",
  0x4001e501u64 => "
      SYSTEM.vbtbkr()[1],
    ",
  0x4001e502u64 => "
      SYSTEM.vbtbkr()[2],
    ",
  0x4001e503u64 => "
      SYSTEM.vbtbkr()[3],
    ",
  0x4001e504u64 => "
      SYSTEM.vbtbkr()[4],
    ",
  0x4001e505u64 => "
      SYSTEM.vbtbkr()[5],
    ",
  0x4001e506u64 => "
      SYSTEM.vbtbkr()[6],
    ",
  0x4001e507u64 => "
      SYSTEM.vbtbkr()[7],
    ",
  0x4001e508u64 => "
      SYSTEM.vbtbkr()[8],
    ",
  0x4001e509u64 => "
      SYSTEM.vbtbkr()[9],
    ",
  0x4001e50au64 => "
      SYSTEM.vbtbkr()[10],
    ",
  0x4001e50bu64 => "
      SYSTEM.vbtbkr()[11],
    ",
  0x4001e50cu64 => "
      SYSTEM.vbtbkr()[12],
    ",
  0x4001e50du64 => "
      SYSTEM.vbtbkr()[13],
    ",
  0x4001e50eu64 => "
      SYSTEM.vbtbkr()[14],
    ",
  0x4001e50fu64 => "
      SYSTEM.vbtbkr()[15],
    ",
  0x4001e510u64 => "
      SYSTEM.vbtbkr()[16],
    ",
  0x4001e511u64 => "
      SYSTEM.vbtbkr()[17],
    ",
  0x4001e512u64 => "
      SYSTEM.vbtbkr()[18],
    ",
  0x4001e513u64 => "
      SYSTEM.vbtbkr()[19],
    ",
  0x4001e514u64 => "
      SYSTEM.vbtbkr()[20],
    ",
  0x4001e515u64 => "
      SYSTEM.vbtbkr()[21],
    ",
  0x4001e516u64 => "
      SYSTEM.vbtbkr()[22],
    ",
  0x4001e517u64 => "
      SYSTEM.vbtbkr()[23],
    ",
  0x4001e518u64 => "
      SYSTEM.vbtbkr()[24],
    ",
  0x4001e519u64 => "
      SYSTEM.vbtbkr()[25],
    ",
  0x4001e51au64 => "
      SYSTEM.vbtbkr()[26],
    ",
  0x4001e51bu64 => "
      SYSTEM.vbtbkr()[27],
    ",
  0x4001e51cu64 => "
      SYSTEM.vbtbkr()[28],
    ",
  0x4001e51du64 => "
      SYSTEM.vbtbkr()[29],
    ",
  0x4001e51eu64 => "
      SYSTEM.vbtbkr()[30],
    ",
  0x4001e51fu64 => "
      SYSTEM.vbtbkr()[31],
    ",
  0x4001e520u64 => "
      SYSTEM.vbtbkr()[32],
    ",
  0x4001e521u64 => "
      SYSTEM.vbtbkr()[33],
    ",
  0x4001e522u64 => "
      SYSTEM.vbtbkr()[34],
    ",
  0x4001e523u64 => "
      SYSTEM.vbtbkr()[35],
    ",
  0x4001e524u64 => "
      SYSTEM.vbtbkr()[36],
    ",
  0x4001e525u64 => "
      SYSTEM.vbtbkr()[37],
    ",
  0x4001e526u64 => "
      SYSTEM.vbtbkr()[38],
    ",
  0x4001e527u64 => "
      SYSTEM.vbtbkr()[39],
    ",
  0x4001e528u64 => "
      SYSTEM.vbtbkr()[40],
    ",
  0x4001e529u64 => "
      SYSTEM.vbtbkr()[41],
    ",
  0x4001e52au64 => "
      SYSTEM.vbtbkr()[42],
    ",
  0x4001e52bu64 => "
      SYSTEM.vbtbkr()[43],
    ",
  0x4001e52cu64 => "
      SYSTEM.vbtbkr()[44],
    ",
  0x4001e52du64 => "
      SYSTEM.vbtbkr()[45],
    ",
  0x4001e52eu64 => "
      SYSTEM.vbtbkr()[46],
    ",
  0x4001e52fu64 => "
      SYSTEM.vbtbkr()[47],
    ",
  0x4001e530u64 => "
      SYSTEM.vbtbkr()[48],
    ",
  0x4001e531u64 => "
      SYSTEM.vbtbkr()[49],
    ",
  0x4001e532u64 => "
      SYSTEM.vbtbkr()[50],
    ",
  0x4001e533u64 => "
      SYSTEM.vbtbkr()[51],
    ",
  0x4001e534u64 => "
      SYSTEM.vbtbkr()[52],
    ",
  0x4001e535u64 => "
      SYSTEM.vbtbkr()[53],
    ",
  0x4001e536u64 => "
      SYSTEM.vbtbkr()[54],
    ",
  0x4001e537u64 => "
      SYSTEM.vbtbkr()[55],
    ",
  0x4001e538u64 => "
      SYSTEM.vbtbkr()[56],
    ",
  0x4001e539u64 => "
      SYSTEM.vbtbkr()[57],
    ",
  0x4001e53au64 => "
      SYSTEM.vbtbkr()[58],
    ",
  0x4001e53bu64 => "
      SYSTEM.vbtbkr()[59],
    ",
  0x4001e53cu64 => "
      SYSTEM.vbtbkr()[60],
    ",
  0x4001e53du64 => "
      SYSTEM.vbtbkr()[61],
    ",
  0x4001e53eu64 => "
      SYSTEM.vbtbkr()[62],
    ",
  0x4001e53fu64 => "
      SYSTEM.vbtbkr()[63],
    ",
  0x4001e540u64 => "
      SYSTEM.vbtbkr()[64],
    ",
  0x4001e541u64 => "
      SYSTEM.vbtbkr()[65],
    ",
  0x4001e542u64 => "
      SYSTEM.vbtbkr()[66],
    ",
  0x4001e543u64 => "
      SYSTEM.vbtbkr()[67],
    ",
  0x4001e544u64 => "
      SYSTEM.vbtbkr()[68],
    ",
  0x4001e545u64 => "
      SYSTEM.vbtbkr()[69],
    ",
  0x4001e546u64 => "
      SYSTEM.vbtbkr()[70],
    ",
  0x4001e547u64 => "
      SYSTEM.vbtbkr()[71],
    ",
  0x4001e548u64 => "
      SYSTEM.vbtbkr()[72],
    ",
  0x4001e549u64 => "
      SYSTEM.vbtbkr()[73],
    ",
  0x4001e54au64 => "
      SYSTEM.vbtbkr()[74],
    ",
  0x4001e54bu64 => "
      SYSTEM.vbtbkr()[75],
    ",
  0x4001e54cu64 => "
      SYSTEM.vbtbkr()[76],
    ",
  0x4001e54du64 => "
      SYSTEM.vbtbkr()[77],
    ",
  0x4001e54eu64 => "
      SYSTEM.vbtbkr()[78],
    ",
  0x4001e54fu64 => "
      SYSTEM.vbtbkr()[79],
    ",
  0x4001e550u64 => "
      SYSTEM.vbtbkr()[80],
    ",
  0x4001e551u64 => "
      SYSTEM.vbtbkr()[81],
    ",
  0x4001e552u64 => "
      SYSTEM.vbtbkr()[82],
    ",
  0x4001e553u64 => "
      SYSTEM.vbtbkr()[83],
    ",
  0x4001e554u64 => "
      SYSTEM.vbtbkr()[84],
    ",
  0x4001e555u64 => "
      SYSTEM.vbtbkr()[85],
    ",
  0x4001e556u64 => "
      SYSTEM.vbtbkr()[86],
    ",
  0x4001e557u64 => "
      SYSTEM.vbtbkr()[87],
    ",
  0x4001e558u64 => "
      SYSTEM.vbtbkr()[88],
    ",
  0x4001e559u64 => "
      SYSTEM.vbtbkr()[89],
    ",
  0x4001e55au64 => "
      SYSTEM.vbtbkr()[90],
    ",
  0x4001e55bu64 => "
      SYSTEM.vbtbkr()[91],
    ",
  0x4001e55cu64 => "
      SYSTEM.vbtbkr()[92],
    ",
  0x4001e55du64 => "
      SYSTEM.vbtbkr()[93],
    ",
  0x4001e55eu64 => "
      SYSTEM.vbtbkr()[94],
    ",
  0x4001e55fu64 => "
      SYSTEM.vbtbkr()[95],
    ",
  0x4001e560u64 => "
      SYSTEM.vbtbkr()[96],
    ",
  0x4001e561u64 => "
      SYSTEM.vbtbkr()[97],
    ",
  0x4001e562u64 => "
      SYSTEM.vbtbkr()[98],
    ",
  0x4001e563u64 => "
      SYSTEM.vbtbkr()[99],
    ",
  0x4001e564u64 => "
      SYSTEM.vbtbkr()[100],
    ",
  0x4001e565u64 => "
      SYSTEM.vbtbkr()[101],
    ",
  0x4001e566u64 => "
      SYSTEM.vbtbkr()[102],
    ",
  0x4001e567u64 => "
      SYSTEM.vbtbkr()[103],
    ",
  0x4001e568u64 => "
      SYSTEM.vbtbkr()[104],
    ",
  0x4001e569u64 => "
      SYSTEM.vbtbkr()[105],
    ",
  0x4001e56au64 => "
      SYSTEM.vbtbkr()[106],
    ",
  0x4001e56bu64 => "
      SYSTEM.vbtbkr()[107],
    ",
  0x4001e56cu64 => "
      SYSTEM.vbtbkr()[108],
    ",
  0x4001e56du64 => "
      SYSTEM.vbtbkr()[109],
    ",
  0x4001e56eu64 => "
      SYSTEM.vbtbkr()[110],
    ",
  0x4001e56fu64 => "
      SYSTEM.vbtbkr()[111],
    ",
  0x4001e570u64 => "
      SYSTEM.vbtbkr()[112],
    ",
  0x4001e571u64 => "
      SYSTEM.vbtbkr()[113],
    ",
  0x4001e572u64 => "
      SYSTEM.vbtbkr()[114],
    ",
  0x4001e573u64 => "
      SYSTEM.vbtbkr()[115],
    ",
  0x4001e574u64 => "
      SYSTEM.vbtbkr()[116],
    ",
  0x4001e575u64 => "
      SYSTEM.vbtbkr()[117],
    ",
  0x4001e576u64 => "
      SYSTEM.vbtbkr()[118],
    ",
  0x4001e577u64 => "
      SYSTEM.vbtbkr()[119],
    ",
  0x4001e578u64 => "
      SYSTEM.vbtbkr()[120],
    ",
  0x4001e579u64 => "
      SYSTEM.vbtbkr()[121],
    ",
  0x4001e57au64 => "
      SYSTEM.vbtbkr()[122],
    ",
  0x4001e57bu64 => "
      SYSTEM.vbtbkr()[123],
    ",
  0x4001e57cu64 => "
      SYSTEM.vbtbkr()[124],
    ",
  0x4001e57du64 => "
      SYSTEM.vbtbkr()[125],
    ",
  0x4001e57eu64 => "
      SYSTEM.vbtbkr()[126],
    ",
  0x4001e57fu64 => "
      SYSTEM.vbtbkr()[127],
    ",
  0x4001e580u64 => "
      SYSTEM.vbtbkr()[128],
    ",
  0x4001e581u64 => "
      SYSTEM.vbtbkr()[129],
    ",
  0x4001e582u64 => "
      SYSTEM.vbtbkr()[130],
    ",
  0x4001e583u64 => "
      SYSTEM.vbtbkr()[131],
    ",
  0x4001e584u64 => "
      SYSTEM.vbtbkr()[132],
    ",
  0x4001e585u64 => "
      SYSTEM.vbtbkr()[133],
    ",
  0x4001e586u64 => "
      SYSTEM.vbtbkr()[134],
    ",
  0x4001e587u64 => "
      SYSTEM.vbtbkr()[135],
    ",
  0x4001e588u64 => "
      SYSTEM.vbtbkr()[136],
    ",
  0x4001e589u64 => "
      SYSTEM.vbtbkr()[137],
    ",
  0x4001e58au64 => "
      SYSTEM.vbtbkr()[138],
    ",
  0x4001e58bu64 => "
      SYSTEM.vbtbkr()[139],
    ",
  0x4001e58cu64 => "
      SYSTEM.vbtbkr()[140],
    ",
  0x4001e58du64 => "
      SYSTEM.vbtbkr()[141],
    ",
  0x4001e58eu64 => "
      SYSTEM.vbtbkr()[142],
    ",
  0x4001e58fu64 => "
      SYSTEM.vbtbkr()[143],
    ",
  0x4001e590u64 => "
      SYSTEM.vbtbkr()[144],
    ",
  0x4001e591u64 => "
      SYSTEM.vbtbkr()[145],
    ",
  0x4001e592u64 => "
      SYSTEM.vbtbkr()[146],
    ",
  0x4001e593u64 => "
      SYSTEM.vbtbkr()[147],
    ",
  0x4001e594u64 => "
      SYSTEM.vbtbkr()[148],
    ",
  0x4001e595u64 => "
      SYSTEM.vbtbkr()[149],
    ",
  0x4001e596u64 => "
      SYSTEM.vbtbkr()[150],
    ",
  0x4001e597u64 => "
      SYSTEM.vbtbkr()[151],
    ",
  0x4001e598u64 => "
      SYSTEM.vbtbkr()[152],
    ",
  0x4001e599u64 => "
      SYSTEM.vbtbkr()[153],
    ",
  0x4001e59au64 => "
      SYSTEM.vbtbkr()[154],
    ",
  0x4001e59bu64 => "
      SYSTEM.vbtbkr()[155],
    ",
  0x4001e59cu64 => "
      SYSTEM.vbtbkr()[156],
    ",
  0x4001e59du64 => "
      SYSTEM.vbtbkr()[157],
    ",
  0x4001e59eu64 => "
      SYSTEM.vbtbkr()[158],
    ",
  0x4001e59fu64 => "
      SYSTEM.vbtbkr()[159],
    ",
  0x4001e5a0u64 => "
      SYSTEM.vbtbkr()[160],
    ",
  0x4001e5a1u64 => "
      SYSTEM.vbtbkr()[161],
    ",
  0x4001e5a2u64 => "
      SYSTEM.vbtbkr()[162],
    ",
  0x4001e5a3u64 => "
      SYSTEM.vbtbkr()[163],
    ",
  0x4001e5a4u64 => "
      SYSTEM.vbtbkr()[164],
    ",
  0x4001e5a5u64 => "
      SYSTEM.vbtbkr()[165],
    ",
  0x4001e5a6u64 => "
      SYSTEM.vbtbkr()[166],
    ",
  0x4001e5a7u64 => "
      SYSTEM.vbtbkr()[167],
    ",
  0x4001e5a8u64 => "
      SYSTEM.vbtbkr()[168],
    ",
  0x4001e5a9u64 => "
      SYSTEM.vbtbkr()[169],
    ",
  0x4001e5aau64 => "
      SYSTEM.vbtbkr()[170],
    ",
  0x4001e5abu64 => "
      SYSTEM.vbtbkr()[171],
    ",
  0x4001e5acu64 => "
      SYSTEM.vbtbkr()[172],
    ",
  0x4001e5adu64 => "
      SYSTEM.vbtbkr()[173],
    ",
  0x4001e5aeu64 => "
      SYSTEM.vbtbkr()[174],
    ",
  0x4001e5afu64 => "
      SYSTEM.vbtbkr()[175],
    ",
  0x4001e5b0u64 => "
      SYSTEM.vbtbkr()[176],
    ",
  0x4001e5b1u64 => "
      SYSTEM.vbtbkr()[177],
    ",
  0x4001e5b2u64 => "
      SYSTEM.vbtbkr()[178],
    ",
  0x4001e5b3u64 => "
      SYSTEM.vbtbkr()[179],
    ",
  0x4001e5b4u64 => "
      SYSTEM.vbtbkr()[180],
    ",
  0x4001e5b5u64 => "
      SYSTEM.vbtbkr()[181],
    ",
  0x4001e5b6u64 => "
      SYSTEM.vbtbkr()[182],
    ",
  0x4001e5b7u64 => "
      SYSTEM.vbtbkr()[183],
    ",
  0x4001e5b8u64 => "
      SYSTEM.vbtbkr()[184],
    ",
  0x4001e5b9u64 => "
      SYSTEM.vbtbkr()[185],
    ",
  0x4001e5bau64 => "
      SYSTEM.vbtbkr()[186],
    ",
  0x4001e5bbu64 => "
      SYSTEM.vbtbkr()[187],
    ",
  0x4001e5bcu64 => "
      SYSTEM.vbtbkr()[188],
    ",
  0x4001e5bdu64 => "
      SYSTEM.vbtbkr()[189],
    ",
  0x4001e5beu64 => "
      SYSTEM.vbtbkr()[190],
    ",
  0x4001e5bfu64 => "
      SYSTEM.vbtbkr()[191],
    ",
  0x4001e5c0u64 => "
      SYSTEM.vbtbkr()[192],
    ",
  0x4001e5c1u64 => "
      SYSTEM.vbtbkr()[193],
    ",
  0x4001e5c2u64 => "
      SYSTEM.vbtbkr()[194],
    ",
  0x4001e5c3u64 => "
      SYSTEM.vbtbkr()[195],
    ",
  0x4001e5c4u64 => "
      SYSTEM.vbtbkr()[196],
    ",
  0x4001e5c5u64 => "
      SYSTEM.vbtbkr()[197],
    ",
  0x4001e5c6u64 => "
      SYSTEM.vbtbkr()[198],
    ",
  0x4001e5c7u64 => "
      SYSTEM.vbtbkr()[199],
    ",
  0x4001e5c8u64 => "
      SYSTEM.vbtbkr()[200],
    ",
  0x4001e5c9u64 => "
      SYSTEM.vbtbkr()[201],
    ",
  0x4001e5cau64 => "
      SYSTEM.vbtbkr()[202],
    ",
  0x4001e5cbu64 => "
      SYSTEM.vbtbkr()[203],
    ",
  0x4001e5ccu64 => "
      SYSTEM.vbtbkr()[204],
    ",
  0x4001e5cdu64 => "
      SYSTEM.vbtbkr()[205],
    ",
  0x4001e5ceu64 => "
      SYSTEM.vbtbkr()[206],
    ",
  0x4001e5cfu64 => "
      SYSTEM.vbtbkr()[207],
    ",
  0x4001e5d0u64 => "
      SYSTEM.vbtbkr()[208],
    ",
  0x4001e5d1u64 => "
      SYSTEM.vbtbkr()[209],
    ",
  0x4001e5d2u64 => "
      SYSTEM.vbtbkr()[210],
    ",
  0x4001e5d3u64 => "
      SYSTEM.vbtbkr()[211],
    ",
  0x4001e5d4u64 => "
      SYSTEM.vbtbkr()[212],
    ",
  0x4001e5d5u64 => "
      SYSTEM.vbtbkr()[213],
    ",
  0x4001e5d6u64 => "
      SYSTEM.vbtbkr()[214],
    ",
  0x4001e5d7u64 => "
      SYSTEM.vbtbkr()[215],
    ",
  0x4001e5d8u64 => "
      SYSTEM.vbtbkr()[216],
    ",
  0x4001e5d9u64 => "
      SYSTEM.vbtbkr()[217],
    ",
  0x4001e5dau64 => "
      SYSTEM.vbtbkr()[218],
    ",
  0x4001e5dbu64 => "
      SYSTEM.vbtbkr()[219],
    ",
  0x4001e5dcu64 => "
      SYSTEM.vbtbkr()[220],
    ",
  0x4001e5ddu64 => "
      SYSTEM.vbtbkr()[221],
    ",
  0x4001e5deu64 => "
      SYSTEM.vbtbkr()[222],
    ",
  0x4001e5dfu64 => "
      SYSTEM.vbtbkr()[223],
    ",
  0x4001e5e0u64 => "
      SYSTEM.vbtbkr()[224],
    ",
  0x4001e5e1u64 => "
      SYSTEM.vbtbkr()[225],
    ",
  0x4001e5e2u64 => "
      SYSTEM.vbtbkr()[226],
    ",
  0x4001e5e3u64 => "
      SYSTEM.vbtbkr()[227],
    ",
  0x4001e5e4u64 => "
      SYSTEM.vbtbkr()[228],
    ",
  0x4001e5e5u64 => "
      SYSTEM.vbtbkr()[229],
    ",
  0x4001e5e6u64 => "
      SYSTEM.vbtbkr()[230],
    ",
  0x4001e5e7u64 => "
      SYSTEM.vbtbkr()[231],
    ",
  0x4001e5e8u64 => "
      SYSTEM.vbtbkr()[232],
    ",
  0x4001e5e9u64 => "
      SYSTEM.vbtbkr()[233],
    ",
  0x4001e5eau64 => "
      SYSTEM.vbtbkr()[234],
    ",
  0x4001e5ebu64 => "
      SYSTEM.vbtbkr()[235],
    ",
  0x4001e5ecu64 => "
      SYSTEM.vbtbkr()[236],
    ",
  0x4001e5edu64 => "
      SYSTEM.vbtbkr()[237],
    ",
  0x4001e5eeu64 => "
      SYSTEM.vbtbkr()[238],
    ",
  0x4001e5efu64 => "
      SYSTEM.vbtbkr()[239],
    ",
  0x4001e5f0u64 => "
      SYSTEM.vbtbkr()[240],
    ",
  0x4001e5f1u64 => "
      SYSTEM.vbtbkr()[241],
    ",
  0x4001e5f2u64 => "
      SYSTEM.vbtbkr()[242],
    ",
  0x4001e5f3u64 => "
      SYSTEM.vbtbkr()[243],
    ",
  0x4001e5f4u64 => "
      SYSTEM.vbtbkr()[244],
    ",
  0x4001e5f5u64 => "
      SYSTEM.vbtbkr()[245],
    ",
  0x4001e5f6u64 => "
      SYSTEM.vbtbkr()[246],
    ",
  0x4001e5f7u64 => "
      SYSTEM.vbtbkr()[247],
    ",
  0x4001e5f8u64 => "
      SYSTEM.vbtbkr()[248],
    ",
  0x4001e5f9u64 => "
      SYSTEM.vbtbkr()[249],
    ",
  0x4001e5fau64 => "
      SYSTEM.vbtbkr()[250],
    ",
  0x4001e5fbu64 => "
      SYSTEM.vbtbkr()[251],
    ",
  0x4001e5fcu64 => "
      SYSTEM.vbtbkr()[252],
    ",
  0x4001e5fdu64 => "
      SYSTEM.vbtbkr()[253],
    ",
  0x4001e5feu64 => "
      SYSTEM.vbtbkr()[254],
    ",
  0x4001e5ffu64 => "
      SYSTEM.vbtbkr()[255],
    ",
  0x4001e600u64 => "
      SYSTEM.vbtbkr()[256],
    ",
  0x4001e601u64 => "
      SYSTEM.vbtbkr()[257],
    ",
  0x4001e602u64 => "
      SYSTEM.vbtbkr()[258],
    ",
  0x4001e603u64 => "
      SYSTEM.vbtbkr()[259],
    ",
  0x4001e604u64 => "
      SYSTEM.vbtbkr()[260],
    ",
  0x4001e605u64 => "
      SYSTEM.vbtbkr()[261],
    ",
  0x4001e606u64 => "
      SYSTEM.vbtbkr()[262],
    ",
  0x4001e607u64 => "
      SYSTEM.vbtbkr()[263],
    ",
  0x4001e608u64 => "
      SYSTEM.vbtbkr()[264],
    ",
  0x4001e609u64 => "
      SYSTEM.vbtbkr()[265],
    ",
  0x4001e60au64 => "
      SYSTEM.vbtbkr()[266],
    ",
  0x4001e60bu64 => "
      SYSTEM.vbtbkr()[267],
    ",
  0x4001e60cu64 => "
      SYSTEM.vbtbkr()[268],
    ",
  0x4001e60du64 => "
      SYSTEM.vbtbkr()[269],
    ",
  0x4001e60eu64 => "
      SYSTEM.vbtbkr()[270],
    ",
  0x4001e60fu64 => "
      SYSTEM.vbtbkr()[271],
    ",
  0x4001e610u64 => "
      SYSTEM.vbtbkr()[272],
    ",
  0x4001e611u64 => "
      SYSTEM.vbtbkr()[273],
    ",
  0x4001e612u64 => "
      SYSTEM.vbtbkr()[274],
    ",
  0x4001e613u64 => "
      SYSTEM.vbtbkr()[275],
    ",
  0x4001e614u64 => "
      SYSTEM.vbtbkr()[276],
    ",
  0x4001e615u64 => "
      SYSTEM.vbtbkr()[277],
    ",
  0x4001e616u64 => "
      SYSTEM.vbtbkr()[278],
    ",
  0x4001e617u64 => "
      SYSTEM.vbtbkr()[279],
    ",
  0x4001e618u64 => "
      SYSTEM.vbtbkr()[280],
    ",
  0x4001e619u64 => "
      SYSTEM.vbtbkr()[281],
    ",
  0x4001e61au64 => "
      SYSTEM.vbtbkr()[282],
    ",
  0x4001e61bu64 => "
      SYSTEM.vbtbkr()[283],
    ",
  0x4001e61cu64 => "
      SYSTEM.vbtbkr()[284],
    ",
  0x4001e61du64 => "
      SYSTEM.vbtbkr()[285],
    ",
  0x4001e61eu64 => "
      SYSTEM.vbtbkr()[286],
    ",
  0x4001e61fu64 => "
      SYSTEM.vbtbkr()[287],
    ",
  0x4001e620u64 => "
      SYSTEM.vbtbkr()[288],
    ",
  0x4001e621u64 => "
      SYSTEM.vbtbkr()[289],
    ",
  0x4001e622u64 => "
      SYSTEM.vbtbkr()[290],
    ",
  0x4001e623u64 => "
      SYSTEM.vbtbkr()[291],
    ",
  0x4001e624u64 => "
      SYSTEM.vbtbkr()[292],
    ",
  0x4001e625u64 => "
      SYSTEM.vbtbkr()[293],
    ",
  0x4001e626u64 => "
      SYSTEM.vbtbkr()[294],
    ",
  0x4001e627u64 => "
      SYSTEM.vbtbkr()[295],
    ",
  0x4001e628u64 => "
      SYSTEM.vbtbkr()[296],
    ",
  0x4001e629u64 => "
      SYSTEM.vbtbkr()[297],
    ",
  0x4001e62au64 => "
      SYSTEM.vbtbkr()[298],
    ",
  0x4001e62bu64 => "
      SYSTEM.vbtbkr()[299],
    ",
  0x4001e62cu64 => "
      SYSTEM.vbtbkr()[300],
    ",
  0x4001e62du64 => "
      SYSTEM.vbtbkr()[301],
    ",
  0x4001e62eu64 => "
      SYSTEM.vbtbkr()[302],
    ",
  0x4001e62fu64 => "
      SYSTEM.vbtbkr()[303],
    ",
  0x4001e630u64 => "
      SYSTEM.vbtbkr()[304],
    ",
  0x4001e631u64 => "
      SYSTEM.vbtbkr()[305],
    ",
  0x4001e632u64 => "
      SYSTEM.vbtbkr()[306],
    ",
  0x4001e633u64 => "
      SYSTEM.vbtbkr()[307],
    ",
  0x4001e634u64 => "
      SYSTEM.vbtbkr()[308],
    ",
  0x4001e635u64 => "
      SYSTEM.vbtbkr()[309],
    ",
  0x4001e636u64 => "
      SYSTEM.vbtbkr()[310],
    ",
  0x4001e637u64 => "
      SYSTEM.vbtbkr()[311],
    ",
  0x4001e638u64 => "
      SYSTEM.vbtbkr()[312],
    ",
  0x4001e639u64 => "
      SYSTEM.vbtbkr()[313],
    ",
  0x4001e63au64 => "
      SYSTEM.vbtbkr()[314],
    ",
  0x4001e63bu64 => "
      SYSTEM.vbtbkr()[315],
    ",
  0x4001e63cu64 => "
      SYSTEM.vbtbkr()[316],
    ",
  0x4001e63du64 => "
      SYSTEM.vbtbkr()[317],
    ",
  0x4001e63eu64 => "
      SYSTEM.vbtbkr()[318],
    ",
  0x4001e63fu64 => "
      SYSTEM.vbtbkr()[319],
    ",
  0x4001e640u64 => "
      SYSTEM.vbtbkr()[320],
    ",
  0x4001e641u64 => "
      SYSTEM.vbtbkr()[321],
    ",
  0x4001e642u64 => "
      SYSTEM.vbtbkr()[322],
    ",
  0x4001e643u64 => "
      SYSTEM.vbtbkr()[323],
    ",
  0x4001e644u64 => "
      SYSTEM.vbtbkr()[324],
    ",
  0x4001e645u64 => "
      SYSTEM.vbtbkr()[325],
    ",
  0x4001e646u64 => "
      SYSTEM.vbtbkr()[326],
    ",
  0x4001e647u64 => "
      SYSTEM.vbtbkr()[327],
    ",
  0x4001e648u64 => "
      SYSTEM.vbtbkr()[328],
    ",
  0x4001e649u64 => "
      SYSTEM.vbtbkr()[329],
    ",
  0x4001e64au64 => "
      SYSTEM.vbtbkr()[330],
    ",
  0x4001e64bu64 => "
      SYSTEM.vbtbkr()[331],
    ",
  0x4001e64cu64 => "
      SYSTEM.vbtbkr()[332],
    ",
  0x4001e64du64 => "
      SYSTEM.vbtbkr()[333],
    ",
  0x4001e64eu64 => "
      SYSTEM.vbtbkr()[334],
    ",
  0x4001e64fu64 => "
      SYSTEM.vbtbkr()[335],
    ",
  0x4001e650u64 => "
      SYSTEM.vbtbkr()[336],
    ",
  0x4001e651u64 => "
      SYSTEM.vbtbkr()[337],
    ",
  0x4001e652u64 => "
      SYSTEM.vbtbkr()[338],
    ",
  0x4001e653u64 => "
      SYSTEM.vbtbkr()[339],
    ",
  0x4001e654u64 => "
      SYSTEM.vbtbkr()[340],
    ",
  0x4001e655u64 => "
      SYSTEM.vbtbkr()[341],
    ",
  0x4001e656u64 => "
      SYSTEM.vbtbkr()[342],
    ",
  0x4001e657u64 => "
      SYSTEM.vbtbkr()[343],
    ",
  0x4001e658u64 => "
      SYSTEM.vbtbkr()[344],
    ",
  0x4001e659u64 => "
      SYSTEM.vbtbkr()[345],
    ",
  0x4001e65au64 => "
      SYSTEM.vbtbkr()[346],
    ",
  0x4001e65bu64 => "
      SYSTEM.vbtbkr()[347],
    ",
  0x4001e65cu64 => "
      SYSTEM.vbtbkr()[348],
    ",
  0x4001e65du64 => "
      SYSTEM.vbtbkr()[349],
    ",
  0x4001e65eu64 => "
      SYSTEM.vbtbkr()[350],
    ",
  0x4001e65fu64 => "
      SYSTEM.vbtbkr()[351],
    ",
  0x4001e660u64 => "
      SYSTEM.vbtbkr()[352],
    ",
  0x4001e661u64 => "
      SYSTEM.vbtbkr()[353],
    ",
  0x4001e662u64 => "
      SYSTEM.vbtbkr()[354],
    ",
  0x4001e663u64 => "
      SYSTEM.vbtbkr()[355],
    ",
  0x4001e664u64 => "
      SYSTEM.vbtbkr()[356],
    ",
  0x4001e665u64 => "
      SYSTEM.vbtbkr()[357],
    ",
  0x4001e666u64 => "
      SYSTEM.vbtbkr()[358],
    ",
  0x4001e667u64 => "
      SYSTEM.vbtbkr()[359],
    ",
  0x4001e668u64 => "
      SYSTEM.vbtbkr()[360],
    ",
  0x4001e669u64 => "
      SYSTEM.vbtbkr()[361],
    ",
  0x4001e66au64 => "
      SYSTEM.vbtbkr()[362],
    ",
  0x4001e66bu64 => "
      SYSTEM.vbtbkr()[363],
    ",
  0x4001e66cu64 => "
      SYSTEM.vbtbkr()[364],
    ",
  0x4001e66du64 => "
      SYSTEM.vbtbkr()[365],
    ",
  0x4001e66eu64 => "
      SYSTEM.vbtbkr()[366],
    ",
  0x4001e66fu64 => "
      SYSTEM.vbtbkr()[367],
    ",
  0x4001e670u64 => "
      SYSTEM.vbtbkr()[368],
    ",
  0x4001e671u64 => "
      SYSTEM.vbtbkr()[369],
    ",
  0x4001e672u64 => "
      SYSTEM.vbtbkr()[370],
    ",
  0x4001e673u64 => "
      SYSTEM.vbtbkr()[371],
    ",
  0x4001e674u64 => "
      SYSTEM.vbtbkr()[372],
    ",
  0x4001e675u64 => "
      SYSTEM.vbtbkr()[373],
    ",
  0x4001e676u64 => "
      SYSTEM.vbtbkr()[374],
    ",
  0x4001e677u64 => "
      SYSTEM.vbtbkr()[375],
    ",
  0x4001e678u64 => "
      SYSTEM.vbtbkr()[376],
    ",
  0x4001e679u64 => "
      SYSTEM.vbtbkr()[377],
    ",
  0x4001e67au64 => "
      SYSTEM.vbtbkr()[378],
    ",
  0x4001e67bu64 => "
      SYSTEM.vbtbkr()[379],
    ",
  0x4001e67cu64 => "
      SYSTEM.vbtbkr()[380],
    ",
  0x4001e67du64 => "
      SYSTEM.vbtbkr()[381],
    ",
  0x4001e67eu64 => "
      SYSTEM.vbtbkr()[382],
    ",
  0x4001e67fu64 => "
      SYSTEM.vbtbkr()[383],
    ",
  0x4001e680u64 => "
      SYSTEM.vbtbkr()[384],
    ",
  0x4001e681u64 => "
      SYSTEM.vbtbkr()[385],
    ",
  0x4001e682u64 => "
      SYSTEM.vbtbkr()[386],
    ",
  0x4001e683u64 => "
      SYSTEM.vbtbkr()[387],
    ",
  0x4001e684u64 => "
      SYSTEM.vbtbkr()[388],
    ",
  0x4001e685u64 => "
      SYSTEM.vbtbkr()[389],
    ",
  0x4001e686u64 => "
      SYSTEM.vbtbkr()[390],
    ",
  0x4001e687u64 => "
      SYSTEM.vbtbkr()[391],
    ",
  0x4001e688u64 => "
      SYSTEM.vbtbkr()[392],
    ",
  0x4001e689u64 => "
      SYSTEM.vbtbkr()[393],
    ",
  0x4001e68au64 => "
      SYSTEM.vbtbkr()[394],
    ",
  0x4001e68bu64 => "
      SYSTEM.vbtbkr()[395],
    ",
  0x4001e68cu64 => "
      SYSTEM.vbtbkr()[396],
    ",
  0x4001e68du64 => "
      SYSTEM.vbtbkr()[397],
    ",
  0x4001e68eu64 => "
      SYSTEM.vbtbkr()[398],
    ",
  0x4001e68fu64 => "
      SYSTEM.vbtbkr()[399],
    ",
  0x4001e690u64 => "
      SYSTEM.vbtbkr()[400],
    ",
  0x4001e691u64 => "
      SYSTEM.vbtbkr()[401],
    ",
  0x4001e692u64 => "
      SYSTEM.vbtbkr()[402],
    ",
  0x4001e693u64 => "
      SYSTEM.vbtbkr()[403],
    ",
  0x4001e694u64 => "
      SYSTEM.vbtbkr()[404],
    ",
  0x4001e695u64 => "
      SYSTEM.vbtbkr()[405],
    ",
  0x4001e696u64 => "
      SYSTEM.vbtbkr()[406],
    ",
  0x4001e697u64 => "
      SYSTEM.vbtbkr()[407],
    ",
  0x4001e698u64 => "
      SYSTEM.vbtbkr()[408],
    ",
  0x4001e699u64 => "
      SYSTEM.vbtbkr()[409],
    ",
  0x4001e69au64 => "
      SYSTEM.vbtbkr()[410],
    ",
  0x4001e69bu64 => "
      SYSTEM.vbtbkr()[411],
    ",
  0x4001e69cu64 => "
      SYSTEM.vbtbkr()[412],
    ",
  0x4001e69du64 => "
      SYSTEM.vbtbkr()[413],
    ",
  0x4001e69eu64 => "
      SYSTEM.vbtbkr()[414],
    ",
  0x4001e69fu64 => "
      SYSTEM.vbtbkr()[415],
    ",
  0x4001e6a0u64 => "
      SYSTEM.vbtbkr()[416],
    ",
  0x4001e6a1u64 => "
      SYSTEM.vbtbkr()[417],
    ",
  0x4001e6a2u64 => "
      SYSTEM.vbtbkr()[418],
    ",
  0x4001e6a3u64 => "
      SYSTEM.vbtbkr()[419],
    ",
  0x4001e6a4u64 => "
      SYSTEM.vbtbkr()[420],
    ",
  0x4001e6a5u64 => "
      SYSTEM.vbtbkr()[421],
    ",
  0x4001e6a6u64 => "
      SYSTEM.vbtbkr()[422],
    ",
  0x4001e6a7u64 => "
      SYSTEM.vbtbkr()[423],
    ",
  0x4001e6a8u64 => "
      SYSTEM.vbtbkr()[424],
    ",
  0x4001e6a9u64 => "
      SYSTEM.vbtbkr()[425],
    ",
  0x4001e6aau64 => "
      SYSTEM.vbtbkr()[426],
    ",
  0x4001e6abu64 => "
      SYSTEM.vbtbkr()[427],
    ",
  0x4001e6acu64 => "
      SYSTEM.vbtbkr()[428],
    ",
  0x4001e6adu64 => "
      SYSTEM.vbtbkr()[429],
    ",
  0x4001e6aeu64 => "
      SYSTEM.vbtbkr()[430],
    ",
  0x4001e6afu64 => "
      SYSTEM.vbtbkr()[431],
    ",
  0x4001e6b0u64 => "
      SYSTEM.vbtbkr()[432],
    ",
  0x4001e6b1u64 => "
      SYSTEM.vbtbkr()[433],
    ",
  0x4001e6b2u64 => "
      SYSTEM.vbtbkr()[434],
    ",
  0x4001e6b3u64 => "
      SYSTEM.vbtbkr()[435],
    ",
  0x4001e6b4u64 => "
      SYSTEM.vbtbkr()[436],
    ",
  0x4001e6b5u64 => "
      SYSTEM.vbtbkr()[437],
    ",
  0x4001e6b6u64 => "
      SYSTEM.vbtbkr()[438],
    ",
  0x4001e6b7u64 => "
      SYSTEM.vbtbkr()[439],
    ",
  0x4001e6b8u64 => "
      SYSTEM.vbtbkr()[440],
    ",
  0x4001e6b9u64 => "
      SYSTEM.vbtbkr()[441],
    ",
  0x4001e6bau64 => "
      SYSTEM.vbtbkr()[442],
    ",
  0x4001e6bbu64 => "
      SYSTEM.vbtbkr()[443],
    ",
  0x4001e6bcu64 => "
      SYSTEM.vbtbkr()[444],
    ",
  0x4001e6bdu64 => "
      SYSTEM.vbtbkr()[445],
    ",
  0x4001e6beu64 => "
      SYSTEM.vbtbkr()[446],
    ",
  0x4001e6bfu64 => "
      SYSTEM.vbtbkr()[447],
    ",
  0x4001e6c0u64 => "
      SYSTEM.vbtbkr()[448],
    ",
  0x4001e6c1u64 => "
      SYSTEM.vbtbkr()[449],
    ",
  0x4001e6c2u64 => "
      SYSTEM.vbtbkr()[450],
    ",
  0x4001e6c3u64 => "
      SYSTEM.vbtbkr()[451],
    ",
  0x4001e6c4u64 => "
      SYSTEM.vbtbkr()[452],
    ",
  0x4001e6c5u64 => "
      SYSTEM.vbtbkr()[453],
    ",
  0x4001e6c6u64 => "
      SYSTEM.vbtbkr()[454],
    ",
  0x4001e6c7u64 => "
      SYSTEM.vbtbkr()[455],
    ",
  0x4001e6c8u64 => "
      SYSTEM.vbtbkr()[456],
    ",
  0x4001e6c9u64 => "
      SYSTEM.vbtbkr()[457],
    ",
  0x4001e6cau64 => "
      SYSTEM.vbtbkr()[458],
    ",
  0x4001e6cbu64 => "
      SYSTEM.vbtbkr()[459],
    ",
  0x4001e6ccu64 => "
      SYSTEM.vbtbkr()[460],
    ",
  0x4001e6cdu64 => "
      SYSTEM.vbtbkr()[461],
    ",
  0x4001e6ceu64 => "
      SYSTEM.vbtbkr()[462],
    ",
  0x4001e6cfu64 => "
      SYSTEM.vbtbkr()[463],
    ",
  0x4001e6d0u64 => "
      SYSTEM.vbtbkr()[464],
    ",
  0x4001e6d1u64 => "
      SYSTEM.vbtbkr()[465],
    ",
  0x4001e6d2u64 => "
      SYSTEM.vbtbkr()[466],
    ",
  0x4001e6d3u64 => "
      SYSTEM.vbtbkr()[467],
    ",
  0x4001e6d4u64 => "
      SYSTEM.vbtbkr()[468],
    ",
  0x4001e6d5u64 => "
      SYSTEM.vbtbkr()[469],
    ",
  0x4001e6d6u64 => "
      SYSTEM.vbtbkr()[470],
    ",
  0x4001e6d7u64 => "
      SYSTEM.vbtbkr()[471],
    ",
  0x4001e6d8u64 => "
      SYSTEM.vbtbkr()[472],
    ",
  0x4001e6d9u64 => "
      SYSTEM.vbtbkr()[473],
    ",
  0x4001e6dau64 => "
      SYSTEM.vbtbkr()[474],
    ",
  0x4001e6dbu64 => "
      SYSTEM.vbtbkr()[475],
    ",
  0x4001e6dcu64 => "
      SYSTEM.vbtbkr()[476],
    ",
  0x4001e6ddu64 => "
      SYSTEM.vbtbkr()[477],
    ",
  0x4001e6deu64 => "
      SYSTEM.vbtbkr()[478],
    ",
  0x4001e6dfu64 => "
      SYSTEM.vbtbkr()[479],
    ",
  0x4001e6e0u64 => "
      SYSTEM.vbtbkr()[480],
    ",
  0x4001e6e1u64 => "
      SYSTEM.vbtbkr()[481],
    ",
  0x4001e6e2u64 => "
      SYSTEM.vbtbkr()[482],
    ",
  0x4001e6e3u64 => "
      SYSTEM.vbtbkr()[483],
    ",
  0x4001e6e4u64 => "
      SYSTEM.vbtbkr()[484],
    ",
  0x4001e6e5u64 => "
      SYSTEM.vbtbkr()[485],
    ",
  0x4001e6e6u64 => "
      SYSTEM.vbtbkr()[486],
    ",
  0x4001e6e7u64 => "
      SYSTEM.vbtbkr()[487],
    ",
  0x4001e6e8u64 => "
      SYSTEM.vbtbkr()[488],
    ",
  0x4001e6e9u64 => "
      SYSTEM.vbtbkr()[489],
    ",
  0x4001e6eau64 => "
      SYSTEM.vbtbkr()[490],
    ",
  0x4001e6ebu64 => "
      SYSTEM.vbtbkr()[491],
    ",
  0x4001e6ecu64 => "
      SYSTEM.vbtbkr()[492],
    ",
  0x4001e6edu64 => "
      SYSTEM.vbtbkr()[493],
    ",
  0x4001e6eeu64 => "
      SYSTEM.vbtbkr()[494],
    ",
  0x4001e6efu64 => "
      SYSTEM.vbtbkr()[495],
    ",
  0x4001e6f0u64 => "
      SYSTEM.vbtbkr()[496],
    ",
  0x4001e6f1u64 => "
      SYSTEM.vbtbkr()[497],
    ",
  0x4001e6f2u64 => "
      SYSTEM.vbtbkr()[498],
    ",
  0x4001e6f3u64 => "
      SYSTEM.vbtbkr()[499],
    ",
  0x4001e6f4u64 => "
      SYSTEM.vbtbkr()[500],
    ",
  0x4001e6f5u64 => "
      SYSTEM.vbtbkr()[501],
    ",
  0x4001e6f6u64 => "
      SYSTEM.vbtbkr()[502],
    ",
  0x4001e6f7u64 => "
      SYSTEM.vbtbkr()[503],
    ",
  0x4001e6f8u64 => "
      SYSTEM.vbtbkr()[504],
    ",
  0x4001e6f9u64 => "
      SYSTEM.vbtbkr()[505],
    ",
  0x4001e6fau64 => "
      SYSTEM.vbtbkr()[506],
    ",
  0x4001e6fbu64 => "
      SYSTEM.vbtbkr()[507],
    ",
  0x4001e6fcu64 => "
      SYSTEM.vbtbkr()[508],
    ",
  0x4001e6fdu64 => "
      SYSTEM.vbtbkr()[509],
    ",
  0x4001e6feu64 => "
      SYSTEM.vbtbkr()[510],
    ",
  0x4001e6ffu64 => "
      SYSTEM.vbtbkr()[511],
    ",
  0x4001e020u64 => "
      SYSTEM.sckdivcr(),
    ",
  0x4001e026u64 => "
      SYSTEM.sckscr(),
    ",
  0x4001e02au64 => "
      SYSTEM.pllcr(),
    ",
  0x4001e02bu64 => "
      SYSTEM.pllccr2(),
    ",
  0x4001e031u64 => "
      SYSTEM.memwait(),
    ",
  0x4001e032u64 => "
      SYSTEM.mosccr(),
    ",
  0x4001e036u64 => "
      SYSTEM.hococr(),
    ",
  0x4001e038u64 => "
      SYSTEM.mococr(),
    ",
  0x4001e03cu64 => "
      SYSTEM.oscsf(),
    ",
  0x4001e03eu64 => "
      SYSTEM.ckocr(),
    ",
  0x4001e03fu64 => "
      SYSTEM.trckcr(),
    ",
  0x4001e040u64 => "
      SYSTEM.ostdcr(),
    ",
  0x4001e041u64 => "
      SYSTEM.ostdsr(),
    ",
  0x4001e050u64 => "
      SYSTEM.slcdsckcr(),
    ",
  0x4001e061u64 => "
      SYSTEM.mocoutcr(),
    ",
  0x4001e062u64 => "
      SYSTEM.hocoutcr(),
    ",
  0x4001e0a2u64 => "
      SYSTEM.moscwtcr(),
    ",
  0x4001e0a5u64 => "
      SYSTEM.hocowtcr(),
    ",
  0x4001e0d0u64 => "
      SYSTEM.usbckcr(),
    ",
  0x4001e413u64 => "
      SYSTEM.momcr(),
    ",
  0x4001e480u64 => "
      SYSTEM.sosccr(),
    ",
  0x4001e481u64 => "
      SYSTEM.somcr(),
    ",
  0x4001e490u64 => "
      SYSTEM.lococr(),
    ",
  0x4001e492u64 => "
      SYSTEM.locoutcr(),
    ",
  0x4001e00cu64 => "
      SYSTEM.sbycr(),
    ",
  0x4001e01cu64 => "
      SYSTEM.mstpcra(),
    ",
  0x4001e092u64 => "
      SYSTEM.snzcr(),
    ",
  0x4001e094u64 => "
      SYSTEM.snzedcr(),
    ",
  0x4001e098u64 => "
      SYSTEM.snzreqcr(),
    ",
  0x4001e09eu64 => "
      SYSTEM.flstop(),
    ",
  0x4001e0a0u64 => "
      SYSTEM.opccr(),
    ",
  0x4001e0aau64 => "
      SYSTEM.sopccr(),
    ",
  0x4001e40eu64 => "
      SYSTEM.syocdcr(),
    ",
  0x4001e417u64 => "
      SYSTEM.lvcmpcr(),
    ",
  0x4001e418u64 => "
      SYSTEM.lvdlvlr(),
    ",
  0x4001e41au64 => "
      SYSTEM.lvdcr0()[0],
    ",
  0x4001e41bu64 => "
      SYSTEM.lvdcr0()[1],
    ",
  0x4001e0e0u64 => "
      SYSTEM.lvdcr1()[0],
    ",
  0x4001e0e2u64 => "
      SYSTEM.lvdcr1()[1],
    ",
  0x4001e0e1u64 => "
      SYSTEM.lvdsr()[0],
    ",
  0x4001e0e3u64 => "
      SYSTEM.lvdsr()[1],
    ",
  0x4001e3feu64 => "
      SYSTEM.prcr(),
    ",
  0x4001e410u64 => "
      SYSTEM.rstsr0(),
    ",
  0x4001e411u64 => "
      SYSTEM.rstsr2(),
    ",
  0x4001e0c0u64 => "
      SYSTEM.rstsr1(),
    ",
  0x4001e0c6u64 => "
      SYSTEM.bkracr(),
    ",
  0x40004000u64 => "
      BUS.busmcnt()[0],
    ",
  0x40004004u64 => "
      BUS.busmcnt()[1],
    ",
  0x40004008u64 => "
      BUS.busmcnt()[2],
    ",
  0x4000400cu64 => "
      BUS.busmcnt()[3],
    ",
  0x40004100u64 => "
      BUS.busscntfli(),
    ",
  0x40004114u64 => "
      BUS.busscnt()[0],
    ",
  0x40004118u64 => "
      BUS.busscnt()[1],
    ",
  0x4000411cu64 => "
      BUS.busscnt()[2],
    ",
  0x40004120u64 => "
      BUS.busscnt()[3],
    ",
  0x40004128u64 => "
      BUS.busscntp6b(),
    ",
  0x40004130u64 => "
      BUS.busscntfbu(),
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
      BUS.buserrstat()[0],
    ",
  0x40004814u64 => "
      BUS.buserrstat()[1],
    ",
  0x40004824u64 => "
      BUS.buserrstat()[2],
    ",
  0x40004834u64 => "
      BUS.buserrstat()[3],
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
      CAN_0.mctl_tx()[0],
      CAN_0.mctl_rx()[0],
    ",
  0x40050821u64 => "
      CAN_0.mctl_tx()[1],
      CAN_0.mctl_rx()[1],
    ",
  0x40050822u64 => "
      CAN_0.mctl_tx()[2],
      CAN_0.mctl_rx()[2],
    ",
  0x40050823u64 => "
      CAN_0.mctl_tx()[3],
      CAN_0.mctl_rx()[3],
    ",
  0x40050824u64 => "
      CAN_0.mctl_tx()[4],
      CAN_0.mctl_rx()[4],
    ",
  0x40050825u64 => "
      CAN_0.mctl_tx()[5],
      CAN_0.mctl_rx()[5],
    ",
  0x40050826u64 => "
      CAN_0.mctl_tx()[6],
      CAN_0.mctl_rx()[6],
    ",
  0x40050827u64 => "
      CAN_0.mctl_tx()[7],
      CAN_0.mctl_rx()[7],
    ",
  0x40050828u64 => "
      CAN_0.mctl_tx()[8],
      CAN_0.mctl_rx()[8],
    ",
  0x40050829u64 => "
      CAN_0.mctl_tx()[9],
      CAN_0.mctl_rx()[9],
    ",
  0x4005082au64 => "
      CAN_0.mctl_tx()[10],
      CAN_0.mctl_rx()[10],
    ",
  0x4005082bu64 => "
      CAN_0.mctl_tx()[11],
      CAN_0.mctl_rx()[11],
    ",
  0x4005082cu64 => "
      CAN_0.mctl_tx()[12],
      CAN_0.mctl_rx()[12],
    ",
  0x4005082du64 => "
      CAN_0.mctl_tx()[13],
      CAN_0.mctl_rx()[13],
    ",
  0x4005082eu64 => "
      CAN_0.mctl_tx()[14],
      CAN_0.mctl_rx()[14],
    ",
  0x4005082fu64 => "
      CAN_0.mctl_tx()[15],
      CAN_0.mctl_rx()[15],
    ",
  0x40050830u64 => "
      CAN_0.mctl_tx()[16],
      CAN_0.mctl_rx()[16],
    ",
  0x40050831u64 => "
      CAN_0.mctl_tx()[17],
      CAN_0.mctl_rx()[17],
    ",
  0x40050832u64 => "
      CAN_0.mctl_tx()[18],
      CAN_0.mctl_rx()[18],
    ",
  0x40050833u64 => "
      CAN_0.mctl_tx()[19],
      CAN_0.mctl_rx()[19],
    ",
  0x40050834u64 => "
      CAN_0.mctl_tx()[20],
      CAN_0.mctl_rx()[20],
    ",
  0x40050835u64 => "
      CAN_0.mctl_tx()[21],
      CAN_0.mctl_rx()[21],
    ",
  0x40050836u64 => "
      CAN_0.mctl_tx()[22],
      CAN_0.mctl_rx()[22],
    ",
  0x40050837u64 => "
      CAN_0.mctl_tx()[23],
      CAN_0.mctl_rx()[23],
    ",
  0x40050838u64 => "
      CAN_0.mctl_tx()[24],
      CAN_0.mctl_rx()[24],
    ",
  0x40050839u64 => "
      CAN_0.mctl_tx()[25],
      CAN_0.mctl_rx()[25],
    ",
  0x4005083au64 => "
      CAN_0.mctl_tx()[26],
      CAN_0.mctl_rx()[26],
    ",
  0x4005083bu64 => "
      CAN_0.mctl_tx()[27],
      CAN_0.mctl_rx()[27],
    ",
  0x4005083cu64 => "
      CAN_0.mctl_tx()[28],
      CAN_0.mctl_rx()[28],
    ",
  0x4005083du64 => "
      CAN_0.mctl_tx()[29],
      CAN_0.mctl_rx()[29],
    ",
  0x4005083eu64 => "
      CAN_0.mctl_tx()[30],
      CAN_0.mctl_rx()[30],
    ",
  0x4005083fu64 => "
      CAN_0.mctl_tx()[31],
      CAN_0.mctl_rx()[31],
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
  0x4001b000u64 => "
      DBG.dbgstr(),
    ",
  0x4001b010u64 => "
      DBG.dbgstopcr(),
    ",
  0x4001b020u64 => "
      DBG.tracectr(),
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
  0x40081000u64 => "
      CTSU.ctsucr0(),
    ",
  0x40081001u64 => "
      CTSU.ctsucr1(),
    ",
  0x40081002u64 => "
      CTSU.ctsusdprs(),
    ",
  0x40081003u64 => "
      CTSU.ctsusst(),
    ",
  0x40081004u64 => "
      CTSU.ctsumch0(),
    ",
  0x40081005u64 => "
      CTSU.ctsumch1(),
    ",
  0x40081006u64 => "
      CTSU.ctsuchac0(),
    ",
  0x40081007u64 => "
      CTSU.ctsuchac1(),
    ",
  0x40081008u64 => "
      CTSU.ctsuchac2(),
    ",
  0x40081009u64 => "
      CTSU.ctsuchac3(),
    ",
  0x4008100au64 => "
      CTSU.ctsuchac4(),
    ",
  0x4008100bu64 => "
      CTSU.ctsuchtrc0(),
    ",
  0x4008100cu64 => "
      CTSU.ctsuchtrc1(),
    ",
  0x4008100du64 => "
      CTSU.ctsuchtrc2(),
    ",
  0x4008100eu64 => "
      CTSU.ctsuchtrc3(),
    ",
  0x4008100fu64 => "
      CTSU.ctsuchtrc4(),
    ",
  0x40081010u64 => "
      CTSU.ctsudclkc(),
    ",
  0x40081011u64 => "
      CTSU.ctsust(),
    ",
  0x40081012u64 => "
      CTSU.ctsussc(),
    ",
  0x40081014u64 => "
      CTSU.ctsuso0(),
    ",
  0x40081016u64 => "
      CTSU.ctsuso1(),
    ",
  0x40081018u64 => "
      CTSU.ctsusc(),
    ",
  0x4008101au64 => "
      CTSU.ctsurc(),
    ",
  0x4008101cu64 => "
      CTSU.ctsuerrs(),
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
  0x4009e000u64 => "
      DAC_8.dacs()[0],
    ",
  0x4009e001u64 => "
      DAC_8.dacs()[1],
    ",
  0x4009e003u64 => "
      DAC_8.dam(),
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
  0x40005200u64 => "
      DMA.dmast(),
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
  0x40041000u64 => "
      ELC.elcr(),
    ",
  0x40041002u64 => "
      ELC.elsegr()[0],
    ",
  0x40041004u64 => "
      ELC.elsegr()[1],
    ",
  0x40041024u64 => "
      ELC.elsr()[5],
    ",
  0x40041028u64 => "
      ELC.elsr()[6],
    ",
  0x4004102cu64 => "
      ELC.elsr()[7],
    ",
  0x40041030u64 => "
      ELC.elsr()[8],
    ",
  0x40041034u64 => "
      ELC.elsr()[9],
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
  0x40041050u64 => "
      ELC.elsr()[2],
    ",
  0x40041054u64 => "
      ELC.elsr()[3],
    ",
  0x40041058u64 => "
      ELC.elsr()[4],
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
      PFS.p000pfs(),
    ",
  0x40040802u64 => "
      PFS.p000pfs_ha(),
    ",
  0x40040803u64 => "
      PFS.p000pfs_by(),
    ",
  0x40040804u64 => "
      PFS.p00pfs()[0],
    ",
  0x40040808u64 => "
      PFS.p00pfs()[1],
    ",
  0x4004080cu64 => "
      PFS.p00pfs()[2],
    ",
  0x40040810u64 => "
      PFS.p00pfs()[3],
    ",
  0x40040814u64 => "
      PFS.p00pfs()[4],
    ",
  0x40040818u64 => "
      PFS.p00pfs()[5],
    ",
  0x4004081cu64 => "
      PFS.p00pfs()[6],
    ",
  0x40040820u64 => "
      PFS.p00pfs()[7],
    ",
  0x40040806u64 => "
      PFS.p00pfs_ha()[0],
    ",
  0x4004080au64 => "
      PFS.p00pfs_ha()[1],
    ",
  0x4004080eu64 => "
      PFS.p00pfs_ha()[2],
    ",
  0x40040812u64 => "
      PFS.p00pfs_ha()[3],
    ",
  0x40040816u64 => "
      PFS.p00pfs_ha()[4],
    ",
  0x4004081au64 => "
      PFS.p00pfs_ha()[5],
    ",
  0x4004081eu64 => "
      PFS.p00pfs_ha()[6],
    ",
  0x40040822u64 => "
      PFS.p00pfs_ha()[7],
    ",
  0x40040807u64 => "
      PFS.p00pfs_by()[0],
    ",
  0x4004080bu64 => "
      PFS.p00pfs_by()[1],
    ",
  0x4004080fu64 => "
      PFS.p00pfs_by()[2],
    ",
  0x40040813u64 => "
      PFS.p00pfs_by()[3],
    ",
  0x40040817u64 => "
      PFS.p00pfs_by()[4],
    ",
  0x4004081bu64 => "
      PFS.p00pfs_by()[5],
    ",
  0x4004081fu64 => "
      PFS.p00pfs_by()[6],
    ",
  0x40040823u64 => "
      PFS.p00pfs_by()[7],
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
      PFS.p110pfs(),
    ",
  0x4004086au64 => "
      PFS.p110pfs_ha(),
    ",
  0x4004086bu64 => "
      PFS.p110pfs_by(),
    ",
  0x4004086cu64 => "
      PFS.p1pfs()[0],
    ",
  0x40040870u64 => "
      PFS.p1pfs()[1],
    ",
  0x40040874u64 => "
      PFS.p1pfs()[2],
    ",
  0x40040878u64 => "
      PFS.p1pfs()[3],
    ",
  0x4004087cu64 => "
      PFS.p1pfs()[4],
    ",
  0x4004086eu64 => "
      PFS.p1pfs_ha()[0],
    ",
  0x40040872u64 => "
      PFS.p1pfs_ha()[1],
    ",
  0x40040876u64 => "
      PFS.p1pfs_ha()[2],
    ",
  0x4004087au64 => "
      PFS.p1pfs_ha()[3],
    ",
  0x4004087eu64 => "
      PFS.p1pfs_ha()[4],
    ",
  0x4004086fu64 => "
      PFS.p1pfs_by()[0],
    ",
  0x40040873u64 => "
      PFS.p1pfs_by()[1],
    ",
  0x40040877u64 => "
      PFS.p1pfs_by()[2],
    ",
  0x4004087bu64 => "
      PFS.p1pfs_by()[3],
    ",
  0x4004087fu64 => "
      PFS.p1pfs_by()[4],
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
  0x40040920u64 => "
      PFS.p408pfs(),
    ",
  0x40040922u64 => "
      PFS.p408pfs_ha(),
    ",
  0x40040923u64 => "
      PFS.p408pfs_by(),
    ",
  0x40040924u64 => "
      PFS.p409pfs(),
    ",
  0x40040926u64 => "
      PFS.p409pfs_ha(),
    ",
  0x40040927u64 => "
      PFS.p409pfs_by(),
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
  0x40040a78u64 => "
      PFS.p9pfs()[0],
    ",
  0x40040a7cu64 => "
      PFS.p9pfs()[1],
    ",
  0x40040a7au64 => "
      PFS.p9pfs_ha()[0],
    ",
  0x40040a7eu64 => "
      PFS.p9pfs_ha()[1],
    ",
  0x40040a7bu64 => "
      PFS.p9pfs_by()[0],
    ",
  0x40040a7fu64 => "
      PFS.p9pfs_by()[1],
    ",
  0x40040d03u64 => "
      PMISC.pwpr(),
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
  0x40006140u64 => "
      ICU.nmisr(),
    ",
  0x40006120u64 => "
      ICU.nmier(),
    ",
  0x40006130u64 => "
      ICU.nmiclr(),
    ",
  0x40006100u64 => "
      ICU.nmicr(),
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
  0x40006200u64 => "
      ICU.selsr0(),
    ",
  0x400061a0u64 => "
      ICU.wupen(),
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
      IIC_0.icwur(),
    ",
  0x40053017u64 => "
      IIC_0.icwur2(),
    ",
  0x40053100u64 => "
      IIC_1.iccr1(),
    ",
  0x40053101u64 => "
      IIC_1.iccr2(),
    ",
  0x40053102u64 => "
      IIC_1.icmr1(),
    ",
  0x40053103u64 => "
      IIC_1.icmr2(),
    ",
  0x40053104u64 => "
      IIC_1.icmr3(),
    ",
  0x40053105u64 => "
      IIC_1.icfer(),
    ",
  0x40053106u64 => "
      IIC_1.icser(),
    ",
  0x40053107u64 => "
      IIC_1.icier(),
    ",
  0x40053108u64 => "
      IIC_1.icsr1(),
    ",
  0x40053109u64 => "
      IIC_1.icsr2(),
    ",
  0x4005310au64 => "
      IIC_1.sarl()[0],
    ",
  0x4005310cu64 => "
      IIC_1.sarl()[1],
    ",
  0x4005310eu64 => "
      IIC_1.sarl()[2],
    ",
  0x4005310bu64 => "
      IIC_1.saru()[0],
    ",
  0x4005310du64 => "
      IIC_1.saru()[1],
    ",
  0x4005310fu64 => "
      IIC_1.saru()[2],
    ",
  0x40053110u64 => "
      IIC_1.icbrl(),
    ",
  0x40053111u64 => "
      IIC_1.icbrh(),
    ",
  0x40053112u64 => "
      IIC_1.icdrt(),
    ",
  0x40053113u64 => "
      IIC_1.icdrr(),
    ",
  0x40044400u64 => "
      IWDT.iwdtrr(),
    ",
  0x40044404u64 => "
      IWDT.iwdtsr(),
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
  0x40047000u64 => "
      MSTP.mstpcrb(),
    ",
  0x40047004u64 => "
      MSTP.mstpcrc(),
    ",
  0x40047008u64 => "
      MSTP.mstpcrd(),
    ",
  0x40000000u64 => "
      MMPU.mmpuctla(),
    ",
  0x40000200u64 => "
      MMPU.mmpuaca()[0],
    ",
  0x40000210u64 => "
      MMPU.mmpuaca()[1],
    ",
  0x40000220u64 => "
      MMPU.mmpuaca()[2],
    ",
  0x40000230u64 => "
      MMPU.mmpuaca()[3],
    ",
  0x40000240u64 => "
      MMPU.mmpuaca()[4],
    ",
  0x40000250u64 => "
      MMPU.mmpuaca()[5],
    ",
  0x40000260u64 => "
      MMPU.mmpuaca()[6],
    ",
  0x40000270u64 => "
      MMPU.mmpuaca()[7],
    ",
  0x40000280u64 => "
      MMPU.mmpuaca()[8],
    ",
  0x40000290u64 => "
      MMPU.mmpuaca()[9],
    ",
  0x400002a0u64 => "
      MMPU.mmpuaca()[10],
    ",
  0x400002b0u64 => "
      MMPU.mmpuaca()[11],
    ",
  0x400002c0u64 => "
      MMPU.mmpuaca()[12],
    ",
  0x400002d0u64 => "
      MMPU.mmpuaca()[13],
    ",
  0x400002e0u64 => "
      MMPU.mmpuaca()[14],
    ",
  0x400002f0u64 => "
      MMPU.mmpuaca()[15],
    ",
  0x40000204u64 => "
      MMPU.mmpusa()[0],
    ",
  0x40000214u64 => "
      MMPU.mmpusa()[1],
    ",
  0x40000224u64 => "
      MMPU.mmpusa()[2],
    ",
  0x40000234u64 => "
      MMPU.mmpusa()[3],
    ",
  0x40000244u64 => "
      MMPU.mmpusa()[4],
    ",
  0x40000254u64 => "
      MMPU.mmpusa()[5],
    ",
  0x40000264u64 => "
      MMPU.mmpusa()[6],
    ",
  0x40000274u64 => "
      MMPU.mmpusa()[7],
    ",
  0x40000284u64 => "
      MMPU.mmpusa()[8],
    ",
  0x40000294u64 => "
      MMPU.mmpusa()[9],
    ",
  0x400002a4u64 => "
      MMPU.mmpusa()[10],
    ",
  0x400002b4u64 => "
      MMPU.mmpusa()[11],
    ",
  0x400002c4u64 => "
      MMPU.mmpusa()[12],
    ",
  0x400002d4u64 => "
      MMPU.mmpusa()[13],
    ",
  0x400002e4u64 => "
      MMPU.mmpusa()[14],
    ",
  0x400002f4u64 => "
      MMPU.mmpusa()[15],
    ",
  0x40000208u64 => "
      MMPU.mmpuea()[0],
    ",
  0x40000218u64 => "
      MMPU.mmpuea()[1],
    ",
  0x40000228u64 => "
      MMPU.mmpuea()[2],
    ",
  0x40000238u64 => "
      MMPU.mmpuea()[3],
    ",
  0x40000248u64 => "
      MMPU.mmpuea()[4],
    ",
  0x40000258u64 => "
      MMPU.mmpuea()[5],
    ",
  0x40000268u64 => "
      MMPU.mmpuea()[6],
    ",
  0x40000278u64 => "
      MMPU.mmpuea()[7],
    ",
  0x40000288u64 => "
      MMPU.mmpuea()[8],
    ",
  0x40000298u64 => "
      MMPU.mmpuea()[9],
    ",
  0x400002a8u64 => "
      MMPU.mmpuea()[10],
    ",
  0x400002b8u64 => "
      MMPU.mmpuea()[11],
    ",
  0x400002c8u64 => "
      MMPU.mmpuea()[12],
    ",
  0x400002d8u64 => "
      MMPU.mmpuea()[13],
    ",
  0x400002e8u64 => "
      MMPU.mmpuea()[14],
    ",
  0x400002f8u64 => "
      MMPU.mmpuea()[15],
    ",
  0x40000102u64 => "
      MMPU.mmpupta(),
    ",
  0x40000c00u64 => "
      SMPU.smpuctl(),
    ",
  0x40000c10u64 => "
      SMPU.smpumbiu(),
    ",
  0x40000c14u64 => "
      SMPU.smpufbiu(),
    ",
  0x40000c18u64 => "
      SMPU.smpusram0(),
    ",
  0x40000c20u64 => "
      SMPU.smpupbiu()[0],
    ",
  0x40000c24u64 => "
      SMPU.smpupbiu()[1],
    ",
  0x40000c28u64 => "
      SMPU.smpupbiu()[2],
    ",
  0x40000d00u64 => "
      SPMON.mspmpuoad(),
    ",
  0x40000d04u64 => "
      SPMON.mspmpuctl(),
    ",
  0x40000d06u64 => "
      SPMON.mspmpupt(),
    ",
  0x40000d08u64 => "
      SPMON.mspmpusa(),
    ",
  0x40000d0cu64 => "
      SPMON.mspmpuea(),
    ",
  0x40000d10u64 => "
      SPMON.pspmpuoad(),
    ",
  0x40000d14u64 => "
      SPMON.pspmpuctl(),
    ",
  0x40000d16u64 => "
      SPMON.pspmpupt(),
    ",
  0x40000d18u64 => "
      SPMON.pspmpusa(),
    ",
  0x40000d1cu64 => "
      SPMON.pspmpuea(),
    ",
  0x40086008u64 => "
      OPAMP.ampmc(),
    ",
  0x40086009u64 => "
      OPAMP.amptrm(),
    ",
  0x4008600au64 => "
      OPAMP.amptrs(),
    ",
  0x4008600bu64 => "
      OPAMP.ampc(),
    ",
  0x4008600cu64 => "
      OPAMP.ampmon(),
    ",
  0x40042000u64 => "
      POEG.poegg()[0],
    ",
  0x40042100u64 => "
      POEG.poegg()[1],
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
  0x40044000u64 => "
      RTC.r64cnt(),
    ",
  0x40044002u64 => "
      RTC.rseccnt(),
      RTC.bcnt0(),
    ",
  0x40044004u64 => "
      RTC.rmincnt(),
      RTC.bcnt1(),
    ",
  0x40044006u64 => "
      RTC.rhrcnt(),
      RTC.bcnt2(),
    ",
  0x40044008u64 => "
      RTC.rwkcnt(),
      RTC.bcnt3(),
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
      RTC.rsecar(),
      RTC.bcnt0ar(),
    ",
  0x40044012u64 => "
      RTC.rminar(),
      RTC.bcnt1ar(),
    ",
  0x40044014u64 => "
      RTC.rhrar(),
      RTC.bcnt2ar(),
    ",
  0x40044016u64 => "
      RTC.rwkar(),
      RTC.bcnt3ar(),
    ",
  0x40044018u64 => "
      RTC.rdayar(),
      RTC.bcnt0aer(),
    ",
  0x4004401au64 => "
      RTC.rmonar(),
      RTC.bcnt1aer(),
    ",
  0x4004401cu64 => "
      RTC.ryrar(),
      RTC.bcnt2aer(),
    ",
  0x4004401eu64 => "
      RTC.ryraren(),
      RTC.bcnt3aer(),
    ",
  0x40044022u64 => "
      RTC.rcr1(),
    ",
  0x40044024u64 => "
      RTC.rcr2(),
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
      RTC.rseccp()[0],
      RTC.bcnt0cp()[0],
    ",
  0x40044062u64 => "
      RTC.rseccp()[1],
      RTC.bcnt0cp()[1],
    ",
  0x40044072u64 => "
      RTC.rseccp()[2],
      RTC.bcnt0cp()[2],
    ",
  0x40044054u64 => "
      RTC.rmincp()[0],
      RTC.bcnt1cp()[0],
    ",
  0x40044064u64 => "
      RTC.rmincp()[1],
      RTC.bcnt1cp()[1],
    ",
  0x40044074u64 => "
      RTC.rmincp()[2],
      RTC.bcnt1cp()[2],
    ",
  0x40044056u64 => "
      RTC.rhrcp()[0],
      RTC.bcnt2cp()[0],
    ",
  0x40044066u64 => "
      RTC.rhrcp()[1],
      RTC.bcnt2cp()[1],
    ",
  0x40044076u64 => "
      RTC.rhrcp()[2],
      RTC.bcnt2cp()[2],
    ",
  0x4004405au64 => "
      RTC.rdaycp()[0],
      RTC.bcnt3cp()[0],
    ",
  0x4004406au64 => "
      RTC.rdaycp()[1],
      RTC.bcnt3cp()[1],
    ",
  0x4004407au64 => "
      RTC.rdaycp()[2],
      RTC.bcnt3cp()[2],
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
      SCI_0.tdrhl(),
      SCI_0.ftdrhl(),
      SCI_0.ftdrh(),
    ",
  0x4007000fu64 => "
      SCI_0.ftdrl(),
    ",
  0x40070010u64 => "
      SCI_0.rdrhl(),
      SCI_0.frdrhl(),
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
  0x40070040u64 => "
      SCI_2.smr(),
      SCI_2.smr_smci(),
    ",
  0x40070041u64 => "
      SCI_2.brr(),
    ",
  0x40070042u64 => "
      SCI_2.scr(),
      SCI_2.scr_smci(),
    ",
  0x40070043u64 => "
      SCI_2.tdr(),
    ",
  0x40070044u64 => "
      SCI_2.ssr(),
      SCI_2.ssr_smci(),
    ",
  0x40070045u64 => "
      SCI_2.rdr(),
    ",
  0x40070046u64 => "
      SCI_2.scmr(),
    ",
  0x40070047u64 => "
      SCI_2.semr(),
    ",
  0x40070048u64 => "
      SCI_2.snfr(),
    ",
  0x40070049u64 => "
      SCI_2.simr1(),
    ",
  0x4007004au64 => "
      SCI_2.simr2(),
    ",
  0x4007004bu64 => "
      SCI_2.simr3(),
    ",
  0x4007004cu64 => "
      SCI_2.sisr(),
    ",
  0x4007004du64 => "
      SCI_2.spmr(),
    ",
  0x4007004eu64 => "
      SCI_2.tdrhl(),
    ",
  0x40070050u64 => "
      SCI_2.rdrhl(),
    ",
  0x40070052u64 => "
      SCI_2.mddr(),
    ",
  0x40070053u64 => "
      SCI_2.dccr(),
    ",
  0x4007005au64 => "
      SCI_2.cdr(),
    ",
  0x4007005cu64 => "
      SCI_2.sptr(),
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
  0x40072100u64 => "
      SPI_1.spcr(),
    ",
  0x40072101u64 => "
      SPI_1.sslp(),
    ",
  0x40072102u64 => "
      SPI_1.sppcr(),
    ",
  0x40072103u64 => "
      SPI_1.spsr(),
    ",
  0x40072104u64 => "
      SPI_1.spdr(),
      SPI_1.spdr_ha(),
    ",
  0x4007210au64 => "
      SPI_1.spbr(),
    ",
  0x4007210bu64 => "
      SPI_1.spdcr(),
    ",
  0x4007210cu64 => "
      SPI_1.spckd(),
    ",
  0x4007210du64 => "
      SPI_1.sslnd(),
    ",
  0x4007210eu64 => "
      SPI_1.spnd(),
    ",
  0x4007210fu64 => "
      SPI_1.spcr2(),
    ",
  0x40072110u64 => "
      SPI_1.spcmd0(),
    ",
  0x4004e000u64 => "
      SSIE_0.ssicr(),
    ",
  0x4004e004u64 => "
      SSIE_0.ssisr(),
    ",
  0x4004e010u64 => "
      SSIE_0.ssifcr(),
    ",
  0x4004e014u64 => "
      SSIE_0.ssifsr(),
    ",
  0x4004e018u64 => "
      SSIE_0.ssiftdr(),
    ",
  0x4004e01cu64 => "
      SSIE_0.ssifrdr(),
    ",
  0x4004e020u64 => "
      SSIE_0.ssitdmr(),
    ",
  0x4004e024u64 => "
      SSIE_0.ssiscr(),
    ",
  0x407ec229u64 => "
      TSN.tscdrh(),
    ",
  0x407ec228u64 => "
      TSN.tscdrl(),
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
      USBFS.d0fifo(),
      USBFS.d0fifol(),
    ",
  0x4009001cu64 => "
      USBFS.d1fifo(),
      USBFS.d1fifol(),
    ",
  0x40090020u64 => "
      USBFS.cfifosel(),
    ",
  0x40090022u64 => "
      USBFS.cfifoctr(),
    ",
  0x40090028u64 => "
      USBFS.d0fifosel(),
    ",
  0x4009002au64 => "
      USBFS.d0fifoctr(),
    ",
  0x4009002cu64 => "
      USBFS.d1fifosel(),
    ",
  0x4009002eu64 => "
      USBFS.d1fifoctr(),
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
  0x400900ccu64 => "
      USBFS.usbmc(),
    ",
  0x400900b0u64 => "
      USBFS.usbbcctrl0(),
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
  0x40085e00u64 => "
      ACMPLP.compmdr(),
    ",
  0x40085e01u64 => "
      ACMPLP.compfir(),
    ",
  0x40085e02u64 => "
      ACMPLP.compocr(),
    ",
  0x40085e04u64 => "
      ACMPLP.compsel0(),
    ",
  0x40085e05u64 => "
      ACMPLP.compsel1(),
    ",
  0x4005c000u64 => "
      ADC_140.adcsr(),
    ",
  0x4005c004u64 => "
      ADC_140.adansa0(),
    ",
  0x4005c006u64 => "
      ADC_140.adansa1(),
    ",
  0x4005c008u64 => "
      ADC_140.adads0(),
    ",
  0x4005c00au64 => "
      ADC_140.adads1(),
    ",
  0x4005c00cu64 => "
      ADC_140.adadc(),
    ",
  0x4005c00eu64 => "
      ADC_140.adcer(),
    ",
  0x4005c010u64 => "
      ADC_140.adstrgr(),
    ",
  0x4005c012u64 => "
      ADC_140.adexicr(),
    ",
  0x4005c014u64 => "
      ADC_140.adansb0(),
    ",
  0x4005c016u64 => "
      ADC_140.adansb1(),
    ",
  0x4005c018u64 => "
      ADC_140.addbldr(),
    ",
  0x4005c01au64 => "
      ADC_140.adtsdr(),
    ",
  0x4005c01cu64 => "
      ADC_140.adocdr(),
    ",
  0x4005c01eu64 => "
      ADC_140.adrd(),
    ",
  0x4005c034u64 => "
      ADC_140.addr()[10],
    ",
  0x4005c036u64 => "
      ADC_140.addr()[11],
    ",
  0x4005c038u64 => "
      ADC_140.addr()[12],
    ",
  0x4005c03au64 => "
      ADC_140.addr()[13],
    ",
  0x4005c03cu64 => "
      ADC_140.addr()[14],
    ",
  0x4005c040u64 => "
      ADC_140.addr()[0],
    ",
  0x4005c042u64 => "
      ADC_140.addr()[1],
    ",
  0x4005c044u64 => "
      ADC_140.addr()[2],
    ",
  0x4005c046u64 => "
      ADC_140.addr()[3],
    ",
  0x4005c048u64 => "
      ADC_140.addr()[4],
    ",
  0x4005c04au64 => "
      ADC_140.addr()[5],
    ",
  0x4005c04cu64 => "
      ADC_140.addr()[6],
    ",
  0x4005c04eu64 => "
      ADC_140.addr()[7],
    ",
  0x4005c050u64 => "
      ADC_140.addr()[8],
    ",
  0x4005c052u64 => "
      ADC_140.addr()[9],
    ",
  0x4005c07au64 => "
      ADC_140.addiscr(),
    ",
  0x4005c080u64 => "
      ADC_140.adgspcr(),
    ",
  0x4005c084u64 => "
      ADC_140.addbldra(),
    ",
  0x4005c086u64 => "
      ADC_140.addbldrb(),
    ",
  0x4005c08au64 => "
      ADC_140.adhvrefcnt(),
    ",
  0x4005c08cu64 => "
      ADC_140.adwinmon(),
    ",
  0x4005c090u64 => "
      ADC_140.adcmpcr(),
    ",
  0x4005c092u64 => "
      ADC_140.adcmpanser(),
    ",
  0x4005c093u64 => "
      ADC_140.adcmpler(),
    ",
  0x4005c094u64 => "
      ADC_140.adcmpansr0(),
    ",
  0x4005c096u64 => "
      ADC_140.adcmpansr1(),
    ",
  0x4005c098u64 => "
      ADC_140.adcmplr0(),
    ",
  0x4005c09au64 => "
      ADC_140.adcmplr1(),
    ",
  0x4005c09cu64 => "
      ADC_140.adcmpdr0(),
    ",
  0x4005c09eu64 => "
      ADC_140.adcmpdr1(),
    ",
  0x4005c0a0u64 => "
      ADC_140.adcmpsr0(),
    ",
  0x4005c0a2u64 => "
      ADC_140.adcmpsr1(),
    ",
  0x4005c0a4u64 => "
      ADC_140.adcmpser(),
    ",
  0x4005c0a6u64 => "
      ADC_140.adcmpbnsr(),
    ",
  0x4005c0a8u64 => "
      ADC_140.adwinllb(),
    ",
  0x4005c0aau64 => "
      ADC_140.adwinulb(),
    ",
  0x4005c0acu64 => "
      ADC_140.adcmpbsr(),
    ",
  0x4005c0ddu64 => "
      ADC_140.adsstrl(),
    ",
  0x4005c0deu64 => "
      ADC_140.adsstrt(),
    ",
  0x4005c0dfu64 => "
      ADC_140.adsstro(),
    ",
  0x4005c0e0u64 => "
      ADC_140.adsstr()[0],
    ",
  0x4005c0e1u64 => "
      ADC_140.adsstr()[1],
    ",
  0x4005c0e2u64 => "
      ADC_140.adsstr()[2],
    ",
  0x4005c0e3u64 => "
      ADC_140.adsstr()[3],
    ",
  0x4005c0e4u64 => "
      ADC_140.adsstr()[4],
    ",
  0x4005c0e5u64 => "
      ADC_140.adsstr()[5],
    ",
  0x4005c0e6u64 => "
      ADC_140.adsstr()[6],
    ",
  0x4005c0e7u64 => "
      ADC_140.adsstr()[7],
    ",
  0x4005c0e8u64 => "
      ADC_140.adsstr()[8],
    ",
  0x4005c0e9u64 => "
      ADC_140.adsstr()[9],
    ",
  0x4005c0eau64 => "
      ADC_140.adsstr()[10],
    ",
  0x4005c0ebu64 => "
      ADC_140.adsstr()[11],
    ",
  0x4005c0ecu64 => "
      ADC_140.adsstr()[12],
    ",
  0x4005c0edu64 => "
      ADC_140.adsstr()[13],
    ",
  0x4005c0eeu64 => "
      ADC_140.adsstr()[14],
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
  0x40078200u64 => "
      GPT_162.gtwp(),
    ",
  0x40078204u64 => "
      GPT_162.gtstr(),
    ",
  0x40078208u64 => "
      GPT_162.gtstp(),
    ",
  0x4007820cu64 => "
      GPT_162.gtclr(),
    ",
  0x40078210u64 => "
      GPT_162.gtssr(),
    ",
  0x40078214u64 => "
      GPT_162.gtpsr(),
    ",
  0x40078218u64 => "
      GPT_162.gtcsr(),
    ",
  0x4007821cu64 => "
      GPT_162.gtupsr(),
    ",
  0x40078220u64 => "
      GPT_162.gtdnsr(),
    ",
  0x40078224u64 => "
      GPT_162.gticasr(),
    ",
  0x40078228u64 => "
      GPT_162.gticbsr(),
    ",
  0x4007822cu64 => "
      GPT_162.gtcr(),
    ",
  0x40078230u64 => "
      GPT_162.gtuddtyc(),
    ",
  0x40078234u64 => "
      GPT_162.gtior(),
    ",
  0x40078238u64 => "
      GPT_162.gtintad(),
    ",
  0x4007823cu64 => "
      GPT_162.gtst(),
    ",
  0x40078240u64 => "
      GPT_162.gtber(),
    ",
  0x40078248u64 => "
      GPT_162.gtcnt(),
    ",
  0x4007824cu64 => "
      GPT_162.gtccra(),
    ",
  0x40078250u64 => "
      GPT_162.gtccrb(),
    ",
  0x40078254u64 => "
      GPT_162.gtccrc(),
    ",
  0x40078258u64 => "
      GPT_162.gtccre(),
    ",
  0x4007825cu64 => "
      GPT_162.gtccrd(),
    ",
  0x40078260u64 => "
      GPT_162.gtccrf(),
    ",
  0x40078264u64 => "
      GPT_162.gtpr(),
    ",
  0x40078268u64 => "
      GPT_162.gtpbr(),
    ",
  0x40078288u64 => "
      GPT_162.gtdtcr(),
    ",
  0x4007828cu64 => "
      GPT_162.gtdvu(),
    ",
  0x40078ff0u64 => "
      GPT_OPS.opscr(),
    ",
};
