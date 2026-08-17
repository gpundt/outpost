use crate::files::LOG_DIR;

use chrono::Utc;
use colored::Colorize;
use log::LevelFilter;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

static LOG_FILENAME: OnceLock<Mutex<String>> = OnceLock::new();

/// Function to safely get the filename of the current log file
fn log_filename() -> &'static Mutex<String> {
    LOG_FILENAME.get_or_init(|| Mutex::new(String::new()))
}

/// Function to safely set the filename of the current log file
fn set_log_filename(name: &str) {
    let mut file = log_filename().lock().unwrap();
    *file = name.to_string();
}

/// Function to safely get a copy of the current log file's filename
pub fn get_log_filename() -> String {
    let file = log_filename().lock().unwrap();
    file.clone()
}

/// Function to initialize the terminal and file logging controller
/// Sets current log filter level
pub fn initialize_logger(log_type: &str, debug: bool) -> Result<(), Box<dyn std::error::Error>> {
    let timestamp = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let filepath: PathBuf =
        PathBuf::from(LOG_DIR).join(format!("outpost_{}_{}.log", log_type, timestamp));

    set_log_filename(filepath.to_str().unwrap_or("invalid utf-8"));
    // CHANGED: keep file handle separately for the format closure
    let log_file = match OpenOptions::new().create(true).append(true).open(&filepath) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to open log file at '{}': {}", filepath.display(), e);
            return Err(Box::new(e));
        }
    };

    colored::control::set_override(true);

    // wrap in Mutex so the closure can mutate it safely
    let log_file = std::sync::Mutex::new(log_file);

    env_logger::Builder::new()
        .format(move |_buf, record| {
            let ts = Utc::now()
                .format("%H:%M:%S")
                .to_string()
                .bright_magenta()
                .dimmed();

            let level = match record.level() {
                log::Level::Error => record.level().to_string().bright_red().bold(),
                log::Level::Warn => record.level().to_string().yellow().bold(),
                log::Level::Info => "".to_string().normal(),
                log::Level::Debug => record.level().to_string().cyan(),
                log::Level::Trace => record.level().to_string().dimmed(),
            };

            // write colored output directly to stdout, bypassing buf
            println!("[{}] {:<5}  {}", ts, level, record.args());

            // write clean output to file via Mutex guard
            if let Ok(mut file) = log_file.lock() {
                let clean_line = format!(
                    "[{}] {:<15} {}\n",
                    Utc::now().format("%Y-%m-%d %H:%M:%S"),
                    record.level(),
                    record.args()
                );
                let _ = file.write_all(clean_line.as_bytes());
            }

            Ok(())
        })
        .filter_level(match debug {
            true => LevelFilter::Trace,
            false => LevelFilter::Info,
        })
        .filter_module("hyper", log::LevelFilter::Info)
        .filter_module("axum", log::LevelFilter::Info)
        .filter_module("tower", log::LevelFilter::Info)
        .filter_module("mio", log::LevelFilter::Info)
        .filter_module("meshtastic", log::LevelFilter::Info)
        .filter_module("sqlx", log::LevelFilter::Info)
        .filter_module("reqwest", log::LevelFilter::Info)
        .init();

    Ok(())
}
