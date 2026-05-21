#![cfg_attr(rustfmt, rustfmt_skip)]

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use std::process;
use std::time::{Duration, Instant};

// Modules
mod char_rom;
mod cpu;
mod memory;
mod vm;
mod vpu;

// Using a nice 4:3 aspect ratio
const WINDOW_HEIGHT: usize          = 600; // SDL Window height
const WINDOW_WIDTH: usize           = 800; // SDL Window width
const RES_HEIGHT: usize             = 150; // Actual height resolution
const RES_WIDTH: usize              = 200; // Actual width resolution
const TARGET_FPS: usize             = 60;  // Target frame rate 
const CLOCK_SPEED: usize            = 10_000_000_000; // 10MHz CPU Clock
const INSTRUCTIONS_PER_FRAME: usize = CLOCK_SPEED / TARGET_FPS;

fn main() {
    // Setup SDL context and video subsystem
    let sdl_context = match sdl2::init() {
        Ok(context) => context,
        Err(e) => {
            eprintln!("failed to initialise SDL2 context: {}", e);
            process::exit(1);
        }
    };
    let video_subsystem = sdl_context.video().unwrap();

    // Setup window
    let window = video_subsystem
        .window("Phosphor-ARC", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    // Setup canvas and texture buffer
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    // Internal streaming texture matrix
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::ARGB8888, RES_HEIGHT, RES_WIDTH)
        .unwrap();

    // Configure VM instance here

    // VM buffer array (example before VPU is setup)
    let mut vm_framebuffer = [0u32; RES_HEIGHT * RES_WIDTH];
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Delay required to get 60 FPS
    let frame_duration = Duration::from_nanos(CLOCK_SPEED / TARGET_FPS as u64);

    'running: loop {
        let frame_start = Instant::now();

        // Event loop
        for event in event_pump.poll_iter() {
            match event {
                // Quit event or escape
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                // Other keypresses below
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    println!("Spacebar pressed!");
                }
                _ => {}
            }
        }

        // Run VM step batch here
        
        // Fill frame buffer with red for now
        for pixel in vm_framebuffer.iter_mut() { *pixel = 0x00FF0000; }

        // Blit and present
        texture
            .update(
                None,
                unsafe {
                    std::slice::from_raw_parts(
                        vm_framebuffer.as_ptr() as *const u8,
                        vm_framebuffer.len() * 4,
                    )
                },
                RES_WIDTH * 4,
            )
            .unwrap();
        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        // Delay to keep to 60FPS
        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }
}
