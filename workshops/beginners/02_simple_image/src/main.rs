extern crate image;

use std::fs::File;
use std::path::Path;

fn main() {
    let image_size = 400;
    let tiles_per_row = 4;
    let tile_size = image_size / tiles_per_row;

    let mut imgbuf = image::ImageBuffer::new(image_size, image_size);

    let red = [255, 0, 0];
    let blue = [0, 255, 0];
    let green = [0, 0, 255];
    let black = [0, 0, 0];

    let colors = [
        [red, blue, green, black],
        [blue, green, black, red],
        [green, black, red, blue],
        [black, red, blue, green],
    ];

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let xx = x / tile_size;
        let yy = y / tile_size;

        let rgb = colors[xx as usize][yy as usize];

        *pixel = image::Rgb(rgb);
    }

    let ref mut fout = File::create(&Path::new("image.png")).unwrap();

    let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}
