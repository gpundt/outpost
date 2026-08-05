use std::sync::OnceLock;
use std::time::{Duration, Instant};

pub fn start_time() -> &'static Instant {
    static START_TIME: OnceLock<Instant> = OnceLock::new();
    START_TIME.get_or_init(Instant::now)
}

pub fn get_uptime() -> Duration {
    start_time().elapsed()
}
