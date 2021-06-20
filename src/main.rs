extern crate image as imagers;
use crate::image::Image;
use imagers::Rgb;

mod image;

fn main() {
    let w = 1280;
    let h = 720;
    let mut img: Image<Rgb<u8>> = Image::new(w, h);

    for x in 0..w {
        for y in 0..h {
            img.set_color_at_pixel(x, 
                y, 
                (x as f32) / ((w - 1) as f32), 
                (y as f32) / ((h - 1) as f32), 
                1.0)
        }
    }

    img.write_png("fractal.png");
}