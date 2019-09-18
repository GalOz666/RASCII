use std::env::args;
use image::{self, DynamicImage, GrayImage};

use RASCII::{structs, initial_image_processing};
use RASCII::structs::{Kernel, CharCell};
use ansi_term::Colour::RGB;
use std::time::Instant;

const ASCII_CHARS: [char;11] = [':', '8', '%', '=', ',', '@', '.', 'X', '&', '~', 'S'];

fn main() {
    let args: Vec<String> = args().collect();
    let kernel = structs::Kernel::new(9);
    assert!(args.len() as usize >= 2, "file path was not provided!");
    let (img, (width, height)) = initial_image_processing(&args[1], &kernel);
    let mut current_point = [0, 0];

    while current_point[1] < height {
        let now = Instant::now();
        let cell = kernel.to_char_cell(&current_point, &img, &ASCII_CHARS);

        let (r, g, b) = cell.color;
        let ascii  = RGB(r, g, b).paint(cell.ascii.to_string());
        current_point = if current_point[0] + kernel.kernel() < width {
            print!("{}", ascii);
            [current_point[0] + kernel.kernel(), current_point[1]]
        } else {
            println!("{}", ascii);
            [0, current_point[1] + (kernel.kernel())]
        };

    }

}