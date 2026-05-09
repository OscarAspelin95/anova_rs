use serde::{Deserialize, Serialize};
use strum::Display;

/// event and response are the main types of data
/// we get back from the apc device.
///
/// event -> data that is continuously sent.
/// response -> if request to device was successful or not.
#[derive(Debug, Serialize, Deserialize, PartialEq, Display)]
pub enum AnovaCommandType {
    // visible devices
    #[serde(rename = "EVENT_APC_WIFI_LIST")]
    EventApcWifiList,
    // available devices?
    #[serde(rename = "EVENT_APC_WIFI_VERSION")]
    EventApcWifiVersion,
    //
    #[serde(rename = "EVENT_USER_STATE")]
    EventUserState,
    //
    #[serde(rename = "EVENT_APC_STATE")]
    EventApcState,
    // websocket response for a request.
    #[serde(rename = "RESPONSE")]
    Response,
}
