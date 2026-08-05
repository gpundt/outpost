use std::sync::OnceLock;
use std::time::{Duration, Instant};

pub fn start_time() -> &'static Instant {
    static START_TIME: OnceLock<Instant> = OnceLock::new();
    START_TIME.get_or_init(Instant::now)
}

pub fn get_uptime() -> Duration {
    start_time().elapsed()
}

pub fn get_uptime_str() -> String {
    let seconds = get_uptime().as_secs();

    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, secs)
}
