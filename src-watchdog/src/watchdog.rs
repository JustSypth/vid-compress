use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

pub fn start_watchdog(main_pid: u32, child_pid: u32) {
    loop {
        if !is_running(main_pid) {
            kill_pid(child_pid).unwrap();
            break;
        }

        sleep(Duration::from_secs(1));
    }
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

fn is_running(pid: u32) -> bool {
    #[cfg(unix)]
    {
        let output = Command::new("kill")
        .args(["-0", &pid.to_string()])
        .output()
        .unwrap();

        output.status.success()
    }

    #[cfg(windows)] // ew windows
    {
        use std::os::windows::process::CommandExt;
        let output = Command::new("tasklist")
        .args(["/FI", &format!("PID eq {}", pid.to_string())])
        .creation_flags(0x08000000)
        .output()
        .unwrap();

        output.status.success()
    }
}