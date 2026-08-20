use axum::{body::Body, extract::ConnectInfo, http::Request, middleware::Next, response::Response};
use log::debug;
use std::net::SocketAddr;

/// Function to log incoming HTTP requests
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

    debug!(
        "HTTP {} {} from {} ({}) -> {}",
        method, endpoint, source, user_agent, status_code
    );

    response
}
