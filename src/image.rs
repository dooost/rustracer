use std::path::Path;

use imagers::ImageBuffer;
use imagers::Rgb;

use crate::color::RgbColor;

pub struct Image {
    pub width: u32,
    pub height: u32,
    buf: ImageBuffer<Rgb<u8>, Vec<u8>>
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Image {
            width,
            height,
            buf: ImageBuffer::new(width, height)
        }
    }

    pub fn set_color_at(&mut self, x: u32, y: u32, color: RgbColor, samples_per_pixel: u32) {
        let adjusted_color = color / (samples_per_pixel as f32);
        let ru8 = (adjusted_color.x * u8::MAX as f32) as u8;
        let gu8 = (adjusted_color.y * u8::MAX as f32) as u8;
        let bu8 = (adjusted_color.z * u8::MAX as f32) as u8;
        let pixel = Rgb([ru8, gu8, bu8]);
        self.buf.put_pixel(x, y, pixel);
    }

    pub fn write_png<Q: AsRef<Path>>(&self, p: Q) {
        self.buf.save(p).unwrap();
    }
}
