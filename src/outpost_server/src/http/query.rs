use crate::arguments::get_arguments;
use axum::Json;
use config::endpoints::{CONFIG_QUERY_ENDPOINT, HEALTH_CHECK_ENDPOINT};
use config::time::get_uptime_str;
use log::{debug, error, info, trace, warn};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheckResponse {
    status: String,
    uptime: String,
}

pub async fn health_check_response() -> Json<HealthCheckResponse> {
    trace!("HTTP GET: {}", HEALTH_CHECK_ENDPOINT);
    let payload = HealthCheckResponse {
        status: "Healthy".to_string(),
        uptime: get_uptime_str(),
    };
    Json(payload)
}

#[derive(Serialize)]
pub struct ConfigResponse {
    verbose: bool,
    port: u16,
}

pub async fn config_query_response() -> Json<ConfigResponse> {
    trace!("HTTP GET: {}", CONFIG_QUERY_ENDPOINT);
    let payload = ConfigResponse {
        verbose: get_arguments().verbose,
        port: get_arguments().port,
    };
    return Json(payload);
}
