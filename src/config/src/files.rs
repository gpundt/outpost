use std::fs;

// ──── Linux ─────────────────────────────────────────────────────────────
#[cfg(not(target_os = "windows"))]
pub const ETC_DIR: &str = "/etc/outpost";
#[cfg(not(target_os = "windows"))]
pub const CONFIG_FILE: &str = "/etc/outpost/outpost.conf";

#[cfg(not(target_os = "windows"))]
pub const OPT_DIR: &str = "/opt/outpost";
#[cfg(not(target_os = "windows"))]
pub const LOG_DIR: &str = "/opt/outpost/logs";
#[cfg(not(target_os = "windows"))]
pub const BIN_DIR: &str = "/opt/outpost/bin";
#[cfg(not(target_os = "windows"))]
pub const TLS_DIR: &str = "/opt/outpost/tls";

// ──── Windows ─────────────────────────────────────────────────────────────
#[cfg(target_os = "windows")]
pub const ETC_DIR: &str = "C:\\Users\\Public\\etc";
#[cfg(target_os = "windows")]
pub const CONFIG_FILE: &str = "C:\\Users\\Public\\etc\\outpost.conf";

#[cfg(target_os = "windows")]
pub const OPT_DIR: &str = "C:\\Users\\Public\\opt";
#[cfg(target_os = "windows")]
pub const LOG_DIR: &str = "C:\\Users\\Public\\logs";
#[cfg(target_os = "windows")]
pub const BIN_DIR: &str = "C:\\Users\\Public\\bin";
#[cfg(target_os = "windows")]
pub const TLS_DIR: &str = "C:\\Users\\Public\\tls";

pub fn create_output_directories() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(ETC_DIR)?;
    fs::create_dir_all(LOG_DIR)?;
    fs::create_dir_all(BIN_DIR)?;
    fs::create_dir_all(TLS_DIR)?;
    Ok(())
}
