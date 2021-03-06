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

use hifive::big::NLEN;
use super::super::arch::Chunk;
use types::{ModType, CurveType, CurvePairingType, SexticTwist, SignOfX};

// Base Bits= 29

// hifive Modulus
pub const MODULUS: [Chunk; NLEN] = [
    0x1FFFFFFD, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF,
    0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFF,
];
pub const R2MODP: [Chunk; NLEN] = [
    0x9000000, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
];
pub const MCONST: Chunk = 0x3;

// hifive Curve
pub const CURVE_COF_I: isize = 8;
pub const CURVE_A: isize = 1;
pub const CURVE_B_I: isize = 11111;
pub const CURVE_COF: [Chunk; NLEN] = [0x8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
pub const CURVE_B: [Chunk; NLEN] = [
    0x2B67, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
];
pub const CURVE_ORDER: [Chunk; NLEN] = [
    0x1E9FA805, 0x197CACB9, 0x1E4EEA9E, 0x17AD70F, 0x1FA9850C, 0x38A0A, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x4000,
];
pub const CURVE_GX: [Chunk; NLEN] = [0xC, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
pub const CURVE_GY: [Chunk; NLEN] = [
    0x5FE8632, 0x15F63428, 0xD976C4, 0x1AACA194, 0x35B6DB5, 0x8E3F7A, 0x52D1B0E, 0xF0A7A36,
    0x1C161D00, 0x8170C70, 0x1185AD59, 0x181B,
];

pub const MODBYTES: usize = 42;
pub const BASEBITS: usize = 29;

pub const MODBITS: usize = 336;
pub const MOD8: usize = 5;
pub const MODTYPE: ModType = ModType::PSEUDO_MERSENNE;
pub const SH: usize = 12;

pub const CURVETYPE: CurveType = CurveType::EDWARDS;
pub const CURVE_PAIRING_TYPE: CurvePairingType = CurvePairingType::NOT;
pub const SEXTIC_TWIST: SexticTwist = SexticTwist::NOT;
pub const ATE_BITS: usize = 0;
pub const SIGN_OF_X: SignOfX = SignOfX::NOT;
pub const HASH_TYPE: usize = 48;
pub const AESKEY: usize = 24;
