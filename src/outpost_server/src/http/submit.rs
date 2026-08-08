use crate::database::backup_database;
use crate::{arguments::get_arguments, meshtastic::errors};
use axum::{Json, Router, http::StatusCode, routing::post};
use config::endpoints::OutpostTask;
use log::{error, info, warn};
use serde::Deserialize;

pub async fn submit_task_response(Json(task): Json<OutpostTask>) -> (StatusCode, String) {
    match task {
        OutpostTask::Backup => match backup_database().await {
            Ok(filepath) => {
                info!("Database backup created: {}", filepath);
                return (
                    StatusCode::OK,
                    format!("Database backup created: {}", filepath),
                );
            }
            Err(e) => {
                error!("Database Backup Failed: {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Database backup failed: {}", e),
                );
            }
        },
        OutpostTask::Beacon => {
            return (
                StatusCode::NOT_IMPLEMENTED,
                format!("Task not implemented yet: {:?}", task),
            );
        }
    }
}
