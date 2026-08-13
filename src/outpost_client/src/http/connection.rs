use std::{sync::OnceLock, time::Duration};

use reqwest::Client;

use config::endpoints::HEALTH_CHECK_ENDPOINT;
use log::{error, info};

/// Globally accessible HTTP client connection object
static SERVER_CONNECTION: OnceLock<Client> = OnceLock::new();

/// Function to safely access the globally accessible HTTP client connection object
pub fn get_server_connection() -> &'static Client {
    SERVER_CONNECTION
        .get()
        .expect("Server connection is not initialized")
}

/// Function to initialize HTTP client connection object
pub fn initialize_http_connecion() -> Result<(), String> {
    let client = match Client::builder().timeout(Duration::from_secs(5)).build() {
        Ok(c) => c,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    match SERVER_CONNECTION
        .set(client)
        .map_err(|_| ConnectionError::AlreadyInitialized.to_string())
    {
        Ok(_) => {
            return Ok(());
        }
        Err(e) => return Err(e),
    };
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

/// Function to test the connecton with the Outpost Server
pub async fn test_server_connection(
    server_ip: String,
    server_port: u16,
    silent: bool,
) -> Result<(), reqwest::Error> {
    // Generate URL
    let url = format!(
        "http://{}:{}{}",
        server_ip,
        server_port,
        HEALTH_CHECK_ENDPOINT.to_string()
    );

    let client = reqwest::Client::new();

    // Send HTTP GET
    let response = match client.get(url).timeout(Duration::from_secs(2)).send().await {
        Ok(r) => r,
        Err(e) => {
            error!("Server Connection Failed ({})", e);
            return Err(e);
        }
    };

    let status = response.status();
    match response.error_for_status() {
        Ok(_) => {
            if !silent {
                info!("Server Connection Successful ({})", status)
            }
        }
        Err(_) => {
            error!("Server Connection Failed ({})", status);
        }
    }

    Ok(())
}
