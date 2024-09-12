use n64_pac::pi::{PeripheralInterface};

// ----------------------------------------------------------------------------
// PI helpers
// ----------------------------------------------------------------------------

pub unsafe fn block_until_done(pi: &PeripheralInterface) {
    while {
        let value = pi.status.read();
        value.read.dma_busy() | value.read.io_busy()
    } { }
}

pub unsafe fn start_transfer_to_dram(pi: &PeripheralInterface, phys_dram_addr: usize, length: usize, cart_addr: usize) {
    pi.dram_addr.write(phys_dram_addr as u32);
    pi.cart_addr.write(cart_addr as u32);
    pi.wr_len.write((length as u32) - 1);
}
