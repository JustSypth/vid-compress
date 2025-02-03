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

        // Generate the ffmpeg command
        let execute_arg = format!(
            "ffmpeg -i '{}' -vcodec libx264 -crf {} -preset {} -acodec aac -b:a 128k -y '{}'",
            path.display(),
            cfg,
            preset,
            output_path.display()
        );

        app.emit("progress", "Compressing file...").unwrap();
        println!("Processing file with this command:\n{}", &execute_arg);

        // Execute the command
        let execute = Command::new("sh")
            .arg("-c")
            .arg(&execute_arg)
            .output()
            .expect("Failed to execute ffmpeg.");


        if execute.status.success() {
            let message = "Video compressed successfully";
            app.emit("progress", message).unwrap();
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
