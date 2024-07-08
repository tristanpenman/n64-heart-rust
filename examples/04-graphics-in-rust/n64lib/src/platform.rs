#[cfg(any(unix, windows, target_vendor = "nintendo64"))]

#[cfg(target_vendor = "nintendo64")]
use core::arch::global_asm;

#[cfg(target_vendor = "nintendo64")]
global_asm!(include_str!("entrypoint.s"));
