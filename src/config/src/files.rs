use std::fs;

// ──── Linux ─────────────────────────────────────────────────────────────
#[cfg(not(target_os = "windows"))]
mod platform {
    pub const ETC_DIR: &str = "/etc/outpost/";
    pub const CONFIG_FILE: &str = "/etc/outpost/outpost.conf";

    pub const OPT_DIR: &str = "/opt/outpost/";
    pub const LOG_DIR: &str = "/opt/outpost/logs/";
    pub const BIN_DIR: &str = "/opt/outpost/bin/";
    pub const TLS_DIR: &str = "/opt/outpost/tls/";
    pub const DATABASE_DIR: &str = "/opt/outpost/db/";
}

// ──── Windows ─────────────────────────────────────────────────────────────
#[cfg(target_os = "windows")]
mod platform {
    pub const ETC_DIR: &str = "C:\\Users\\Public\\outpost\\etc\\";
    pub const CONFIG_FILE: &str = "C:\\Users\\Public\\outpost\\etc\\outpost.conf";

    pub const OPT_DIR: &str = "C:\\Users\\Public\\outpost\\opt\\";
    pub const LOG_DIR: &str = "C:\\Users\\Public\\outpost\\opt\\logs\\";
    pub const BIN_DIR: &str = "C:\\Users\\Public\\outpost\\opt\\bin\\";
    pub const TLS_DIR: &str = "C:\\Users\\Public\\outpost\\opt\\tls\\";
    pub const DATABASE_DIR: &str = "C:\\Users\\Public\\outpost\\opt\\db\\";
}

pub use platform::*;
pub fn create_output_directories(mode: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(ETC_DIR)?;
    fs::create_dir_all(LOG_DIR)?;
    fs::create_dir_all(BIN_DIR)?;
    fs::create_dir_all(TLS_DIR)?;

    if mode == "server" {
        fs::create_dir_all(DATABASE_DIR)?;
    }
    Ok(())
}
