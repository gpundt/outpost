mod arguments;

pub mod http;
use arguments::{get_arguments, initialize_arguments};

use config::{files::create_output_directories, logging::initialize_logger, time::start_time};

fn main() {
    start_time();
    initialize_arguments();

    if get_arguments().test {
        return;
    }

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
}
