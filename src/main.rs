#![cfg_attr(rustfmt, rustfmt_skip)]
// External imports
use std::{
    env,
    fs,
    process,
    time::{
        Duration,
        Instant
    },
};

// Internal imports
use crate::vm::PhosphorArc;

// Modules
mod cpu;
mod vm;
mod memory;

// Constants
const FRAME_DURATION_MS: u64 = 16; // 60FPS target

fn main() {
    // User must provide at least a binary file (to begin with)
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <bin file> [--debug]", &args[0]);
        process::exit(1);
    }

    // Check if debug mode is enabled
    let debug_enabled = args.contains(&"--debug".to_string());
    if debug_enabled {
        println!("Debug output enabled");
        // Remove --debug from arguments
        args.retain(|arg| arg != "--debug");
    }

    // Load binary
    let bin_data = match fs::read(&args[1]) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("error reading binary file: {}", e);
            process::exit(1);
        }
    };

    // Create an instance of the VM
    let mut vm = PhosphorArc::new();

    // Set debug if enabled
    vm.cpu.debug_enabled = debug_enabled;

    // Load binary data into ROM
    let _ = match vm.bus.mem.load_rom(&bin_data) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Failed to load binary data into ROM: {}", e);
            process::exit(1);
        }
    };
    // Main emulator loop
    loop {
        // Start loop timer
        let start = Instant::now();

        // Run a frames-worth of CPU cycles
        vm.run_frame();

        // Check if we have to sleep to keep to 60FPS
        let elapsed = start.elapsed().as_millis() as u64;
        if elapsed < FRAME_DURATION_MS {
            std::thread::sleep(Duration::from_millis(FRAME_DURATION_MS - elapsed));
        }
    }
}
