use crate::database::{backup_database, insert_task_request_finish, insert_task_request_start};
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
    let row_id = match insert_task_request_start("backup").await {
        Ok(i) => i,
        Err(e) => {
            error!("Error inserting task into database: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TaskResponse {
                    task: OutpostTask::Backup,
                    success: false,
                    message: e.to_string(),
                }),
            );
        }
    };

    let (successful, status_code, response) = match backup_database().await {
        Ok(filepath) => (
            true,
            StatusCode::OK,
            Json(TaskResponse {
                task: OutpostTask::Backup,
                success: true,
                message: format!("Database backup created: {}", filepath),
            }),
        ),
        Err(e) => {
            error!("Database backup failed: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TaskResponse {
                    task: OutpostTask::Backup,
                    success: false,
                    message: format!("Database backup failed: {}", e),
                }),
            );
        }
    };

    match insert_task_request_finish(row_id, successful).await {
        Ok(_) => return (status_code, response),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TaskResponse {
                    task: OutpostTask::Backup,
                    success: false,
                    message: format!(
                        "Database backup successful, tasks table update failed: {}",
                        e.to_string(),
                    ),
                }),
            );
        }
    };
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
