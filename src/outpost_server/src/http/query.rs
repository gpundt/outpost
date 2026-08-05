use axum::Json;
use config::endpoints::HEALTH_CHECK_ENDPOINT;
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
