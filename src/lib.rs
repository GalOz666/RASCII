use image::{self, imageops::blur, png::PNGReader::Read, GenericImage, DynamicImage, ImageDecoder, ImageBuffer, GenericImageView, Rgb, RgbImage, Rgba, GrayImage,
FilterType::*};
use math::round;
pub use structs;


const MAX_CHARS: u16 = 80;

pub fn initial_image_processing(path: &str, kernel: &structs::Kernel) -> (DynamicImage, GrayImage) {
    // add error handling
    let kern_factor = kernel.kernel();

    let img = image::open(&path).unwrap().blur(5.0);
    // get side and see that it fits with kernel:
    let dimensions = image::image_dimensions(&path).unwrap();
    // i.e., if the size of the picture will be bigger than the maximum amount of characters in the terminal

    let w: u16 = if dimensions.0 > MAX_CHARS as u32 * kern_factor {
        MAX_CHARS*kern_size
    } else {
        (dimensions.0 - (dimensions.0%kern_factor)).parse()
    };
    let h = dimensions.1 - (dimensions.1%kern_factor);
    cropped_rgb: DynamicImage  = img.resize(w as u32, h,  Gaussian);
    cropped_grey: GrayImage  = cropped_rgb.to_luma;
    (cropped_img, cropped_grey)

}

pub fn grey_to_ascii(color: u8, ascii: vec<char>) -> char {
    let ascii_len = ascii.len();
    let metre = 255/ascii_len;
    let value = color as f64 / metre as f64;
    let index = round::floor(value, 0);
    ascii[index as uzie]
}

pub fn write_to_term(char_cell: CharCell){
    ()
}


