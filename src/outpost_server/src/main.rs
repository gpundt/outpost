mod arguments;

use axum::{Json, Router, routing::get};
use serde::Serialize;

pub mod http;
use crate::http::health_check_response;
use arguments::Args;
use clap::Parser;
use config::endpoints::HEALTH_CHECK_ENDPOINT;
use config::files::{BIN_DIR, ETC_DIR, LOG_DIR, OPT_DIR, create_output_directories};
use config::logging::initialize_logger;
use config::time::start_time;
use log::{debug, error, info, trace, warn};

#[tokio::main]
async fn main() {
    start_time();
    let args: Args = Args::parse();

    match create_output_directories() {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e.to_string());
            return;
        }
    }
    match initialize_logger("server", args.verbose) {
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

    let app = Router::new().route(HEALTH_CHECK_ENDPOINT, get(health_check_response));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(graceful_exit())
        .await
        .unwrap();
}

async fn graceful_exit() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("Received Ctrl+c, shutting down gracefully..."),
        _ = terminate => info!("Received SIGTERM, shutting down gracefully"),
    }
}
