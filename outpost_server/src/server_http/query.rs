use crate::arguments::get_arguments;
use crate::meshtastic::connection::global_connection;
use crate::server_database::{
    schema::is_db_connected,
    select::{
        select_http_requests_by_count, select_meshtastic_nodes, select_meshtastic_positions,
        select_meshtastic_raw_by_count, select_meshtastic_texts_by_count,
    },
};
use crate::server_http::errors::{QueryError, SerializeError};

use axum::Json;
use axum::http::StatusCode;
use config::{logging::get_log_filename, time::get_uptime_str};
use http::query::{
    ConfigResponse, HealthCheckResponse, QueryRequest, QueryType, StatusResponse,
    extract_count_parameter,
};

/// Master function to handle and direct all http query endpoint requests
/// Returns an HTTP status code and a JSON response from an individual generation function
pub async fn generate_query_response(Json(request): Json<QueryRequest>) -> (StatusCode, String) {
    match request.query_type {
        QueryType::HealthCheck => generate_health_check_response().await,
        QueryType::ServerConfig => generate_config_query_response().await,
        QueryType::ServerStatus => generate_status_query_response().await,
        QueryType::HttpRequests => generate_http_requests_query_response(request.parameters).await,
        QueryType::Nodes => generate_nodes_query_response(request.parameters).await,
        QueryType::Positions => generate_positions_query_response(request.parameters).await,
        QueryType::RawPackets => generate_raw_packets_query_response(request.parameters).await,
        QueryType::Texts => generate_texts_query_response(request.parameters).await,
        QueryType::CpuMetrics => (StatusCode::NOT_IMPLEMENTED, "".to_string()),
        QueryType::RamMetrics => (StatusCode::NOT_IMPLEMENTED, "".to_string()),
        QueryType::StorageMetrics => (StatusCode::NOT_IMPLEMENTED, "".to_string()),
    }
}

/// Function to generate and return the health_check endpoint JSON response
/// Returns an HTTP status code and a JSON response
pub async fn generate_health_check_response() -> (StatusCode, String) {
    let payload = HealthCheckResponse {
        status: "Healthy".to_string(),
        uptime: get_uptime_str(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    match serde_json::to_string(&payload) {
        Ok(s) => (StatusCode::OK, s),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                SerializeError::new("http_requests_query_response".to_string(), e.to_string())
                    .jsonify(),
            );
        }
    }
}

/// Function to generate and return the config endpoint JSON response
/// Returns an HTTP status code and a JSON response
pub async fn generate_config_query_response() -> (StatusCode, String) {
    let payload = ConfigResponse {
        debug: get_arguments().debug,
        http_port: get_arguments().http_port,
        serial_port: get_arguments().serial_port.clone(),
        log_level: log::max_level().to_string(),
        log_file: get_log_filename(),
    };

    match serde_json::to_string(&payload) {
        Ok(s) => (StatusCode::OK, s),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                SerializeError::new("http_requests_query_response".to_string(), e.to_string())
                    .jsonify(),
            );
        }
    }
}

/// Function to generate and return the status endpoint JSON response
/// Returns an HTTP status code and a JSON response
pub async fn generate_status_query_response() -> (StatusCode, String) {
    let db_connection = is_db_connected().await;

    let payload = StatusResponse {
        status: "Healthy".to_string(),
        uptime: get_uptime_str(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        serial_connected: global_connection().lock().await.is_connected(),
        serial_port: get_arguments().serial_port.clone(),
        database_reachable: db_connection,
        packets_received: 0,
        last_packet_received: "".to_string(),
        connected_peers: 0,
    };

    match serde_json::to_string(&payload) {
        Ok(s) => (StatusCode::OK, s),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                SerializeError::new("http_requests_query_response".to_string(), e.to_string())
                    .jsonify(),
            );
        }
    }
}

/// Function to generate and return the http_request query endpoint JSON response
/// Returns an HTTP status code and a JSON response
pub async fn generate_http_requests_query_response(
    parameters: Option<serde_json::Value>,
) -> (StatusCode, String) {
    let requests = match select_http_requests_by_count(100).await {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                QueryError::new(
                    "http_requests".to_string(),
                    "http_requests_query_response".to_string(),
                    e.to_string(),
                )
                .jsonify(),
            );
        }
    };

    match serde_json::to_string(&requests) {
        Ok(s) => (StatusCode::OK, s),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                SerializeError::new("http_requests_query_response".to_string(), e.to_string())
                    .jsonify(),
            );
        }
    }
}

/// Function to generate and return the HTTP response for the meshtastic_nodes query
/// Returns an HTTP status code and a JSON response
pub async fn generate_nodes_query_response(
    parameters: Option<serde_json::Value>,
) -> (StatusCode, String) {
    let nodes = match select_meshtastic_nodes().await {
        Ok(n) => n,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                QueryError::new(
                    "meshtastic_nodes".to_string(),
                    "nodes_query_response".to_string(),
                    e.to_string(),
                )
                .jsonify(),
            );
        }
    };

    match serde_json::to_string(&nodes) {
        Ok(s) => (StatusCode::OK, s),
        Err(e) => {
            return (
                StatusCode::OK,
                SerializeError::new("nodes_query_response".to_string(), e.to_string()).jsonify(),
            );
        }
    }
}

/// Function to generate and return the HTTP response for the meshtastic_positions query
/// Returns an HTTP status code and a JSON response
pub async fn generate_positions_query_response(
    parameters: Option<serde_json::Value>,
) -> (StatusCode, String) {
    let positions = match select_meshtastic_positions().await {
        Ok(p) => p,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                QueryError::new(
                    "meshtastic_positions".to_string(),
                    "positions_query_response".to_string(),
                    e.to_string(),
                )
                .jsonify(),
            );
        }
    };

    match serde_json::to_string(&positions) {
        Ok(s) => (StatusCode::OK, s),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                SerializeError::new("positions_query_response".to_string(), e.to_string())
                    .jsonify(),
            );
        }
    }
}

/// Function to generate and return the HTTP response for the raw_packets query
/// Returns an HTTP status code and a JSON response
pub async fn generate_raw_packets_query_response(
    parameters: Option<serde_json::Value>,
) -> (StatusCode, String) {
    let row_count = extract_count_parameter(&parameters, 100);

    let raw_packets = match select_meshtastic_raw_by_count(row_count).await {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                QueryError::new(
                    "meshtastic_raw".to_string(),
                    "raw_query_response".to_string(),
                    e.to_string(),
                )
                .jsonify(),
            );
        }
    };

    match serde_json::to_string(&raw_packets) {
        Ok(s) => (StatusCode::OK, s),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                SerializeError::new("raw_query_response".to_string(), e.to_string()).jsonify(),
            );
        }
    }
}

/// Function to generate and return the HTTP response for the texts query option
/// Returns an HTTP status code and a JSON response
pub async fn generate_texts_query_response(
    parameters: Option<serde_json::Value>,
) -> (StatusCode, String) {
    let row_count = extract_count_parameter(&parameters, 100);
    let texts = match select_meshtastic_texts_by_count(row_count).await {
        Ok(t) => t,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                QueryError::new(
                    "meshtastic_texts".to_string(),
                    "texts_query_response".to_string(),
                    e.to_string(),
                )
                .jsonify(),
            );
        }
    };

    match serde_json::to_string(&texts) {
        Ok(s) => (StatusCode::OK, s),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                SerializeError::new("texts_query_response".to_string(), e.to_string()).jsonify(),
            );
        }
    }
}
