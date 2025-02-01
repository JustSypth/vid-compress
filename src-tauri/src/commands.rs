use native_dialog::FileDialog;

#[tauri::command]
pub async fn get_path() -> Result<String, String> {
    let path = FileDialog::new()
        .add_filter("Video Files", &["mp4", "avi", "mkv", "mov", "flv", "wmv", "webm"])
        .show_open_single_file()
        .map_err(|e| e.to_string())?;

    match path {
        Some(path) => Ok(path.to_string_lossy().to_string()),
        None => Err("No file selected".to_string()),
    }
}