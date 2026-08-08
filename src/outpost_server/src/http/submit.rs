use crate::database::backup_database;
use crate::{arguments::get_arguments, meshtastic::errors};
use axum::{Json, Router, http::StatusCode, routing::post};
use config::endpoints::{OutpostTask, TaskRequest, TaskResponse};
use log::{error, info, warn};
use serde::Deserialize;

pub async fn submit_task_response(
    Json(request): Json<TaskRequest>,
) -> (StatusCode, Json<TaskResponse>) {
    info!("Task received: {:?}", request.task);
    match request.task {
        OutpostTask::Backup => handle_backup().await,
        OutpostTask::Beacon => handle_beacon().await,
    }
}

async fn handle_backup() -> (StatusCode, Json<TaskResponse>) {
    match backup_database().await {
        Ok(filepath) => (
            StatusCode::OK,
            Json(TaskResponse {
                task: OutpostTask::Backup,
                success: true,
                message: format!("Database backup created: {}", filepath),
            }),
        ),
        Err(e) => {
            error!("Database backup failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TaskResponse {
                    task: OutpostTask::Backup,
                    success: false,
                    message: format!("Database backup failed: {}", e),
                }),
            )
        }
    }
}

async fn handle_beacon() -> (StatusCode, Json<TaskResponse>) {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(TaskResponse {
            task: OutpostTask::Beacon,
            success: false,
            message: format!("Beacon task not implemented yet"),
        }),
    )
}
