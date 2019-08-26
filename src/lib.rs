use image::{self,
            imageops::blur,
            png::PNGReader::Read,
            GenericImage,
            ImageDecoder};
use exoquant::*;


const ASCII_CHARS: [char;7] = ['8', '%', '=', '+', '@', 'W', 'X'];

mod pixle_analysis {


/// kernel based RGB color averaging for each cell:
// img.get_pixel_mut(x, y) for getting a pixel

/// different shades of the same color will be the same character
/// with each color having a unique character assignment (character assignment is unique on each run - as it's more fun) (~20 as per number of colors)
///
/// We will use a gray-scale image for the character analysis, and use a blurry and quantized image for the coloration

/// combine both to a cell struct along with a locator(x, y)
/// such that we will have a cell with character + color + location, ready to be written to terminal

}


/// image_handling:

// import image: either .png or .jpg -> with GenericImageView trat
// I think we can do it with O:read image::png::PNGDecoder:new(raw_image)?

/// process image (gaussian blur)
fn initial_image_processing(path: &str) -> impl GenericImage {

    // add error handling
    // add shrinking mechanism if the amount of cells exceeds terminal full screen (in current resolution?)

    let img = image::open(path).unwrap();
    // get side and see that it fits with kernel:
    let dim = img.dimensions();
    let w = dim.0 - (dim.0%9); // TODO: replace by kernel value...
    let h = dim.1 - (dim.1%9); // TODO: replace by kernel value...
    cropped_img = img.thumbnail(w, h);
    drop(img);
    cropped_img.grayscale()

}

// maybe not needed....???
fn quantized_image(&image: impl GenericImage, num_colors: usize) -> (Vec<Color>, Vec<u8>) {
    let (palette, indexed_data) = convert_to_indexed(
        image.pixels, image.width, num_colors,
      &optimizer::KMeans, &ditherer::FloydSteinberg::new());
    return (palette, indexed_data)
}
/// each mapped to a character (assignment will be semi-random as it's more fun!)

/// save ascii to .png image (other formats maybe too?)
// get a buffer: let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
// and save it imgbuf.save("bla bla.png").unwrap();

