use crate::database::{
    backup_database, delete_from_table, insert_task_request_finish, insert_task_request_start,
};
use axum::{Json, http::StatusCode};
use config::tasks::{OutpostTask, TaskRequest, TaskResponse};
use log::info;

pub async fn submit_task_response(
    Json(request): Json<TaskRequest>,
) -> (StatusCode, Json<TaskResponse>) {
    info!("Task Received: {:?}", request.task);
    match request.task {
        OutpostTask::Backup => handle_backup().await,
        OutpostTask::Beacon => handle_beacon().await,
        OutpostTask::RefreshNodes => handle_refresh_nodes().await,
        OutpostTask::RefreshHTTPRequests => handle_refresh_http_requests().await,
        OutpostTask::RefreshPositions => handle_refresh_positions().await,
        OutpostTask::RefreshRaw => handle_refresh_raw().await,
    }
}

async fn handle_backup() -> (StatusCode, Json<TaskResponse>) {
    let row_id = match insert_task_request_start("backup").await {
        Ok(i) => i,
        Err(e) => {
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

async fn handle_refresh_nodes() -> (StatusCode, Json<TaskResponse>) {
    let row_id = match insert_task_request_start("refresh_nodes").await {
        Ok(i) => i,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TaskResponse {
                    task: OutpostTask::RefreshNodes,
                    success: false,
                    message: e.to_string(),
                }),
            );
        }
    };

    let (successful, status_code, response) = match delete_from_table("meshtastic_nodes").await {
        Ok(_) => (
            true,
            StatusCode::OK,
            TaskResponse {
                task: OutpostTask::RefreshNodes,
                success: true,
                message: format!("Meshtastic nodes list successfully refreshed"),
            },
        ),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TaskResponse {
                    task: OutpostTask::RefreshNodes,
                    success: false,
                    message: format!("Meshtastic nodes list refresh failed: {}", e),
                }),
            );
        }
    };

    match insert_task_request_finish(row_id, successful).await {
        Ok(_) => return (status_code, Json(response)),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TaskResponse {
                    task: OutpostTask::Backup,
                    success: false,
                    message: format!(
                        "Meshtastic Nodes list refresh successful, tasks table update failed: {}",
                        e.to_string(),
                    ),
                }),
            );
        }
    };
}

async fn handle_refresh_http_requests() -> (StatusCode, Json<TaskResponse>) {
    let row_id = match insert_task_request_start("refresh_http_requests").await {
        Ok(i) => i,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TaskResponse {
                    task: OutpostTask::RefreshHTTPRequests,
                    success: false,
                    message: e.to_string(),
                }),
            );
        }
    };

    let (successful, status_code, response) = match delete_from_table("http_requests").await {
        Ok(_) => (
            true,
            StatusCode::OK,
            TaskResponse {
                task: OutpostTask::RefreshHTTPRequests,
                success: true,
                message: format!("HTTP Requests list successfully refreshed"),
            },
        ),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TaskResponse {
                    task: OutpostTask::RefreshHTTPRequests,
                    success: false,
                    message: format!("HTTP Requests list refresh failed: {}", e),
                }),
            );
        }
    };

    match insert_task_request_finish(row_id, successful).await {
        Ok(_) => return (status_code, Json(response)),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TaskResponse {
                    task: OutpostTask::RefreshHTTPRequests,
                    success: false,
                    message: format!(
                        "HTTP Requests list refresh successful, tasks table update failed: {}",
                        e.to_string(),
                    ),
                }),
            );
        }
    };
}

async fn handle_refresh_raw() -> (StatusCode, Json<TaskResponse>) {
    let row_id = match insert_task_request_start("refresh_raw").await {
        Ok(i) => i,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TaskResponse {
                    task: OutpostTask::RefreshRaw,
                    success: false,
                    message: e.to_string(),
                }),
            );
        }
    };

    let (successful, status_code, response) = match delete_from_table("meshtastic_raw").await {
        Ok(_) => (
            true,
            StatusCode::OK,
            TaskResponse {
                task: OutpostTask::RefreshRaw,
                success: true,
                message: format!("Meshtastic raw packets list successfully refreshed"),
            },
        ),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TaskResponse {
                    task: OutpostTask::RefreshRaw,
                    success: false,
                    message: format!("Meshtastic raw packets list refresh failed: {}", e),
                }),
            );
        }
    };

    match insert_task_request_finish(row_id, successful).await {
        Ok(_) => return (status_code, Json(response)),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TaskResponse {
                    task: OutpostTask::RefreshRaw,
                    success: false,
                    message: format!(
                        "Meshtastic raw packets list refresh successful, tasks table update failed: {}",
                        e.to_string(),
                    ),
                }),
            );
        }
    };
}

async fn handle_refresh_positions() -> (StatusCode, Json<TaskResponse>) {
    let row_id = match insert_task_request_start("refresh_positions").await {
        Ok(i) => i,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TaskResponse {
                    task: OutpostTask::RefreshPositions,
                    success: false,
                    message: e.to_string(),
                }),
            );
        }
    };

    let (successful, status_code, response) = match delete_from_table("meshtastic_positions").await
    {
        Ok(_) => (
            true,
            StatusCode::OK,
            TaskResponse {
                task: OutpostTask::RefreshPositions,
                success: true,
                message: format!("Meshtastic positions list successfully refreshed"),
            },
        ),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TaskResponse {
                    task: OutpostTask::RefreshPositions,
                    success: false,
                    message: format!("Meshtastic positions list refresh failed: {}", e),
                }),
            );
        }
    };

    match insert_task_request_finish(row_id, successful).await {
        Ok(_) => return (status_code, Json(response)),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TaskResponse {
                    task: OutpostTask::RefreshPositions,
                    success: false,
                    message: format!(
                        "Meshtastic positions list refresh successful, tasks table update failed: {}",
                        e.to_string(),
                    ),
                }),
            );
        }
    };
}
