use image::{self, DynamicImage};
use math::round;

use structs::Kernel;

pub mod structs;

const MAX_CHARS: u16 = 200;

pub fn initial_image_processing(path: &str, kernel: &Kernel) -> (DynamicImage, (u32, u32)) {
    // add error handling
    let kern_factor = kernel.kernel();
    let img = image::open(&path).unwrap().blur(5.0);
    // get side and see that it fits with kernel:

    let dimensions = image::image_dimensions(&path).unwrap();
    // i.e., if the size of the picture will be bigger than the maximum amount of characters in the terminal

    let w: u32 = if dimensions.0 > MAX_CHARS as u32 * kern_factor {
        MAX_CHARS as u32 * kern_factor
    } else {
        (dimensions.0 - (dimensions.0%kern_factor))
    };
    let h = dimensions.1 - (dimensions.1%kern_factor);

    let cropped_rgb: DynamicImage  = img.thumbnail_exact(w , h);
    (cropped_rgb, (w, h))

}

pub fn grey_to_ascii(color: u8, ascii: &[char]) -> char {
    let ascii_len = ascii.len();
    let metre = 255/ascii_len;
    let value = color as f64 / metre as f64;
    let index = round::floor(value, 0);
    ascii[index as usize]
}