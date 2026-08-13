mod arguments;

pub mod http;
use std::time::Duration;

use arguments::{get_arguments, initialize_arguments};

use config::{
    endpoints::HEALTH_CHECK_ENDPOINT, files::create_output_directories, logging::initialize_logger,
    time::start_time,
};

#[tokio::main]
async fn main() {
    start_time();
    initialize_arguments();

    if get_arguments().test {
        test_server_connection(
            get_arguments().clone().server_ip,
            get_arguments().server_port,
        )
        .await;
        return;
    }

    match create_output_directories("client") {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e.to_string());
            return;
        }
    }
    match initialize_logger("client", get_arguments().debug) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e.to_string());
            return;
        }
    };
}

async fn test_server_connection(server_ip: String, server_port: u16) {
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
            println!("Server Connection Status: Failed ({})", e);
            return;
        }
    };

    match response.status() {
        reqwest::StatusCode::OK => println!(
            "Server Connection Status: Successful ({})",
            response.status()
        ),
        _ => println!("Server Connection Status: Failed ({})", response.status()),
    }
}
