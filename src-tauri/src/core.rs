use std::path::PathBuf;
use std::process::Command;
use tauri::{AppHandle, Emitter};

pub fn begin(app: &AppHandle, path: &PathBuf, cfg: &String, preset: &String) {
    if !is_video(path) {
        app.emit("progress", "Please enter a valid video file.").unwrap();
        return;
    }

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

    app.emit("progress", "Compressing file...").unwrap();
    println!("Processing file with this command:\nffmpeg {}", execute_arg.join(" "));

    app.emit("progress", "Compressing file...").unwrap();
    // println!("Processing file with this command:\n{}", &execute_arg);

    // Execute the command
    let execute = Command::new("ffmpeg")
        .args(&execute_arg)
        .output()
        .expect("Failed to execute ffmpeg.");


    if execute.status.success() {
        let message = "Video compressed successfully";
        app.emit("progress", message).unwrap();
        println!("{message}")
    } else {
        let message = format!("Process failed with status: {}", execute.status);
        eprint!("{}", String::from_utf8_lossy(&execute.stderr));
        app.emit("progress", message).unwrap();
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
