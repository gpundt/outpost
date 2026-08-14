mod arguments;
pub mod client_http;

use crate::client_http::{
    connection::set_server_config,
    query::{query_health_check, query_server_config, query_server_status},
};

use arguments::{get_arguments, initialize_arguments};
use config::{files::create_output_directories, logging::initialize_logger, time::start_time};
use log::error;

/// Outpost server entrypoint
#[tokio::main]
async fn main() {
    start_time();
    initialize_arguments();

    if let Err(e) = run().await {
        error!("{}", e);
        std::process::exit(1);
    }
}

/// Function to handle execution of client startup
async fn run() -> Result<(), Box<dyn std::error::Error>> {
    create_output_directories("client")?;
    initialize_logger("client", get_arguments().debug)?;

    set_server_config(
        get_arguments().server_ip.clone(),
        get_arguments().server_port,
    );

    if get_arguments().test {
        match query_health_check(false).await {
            _ => {
                std::process::exit(0);
            }
        };
    }

    query_health_check(true).await?;
    query_server_config().await?;
    query_server_status().await?;

    Ok(())
}
