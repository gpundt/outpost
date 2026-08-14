use crate::client_http::connection::get_server_config;

use super::connection::get_server_connection;
use http::{
    endpoints::{
        CONFIG_QUERY_ENDPOINT, HEALTH_CHECK_ENDPOINT, QUERY_ENDPOINT, STATUS_QUERY_ENDPOINT,
    },
    query::{ConfigResponse, HealthCheckResponse, StatusResponse},
};
use log::info;
use reqwest::{self, Response};
use serde_json;

#[derive(Debug)]
pub enum QueryError {
    Http(reqwest::Error),
    Serialization(serde_json::Error),
}
impl std::fmt::Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryError::Http(e) => write!(f, "HTTP/Network Error: {e}"),
            QueryError::Serialization(e) => write!(f, "JSON Serialization Error: {e}"),
        }
    }
}

impl std::error::Error for QueryError {}

pub async fn query_health_check(silent: bool) -> Result<HealthCheckResponse, QueryError> {
    let response = get_server_connection()
        .get(get_server_config().url(HEALTH_CHECK_ENDPOINT.to_string()))
        .send()
        .await
        .map_err(|e| QueryError::Http(e))?;

    let successful_response = response
        .error_for_status()
        .map_err(|e| QueryError::Http(e))?;

    let body_text = successful_response
        .text()
        .await
        .map_err(|e| QueryError::Http(e))?;

    let serialized_response: HealthCheckResponse =
        serde_json::from_str(&body_text).map_err(|e| QueryError::Serialization(e))?;

    if !silent {
        info!("{:?}", serialized_response);
    }
    Ok(serialized_response)
}

pub async fn query_server_status() -> Result<StatusResponse, QueryError> {
    let response = get_server_connection()
        .get(get_server_config().url(STATUS_QUERY_ENDPOINT.to_string()))
        .send()
        .await
        .map_err(|e| QueryError::Http(e))?;

    let successful_response = response
        .error_for_status()
        .map_err(|e| QueryError::Http(e))?;

    let body_text = successful_response
        .text()
        .await
        .map_err(|e| QueryError::Http(e))?;

    let serialized_response: StatusResponse =
        serde_json::from_str(&body_text).map_err(|e| QueryError::Serialization(e))?;

    info!("{:?}", serialized_response);
    Ok(serialized_response)
}

pub async fn query_server_config() -> Result<ConfigResponse, QueryError> {
    let response = get_server_connection()
        .get(get_server_config().url(CONFIG_QUERY_ENDPOINT.to_string()))
        .send()
        .await
        .map_err(|e| QueryError::Http(e))?;

    let successful_response = response
        .error_for_status()
        .map_err(|e| QueryError::Http(e))?;

    let body_text = successful_response
        .text()
        .await
        .map_err(|e| QueryError::Http(e))?;

    let serialized_response: ConfigResponse =
        serde_json::from_str(&body_text).map_err(|e| QueryError::Serialization(e))?;

    info!("{:?}", serialized_response);
    Ok(serialized_response)
}
