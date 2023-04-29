#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}

#[no_mangle]
extern "C" fn call_rust() -> i32 {
  1
}
