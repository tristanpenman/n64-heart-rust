use volatile::Volatile;

struct PifData {
    data: [Volatile<u32>;16],
}

unsafe fn mmio_ref() -> &'static mut PifData {
    &mut *(super::uncached_mut_from_phys_unchecked(0x1FC0_07C0))
}

pub unsafe fn disable_pif_reset() {
    mmio_ref().data[15].write(8);
}
