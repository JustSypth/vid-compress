use tauri::Manager;
use tauri::Size;

mod commands;
mod core;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .setup(|app| {
        let main_window = app.get_webview_window("main").unwrap();
        main_window.set_size(Size::Logical(tauri::LogicalSize {
            width: 402.0,
            height: 522.0,
        })).unwrap();
        Ok(())
    })
    .invoke_handler(tauri::generate_handler![
        commands::start, commands::stop, commands::open_url,
        commands::get_os, commands::get_version, commands::open_file,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
