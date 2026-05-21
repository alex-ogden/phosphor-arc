#![cfg_attr(rustfmt, rustfmt_skip)]
pub mod flags {
    pub const FLAG_C: u8    = 0x08; // bit 3 - Carry
    pub const FLAG_N: u8    = 0x10; // bit 4 - Subtract
    pub const FLAG_H: u8    = 0x20; // bit 5 - Half carry
    pub const FLAG_Z: u8    = 0x40; // bit 6 - Zero
    pub const FLAG_S: u8    = 0x80; // bit 7 - Sign
}

pub struct Cpu {
    pub pc:             u16,
    pub sp:             u16,
    pub flag_reg:       u16,
    pub gp_registers:   [u16; 8],
    pub is_halted:      bool,
    pub debug_enabled:  bool,
}

impl Cpu {
    pub fn new(debug_enabled: &bool) -> Self {
        Self {
            pc:             0x0000,
            sp:             0xFFFF,
            flag_reg:       0x0000,
            gp_registers:   [0u16; 8],
            is_halted:      false,
            debug_enabled:  *debug_enabled,
        }
    }
}
