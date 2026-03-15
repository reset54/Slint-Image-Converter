use std::path::Path;
use anyhow::{Context, Result};
use crate::core::types::ConversionConfig;
use crate::processor::alpha::AlphaProcessor;


pub struct ImageConverter;


impl ImageConverter {
    pub fn process_file(input_path: &Path, output_path: &Path, config: &ConversionConfig) -> Result<()> {
        let img = image::open(input_path)
            .with_context(|| format!("Failed to open image: {:?}", input_path))?;

        let mut processed_img = AlphaProcessor::process(img, &config.alpha_mode);

        if let (Some(w), Some(h)) = (config.resize_width, config.resize_height) {
            processed_img = processed_img.resize(w, h, image::imageops::FilterType::Lanczos3);
        }

        processed_img.save(output_path)
            .with_context(|| format!("Failed to save image to: {:?}", output_path))?;

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::AlphaMode;

    #[test]
    fn test_process_file_invalid_path() {
        let config = ConversionConfig {
            output_format: "png".to_string(),
            quality: 100,
            alpha_mode: AlphaMode::Keep,
            resize_width: None,
            resize_height: None,
        };

        let result = ImageConverter::process_file(
            Path::new("non_existent.jpg"),
            Path::new("output.png"),
            &config
        );

        assert!(result.is_err());
    }
}
