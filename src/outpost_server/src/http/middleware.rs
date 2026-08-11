use crate::database::insert_http_request;
use crate::database::schema::HTTPRequestEntry;
use axum::{body::Body, extract::ConnectInfo, http::Request, middleware::Next, response::Response};
use chrono::Utc;
use log::info;
use std::net::SocketAddr;

pub async fn log_request_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let method = req.method().to_string();
    let endpoint = req.uri().path().to_string();
    let source = addr.to_string();

    let user_agent = req
        .headers()
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    // pass request to handler to give client a response
    let response = next.run(req).await;
    let status_code = response.status().as_u16();

    info!(
        "HTTP {} {} from {} ({}) -> {}",
        method, endpoint, source, user_agent, status_code
    );

    tokio::spawn(async move {
        if let Err(e) = insert_http_request(HTTPRequestEntry {
            id: 0,
            method,
            source,
            endpoint,
            user_agent,
            status_code: status_code,
            timestamp: Utc::now().naive_utc(),
        })
        .await
        {
            log::error!("Failed to log HTTP request to database: {}", e);
        }
    });

    response
}
