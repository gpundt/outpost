mod arguments;
pub mod http;

use crate::http::connection::{initialize_http_connecion, test_server_connection};

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

    if get_arguments().test {
        let _ = test_server_connection(
            get_arguments().clone().server_ip,
            get_arguments().server_port,
            false,
        )
        .await?;
        return Ok(());
    }

    initialize_http_connecion()?;

    test_server_connection(
        get_arguments().clone().server_ip,
        get_arguments().server_port,
        true,
    )
    .await?;

    Ok(())
}
