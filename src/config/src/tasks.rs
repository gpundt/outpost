use serde::{Deserialize, Serialize};

/// Enum to restrict the available task options
#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OutpostTask {
    Backup,
    Beacon,
    RefreshNodes,
    RefreshRaw,
    RefreshHttpRequests,
    RefreshPositions,
}

/// Struct to organize an individual task request
#[derive(Deserialize, Serialize, Debug)]
pub struct TaskRequest {
    pub task: OutpostTask,
    pub parameters: Option<serde_json::Value>,
}

/// Struct to organize an individual task response
#[derive(Serialize, Debug)]
pub struct TaskResponse {
    pub task: OutpostTask,
    pub success: bool,
    pub message: String,
}
