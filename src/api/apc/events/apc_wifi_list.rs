use super::apc_state::ApcStatePayload;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnovaDevice {
    #[serde(rename = "cookerId")]
    pub cooker_id: String,
    pub name: String,
    pub r#type: String,
    #[serde(rename = "pairedAt")]
    pub paired_at: String,
    // not sure about this
    pub apc_state: Option<ApcStatePayload>,
}
