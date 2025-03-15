use std::path::Path;
use std::path::PathBuf;
use std::process::Stdio;
use tauri::{AppHandle, Emitter};
use tokio::io::AsyncReadExt;
use tokio::process::Child;
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

    let debug_message = format!(
        "{} Path: {} CFG: {} Preset: {}",
        "DEBUG:".bold(),
        path.to_string_lossy().to_string(),
        cfg,
        preset
    );
    println!("{}", debug_message.blue());

    let input_path = path.display().to_string();
    let output_path = path.with_file_name(format!(
            "{}-output.{}",
            path.file_stem().unwrap().to_str().unwrap(),
            path.extension().unwrap().to_str().unwrap()
        ));
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
    
    let process_message = format!(
        "{}\n{} {}",
        "Processing file with this command:".bold(),
        "ffmpeg".italic(),
        execute_arg.join(" ").italic()
    );
    println!("{process_message}");

    app.emit(PROCESSING, "true").unwrap();
    let animation_handle: JoinHandle<()> = tokio::spawn(play_compressing(app.clone()));

    let ffmpeg = get_binary("ffmpeg");
    let mut child_ffmpeg: Child;
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        child_ffmpeg = Command::new(ffmpeg)
        .args(&execute_arg)
        .creation_flags(0x08000000)
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    }
    #[cfg(unix)]
    {
        child_ffmpeg = Command::new(ffmpeg)
        .args(&execute_arg)
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    }

    let watchdog = get_binary("vid-compress-watchdog");
    let mut child_watchdog: Child;
    let main_pid = std::process::id();
    let ffmpeg_pid = child_ffmpeg.id().expect("Failure getting child pid!");
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        child_watchdog = Command::new(watchdog)
        .args([main_pid.to_string(), child_pid.to_string()])
        .creation_flags(0x08000000)
        .spawn()
        .unwrap();
    }
    #[cfg(unix)]
    {
        child_watchdog = Command::new(watchdog)
        .args([main_pid.to_string(), ffmpeg_pid.to_string()])
        .spawn()
        .unwrap();
    }

    let status = child_ffmpeg.wait().await.unwrap();
    let mut stderr = String::from("");
    child_ffmpeg.stderr.take().unwrap().read_to_string(&mut stderr).await.unwrap();

    child_watchdog.kill().await.unwrap_or_else(|e| eprintln!("{} {e}", "Could not kill watchdog:".red().bold()));
    app.emit(PROCESSING, "false").unwrap();
    animation_handle.abort();
    
    if status.success() {
        let message = "Video compressed successfully";
        app.emit(STATUS, &message).unwrap();
        let message = format!("{}", message.green().bold());
        println!("{message}")
    } else {
        let message = format!("Process failed with status: {}", &status);
        // eprint!("{}", &stderr);
        app.emit(STATUS, message).unwrap();
    }
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

fn get_binary(binary_name: &str) -> PathBuf {
    if std::env::var("CARGO").is_ok() {
        return Path::new("../bin").join(if cfg!(windows) { format!("{binary_name}.exe") } else { format!("{binary_name}") });
    } else {
        return Path::new("bin").join(if cfg!(windows) { format!("{binary_name}.exe") } else { format!("{binary_name}") });
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