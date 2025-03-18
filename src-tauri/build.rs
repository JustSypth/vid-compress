use std::env;
use std::path::Path;
use std::fs::create_dir_all;

fn main() {
    move_binaries(&["ffmpeg", "vid-compress-watchdog"]);
    tauri_build::build()
}

fn move_binaries(binaries: &[&str]) {
    for bin in binaries {
        println!("cargo:warning={bin}");
        let out_dir = env::var("PROFILE").unwrap();
    
        let bin_src = Path::new("..").join("bin").join(if cfg!(windows) { format!("{bin}.exe") } else { format!("{bin}") });
        let bin_dest = Path::new("target").join(&out_dir).join("bin");
        
        if !bin_src.exists() {
            panic!("FFmpeg binary not found at: {}", bin_src.display());
        }
        if !bin_dest.exists() {
            create_dir_all(&bin_dest).unwrap();
        }
        
        let mut options = fs_extra::file::CopyOptions::new();
        options.overwrite = true;
    
        let bin_dest = bin_dest.join(if cfg!(windows) { format!("{bin}.exe") } else { format!("{bin}") });
        fs_extra::file::copy(bin_src, &bin_dest, &options).unwrap();
    
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            use std::fs::{set_permissions, Permissions};
            set_permissions(bin_dest, Permissions::from_mode(0o755)).unwrap();
        }
    }
}