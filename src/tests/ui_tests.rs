#[cfg(test)]
mod tests {
    use crate::image_converter_app::{ImageConverterApp, UiController};
    use slint::{SharedString, Model};

    #[test]
    fn test_ui_complete_workflow() {
        // Single initialization for the entire test suite
        let app = ImageConverterApp::new().expect("Failed to create app");
        app.setup_callbacks();

        // --- 1. Test Duplicate Prevention ---
        let path: SharedString = "W:/images/photo.jpg".into();

        // Simulating the logic from callbacks.rs manually since we can't trigger native dialog
        for _ in 0..2 {
            let is_duplicate = (0..app.files_model.row_count()).any(|i| {
                app.files_model.row_data(i) == Some(path.clone())
            });
            if !is_duplicate {
                app.files_model.push(path.clone());
            }
        }
        assert_eq!(app.files_model.row_count(), 1, "Should skip duplicates");

        // --- 2. Test Removal Logic ---
        app.files_model.push("W:/images/second.png".into());
        app.ui.invoke_remove_file(0);
        assert_eq!(app.files_model.row_count(), 1);
        assert_eq!(app.files_model.row_data(0), Some("W:/images/second.png".into()));

        // --- 3. Test Clear Logic ---
        app.ui.invoke_clear_pressed();
        assert_eq!(app.files_model.row_count(), 0);
        assert_eq!(app.ui.get_has_files(), false);

        // --- 4. Test Conversion Start State ---
        app.files_model.push("test.jpg".into());
        app.ui.set_has_files(true);
        app.ui.invoke_convert_requested();

        assert_eq!(app.ui.get_is_processing(), true, "UI must lock during processing");
        assert!(
            app.ui.get_status_text().to_string().contains("Preparing"),
            "Status text mismatch"
        );
    }
}

