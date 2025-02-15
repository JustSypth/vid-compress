use std::env;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::fs::{create_dir_all, set_permissions, Permissions};

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    
    let bin_dir = Path::new("..").join("bin");
    let platform_dir = match target_os.as_str() {
        "windows" => "ffmpeg-windows",
        "linux" => "ffmpeg-linux",
        _ => panic!("Unsupported platform"),
    };
    
    let ffmpeg_src = bin_dir.join(platform_dir).join(if cfg!(windows) { "bin/ffmpeg.exe" } else { "ffmpeg" });
    let out_dir = env::var("PROFILE").unwrap();
    let ffmpeg_dest = Path::new("target").join(&out_dir).join("bin");
    
    if !ffmpeg_src.exists() {
        panic!("FFmpeg binary not found at: {}", ffmpeg_src.display());
    }
    if !ffmpeg_dest.exists() {
        create_dir_all(&ffmpeg_dest).unwrap();
    }
    
    println!("ffmpeg_src: {}, ffmpeg_dest: {}", ffmpeg_src.display(), ffmpeg_dest.display());
    
    let mut options = fs_extra::file::CopyOptions::new();
    options.overwrite = true;

    fs_extra::file::copy(ffmpeg_src, ffmpeg_dest.join(if cfg!(windows) { "ffmpeg.exe" } else { "ffmpeg" }), &options).unwrap();

    #[cfg(unix)]
    {
        set_permissions(ffmpeg_dest, Permissions::from_mode(0o755)).unwrap();
    }

    tauri_build::build()
}