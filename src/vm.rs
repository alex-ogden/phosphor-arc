#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::cpu::Cpu;
use crate::char_rom::CharacterRom;
use crate::vpu::Vpu;

pub struct PhosphorArc {
    pub cpu: Cpu,
    pub bus: PhosphorArcBus,
}

impl PhosphorArc {
    pub fn new(debug_enabled: &bool) -> Self {
        Self {
            cpu: Cpu::new(debug_enabled),
            bus: PhosphorArcBus::new(),
        }
    }
}

// Bus trait for reading and writing to memory
pub trait Bus {
    fn read_word(&mut self, addr: u16) -> u8;
    fn write_word(&mut self, addr: u16, val: u8);
    fn read_char(&mut self, ascii_code: u8, row: u8) -> u8;
}

pub struct PhosphorArcBus {
    pub ram:            Vec<u8>,        // System RAM
    pub bios:           Vec<u8>,        // 4KB Read-only boot ROM
    pub char_rom:       CharacterRom,   // 5440-byte character ROM
    pub cycle_counter:  u64,            // CPU cycle counter
    pub vpu:            Vpu,            // Video processing unit
}

impl PhosphorArcBus {
    pub fn new() -> Self {
        Self {
            ram:            vec![0; 65536],
            bios:           vec![0; 4096],
            char_rom:       CharacterRom::new(),
            cycle_counter:  0,
            vpu:            Vpu::new(),
        }
    }
}

impl Bus for PhosphorArcBus {
    fn read_word(&mut self, addr: u16) -> u8 { todo!(); }
    fn write_word(&mut self, addr: u16, val: u8) { todo!(); }

    // Reads a character from character ROM
    fn read_char(&mut self, ascii_code: u8, row: u8) -> u8 {
        self.char_rom.read_row(ascii_code, row)
    }
}
