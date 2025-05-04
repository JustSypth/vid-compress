use std::panic;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::path::Path;
use std::path::PathBuf;
use std::process::Stdio;
use regex::Regex;
use tauri::{AppHandle, Emitter, Listener};
use tokio::sync::{mpsc, oneshot};
use tokio::sync::oneshot::channel;
use tokio::process::Child;
use tokio::process::Command;
use tokio::time::Duration;
use tokio::time::sleep;
use tokio::task::JoinHandle;
use tokio_util::codec::{FramedRead, LinesCodec};
use tokio_stream::StreamExt;
use colored::Colorize;

const PROCESSING: &str = "PROCESSING";
const STATUS: &str = "STATUS";
const PERCENTAGE: &str = "PERCENTAGE";
const VID_EXISTS: &str = "VID_EXISTS";
const RESPONSE_IGNORE_EXISTING: &str = "RESPONSE_IGNORE_EXISTING";

lazy_static::lazy_static! {
    static ref STOPPED: AtomicBool = AtomicBool::new(false);
    static ref CHILD_PIDS: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(Vec::new()));
    static ref ANIMATION_HANDLE: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>> = Arc::new(Mutex::new(None));
}

pub fn set_panic(app: Arc<AppHandle>) {
    panic::set_hook(Box::new(move |panic_info| {
        if let Some(handle) = ANIMATION_HANDLE.lock().unwrap().take() {
            handle.abort();
        }

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
        kill_pid(pid).await.unwrap_or_else(|e|
            eprintln!("{}", e)
        );
    }

    STOPPED.store(true, Ordering::Release);
}

pub async fn begin(app: &AppHandle, path: &String, crf: &String, preset: &String, audio_bitrate: &String, hevc_enabled: &bool) {
    let app_arc = Arc::new(app.clone());
    set_panic(app_arc);

    app.emit(PROCESSING, "true").unwrap();

    let ffmpeg = get_binary("vid-compress-ffmpeg");
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
        // "-nostdin",
        "-progress", "pipe:1",
    ];

    if is_video(&output_path) {
        app.emit(VID_EXISTS, "").unwrap();

        let (sender, receiver): (tokio::sync::oneshot::Sender<()>, _) = channel();
        let sender = std::sync::Arc::new(std::sync::Mutex::new(Some(sender)));
        let sender_clone = sender.clone();

        let app_clone = app.clone();

        app.once(RESPONSE_IGNORE_EXISTING, move |e| {
            let ignore: bool = e.payload().parse().unwrap();
            
            if let Some(handle) = ANIMATION_HANDLE.lock().unwrap().take() {
                handle.abort();
            }

            match ignore {
                true => {
                    if let Some(s) = sender_clone.lock().unwrap().take() {
                        let _ = s.send(());
                    }
                },
                false => {
                    app_clone.emit(PROCESSING, "false").unwrap();
                    return;
                },
            }
        });

        receiver.await.unwrap();
    }

    println!("{}", format!("{} {}", "Set codec:".bold(), codec).blue());
    let process_message = format!(
        "{}\n{} {}",
        "Processing file with this command:".bold(),
        "ffmpeg".italic(),
        execute_arg.join(" ").italic()
    );
    println!("{process_message}");
    
    let mut child_ffmpeg: Child;
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        child_ffmpeg = Command::new(ffmpeg)
        .args(&execute_arg)
        .creation_flags(0x08000000)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    }
    #[cfg(unix)]
    {
        child_ffmpeg = Command::new(ffmpeg)
        .args(&execute_arg)
        .stdout(Stdio::piped())
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
    
    let out_pipe = child_ffmpeg.stdout.take().unwrap();
    let err_pipe = child_ffmpeg.stderr.take().unwrap();

    let animation_handle: JoinHandle<()> = tokio::spawn(play_progress(app.clone(), out_pipe, err_pipe));
    *ANIMATION_HANDLE.lock().unwrap() = Some(animation_handle);

    let status = child_ffmpeg.wait().await.unwrap(); //wait until compressing is over

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

async fn kill_pid(pid: u32) -> Result<(), String> {
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

        match output.await {
            Ok(_) => Ok(()),
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

async fn play_progress(app: AppHandle, stdout: tokio::process::ChildStdout, stderr: tokio::process::ChildStderr) {
    let mut stdout = FramedRead::new(stdout, LinesCodec::new()).map(|data| data.expect("Fail on stdout"));
    let mut stderr = FramedRead::new(stderr, LinesCodec::new()).map(|data| data.expect("Fail on stderr"));

    let (tx_duration, mut rx_duration) = oneshot::channel::<u64>();
    tokio::spawn(async move {
        let re_duration = Regex::new(r"Duration: ([^,]*)").unwrap();

        while let Some(line) = stderr.next().await {
            if let Some(i) = re_duration.captures(&line) {
                let duration: u64 = {
                    let i: Vec<&str> = i[1].split(':').collect();
                    
                    let hours: u64 = i[0].parse().unwrap();
                    let minutes: u64 = i[1].parse().unwrap();
                    let (seconds, centiseconds): (u64, u64) = {
                        let seconds_part: Vec<&str> = i[2].split('.').collect();
                        let seconds: u64 = seconds_part[0].parse().unwrap();
                        let centiseconds: u64 = seconds_part[1].parse().unwrap();
                        (seconds, centiseconds)
                    };

                    let in_microseconds: u64 =
                        hours * 3_600_000_000 +
                        minutes * 60_000_000 +
                        seconds * 1_000_000 +
                        centiseconds * 10_000;
                    in_microseconds
                };

                tx_duration.send(duration).unwrap();
                return;
            }
        }
    });

    let (tx_time, mut rx_time) = mpsc::channel::<u64>(1);
    tokio::spawn(async move {
        let re_time = Regex::new(r"out_time_ms=([^\n]*)").unwrap();
        while let Some(line) = stdout.next().await {
            if let Some(i) = re_time.captures(&line) {
                let time = i[1].trim().parse().unwrap_or_else(|_| 0);

                let _ = tx_time.try_send(time);
            }
        }
    });

    let mut duration: u64 = 0;
    #[allow(unused_assignments)]
    let mut time: u64 = 0;
    let mut started = false;

    loop {
        if duration == 0 {
            duration = rx_duration.try_recv().ok().unwrap_or_else(|| 0);
            continue;
        }

        time = rx_time.try_recv().ok().unwrap_or_else(|| 0);

        let mut percentage: f32 = (time as f32 / duration as f32)*100.0;

        if percentage > 0.0 { started = true; }
        if percentage == 0.0 && started { percentage = 100.0; }
        

        app.emit(PERCENTAGE, format!("{:.2}%", percentage)).unwrap();

        sleep(Duration::from_millis(1000)).await;
    }
}