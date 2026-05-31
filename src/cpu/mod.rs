#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::vm::Bus;

mod opcodes;

pub mod flags {
    pub const FLAG_S: u8    = 0x80;     // Bit 7 - Sign
    pub const FLAG_Z: u8    = 0x40;     // Bit 6 - Zero
    pub const FLAG_H: u8    = 0x20;     // Bit 5 - Half-carry
    pub const FLAG_N: u8    = 0x10;     // Bit 4 - Subtract
    pub const FLAG_C: u8    = 0x08;     // Bit 3 - Carry
}

pub struct Arc16Cpu {
    // Special registers
    pub pc:             u16,        // Program Counter
    pub sp:             u16,        // Stack Pointer

    // Arithmetic and flag registers
    pub acc:            u16,        // Primary accumulator register
    pub acc2:           u16,        // Can be used for storage/arithmetic on 32-bit numbers
    pub flag:           u8,         // Flag register (only lower byte used currently)

    // General-purpose registers
    pub r:              [u16; 8],
    pub r0:             u16,
    pub r1:             u16,
    pub r2:             u16,
    pub r3:             u16,
    pub r4:             u16,
    pub r5:             u16,
    pub r6:             u16,
    pub r7:             u16,

    // VM state tracking
    pub ie_pending:     bool,       // Interrupt-enable pending (interrupt coming next cycle)
    pub is_halted:      bool,       // Is CPU halted?
    pub debug_enabled:  bool,       // Is debug mode enabled?
}

impl Arc16Cpu {
    pub fn new() -> Self {
        Self {
            pc:             0x0000,
            sp:             0xFFFF,

            acc:            0x0000,
            acc2:           0x0000,
            flag:           0x00,

            r:              [0; 8],
            r0:             0x0000,
            r1:             0x0000,
            r2:             0x0000,
            r3:             0x0000,
            r4:             0x0000,
            r5:             0x0000,
            r6:             0x0000,
            r7:             0x0000,

            ie_pending:     false,
            is_halted:      false,
            debug_enabled:  false,
        }
    }

    pub fn step(&mut self, bus: &mut impl Bus) {
        let instr = bus.read_rom_word(self.pc);
        self.pc = self.pc.wrapping_add(2);
        bus.tick(1);
        self.decode_and_execute(instr, bus)
    }

    fn get_flag(&self, flag: u8) -> bool {
        self.flag & flag != 0
    }

    fn set_flag(&mut self, flag: u8, condition: bool) {
        if condition {
            self.flag |= flag;
        } else {
            self.flag &= !flag;
        }
    }

    fn get_acc_pair(&self) -> u32 { (self.acc as u32) << 16 | self.acc2 as u32 }
    fn set_acc_pair(&mut self, val: u32) { self.acc = (val >> 16) as u16; self.acc2 = val as u16; }

    fn get_r_pair(&self, index: usize) -> u32 { (self.r[index] as u32) << 16 | self.r[index + 1] as u32 }
    fn set_r_pair(&mut self, index: usize, val: u32) { self.r[index] = (val >> 16) as u16; self.r[index + 1] = val as u16; }
}
