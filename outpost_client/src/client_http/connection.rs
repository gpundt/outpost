use reqwest::Client;
use std::{sync::OnceLock, time::Duration};

use log::error;

/// Struct to store server ip, port, and other info
#[derive(Debug)]
pub struct ServerConfig {
    pub ip_address: String,
    pub http_port: u16,
}
impl ServerConfig {
    pub fn new(ip_addr: String, port: u16) -> Self {
        Self {
            ip_address: ip_addr,
            http_port: port,
        }
    }
    pub fn url(&self, endpoint: String) -> String {
        format!("http://{}:{}{}", self.ip_address, self.http_port, endpoint)
    }
}

/// Globally accessible server config struct
static SERVER_CONFIG: OnceLock<ServerConfig> = OnceLock::new();

/// Function to safely initialize the server config struct
pub fn initialize_server_config(ip: String, port: u16) {
    if let Err(e) = SERVER_CONFIG.set(ServerConfig::new(ip, port)) {
        error!("Failed to get or init SERVER_CONNECTION: {:?}", e);
        std::process::exit(1);
    }
}

/// Function to safely get the global server config struct
pub fn get_server_config() -> &'static ServerConfig {
    match SERVER_CONFIG.get() {
        Some(s) => s,
        None => {
            error!("Failed to get SERVER_CONFIG");
            std::process::exit(1);
        }
    }
}

/// Globally accessible HTTP client connection object
static SERVER_CONNECTION: OnceLock<Client> = OnceLock::new();

/// Function to safely access the globally accessible HTTP client connection object
pub fn get_server_connection() -> &'static Client {
    SERVER_CONNECTION.get_or_init(|| {
        match Client::builder()
            .user_agent(format!(
                "outpost-client_{}",
                env!("CARGO_PKG_VERSION").to_string()
            ))
            .timeout(Duration::from_secs(5))
            .build()
        {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to get or init SERVER_CONNECTION: {}", e);
                std::process::exit(1);
            }
        }
    })
}

/// Custom errors to indentify server connection errors
#[derive(Debug)]
pub enum ConnectionError {
    ClientBuildFailure(String),
    AlreadyInitialized,
    ServerConnectionFailed(String),
}
impl ConnectionError {
    /// Function to return the custom errors as a string
    pub fn to_string(&self) -> String {
        match self {
            ConnectionError::ClientBuildFailure(e) => {
                format!("HTTP connection client build failed: {}", e).to_string()
            }
            ConnectionError::AlreadyInitialized => {
                "GLOBAL_CONNECTION already initialized".to_string()
            }
            ConnectionError::ServerConnectionFailed(e) => {
                format!("Server connection failed: {}", e).to_string()
            }
        }
    }
}
