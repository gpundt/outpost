use std::fs;

// ──── Linux ─────────────────────────────────────────────────────────────
#[cfg(not(target_os = "windows"))]
mod platform {
    pub const ETC_DIR: &str = "/etc/outpost/";
    pub const CONFIG_FILE: &str = "/etc/outpost/outpost.conf";

    pub const TLS_DIR: &str = "/opt/outpost/tls/";
    pub const TLS_CA_CERT: &str = "/opt/outpost/tls/ca.crt";
    pub const TLS_SERVER_CERT: &str = "/opt/outpost/tls/server.crt";
    pub const TLS_SERVER_KEY: &str = "/opt/outpost/tls/server.key";
    pub const TLS_CLIENT_CERT: &str = "/opt/outpost/tls/client.crt";
    pub const TLS_CLIENT_KEY: &str = "/opt/outpost/tls/client.key";

    pub const OPT_DIR: &str = "/opt/outpost/";
    pub const LOG_DIR: &str = "/opt/outpost/logs/";
    pub const BIN_DIR: &str = "/opt/outpost/bin/";
    pub const DATABASE_DIR: &str = "/opt/outpost/db/";
}

// ──── Windows ─────────────────────────────────────────────────────────────
#[cfg(target_os = "windows")]
mod platform {
    pub const ETC_DIR: &str = "C:\\Users\\Public\\outpost\\etc\\";
    pub const CONFIG_FILE: &str = "C:\\Users\\Public\\outpost\\etc\\outpost.conf";

    pub const TLS_DIR: &str = "C:\\Users\\Public\\outpost\\opt\\tls\\";
    pub const TLS_CA_CERT: &str = "C:\\Users\\Public\\outpost\\opt\\tls\\ca.crt";
    pub const TLS_SERVER_CERT: &str = "C:\\Users\\Public\\outpost\\opt\\tls\\server.crt";
    pub const TLS_SERVER_KEY: &str = "C:\\Users\\Public\\outpost\\opt\\tls\\server.key";
    pub const TLS_CLIENT_CERT: &str = "C:\\Users\\Public\\outpost\\opt\\tls\\client.crt";
    pub const TLS_CLIENT_KEY: &str = "C:\\Users\\Public\\outpost\\opt\\tls\\client.key";

    pub const OPT_DIR: &str = "C:\\Users\\Public\\outpost\\opt\\";
    pub const LOG_DIR: &str = "C:\\Users\\Public\\outpost\\opt\\logs\\";
    pub const BIN_DIR: &str = "C:\\Users\\Public\\outpost\\opt\\bin\\";
    pub const DATABASE_DIR: &str = "C:\\Users\\Public\\outpost\\opt\\db\\";
}

pub use platform::*;

/// Function to ensure the important directories always exist
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
