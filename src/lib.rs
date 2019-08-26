mod pixle_analysis {

    const ASCII_CHARS: (char) = ('#', ' . ', )

/// kernel based  RGB color averaging for each cell:
/// different shades of blue would need to be in the same character (done by analysis of color distance in LAB space:
/// https://www.pyimagesearch.com/2016/02/15/determining-object-color-with-opencv/)
/// OR:
/// different shades of the same color will be the same color - meaning that the image will be compressed into ~20 colors accordingly -
/// with each color having a unique character assignment (character assignment is unique on each run - as it's more fun) (~20 as per number of colors)

/// combine both to a cell struct along with a locator(x, y)
/// such that we will have a cell with character + color + location, read to be written
/// into the terminal

}


mod image_handling {


    use image::imageops::blur;
    use image::png::PNGReader::Read;

    // import image: either .png or .jpg -> with GenericImageView trat
    // I think we can do it with O:read image::png::PNGDecoder:new(raw_image)?

    // get dimensions and crop to fit pixel-kernel-size - image.dimensions()...


    /// process image (gaussian blur)
    /// each mapped to a character (assignment will be semi-random as it's more fun!)

    /// save ascii to .png image (other formats maybe too?)
}
