use image::{self, imageops::blur,
            png::PNGReader::Read,
            GenericImage, DynamicImage,
            ImageDecoder, ImageBuffer,
            GenericImageView, Rgb};
use exoquant::*;

pub struct Kernel {

    kernel: u32

}

pub trait KernelOps<T, F>
    where T: FromPrimitive,
          F: GenericImage
    {

    fn get_kernel_locators(&self, &start_pos: &[usize; 2]) -> vec<[usize; 2]> {
        let mut kern : Vec<usize> = Vec::new();
        for outer_num in 0..self.kernel {
            for inner_num in 0..self.kernel {
                kern.push([start_pos.0+outer_num, start_pos.1+inner_num]);
            }
        kern
        }
    }


    fn kernel_colors(self, image: F) -> vec<vec<T>>;
}

fn dominant_color_by_kernel(){
    ()
}
}
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
pub fn initial_image_processing(path: &str, kernel: &u32) -> (DynamicImage, DynamicImage) {

    // add error handling
    // add shrinking mechanism if the amount of cells exceeds terminal full screen (in current resolution?)
    let img = image::open(&path).unwrap();
    // get side and see that it fits with kernel:
    kernel_abs_len = kernel.into_iter().flatten();
    let dim = image::image_dimensions(&path).unwrap(); // might need to take this outside for reuse
    let w = dim.0 - (dim.0%kernel);
    let h = dim.1 - (dim.1%kernel);
    cropped_img = img.thumbnail(w, h).grayscale();
    (cropped_img, img)

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

// maybe not needed....
fn quantized_image(path: &str, num_colors: usize) -> (Vec<Color>, Vec<u8>) {
    let image = image::open(&path).unwrap();
    let (palette, indexed_data) = convert_to_indexed(
        image.pixels, image.width, num_colors,
      &optimizer::KMeans, &ditherer::FloydSteinberg::new());
    return (palette, indexed_data)
}




                                        // image.as_rgb8().unwrap()


fn dominant_color_by_kernel(){
    ()
}