use std::path::Path;
use std::path::PathBuf;
use std::process::Output;
use tauri::{AppHandle, Emitter};
use tokio::process::Command;
use tokio::time::Duration;
use tokio::time::sleep;
use tokio::task::JoinHandle;
use colored::Colorize;

const STATUS: &str = "STATUS";
const PROCESSING: &str = "PROCESSING";

pub async fn begin(app: &AppHandle, path: &String, cfg: &String, preset: &String) {
    let path: PathBuf = PathBuf::from(path);

    if !is_video(&path) {
        eprintln!("{}", "Invalid file path provided".red());
        app.emit(STATUS, "Please enter a valid video file.").unwrap();
        return;
    }

    print_debug(&path, &cfg, &preset);

    let output_path = path.with_file_name(format!(
            "{}-output.{}",
            path.file_stem().unwrap().to_str().unwrap(),
            path.extension().unwrap().to_str().unwrap()
        ));

    let input_path = path.display().to_string();
    let crf_value = cfg.to_string();
    let output_path_str = output_path.display().to_string();

    let execute_arg = vec![
        "-i", &input_path,
        "-vcodec", "libx264",
        "-crf", &crf_value,
        "-preset", &preset,
        "-acodec", "aac",
        "-b:a", "128k",
        "-y", &output_path_str,
    ];
    
    let handle: JoinHandle<()> = tokio::spawn(play_compressing(app.clone()));

    let process_command = format!(
        "{}\n{} {}",
        "Processing file with this command:".bold(),
        "ffmpeg".italic(),
        execute_arg.join(" ").italic()
    );
    println!("{process_command}");
    
    app.emit(PROCESSING, "true").unwrap();

    let ffmpeg = get_ffmpeg();
    let execute: Output;
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        execute = Command::new(ffmpeg)
        .args(&execute_arg)
        .creation_flags(0x08000000)
        .output()
        .await
        .unwrap();
    }
    #[cfg(unix)]
    {
        execute = Command::new(ffmpeg)
        .args(&execute_arg)
        .output()
        .await
        .unwrap();
    }

    app.emit(PROCESSING, "false").unwrap();
    handle.abort();
    
    if execute.status.success() {
        let message = "Video compressed successfully";
        app.emit(STATUS, &message).unwrap();
        let message = format!("{}", message.green().bold());
        println!("{message}")
    } else {
        let message = format!("Process failed with status: {}", execute.status);
        eprint!("{}", String::from_utf8_lossy(&execute.stderr));
        app.emit(STATUS, message).unwrap();
    }
}

fn print_debug(path: &PathBuf, cfg: &String, preset: &String) {
    let debug_message = format!(
        "{} Path: {} CFG: {} Preset: {}",
        "DEBUG:".bold(),
        path.to_string_lossy().to_string(),
        cfg,
        preset
    );

    println!("{}", debug_message.blue());
}

fn is_video(path: &PathBuf) -> bool {
    let video_extensions: [&str; 7] = ["mp4", "avi", "mkv", "mov", "flv", "wmv", "webm"];

    if path.exists() && path.is_file() {
        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) { //whether the file even has an extension
            return video_extensions.contains(&extension.to_lowercase().as_str()); //whether the file has a vid extension
        }
    }

    false
}

fn get_ffmpeg() -> PathBuf {
    if std::env::var("CARGO").is_ok() {
        return Path::new("../bin").join(if cfg!(windows) { "ffmpeg-windows/bin/ffmpeg.exe" } else { "ffmpeg-linux/ffmpeg" });
    } else {
        return Path::new("bin").join(if cfg!(windows) { "ffmpeg.exe" } else { "ffmpeg" });
    }
}

async fn play_compressing(app: AppHandle) {
    let frames: [&str; 4] = ["Compressing", "Compressing.", "Compressing..", "Compressing..."];
    let mut index = 0;
    
    loop {
        app.emit(STATUS, frames[index]).unwrap();
        index = (index + 1) % frames.len(); //to cycle
        sleep(Duration::from_millis(500)).await;
    }
}