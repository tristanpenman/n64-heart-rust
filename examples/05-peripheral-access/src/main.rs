#![feature(asm_experimental_arch)]
#![deny(clippy::all)]
#![no_main]
#![no_std]

extern crate n64lib;

use core::arch::asm;

#[no_mangle]
fn main() {
    loop {
        unsafe { asm!("nop"); }
    }
}
