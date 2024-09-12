#![no_main]
#![no_std]

extern crate n64_pac;
extern crate n64lib;
extern crate volatile;

#[macro_use]
mod console;
mod cont;
mod fbcon;

use n64_pac::si::SerialInterface;
use n64_pac::vi::VideoInterface;

use n64lib::gfx;
use n64lib::pi;

const FRAME_BUFFER_PHYS_ADDR: usize = 0x0010_0000;

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
unsafe fn main() {
    let si = unsafe { SerialInterface::new() };
    let vi = unsafe { VideoInterface::new() };

    console::setup(&vi, FRAME_BUFFER_PHYS_ADDR);
    cont::setup(&si);

    let mut cur_slide_num: usize = 0;
    let mut key_state = ProgressionState::WaitingForDown;

    loop {
        let cur_slide = &SLIDES[cur_slide_num];

        match cur_slide {
            &Slide::Text(text) => {
                console::clear(gfx::vi_next_buffer(&vi) as usize);
                println!("{}", text);
                console::flush();
            }
            &Slide::Image(offset) => {
                let cart_data_base = 0x1010_1000;
                let image_size = gfx::fb_width() * gfx::fb_height() * 2;
                let cart_image_base = image_size * offset + cart_data_base;

                pi::start_transfer_to_dram(gfx::vi_next_buffer(&vi) as usize, image_size, cart_image_base);
                pi::block_until_done();
            }
        }

        let keys = cont::scan(&si).unwrap();

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

        gfx::vi_wait_for_vblank(&vi);
        gfx::vi_swap_buffer(&vi);
    }
}

const TITLE: &'static str = "









       Let's be serious, though...









";
