use core::fmt;

use volatile::Volatile;

mod font;

const FB_WIDTH: usize = 320;
const FB_HEIGHT: usize = 240;
const NUM_CHARS_WIDTH: usize = FB_WIDTH / 8;
const NUM_CHARS_HEIGHT: usize = FB_HEIGHT / 8;
const NUM_CHARS: usize = NUM_CHARS_WIDTH * NUM_CHARS_HEIGHT;

pub struct FramebufferConsole<'a, T: 'a + Copy> {
    fb_width: usize,
    fb_height: usize,
    buffer: &'a mut [Volatile<T>],
    char_buffer: [char;NUM_CHARS],
    cur_x: usize,
    cur_y: usize,
    foreground: T,
    background: T,
    null_is_transparent: bool,
}

impl <'a, T: 'a + Copy> FramebufferConsole<'a, T> {
    pub fn new(fb_width: usize, fb_height: usize, buffer: &'a mut [Volatile<T>], foreground: T, background: T, null_is_transparent: bool) -> Option<FramebufferConsole<'a, T>> {
        if FB_WIDTH != fb_width || FB_HEIGHT != fb_height {
            return None;
        }

        if fb_width * fb_height != buffer.len() {
            return None;
        }

        Some(FramebufferConsole {
            fb_width: fb_width,
            fb_height: fb_height,
            buffer: buffer,
            char_buffer: ['\0'; NUM_CHARS],
            cur_x: 0,
            cur_y: 0,
            foreground: foreground,
            background: background,
            null_is_transparent: null_is_transparent,
        })
    }

    pub fn clear(&mut self) {
        self.char_buffer = ['\0'; NUM_CHARS];
        self.cur_x = 0;
        self.cur_y = 0;
    }

    pub fn puts(&mut self, s: &str) {
        for c in s.chars() {
            self.putc(c);
        }
    }

    pub fn scroll(&mut self) {
        for offset in 0..NUM_CHARS {
            if offset >= (NUM_CHARS - NUM_CHARS_WIDTH) {
                self.char_buffer[offset] = '\0';
            } else {
                let prev = self.char_buffer[offset + NUM_CHARS_WIDTH];
                self.char_buffer[offset] = prev;
            }
        }
    }

    pub fn putc(&mut self, c: char) {
        match c {
            '\n' => {
                self.cur_y += 1;
                self.cur_x = 0;
            },
            _ => {
                self.char_buffer[self.cur_x + (self.cur_y * NUM_CHARS_WIDTH)] = c;
                self.cur_x += 1;
            },
        }

        if self.cur_x == NUM_CHARS_WIDTH {
            self.cur_y += 1;
            self.cur_x = 0;
        }

        if self.cur_y == NUM_CHARS_HEIGHT {
            self.cur_y -= 1;
            self.scroll();
        }
    }

    pub fn flush(&mut self) {
        for y in 0..self.fb_height {
            for x in 0..self.fb_width {
                let char_x = x / 8;
                let char_y = y / 8;
                let character = self.char_buffer[char_x + (char_y * NUM_CHARS_WIDTH)];

                if character == '\0' && self.null_is_transparent {
                    continue;
                }

                let font_row = y % 8;
                let font_bits = font::CONSOLE_FONT_8X8[((character as usize) * 8) + font_row];
                let font_bit = ((font_bits >> (8 - (x % 8))) & 1) != 0;

                self.buffer[x + (y * self.fb_width)].write(if font_bit {
                    self.foreground
                } else {
                    self.background
                })
            }
        }
    }
}

impl<'a, T: 'a + Copy> fmt::Write for FramebufferConsole<'a, T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.puts(s);
        Ok(())
    }
}