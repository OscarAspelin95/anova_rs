use serde::{Deserialize, Serialize};
use strum::EnumString;

#[derive(Debug, EnumString, Serialize, Deserialize)]
pub enum AnovaResponseStatus {
    #[strum(to_string = "ok")]
    #[serde(rename = "ok")]
    OK,
    #[serde(rename = "error")]
    #[strum(to_string = "error")]
    ERROR,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnovaResponsePayload {
    pub status: AnovaResponseStatus,
}
