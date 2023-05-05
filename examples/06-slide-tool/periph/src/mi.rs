use volatile::Volatile;

type Vu32 = Volatile<u32>;

pub struct MmioRegisterSet {
    pub mode: Vu32,          //00
    pub version: Vu32,       //04
    pub intr: Vu32,          //08
    pub intr_mask: Vu32,     //0C
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum Intr {
    SP = 0,
    SI = 1,
    AI = 2,
    VI = 3,
    PI = 4,
    DP = 5,
}

#[derive(Debug, Copy, Clone)]
pub struct IntrMask {
    mask_reg: u32,
}

const CLEAR_MASK_PATTERN: u32 = 0b_01;
const SET_MASK_PATTERN:   u32 = 0b_10;

const fn clear_mask_for_intr(intr: Intr) -> u32 {
    CLEAR_MASK_PATTERN << (2 * (intr as u32))
}

const fn set_mask_for_intr(intr: Intr) -> u32 {
    SET_MASK_PATTERN << (2 * (intr as u32))
}

impl IntrMask {
    pub fn new() -> IntrMask {
        IntrMask {
            mask_reg: set_mask_for_intr(Intr::SP) |
                      set_mask_for_intr(Intr::SI) |
                      set_mask_for_intr(Intr::AI) |
                      set_mask_for_intr(Intr::VI) |
                      set_mask_for_intr(Intr::PI) |
                      set_mask_for_intr(Intr::DP),
        }
    }

    fn mask_mask(&mut self, intr: Intr) {
        self.mask_reg &= !(0b_11 << (2* (intr as u32)));
    }

    pub fn set_mask(&mut self, intr: Intr) {
        self.mask_mask(intr.clone());
        self.mask_reg |= set_mask_for_intr(intr);
    }

    pub fn clear_mask(&mut self, intr: Intr) {
        self.mask_mask(intr.clone());
        self.mask_reg |= clear_mask_for_intr(intr);
    }
}

pub unsafe fn mmio_ref() -> &'static mut MmioRegisterSet {
    &mut *(super::uncached_mut_from_phys_unchecked(super::MI_MMIO_BASE_PHYS))
}

pub unsafe fn version() -> u32 {
    mmio_ref().version.read()
}

pub unsafe fn write_mask(intr_mask: &IntrMask) {
    mmio_ref().intr_mask.write(intr_mask.mask_reg);
}

pub unsafe fn cur_mask() -> u32 {
    mmio_ref().intr_mask.read()
}
