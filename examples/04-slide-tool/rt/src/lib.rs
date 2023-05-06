#![feature(asm_experimental_arch)]
#![feature(naked_functions)]
#![feature(start)]

#![no_std]

use core::arch::global_asm;

extern crate periph;

use periph::pif;

global_asm!(include_str!("boot.s"));

#[no_mangle]
unsafe extern "C" fn rust_entrypoint() {
  pif::disable_pif_reset();

  extern "C" {
    fn entry();
  }

  entry();
}
