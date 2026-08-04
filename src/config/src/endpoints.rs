use const_format::formatcp;

pub const API_VERSION: &str = "v1";
pub const API_ENDPOINT: &str = formatcp!("api/{}", API_VERSION);

pub const QUERY_ENDPOINT: &str = formatcp!("{}/query", API_ENDPOINT);
pub const HEALTH_CHECK_ENDPOINT: &str = formatcp!("{}/health_check", QUERY_ENDPOINT);
