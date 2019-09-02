use image::{self, imageops::blur, GenericImage, DynamicImage, ImageDecoder, ImageBuffer, GenericImageView, Rgb, RgbImage, Rgba, GrayImage,
FilterType::*;
use math::round;
use crate::structs::Kernel;

pub mod structs;


const MAX_CHARS: u16 = 80;

pub fn initial_image_processing(path: &str, kernel: &structs::Kernel) -> (DynamicImage, GrayImage) {
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
    let cropped_rgb: DynamicImage  = img.resize(w , h,  Gaussian);
    let cropped_grey: GrayImage  = cropped_rgb.to_luma();
    (cropped_rgb, cropped_grey)

}

pub fn grey_to_ascii(color: Luma<u8>, ascii: &[char;11]) -> char {
    let ascii_len = ascii.len();
    let metre = 255/ascii_len;
    let value = color as f64 / metre as f64;
    let index = round::floor(value, 0);
    ascii[index as usize]
}

pub fn write_to_term(char_cell: structs::CharCell){
    ()
}

pub fn highpass_filter(image: DynamicImage) -> DynamicImage {
    let small: f32 = -1.0/9.0;
    let big: f32 = 8.0/9.0;
    image.filter3x3(&[small, small, small, small, big, small, small, small, small])
}

pub fn get_approximate_shape_from_kernel(kernel: Kernel, binary_kernel: Vec<Vec<bool>>){
    ()
}
