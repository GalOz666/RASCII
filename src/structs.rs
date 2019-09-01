use image::{self, imageops::blur,  GenericImage, DynamicImage, ImageDecoder,
            ImageBuffer, GenericImageView, Rgb, RgbImage, GrayImage, ImageLuma8};
use crate::grey_to_ascii;
use counter::Counter;
use std::array::FixedSizeArray;
use num::FromPrimitive;

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

    matrix: Vec<Vec<bool>>
}

pub trait KernelOperations<T, F>
    where T: FromPrimitive,
          F: GenericImage + GenericImageView
    {

        // test that the size conforms to kernel: u32 size
    fn get_kernel_locators(&self, start_pos: [usize; 2]) -> Vec<[usize; 2]> {
        let mut kern : Vec<[usize; 2]> = Vec::new();
        for outer_num in 0..self.kernel {
            for inner_num in 0..self.kernel {
                kern.push([start_pos.0+outer_num, start_pos.1+inner_num]);
            }
         }
        kern
    }

    fn kernel_colors(&self, kernel_locator: &Vec<[usize;2]>, image: F) -> Vec<Vec<T>>;

    fn dominant_color_by_kernel(&self, kernel_locator: &Vec<[usize;2]>, image: &F) -> Vec<T>;

}

impl Kernel {
    pub fn new(num: u32) -> Self {
        Kernel { kernel: num }
    }
    pub fn kernel(&self) -> u32{
        self.kernel
    }

    fn to_binary(&self)-> BinaryKernel{
        let capacity = self.kernel as usize *  self.kernel as usize;
        let mut matrix: Vec<Vec<bool>> = Vec::with_capacity(capacity);
        for _ in 0..self.kernel {
            let mut inner: Vec<bool> = Vec::with_capacity(self.kernel as usize);
            for _ in 0..self.kernel {
                inner.push(0 as bool);
            outer_vec.push(inner);
            }
        }
        BinaryKernel {matrix}
    }
}

impl KernelOperations<u8, &DynamicImage> for Kernel {
    fn kernel_colors(&self, kernel_locator: &Vec<[usize; 2]>, image: &DynamicImage) -> Vec<Vec<u8>> {
        assert!(kernel_locator.len() as u32 = self.kernel);

        let mut colors = Vec::new();

        for (x, y) in kernel_locator {
            pixel = image.get_pixel(x, y);
            let pixel(num) = num;
            colors.push(num);
        }
        colors
    }
    fn dominant_color_by_kernel(&self, kernel_locator: &Vec<[usize; 2]>, image: &DynamicImage) -> Vec<u8> {
        img = if let ImageLuma8(img) = image {
            img
        } else {
            image
        };
        let colors: Vec<Vec<u8>> = self.kernel_colors(img);
        let counter: Counter<_, i8> = colors.collect();
        counter[0][0]
    }
}

impl CharCell {
                                                        //image::new(PATH)          to_luma
    pub fn new(kernel: Kernel, start_pos: [usize; 2], image: &DynamicImage, grey_img: &ImageLuma8, ascii: &[char;11]) -> Self {
        locators = kernel.get_kernel_locators(start_pos);
        let (x, y) = locators[0];
        let x = (x + kernel.kernel()) / 9;
        let y = (y + kernel.kernel()) / 9;
        let color = kernel.dominant_color_by_kernel(locators, image);
        let grey_color = kernel.dominant_color_by_kernel(locators, grey_image);
        let ascii = grey_to_ascii(grey_color[0], ascii);
        return CharCell { x, y, color, ascii }
    }
}