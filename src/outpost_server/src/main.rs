mod arguments;

pub mod http;
use crate::http::initialize_http_listener;
use arguments::{get_arguments, init_arguments};
use config::endpoints::HEALTH_CHECK_ENDPOINT;
use config::files::{BIN_DIR, ETC_DIR, LOG_DIR, OPT_DIR, create_output_directories};
use config::logging::initialize_logger;
use config::time::start_time;
use log::{debug, error, info, trace, warn};

#[tokio::main]
async fn main() {
    start_time();
    init_arguments();

    match create_output_directories() {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e.to_string());
            return;
        }
    }
    match initialize_logger("server", get_arguments().verbose) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e.to_string());
            return;
        }
    }

    trace!("{}", BIN_DIR);
    debug!("{}", OPT_DIR);
    info!("{}", ETC_DIR);
    warn!("{}", LOG_DIR);
    error!("{}", HEALTH_CHECK_ENDPOINT);

    initialize_http_listener().await;
}
