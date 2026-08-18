mod arguments;
pub mod client_http;
pub mod ui;

use std::{thread, time::Duration};

use crate::{
    client_http::{
        connection::initialize_server_config,
        query::{
            query_server_config, query_server_health_check, query_server_http_requests,
            query_server_nodes, query_server_positions, query_server_raw_packets,
            query_server_status, query_server_texts,
        },
    },
    ui::{app::App, header::update_server_status},
};

use arguments::{get_arguments, initialize_arguments};
use config::{files::create_output_directories, logging::initialize_logger, time::start_time};
use log::error;
use tokio::runtime::Runtime;

/// Outpost server entrypoint
#[tokio::main]
async fn main() {
    start_time();
    initialize_arguments();

    if let Err(e) = setup_client().await {
        error!("{}", e);
        std::process::exit(1);
    }

    // Background task: polls the server every second and publishes the result.
    thread::spawn(|| {
        let rt = Runtime::new().unwrap();
        loop {
            rt.block_on(update_server_status());
            thread::sleep(Duration::from_secs(5))
        }
    });

    let _ = App::new().run();
}

/// Function to handle execution of client startup
async fn setup_client() -> Result<(), Box<dyn std::error::Error>> {
    create_output_directories("client")?;

    initialize_logger(
        "client",
        get_arguments().debug,
        get_arguments().connection_test,
    )?;

    initialize_server_config(
        get_arguments().server_ip.clone(),
        get_arguments().server_port,
    );

    if get_arguments().connection_test {
        query_server_health_check().await;
        query_server_config().await;
        query_server_status().await;
        query_server_texts(None).await;
        query_server_http_requests(Some(20)).await;
        query_server_nodes().await;
        query_server_positions().await;
        query_server_raw_packets().await;
        std::process::exit(0)
    }

    Ok(())
}
