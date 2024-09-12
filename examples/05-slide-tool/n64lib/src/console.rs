use core::fmt;
use core::slice;

use fbcon::FramebufferConsole;
use gfx;

use n64_pac::vi::VideoInterface;

use volatile::Volatile;

static mut CON: Option<FramebufferConsole<'static, u16>> = None;

fn con() -> &'static mut FramebufferConsole<'static, u16> {
     unsafe { match CON {
         Some(ref mut x) => &mut *x,
         None => panic!(),
     }}
}

pub fn print_con(args: fmt::Arguments) {
    use core::fmt::Write;
    con().write_fmt(args).unwrap();
}

pub fn flush() {
    con().flush();
}

pub unsafe fn clear(frame_buffer_phys: usize) {
    let frame_buffer_uncached = (frame_buffer_phys | 0xA000_0000) as *mut Volatile<u16>;

    let fb = slice::from_raw_parts_mut(frame_buffer_uncached, gfx::fb_width() * gfx::fb_height());

    con().clear(fb);
}

pub unsafe fn setup(vi: &VideoInterface, frame_buffer_phys: usize) {
    let frame_buffer_uncached = (frame_buffer_phys | 0xA000_0000) as *mut Volatile<u16>;

    gfx::vi_init(&vi, frame_buffer_phys);

    let fb = slice::from_raw_parts_mut(frame_buffer_uncached, gfx::fb_width() * gfx::fb_height());

    CON = FramebufferConsole::new(gfx::fb_width(), gfx::fb_height(), fb, 0x0000, 0xFFFE, false);
}
