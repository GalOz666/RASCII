use image::{self, imageops::blur, png::PNGReader::Read, GenericImage, DynamicImage, ImageDecoder, ImageBuffer, GenericImageView, Rgb, RgbImage, Rgba, GrayImage,
FilterType::*};
use exoquant::*;
pub use structs;


const MAX_CHARS: u16 = 80;

pub mod pixel_analysis {


/// kernel based RGB color averaging for each cell:
// img.get_pixel_mut(x, y) for getting a pixel
    // enumerate_pixels_mut for (x, y, pixel) !

/// different shades of the same color will be the same character
/// with each color having a unique character assignment (character assignment is unique on each run - as it's more fun)
///
///
/// We will use a gray-scale (new_luma8()/grayscale?) image for the character analysis, and use a blurry and quantized image for the coloration (or maybe just rgb 8bit image?!).
// See example on how gray-scale ascii is elegant:
// https://gist.github.com/mayjs/5dc934d42bad05825ea9cd5a26517d97

/// combine both to a cell struct along with a locator(x, y)
/// such that we will have a cell with character + color + location, ready to be written to terminal

}


/// image_handling:

// import image: either .png or .jpg -> with GenericImageView trat
// I think we can do it with O:read image::png::PNGDecoder:new(raw_image)?

/// process image (gaussian blur)
pub fn initial_image_processing(path: &str, kernel: &structs::Kernel) -> (DynamicImage, GrayImage) {
    // add error handling
    let kern_factor = kernel.kernel();

    let img = image::open(&path).unwrap();
    // get side and see that it fits with kernel:
    let dimensions = image::image_dimensions(&path).unwrap();
    // i.e., if the size of the picture will be bigger than the maximum amount of characters in the terminal
    let w: u16;
    if dimensions.0 > MAX_CHARS as u32 * kern_factor {
        w = MAX_CHARS*kern_size
    } else {
        w = dimensions.0 - (dimensions.0%kern_factor);
    }
    let h = dimensions.1 - (dimensions.1%kern_factor);
    cropped_rgb: DynamicImage  = img.resize(w as u32, h,  Gaussian);
    cropped_grey: GrayImage  = cropped_rgb.to_luma;
    (cropped_img, cropped_grey)

}


pub fn grey_to_ascii(color: vec<u8>, ascii: vec<char>) -> char {
    '#'
}




/// save ascii to .png image (other formats maybe too?)
// get a buffer: let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
// and save it imgbuf.save("bla bla.png").unwrap();

// maybe not needed....
fn quantized_image(path: &str, num_colors: usize) -> (Vec<Color>, Vec<u8>) {
    let image = image::open(&path).unwrap();
    let (palette, indexed_data) = convert_to_indexed(
        image.pixels, image.width, num_colors,
      &optimizer::KMeans, &ditherer::FloydSteinberg::new());
    return (palette, indexed_data)
}




