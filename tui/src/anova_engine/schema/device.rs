use crate::types::AnovaDevice;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

impl AnovaCommand {
    pub fn is_apc_wifi_list_response(&self) -> bool {
        return self.command == AnovaCommandType::EventApcWifiList;
    }
}

/// Docstring
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AnovaCommandType {
    #[serde(rename = "EVENT_APC_WIFI_LIST")]
    EventApcWifiList,
    // currently unused.
    #[serde(rename = "EVENT_APO_WIFI_LIST")]
    EventApoWifiList,
    // currently unused.
    #[serde(rename = "EVENT_APC_WIFI_VERSION")]
    EventApcWifiVersion,
}
