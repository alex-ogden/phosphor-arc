#![cfg_attr(rustfmt, rustfmt_skip)]

pub struct Vpu {
    pub frame_buffer: Vec<u32>,   // Frame buffer (200 x 150)
}

impl Vpu {
    pub fn new() -> Self {
        Self { frame_buffer: Vec::with_capacity(200 * 150) }
    }
}
