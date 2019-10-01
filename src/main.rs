#![allow(non_snake_case)]

use std::env::args;

use ansi_term::Colour::RGB;

use rascii::{initial_image_processing, structs};

const DEFAULT_KERN: u32 =  9;
static ASCII_CHARS: [char;11] = [':', '8', '%', '=', ',', '@', '.', 'X', '&', '~', 'S'];

fn main() {
    // todo: screen resolution optimization (add " " to normal screen - "" to small screen)
    let args: Vec<String> = args().collect();
    assert!(args.len() as usize >= 2, "file path was not provided!");
    let kern_arg: u32 = match args.get(2){
        Some(num) => num.parse().expect("invalid kernel number!"),
        None => DEFAULT_KERN
    };
    let mut kernel = structs::Kernel::new(kern_arg);
    let (img, (width, height)) = initial_image_processing(&args[1], &kernel);
    let mut current_point = [0, 0];
    let grey = img.to_luma();
    while current_point[1] < height {
        let cell = kernel.to_char_cell(&current_point, &img, &grey, &ASCII_CHARS);
        let (r, g, b) = cell.color;
        let ascii  = RGB(r, g, b).paint(cell.ascii.to_string());
        current_point = if current_point[0] + kernel.kernel() < width {
            print!("{} ", ascii);
            [current_point[0] + kernel.kernel(), current_point[1]]
        } else {
            println!("{} ", ascii);
            [0, current_point[1] + (kernel.kernel())]
        };
    }
}