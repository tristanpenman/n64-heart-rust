use volatile::Volatile;

type Vu32 = Volatile<u32>;

pub const STATUS_DMA_BUSY: u32 = 0b_00001;
pub const STATUS_IO_BUSY:  u32 = 0b_00010;

pub struct MmioRegisterSet {
    pub addr: Vu32,          //00
    pub start_write: Vu32,   //04
    pub rsvd_08: Vu32,       //08
    pub rsvd_0c: Vu32,       //0C
    pub start_read: Vu32,    //10
    pub rsvd_14: Vu32,       //14
    pub status: Vu32,        //18
}

pub unsafe fn mmio_ref() -> &'static mut MmioRegisterSet {
    &mut *(super::uncached_mut_from_phys_unchecked(super::SI_MMIO_BASE_PHYS))
}

pub unsafe fn dma_wait() {
    let si = mmio_ref();

    while (si.status.read() & (STATUS_DMA_BUSY | STATUS_IO_BUSY)) != 0 { }
}

pub unsafe fn write_pif(dram_addr: usize) {
    let si = mmio_ref();

    si.addr.write(dram_addr as u32);
    si.start_read.write(0x1fc0_07c0);
}

pub unsafe fn read_pif(dram_addr: usize) {
    let si = mmio_ref();

    si.addr.write(dram_addr as u32);
    si.start_write.write(0x1fc0_07c0);
}