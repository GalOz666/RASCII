use image::{self, imageops::blur, GenericImage, DynamicImage::{self, *}, ImageDecoder, ImageBuffer, GenericImageView, Rgb, RgbImage, GrayImage, ImageLuma8, Rgba, Luma};
use crate::grey_to_ascii;
use counter::Counter;

pub struct CharCell {
    x: u32,
    y: u32,
    color: Vec<u8>,
    ascii: char
}

pub struct Kernel {

    kernel: u32,
}

pub struct BinaryKernel {

    matrix: Vec<Vec<bool>>,
}

impl Kernel {
    pub fn new(num: u32) -> Self {
        Kernel { kernel: num }
    }
    pub fn kernel(&self) -> u32 {
        self.kernel
    }

    fn to_binary(&self) -> BinaryKernel {
        let capacity = self.kernel() as usize * self.kernel() as usize;
        let mut matrix: Vec<Vec<bool>> = Vec::with_capacity(capacity);
        for _ in 0..self.kernel() {
            let mut inner: Vec<bool> = Vec::with_capacity(self.kernel() as usize);
            for _ in 0..self.kernel() {
                inner.push(false);
                matrix.push(inner);
            }
        }
        BinaryKernel::new(matrix)
    }

    fn get_kernel_locators(&self, start_pos: &[u32; 2]) -> Vec<[u32; 2]> {
        let mut kern: Vec<[u32; 2]> = Vec::new();
        for outer_num in 0..self.kernel() {
            for inner_num in 0..self.kernel() {
                kern.push([start_pos[0] + outer_num, start_pos[1] + inner_num]);
            }
        }
        kern
    }


    fn kernel_colors(&self, kernel_locator: &Vec<[u32; 2]>, image: &DynamicImage) -> Vec<Vec<u8>> {
        assert_eq!(kernel_locator.len() as u32, self.kernel());
        let mut colors = Vec::with_capacity(self.kernel() as usize);

        for loc in kernel_locator {
            let pixel = image.get_pixel(loc[0], loc[1]);
            if let Rgba(img) = pixel {
                colors.push(img.to_vec());
            };
        }
        colors
    }
    fn kernel_greys(&self, kernel_locator: &Vec<[u32; 2]>, image: &DynamicImage) -> Vec<u8> {
        assert_eq!(kernel_locator.len() as u32, self.kernel());
        let image = image.to_luma();
        let mut colors = Vec::with_capacity(self.kernel() as usize);

        for loc in kernel_locator {
            let pixel = image.get_pixel(loc[0], loc[1]);
            if let Luma(img) = pixel {
                colors.push(img[0]);
            };
        }
        colors
    }


    fn dominant_color_by_kernel(&self, kernel_locator: &Vec<[u32; 2]>, image: &DynamicImage) -> Vec<u8> {
        let colors: Vec<Vec<u8>> = self.kernel_colors(kernel_locator, image);
        let counter: Counter<_, u8> = colors.iter().collect();
        counter[&0][0]
    }
    fn dominant_grey_by_kernel(&self, kernel_locator: &Vec<[u32; 2]>, image: &DynamicImage) -> u8 {
        let grey_img = image.to_luma();
        let colors: Vec<u8> = self.kernel_greys(kernel_locator, image);
        let counter: Counter<_, u8> = colors.iter().collect();
        counter[&0][0]
    }

    pub fn to_char_cell(&self, start_pos: &[u32; 2], image: &DynamicImage, ascii: &[char]) -> CharCell {
        let x = (start_pos[0] + self.kernel()) / 9;
        let y = (start_pos[1] + self.kernel()) / 9;
        let locators = self.get_kernel_locators(start_pos);
        let color = self.dominant_color_by_kernel(&locators, image);
        let grey_color = self.dominant_grey_by_kernel(&locators, image);
        let ascii = grey_to_ascii(grey_color, ascii);
        return CharCell { x, y, color, ascii }
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