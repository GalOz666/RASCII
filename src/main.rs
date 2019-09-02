use std::env::args;
use image;
use image::{DynamicImage, GrayImage};

pub mod structs;
pub mod lib;


const ASCII_CHARS: [char;11] = [':', '8', '%', '=', ',', '@', '.', 'X', '&', '~', 'S'];

fn main() {
    // get path fron env
    let args: Vec<String> = args().collect();
    // create Dynamic image
    let kernel = structs::Kernel::new(9);
    let img: DynamicImage = image::open(args[1]).unwrap();
//    grey: GrayImage = img.to_luma();


 // spawn threads to handle pixle analysis and produce terminal cells

 // write to screen at the correct sequence and\or save to file
}