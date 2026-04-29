use super::ApcCommands;
use serde::Serialize;
use uuid::Uuid;

/// We can probably have some builder pattern here.
#[derive(Debug, Serialize)]
pub struct ApcStopPayload {
    #[serde(rename = "cookerId")]
    pub cooker_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(Debug, Serialize)]
pub struct ApcStop {
    pub command: ApcCommands,
    #[serde(rename = "requestId")]
    pub request_id: Uuid,
    pub payload: ApcStopPayload,
}
