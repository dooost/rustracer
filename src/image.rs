use std::path::Path;

use imagers::EncodableLayout;
use imagers::Pixel;
use imagers::ImageBuffer;
use imagers::Rgb;

pub struct Image<P: Pixel> {
    pub width: u32,
    pub height: u32,
    buf: ImageBuffer<P, Vec<P::Subpixel>>
}

impl<P> Image<P>
where
    P: Pixel + 'static,
    P::Subpixel: 'static,
{
    pub fn new(width: u32, height: u32) -> Self {
        Image {
            width,
            height,
            buf: ImageBuffer::new(width, height)
        }
    }
}

impl Image<Rgb<u8>> {

    pub fn set_color_at_pixel(&mut self, x: u32, y: u32, r: f32, g: f32, b: f32) {
        let ru8 = (r * u8::MAX as f32) as u8;
        let gu8 = (g * u8::MAX as f32) as u8;
        let bu8 = (b * u8::MAX as f32) as u8;
        let pixel = Rgb([ru8, gu8, bu8]);
        self.buf.put_pixel(x, y, pixel);
    }
}

impl<P: Pixel + 'static> Image<P>
where
    [P::Subpixel]: EncodableLayout,
{

    pub fn write_png<Q: AsRef<Path>>(&self, p: Q) {
        self.buf.save(p).unwrap();
    }
}