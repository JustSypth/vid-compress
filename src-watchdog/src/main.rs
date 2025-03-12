mod watchdog;

fn main() {
    watchdog::start_watchdog(0, 0);
}