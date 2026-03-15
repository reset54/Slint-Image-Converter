//use std::rc::Rc;
use slint::{ComponentHandle, Model};
use crate::image_converter_app::{ImageConverterApp, UiController};

impl UiController for ImageConverterApp {
    fn setup_callbacks(&self) {
        let ui_weak = self.ui.as_weak();
        let files_model = self.files_model.clone();
        let ui = ui_weak.unwrap();

        // 1. Add Files callback
        let ui_handle = ui_weak.clone();
        let model = files_model.clone();
        ui.on_add_files(move || {
            if let Some(new_files) = Self::select_files() {
                for path in new_files {
                    model.push(path.to_string_lossy().to_string().into());
                }
                if let Some(ui_instance) = ui_handle.upgrade() {
                    ui_instance.set_status_text("Files added".into());
                    ui_instance.set_has_files(true);
                }
            }
        });

        // 2. Remove Single File callback
        let ui_handle = ui_weak.clone();
        let model = files_model.clone();
        ui.on_remove_file(move |index| {
            let idx = index as usize;
            if idx < model.row_count() {
                model.remove(idx);

                if let Some(ui_instance) = ui_handle.upgrade() {
                    let count = model.row_count();
                    ui_instance.set_has_files(count > 0);
                    ui_instance.set_status_text(format!("Removed file. {} left", count).into());
                }
            }
        });

        // 3. Clear All callback
        let ui_handle = ui_weak.clone();
        let model = files_model.clone();
        ui.on_clear_pressed(move || {
            model.set_vec(vec![]);
            if let Some(ui_instance) = ui_handle.upgrade() {
                ui_instance.set_has_files(false);
                ui_instance.set_status_text("Queue cleared".into());
            }
        });

        // 4. Convert callback
        let ui_handle = ui_weak.clone();
        let model = files_model.clone();
        ui.on_convert_requested(move || {
            Self::handle_conversion(ui_handle.clone(), model.clone());
        });
    }
}
