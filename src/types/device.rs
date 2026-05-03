use serde::{Deserialize, Serialize};

use crate::api::{ApcStatePayload, JobMode, TemperatureUnit};

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

impl AnovaDevice {
    pub fn is_connected(&self) -> bool {
        self.apc_state.is_some()
    }

    /// We can probably make this better with .map(...).
    pub fn is_running(&self) -> bool {
        let apc_state = match &self.apc_state {
            None => return false,
            Some(apc_state) => apc_state,
        };

        apc_state.state.job.mode == JobMode::Cook
    }

    pub fn current_temperature_unit(&self) -> Option<TemperatureUnit> {
        self.apc_state
            .as_ref()
            .map(|apc_state| apc_state.state.job.temperature_unit.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Devices {
    pub current_index: Option<usize>,
    pub next_index: Option<usize>,
    pub devices: Vec<AnovaDevice>,
}

impl Default for Devices {
    fn default() -> Self {
        Self::new()
    }
}

impl Devices {
    pub fn new() -> Self {
        Self {
            current_index: None,
            next_index: None,
            devices: vec![],
        }
    }

    pub fn next_device(&mut self) {
        let next_index = match self.next_index {
            None => return,
            Some(next_index) => next_index,
        };

        self.next_index = Some((next_index + 1).min(self.devices.len() - 1))
    }

    pub fn previous_device(&mut self) {
        let next_index = match self.next_index {
            None => return,
            Some(next_index) => next_index,
        };

        self.next_index = Some(next_index.saturating_sub(1));
    }

    pub fn update_device(&mut self) {
        match (self.current_index, self.next_index) {
            // no currently chosen device
            (None, Some(next_index)) => self.current_index = Some(next_index),

            // check if update or unset.
            (Some(current_index), Some(next_index)) => match current_index == next_index {
                true => self.current_index = None,
                false => self.current_index = Some(next_index),
            },
            _ => {}
        }
    }

    pub fn current_device(&self) -> Option<&AnovaDevice> {
        match self.current_index {
            None => None,
            Some(current_index) => self.devices.get(current_index),
        }
    }

    pub fn update_devices(&mut self, devices: Vec<AnovaDevice>) {
        if devices.is_empty() {
            return;
        }

        self.devices = devices;
        self.next_index = Some(0);
    }

    pub fn set_apc_state(&mut self, apc_state: ApcStatePayload) {
        if let Some(device) = self
            .devices
            .iter_mut()
            .find(|d| d.cooker_id == apc_state.cooker_id)
        {
            device.apc_state = Some(apc_state);
        }
    }
}
