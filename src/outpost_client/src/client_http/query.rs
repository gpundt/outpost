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
            QueryError::Http(e) => write!(f, "HTTP/Network Error: {e}"),
            QueryError::Serialization(e) => write!(f, "JSON Serialization Error: {e}"),
        }
    }
}

impl std::error::Error for QueryError {}

pub async fn query_server(
    endpoint: String,
    body: Option<QueryRequest>,
) -> Result<QueryResponse, QueryError> {
    let response = get_server_connection()
        .get(get_server_config().url(endpoint.clone()))
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
            error!("Invalid endpoint: {}", endpoint);
            std::process::exit(1);
        }
    };

    if get_arguments().debug {
        debug!("{:?}", packaged_struct);
    }
    Ok(packaged_struct)
}
