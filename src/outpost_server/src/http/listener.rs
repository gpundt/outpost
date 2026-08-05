use super::query::health_check_response;
use axum::{
    Router,
    routing::{MethodRouter, get},
};
use config::endpoints::HEALTH_CHECK_ENDPOINT;
use log::{debug, error, info, trace, warn};

pub async fn initialize_http_listener(port_number: u16) {
    let app = Router::new();
    let app = initialize_query_endpoint(
        app,
        HEALTH_CHECK_ENDPOINT.to_string(),
        get(health_check_response),
    );

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port_number))
        .await
        .unwrap();
    info!("Listening on 0.0.0.0:{}", port_number);
    axum::serve(listener, app)
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
