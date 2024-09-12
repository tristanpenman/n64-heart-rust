#![allow(dead_code)]

use core::sync::atomic::{Ordering, compiler_fence};

use n64_pac::si::{SerialInterface};

use volatile::Volatile;

// ----------------------------------------------------------------------------
// ScanPacket
// ----------------------------------------------------------------------------

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

static mut SCAN_PACKET: ScanPacket = ScanPacket {
    values: [0; 8],
};

// ----------------------------------------------------------------------------
// SI helpers
// ----------------------------------------------------------------------------

pub unsafe fn si_dma_wait(si: &SerialInterface) {
    while {
        let value = si.status.read();
        value.dma_busy() | value.io_busy()
    } { }
}

pub unsafe fn si_write_pif(si: &SerialInterface, dram_addr: usize) {
    si.dram_addr.write(dram_addr as u32);
    si.pif_ad_wr64b.write(0x1fc0_07c0);
}

pub unsafe fn si_read_pif(si: &SerialInterface, dram_addr: usize) {
    si.dram_addr.write(dram_addr as u32);
    si.pif_ad_rd64b.write(0x1fc0_07c0);
}

// ----------------------------------------------------------------------------
// Higher level interface
// ----------------------------------------------------------------------------

pub fn setup(si: &SerialInterface) {
    unsafe {
        let init_packet_addr = (&INIT_PACKET as *const ScanPacket) as usize;

        si_dma_wait(si);
        si_write_pif(si, init_packet_addr);
        si_dma_wait(si);
    }
}

pub fn scan(si: &SerialInterface) -> Option<u32> {
    Some(unsafe {
        let scan_packet_addr = (&mut SCAN_PACKET as *mut ScanPacket) as usize;

        si_read_pif(si, scan_packet_addr);

        compiler_fence(Ordering::AcqRel);

        let ptr = 0xbfc0_07c0 as *const [Volatile<u32>;8];

        (*ptr)[1].read()
    })
}
