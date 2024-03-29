use std::collections::HashMap;

use counter::Counter;
use image::{self, DynamicImage, GenericImageView, GrayImage, Luma, Rgba};

use crate::grey_to_ascii;

pub struct CharCell<'a> {

    pub color: (u8, u8, u8),
    pub ascii:  &'a mut char,

}

#[derive(Debug)]
pub struct Kernel {

    kernel: u32,
    _cache: HashMap<u8, char>,

}

impl Kernel {
    pub fn new(num: u32) -> Self {
        Kernel { kernel: num, _cache: HashMap::new() }
    }
    pub fn kernel(&self) -> u32 {
        self.kernel
    }

    fn get_kernel_locators(&self, start_pos: &[u32; 2]) -> Vec<[u32; 2]> {
        let mut kern: Vec<[u32; 2]> = Vec::with_capacity(self.kernel as usize);
        let num = (self.kernel() as f64).sqrt() as u32;
        assert_eq!(num as f64, (self.kernel() as f64).sqrt(),
        "kernel needs to have a non float square root!");
        for outer_num in 0..num {
            for inner_num in 0..num {
                kern.push([start_pos[0] + outer_num, start_pos[1] + inner_num]);
            }
        }
        kern
    }


    fn kernel_colors(&self, kernel_locator: &Vec<[u32; 2]>, image: &DynamicImage) -> Vec<Vec<u8>> {
        assert_eq!(kernel_locator.len() as u32, self.kernel(),
                   "kernel locator length and actual kernel value are not the same!");
        let mut colors = Vec::with_capacity(self.kernel() as usize);

        for loc in kernel_locator {
            let pixel = image.get_pixel(loc[0], loc[1]);
            let Rgba(img) = pixel;
            colors.push(img.to_vec())

        }
        colors
    }
    fn kernel_greys(&self, kernel_locator: &Vec<[u32; 2]>, image: &GrayImage) -> Vec<u8> {
        assert_eq!(kernel_locator.len() as u32, self.kernel(),
        "kernel locator lenght and actual kernel value are not the same!");
        let mut colors = Vec::with_capacity(self.kernel() as usize);

        for loc in kernel_locator {
            let pixel = image.get_pixel(loc[0], loc[1]);
            let Luma(img) = pixel;
            colors.push(img[0]);

        }
        colors
    }


    fn dominant_color_by_kernel(&self, kernel_locator: &Vec<[u32; 2]>, image: &DynamicImage) -> Vec<u8> {
        let colors: Vec<Vec<u8>> = self.kernel_colors(kernel_locator, image);
        let counter: Counter<_, u8> = colors.iter().collect();
        let f = counter.most_common_ordered();
        f[0].0.to_vec()
    }
    fn dominant_grey_by_kernel(&self, kernel_locator: &Vec<[u32; 2]>, grey: &GrayImage) -> u8 {
        let colors: Vec<u8> = self.kernel_greys(kernel_locator, grey);
        let counter: Counter<_, u8> = colors.iter().collect();
        let f = counter.most_common_ordered();
        *f[0].0
    }

    pub fn get_char_cell(&mut self, start_pos: &[u32; 2], image:  &DynamicImage, grey: &GrayImage, ascii_list: &'static [char]) -> CharCell {

        let x  = start_pos[0] as f64 / self.kernel() as f64;
        let y = start_pos[1] as f64 / self.kernel() as f64;
        assert_eq!(x.trunc(), x, "x start position does not conform to kernel size");
        assert_eq!(y.trunc(), y, "y start position does not conform to kernel size");
        let locators = self.get_kernel_locators(start_pos);
        let color = self.dominant_color_by_kernel(&locators, image);
        let grey_color = self.dominant_grey_by_kernel(&locators, grey);
        // caching unknown / fetching known results:
        let ascii = self._cache
                .entry(grey_color)
                .or_insert_with(|| grey_to_ascii(grey_color, ascii_list));

        CharCell { color: (color[0], color[1], color[2]), ascii }
    }
}
