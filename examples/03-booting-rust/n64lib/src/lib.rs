#![cfg_attr(target_vendor = "nintendo64", feature(asm_experimental_arch))]
#![no_std]

pub mod ipl3font;
pub mod platform;
pub mod prelude;
pub mod vi;

#[no_mangle]
fn panic_main() -> ! {
    panic!("Main cannot return");
}
