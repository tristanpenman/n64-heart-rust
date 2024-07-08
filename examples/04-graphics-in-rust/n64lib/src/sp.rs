use volatile::Volatile;

type Vu32 = Volatile<u32>;

pub struct MmioRegisterSet {
    pub status: Vu32,         //00
    pub origin: Vu32,         //04
    pub width: Vu32,          //08
    pub v_intr: Vu32,         //0c
    pub v_current_line: Vu32, //10
    pub timing: Vu32,         //14
    pub v_sync: Vu32,         //18
    pub h_sync: Vu32,         //1C
    pub h_sync_leap: Vu32,    //20
    pub h_video: Vu32,        //24
    pub v_video: Vu32,        //28
    pub v_burst: Vu32,        //2c
    pub x_scale: Vu32,        //30
    pub y_scale: Vu32,        //34
}

pub const STATUS_BPP16: u32 = 0x002;

pub unsafe fn mmio_ref() -> &'static mut MmioRegisterSet {
    &mut *(super::uncached_mut_from_phys_unchecked(super::VI_MMIO_BASE_PHYS))
}

pub unsafe fn wait_for_vblank() {
    let mmio = mmio_ref();
    //And now wait for the the vblank line
    while mmio.v_current_line.read() != 0x1E0 {}
}

pub unsafe fn screen_ntsc(width: u32, height: u32, status: u32, origin: usize) {
    let mmio = mmio_ref();

    mmio.status.write(status);
    mmio.origin.write(origin as u32);
    mmio.width.write(width);
    mmio.v_intr.write(0x200);
    mmio.v_current_line.write(0);
    mmio.timing.write(0x3E52239);
    mmio.v_sync.write(0x20d);
    mmio.h_sync.write(0x0c15);
    mmio.h_sync_leap.write(0x0c150c15);
    mmio.h_video.write(0x6C02EC);
    mmio.v_video.write(0x2501FF);
    mmio.v_burst.write(0xE0204);
    mmio.x_scale.write(0x100 * (width / 160));
    mmio.y_scale.write(0x100 * (height / 60));
}
