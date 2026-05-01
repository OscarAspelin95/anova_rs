//! small interface for converting the raw api payload from device to
//! a more simplified version we'll store the the app state.

use serde::{Deserialize, Serialize};

use super::from_device::{
    ApcState, ApcStatePayload, Job, JobStatus, NetworkInfo, PinInfo, TemperatureInfo,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApcStateSimple {
    pub job: Job,
    pub job_status: JobStatus,
    pub network_info: NetworkInfo,
    pub pin_info: PinInfo,
    pub temperature_info: TemperatureInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApcStatePayloadSimple {
    pub cooker_id: String,
    pub cooker_type: String,
    pub state: ApcStateSimple,
}

impl From<ApcState> for ApcStateSimple {
    fn from(apc_state: ApcState) -> Self {
        Self {
            job: apc_state.job,
            job_status: apc_state.job_status,
            network_info: apc_state.network_info,
            pin_info: apc_state.pin_info,
            temperature_info: apc_state.temperature_info,
        }
    }
}

impl From<ApcStatePayload> for ApcStatePayloadSimple {
    fn from(apc_state_payload: ApcStatePayload) -> Self {
        Self {
            cooker_id: apc_state_payload.cooker_id,
            cooker_type: apc_state_payload.cooker_type,
            state: apc_state_payload.state.into(),
        }
    }
}
