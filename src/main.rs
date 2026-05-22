use std::env;
use std::process;
use std::time::{Duration, Instant};

use crate::vm::PhosphorArc;

mod cpu;
mod vm;

const FRAME_DURATION_MS: u64 = 16; // 60FPS target

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <bin file> [--debug]", &args[0]);
        process::exit(1);
    }

    let debug_enabled = args.contains(&"--debug".to_string());
    if debug_enabled {
        println!("Debug output enabled");
    }

    // Create an instance of the VM
    let mut vm = PhosphorArc::new();

    // Set debug if enabled
    vm.cpu.debug_enabled = debug_enabled;

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
