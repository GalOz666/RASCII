use image::{self,
            imageops::blur,
            png::PNGReader::Read,
            GenericImage,
            DynamicImage,
            ImageDecoder};
use exoquant::*;


const ASCII_CHARS: [char;7] = ['8', '%', '=', '+', '@', 'W', 'X'];

pub struct CharCell {
    x: usize,
    y: usize,
    color: [u32; 3],
    ascii: char
}


pub mod pixel_analysis {


/// kernel based RGB color averaging for each cell:
// img.get_pixel_mut(x, y) for getting a pixel

/// different shades of the same color will be the same character
/// with each color having a unique character assignment (character assignment is unique on each run - as it's more fun)
///
///
/// We will use a gray-scale image for the character analysis, and use a blurry and quantized image for the coloration.
// See example on how gray-scale ascii is elegant:
// https://gist.github.com/mayjs/5dc934d42bad05825ea9cd5a26517d97

/// combine both to a cell struct along with a locator(x, y)
/// such that we will have a cell with character + color + location, ready to be written to terminal

}


/// image_handling:

// import image: either .png or .jpg -> with GenericImageView trat
// I think we can do it with O:read image::png::PNGDecoder:new(raw_image)?

/// process image (gaussian blur)
pub fn initial_image_processing<A, B>(path: &str) -> (DynamicImage, DynamicImage) {

    // add error handling
    // add shrinking mechanism if the amount of cells exceeds terminal full screen (in current resolution?)

    let img = image::open(&path).unwrap();
    // get side and see that it fits with kernel:
    let dim = image::image_dimensions(&path).unwrap();
    let w = dim.0 - (dim.0%9); // TODO: replace by kernel value...
    let h = dim.1 - (dim.1%9); // TODO: replace by kernel value...
    cropped_img = img.thumbnail(w, h).grayscale();
    (cropped_img, img)

}

// maybe not needed....???
fn quantized_image(path: &str, num_colors: usize) -> (Vec<Color>, Vec<u8>) {
    let image = image::open(&path).unwrap();
    let (palette, indexed_data) = convert_to_indexed(
        image.pixels, image.width, num_colors,
      &optimizer::KMeans, &ditherer::FloydSteinberg::new());
    return (palette, indexed_data)
}

mod pixle_mapping {
    fn pixle_map() {
        ()
    }

    fn colored_pmap() {
        ()
    }
}

/// each mapped to a character (assignment will be semi-random as it's more fun!)

/// save ascii to .png image (other formats maybe too?)
// get a buffer: let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
// and save it imgbuf.save("bla bla.png").unwrap();

