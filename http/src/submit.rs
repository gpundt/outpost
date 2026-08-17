use config::tasks::OutpostTask;
use serde::{Deserialize, Serialize};

use log::error;

/// Struct to organize an individual task request
#[derive(Deserialize, Serialize, Debug)]
pub struct TaskRequest {
    pub task: OutpostTask,
    pub parameters: Option<serde_json::Value>,
}
impl std::fmt::Display for TaskRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n\tTask: {}\n\tParameters: {:?}",
            self.task, self.parameters
        )
    }
}
impl TaskRequest {
    pub fn new(task_type: OutpostTask, parameters: Option<serde_json::Value>) -> Self {
        Self {
            task: task_type,
            parameters,
        }
    }
}

/// Struct to organize an individual task response
#[derive(Serialize, Debug, Deserialize)]
pub struct TaskResponse {
    pub task: OutpostTask,
    pub success: bool,
    pub message: String,
}
impl std::fmt::Display for TaskResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n\tTask: {}\n\tSuccess: {}\n\tMessage: {}",
            self.task, self.success, self.message
        )
    }
}
impl TaskResponse {
    pub fn from_json(json: String) -> Result<Self, serde_json::Error> {
        match serde_json::from_str::<TaskResponse>(&json) {
            Ok(r) => return Ok(r),
            Err(e) => {
                error!("Failed to deserialize with TaskResponse::from_json: {}", e);
                return Err(e);
            }
        }
    }
}
