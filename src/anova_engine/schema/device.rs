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

/// Docstring
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AnovaCommandType {
    // visible devices
    #[serde(rename = "EVENT_APC_WIFI_LIST")]
    EventApcWifiList,
    // available devices?
    #[serde(rename = "EVENT_APC_WIFI_VERSION")]
    EventApcWifiVersion,
    // ?
    #[serde(rename = "EVENT_USER_STATE")]
    EventUserState,
    //
    #[serde(rename = "EVENT_APC_STATE")]
    EventApcState,
}
