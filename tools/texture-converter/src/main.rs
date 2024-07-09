use image::{GenericImageView};
use std::fs::File;
use std::io::Write;

fn main() {
  let input_filename = std::env::args().nth(1).expect("no input filename provided");
  let output_filename = std::env::args().nth(2).expect("no output filename provided");

  let input = image::open(input_filename).expect("file not found");

  let result = File::create(output_filename);

  let mut output = match result {
    Ok(file) => file,
    Err(error) => panic!("failed to open output file {:?}", error),
  };

  for (_x, _y, pixel) in input.pixels() {
    let r = pixel[0] >> 3;
    let g = pixel[1] >> 3;
    let b = pixel[2] >> 3;
    let a = pixel[3] >> 7;

    // rrrrrggg
    let b1 = r << 3 | g >> 2;

    // ggbbbbba
    let b2 = ((g & 0b11) << 6) | b << 1 | a;

    let result = output.write_all(&[b1, b2]);
    match result {
      Ok(_) => {},
      Err(error) => panic!("failed to write to file {:?}", error),
    };
  }
}
