use crate::arguments::get_arguments;
use crate::database::is_db_connected;
use crate::database::select::{
    select_meshtastic_nodes, select_meshtastic_positions, select_meshtastic_raw_by_count,
    select_meshtastic_texts_by_count,
};
use crate::http::errors::{QueryError, SerializeError};
use crate::meshtastic::connection::global_connection;
use axum::Json;
use config::logging::get_log_filename;
use config::time::get_uptime_str;
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

pub async fn nodes_query_response() -> String {
    let nodes = match select_meshtastic_nodes().await {
        Ok(n) => n,
        Err(e) => {
            return QueryError::new(
                "meshtastic_nodes".to_string(),
                "nodes_query_response".to_string(),
                e.to_string(),
            )
            .jsonify();
        }
    };

    match serde_json::to_string(&nodes) {
        Ok(s) => s,
        Err(e) => {
            return SerializeError::new("nodes_query_response".to_string(), e.to_string())
                .jsonify();
        }
    }
}

pub async fn positions_query_response() -> String {
    let positions = match select_meshtastic_positions().await {
        Ok(p) => p,
        Err(e) => {
            return QueryError::new(
                "meshtastic_positions".to_string(),
                "positions_query_response".to_string(),
                e.to_string(),
            )
            .jsonify();
        }
    };

    match serde_json::to_string(&positions) {
        Ok(s) => s,
        Err(e) => {
            return SerializeError::new("positions_query_response".to_string(), e.to_string())
                .jsonify();
        }
    }
}

pub async fn raw_packets_query_response() -> String {
    let raw_packets = match select_meshtastic_positions().await {
        Ok(r) => r,
        Err(e) => {
            return QueryError::new(
                "meshtastic_raw".to_string(),
                "raw_query_response".to_string(),
                e.to_string(),
            )
            .jsonify();
        }
    };

    match serde_json::to_string(&raw_packets) {
        Ok(s) => s,
        Err(e) => {
            return SerializeError::new("raw_query_response".to_string(), e.to_string()).jsonify();
        }
    }
}

pub async fn texts_query_response() -> String {
    let texts = match select_meshtastic_positions().await {
        Ok(t) => t,
        Err(e) => {
            return QueryError::new(
                "meshtastic_texts".to_string(),
                "texts_query_response".to_string(),
                e.to_string(),
            )
            .jsonify();
        }
    };

    match serde_json::to_string(&texts) {
        Ok(s) => s,
        Err(e) => {
            return SerializeError::new("texts_query_response".to_string(), e.to_string())
                .jsonify();
        }
    }
}
