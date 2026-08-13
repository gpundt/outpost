use std::{sync::OnceLock, time::Duration};

use reqwest::Client;

use config::endpoints::HEALTH_CHECK_ENDPOINT;
use log::{error, info};

static SERVER_CONNECTION: OnceLock<Client> = OnceLock::new();

pub fn get_server_connection() -> &'static Client {
    SERVER_CONNECTION
        .get()
        .expect("Server connection is not initialized")
}

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

#[derive(Debug)]
pub enum ConnectionError {
    ClientBuildFailure(String),
    AlreadyInitialized,
}
impl ConnectionError {
    pub fn to_string(&self) -> String {
        match self {
            ConnectionError::ClientBuildFailure(e) => {
                format!("HTTP connection client build failed: {}", e).to_string()
            }
            ConnectionError::AlreadyInitialized => {
                "GLOBAL_CONNECTION already initialized".to_string()
            }
        }
    }
}

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
