use config::endpoints::HEALTH_CHECK_ENDPOINT;
use config::files::{BIN_DIR, ETC_DIR, LOG_DIR, OPT_DIR, create_output_directories};
use config::logging::initialize_logger;

use log::{debug, error, info, trace, warn};

fn main() {
    match create_output_directories() {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e.to_string());
            return;
        }
    }
    match initialize_logger("server") {
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
}
