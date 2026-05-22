#![cfg_attr(rustfmt, rustfmt_skip)]

use super::Arc16Cpu;
use super::flags::*;
use crate::vm::Bus;

impl Arc16Cpu {
    pub(super) fn decode_and_execute(&mut self, instr: u16, bus: &mut impl Bus) {
        let opcode  = ((instr & 0xFF00) >> 8) as u8;
        let dest    = ((instr & 0x00F0) >> 4) as u8;
        let src     = (instr & 0x000F) as u8;

        match opcode {
            0x01 => self.load_rr_rr(dest, src),
            0x02 => self.load_rr_nn(dest, bus),
            0x03 => self.load_rr_rr_ind(dest, src, bus),
            0x04 => self.load_rr_nn_ind(dest, bus),
            0x05 => self.load_rr_ind_rr(dest, src, bus),
            0x06 => self.load_rr_ind_nn(dest, bus),
            _ => {
                if self.debug_enabled {
                    eprintln!("Unimplemented instruction: {:04X} (opcode: {:02X})", instr, opcode);
                }
            }
        }
    }

    fn load_rr_rr(&mut self, dest: u8, src: u8) {
        let src_val = match src {
            0x01 => self.r0,
            0x02 => self.r1,
            0x03 => self.r2,
            0x04 => self.r3,
            0x05 => self.r4,
            0x06 => self.r5,
            0x07 => self.r6,
            0x08 => self.r7,
            _ => unreachable!(),
        };

        match dest {
            0x01 => self.r0 = src_val,
            0x02 => self.r1 = src_val,
            0x03 => self.r2 = src_val,
            0x04 => self.r3 = src_val,
            0x05 => self.r4 = src_val,
            0x06 => self.r5 = src_val,
            0x07 => self.r6 = src_val,
            0x08 => self.r7 = src_val,
            _ => unreachable!(),
        }
    }

    fn load_rr_nn(&mut self, dest: u8, bus: &mut impl Bus) {
        let src_val = bus.read_rom_word(self.pc); 
        self.pc = self.pc.wrapping_add(2);

        match dest {
            0x01 => self.r0 = src_val,
            0x02 => self.r1 = src_val,
            0x03 => self.r2 = src_val,
            0x04 => self.r3 = src_val,
            0x05 => self.r4 = src_val,
            0x06 => self.r5 = src_val,
            0x07 => self.r6 = src_val,
            0x08 => self.r7 = src_val,
            _ => unreachable!(),
        }
    }

    fn load_rr_rr_ind(&mut self, dest: u8, src: u8, bus: &mut impl Bus) {
        let src_val = match src {
            0x01 => bus.read_ram_word(self.r0),
            0x02 => bus.read_ram_word(self.r1),
            0x03 => bus.read_ram_word(self.r2),
            0x04 => bus.read_ram_word(self.r3),
            0x05 => bus.read_ram_word(self.r4),
            0x06 => bus.read_ram_word(self.r5),
            0x07 => bus.read_ram_word(self.r6),
            0x08 => bus.read_ram_word(self.r7),
            _ => unreachable!(),
        };

        match dest {
            0x01 => self.r0 = src_val,
            0x02 => self.r1 = src_val,
            0x03 => self.r2 = src_val,
            0x04 => self.r3 = src_val,
            0x05 => self.r4 = src_val,
            0x06 => self.r5 = src_val,
            0x07 => self.r6 = src_val,
            0x08 => self.r7 = src_val,
            _ => unreachable!(),
        }
    }

    fn load_rr_nn_ind(&mut self, dest: u8, bus: &mut impl Bus) {
        let src_addr = bus.read_rom_word(self.pc);
        self.pc = self.pc.wrapping_add(2);
        let src_val = bus.read_ram_word(src_addr);

        match dest {
            0x01 => self.r0 = src_val,
            0x02 => self.r1 = src_val,
            0x03 => self.r2 = src_val,
            0x04 => self.r3 = src_val,
            0x05 => self.r4 = src_val,
            0x06 => self.r5 = src_val,
            0x07 => self.r6 = src_val,
            0x08 => self.r7 = src_val,
            _ => unreachable!(),
        }
    }

    fn load_rr_ind_rr(&mut self, dest: u8, src: u8, bus: &mut impl Bus) {
        let src_val = match src {
            0x01 => self.r0,
            0x02 => self.r1,
            0x03 => self.r2,
            0x04 => self.r3,
            0x05 => self.r4,
            0x06 => self.r5,
            0x07 => self.r6,
            0x08 => self.r7,
            _ => unreachable!(),
        };

        let dest_addr = match dest {
            0x01 => self.r0,
            0x02 => self.r1,
            0x03 => self.r2,
            0x04 => self.r3,
            0x05 => self.r4,
            0x06 => self.r5,
            0x07 => self.r6,
            0x08 => self.r7,
            _ => unreachable!(),
        };

        bus.write_ram_word(dest_addr, src_val);
    }

    fn load_rr_ind_nn(&mut self, dest: u8, bus: &mut impl Bus) {
        let src_val = bus.read_rom_word(self.pc);
        self.pc = self.pc.wrapping_add(2);

        match dest {
            0x01 => bus.write_ram_word(self.r0, src_val),
            0x02 => bus.write_ram_word(self.r1, src_val),
            0x03 => bus.write_ram_word(self.r2, src_val),
            0x04 => bus.write_ram_word(self.r3, src_val),
            0x05 => bus.write_ram_word(self.r4, src_val),
            0x06 => bus.write_ram_word(self.r5, src_val),
            0x07 => bus.write_ram_word(self.r6, src_val),
            0x08 => bus.write_ram_word(self.r7, src_val),
            _ => unreachable!(),
        }
    }
}
