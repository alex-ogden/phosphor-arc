#![cfg_attr(rustfmt, rustfmt_skip)]
use anyhow::{anyhow, Result};

pub struct Memory {
    pub ram: [u8; 0xFFFF],  // 64KB RAM
    pub rom: [u8; 0x1000],  // 4KB ROM
}

impl Memory {
    pub fn new() -> Self {
        Self {
            ram: [0u8; 0xFFFF],
            rom: [0u8; 0x1000],
        }
    }

    pub fn load_rom(&mut self, rom_data: &[u8]) -> Result<()> {
        let len = rom_data.len().min(0x1000);   // Ensure ROM is at least 4KB
        if len > self.rom.len() {
            return Err(anyhow!("Binary file too large: expected {} bytes, got {} bytes", len, self.rom.len()));
        }

        self.rom[..len].copy_from_slice(&rom_data[..len]);
        Ok(())
    }
}
