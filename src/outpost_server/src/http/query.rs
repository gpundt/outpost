use axum::Json;
use config::endpoints::HEALTH_CHECK_ENDPOINT;
use config::time::get_uptime;
use log::{debug, error, info, trace, warn};
use serde::Serialize;
use std::time::Duration;

#[derive(Serialize)]
pub struct HealthCheckResponse {
    status: String,
    uptime: Duration,
}

pub async fn health_check_response() -> Json<HealthCheckResponse> {
    trace!("HTTP GET: {}", HEALTH_CHECK_ENDPOINT);
    let payload = HealthCheckResponse {
        status: "Healthy".to_string(),
        uptime: get_uptime(),
    };
    Json(payload)
}
