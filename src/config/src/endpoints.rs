use const_format::formatcp;
use serde::{Deserialize, Serialize};

pub const API_VERSION: &str = "v1";
pub const API_ENDPOINT: &str = formatcp!("/api/{}", API_VERSION);

pub const QUERY_ENDPOINT: &str = formatcp!("{}/query", API_ENDPOINT);
pub const HEALTH_CHECK_ENDPOINT: &str = formatcp!("{}/health_check", QUERY_ENDPOINT);
pub const CONFIG_QUERY_ENDPOINT: &str = formatcp!("{}/config", QUERY_ENDPOINT);
pub const STATUS_QUERY_ENDPOINT: &str = formatcp!("{}/status", QUERY_ENDPOINT);

pub const SUBMISSION_ENDPOINT: &str = formatcp!("{}/submit", API_ENDPOINT);
pub const SUBMIT_CONFIG_ENDPOINT: &str = formatcp!("{}/config", SUBMISSION_ENDPOINT);
pub const SUBMIT_TASK_ENDPOINT: &str = formatcp!("{}/task", SUBMISSION_ENDPOINT);

pub const TRANSMISSION_ENDPOINT: &str = formatcp!("{}/transmit", API_ENDPOINT);
pub const TRANSMIT_TEXT_ENDPOINT: &str = formatcp!("{}/text", TRANSMISSION_ENDPOINT);

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OutpostTask {
    Backup,
    Beacon,
    RefreshNodes,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TaskRequest {
    pub task: OutpostTask,
    pub parameters: Option<serde_json::Value>,
}

#[derive(Serialize, Debug)]
pub struct TaskResponse {
    pub task: OutpostTask,
    pub success: bool,
    pub message: String,
}
