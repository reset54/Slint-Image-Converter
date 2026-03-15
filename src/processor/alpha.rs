use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use crate::core::types::AlphaMode;


pub struct AlphaProcessor;


impl AlphaProcessor {
    pub fn process(img: DynamicImage, mode: &AlphaMode) -> DynamicImage {
        match mode {
            AlphaMode::Keep => img,
            AlphaMode::Remove => {
                DynamicImage::ImageRgb8(img.to_rgb8())
            }
            AlphaMode::Premultiply => {
                let (width, height) = img.dimensions();
                let mut output = ImageBuffer::new(width, height);
                let rgba_img = img.to_rgba8();

                for (x, y, pixel) in rgba_img.enumerate_pixels() {
                    let alpha = pixel[3] as f32 / 255.0;
                    let r = (pixel[0] as f32 * alpha) as u8;
                    let g = (pixel[1] as f32 * alpha) as u8;
                    let b = (pixel[2] as f32 * alpha) as u8;
                    
                    output.put_pixel(x, y, Rgba([r, g, b, pixel[3]]));
                }

                DynamicImage::ImageRgba8(output)
            }
            AlphaMode::Constant(level) => {
                let (width, height) = img.dimensions();
                let mut output = ImageBuffer::new(width, height);
                let rgba_img = img.to_rgba8();

                for (x, y, pixel) in rgba_img.enumerate_pixels() {
                    let mut new_pixel = *pixel;
                    new_pixel[3] = *level;
                    output.put_pixel(x, y, new_pixel);
                }

                DynamicImage::ImageRgba8(output)
            }
        }
    }
}
