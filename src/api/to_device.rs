//! outgoing API payload to the device.

use super::types::TemperatureUnit;
use serde::Serialize;
use uuid::Uuid;

/// App -> Engine
#[derive(Debug, Clone, Serialize)]
pub enum ApiRequest {
    // start cooking session.
    Start(ApcStartPayload),
    // switch C <-> F.
    Set(ApcSetPayload),
    // stop cooking session.
    Stop(ApcStopPayload),
}

// -----------------------------------------
/// Engine -> API
#[derive(Debug, Clone, Serialize)]
pub enum ApiRequestCommand {
    #[serde(rename = "CMD_APC_START")]
    CmdApcStart,
    #[serde(rename = "CMD_APC_SET_TEMPERATURE_UNIT")]
    CmdApcSetTemperatureUnit,
    #[serde(rename = "CMD_APC_STOP")]
    CmdApcStop,
}

// ----------------------------------------
/// Engine -> API
#[derive(Debug, Clone, Serialize)]
pub struct ApcStartPayload {
    #[serde(rename = "cookerId")]
    pub cooker_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "targetTemperature")]
    pub target_temperature: f64,
    pub unit: TemperatureUnit,
    pub timer: u64,
}

/// Engine -> API
#[derive(Debug, Clone, Serialize)]
pub struct ApcStart {
    pub command: ApiRequestCommand,
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    pub payload: ApcStartPayload,
}

/// Engine -> API
impl From<ApcStartPayload> for ApcStart {
    fn from(payload: ApcStartPayload) -> Self {
        Self {
            command: ApiRequestCommand::CmdApcStart,
            request_id: Uuid::new_v4(),
            payload,
        }
    }
}

// ----------------------------------------
/// Engine -> API
#[derive(Debug, Clone, Serialize)]
pub struct ApcStopPayload {
    #[serde(rename = "cookerId")]
    pub cooker_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApcStop {
    pub command: ApiRequestCommand,
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    pub payload: ApcStopPayload,
}

/// Engine -> API
impl From<ApcStopPayload> for ApcStop {
    fn from(payload: ApcStopPayload) -> Self {
        Self {
            command: ApiRequestCommand::CmdApcStop,
            request_id: Uuid::new_v4(),
            payload,
        }
    }
}

// ----------------------------------------
/// Engine -> API
#[derive(Debug, Clone, Serialize)]
pub struct ApcSetPayload {
    #[serde(rename = "cookerId")]
    pub cooker_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub unit: TemperatureUnit,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApcSet {
    pub command: ApiRequestCommand,
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    pub payload: ApcSetPayload,
}

/// Engine -> API
impl From<ApcSetPayload> for ApcSet {
    fn from(payload: ApcSetPayload) -> Self {
        Self {
            command: ApiRequestCommand::CmdApcSetTemperatureUnit,
            request_id: Uuid::new_v4(),
            payload,
        }
    }
}
