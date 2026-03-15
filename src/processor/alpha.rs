use image::{DynamicImage, Rgba};
use crate::core::types::AlphaMode;


pub struct AlphaProcessor;


impl AlphaProcessor {
    pub fn process(img: DynamicImage, mode: &AlphaMode) -> DynamicImage {
        match mode {
            AlphaMode::Keep => img,

            AlphaMode::Remove => {
                // to_rgb8() effectively removes the alpha channel
                DynamicImage::ImageRgb8(img.to_rgb8())
            }

            AlphaMode::Constant(level) => {
                let mut rgba = img.to_rgba8();
                // Direct modification of the buffer is faster than put_pixel
                for pixel in rgba.pixels_mut() {
                    pixel[3] = *level;
                }
                DynamicImage::ImageRgba8(rgba)
            }

            AlphaMode::Premultiply => {
                let mut rgba = img.to_rgba8();
                for pixel in rgba.pixels_mut() {
                    let alpha = pixel[3] as f32 / 255.0;
                    pixel[0] = (pixel[0] as f32 * alpha) as u8;
                    pixel[1] = (pixel[1] as f32 * alpha) as u8;
                    pixel[2] = (pixel[2] as f32 * alpha) as u8;
                }
                DynamicImage::ImageRgba8(rgba)
            }
        }
    }
}
