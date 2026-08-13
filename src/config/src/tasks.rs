use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OutpostTask {
    Backup,
    Beacon,
    Refresh_Nodes,
    Refresh_Raw,
    Refresh_HTTP_Requests,
    Refresh_Positions,
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
