use std::path::PathBuf;
use std::rc::Rc;
use slint::{ComponentHandle, VecModel, SharedString, Model};
use anyhow::{Context, Result};

use crate::core::types::{AlphaMode, ConversionConfig};
use crate::processor::worker::ConversionWorker;
use crate::slint::AppWindow;


pub struct ImageConverterApp {
    ui: AppWindow,
    files_model: Rc<VecModel<SharedString>>,
}


impl ImageConverterApp {
    pub fn new() -> Result<Self> {
        let ui = AppWindow::new().context("Failed to initialize Slint window")?;
        let files_model = Rc::new(VecModel::<SharedString>::default());

        ui.set_file_list(files_model.clone().into());

        Ok(Self { ui, files_model })
    }


    pub fn run(self) -> Result<()> {
        self.setup_callbacks();
        self.ui.run()?;
        Ok(())
    }


    fn setup_callbacks(&self) {
        let ui_handle = self.ui.as_weak();
        let files_model = self.files_model.clone();

        // Handler for clearing the list
        let clear_model = files_model.clone();
        let clear_ui_handle = ui_handle.clone();
        self.ui.on_clear_pressed(move || {
            clear_model.set_vec(vec![]);
            if let Some(ui) = clear_ui_handle.upgrade() {
                ui.set_status_text("Queue cleared".into());
            }
        });

        // Add files handler
        let add_ui_handle = ui_handle.clone();
        let add_files_model = files_model.clone();
        self.ui.on_add_files(move || {
            Self::handle_add_files(add_ui_handle.clone(), add_files_model.clone());
        });

        // Convert requested
        let convert_ui_handle = ui_handle.clone();
        let convert_files_model = files_model.clone();
        self.ui.on_convert_requested(move || {
            Self::handle_conversion(convert_ui_handle.clone(), convert_files_model.clone());
        });
    }


    fn handle_add_files(ui_handle: slint::Weak<AppWindow>, model: Rc<VecModel<SharedString>>) {
        let files = rfd::FileDialog::new()
            .add_filter("Images", &["png", "jpg", "jpeg", "webp", "bmp", "tga"])
            .pick_files();

        if let Some(paths) = files {
            for path in paths {
                model.push(path.to_string_lossy().to_string().into());
            }

            if let Some(ui) = ui_handle.upgrade() {
                ui.set_status_text(format!("Added {} files", model.row_count()).into());
            }
        }
    }


    fn handle_conversion(ui_handle: slint::Weak<AppWindow>, model: Rc<VecModel<SharedString>>) {
        let ui = match ui_handle.upgrade() {
            Some(instance) => instance,
            None => return,
        };

        ui.set_is_processing(true);
        ui.set_progress(0.0);

        // Determine alpha mode based on checkbox
        let alpha_mode = if ui.get_use_alpha_channel() {
            let level = (ui.get_alpha_value() * 255.0) as u8;
            crate::core::types::AlphaMode::Constant(level)
        } else {
            crate::core::types::AlphaMode::Keep
        };

        let config = ConversionConfig {
            output_format: ui.get_selected_format().to_string(),
            quality: 90, // Default for now
            alpha_mode,
            resize_width: None,
            resize_height: None,
        };

        let mut tasks = Vec::new();
        for i in 0..model.row_count() {
            if let Some(file_str) = model.row_data(i) {
                tasks.push(std::path::PathBuf::from(file_str.as_str()));
            }
        }

        ConversionWorker::spawn_processing_thread(ui_handle, config, tasks);
    }
}
