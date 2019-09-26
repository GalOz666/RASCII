#![allow(non_snake_case)]

use std::env::args;
use std::io;
use std::io::Write;

use ansi_term::Colour::RGB;

use RASCII::{initial_image_processing, structs};

const ASCII_CHARS: [char;11] = [':', '8', '%', '=', ',', '@', '.', 'X', '&', '~', 'S'];

fn main() {
    let args: Vec<String> = args().collect();
    let kernel = structs::Kernel::new(9);
    assert!(args.len() as usize >= 2, "file path was not provided!");
    let (img, (width, height)) = initial_image_processing(&args[1], &kernel);
    let mut current_point = [0, 0];
    let grey = img.to_luma();
    let stdout = io::stdout();
    let mut lock = stdout.lock();
    while current_point[1] < height {

        let cell = kernel.to_char_cell(&current_point, &img, &grey, &ASCII_CHARS);

        let (r, g, b) = cell.color;
        let ascii  = RGB(r, g, b).paint(cell.ascii.to_string());
        current_point = if current_point[0] + kernel.kernel() < width {
            write!(lock, "{} ", ascii).unwrap();
            [current_point[0] + kernel.kernel(), current_point[1]]
        } else {
            writeln!(lock, "{} ", ascii).unwrap();
            [0, current_point[1] + (kernel.kernel())]
        };
    }
}