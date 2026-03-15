use std::path::PathBuf;
use std::thread;
use crate::core::types::ConversionConfig;
use crate::processor::convert::ImageConverter;
use crate::slint::AppWindow;

pub struct ConversionWorker;

impl ConversionWorker {
    pub fn spawn_processing_thread(
        ui_handle: slint::Weak<AppWindow>,
        config: ConversionConfig,
        tasks: Vec<PathBuf>,
    ) {
        // Validation: do not spawn thread if no tasks
        if tasks.is_empty() {
            return;
        }

        thread::spawn(move || {
            let total = tasks.len() as f32;
            let mut success_count = 0;

            for (index, input_path) in tasks.iter().enumerate() {
                let mut output_path = input_path.clone();
                let stem = input_path.file_stem().unwrap_or_default().to_string_lossy();

                // Dynamic extension from config instead of hardcoded .png
                let new_file_name = format!("conv_{}.{}", stem, config.output_format.to_lowercase());
                output_path.set_file_name(new_file_name);

                let result = ImageConverter::process_file(input_path, &output_path, &config);

                if result.is_ok() {
                    success_count += 1;
                }

                Self::update_ui_progress(&ui_handle, index + 1, total);
            }

            Self::finalize_ui(&ui_handle, success_count);
        });
    }

    fn update_ui_progress(ui_handle: &slint::Weak<AppWindow>, current: usize, total: f32) {
        let progress = current as f32 / total;
        let msg = format!("Processing: {} / {}", current, total as usize);

        let handle = ui_handle.clone();
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = handle.upgrade() {
                ui.set_progress(progress);
                ui.set_status_text(msg.into());
            }
        });
    }

    fn finalize_ui(ui_handle: &slint::Weak<AppWindow>, success: usize) {
        let handle = ui_handle.clone();
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = handle.upgrade() {
                ui.set_is_processing(false);
                ui.set_status_text(format!("Finished. Successfully processed {} images", success).into());
            }
        });
    }
}

