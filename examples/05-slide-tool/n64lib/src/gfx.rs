extern crate n64_pac;

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

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

const FRAME_BUFFER_SIZE: usize = WIDTH * HEIGHT * 2;

pub fn vi_init(vi: &VideoInterface, frame_buffer_phys: usize) {
    // Set control register (0x320E)
    let mut value = vi.ctrl.read();
    value.set_depth(ColorDepth::BPP16);
    value.set_aa_mode(AntiAliasMode::ResamplingOnly);
    value.set_pixel_advance(1);
    vi.ctrl.write(value);

    // Set remaining registers
    vi.origin.write(frame_buffer_phys as u32);
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

pub fn vi_next_buffer(vi: &VideoInterface) -> u32 {
    let origin = vi.origin.read();

    if origin & 0xFFFFF != 0 {
        (origin & 0xFFF00000) as u32
    } else {
        (origin as usize + FRAME_BUFFER_SIZE) as u32
    }
}

pub fn vi_swap_buffer(vi: &VideoInterface) {
    vi.origin.write(vi_next_buffer(vi));
}

pub fn vi_wait_for_vblank(vi: &VideoInterface) {
    while vi.v_current.read() != 0x1E0 {}
}

pub fn fb_height() -> usize {
    HEIGHT
}

pub fn fb_width() -> usize {
    WIDTH
}
