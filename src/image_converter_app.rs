use std::path::PathBuf;
use std::rc::Rc;
use slint::{ComponentHandle, VecModel, SharedString, Model};
use anyhow::{Context, Result};
use rfd::FileDialog;

use crate::slint::AppWindow;

/// Trait defining the UI interaction contract
pub trait UiController {
    fn setup_callbacks(&self);
}

pub struct ImageConverterApp {
    pub(crate) ui: AppWindow,
    pub(crate) files_model: Rc<VecModel<SharedString>>,
}

impl ImageConverterApp {
    /// Constructor: initializes Slint window and data models
    pub fn new() -> Result<Self> {
        let ui = AppWindow::new()
            .context("Failed to initialize Slint window")?;

        let files_model = Rc::new(VecModel::<SharedString>::default());

        // Initial state sync
        ui.set_file_list(files_model.clone().into());
        ui.set_has_files(false);

        Ok(Self {
            ui,
            files_model,
        })
    }

    /// Application entry point: sets up logic and starts event loop
    pub fn run(self) -> Result<()> {
        // Принудительно используем метод трейта
        UiController::setup_callbacks(&self);

        self.ui.run().context("Failed to execute Slint event loop")?;
        Ok(())
    }

    /// Opens native OS dialog to select multiple images
    pub(crate) fn select_files() -> Option<Vec<PathBuf>> {
        FileDialog::new()
            .add_filter("Images", &["png", "jpg", "jpeg", "webp", "bmp", "tga"])
            .set_title("Select images to convert")
            .pick_files()
    }

    /// Bridges UI state with the background processing worker
    pub(crate) fn handle_conversion(ui_handle: slint::Weak<AppWindow>, model: Rc<VecModel<SharedString>>) {
        let ui = match ui_handle.upgrade() {
            Some(instance) => instance,
            None => return,
        };

        ui.set_is_processing(true);
        ui.set_status_text("Preparing tasks...".into());

        // Extract tasks from the model
        let mut tasks = Vec::new();
        for i in 0..model.row_count() {
            if let Some(file_str) = model.row_data(i) {
                tasks.push(PathBuf::from(file_str.as_str()));
            }
        }

        // Build config from current UI properties
        let config = crate::core::types::ConversionConfig {
            output_format: ui.get_selected_format().to_string(),
            quality: 90,
            alpha_mode: if ui.get_use_alpha_channel() {
                crate::core::types::AlphaMode::Constant((ui.get_alpha_value() * 255.0) as u8)
            } else {
                crate::core::types::AlphaMode::Keep
            },
            resize_width: None,
            resize_height: None,
        };

        // Offload to background thread
        crate::processor::worker::ConversionWorker::spawn_processing_thread(
            ui_handle,
            config,
            tasks,
        );
    }
}

