extern crate image as imagers;

mod image;
mod math;
mod color;

use crate::image::Image;
use color::RgbColor;

fn main() {
    let w = 1280;
    let h = 720;
    let mut img = Image::new(w, h);

    for x in 0..w {
        for y in 0..h {
            let color = RgbColor::new(
                (x as f32) / ((w - 1) as f32),
                (y as f32) / ((h - 1) as f32),
                1.0
            );
            img.set_color_at(x, y, color);
        }
    }

    img.write_png("fractal.png");
}