#![feature(asm_experimental_arch)]
// #![feature(lang_items)]
#![feature(exclusive_range_pattern)]
#![feature(naked_functions)]
#![feature(start)]

#![no_std]

extern crate volatile;

pub const CACHED_BASE: usize = 0x8000_0000;
pub const UNCACHED_BASE: usize = 0xA000_0000;

pub const PHYS_SEGMENT_SIZE: usize = 0x2000_0000;

pub const SP_MMIO_BASE_PHYS: usize = 0x0400_0000;
pub const DP_CMD_MMIO_BASE_PHYS: usize = 0x0410_0000;
pub const DP_SPAN_MMIO_BASE_PHYS: usize = 0x0420_0000;
pub const MI_MMIO_BASE_PHYS: usize = 0x0430_0000;
pub const VI_MMIO_BASE_PHYS: usize = 0x0440_0000;
pub const AI_MMIO_BASE_PHYS: usize = 0x0450_0000;
pub const PI_MMIO_BASE_PHYS: usize = 0x0460_0000;
pub const RI_MMIO_BASE_PHYS: usize = 0x0470_0000;
pub const SI_MMIO_BASE_PHYS: usize = 0x0480_0000;

pub mod mi;
pub mod mips;
pub mod pi;
pub mod pif;
pub mod si;
pub mod sp;
pub mod vi;

pub const fn uncached_mut_from_phys_unchecked<T>(phys_addr: usize) -> *mut T {
    (phys_addr + UNCACHED_BASE) as *mut T
}

pub fn uncached_mut_from_phys<T>(phys_addr: usize) -> Option<*mut T> {
    match phys_addr {
        0 .. PHYS_SEGMENT_SIZE => {
            Some(uncached_mut_from_phys_unchecked(phys_addr))
        },
        _ => {
            None
        },
    }
}
