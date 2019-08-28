use std::env::args;
use image;
use crate::structs::{CharCell, Kernel};


const ASCII_CHARS: [char;11] = [':', '8', '%', '=', ',', '@', '.', 'X', '&', '~', 'S'];

fn main() {
    /// get path fron env
    args = args().collect();
    /// create Dynamic image
    image::open(&args[1]);
    kernel = Kernel::new(9);
    char_cell = CharCell;

 // spawn threads to handle pixle analysis and produce terminal cells

 // write to screen at the correct sequence and\or save to file
}