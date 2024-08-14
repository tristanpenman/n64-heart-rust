use core::fmt;
use core::slice;

use fbcon::FramebufferConsole;

use n64lib::uncached_mut_from_phys;
use n64lib::vi;

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

pub unsafe fn setup(framebuffer_phys: usize, width: usize, height: usize) {
    let framebuffer_ptr: *mut Volatile<u16> = uncached_mut_from_phys(framebuffer_phys)
            .unwrap();

    FB = Some(slice::from_raw_parts_mut(framebuffer_ptr, width * height));

    vi::screen_ntsc(width as u32, height as u32, vi::STATUS_BPP16, framebuffer_phys);

    CON = FramebufferConsole::new(width, height, fb(), 0x0000, 0xFFFE, false);
}
