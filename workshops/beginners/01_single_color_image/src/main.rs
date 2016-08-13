extern crate image;

use std::fs::File;
use std::path::Path;

fn main() {
    let image_size = 400;

    let mut imgbuf = image::ImageBuffer::new(image_size, image_size);

    let red = [255, 0, 0];

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb(red);
    }

    let ref mut fout = File::create(&Path::new("image.png")).unwrap();

    let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}
