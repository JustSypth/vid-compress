use crate::core;
use std::path::PathBuf;
use native_dialog::FileDialog;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_path() -> Result<String, String> {
    let path = FileDialog::new()
        .add_filter(
            "Video Files",
            &["mp4", "avi", "mkv", "mov", "flv", "wmv", "webm"],
        )
        .show_open_single_file()
        .map_err(|e| e.to_string())?;

    match path {
        Some(path) => Ok(path.to_string_lossy().to_string()),
        None => Err("No file selected".to_string()),
    }
}

#[tauri::command]
pub async fn begin(app: AppHandle, path: String, cfg: String, preset: String) {
    println!("DEBUG: Path: {path} CFG: {cfg} Preset: {preset}");

    let preset = if preset.is_empty() {"slow"} else {&preset}.to_string();
    let path = PathBuf::from(path);

    core::begin(&app, &path, &cfg, &preset);
}
