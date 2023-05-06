use volatile::Volatile;

type Vu32 = Volatile<u32>;

pub struct MmioRegisterSet {
    pub dram_addr: Vu32,     //00
    pub cart_addr: Vu32,     //04
    pub rd_length: Vu32,     //08
    pub wr_length: Vu32,     //0C
    pub status: Vu32,        //10
    pub bsd_dom1_lat: Vu32,  //14
    pub bsd_dom1_pwd: Vu32,  //18
    pub bsd_dom1_pgs: Vu32,  //1C
    pub bsd_dom1_rls: Vu32,  //20
    pub bsd_dom2_lat: Vu32,  //24
    pub bsd_dom2_pwd: Vu32,  //28
    pub bsd_dom2_pgs: Vu32,  //2C
    pub bsd_dom2_rls: Vu32,  //30
}

pub unsafe fn mmio_ref() -> &'static mut MmioRegisterSet {
    &mut *(super::uncached_mut_from_phys_unchecked(super::PI_MMIO_BASE_PHYS))
}

pub unsafe fn block_until_done() {
    let pi = mmio_ref();

    while (pi.status.read() & 1) == 1 { }
}

pub unsafe fn start_transfer_to_dram(phys_dram_addr: usize, length: usize, cart_addr: usize) {
    let pi = mmio_ref();

    //TODO validity checks here
    pi.dram_addr.write(phys_dram_addr as u32);
    pi.cart_addr.write(cart_addr as u32);
    pi.wr_length.write((length as u32) - 1);
}
