use core::fmt;
use log::error;
use sqlx::FromRow;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Struct to organize the contents of a row inside the http_requests db table
#[derive(Debug, Default, FromRow, Serialize, Clone, Deserialize)]
pub struct HTTPRequestEntry {
    pub id: i32,
    pub method: String,
    pub source: String,
    pub endpoint: String,
    pub user_agent: String,
    pub status_code: u16,
    pub timestamp: NaiveDateTime,
}
impl fmt::Display for HTTPRequestEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n  {:<33} {:<21} {:<6} {:<22} -> {:<23} ({})",
            format!("[{}]:", self.timestamp),
            self.source,
            self.method,
            self.user_agent,
            self.endpoint,
            self.status_code
        )
    }
}
impl HTTPRequestEntry {
    pub fn from_json(json: String) -> Result<Vec<Self>, serde_json::Error> {
        match serde_json::from_str::<Vec<HTTPRequestEntry>>(&json) {
            Ok(e) => Ok(e),
            Err(e) => {
                error!(
                    "Failed to deserialize with HTTPRequestEntry::from_json: {}",
                    e
                );
                return Err(e);
            }
        }
    }
}

/// Struct to organize the contents of a row inside the tasks db table
#[derive(Debug, Default, FromRow, Serialize, Clone, Deserialize)]
pub struct TaskRequestEntry {
    pub id: i32,
    pub task_type: String,
    pub requested_at: NaiveDateTime,
    pub finished_at: NaiveDateTime,
    pub successful: bool,
}
impl fmt::Display for TaskRequestEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}): {} - {}",
            self.task_type, self.successful, self.requested_at, self.finished_at
        )
    }
}
impl TaskRequestEntry {
    pub fn from_json(json: String) -> Result<Vec<Self>, serde_json::Error> {
        match serde_json::from_str::<Vec<TaskRequestEntry>>(&json) {
            Ok(e) => Ok(e),
            Err(e) => {
                error!(
                    "Failed to deserialize with TaskRequestEntry::from_json: {}",
                    e
                );
                return Err(e);
            }
        }
    }
}

/// Struct to organize the contents of a row inside the meshtastic_texts db table
#[derive(Debug, Default, FromRow, Serialize, Clone, Deserialize)]
pub struct MeshtasticTextEntry {
    pub id: i32,
    pub timestamp: NaiveDateTime,
    pub src_id: u32,
    pub dst_id: u32,
    pub message: String,
}
impl fmt::Display for MeshtasticTextEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {} -> {}: {}",
            self.timestamp, self.src_id, self.dst_id, self.message
        )
    }
}
impl MeshtasticTextEntry {
    pub fn from_json(json: String) -> Result<Vec<Self>, serde_json::Error> {
        match serde_json::from_str::<Vec<MeshtasticTextEntry>>(&json) {
            Ok(e) => Ok(e),
            Err(e) => {
                error!(
                    "Failed to deserialize with MeshtasticTextEntry::from_json: {}",
                    e
                );
                return Err(e);
            }
        }
    }
}

/// Struct to organize the contents of a row inside the meshtastic_positions db table
#[derive(Debug, Default, FromRow, Serialize, Clone, Deserialize)]
pub struct MeshtasticPositionEntry {
    pub id: i32,
    pub latitude: i32,
    pub longitude: i32,
    pub altitude: i32,
    pub time: u32,
    pub timestamp: u32,
    pub next_update: u32,
}
impl fmt::Display for MeshtasticPositionEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {}, {} ({}): {}",
            self.timestamp, self.latitude, self.longitude, self.altitude, self.time
        )
    }
}
impl MeshtasticPositionEntry {
    pub fn from_json(json: String) -> Result<Vec<Self>, serde_json::Error> {
        match serde_json::from_str::<Vec<MeshtasticPositionEntry>>(&json) {
            Ok(e) => Ok(e),
            Err(e) => {
                error!(
                    "Failed to deserialize with MeshtasticPositionEntry::from_json: {}",
                    e
                );
                return Err(e);
            }
        }
    }
}

/// Struct to organize the contents of a row inside the meshtastic_nodes db table
#[derive(Debug, Default, FromRow, Serialize, Clone, Deserialize)]
pub struct MeshtasticNodeEntry {
    pub id: i32,
    pub node_num: u32,
    pub node_id: String,
    pub node_long_name: String,
    pub node_short_name: String,
    pub hw_model: i32,
    pub role: i32,
    pub last_heard: u32,
    pub uptime: u32,
    pub channel: u32,
    pub hops_away: u32,
}
impl fmt::Display for MeshtasticNodeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}, {}) [{}] uptime: {}",
            self.node_id, self.node_long_name, self.node_short_name, self.hw_model, self.uptime
        )
    }
}
impl MeshtasticNodeEntry {
    pub fn from_json(json: String) -> Result<Vec<Self>, serde_json::Error> {
        match serde_json::from_str::<Vec<MeshtasticNodeEntry>>(&json) {
            Ok(e) => Ok(e),
            Err(e) => {
                error!(
                    "Failed to deserialize with MeshtasticNodeEntry::from_json: {}",
                    e
                );
                return Err(e);
            }
        }
    }
}

/// Struct to organize the contents of a row inside the meshtastic_raw db table
#[derive(Debug, Default, FromRow, Serialize, Clone, Deserialize)]
pub struct MeshtasticRawEntry {
    pub id: i32,
    pub src_node: u32,
    pub dst_node: u32,
    pub channel: u32,
    pub hop_limit: u32,
    pub hop_start: u32,
    pub next_hop: u32,
    pub encrypted: bool,
}
impl fmt::Display for MeshtasticRawEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> {} (channel: {}), encrypted: {}",
            self.src_node, self.dst_node, self.channel, self.encrypted
        )
    }
}
impl MeshtasticRawEntry {
    pub fn from_json(json: String) -> Result<Vec<Self>, serde_json::Error> {
        match serde_json::from_str::<Vec<MeshtasticRawEntry>>(&json) {
            Ok(e) => Ok(e),
            Err(e) => {
                error!(
                    "Failed to deserialize with MeshtasticRawEntry::from_json: {}",
                    e
                );
                return Err(e);
            }
        }
    }
}
