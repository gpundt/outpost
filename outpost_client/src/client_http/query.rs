use core::fmt;
use database::schema::{
    HTTPRequestEntry, MeshtasticNodeEntry, MeshtasticPositionEntry, MeshtasticRawEntry,
    MeshtasticTextEntry,
};

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
    Texts(Vec<MeshtasticTextEntry>),
    Nodes(Vec<MeshtasticNodeEntry>),
    Positions(Vec<MeshtasticPositionEntry>),
    RawPackets(Vec<MeshtasticRawEntry>),
    HttpRequests(Vec<HTTPRequestEntry>),
}
impl fmt::Display for QueryResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryResponse::HealthCheck(r) => write!(f, "[Server Health Check]: {}", r),
            QueryResponse::Config(r) => write!(f, "[Server Config Query]: {}", r),
            QueryResponse::Status(r) => write!(f, "[Server Status Query]: {}", r),
            QueryResponse::Texts(r) => {
                let _ = write!(f, "[Meshtastic Texts Query]:");
                for entry in r {
                    let _ = write!(f, "{}", entry);
                }
                Ok(())
            }
            QueryResponse::HttpRequests(r) => {
                let _ = write!(f, "[HTTP Requests Query]:");
                for entry in r {
                    let _ = write!(f, "{}", entry);
                }
                Ok(())
            }
            QueryResponse::Nodes(r) => {
                let _ = write!(f, "[Meshtastic Nodes Query]:");
                for entry in r {
                    let _ = write!(f, "{}", entry);
                }
                Ok(())
            }
            QueryResponse::Positions(r) => {
                let _ = write!(f, "[Meshtastic Positions Query]:");
                for entry in r {
                    let _ = write!(f, "{}", entry);
                }
                Ok(())
            }
            QueryResponse::RawPackets(r) => {
                let _ = write!(f, "[Raw Packets Query]:");
                for entry in r {
                    let _ = write!(f, "{}", entry);
                }
                Ok(())
            }
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
async fn query_server(
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
        QueryType::Texts => {
            QueryResponse::Texts(MeshtasticTextEntry::from_json(body_text).unwrap())
        }
        QueryType::Nodes => {
            QueryResponse::Nodes(MeshtasticNodeEntry::from_json(body_text).unwrap())
        }
        QueryType::HttpRequests => {
            QueryResponse::HttpRequests(HTTPRequestEntry::from_json(body_text).unwrap())
        }
        QueryType::RawPackets => {
            QueryResponse::RawPackets(MeshtasticRawEntry::from_json(body_text).unwrap())
        }
        QueryType::Positions => {
            QueryResponse::Positions(MeshtasticPositionEntry::from_json(body_text).unwrap())
        }
        _ => {
            error!("Unsupported type");
            return None;
        }
    };

    if get_arguments().debug {
        debug!("{}", packaged_struct);
    }
    Some(packaged_struct)
}

pub async fn query_server_health_check() -> Option<HealthCheckResponse> {
    match query_server(QueryType::HealthCheck, None).await? {
        QueryResponse::HealthCheck(r) => Some(r),
        other => {
            error!("Unexpected response type for HealthCheck: {}", other);
            None
        }
    }
}

pub async fn query_server_config() -> Option<ConfigResponse> {
    match query_server(QueryType::ServerConfig, None).await? {
        QueryResponse::Config(r) => Some(r),
        other => {
            error!("Unexpected response type for ServerConfig: {}", other);
            None
        }
    }
}

pub async fn query_server_status() -> Option<StatusResponse> {
    match query_server(QueryType::ServerStatus, None).await? {
        QueryResponse::Status(r) => Some(r),
        other => {
            error!("Unexpected response type for ServerStatus: {}", other);
            None
        }
    }
}

pub async fn query_server_texts(count: Option<u32>) -> Option<Vec<MeshtasticTextEntry>> {
    let parameters = Some(serde_json::json!({ "count": count.unwrap_or(100) }));
    match query_server(QueryType::Texts, parameters).await? {
        QueryResponse::Texts(r) => Some(r),
        other => {
            error!("Unexpected response type for Texts: {}", other);
            None
        }
    }
}

pub async fn query_server_nodes() -> Option<Vec<MeshtasticNodeEntry>> {
    match query_server(QueryType::Nodes, None).await? {
        QueryResponse::Nodes(r) => Some(r),
        other => {
            error!("Unexpected response type for Nodes: {}", other);
            None
        }
    }
}

pub async fn query_server_raw_packets() -> Option<Vec<MeshtasticRawEntry>> {
    match query_server(QueryType::RawPackets, None).await? {
        QueryResponse::RawPackets(r) => Some(r),
        other => {
            error!("Unexpected response type for RawPackets: {}", other);
            None
        }
    }
}

pub async fn query_server_positions() -> Option<Vec<MeshtasticPositionEntry>> {
    match query_server(QueryType::Positions, None).await? {
        QueryResponse::Positions(r) => Some(r),
        other => {
            error!("Unexpected response type for Positions: {}", other);
            None
        }
    }
}

pub async fn query_server_http_requests(count: Option<u32>) -> Option<Vec<HTTPRequestEntry>> {
    let parameters = Some(serde_json::json!({ "count": count.unwrap_or(100) }));
    match query_server(QueryType::HttpRequests, parameters).await? {
        QueryResponse::HttpRequests(r) => Some(r),
        other => {
            error!("Unexpected response type for HttpRequests: {}", other);
            None
        }
    }
}
