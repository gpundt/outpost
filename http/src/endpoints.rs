use const_format::formatcp;

pub const API_VERSION: &str = "v1";
pub const API_ENDPOINT: &str = formatcp!("/api/{}", API_VERSION);

pub const QUERY_ENDPOINT: &str = formatcp!("{}/query", API_ENDPOINT);

pub const SUBMISSION_ENDPOINT: &str = formatcp!("{}/submit", API_ENDPOINT);
pub const SUBMIT_CONFIG_ENDPOINT: &str = formatcp!("{}/config", SUBMISSION_ENDPOINT);
pub const SUBMIT_TASK_ENDPOINT: &str = formatcp!("{}/task", SUBMISSION_ENDPOINT);

pub const TRANSMISSION_ENDPOINT: &str = formatcp!("{}/transmit", API_ENDPOINT);
pub const TRANSMIT_TEXT_ENDPOINT: &str = formatcp!("{}/text", TRANSMISSION_ENDPOINT);
