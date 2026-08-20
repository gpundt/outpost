use crate::arguments::get_arguments;
use crate::meshtastic::connection::global_connection;
use crate::server_database::{
    delete::delete_from_table,
    insert::{insert_task_request_finish, insert_task_request_start},
    schema::backup_database,
};
use axum::{Json, http::StatusCode};
use config::tasks::OutpostTask;
use http::submit::{TaskRequest, TaskResponse};
use log::{error, info, warn};

/// Master functon to handle and direct task submission options
pub async fn task_submission_response(
    Json(request): Json<TaskRequest>,
) -> (StatusCode, Json<TaskResponse>) {
    info!("Task Received: {:?}", request.task);
    match request.task {
        OutpostTask::Backup => handle_backup().await,
        OutpostTask::Beacon => handle_beacon().await,
        OutpostTask::PurgeNodes => handle_purge_nodes().await,
        OutpostTask::PurgePositions => handle_purge_positions().await,
        OutpostTask::PurgeRaw => handle_purge_raw().await,
        OutpostTask::ReconnectSerial => handle_reconnect_serial().await,
        OutpostTask::Restart => handle_restart().await,
    }
}

/// Function to handle a backup task request
/// Returns an HTTP status code and a JSON response
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

/// Function to handle a beacon task request
/// Returns an HTTP status code and a JSON response
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

/// Function to handle a purge_nodes task request
/// Returns an HTTP status code and a JSON response
async fn handle_purge_nodes() -> (StatusCode, Json<TaskResponse>) {
    let row_id = match insert_task_request_start("purge_nodes").await {
        Ok(i) => i,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TaskResponse {
                    task: OutpostTask::PurgeNodes,
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
                task: OutpostTask::PurgeNodes,
                success: true,
                message: format!("Meshtastic nodes list successfully refreshed"),
            },
        ),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TaskResponse {
                    task: OutpostTask::PurgeNodes,
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

/// Function to handle a purge_raw task request
/// Returns an HTTP status code and a JSON response
async fn handle_purge_raw() -> (StatusCode, Json<TaskResponse>) {
    let row_id = match insert_task_request_start("purge_raw").await {
        Ok(i) => i,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TaskResponse {
                    task: OutpostTask::PurgeRaw,
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
                task: OutpostTask::PurgeRaw,
                success: true,
                message: format!("Meshtastic raw packets list successfully refreshed"),
            },
        ),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TaskResponse {
                    task: OutpostTask::PurgeRaw,
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
                    task: OutpostTask::PurgeRaw,
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

/// Function to handle a purge_positions task request
/// Returns an HTTP status code and a JSON response
async fn handle_purge_positions() -> (StatusCode, Json<TaskResponse>) {
    let row_id = match insert_task_request_start("purge_positions").await {
        Ok(i) => i,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TaskResponse {
                    task: OutpostTask::PurgePositions,
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
                task: OutpostTask::PurgePositions,
                success: true,
                message: format!("Meshtastic positions list successfully refreshed"),
            },
        ),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TaskResponse {
                    task: OutpostTask::PurgePositions,
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
                    task: OutpostTask::PurgePositions,
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

/// Function to initiate a reconnect with the server's serial device
/// Returns an HTTP status code and a JSON response
async fn handle_reconnect_serial() -> (StatusCode, Json<TaskResponse>) {
    let serial_port = get_arguments().serial_port.clone();

    // CHANGED: .lock().await - safe to hold across .await with tokio Mutex
    let mut connection = global_connection().lock().await;

    if connection.is_connected() {
        return (
            StatusCode::ALREADY_REPORTED,
            Json(TaskResponse {
                task: OutpostTask::ReconnectSerial,
                success: true,
                message: "Serial device already connected".to_string(),
            }),
        );
    }

    info!("Connecting to serial device: {}", serial_port);

    match connection.connect(serial_port.clone(), 115200).await {
        Ok(()) => (
            StatusCode::OK,
            Json(TaskResponse {
                task: OutpostTask::ReconnectSerial,
                success: true,
                message: format!("Successfully connected to {}", serial_port),
            }),
        ),
        Err(e) => {
            error!("{}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TaskResponse {
                    task: OutpostTask::ReconnectSerial,
                    success: false,
                    message: e.to_string(),
                }),
            )
        }
    }
}

/// Function to gracefully exit and allow the systemd daemon to reload the server
/// Returns an HTTP status code and a JSON response
async fn handle_restart() -> (StatusCode, Json<TaskResponse>) {
    warn!("Restart task received - shutting down after response is sent");

    tokio::spawn(async {
        tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;
        info!("Executing restart - exiting process");
        std::process::exit(0)
    });

    (
        StatusCode::OK,
        Json(TaskResponse {
            task: OutpostTask::Restart,
            success: true,
            message: "Daemon is restarting. Reconnect shortly".to_string(),
        }),
    )
}
