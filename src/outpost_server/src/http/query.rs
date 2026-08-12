use crate::arguments::get_arguments;
use crate::database::is_db_connected;
use crate::database::select::{
    select_http_requests_by_count, select_meshtastic_nodes, select_meshtastic_positions,
    select_meshtastic_raw_by_count, select_meshtastic_texts_by_count,
};
use crate::http::errors::{QueryError, SerializeError};
use crate::meshtastic::connection::global_connection;
use axum::Json;
use axum::http::StatusCode;
use config::logging::get_log_filename;
use config::query::{QueryRequest, QueryType, extract_count_parameter};
use config::time::get_uptime_str;
use serde::Serialize;

pub async fn query_response(Json(request): Json<QueryRequest>) -> (StatusCode, String) {
    match request.query_type {
        QueryType::HTTP_Requests => http_requests_query_response(request.parameters).await,
        QueryType::Nodes => nodes_query_response(request.parameters).await,
        QueryType::Positions => positions_query_response(request.parameters).await,
        QueryType::Raw_Packets => raw_packets_query_response(request.parameters).await,
        QueryType::Texts => texts_query_response(request.parameters).await,
    }
}

#[derive(Serialize)]
pub struct HealthCheckResponse {
    status: String,
    uptime: String,
    version: String,
}

pub async fn health_check_response() -> Json<HealthCheckResponse> {
    let payload = HealthCheckResponse {
        status: "Healthy".to_string(),
        uptime: get_uptime_str(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };
    Json(payload)
}

#[derive(Serialize)]
pub struct ConfigResponse {
    debug: bool,
    http_port: u16,
    serial_port: Option<String>,
    log_level: String,
    log_file: String,
}

pub async fn config_query_response() -> Json<ConfigResponse> {
    let payload = ConfigResponse {
        debug: get_arguments().debug,
        http_port: get_arguments().http_port,
        serial_port: get_arguments().serial_port.clone(),
        log_level: log::max_level().to_string(),
        log_file: get_log_filename(),
    };
    Json(payload)
}

#[derive(Serialize)]
pub struct StatusResponse {
    status: String,
    uptime: String,
    version: String,
    serial_connected: bool,
    serial_port: Option<String>,
    database_reachable: bool,
    packets_received: u32,
    last_packet_received: String,
    connected_peers: u16,
}

pub async fn status_query_response() -> Json<StatusResponse> {
    let db_connection = is_db_connected().await;

    let payload = StatusResponse {
        status: "Healthy".to_string(),
        uptime: get_uptime_str(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        serial_connected: global_connection().lock().unwrap().is_connected(),
        serial_port: get_arguments().serial_port.clone(),
        database_reachable: db_connection,
        packets_received: 0,
        last_packet_received: "".to_string(),
        connected_peers: 0,
    };
    return Json(payload);
}

pub async fn http_requests_query_response(
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

pub async fn nodes_query_response(parameters: Option<serde_json::Value>) -> (StatusCode, String) {
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

pub async fn positions_query_response(
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

pub async fn raw_packets_query_response(
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

pub async fn texts_query_response(parameters: Option<serde_json::Value>) -> (StatusCode, String) {
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
