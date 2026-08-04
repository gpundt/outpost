use config::endpoints::HEALTH_CHECK_ENDPOINT;
use config::files::{ETC_DIR, OPT_DIR, create_output_directories};
use config::logging::initialize_logger;

use log::{error, info};

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

    info!("{}", ETC_DIR);
    info!("{}", OPT_DIR);
    error!("{}", HEALTH_CHECK_ENDPOINT);
}
