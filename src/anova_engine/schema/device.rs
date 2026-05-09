use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::api::apc::{events::apc_wifi_list::AnovaDevice, types::AnovaCommandType};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnovaDevices {
    pub devices: Vec<AnovaDevice>,
}

/// Docstring
#[derive(Debug, Serialize, Deserialize)]
pub struct AnovaCommand {
    pub command: AnovaCommandType,
    pub payload: Value,
}
