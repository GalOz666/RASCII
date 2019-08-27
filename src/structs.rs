use image::{self, imageops::blur, png::PNGReader::Read, GenericImage, DynamicImage, ImageDecoder, ImageBuffer, GenericImageView, Rgb, RgbImage, Rgba};

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

    fn get_kernel_locators(&self, start_pos: &[usize; 2]) -> vec<[usize; 2]> {
        let mut kern : Vec<usize> = Vec::new();
        for outer_num in 0..self.kernel {
            for inner_num in 0..self.kernel {
                kern.push([start_pos.0+outer_num, start_pos.1+inner_num]);
            }
        kern
        }
    }

    fn kernel_colors(&self, &kernel_locator: vec<[usize;2]>, image: &F) -> vec<vec<T>>;

    fn dominant_color_by_kernel(&self, image: &F) -> vec<T>{ // todo: fill this with real func
        let colors = self.kernel_colors(image);
        colors[1]
    }

}

// for grey-scale images
impl KernelOperations<u8, RgbImage> for Kernel {                 // image.as_rgb8().unwrap()
    fn kernel_colors(&self, &kernel_locator: vec<[usize; 2]>, image: &RgbImage) -> vec<vec<u8>> {
        assert!(kernel_locator.len() as u32 = self.kernel);

        let mut colors = Vec::new();

        for (x,y) in kernel_locator {
                pixel: &Rgb<u8> = image.get_pixel(x, y);
                let pixel(num) = grey_color;
                colors.push(pixel);

        }
        colors
    }
}
