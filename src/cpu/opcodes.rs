#![cfg_attr(rustfmt, rustfmt_skip)]

use super::Arc16Cpu;
use super::flags::*;
use crate::vm::Bus;

impl Arc16Cpu {
    pub(super) fn decode_and_execute(&mut self, instr: u16, bus: &mut impl Bus) -> u64 {
        let opcode      = instr & 0xF000 >> 12;
        let dst_reg     = instr & 0x0F00 >> 8;
        let src_reg_a   = instr & 0x00F0 >> 4;
        let src_reg_b   = instr & 0x000F;

        67
        // Opcode matching to come here
    }
}
