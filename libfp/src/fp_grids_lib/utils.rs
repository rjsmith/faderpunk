// Copyright (C) 2012 Emilie Gillet.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
//
// Based on the original Grids by Emilie Gillet.
// 
// Ported from https://github.com/thorinside/nt_grids/blob/main/nt_grids_utils.h
//
// Utility functions (Rust versions of avrlib/op.h functions)

pub fn u8_mix(a:u8, b: u8, balance:u8) -> u8 {
    return ((a as u16) * (255 - balance) + ((b as u16) * balance) >> 8) as u8;
}

pub fn u8_u8_mul_shift_8(a: u8, b: u8) -> u8 {
    return (((a as u16) * b) >> 8) as u8;
}

pub fn u8_u8_mul(a: u8, b: u8) -> u16 {
    return (a as u16) * b;
}