use crate::arguments::get_arguments;
use crate::meshtastic::connection::global_connection;
use axum::Json;
use config::endpoints::{CONFIG_QUERY_ENDPOINT, HEALTH_CHECK_ENDPOINT, STATUS_QUERY_ENDPOINT};
use config::logging::get_log_filename;
use config::time::get_uptime_str;
use log::{debug, error, info, trace, warn};
use serde::Serialize;

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
    let payload = StatusResponse {
        status: "Healthy".to_string(),
        uptime: get_uptime_str(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        serial_connected: global_connection().lock().unwrap().is_connected(),
        serial_port: get_arguments().serial_port.clone(),
        database_reachable: false,
        packets_received: 0,
        last_packet_received: "".to_string(),
        connected_peers: 0,
    };
    return Json(payload);
}
