use crate::{arguments::get_arguments, client_http::connection::get_server_config};

use super::connection::get_server_connection;
use http::{
    endpoints::{
        CONFIG_QUERY_ENDPOINT, HEALTH_CHECK_ENDPOINT, QUERY_ENDPOINT, STATUS_QUERY_ENDPOINT,
    },
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
pub async fn query_server(endpoint: String, body: Option<QueryRequest>) -> Option<QueryResponse> {
    let mut response = get_server_connection().get(get_server_config().url(endpoint.clone()));

    if let Some(request_body) = body {
        response = response.json(&request_body);
    }

    let response = match response.send().await.map_err(|e| QueryError::Http(e)) {
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
        QUERY_ENDPOINT => {
            QueryResponse::Database(serde_json::from_str(&body_text.to_string()).unwrap())
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

pub async fn query_server_texts(count: Option<u32>) -> Option<QueryResponse> {
    let body = QueryRequest {
        query_type: QueryType::Texts,
        parameters: Some(serde_json::json!({ "count": count.unwrap_or(100) })),
    };
    query_server(QUERY_ENDPOINT.to_string(), Some(body)).await
}

pub async fn query_server_nodes() -> Option<QueryResponse> {
    let body = QueryRequest {
        query_type: QueryType::Nodes,
        parameters: None,
    };
    query_server(QUERY_ENDPOINT.to_string(), Some(body)).await
}

pub async fn query_server_raw_packets() -> Option<QueryResponse> {
    let body = QueryRequest {
        query_type: QueryType::RawPackets,
        parameters: None,
    };
    query_server(QUERY_ENDPOINT.to_string(), Some(body)).await
}

pub async fn query_server_positions() -> Option<QueryResponse> {
    let body = QueryRequest {
        query_type: QueryType::Positions,
        parameters: None,
    };
    query_server(QUERY_ENDPOINT.to_string(), Some(body)).await
}

pub async fn query_server_http_requests(count: Option<u32>) -> Option<QueryResponse> {
    let body = QueryRequest {
        query_type: QueryType::HttpRequests,
        parameters: Some(serde_json::json!({ "count": count.unwrap_or(100) })),
    };
    query_server(QUERY_ENDPOINT.to_string(), Some(body)).await
}
