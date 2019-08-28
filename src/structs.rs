use image::{self, imageops::blur, png::PNGReader::Read, GenericImage, DynamicImage, ImageDecoder, ImageBuffer, GenericImageView, Rgb, RgbImage, GrayImage};
use pixelrust::grey_to_ascii;
use counter::Counter;

pub struct CharCell {
    x: usize,
    y: usize,
    color: [u32; 3],
    ascii: char
}

pub struct Kernel {

    kernel: u32

}

pub trait KernelOperations<T, F>
    where T: FromPrimitive,
          F: GenericImage + GenericImageView
    {

        // test that the size conforms to kernel: u32 size
    fn get_kernel_locators(&self, start_pos: &[usize; 2]) -> vec<[usize; 2]> {
        let mut kern : Vec<usize> = Vec::new();
        for outer_num in 0..self.kernel {
            for inner_num in 0..self.kernel {
                kern.push([start_pos.0+outer_num, start_pos.1+inner_num]);
            }
        kern
        }
    }

    fn kernel_colors(&self, kernel_locator: &vec<[usize;2]>, image: F) -> vec<vec<T>>;

    fn dominant_color_by_kernel(&self, kernel_locator: &vec<[usize;2]>, image: &F) -> vec<T>;

}

impl Kernel {
    pub fn new(num: u32) -> Self {
        Kernel { kernel: num }
    }
    pub fn kernel(&self) -> u32{
        self.kernel
    }
}

// for grey-scale images and RGB!
impl KernelOperations<u8, &DynamicImage> for Kernel {
    // image.as_rgb8().unwrap()
    fn kernel_colors(&self, kernel_locator: &vec<[usize; 2]>, image: &DynamicImage) -> vec<vec<u8>> {
        assert!(kernel_locator.len() as u32 = self.kernel);

        let mut colors = Vec::new();

        for (x, y) in kernel_locator {
            pixel = image.get_pixel(x, y);
            let pixel(num) = grey_color;
            colors.push(*pixel);
        }
        colors
    }
    fn dominant_color_by_kernel(&self, kernel_locator: &vec<[usize; 2]>, image: &DynamicImage) -> vec<u8> {
        let colors: vec<vec<u8>> = self.kernel_colors(image);
        let counter: Counter<_, i8> = colors.collect();
        counter[0][0]
    }
}

impl CharCell {
                                                        //image::new(PATH)          to_luma
    fn new(kernel: Kernel, start_pos: &[usize; 2], image: &DynamicImage, grey_img: &ImageLuma8, ascii: &vec<char>) -> Self {
        locators = kernel.get_kernel_locators(start_pos);
        let (x, y) = locators[0];
        let x = (x + kernel.kernel()) / 9;
        let y = (y + kernel.kernel()) / 9;
        let color = kernel.dominant_color_by_kernel(locators, image);
        let grey_img(grey) = grey;
        let grey_color = kernel.dominant_color_by_kernel(locators, grey);
        let ascii = grey_to_ascii(grey_color[0], ascii);
        return CharCell { x, y, color, ascii }
    }
}