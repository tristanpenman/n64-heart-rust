#![allow(dead_code)]

use core::sync::atomic::{Ordering, compiler_fence};

use periph::si;

use volatile::Volatile;

#[repr(align(64))]
struct ScanPacket {
    values: [u64; 8],
}

const INIT_PACKET: ScanPacket = ScanPacket {
    values: [
        0xff_01_04_01_00000000,
        0,
        0,
        0,
        0xfe_00_00_00_00000000,
        0,
        0,
        1,
    ]
};

impl ScanPacket {
    fn new() -> ScanPacket {
        ScanPacket {
            values: [0u64; 8],
        }
    }
}

pub fn init() {
    unsafe {
        let init_packet_addr = (&INIT_PACKET as *const ScanPacket) as usize;

        si::dma_wait();

        si::write_pif(init_packet_addr);

        si::dma_wait();
    }
}

static mut SCAN_PACKET: ScanPacket = ScanPacket {
    values: [0; 8],
};

pub fn scan() -> Option<u32> {
    Some(unsafe {
        let scan_packet_addr = (&mut SCAN_PACKET as *mut ScanPacket) as usize;

        si::read_pif(scan_packet_addr);

        compiler_fence(Ordering::AcqRel);

        let ptr = 0xbfc0_07c0 as *const [Volatile<u32>;8];

        (*ptr)[1].read()
    })
}
