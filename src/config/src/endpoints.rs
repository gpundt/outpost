use const_format::formatcp;

pub const API_VERSION: &str = "v1";
pub const API_ENDPOINT: &str = formatcp!("/api/{}", API_VERSION);

pub const QUERY_ENDPOINT: &str = formatcp!("{}/query", API_ENDPOINT);
pub const HEALTH_CHECK_ENDPOINT: &str = formatcp!("{}/health_check", QUERY_ENDPOINT);
pub const CONFIG_QUERY_ENDPOINT: &str = formatcp!("{}/config", QUERY_ENDPOINT);
pub const STATUS_QUERY_ENDPOINT: &str = formatcp!("{}/status", QUERY_ENDPOINT);
pub const TEXTS_QUERY_ENDPOINT: &str = formatcp!("{}/texts", QUERY_ENDPOINT);
pub const NODES_QUERY_ENDPOINT: &str = formatcp!("{}/nodes", QUERY_ENDPOINT);
pub const POSITIONS_QUERY_ENDPOINT: &str = formatcp!("{}/positions", QUERY_ENDPOINT);
pub const RAW_PACKETS_QUERY_ENDPOINT: &str = formatcp!("{}/raw_packets", QUERY_ENDPOINT);

pub const SUBMISSION_ENDPOINT: &str = formatcp!("{}/submit", API_ENDPOINT);
pub const SUBMIT_CONFIG_ENDPOINT: &str = formatcp!("{}/config", SUBMISSION_ENDPOINT);
pub const SUBMIT_TASK_ENDPOINT: &str = formatcp!("{}/task", SUBMISSION_ENDPOINT);

pub const TRANSMISSION_ENDPOINT: &str = formatcp!("{}/transmit", API_ENDPOINT);
pub const TRANSMIT_TEXT_ENDPOINT: &str = formatcp!("{}/text", TRANSMISSION_ENDPOINT);
