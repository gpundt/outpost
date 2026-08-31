use std::{
    error::Error,
    sync::{Arc, OnceLock, RwLock},
};

use crate::client_http::{
    query::query_server_tasks,
    submit::{
        submit_backup_task, submit_beacon_task, submit_purge_nodes_task,
        submit_purge_positions_task, submit_purge_raw_task, submit_reconnect_serial_task,
        submit_restart_task,
    },
};

use super::query::{
    query_server_config, query_server_health_check, query_server_nodes, query_server_positions,
    query_server_raw_packets, query_server_status, query_server_texts,
};
use config::tasks::OutpostTask;
use database::schema::{
    MeshtasticNodeEntry, MeshtasticPositionEntry, MeshtasticRawEntry, MeshtasticTextEntry,
    TaskRequestEntry,
};
use http::query::{ConfigResponse, HealthCheckResponse, StatusResponse};
use http::submit::TaskResponse;

#[derive(Default, Debug, Clone)]
pub struct ResponseStorage {
    pub health_check: Option<HealthCheckResponse>,
    pub server_config: Option<ConfigResponse>,
    pub server_status: Option<StatusResponse>,
    pub server_texts: Option<Vec<MeshtasticTextEntry>>,
    pub server_tasks: Option<Vec<TaskRequestEntry>>,
    pub server_nodes: Option<Vec<MeshtasticNodeEntry>>,
    pub server_positions: Option<Vec<MeshtasticPositionEntry>>,
    pub server_raw_packets: Option<Vec<MeshtasticRawEntry>>,
    pub server_task_response: Option<TaskResponse>,
}

pub fn global_response_storage() -> &'static Arc<RwLock<ResponseStorage>> {
    static STORAGE: OnceLock<Arc<RwLock<ResponseStorage>>> = OnceLock::new();
    STORAGE.get_or_init(|| Arc::new(RwLock::new(ResponseStorage::default())))
}

pub async fn update_response_storage_master() {
    let _ = update_health_check().await;
    let _ = update_server_config().await;
    let _ = update_server_status().await;
    let _ = update_server_tasks().await;
    let _ = update_server_texts().await;
    let _ = update_server_nodes().await;
    let _ = update_server_positions().await;
    let _ = update_server_raw_packets().await;
}

async fn update_health_check() -> Result<(), Box<dyn Error>> {
    let health_check = query_server_health_check().await;
    let mut storage = global_response_storage().write().unwrap();
    storage.health_check = health_check;

    Ok(())
}
async fn update_server_config() -> Result<(), Box<dyn Error>> {
    let server_config = query_server_config().await;
    let mut storage = global_response_storage().write().unwrap();
    storage.server_config = server_config;

    Ok(())
}
async fn update_server_status() -> Result<(), Box<dyn Error>> {
    let server_status = query_server_status().await;
    let mut storage = global_response_storage().write().unwrap();
    storage.server_status = server_status;

    Ok(())
}
async fn update_server_texts() -> Result<(), Box<dyn Error>> {
    let server_texts = query_server_texts(None).await;
    let mut storage = global_response_storage().write().unwrap();
    storage.server_texts = server_texts;

    Ok(())
}
async fn update_server_tasks() -> Result<(), Box<dyn Error>> {
    let server_tasks = query_server_tasks(None).await;
    let mut storage = global_response_storage().write().unwrap();
    storage.server_tasks = server_tasks;

    Ok(())
}
async fn update_server_nodes() -> Result<(), Box<dyn Error>> {
    let server_nodes = query_server_nodes().await;
    let mut storage = global_response_storage().write().unwrap();
    storage.server_nodes = server_nodes;

    Ok(())
}
async fn update_server_positions() -> Result<(), Box<dyn Error>> {
    let server_positions = query_server_positions().await;
    let mut storage = global_response_storage().write().unwrap();
    storage.server_positions = server_positions;

    Ok(())
}
async fn update_server_raw_packets() -> Result<(), Box<dyn Error>> {
    let server_raw_packets = query_server_raw_packets().await;
    let mut storage = global_response_storage().write().unwrap();
    storage.server_raw_packets = server_raw_packets;

    Ok(())
}
pub async fn update_server_task_response(task_type: OutpostTask) -> Result<(), Box<dyn Error>> {
    let mut storage = global_response_storage().write().unwrap();

    let response: Option<TaskResponse> = match task_type {
        OutpostTask::Backup => submit_backup_task().await,
        OutpostTask::Beacon => submit_beacon_task().await,
        OutpostTask::PurgeNodes => submit_purge_nodes_task().await,
        OutpostTask::PurgeRaw => submit_purge_raw_task().await,
        OutpostTask::PurgePositions => submit_purge_positions_task().await,
        OutpostTask::ReconnectSerial => submit_reconnect_serial_task().await,
        OutpostTask::Restart => submit_restart_task().await,
    };
    storage.server_task_response = response;

    Ok(())
}

pub fn get_health_check() -> Option<HealthCheckResponse> {
    let storage = global_response_storage().read().unwrap();
    return storage.clone().health_check;
}
pub fn get_server_config() -> Option<ConfigResponse> {
    let storage = global_response_storage().read().unwrap();
    return storage.clone().server_config;
}
pub fn get_server_status() -> Option<StatusResponse> {
    let storage = global_response_storage().read().unwrap();
    return storage.clone().server_status;
}
pub fn get_server_texts() -> Option<Vec<MeshtasticTextEntry>> {
    let storage = global_response_storage().read().unwrap();
    return storage.clone().server_texts;
}
pub fn get_server_tasks() -> Option<Vec<TaskRequestEntry>> {
    let storage = global_response_storage().read().unwrap();
    return storage.clone().server_tasks;
}
pub fn get_server_nodes() -> Option<Vec<MeshtasticNodeEntry>> {
    let storage = global_response_storage().read().unwrap();
    return storage.clone().server_nodes;
}
pub fn get_server_positions() -> Option<Vec<MeshtasticPositionEntry>> {
    let storage = global_response_storage().read().unwrap();
    return storage.clone().server_positions;
}
pub fn get_server_raw_packets() -> Option<Vec<MeshtasticRawEntry>> {
    let storage = global_response_storage().read().unwrap();
    return storage.clone().server_raw_packets;
}
pub fn get_server_task_response() -> Option<TaskResponse> {
    let storage = global_response_storage().read().unwrap();
    return storage.clone().server_task_response;
}
