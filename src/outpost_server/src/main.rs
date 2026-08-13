mod arguments;

pub mod database;
pub mod http;
pub mod meshtastic;
use crate::database::schema::initialize_database;
use crate::http::listener::initialize_http_listener;
use crate::meshtastic::connection::global_connection;
use arguments::{get_arguments, initialize_arguments};
use config::{files::create_output_directories, logging::initialize_logger, time::start_time};
use log::{error, info};
use meshtastic::device::{enumerate_serial_devices, list_serial_devices};

#[tokio::main]
async fn main() {
    start_time();
    initialize_arguments();

    if get_arguments().enumerate {
        list_serial_devices(enumerate_serial_devices());
        return;
    }

    match create_output_directories("server") {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e.to_string());
            return;
        }
    }
    match initialize_logger("server", get_arguments().debug) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e.to_string());
            return;
        }
    };

    if let None = get_arguments().serial_port {
        error!("You must specify a serial port: --serial-port PORT");
        return;
    }

    match initialize_database().await {
        Ok(database_url) => info!("Database initialized: {}", database_url),
        Err(e) => {
            error!("Database Initialization Failure: {}", e.to_string());
            return;
        }
    }

    {
        let mut connection = global_connection().lock().unwrap();

        if connection.is_connected() {
            let _ = connection.disconnect();
        }

        info!(
            "Connecting to serial device: {}",
            get_arguments().serial_port.clone().unwrap()
        );
        match connection
            .connect(get_arguments().serial_port.clone(), 115200)
            .await
        {
            Ok(()) => {}
            Err(e) => {
                error!("{}", e.to_string())
            }
        }
    }

    initialize_http_listener().await;
}
