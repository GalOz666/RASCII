#![allow(non_snake_case)]

use std::env::args;

use ansi_term::Colour::RGB;

use rascii::{initial_image_processing, structs};

const DEFAULT_KERN: u32 =  9;
static ASCII_CHARS: [char;11] = [':', '8', '%', '=', ',', '@', '.', 'X', '&', '~', 'S'];


fn main() {
    let args: Vec<String> = args().collect();
    let screen_arg = String::from("--small-screen");
    assert!(args.len() as usize >= 2, "file path was not provided!");
    let kern_arg: u32 = match args.get(2){
        Some(num) => num.parse().unwrap_or_else(|_x| DEFAULT_KERN),
        None => DEFAULT_KERN
    };
    let pad = if args.contains(&screen_arg) {
        ""
    } else {
        " "
    };
    let mut kernel = structs::Kernel::new(kern_arg);
    let (img, (width, height)) = initial_image_processing(&args[1], &kernel);
    let mut current_point = [0, 0];
    let grey = img.to_luma();
    while current_point[1] < height {
        let cell = kernel.get_char_cell(&current_point, &img, &grey, &ASCII_CHARS);
        let (r, g, b) = cell.color;
        let ascii  = RGB(r, g, b).paint(cell.ascii.to_string());
        current_point = if current_point[0] + kernel.kernel() < width {
            print!("{}{}", ascii, pad);
            [current_point[0] + kernel.kernel(), current_point[1]]
        } else {
            println!("{}{}", ascii, pad);
            [0, current_point[1] + (kernel.kernel())]
        };
    }
}