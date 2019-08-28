use std::env::args;
use image;
use image::{DynamicImage, GrayImage};

pub mod structs;


const ASCII_CHARS: [char;11] = [':', '8', '%', '=', ',', '@', '.', 'X', '&', '~', 'S'];

fn main() {
    /// get path fron env
    args = args().collect();
    /// create Dynamic image
    kernel = structs::Kernel::new(9);
//    img: DynamicImage = image::open(&args[1]).unwrap();
//    grey: GrayImage = img.to_luma();
//    char_cell = structs::CharCell::(kernel, img, grey)

 // spawn threads to handle pixle analysis and produce terminal cells

 // write to screen at the correct sequence and\or save to file
}