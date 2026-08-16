use core::fmt;

use crate::{arguments::get_arguments, client_http::connection::get_server_config};

use super::connection::get_server_connection;
use http::{
    endpoints::QUERY_ENDPOINT,
    query::{ConfigResponse, HealthCheckResponse, QueryRequest, QueryType, StatusResponse},
};
use log::{debug, error};
use reqwest::{self};
use serde_json;

/// Enum to organize the available query response options
#[derive(Debug)]
pub enum QueryResponse {
    HealthCheck(HealthCheckResponse),
    Config(ConfigResponse),
    Status(StatusResponse),
    Database(Option<Vec<serde_json::Value>>),
}
impl fmt::Display for QueryResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryResponse::Database(r) => write!(f, "[Database Query]: {:?}", r),
            QueryResponse::HealthCheck(r) => write!(f, "[Health Check]: {}", r),
            QueryResponse::Config(r) => write!(f, "[Config Query]: {}", r),
            QueryResponse::Status(r) => write!(f, "[Status Query]: {}", r),
        }
    }
}

/// Enum to organize the available query error options
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

/// Generic function to query and endpoint and return an organized response struct
pub async fn query_server(
    query_type: QueryType,
    parameters: Option<serde_json::Value>,
) -> Option<QueryResponse> {
    let response = get_server_connection().get(get_server_config().url(QUERY_ENDPOINT.to_string()));

    let request_body = QueryRequest::new(query_type.clone(), parameters);

    let response = match response
        .json(&request_body)
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

    let packaged_struct: QueryResponse = match query_type.clone() {
        QueryType::HealthCheck => {
            QueryResponse::HealthCheck(HealthCheckResponse::from_json(body_text).unwrap())
        }
        QueryType::ServerConfig => {
            QueryResponse::Config(ConfigResponse::from_json(body_text).unwrap())
        }
        QueryType::ServerStatus => {
            QueryResponse::Status(StatusResponse::from_json(body_text).unwrap())
        }
        _ => QueryResponse::Database(serde_json::from_str(&body_text.to_string()).unwrap()),
    };

    if get_arguments().debug {
        debug!("{}", packaged_struct);
    }
    Some(packaged_struct)
}

pub async fn query_server_health_check() -> Option<QueryResponse> {
    query_server(QueryType::HealthCheck, None).await
}

pub async fn query_server_config() -> Option<QueryResponse> {
    query_server(QueryType::ServerConfig, None).await
}

pub async fn query_server_status() -> Option<QueryResponse> {
    query_server(QueryType::ServerStatus, None).await
}

pub async fn query_server_texts(count: Option<u32>) -> Option<QueryResponse> {
    let parameters = Some(serde_json::json!({ "count": count.unwrap_or(100) }));
    query_server(QueryType::Texts, parameters).await
}

pub async fn query_server_nodes() -> Option<QueryResponse> {
    query_server(QueryType::Nodes, None).await
}

pub async fn query_server_raw_packets() -> Option<QueryResponse> {
    query_server(QueryType::RawPackets, None).await
}

pub async fn query_server_positions() -> Option<QueryResponse> {
    query_server(QueryType::Positions, None).await
}

pub async fn query_server_http_requests(count: Option<u32>) -> Option<QueryResponse> {
    let parameters = Some(serde_json::json!({ "count": count.unwrap_or(100) }));
    query_server(QueryType::HttpRequests, parameters).await
}
