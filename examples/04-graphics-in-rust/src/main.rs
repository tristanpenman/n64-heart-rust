#![no_std]
#![no_main]

extern crate periph;
extern crate rt;
extern crate panic_halt;
extern crate volatile;

#[macro_use]
pub mod console;
mod cont;
mod fbcon;

use periph::pi;
use periph::vi;

const FRAMEBUFFER_PHYS_ADDR: usize = 0x0030_0000;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

enum Slide {
    Text(&'static str),
    Image(usize),
}

const SLIDES: [Slide;3] = [
    Slide::Image(0),
    Slide::Image(1),
    Slide::Text(TITLE),
];

enum ProgressionState {
    WaitingForDown,
    WaitingForAUp,
    WaitingForBUp,
}

const A_KEY: u32 = 0x8000_0000;
const B_KEY: u32 = 0x4000_0000;

#[no_mangle]
pub unsafe extern "C" fn entry() {
    console::setup(FRAMEBUFFER_PHYS_ADDR, WIDTH, HEIGHT);

    cont::init();

    let mut cur_slide_num: usize = 0;

    let mut key_state = ProgressionState::WaitingForDown;

    loop {
        let cur_slide = &SLIDES[cur_slide_num];

        match cur_slide {
            &Slide::Text(text) => {
                console::clear();
                println!("{}", text);
                console::flush();
            }
            &Slide::Image(offset) => {
                let cart_data_base = 0x1010_1000;
                let image_size = WIDTH * HEIGHT * 2;
                let cart_image_base = (image_size * offset) + cart_data_base;

                pi::start_transfer_to_dram(FRAMEBUFFER_PHYS_ADDR, 
                        image_size, cart_image_base);

                pi::block_until_done();
            }
        }

        let keys = cont::scan().unwrap();

        let new_key_state = match &key_state {
            &ProgressionState::WaitingForDown => {
                if keys & A_KEY != 0 {
                    ProgressionState::WaitingForAUp
                } else if keys & B_KEY != 0 {
                    ProgressionState::WaitingForBUp
                } else {
                    ProgressionState::WaitingForDown
                }
            }

            &ProgressionState::WaitingForAUp => {
                if keys & A_KEY == 0 {
                    if cur_slide_num != SLIDES.len() -1 {
                        cur_slide_num += 1;
                    }
                    ProgressionState::WaitingForDown
                } else {
                    ProgressionState::WaitingForAUp
                }
            }

            &ProgressionState::WaitingForBUp => {
                if keys & B_KEY == 0 {
                    if cur_slide_num != 0 {
                        cur_slide_num -= 1;
                    }
                    ProgressionState::WaitingForDown
                } else {
                    ProgressionState::WaitingForBUp
                }
            }
        };

        key_state = new_key_state;

        vi::wait_for_vblank();
    }
}

const TITLE: &'static str = "









       Let's be serious, though...









";
