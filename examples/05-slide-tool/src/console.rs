use core::fmt;
use core::slice;

use fbcon::FramebufferConsole;

use n64_pac::vi::{
    VideoInterface
};

use n64lib::gfx;

use volatile::Volatile;

static mut FB: Option<&mut [Volatile<u16>]> = None;
static mut CON: Option<FramebufferConsole<'static, u16>> = None;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::print_con((format_args!($($arg)*))));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

unsafe fn fb() -> &'static mut [Volatile<u16>] {
    match FB {
        Some(ref mut x) => &mut *x,
        None => panic!(),
    }
}

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

pub fn clear() {
    con().clear();
}

pub unsafe fn setup(vi: &VideoInterface, frame_buffer_phys: usize) {


    let fb_origin = (frame_buffer_phys | 0xA000_0000) as *mut Volatile<u16>;

    gfx::vi_init(&vi, frame_buffer_phys);

    FB = Some(slice::from_raw_parts_mut(fb_origin, gfx::fb_width() * gfx::fb_height()));

    CON = FramebufferConsole::new(gfx::fb_width(), gfx::fb_height(), fb(), 0x0000, 0xFFFE, false);
}
