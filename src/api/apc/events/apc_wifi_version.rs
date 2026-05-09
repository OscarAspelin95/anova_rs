use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtaInfo {
    pub available: bool,
    pub description: String,
    pub required: bool,
    pub url: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cooker {
    #[serde(rename = "cookerId")]
    pub cooker_id: String,
    pub ota: OtaInfo,
    #[serde(rename = "type")]
    pub r#type: String,
    pub version: String,
}
