use image::{self, imageops::blur, GenericImage, DynamicImage::{self, *}, ImageDecoder, ImageBuffer, GenericImageView, Rgb, RgbImage, GrayImage, ImageLuma8, Rgba, Luma};
use crate::grey_to_ascii;
use counter::Counter;

pub struct CharCell {
    x: usize,
    y: usize,
    color: Vec<u8>,
    ascii: char
}

pub struct Kernel {

    kernel: u32
}

struct BinaryKernel {

    pub matrix: Vec<Vec<bool>>
}

pub trait KernelOperations
    {

        // test that the size conforms to kernel: u32 size
    fn get_kernel_locators(&self, start_pos: [u32; 2]) -> Vec<[u32; 2]> {
        let mut kern : Vec<[u32; 2]> = Vec::new();
        for outer_num in 0..self.kernel() {
            for inner_num in 0..self.kernel() {
                kern.push([start_pos[0]+outer_num, start_pos[1]+inner_num]);
            }
         }
        kern
    }

    fn kernel_colors(&self, kernel_locator: &Vec<[u32;2]>, image: &DynamicImage) -> Vec<Vec<u8>>;

    fn kernel_greys(&self, kernel_locator: &Vec<[u32;2]>, image: &DynamicImage) -> Vec<u8>;

    fn dominant_color_by_kernel(&self, kernel_locator: &Vec<[u32;2]>, image: &DynamicImage) -> Vec<u8>;

    fn dominant_grey_by_kernel(&self, kernel_locator: &Vec<[u32;2]>, image: &DynamicImage) -> u8;

}

impl Kernel {
    pub fn new(num: u32) -> Self {
        Kernel { kernel: num }
    }
    pub fn kernel(&self) -> u32{
        self.kernel
    }

    fn to_binary(&self)-> BinaryKernel {
        let capacity = self.kernel as usize *  self.kernel as usize;
        let mut matrix: Vec<Vec<bool>> = Vec::with_capacity(capacity);
        for _ in 0..self.kernel {
            let mut inner: Vec<bool> = Vec::with_capacity(self.kernel as usize);
            for _ in 0..self.kernel {
                inner.push(false);
            matrix.push(inner);
            }
        }
        structs::BinaryKernel::new{matrix}
    }
}

impl KernelOperations for Kernel {
    fn kernel_colors(&self, kernel_locator: &Vec<[u32; 2]>, image: &DynamicImage) -> Vec<Vec<u8>>  {
        assert_eq!(kernel_locator.len() as u32,  self.kernel());

        let mut colors = Vec::with_capacity(self.kernel() as usize);

        for loc in kernel_locator {
            let pixel = match image {
                Rgba(img) => img.get_pixel(loc[0], loc[1]),
                _ => panic!("Supports only RGBA!")
                };
            if let Rgba(color) = pixel {
                 colors.push(color.to_vec())
                }
        }
        colors
    }
    fn kernel_greys(&self, kernel_locator: &Vec<[u32; 2]>, image: &DynamicImage) -> Vec<u8> {


    }

    fn dominant_color_by_kernel(&self, kernel_locator: &Vec<[u32; 2]>, image: &DynamicImage) -> Vec<u8> {
        let colors: Vec<Vec<u8>> = self.kernel_colors(kernel_locator, image);
        let counter: Counter<_, i8> = colors.collect();
        counter[0][0]
    }
    fn dominant_grey_by_kernel(&self, kernel_locator: &Vec<[u32; 2]>, image: &DynamicImage) -> u8 {
        grey_img = image.to_luma();

    }
}

impl BinaryKernel {
    pub fn matrix(&self) -> &Vec<Vec<bool>>{
        &self.matrix
    }
    pub fn new(matrix: Vec<Vec<bool>>) -> Self {
        BinaryKernel{matrix}
    }
}

impl CharCell {
                                                        //image::new(PATH)              to_luma
    pub fn new(kernel: Kernel, start_pos: [u32; 2], image: &DynamicImage, ascii: &[char;11]) -> Self {
        let locators = kernel.get_kernel_locators(start_pos);
        let grey_image = image.to_luma();
        let (x, y) = locators[0];
        let x = (x + kernel.kernel()) / 9;
        let y = (y + kernel.kernel()) / 9;
        let color = kernel.dominant_color_by_kernel(&locators, image);
        let grey_color = kernel.dominant_color_by_kernel(&locators, &ImageLuma8(grey_image));
        let ascii = grey_to_ascii(grey_color[0], ascii);
        return CharCell { x, y, color, ascii }
    }
}