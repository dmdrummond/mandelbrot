extern crate num;
extern crate image;

use num::Complex;
use image::{ImageBuffer};
use std::fs::File;


fn main() {

    let width = 7000;
    let height = 4000;

    let x_factor = 3.5 / width as f64;
    let y_factor = 2.0 / height as f64;

    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let mut z = Complex::new(0.0, 0.0);
        let c = Complex::new((x as f64 * x_factor) - 2.5, (y as f64 * y_factor) - 1.0 );
        let mut i = 0;
        let max = 10000;

        while i < max && z.norm() < 2.0 {
            i = i + 1;
            z = z * z + c
        }
        if i < max {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });

    let ref mut fout = File::create("fractal3.png").unwrap();

    // We must indicate the image's color type and what format to save as
    image::ImageLuma8(img).save(fout, image::PNG).unwrap();

}
