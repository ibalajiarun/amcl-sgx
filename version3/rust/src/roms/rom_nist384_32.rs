/*
Licensed to the Apache Software Foundation (ASF) under one
or more contributor license agreements.  See the NOTICE file
distributed with this work for additional information
regarding copyright ownership.  The ASF licenses this file
to you under the Apache License, Version 2.0 (the
"License"); you may not use this file except in compliance
with the License.  You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing,
software distributed under the License is distributed on an
"AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
KIND, either express or implied.  See the License for the
specific language governing permissions and limitations
under the License.
*/

use nist384::big::NLEN;
use arch::Chunk;
use std;
use types::{ModType, CurveType, CurvePairingType, SexticTwist, SignOfX};

// Base Bits= 29

// Base Bits= 29
// nist384 Modulus
pub const MODULUS: [Chunk; NLEN] = [
    0x1FFFFFFF, 0x7, 0x0, 0x1FFFFE00, 0x1FFFEFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF,
    0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x7F,
];
pub const R2MODP: [Chunk; NLEN] = [
    0x0, 0x8000, 0x1FF80000, 0x1FFFFF, 0x2000000, 0x0, 0x0, 0x1FFFFFFC, 0xF, 0x100, 0x400, 0x0,
    0x0, 0x0,
];
pub const MCONST: Chunk = 0x1;

// nist384 Curve
pub const CURVE_COF_I: isize = 1;
pub const CURVE_A: isize = -3;
pub const CURVE_B_I: isize = 0;
pub const CURVE_COF: [Chunk; NLEN] = [
    0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
];
pub const CURVE_B: [Chunk; NLEN] = [
    0x13EC2AEF, 0x142E476E, 0xBB4674A, 0xC731B14, 0x1875AC65, 0x447A809, 0x4480C50, 0xDDFD028,
    0x19181D9C, 0x1F1FC168, 0x623815A, 0x47DCFC9, 0x1312FA7E, 0x59,
];
pub const CURVE_ORDER: [Chunk; NLEN] = [
    0xCC52973, 0x760CB56, 0xC29DEBB, 0x141B6491, 0x12DDF581, 0x6C0FA1B, 0x1FFF1D8D, 0x1FFFFFFF,
    0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x7F,
];
pub const CURVE_GX: [Chunk; NLEN] = [
    0x12760AB7, 0x12A2F1C3, 0x154A5B0E, 0x5E4BB7E, 0x2A38550, 0xF0412A, 0xE6167DD, 0xC5174F3,
    0x146E1D3B, 0x1799056B, 0x3AC71C7, 0x1D160A6F, 0x87CA22B, 0x55,
];
pub const CURVE_GY: [Chunk; NLEN] = [
    0x10EA0E5F, 0x1218EBE4, 0x1FA0675E, 0x1639C3A, 0xB8C00A6, 0x1889DAF8, 0x11F3A768, 0x17A51342,
    0x9F8F41D, 0x1C9496E1, 0x1767A62F, 0xC4C58DE, 0x17DE4A9, 0x1B,
];

pub const MODBYTES: usize = 48;
pub const BASEBITS: usize = 29;

pub const MODBITS: usize = 384;
pub const MOD8: usize = 7;
pub const MODTYPE: ModType = ModType::NOT_SPECIAL;
pub const SH: usize = std::cmp::min(14, BASEBITS*(1+((8*MODBYTES-1)/BASEBITS))-MODBITS);

pub const CURVETYPE: CurveType = CurveType::WEIERSTRASS;
pub const CURVE_PAIRING_TYPE: CurvePairingType = CurvePairingType::NOT;
pub const SEXTIC_TWIST: SexticTwist = SexticTwist::NOT;
pub const SIGN_OF_X: SignOfX = SignOfX::NOT;
pub const HASH_TYPE: usize = 48;
pub const AESKEY: usize = 24;
