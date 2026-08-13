use crate::arguments::get_arguments;
use crate::http::{
    middleware::log_request_middleware,
    query::{
        generate_config_query_response, generate_health_check_response, generate_query_response,
        generate_status_query_response,
    },
    submit::task_submission_response,
};
use axum::routing::post;
use axum::{
    Router, middleware,
    routing::{MethodRouter, get},
};
use config::endpoints::{
    CONFIG_QUERY_ENDPOINT, HEALTH_CHECK_ENDPOINT, QUERY_ENDPOINT, STATUS_QUERY_ENDPOINT,
    SUBMIT_TASK_ENDPOINT,
};
use log::{LevelFilter, debug, info};
use std::net::SocketAddr;

/// Master function to initialize all individual HTTP endpoints
pub async fn initialize_http_listener() {
    let app = Router::new();
    let app = _initialize_endpoint(
        app,
        HEALTH_CHECK_ENDPOINT.to_string(),
        get(generate_health_check_response),
    );
    let app = _initialize_endpoint(
        app,
        STATUS_QUERY_ENDPOINT.to_string(),
        get(generate_status_query_response),
    );
    let app = _initialize_endpoint(
        app,
        QUERY_ENDPOINT.to_string(),
        get(generate_query_response),
    );

    let app = _initialize_endpoint(
        app,
        SUBMIT_TASK_ENDPOINT.to_string(),
        post(task_submission_response),
    );

    let app = match log::max_level() {
        LevelFilter::Trace => {
            let app = _initialize_endpoint(
                app,
                CONFIG_QUERY_ENDPOINT.to_string(),
                get(generate_config_query_response),
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

/// Helper function to initialize an individual query endp
fn _initialize_endpoint(app: Router, path: String, response_func: MethodRouter) -> Router {
    debug!("Initialized Endpoint: {}", path);
    app.route(&path, response_func)
}

/// Function to gracefully shutdown the HTTP endpoint handler
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
