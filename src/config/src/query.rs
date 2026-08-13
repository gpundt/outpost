use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryType {
    Texts,
    Nodes,
    RawPackets,
    Positions,
    HttpRequests,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryRequest {
    pub query_type: QueryType,
    pub parameters: Option<serde_json::Value>,
}

pub fn extract_count_parameter(parameters: &Option<serde_json::Value>, default: u32) -> u32 {
    parameters
        .as_ref()
        .and_then(|p| p.get("count"))
        .and_then(|v| v.as_u64())
        .and_then(|v| u32::try_from(v).ok())
        .unwrap_or(default)
}
