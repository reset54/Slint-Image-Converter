use std::path::PathBuf;


pub enum AlphaMode {
    Keep,
    Remove,
    Premultiply,
    Constant(u8),
}


pub struct ConversionConfig {
    pub output_format: String,
    pub quality: u8,
    pub alpha_mode: AlphaMode,
    pub resize_width: Option<u32>,
    pub resize_height: Option<u32>,
}


pub struct ImageTask {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub config: ConversionConfig,
}
