use serde::{Deserialize, Serialize};

/// Enum to restrict the available query options
#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryType {
    Texts,
    Nodes,
    RawPackets,
    Positions,
    HttpRequests,
}

/// Struct to organize an individual query request
#[derive(Deserialize, Serialize, Debug)]
pub struct QueryRequest {
    pub query_type: QueryType,
    pub parameters: Option<serde_json::Value>,
}

/// Function to isolate the 'count' key and value from a QueryResonse's parameters field
pub fn extract_count_parameter(parameters: &Option<serde_json::Value>, default: u32) -> u32 {
    parameters
        .as_ref()
        .and_then(|p| p.get("count"))
        .and_then(|v| v.as_u64())
        .and_then(|v| u32::try_from(v).ok())
        .unwrap_or(default)
}

/// Struct to organize the JSON response for the health_check query endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheckResponse {
    pub status: String,
    pub uptime: String,
    pub version: String,
}

/// Struct to organize the JSON response for the config query endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigResponse {
    pub debug: bool,
    pub http_port: u16,
    pub serial_port: Option<String>,
    pub log_level: String,
    pub log_file: String,
}

/// Struct to organize the JSON response for the status query endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct StatusResponse {
    pub status: String,
    pub uptime: String,
    pub version: String,
    pub serial_connected: bool,
    pub serial_port: Option<String>,
    pub database_reachable: bool,
    pub packets_received: u32,
    pub last_packet_received: String,
    pub connected_peers: u16,
}
