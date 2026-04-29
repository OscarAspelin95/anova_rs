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
    pub apc_state: Option<ApcStatePayloadSimple>,
}

impl AnovaDevice {
    pub fn mock_devices() -> Vec<AnovaDevice> {
        vec![]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Devices {
    pub current_index: Option<usize>,
    pub next_index: Option<usize>,
    pub devices: Vec<AnovaDevice>,
}

impl Devices {
    pub fn mock() -> Self {
        Self {
            current_index: None,
            next_index: None,
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

    pub fn update_devices(&mut self, devices: Vec<AnovaDevice>) {
        if devices.is_empty() {
            return;
        }

        self.devices = devices;
        self.next_index = Some(0);
    }

    pub fn set_apc_state(&mut self, apc_state_simple: ApcStatePayloadSimple) {
        match self
            .devices
            .iter_mut()
            .find(|d| d.cooker_id == apc_state_simple.cooker_id)
        {
            Some(device) => {
                device.apc_state = Some(apc_state_simple);
            }
            None => {}
        }
    }
}

// ------------------------------
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

// --------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "hasValidSubscription")]
    pub has_valid_subscription: bool,
    #[serde(rename = "isLegacyAccount")]
    pub is_legacy_account: bool,
    #[serde(rename = "renewalPeriod")]
    pub renewal_period: String,
    #[serde(rename = "productId")]
    pub product_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatePayload {
    #[serde(rename = "isConnectedToAlexa")]
    pub is_connected_to_alexa: bool,
    #[serde(rename = "isConnectedToGoogleHome")]
    pub is_connected_to_google_home: bool,
    #[serde(rename = "sousVideSubscription")]
    pub sous_vide_subscription: Subscription,
    #[serde(rename = "ovenSubscription")]
    pub oven_subscription: Subscription,
}

// --------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioControl {
    #[serde(rename = "file-name")]
    pub file_name: String,
    pub volume: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapTouch {
    #[serde(rename = "minus-button")]
    pub minus_button: u32,
    #[serde(rename = "play-button")]
    pub play_button: u32,
    #[serde(rename = "plus-button")]
    pub plus_button: u32,
    #[serde(rename = "target-temperature-button")]
    pub target_temperature_button: u32,
    #[serde(rename = "timer-button")]
    pub timer_button: u32,
    #[serde(rename = "water-temperature-button")]
    pub water_temperature_button: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DutyCycle {
    #[serde(rename = "duty-cycle")]
    pub duty_cycle: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    #[serde(rename = "cook-time-seconds")]
    pub cook_time_seconds: u32,
    pub id: String,
    pub mode: String,
    #[serde(rename = "ota-url")]
    pub ota_url: String,
    #[serde(rename = "target-temperature")]
    pub target_temperature: f64,
    #[serde(rename = "temperature-unit")]
    pub temperature_unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobSimple {
    #[serde(rename = "cook-time-seconds")]
    pub cook_time_seconds: u32,
    pub mode: String,
    #[serde(rename = "ota-url")]
    pub target_temperature: f64,
}

impl From<Job> for JobSimple {
    fn from(job: Job) -> Self {
        Self {
            cook_time_seconds: job.cook_time_seconds,
            mode: job.mode,
            target_temperature: job.target_temperature,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobStatus {
    #[serde(rename = "cook-time-remaining")]
    pub cook_time_remaining: u32,
    #[serde(rename = "job-start-systick")]
    pub job_start_systick: u64,
    #[serde(rename = "provisioning-pairing-code")]
    pub provisioning_pairing_code: u32,
    pub state: String,
    #[serde(rename = "state-change-systick")]
    pub state_change_systick: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotorInfo {
    pub rpm: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub bssid: String,
    #[serde(rename = "connection-status")]
    pub connection_status: String,
    #[serde(rename = "is-provisioning")]
    pub is_provisioning: bool,
    #[serde(rename = "mac-address")]
    pub mac_address: String,
    pub mode: String,
    #[serde(rename = "security-type")]
    pub security_type: String,
    pub ssid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinInfo {
    #[serde(rename = "device-safe")]
    pub device_safe: u32,
    #[serde(rename = "motor-stuck")]
    pub motor_stuck: u32,
    #[serde(rename = "water-leak")]
    pub water_leak: u32,
    #[serde(rename = "water-level-critical")]
    pub water_level_critical: u32,
    #[serde(rename = "water-level-low")]
    pub water_level_low: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo2640 {
    #[serde(rename = "firmware-version")]
    pub firmware_version: String,
    #[serde(rename = "firmware-version-sha")]
    pub firmware_version_sha: String,
    #[serde(rename = "largest-free-heap-size")]
    pub largest_free_heap_size: u32,
    #[serde(rename = "mcu-temperature")]
    pub mcu_temperature: f64,
    pub systick: u64,
    #[serde(rename = "total-free-heap-size")]
    pub total_free_heap_size: u32,
    #[serde(rename = "total-heap-size")]
    pub total_heap_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo3220 {
    #[serde(rename = "firmware-version")]
    pub firmware_version: String,
    #[serde(rename = "firmware-version-sha")]
    pub firmware_version_sha: String,
    #[serde(rename = "fwUpgradeStatus")]
    pub fw_upgrade_status: u32,
    #[serde(rename = "has-real-cert-catalog")]
    pub has_real_cert_catalog: bool,
    #[serde(rename = "largest-free-heap-size")]
    pub largest_free_heap_size: u32,
    pub systick: u64,
    #[serde(rename = "total-free-heap-size")]
    pub total_free_heap_size: u32,
    #[serde(rename = "total-heap-size")]
    pub total_heap_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureInfo {
    #[serde(rename = "heater-temperature")]
    pub heater_temperature: f64,
    #[serde(rename = "triac-temperature")]
    pub triac_temperature: f64,
    #[serde(rename = "water-temperature")]
    pub water_temperature: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureInfoSimple {
    pub heater_temperature: f64,
    pub water_temperature: f64,
}

impl From<TemperatureInfo> for TemperatureInfoSimple {
    fn from(temperature_info: TemperatureInfo) -> Self {
        Self {
            heater_temperature: temperature_info.heater_temperature,
            water_temperature: temperature_info.water_temperature,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApcState {
    #[serde(rename = "audio-control")]
    pub audio_control: AudioControl,
    #[serde(rename = "boot-id")]
    pub boot_id: String,
    #[serde(rename = "cap-touch")]
    pub cap_touch: CapTouch,
    #[serde(rename = "heater-control")]
    pub heater_control: DutyCycle,
    pub job: Job,
    #[serde(rename = "job-status")]
    pub job_status: JobStatus,
    #[serde(rename = "motor-control")]
    pub motor_control: DutyCycle,
    #[serde(rename = "motor-info")]
    pub motor_info: MotorInfo,
    #[serde(rename = "network-info")]
    pub network_info: NetworkInfo,
    #[serde(rename = "pin-info")]
    pub pin_info: PinInfo,
    #[serde(rename = "system-info-2640")]
    pub system_info_2640: SystemInfo2640,
    #[serde(rename = "system-info-3220")]
    pub system_info_3220: SystemInfo3220,
    #[serde(rename = "temperature-info")]
    pub temperature_info: TemperatureInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApcStatePayload {
    #[serde(rename = "cookerId")]
    pub cooker_id: String,
    #[serde(rename = "type")]
    pub cooker_type: String,
    pub state: ApcState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApcStatePayloadSimple {
    pub cooker_id: String,
    pub job: JobSimple,
    pub temperature: TemperatureInfoSimple,
}

impl From<ApcStatePayload> for ApcStatePayloadSimple {
    fn from(apc_state_payload: ApcStatePayload) -> Self {
        Self {
            cooker_id: apc_state_payload.cooker_id,
            job: apc_state_payload.state.job.into(),
            temperature: apc_state_payload.state.temperature_info.into(),
        }
    }
}
