mod arguments;
pub mod client_http;

use crate::client_http::{
    connection::initialize_server_config,
    query::{
        query_server, query_server_config, query_server_health_check, query_server_http_requests,
        query_server_nodes, query_server_positions, query_server_raw_packets, query_server_status,
        query_server_texts,
    },
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

    initialize_server_config(
        get_arguments().server_ip.clone(),
        get_arguments().server_port,
    );

    if get_arguments().test {
        // match query_server(HEALTH_CHECK_ENDPOINT.to_string(), None).await {
        //     _ => {
        //         std::process::exit(0);
        //     }
        // };
    }

    query_server_health_check().await;
    query_server_config().await;
    query_server_status().await;
    query_server_texts(None).await;
    query_server_http_requests(Some(20)).await;
    query_server_nodes().await;
    query_server_positions().await;
    query_server_raw_packets().await;

    Ok(())
}
