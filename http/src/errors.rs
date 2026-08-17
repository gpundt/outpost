/// Enum to organize the available query error options
#[derive(Debug)]
pub enum RequestError {
    Http(reqwest::Error),
    Serialization(serde_json::Error),
}
impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestError::Http(e) => write!(f, "HTTP/Network Failure: {e}"),
            RequestError::Serialization(e) => write!(f, "JSON Serialization Failure: {e}"),
        }
    }
}
impl std::error::Error for RequestError {}
