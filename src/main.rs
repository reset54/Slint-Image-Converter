pub mod core {
    pub mod types;
}

pub mod processor {
    pub mod alpha;
    pub mod convert;
    pub mod worker;
}

pub mod image_converter_app;

use anyhow::Result;
use crate::image_converter_app::ImageConverterApp;


// Encapsulate Slint modules to avoid duplication errors
pub mod slint {
    slint::include_modules!();
}


fn main() -> Result<()> {
    let app = ImageConverterApp::new()?;
    app.run()?;

    Ok(())
}

