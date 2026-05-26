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
            0x07 => self.load_rr_ind_rr_ind(dest, src, bus),
            0x08 => self.load_rr_ind_nn_ind(dest, bus),
            0x09 => self.load_nn_ind_rr(src, bus),
            0x0A => self.load_nn_ind_nn(bus),
            0x0B => self.load_nn_ind_rr_ind(src, bus),
            0x0C => self.load_nn_ind_nn_ind(bus),
            0x0D => self.load_acc_rr(src),
            0x0E => self.load_acc_nn(bus),
            0x0F => self.load_acc_rr_ind(src, bus),
            0x10 => self.load_acc_nn_ind(bus),
            0x11 => self.load_rrrr_rrrr(dest, src),
            0x12 => self.load_rrrr_nnnn(dest, bus),
            0x13 => self.load_rrrr_rr_ind(dest, src, bus),
            0x14 => self.load_rrrr_nn_ind(dest, bus),
            0x15 => self.load_rr_ind_rrrr(dest, src, bus),
            0x16 => self.load_rr_ind_nnnn(dest, bus),
            0x17 => self.load_nn_ind_rrrr(src, bus),
            0x18 => self.load_nn_ind_nnnn(bus),
            0x19 => self.load_nn_ind_rr_ind(src, bus),
            0x1A => self.load_nn_ind_nn_ind(bus),
            0x1B => self.load_accp_rrrr(src),
            0x1C => self.load_accp_nnnn(bus),
            0x1D => self.load_accp_rr_ind(src, bus),
            0x1E => self.load_accp_nn_ind(bus),
            0x1F => self.loadr_rr_rr_ind(dest, src, bus),
            0x20 => self.loadr_rr_nn_ind(dest, bus),
            0x21 => self.loadr_acc_rr_ind(dest, bus),
            0x22 => self.loadr_acc_nn_ind(bus),
            0x23 => self.loadr_rrrr_rr_ind(dest, src, bus),
            0x24 => self.loadr_rrrr_nn_ind(dest, bus),
            0x25 => self.loadr_accp_rr_ind(src, bus),
            0x26 => self.loadr_accp_nn_ind(bus),
            0x27 => self.add_acc_rr(src),
            0x28 => self.add_acc_nn(bus),
            0x29 => self.add_acc_rr_ind(src, bus),
            0x2A => self.add_acc_nn_ind(bus),
            0x2B => self.add_accp_rrrr(src),
            0x2C => self.add_accp_nnnn(bus),
            0x2D => self.add_accp_rr_ind(src, bus),
            0x2E => self.add_accp_nn_ind(bus),
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
        let src_val = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);

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
        let src_addr = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
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
        let src_val = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);

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

    fn load_rr_ind_rr_ind(&mut self, dest: u8, src: u8, bus: &mut impl Bus) {
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

    fn load_rr_ind_nn_ind(&mut self, dest: u8, bus: &mut impl Bus) {
        let src_addr = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let src_val = bus.read_ram_word(src_addr);

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

    fn load_nn_ind_rr(&mut self, src: u8, bus: &mut impl Bus) {
        let dest_addr = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);

        match src {
            0x01 => bus.write_ram_word(dest_addr, self.r0),
            0x02 => bus.write_ram_word(dest_addr, self.r1),
            0x03 => bus.write_ram_word(dest_addr, self.r2),
            0x04 => bus.write_ram_word(dest_addr, self.r3),
            0x05 => bus.write_ram_word(dest_addr, self.r4),
            0x06 => bus.write_ram_word(dest_addr, self.r5),
            0x07 => bus.write_ram_word(dest_addr, self.r6),
            0x08 => bus.write_ram_word(dest_addr, self.r7),
            _ => unreachable!(),
        }
    }

    fn load_nn_ind_nn(&mut self, bus: &mut impl Bus) {
        let dest_addr = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let src_val = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);

        bus.write_ram_word(dest_addr, src_val);
    }
    
    fn load_nn_ind_rr_ind(&mut self, src: u8, bus: &mut impl Bus) {
        let dest_addr = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
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

        bus.write_ram_word(dest_addr, src_val);
    }
    
    fn load_nn_ind_nn_ind(&mut self, bus: &mut impl Bus) {
        let dest_addr = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let src_addr = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let src_val = bus.read_ram_word(src_addr);

        bus.write_ram_word(dest_addr, src_val);
    }
    
    fn load_acc_rr(&mut self, src: u8) {
        match src {
            0x01 => self.acc = self.r0,
            0x02 => self.acc = self.r1,
            0x03 => self.acc = self.r2,
            0x04 => self.acc = self.r3,
            0x05 => self.acc = self.r4,
            0x06 => self.acc = self.r5,
            0x07 => self.acc = self.r6,
            0x08 => self.acc = self.r7,
            _ => unreachable!(),
        }
    }
    
    fn load_acc_nn(&mut self, bus: &mut impl Bus) {
        let src_val = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        self.acc = src_val;
    }
    
    fn load_acc_rr_ind(&mut self, src: u8, bus: &mut impl Bus) {
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

        self.acc = src_val;
    }
    
    fn load_acc_nn_ind(&mut self, bus: &mut impl Bus) {
        let src_addr = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let src_val = bus.read_ram_word(src_addr);

        self.acc = src_val;
    }
    
    fn load_rrrr_rrrr(&mut self, dest: u8, src: u8) {
        let src_val = match src {
            0x00 => self.get_r01(),
            0x02 => self.get_r23(),
            0x04 => self.get_r45(),
            0x06 => self.get_r67(),
            _ => unreachable!(),
        };

        match dest {
            0x00 => self.set_r01(src_val),
            0x02 => self.set_r23(src_val),
            0x04 => self.set_r45(src_val),
            0x06 => self.set_r67(src_val),
            _ => unreachable!(),
        }
    }
    
    fn load_rrrr_nnnn(&mut self, dest: u8, bus: &mut impl Bus) {
        let src_lo = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let src_hi = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let src_val = (src_hi as u32) << 16 | src_lo as u32;

        match dest {
            0x00 => self.set_r01(src_val),
            0x02 => self.set_r23(src_val),
            0x04 => self.set_r45(src_val),
            0x06 => self.set_r67(src_val),
            _ => unreachable!(),
        }
    }
    
    fn load_rrrr_rr_ind(&mut self, dest: u8, src: u8, bus: &mut impl Bus) {
        let (src_lo, src_hi) = match src {
            0x01 => (bus.read_ram_word(self.r0), bus.read_ram_word(self.r0.wrapping_add(2))),
            0x02 => (bus.read_ram_word(self.r1), bus.read_ram_word(self.r1.wrapping_add(2))),
            0x03 => (bus.read_ram_word(self.r2), bus.read_ram_word(self.r2.wrapping_add(2))),
            0x04 => (bus.read_ram_word(self.r3), bus.read_ram_word(self.r3.wrapping_add(2))),
            0x05 => (bus.read_ram_word(self.r4), bus.read_ram_word(self.r4.wrapping_add(2))),
            0x06 => (bus.read_ram_word(self.r5), bus.read_ram_word(self.r5.wrapping_add(2))),
            0x07 => (bus.read_ram_word(self.r6), bus.read_ram_word(self.r6.wrapping_add(2))),
            0x08 => (bus.read_ram_word(self.r7), bus.read_ram_word(self.r7.wrapping_add(2))),
            _ => unreachable!(),
        };
        let src_val = (src_hi as u32) << 16 | src_lo as u32;

        match dest {
            0x00 => self.set_r01(src_val),
            0x02 => self.set_r23(src_val),
            0x04 => self.set_r45(src_val),
            0x06 => self.set_r67(src_val),
            _ => unreachable!(),
        }
    }
    
    fn load_rrrr_nn_ind(&mut self, dest: u8, bus: &mut impl Bus) {
        let src_addr = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let (src_lo, src_hi) = (bus.read_ram_word(src_addr), bus.read_rom_word(src_addr.wrapping_add(2)));
        let src_val = (src_hi as u32) << 16 | src_lo as u32;
        
        match dest {
            0x00 => self.set_r01(src_val),
            0x02 => self.set_r23(src_val),
            0x04 => self.set_r45(src_val),
            0x06 => self.set_r67(src_val),
            _ => unreachable!(),
        }
    }
    
    fn load_rr_ind_rrrr(&mut self, dest: u8, src: u8, bus: &mut impl Bus) {
        let src_val = match src {
            0x00 => self.get_r01(),
            0x02 => self.get_r23(),
            0x04 => self.get_r45(),
            0x06 => self.get_r67(),
            _ => unreachable!(),
        };

        let (src_hi, src_lo) = ((src_val >> 16) as u16, src_val as u16);

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

        bus.write_ram_word(dest_addr, src_lo);
        bus.write_ram_word(dest_addr.wrapping_add(2), src_hi);
    }
    
    fn load_rr_ind_nnnn(&mut self, dest: u8, bus: &mut impl Bus) {
        let src_lo = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let src_hi = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);

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

        bus.write_ram_word(dest_addr, src_lo);
        bus.write_ram_word(dest_addr.wrapping_add(2), src_hi);
    }
    
    fn load_nn_ind_rrrr(&mut self, src: u8, bus: &mut impl Bus) {
        let dest_addr = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let src_val = match src {
            0x00 => self.get_r01(),
            0x02 => self.get_r23(),
            0x04 => self.get_r45(),
            0x06 => self.get_r67(),
            _ => unreachable!(),
        };
        let (src_hi, src_lo) = ((src_val >> 16) as u16, src_val as u16);
        
        bus.write_ram_word(dest_addr, src_lo);
        bus.write_ram_word(dest_addr.wrapping_add(2), src_hi);
    }
    
    fn load_nn_ind_nnnn(&mut self, bus: &mut impl Bus) {
        let dest_addr = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let src_lo = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let src_hi = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);

        bus.write_ram_word(dest_addr, src_lo);
        bus.write_ram_word(dest_addr.wrapping_add(2), src_hi);
    }
    
    fn load_accp_rrrr(&mut self, src: u8) {
        let src_val = match src {
            0x00 => self.get_r01(),
            0x02 => self.get_r23(),
            0x04 => self.get_r45(),
            0x06 => self.get_r67(),
            _ => unreachable!(),
        };

        self.set_acc_pair(src_val);
    }
    
    fn load_accp_nnnn(&mut self, bus: &mut impl Bus) {
        let src_lo = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let src_hi = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let src_val = (src_hi as u32) << 16 | src_lo as u32;

        self.set_acc_pair(src_val);
    }
    
    fn load_accp_rr_ind(&mut self, src: u8, bus: &mut impl Bus) {
        let (src_lo, src_hi) = match src {
            0x01 => (bus.read_ram_word(self.r0), bus.read_ram_word(self.r0.wrapping_add(2))),
            0x02 => (bus.read_ram_word(self.r1), bus.read_ram_word(self.r1.wrapping_add(2))),
            0x03 => (bus.read_ram_word(self.r2), bus.read_ram_word(self.r2.wrapping_add(2))),
            0x04 => (bus.read_ram_word(self.r3), bus.read_ram_word(self.r3.wrapping_add(2))),
            0x05 => (bus.read_ram_word(self.r4), bus.read_ram_word(self.r4.wrapping_add(2))),
            0x06 => (bus.read_ram_word(self.r5), bus.read_ram_word(self.r5.wrapping_add(2))),
            0x07 => (bus.read_ram_word(self.r6), bus.read_ram_word(self.r6.wrapping_add(2))),
            0x08 => (bus.read_ram_word(self.r7), bus.read_ram_word(self.r7.wrapping_add(2))),
            _ => unreachable!(),
        };
        let src_val = (src_hi as u32) << 16 | src_lo as u32;

        self.set_acc_pair(src_val);
    }
    
    fn load_accp_nn_ind(&mut self, bus: &mut impl Bus) {
        let src_addr = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let (src_lo, src_hi) = (bus.read_ram_word(src_addr), bus.read_ram_word(src_addr.wrapping_add(2)));
        let src_val = (src_hi as u32) << 16 | src_lo as u32;

        self.set_acc_pair(src_val);
    }
    
    fn loadr_rr_rr_ind(&mut self, dest: u8, src: u8, bus: &mut impl Bus) {
        let src_addr = match src {
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
        let src_val = bus.read_rom_word(src_addr);

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
    
    fn loadr_rr_nn_ind(&mut self, dest: u8, bus: &mut impl Bus) {
        let src_addr = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let src_val = bus.read_rom_word(src_addr);

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
    
    fn loadr_acc_rr_ind(&mut self, dest: u8, bus: &mut impl Bus) {
        let src_val = match dest {
            0x01 => bus.read_rom_word(self.r0),
            0x02 => bus.read_rom_word(self.r1),
            0x03 => bus.read_rom_word(self.r2),
            0x04 => bus.read_rom_word(self.r3),
            0x05 => bus.read_rom_word(self.r4),
            0x06 => bus.read_rom_word(self.r5),
            0x07 => bus.read_rom_word(self.r6),
            0x08 => bus.read_rom_word(self.r7),
            _ => unreachable!(),
        };
        
        self.acc = src_val;
    }
    
    fn loadr_acc_nn_ind(&mut self, bus: &mut impl Bus) {
        let src_addr = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let src_val = bus.read_rom_word(src_addr);
        self.acc = src_val;
    }
    
    fn loadr_rrrr_rr_ind(&mut self, dest: u8, src: u8, bus: &mut impl Bus) {
        let src_addr = match src {
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
        let (src_lo, src_hi) = (bus.read_rom_word(src_addr), bus.read_rom_word(src_addr.wrapping_add(2)));
        let src_val = (src_hi as u32) << 16 | src_lo as u32;

        match dest {
            0x00 => self.set_r01(src_val),
            0x02 => self.set_r23(src_val),
            0x04 => self.set_r45(src_val),
            0x06 => self.set_r67(src_val),
            _ => unreachable!(),
        }
    }
    
    fn loadr_rrrr_nn_ind(&mut self, dest: u8, bus: &mut impl Bus) {
        let src_addr = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let (src_lo, src_hi) = (bus.read_rom_word(src_addr), bus.read_rom_word(src_addr.wrapping_add(2)));
        let src_val = (src_hi as u32) << 16 | src_lo as u32;

        match dest {
            0x00 => self.set_r01(src_val),
            0x02 => self.set_r23(src_val),
            0x04 => self.set_r45(src_val),
            0x06 => self.set_r67(src_val),
            _ => unreachable!(),
        }
    }
    
    fn loadr_accp_rr_ind(&mut self, src: u8, bus: &mut impl Bus) {
        let src_addr = match src {
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
        let (src_lo, src_hi) = (bus.read_rom_word(src_addr), bus.read_rom_word(src_addr.wrapping_add(2)));
        let src_val = (src_hi as u32) << 16 | src_lo as u32;

        self.set_acc_pair(src_val);
    }
    
    fn loadr_accp_nn_ind(&mut self, bus: &mut impl Bus) {
        let src_addr = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let (src_lo, src_hi) = (bus.read_rom_word(src_addr), bus.read_rom_word(src_addr.wrapping_add(2)));
        let src_val = (src_hi as u32) << 16 | src_lo as u32;

        self.set_acc_pair(src_val);
    }
    
    fn add_acc_rr(&mut self, src: u8) {
        let acc = self.acc;
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

        let result = self.acc.wrapping_add(src_val);

        self.acc = result;

        self.set_flag(FLAG_S, result & 0x8000 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (acc & 0xF) + (src_val & 0xF) > 0xF);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, (acc as u32) + (src_val as u32) > 0xFFFF);
    }

    fn add_acc_nn(&mut self, bus: &mut impl Bus) {
        let acc = self.acc;
        let src_val = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let result = self.acc.wrapping_add(src_val);
    
        self.acc = result;

        self.set_flag(FLAG_S, result & 0x8000 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (acc & 0xF) + (src_val & 0xF) > 0xF);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, (acc as u32) + (src_val as u32) > 0xFFFF);
    }
    
    fn add_acc_rr_ind(&mut self, src: u8, bus: &mut impl Bus) {
        let acc = self.acc;
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

        let result = self.acc.wrapping_add(src_val);
        
        self.acc = result;


        self.set_flag(FLAG_S, result & 0x8000 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (acc & 0xF) + (src_val & 0xF) > 0xF);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, (acc as u32) + (src_val as u32) > 0xFFFF);
    }
    
    fn add_acc_nn_ind(&mut self, bus: &mut impl Bus) {
        let acc = self.acc;
        let src_addr = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let src_val = bus.read_ram_word(src_addr);

        let result = self.acc.wrapping_add(2);

        self.acc = result;

        self.set_flag(FLAG_S, result & 0x8000 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (acc & 0xF) + (src_val & 0xF) > 0xF);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, (acc as u32) + (src_val as u32) > 0xFFFF);
    }
    
    fn add_accp_rrrr(&mut self, src: u8) {
        let accp = self.get_acc_pair();
        let src_val = match src {
            0x00 => self.get_r01(),
            0x02 => self.get_r23(),
            0x04 => self.get_r45(),
            0x06 => self.get_r67(),
            _ => unreachable!(),
        };

        let result = self.get_acc_pair().wrapping_add(accp);

        self.set_acc_pair(result);

        self.set_flag(FLAG_S, result & 0x80000000 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (accp & 0xFFFF) + (src_val & 0xFFFF) > 0xFFFF);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, (accp as u64) + (src_val as u64) > 0xFFFFFFFF);
    }
    
    fn add_accp_nnnn(&mut self, bus: &mut impl Bus) {
        let accp = self.get_acc_pair();
        let src_lo = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let src_hi = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let src_val = (src_hi as u32) << 16 | src_lo as u32;

        let result = self.get_acc_pair().wrapping_add(src_val);

        self.set_acc_pair(result);

        self.set_flag(FLAG_S, result & 0x80000000 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (accp & 0xFFFF) + (src_val & 0xFFFF) > 0xFFFF);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, (accp as u64) + (src_val as u64) > 0xFFFFFFFF);
    }
    
    fn add_accp_rr_ind(&mut self, src: u8, bus: &mut impl Bus) {
        let accp = self.get_acc_pair();
        let src_addr = match src {
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
        let (src_lo, src_hi) = (bus.read_ram_word(src_addr), bus.read_ram_word(src_addr.wrapping_add(2)));
        let src_val = (src_hi as u32) << 16 | src_lo as u32;

        let result = self.get_acc_pair().wrapping_add(src_val);

        self.set_acc_pair(result);
        
        self.set_flag(FLAG_S, result & 0x80000000 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (accp & 0xFFFF) + (src_val & 0xFFFF) > 0xFFFF);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, (accp as u64) + (src_val as u64) > 0xFFFFFFFF);
    }
    
    fn add_accp_nn_ind(&mut self, bus: &mut impl Bus) {
        let accp = self.get_acc_pair();
        let src_addr = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let (src_lo, src_hi) = (bus.read_ram_word(src_addr), bus.read_ram_word(src_addr.wrapping_add(2)));
        let src_val = (src_hi as u32) << 16 | src_lo as u32;

        let result = self.get_acc_pair().wrapping_add(src_val);

        self.set_acc_pair(result);

        self.set_flag(FLAG_S, result & 0x80000000 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (accp & 0xFFFF) + (src_val & 0xFFFF) > 0xFFFF);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, (accp as u64) + (src_val as u64) > 0xFFFFFFFF);
    }
    
    fn addr_acc_rr_ind(&mut self, src:u8, bus: &mut impl Bus) {
        let acc = self.acc;
        
        let src_val = match src {
            0x01 => bus.read_rom_word(self.r0),
            0x02 => bus.read_rom_word(self.r1),
            0x03 => bus.read_rom_word(self.r2),
            0x04 => bus.read_rom_word(self.r3),
            0x05 => bus.read_rom_word(self.r4),
            0x06 => bus.read_rom_word(self.r5),
            0x07 => bus.read_rom_word(self.r6),
            0x08 => bus.read_rom_word(self.r7),
            _ => unreachable!(),
        };

        let result = self.acc.wrapping_add(src_val);

        self.acc = result;

        self.set_flag(FLAG_S, result & 0x8000 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (acc & 0xF) + (src_val & 0xF) > 0xF);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, (acc as u32) + (src_val as u32) > 0xFFFF);
    }
    
    fn addr_acc_nn_ind(&mut self, bus: &mut impl Bus) {
        let acc = self.acc;
        let src_addr = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let src_val = bus.read_rom_word(src_addr);

        let result = self.acc.wrapping_add(src_val);
        self.acc = result;

        self.set_flag(FLAG_S, result & 0x8000 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (acc & 0xF) + (src_val & 0xF) > 0xF);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, (acc as u32) + (src_val as u32) > 0xFFFF);
    }
    
    fn addr_accp_rr_ind(&mut self, src: u8, bus: &mut impl Bus) {
        let accp = self.get_acc_pair();
        let src_addr = match src {
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
        let (src_lo, src_hi) = (bus.read_rom_word(src_addr), bus.read_rom_word(src_addr.wrapping_add(2)));
        let src_val = (src_hi as u32) << 16 | src_lo as u32;

        let result = self.get_acc_pair().wrapping_add(src_val);
        self.set_acc_pair(result);

        self.set_flag(FLAG_S, result & 0x80000000 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (accp & 0xFFFF) + (src_val & 0xFFFF) > 0xFFFF);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, (accp as u64) + (src_val as u64) > 0xFFFFFFFF);
    }
    
    fn addr_accp_nn_ind(&mut self, bus: &mut impl Bus) {
        let accp = self.get_acc_pair();
        let src_addr = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let (src_lo, src_hi) = (bus.read_rom_word(src_addr), bus.read_rom_word(src_addr.wrapping_add(2)));
        let src_val = (src_hi as u32) << 16 | src_lo as u32;

        let result = self.get_acc_pair().wrapping_add(src_val);
        self.set_acc_pair(result);
        
        self.set_flag(FLAG_S, result & 0x80000000 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (accp & 0xFFFF) + (src_val & 0xFFFF) > 0xFFFF);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, (accp as u64) + (src_val as u64) > 0xFFFFFFFF);
    }
    
    fn adc_acc_rr(&mut self, src: u8) {
        let acc = self.acc;
        let carry = if self.get_flag(FLAG_C) { 1 } else { 0 };
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

        let result = self.acc.wrapping_add(src_val).wrapping_add(carry);

        self.set_flag(FLAG_S, result & 0x8000 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (acc & 0xF) + (src_val & 0xF) > 0xF);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, (acc as u32) + (src_val as u32) + (carry as u32) > 0xFFFF);
    }
    
    fn adc_acc_nn(&mut self, bus: &mut impl Bus) {
        let acc = self.acc;
        let carry = if self.get_flag(FLAG_C) { 1 } else { 0 };
        let src_val = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let result = self.acc.wrapping_add(src_val).wrapping_add(carry);

        self.acc = result;

        self.set_flag(FLAG_S, result & 0x8000 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (acc & 0xF) + (src_val & 0xF) > 0xF);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, (acc as u32) + (src_val as u32) + (carry as u32) > 0xFFFF);
    }
    
    fn adc_acc_rr_ind(&mut self, src: u8, bus: &mut impl Bus) {
        let acc = self.acc;
        let carry = if self.get_flag(FLAG_C) { 1 } else { 0 };
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
        let result = self.acc.wrapping_add(src_val).wrapping_add(carry);

        self.acc = result;

        self.set_flag(FLAG_S, result & 0x8000 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (acc & 0xF) + (src_val & 0xF) > 0xF);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, (acc as u32) + (src_val as u32) + (carry as u32) > 0xFFFF);
    }
    
    fn adc_acc_nn_ind(&mut self, bus: &mut impl Bus) {
        let acc = self.acc;
        let carry = if self.get_flag(FLAG_C) { 1 } else { 0 };
        let src_addr = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let src_val = bus.read_ram_word(src_addr);
        let result = self.acc.wrapping_add(src_val).wrapping_add(carry);

        self.acc = result;

        self.set_flag(FLAG_S, result & 0x8000 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (acc & 0xF) + (src_val & 0xF) > 0xF);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, (acc as u32) + (src_val as u32) + (carry as u32) > 0xFFFF);
    }
    
    fn adc_accp_rrrr(&mut self, src: u8) {
        let accp = self.get_acc_pair();
        let carry = if self.get_flag(FLAG_C) { 1 } else { 0 };
        let src_val = match src {
            0x00 => self.get_r01(),
            0x02 => self.get_r23(),
            0x04 => self.get_r45(),
            0x06 => self.get_r67(),
            _ => unreachable!(),
        };
        let result = self.get_acc_pair().wrapping_add(src_val).wrapping_add(carry);

        self.set_acc_pair(result);

        self.set_flag(FLAG_S, result & 0x80000000 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (accp & 0xFFFF) + (src_val & 0xFFFF) > 0xFFFF);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, (accp as u64) + (src_val as u64) + (carry as u64) > 0xFFFFFFFF);
    }
    
    fn adc_accp_nnnn(&mut self, bus: &mut impl Bus) {
        let accp = self.get_acc_pair();
        let carry = if self.get_flag(FLAG_C) { 1 } else { 0 };
        let src_lo = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let src_hi = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let src_val = (src_hi as u32) << 16 | src_lo as u32;
        let result = self.get_acc_pair().wrapping_add(src_val).wrapping_add(carry);

        self.set_acc_pair(result);

        self.set_flag(FLAG_S, result & 0x80000000 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (accp & 0xFFFF) + (src_val & 0xFFFF) > 0xFFFF);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, (accp as u64) + (src_val as u64) + (carry as u64) > 0xFFFFFFFF);
    }
    
    fn adc_accp_rr_ind(&mut self, src: u8, bus: &mut impl Bus) {
        let accp = self.get_acc_pair();
        let carry = if self.get_flag(FLAG_C) { 1 } else { 0 };
        let src_addr = match src {
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
        let (src_lo, src_hi) = (bus.read_ram_word(src_addr), bus.read_ram_word(src_addr.wrapping_add(2)));
        let src_val = (src_hi as u32) << 16 | src_lo as u32;
        let result = self.get_acc_pair().wrapping_add(src_val).wrapping_add(carry);

        self.set_acc_pair(result);

        self.set_flag(FLAG_S, result & 0x80000000 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (accp & 0xFFFF) + (src_val & 0xFFFF) > 0xFFFF);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, (accp as u64) + (src_val as u64) + (carry as u64) > 0xFFFFFFFF);
    }
    
    fn adc_accp_nn_ind(&mut self, bus: &mut impl Bus) {
        let accp = self.get_acc_pair();
        let carry = if self.get_flag(FLAG_C) { 1 } else { 0 };
        let src_addr = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let (src_lo, src_hi) = (bus.read_ram_word(src_addr), bus.read_ram_word(src_addr.wrapping_add(2)));
        let src_val = (src_hi as u32) << 16 | src_lo as u32;
        let result = self.get_acc_pair().wrapping_add(src_val).wrapping_add(carry);

        self.set_acc_pair(result);

        self.set_flag(FLAG_S, result & 0x80000000 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (accp & 0xFFFF) + (src_val & 0xFFFF) > 0xFFFF);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, (accp as u64) + (src_val as u64) + (carry as u64) > 0xFFFFFFFF);
    }
    
    fn adcr_acc_rr_ind(&mut self, src: u8, bus: &mut impl Bus) {
        let acc= self.acc;
        let carry = if self.get_flag(FLAG_C) { 1 } else { 0 };
        let src_val = match src {
            0x01 => bus.read_rom_word(self.r0),
            0x02 => bus.read_rom_word(self.r1),
            0x03 => bus.read_rom_word(self.r2),
            0x04 => bus.read_rom_word(self.r3),
            0x05 => bus.read_rom_word(self.r4),
            0x06 => bus.read_rom_word(self.r5),
            0x07 => bus.read_rom_word(self.r6),
            0x08 => bus.read_rom_word(self.r7),
            _ => unreachable!(),
        };
        let result = self.acc.wrapping_add(src_val).wrapping_add(carry);

        self.acc = result;

        self.set_flag(FLAG_S, result & 0x8000 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (acc & 0xFF) + (src_val & 0xFF) > 0xFF);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, (acc as u32) + (src_val as u32) + (carry as u32) > 0xFFFF);
    }
    
    fn adcr_acc_nn_ind(&mut self, bus: &mut impl Bus) {
        let acc = self.acc;
        let carry = if self.get_flag(FLAG_C) { 1 } else { 0 };
        let src_addr = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let src_val = bus.read_rom_word(src_addr);
        let result = self.acc.wrapping_add(src_val).wrapping_add(carry);

        self.acc = result;

        self.set_flag(FLAG_S, result & 0x8000 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (acc & 0xFF) + (src_val & 0xFF) > 0xFF);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, (acc as u32) + (src_val as u32) + (carry as u32) > 0xFFFF);
    }
    
    fn adcr_accp_rr_ind(&mut self, src: u8, bus: &mut impl Bus) {
        let accp = self.get_acc_pair();
        let carry = if self.get_flag(FLAG_C) { 1 } else { 0 };
        let src_addr = match src {
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
        let (src_lo, src_hi) = (bus.read_rom_word(src_addr), bus.read_rom_word(src_addr.wrapping_add(2)));
        let src_val = (src_hi as u32) << 16 | src_lo as u32;
        let result = self.get_acc_pair().wrapping_add(src_val).wrapping_add(carry);

        self.set_acc_pair(result);

        self.set_flag(FLAG_S, result & 0x80000000 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (accp & 0xFFFF) + (src_val & 0xFFFF) > 0xFFFF);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, (accp as u64) + (src_val as u64) + (carry as u64) > 0xFFFFFFFF);
    }
    
    fn adcr_accp_nn_ind(&mut self, bus: &mut impl Bus) {
        let accp = self.get_acc_pair();
        let carry = if self.get_flag(FLAG_C) { 1 } else { 0 };
        let src_addr = bus.read_rom_word(self.pc); self.pc = self.pc.wrapping_add(2);
        let (src_lo, src_hi) = (bus.read_rom_word(src_addr), bus.read_rom_word(src_addr.wrapping_add(2)));
        let src_val = (src_hi as u32) << 16 | src_lo as u32;
        let result = self.get_acc_pair().wrapping_add(src_val).wrapping_add(carry);

        self.set_acc_pair(result);

        self.set_flag(FLAG_S, result & 0x80000000 != 0);
        self.set_flag(FLAG_Z, result == 0);
        self.set_flag(FLAG_H, (accp & 0xFFFF) + (src_val & 0xFFFF) > 0xFFFF);
        self.set_flag(FLAG_N, false);
        self.set_flag(FLAG_C, (accp as u64) + (src_val as u64) + (carry as u64) > 0xFFFFFFFF);
    }
    
    fn sub_acc_rr() {}
    
    fn sub_acc_nn() {}
    
    fn sub_acc_rr_ind() {}
    
    fn sub_acc_nn_ind() {}
    
    fn sub_accp_rrrr() {}
    
    fn sub_accp_nnnn() {}
    
    fn sub_accp_rr_ind() {}
    
    fn sub_accp_nn_ind() {}
    
    fn subr_acc_rr_ind() {}
    
    fn subr_acc_nn_ind() {}
    
    fn subr_accp_rr_ind() {}
    
    fn subr_accp_nn_ind() {}
    
    fn sbc_acc_rr() {}
    
    fn sbc_acc_nn() {}
    
    fn sbc_acc_rr_ind() {}
    
    fn sbc_acc_nn_ind() {}
    
    fn sbc_accp_rrrr() {}
    
    fn sbc_accp_nnnn() {}
    
    fn sbc_accp_rr_ind() {}
    
    fn sbc_accp_nn_ind() {}
    
    fn sbcr_acc_rr_ind() {}
    
    fn sbcr_acc_nn_ind() {}
    
    fn sbcr_accp_rr_ind() {}
    
    fn sbcr_accp_nn_ind() {}
    
    fn inc_rr() {}
    
    fn inc_rr_ind() {}
    
    fn inc_nn_ind() {}
    
    fn incp_rrrr() {}
    
    fn incp_rr_ind() {}
    
    fn incp_nn_ind() {}
    
    fn dec_rr() {}
    
    fn dec_rr_ind() {}
    
    fn dec_nn_ind() {}
    
    fn decp_rrrr() {}
    
    fn decp_rr_ind() {}
    
    fn decp_nn_ind() {}
}
