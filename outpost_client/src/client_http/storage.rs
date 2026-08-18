use std::{
    error::Error,
    sync::{Arc, OnceLock, RwLock},
};

use crate::client_http::query::query_server_tasks;

use super::query::{
    query_server_config, query_server_health_check, query_server_http_requests, query_server_nodes,
    query_server_positions, query_server_raw_packets, query_server_status, query_server_texts,
};
use database::schema::{
    HTTPRequestEntry, MeshtasticNodeEntry, MeshtasticPositionEntry, MeshtasticRawEntry,
    MeshtasticTextEntry, TaskRequestEntry,
};
use http::query::{ConfigResponse, HealthCheckResponse, StatusResponse};

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
    pub server_http_requests: Option<Vec<HTTPRequestEntry>>,
}

pub fn global_response_storage() -> &'static Arc<RwLock<ResponseStorage>> {
    static STORAGE: OnceLock<Arc<RwLock<ResponseStorage>>> = OnceLock::new();
    STORAGE.get_or_init(|| Arc::new(RwLock::new(ResponseStorage::default())))
}

pub async fn update_health_check() -> Result<(), Box<dyn Error>> {
    let health_check = query_server_health_check().await;
    let mut storage = global_response_storage().write().unwrap();
    storage.health_check = health_check;

    Ok(())
}
pub async fn update_server_config() -> Result<(), Box<dyn Error>> {
    let server_config = query_server_config().await;
    let mut storage = global_response_storage().write().unwrap();
    storage.server_config = server_config;

    Ok(())
}
pub async fn update_server_status() -> Result<(), Box<dyn Error>> {
    let server_status = query_server_status().await;
    let mut storage = global_response_storage().write().unwrap();
    storage.server_status = server_status;

    Ok(())
}
pub async fn update_server_texts() -> Result<(), Box<dyn Error>> {
    let server_texts = query_server_texts(None).await;
    let mut storage = global_response_storage().write().unwrap();
    storage.server_texts = server_texts;

    Ok(())
}
pub async fn update_server_tasks() -> Result<(), Box<dyn Error>> {
    let server_tasks = query_server_tasks(None).await;
    let mut storage = global_response_storage().write().unwrap();
    storage.server_tasks = server_tasks;

    Ok(())
}
pub async fn update_server_nodes() -> Result<(), Box<dyn Error>> {
    let server_nodes = query_server_nodes().await;
    let mut storage = global_response_storage().write().unwrap();
    storage.server_nodes = server_nodes;

    Ok(())
}
pub async fn update_server_positions() -> Result<(), Box<dyn Error>> {
    let server_positions = query_server_positions().await;
    let mut storage = global_response_storage().write().unwrap();
    storage.server_positions = server_positions;

    Ok(())
}
pub async fn update_server_raw_packets() -> Result<(), Box<dyn Error>> {
    let server_raw_packets = query_server_raw_packets().await;
    let mut storage = global_response_storage().write().unwrap();
    storage.server_raw_packets = server_raw_packets;

    Ok(())
}
pub async fn update_server_http_requests() -> Result<(), Box<dyn Error>> {
    let server_http_requests = query_server_http_requests(None).await;
    let mut storage = global_response_storage().write().unwrap();
    storage.server_http_requests = server_http_requests;

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
pub fn get_server_http_requests() -> Option<Vec<HTTPRequestEntry>> {
    let storage = global_response_storage().read().unwrap();
    return storage.clone().server_http_requests;
}
