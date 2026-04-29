use super::{ApcCommands, Unit};
use serde::Serialize;
use uuid::Uuid;

/// We can probably have some builder pattern here.
#[derive(Debug, Serialize)]
pub struct ApcSetPayload {
    #[serde(rename = "cookerId")]
    pub cooker_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub unit: Unit,
}

#[derive(Debug, Serialize)]
pub struct ApcSet {
    pub command: ApcCommands,
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    pub payload: ApcSetPayload,
}
