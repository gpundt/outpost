use core::fmt;

use log::error;
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
impl fmt::Display for QueryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueryType::HttpRequests => write!(f, "http_requests"),
            QueryType::Nodes => write!(f, "nodes"),
            QueryType::Positions => write!(f, "positions"),
            QueryType::RawPackets => write!(f, "raw_packets"),
            QueryType::Texts => write!(f, "texts"),
        }
    }
}

/// Struct to organize an individual query request
#[derive(Deserialize, Serialize, Debug)]
pub struct QueryRequest {
    pub query_type: QueryType,
    pub parameters: Option<serde_json::Value>,
}
impl fmt::Display for QueryRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "query_type: {}, parameters: {:?}",
            self.query_type, self.parameters
        )
    }
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
impl fmt::Display for HealthCheckResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n\tStatus: {}\n\tUptime: {}\n\tVersion: {}",
            self.status, self.uptime, self.version
        )
    }
}
impl HealthCheckResponse {
    pub fn from_json(json: String) -> Result<Self, serde_json::Error> {
        match serde_json::from_str::<HealthCheckResponse>(&json) {
            Ok(r) => {
                return Ok(r);
            }
            Err(e) => {
                error!(
                    "Failed to deserialize with HealthCheckResponse::from_json: {}",
                    json
                );
                return Err(e);
            }
        }
    }
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
impl fmt::Display for ConfigResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n\tDebug: {}\n\tHTTP Port: {}\n\tSerial Port: {}\n\tLog Level: {}\n\tLog File: {}",
            self.debug,
            self.http_port,
            self.serial_port.clone().unwrap_or("None".to_string()),
            self.log_level,
            self.log_file
        )
    }
}
impl ConfigResponse {
    pub fn from_json(json: String) -> Result<Self, serde_json::Error> {
        match serde_json::from_str::<ConfigResponse>(&json) {
            Ok(r) => {
                return Ok(r);
            }
            Err(e) => {
                error!(
                    "Failed to deserialize with ConfigResponse::from_json: {}",
                    json
                );
                return Err(e);
            }
        }
    }
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
impl fmt::Display for StatusResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n\tStatus: {}\n\tUptime: {}\n\tVersion: {}\n\tSerial Connected: {}\n\tSerial Port: {}\n\tDatabase Reachable: {}\n\tPackets Received: {}\n\tLast Packet Received: {}\n\tConnected Peers: {}",
            self.status,
            self.uptime,
            self.version,
            self.serial_connected,
            self.serial_port.clone().unwrap_or("None".to_string()),
            self.database_reachable,
            self.packets_received,
            self.last_packet_received,
            self.connected_peers
        )
    }
}
impl StatusResponse {
    pub fn from_json(json: String) -> Result<Self, serde_json::Error> {
        match serde_json::from_str::<StatusResponse>(&json) {
            Ok(r) => {
                return Ok(r);
            }
            Err(e) => {
                error!(
                    "Failed to deserialize with StatusResponse::from_json: {}",
                    json
                );
                return Err(e);
            }
        }
    }
}
