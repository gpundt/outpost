use super::query::{health_check_response, status_query_response};
use crate::http::middleware::log_request_middleware;
use crate::{arguments::get_arguments, http::query::config_query_response};
use axum::middleware::from_fn;
use axum::{
    Router, middleware,
    routing::{MethodRouter, get},
};
use config::endpoints::{CONFIG_QUERY_ENDPOINT, HEALTH_CHECK_ENDPOINT, STATUS_QUERY_ENDPOINT};
use log::{LevelFilter, debug, error, info, trace, warn};
use std::net::SocketAddr;

pub async fn initialize_http_listener() {
    let app = Router::new();
    let app = initialize_query_endpoint(
        app,
        HEALTH_CHECK_ENDPOINT.to_string(),
        get(health_check_response),
    );
    let app = initialize_query_endpoint(
        app,
        STATUS_QUERY_ENDPOINT.to_string(),
        get(status_query_response),
    );

    let app = match log::max_level() {
        LevelFilter::Trace => {
            let app = initialize_query_endpoint(
                app,
                CONFIG_QUERY_ENDPOINT.to_string(),
                get(config_query_response),
            );
            app
        }
        _ => app,
    };

    let app = app.layer(middleware::from_fn(log_request_middleware));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", get_arguments().http_port))
        .await
        .unwrap();
    info!("Listening on 0.0.0.0:{}", get_arguments().http_port);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(graceful_exit())
    .await
    .unwrap();
}

pub fn initialize_query_endpoint(app: Router, path: String, response_func: MethodRouter) -> Router {
    debug!("Initialized {} Endpoint", path);
    app.route(&path, response_func)
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
