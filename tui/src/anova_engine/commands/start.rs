use super::{ApcCommands, Unit};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ApcStartPayload {
    #[serde(rename = "cookerId")]
    pub cooker_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "targetTemperature")]
    pub target_temperature: f64,
    pub unit: Unit,
    pub timer: u64,
}

#[derive(Debug, Serialize)]
pub struct ApcStart {
    pub command: ApcCommands,
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    pub payload: ApcStartPayload,
}
