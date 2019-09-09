use image::{self, imageops::blur, GenericImage, DynamicImage::{self, *}, ImageDecoder, ImageBuffer, GenericImageView, Rgb, RgbImage, GrayImage, ImageLuma8, Rgba, Luma};
use counter::Counter;
use crate::grey_to_ascii;
use tuikit::attr::Color;

pub struct CharCell {
    pub x: usize,
    pub y: usize,
    pub color: Color,
    pub ascii: char
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
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
            }
        matrix.push(inner);
        }
        BinaryKernel::new(matrix)
    }

    fn get_kernel_locators(&self, start_pos: &[u32; 2]) -> Vec<[u32; 2]> {
        let mut kern: Vec<[u32; 2]> = Vec::new();
        let num = (self.kernel() as f64).sqrt() as u32;
        for outer_num in 0..num {
            for inner_num in 0..num {
                kern.push([start_pos[0] + outer_num, start_pos[1] + inner_num]);
            }
        }
        kern
    }


    fn kernel_colors(&self, kernel_locator: &Vec<[u32; 2]>, image: &DynamicImage) -> Vec<Vec<u8>> {
        println!("{:?}", kernel_locator);
        assert_eq!(kernel_locator.len() as u32, self.kernel(),
        "kernel locator lenght and actual kernel value are not the same!");
        let mut colors = Vec::with_capacity(self.kernel() as usize);

        for loc in kernel_locator {
            let pixel = image.get_pixel(loc[0], loc[1]);
            if let Rgba(img) = pixel {
                colors.push(img.to_vec());
            } else {
                panic!("something went wrong with the pixel!")
            };

        }
        colors
    }
    fn kernel_greys(&self, kernel_locator: &Vec<[u32; 2]>, image: &DynamicImage) -> Vec<u8> {
        println!("{:?}", kernel_locator);
        assert_eq!(kernel_locator.len() as u32, self.kernel(),
        "kernel locator lenght and actual kernel value are not the same!");
        let image = image.to_luma();
        let mut colors = Vec::with_capacity(self.kernel() as usize);

        for loc in kernel_locator {
            let pixel = image.get_pixel(loc[0], loc[1]);
            if let Luma(img) = pixel {
                colors.push(img[0]);
            } else {
                panic!("something went wrong with the pixel!")
            };
        }
        colors
    }


    fn dominant_color_by_kernel(&self, kernel_locator: &Vec<[u32; 2]>, image: &DynamicImage) -> Vec<u8> {
        let colors: Vec<Vec<u8>> = self.kernel_colors(kernel_locator, image);
        let counter: Counter<_, u8> = colors.iter().collect();
        let f = counter.most_common_ordered();
        f[0].0.to_vec()
    }
    fn dominant_grey_by_kernel(&self, kernel_locator: &Vec<[u32; 2]>, image: &DynamicImage) -> u8 {
        let colors: Vec<u8> = self.kernel_greys(kernel_locator, image);
        let counter: Counter<_, u8> = colors.iter().collect();
        let f = counter.most_common_ordered();
        *f[0].0
    }

    pub fn to_char_cell(&self, start_pos: &[u32; 2], image:  &DynamicImage, ascii: &[char]) -> CharCell {
        let x: usize = ((start_pos[0] + self.kernel()) / 9) as usize;
        let y: usize = ((start_pos[1] + self.kernel()) / 9) as usize;
        let locators = self.get_kernel_locators(start_pos);
        let color = self.dominant_color_by_kernel(&locators, image);
        let grey_color = self.dominant_grey_by_kernel(&locators, image);
        let ascii = grey_to_ascii(grey_color, ascii);
        return CharCell { x , y , color: Color::Rgb(color[0], color[1], color[2]), ascii }
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