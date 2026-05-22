#![cfg_attr(rustfmt, rustfmt_skip)]
use crate::cpu::Arc16Cpu;
use crate::memory::Memory;

const CYCLES_PER_FRAME: usize = 166_666;

pub trait Bus {
    fn read_ram_word(&self, addr: u16) -> u16; // Reads two bytes from a memory location
    fn write_word(&mut self, addr: u16, val: u16); // Writes two bytes to a memory location
    fn read_rom_word(&self, addr: u16) -> u16;
}

pub struct PhosphorArc {
    pub cpu: Arc16Cpu,
    pub bus: PhosphorArcBus,
}

impl PhosphorArc {
    pub fn new() -> Self {
        Self {
            cpu: Arc16Cpu::new(),
            bus: PhosphorArcBus::new(),
        }
    }

    pub fn run_frame(&mut self) {
        while self.bus.cycle_counter < CYCLES_PER_FRAME as u64 {
            let cycles_consumed = self.cpu.step(&mut self.bus);
            self.bus.cycle_counter += cycles_consumed;
        }

        // Reset frames before returning
        self.bus.cycle_counter -= CYCLES_PER_FRAME as u64;
    }
}

pub struct PhosphorArcBus {
    pub mem: Memory,
    pub cycle_counter: u64,
}

impl PhosphorArcBus {
    pub fn new() -> Self {
        Self {
            mem: Memory::new(),
            cycle_counter: 0,
        }
    }
}

impl Bus for PhosphorArcBus {
    // Reads a 16-bit value from a starting address in memory
    fn read_ram_word(&self, addr: u16) -> u16 {
        let lo = self.mem.ram[addr as usize] as u16;
        let hi = self.mem.ram[addr.wrapping_add(1) as usize] as u16;

        hi << 8 | lo
    }

    // Takes a 16-bit value and writes it to memory at starting address
    fn write_word(&mut self, addr: u16, val: u16) {
        let hi = (val >> 8) as u8;
        let lo = val as u8;
        self.mem.ram[addr as usize] = lo;
        self.mem.ram[addr.wrapping_add(1) as usize] = hi;
    }

    fn read_rom_word(&self, addr: u16) -> u16 {
        let lo = self.mem.rom[addr as usize] as u16;
        let hi = self.mem.rom[addr.wrapping_add(1) as usize] as u16;

        hi << 8 | lo
    }
}
