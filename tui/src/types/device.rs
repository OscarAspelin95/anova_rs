use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnovaDevice {
    #[serde(rename = "cookerId")]
    pub cooker_id: String,
    pub name: String,
    pub r#type: String,
    #[serde(rename = "pairedAt")]
    pub paired_at: String,
}

impl AnovaDevice {
    pub fn mock_devices() -> Vec<AnovaDevice> {
        vec![
            AnovaDevice {
                cooker_id: "1".into(),
                name: "1".into(),
                r#type: "1".into(),
                paired_at: "1".into(),
            },
            AnovaDevice {
                cooker_id: "1".into(),
                name: "1".into(),
                r#type: "1".into(),
                paired_at: "1".into(),
            },
        ]
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Devices {
    pub current_index: Option<usize>,
    pub next_index: Option<usize>,
    pub devices: Vec<AnovaDevice>,
}

impl Devices {
    pub fn mock() -> Self {
        Self {
            current_index: None,
            next_index: Some(0),
            devices: AnovaDevice::mock_devices(),
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

    pub fn current_device<'a>(&'a self) -> Option<&'a AnovaDevice> {
        match self.current_index {
            None => None,
            Some(current_index) => self.devices.get(current_index),
        }
    }
}
