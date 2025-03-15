mod watchdog;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let main_pid: u32 = args[1].parse().unwrap();
    let child_pid: u32 = args[2].parse().unwrap();

    watchdog::start_watchdog(main_pid, child_pid);
}