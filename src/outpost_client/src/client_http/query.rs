use crate::{arguments::get_arguments, client_http::connection::get_server_config};

use super::connection::get_server_connection;
use http::{
    endpoints::{
        CONFIG_QUERY_ENDPOINT, HEALTH_CHECK_ENDPOINT, QUERY_ENDPOINT, STATUS_QUERY_ENDPOINT,
    },
    query::{ConfigResponse, HealthCheckResponse, QueryRequest, StatusResponse},
};
use log::{debug, error};
use reqwest::{self};
use serde_json;

#[derive(Debug)]
pub enum QueryResponse {
    HealthCheck(HealthCheckResponse),
    Config(ConfigResponse),
    Status(StatusResponse),
}

#[derive(Debug)]
pub enum QueryError {
    Http(reqwest::Error),
    Serialization(serde_json::Error),
}
impl std::fmt::Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryError::Http(e) => write!(f, "HTTP/Network Failure: {e}"),
            QueryError::Serialization(e) => write!(f, "JSON Serialization Failure: {e}"),
        }
    }
}

impl std::error::Error for QueryError {}

pub async fn query_server(endpoint: String, body: Option<QueryRequest>) -> Option<QueryResponse> {
    let response = match get_server_connection()
        .get(get_server_config().url(endpoint.clone()))
        .send()
        .await
        .map_err(|e| QueryError::Http(e))
    {
        Ok(b) => b,
        Err(e) => {
            error!("{}", e);
            return None;
        }
    };

    let successful_response = match response.error_for_status().map_err(|e| QueryError::Http(e)) {
        Ok(b) => b,
        Err(e) => {
            error!("{}", e);
            return None;
        }
    };

    let body_text = match successful_response
        .text()
        .await
        .map_err(|e| QueryError::Http(e))
    {
        Ok(b) => b,
        Err(e) => {
            error!("{}", e);
            return None;
        }
    };

    let packaged_struct: QueryResponse = match endpoint.as_str() {
        HEALTH_CHECK_ENDPOINT => {
            QueryResponse::HealthCheck(HealthCheckResponse::from_json(body_text).unwrap())
        }
        CONFIG_QUERY_ENDPOINT => {
            QueryResponse::Config(ConfigResponse::from_json(body_text).unwrap())
        }
        STATUS_QUERY_ENDPOINT => {
            QueryResponse::Status(StatusResponse::from_json(body_text).unwrap())
        }
        _ => {
            error!("Unsupported query_server Endpoint: {}", endpoint);
            return None;
        }
    };

    if get_arguments().debug {
        debug!("{:?}", packaged_struct);
    }
    Some(packaged_struct)
}
