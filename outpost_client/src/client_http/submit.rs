use super::connection::get_server_connection;
use config::tasks::OutpostTask;
use http::{
    endpoints::SUBMISSION_ENDPOINT,
    errors::RequestError,
    submit::{TaskRequest, TaskResponse},
};

use crate::{arguments::get_arguments, client_http::connection::get_server_config};

use log::{error, info};
use serde_json;

/// Generic function to submit a task to the server and return an organized response struct
async fn submit_task(
    task_type: OutpostTask,
    parameters: Option<serde_json::Value>,
) -> Option<TaskResponse> {
    let response =
        get_server_connection().post(get_server_config().url(SUBMISSION_ENDPOINT.to_string()));

    let request_body = TaskRequest::new(task_type, parameters);

    let response = match response
        .json(&request_body)
        .send()
        .await
        .map_err(|e| RequestError::Http(e))
    {
        Ok(b) => b,
        Err(e) => {
            error!("{}", e);
            return None;
        }
    };

    let successful_response = match response
        .error_for_status()
        .map_err(|e| RequestError::Http(e))
    {
        Ok(b) => b,
        Err(e) => {
            error!("{}", e);
            return None;
        }
    };

    let body_text = match successful_response
        .text()
        .await
        .map_err(|e| RequestError::Http(e))
    {
        Ok(b) => b,
        Err(e) => {
            error!("{}", e);
            return None;
        }
    };

    let packaged_struct = TaskResponse::from_json(body_text).unwrap();

    if get_arguments().debug {
        info!("{}", packaged_struct);
    }

    Some(packaged_struct)
}

pub async fn submit_backup_task() -> Option<TaskResponse> {
    submit_task(OutpostTask::Backup, None).await
}

pub async fn submit_beacon_task() -> Option<TaskResponse> {
    submit_task(OutpostTask::Beacon, None).await
}

pub async fn submit_purge_nodes_task() -> Option<TaskResponse> {
    submit_task(OutpostTask::PurgeNodes, None).await
}

pub async fn submit_purge_positions_task() -> Option<TaskResponse> {
    submit_task(OutpostTask::PurgePositions, None).await
}

pub async fn submit_reconnect_serial_task() -> Option<TaskResponse> {
    submit_task(OutpostTask::ReconnectSerial, None).await
}

pub async fn submit_restart_task() -> Option<TaskResponse> {
    submit_task(OutpostTask::Restart, None).await
}
