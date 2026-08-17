mod arguments;

pub mod meshtastic;
pub mod server_database;
pub mod server_http;
use crate::meshtastic::connection::global_connection;
use crate::server_database::schema::initialize_database;
use crate::server_http::listener::initialize_http_listener;
use arguments::{get_arguments, initialize_arguments};
use config::{files::create_output_directories, logging::initialize_logger, time::start_time};
use log::{error, info};
use meshtastic::device::{enumerate_serial_devices, list_serial_devices};

/// Outpost server entrypoint
#[tokio::main]
async fn main() {
    start_time();
    initialize_arguments();

    if get_arguments().enumerate {
        list_serial_devices(enumerate_serial_devices());
        return;
    }

    if let Err(e) = setup_server().await {
        eprintln!("Fatal error: {}", e);
        std::process::exit(1);
    }

    initialize_http_listener().await;
}

/// Function to handle execution of server startup
async fn setup_server() -> Result<(), Box<dyn std::error::Error>> {
    {
        create_output_directories("server")?;
        initialize_logger("server", get_arguments().debug)?;
    }

    let serial_port = get_arguments().serial_port.clone();

    initialize_database()
        .await
        .map(|url| info!("Database Initialized: {}", url))?;

    {
        let mut connection = global_connection().lock().await;

        if connection.is_connected() {
            let _ = connection.disconnect();
        }

        info!("Connecting to serial device: {}", serial_port);
        match connection.connect(serial_port.clone(), 115200).await {
            Ok(()) => {}
            Err(e) => {
                error!("{}", e.to_string())
            }
        }
    }

    Ok(())
}
