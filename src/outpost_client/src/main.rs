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

    if get_arguments().test {
        let _ = test_server_connection(
            get_arguments().clone().server_ip,
            get_arguments().server_port,
            false,
        )
        .await;
        return;
    }

    match initialize_http_connecion() {
        Ok(_) => {}
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    match test_server_connection(
        get_arguments().clone().server_ip,
        get_arguments().server_port,
        true,
    )
    .await
    {
        Ok(_) => {}
        Err(_) => {
            return;
        }
    };
}
