use std::env;
use crate::core;
use native_dialog::FileDialog;
use tauri::AppHandle;
use webbrowser;

#[tauri::command]
pub async fn get_os() -> String {
    env::consts::OS.to_string()
}

#[tauri::command]
pub async fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[tauri::command]
pub async fn open_url(url: String) {
    webbrowser::open(&url).unwrap_or_else(|e| eprintln!("Couldn't open url: {}", e));
}

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
pub async fn begin(app: AppHandle, path: String, crf: String, preset: String, hevc: bool) {
    core::begin(&app, &path, &crf, &preset, &hevc).await;
}
