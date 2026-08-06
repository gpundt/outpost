mod arguments;

pub mod http;
pub mod meshtastic;
use crate::http::initialize_http_listener;
use crate::meshtastic::connection::{global_connection, global_runtime};
use arguments::{get_arguments, init_arguments};
use config::files::create_output_directories;
use config::logging::initialize_logger;
use config::time::start_time;
use log::{debug, error, info, trace, warn};
use meshtastic::device::{enumerate_serial_devices, list_serial_devices};

#[tokio::main]
async fn main() {
    start_time();
    init_arguments();

    if get_arguments().enumerate {
        list_serial_devices(enumerate_serial_devices());
        return;
    }

    match create_output_directories() {
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
