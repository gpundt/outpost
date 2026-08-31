use core::fmt;

use serde::{Deserialize, Serialize};

/// Enum to restrict the available task options
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OutpostTask {
    Backup,
    Beacon,
    PurgeNodes,
    PurgeRaw,
    PurgePositions,
    ReconnectSerial,
    Restart,
}

impl fmt::Display for OutpostTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutpostTask::Backup => write!(f, "backup"),
            OutpostTask::Beacon => write!(f, "beacon"),
            OutpostTask::PurgeNodes => write!(f, "purge_nodes"),
            OutpostTask::PurgePositions => write!(f, "purge_positions"),
            OutpostTask::PurgeRaw => write!(f, "purge_raw"),
            OutpostTask::ReconnectSerial => write!(f, "reconnect_serial"),
            OutpostTask::Restart => write!(f, "restart"),
        }
    }
}
