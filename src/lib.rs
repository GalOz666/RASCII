use image::{self,
            imageops::blur,
            png::PNGReader::Read,
            GenericImage,
            ImageDecoder};
use exoquant::*;


const ASCII_CHARS: [char;7] = ['8', '%', '=', '+', '@', 'W', 'X'];

mod pixle_analysis {


/// kernel based RGB color averaging for each cell:
/// different shades of the same color will be the same character
/// with each color having a unique character assignment (character assignment is unique on each run - as it's more fun) (~20 as per number of colors)
///
/// We will build a highly quantized image for the character analysis, and use the-original-but-blurred image for character coloration.

/// combine both to a cell struct along with a locator(x, y)
/// such that we will have a cell with character + color + location, read to be written
/// into the terminal

}


/// image_handling:

// import image: either .png or .jpg -> with GenericImageView trat
// I think we can do it with O:read image::png::PNGDecoder:new(raw_image)?

/// process image (gaussian blur)
fn initial_image_processing(path: &str) -> impl GenericImage {

    let img = image::open(path).unwrap();
    // get side and see that it fits with kernel:
    let dim = img.dimensions();
    let w = dim.0 - (dim.0%9); // TODO: replace by kernel value...
    let h = dim.1 - (dim.1%9); // TODO: replace by kernel value...
    cropped_img = img.thumbnail(w, h);
    drop(img);
    cropped_img

}

fn quantized_image(&image: impl GenericImage, num_colors: usize) -> (Vec<Color>, Vec<u8>) {
    let (palette, indexed_data) = convert_to_indexed(
        image.pixels, image.width, num_colors,
      &optimizer::KMeans, &ditherer::FloydSteinberg::new());
    return (palette, indexed_data)
}
/// each mapped to a character (assignment will be semi-random as it's more fun!)

/// save ascii to .png image (other formats maybe too?)

