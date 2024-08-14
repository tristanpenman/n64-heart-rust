#![feature(asm_experimental_arch)]
#![deny(clippy::all)]
#![no_main]
#![no_std]

extern crate n64_pac;
extern crate n64lib;

use n64_pac::vi::{
    AntiAliasMode,
    BurstReg,
    ColorDepth,
    HSyncLeapReg,
    HSyncReg,
    HVideoReg,
    VBurstReg,
    VideoInterface,
    VVideoReg,
    YScaleReg,
    XScaleReg
};

use n64lib::ipl3font;

use core::arch::asm;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

const FRAME_BUFFER: u32 = 0xA010_0000;
const FRAME_BUFFER_SIZE: usize = WIDTH * HEIGHT * 2;

// Colors are 5:5:5:1 RGB with a 16-bit color depth.
#[allow(clippy::unusual_byte_groupings)]
const WHITE: u16 = 0b11111_11111_11111_1;

fn vi_init(vi: &VideoInterface) {
    // Clear both frame buffers to black
    let frame_buffer = FRAME_BUFFER as usize;
    for i in 0..WIDTH * HEIGHT {
        let p = (frame_buffer + i * 4) as *mut u32;
        unsafe {
            *p = 0x1001_1001;
        }
    }

    // Set control register (0x320E)
    let mut value = vi.ctrl.read();
    value.set_depth(ColorDepth::BPP16);
    value.set_gamma_dither_enable(true);
    value.set_gamma_enable(true);
    value.set_aa_mode(AntiAliasMode::ResamplingOnly);
    value.set_pixel_advance(1);
    vi.ctrl.write(value);

    // Set remaining registers
    vi.origin.write(FRAME_BUFFER);
    vi.width.write(WIDTH as u32);
    vi.v_intr.write(0x200);
    vi.v_current.write(0);
    vi.burst.write(BurstReg(0x3E52239));
    vi.v_sync.write(0x20d);
    vi.h_sync.write(HSyncReg(0x0c15));
    vi.h_sync_leap.write(HSyncLeapReg(0x0c150c15));
    vi.h_video.write(HVideoReg(0x6C02EC));
    vi.v_video.write(VVideoReg(0x2501FF));
    vi.v_burst.write(VBurstReg(0xE0204));
    vi.x_scale.write(XScaleReg((0x100 * (WIDTH / 160)) as u32));
    vi.y_scale.write(YScaleReg((0x100 * (HEIGHT / 60)) as u32));
}

fn vi_next_buffer(vi: &VideoInterface) -> u32 {
    let origin = vi.origin.read();

    if origin & 0xFFFFF != 0 {
        FRAME_BUFFER as u32
    } else {
        (FRAME_BUFFER as usize + FRAME_BUFFER_SIZE) as u32
    }
}

fn vi_swap_buffer(vi: &VideoInterface) {
    vi.origin.write(vi_next_buffer(vi));
}

#[no_mangle]
fn main() {
    let vi = unsafe { VideoInterface::new() };

    vi_init(&vi);
    ipl3font::draw_str_centered(WHITE, "Hello Rust!", vi_next_buffer(&vi) as usize);
    vi_swap_buffer(&vi);

    loop {
        unsafe { asm!("nop"); }
    }
}
