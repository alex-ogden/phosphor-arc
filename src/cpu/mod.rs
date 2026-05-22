#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::vm::Bus;

pub struct Cpu {
    pub pc:             u16,        // Program Counter
    pub sp:             u16,        // Stack Pointer
    pub acc_reg:        u16,        // Accumulator Register
    pub gp_reg:         [u16; 8],   // General-purpose Registers (8)
    pub ie_pending:     bool,       // Interrupt-enable pending (interrupt coming next cycle)
    pub is_halted:      bool,       // Is CPU halted?
    pub debug_enabled:  bool,       // Is debug mode enabled?
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            pc:             0x0000,
            sp:             0xFFFF,
            acc_reg:        0x0000,
            gp_reg:         [0u16; 8],
            ie_pending:     false,
            is_halted:      false,
            debug_enabled:  false,
        }
    }

    pub fn step(&mut self, bus: &mut impl Bus) -> u64 {
        // Load next instruction from PC
        // Increment PC
        // Decode instruction
        // Execute
        // Return number of cycles taken
        todo!();
    }
}
