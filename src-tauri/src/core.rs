use std::panic;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
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

lazy_static::lazy_static! {
    static ref STOPPED: AtomicBool = AtomicBool::new(false);
    static ref CHILD_PIDS: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(Vec::new()));
    static ref ANIMATION_HANDLE: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>> = Arc::new(Mutex::new(None));
}

pub fn set_panic(app: Arc<AppHandle>) {
    panic::set_hook(Box::new(move |panic_info| {
        let error_msg = format!("{}\n{}", "The app's core failed with this message:".bold(), panic_info).red();
        eprintln!("{error_msg}");
        app.emit(PROCESSING, "false").unwrap();
    }));
}

pub async fn stop() {
    if let Some(handle) = ANIMATION_HANDLE.lock().unwrap().take() {
        handle.abort();
    }

    let pids = {
        let mut guard = CHILD_PIDS.lock().unwrap();
        std::mem::take(&mut *guard)
    };

    for pid in pids {
        println!("Killing PID: {pid}");
        kill_pid(pid).unwrap_or_else(|e|
            eprintln!("{}", e)
        );
    }

    STOPPED.store(true, Ordering::Release);
}

pub async fn begin(app: &AppHandle, path: &String, crf: &String, preset: &String, audio_bitrate: &String, hevc_enabled: &bool) {
    let app_arc = Arc::new(app.clone());
    set_panic(app_arc);

    app.emit(PROCESSING, "true").unwrap();

    let ffmpeg = get_binary("ffmpeg");
    let watchdog = get_binary("vid-compress-watchdog");

    let path: PathBuf = PathBuf::from(path);
    let mut codec = "h264";
    if *hevc_enabled {
        codec = "hevc";
    }


    if !is_video(&path) {
        eprintln!("{}", "Invalid file path provided".red());
        app.emit(STATUS, "Please enter a valid video file.").unwrap();
        app.emit(PROCESSING, "false").unwrap();
        return;
    }

    let input_path = path.display().to_string();
    let output_path = path.with_file_name(format!(
            "{}-output.{}",
            path.file_stem().unwrap().to_str().unwrap(),
            path.extension().unwrap().to_str().unwrap()
        ));

    let output_path_str = output_path.display().to_string();
    let execute_arg = vec![
        "-i", &input_path,
        "-vcodec", &codec,
        "-crf", &crf,
        "-preset", &preset,
        "-acodec", "aac",
        "-b:a", &audio_bitrate,
        "-y", &output_path_str,
    ];

    println!("{}", format!("{} {}", "Set codec:".bold(), codec).blue());
    let process_message = format!(
        "{}\n{} {}",
        "Processing file with this command:".bold(),
        "ffmpeg".italic(),
        execute_arg.join(" ").italic()
    );
    println!("{process_message}");

    let animation_handle: JoinHandle<()> = tokio::spawn(play_compressing(app.clone()));
    *ANIMATION_HANDLE.lock().unwrap() = Some(animation_handle);
    
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

    let main_pid = std::process::id();
    let ffmpeg_pid = child_ffmpeg.id().unwrap();

    let mut child_watchdog: Child;
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        child_watchdog = Command::new(watchdog)
        .args([main_pid.to_string(), ffmpeg_pid.to_string()])
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

    let watchdog_pid = child_watchdog.id().unwrap();
    CHILD_PIDS.lock().unwrap().extend([ffmpeg_pid, watchdog_pid]);
    
    let status = child_ffmpeg.wait().await.unwrap(); //wait until compressing is over
    
    let mut stderr = String::from("");
    child_ffmpeg.stderr.take().unwrap().read_to_string(&mut stderr).await.unwrap();

    CHILD_PIDS.lock().unwrap().clear();
    child_watchdog.kill().await.unwrap_or_else(|e| eprintln!("{} {e}", "Could not kill watchdog:".red().bold()));
    if let Some(handle) = ANIMATION_HANDLE.lock().unwrap().take() {
        handle.abort();
    }

    let message = if STOPPED.load(Ordering::Acquire) {
        STOPPED.store(false, Ordering::Release);
        format!("Forcefully stopped compression")
    } else if status.success() {
        let temp_msg = String::from("Video compressed successfully");
        let message = format!("{}", temp_msg.green().bold());
        println!("{message}");
        temp_msg
        
    } else {
        format!("Process failed with status: {}", &status)
    };

    app.emit(STATUS, message).unwrap();

    app.emit(PROCESSING, "false").unwrap();
}

fn kill_pid(pid: u32) -> Result<(), String> {
    #[cfg(unix)]
    {
        use nix::sys::signal::{self, Signal};
        let pid = nix::unistd::Pid::from_raw(pid as i32);

        match signal::kill(pid, Signal::SIGKILL) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Couldn't kill process: {}", e))
        }
    }
    #[cfg(windows)] {
        use std::os::windows::process::CommandExt;
        let output = Command::new("taskkill")
        .args(["/FI", &format!("PID eq {}", pid.to_string())])
        .creation_flags(0x08000000)
        .output();

        match output {
            Ok(_) => todo!(),
            Err(e) => Err(format!("Couldn't kill process: {}", e)),
        }
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