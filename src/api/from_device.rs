//! incoming raw API payload from the device.

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

use crate::api::TemperatureUnit;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtaInfo {
    pub available: bool,
    pub description: String,
    pub required: bool,
    pub url: String,
    pub version: String,
}

/// not sure if this got lost somewhere...
/// we probably need this.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cooker {
    #[serde(rename = "cookerId")]
    pub cooker_id: String,
    pub ota: OtaInfo,
    #[serde(rename = "type")]
    pub r#type: String,
    pub version: String,
}

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
    pub duty_cycle: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Display, EnumString)]
pub enum JobMode {
    #[serde(rename = "IDLE")]
    Idle,
    #[serde(rename = "COOK")]
    Cook,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Display, EnumString)]
pub enum JobStatusState {
    #[strum(to_string = "-")]
    #[serde(rename = "")]
    Empty,
    #[strum(to_string = "Cooking")]
    #[serde(rename = "COOKING")]
    Cooking,
    #[strum(to_string = "Preheating")]
    #[serde(rename = "PREHEATING")]
    PreHeating,
    #[strum(to_string = "Maintaining")]
    #[serde(rename = "MAINTAINING")]
    Maintaining,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    #[serde(rename = "cook-time-seconds")]
    pub cook_time_seconds: u32,
    pub id: String,
    pub mode: JobMode,
    #[serde(rename = "ota-url")]
    pub ota_url: String,
    #[serde(rename = "target-temperature")]
    pub target_temperature: f64,
    #[serde(rename = "temperature-unit")]
    pub temperature_unit: TemperatureUnit,
}

impl Job {
    pub fn target_temperature_auto(&self) -> f64 {
        match self.temperature_unit {
            TemperatureUnit::C => self.target_temperature,
            TemperatureUnit::F => c_to_f(self.target_temperature),
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
    pub state: JobStatusState,
    #[serde(rename = "state-change-systick")]
    pub state_change_systick: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotorInfo {
    pub rpm: f64,
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
    pub heater_temperature: f64, // always celsius
    #[serde(rename = "triac-temperature")]
    pub triac_temperature: f64, // always celsius
    #[serde(rename = "water-temperature")]
    pub water_temperature: f64, // always celsius
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

// ------------------------------
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

// ---------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureDisplay {
    pub heater_temperature: String,
    pub triac_temperature: String,
    pub water_temperature: String,
}

pub fn c_to_f(c: f64) -> f64 {
    (c * 1.8) + 32.0
}

impl ApcState {
    pub fn temperature_info_auto(&self) -> TemperatureInfo {
        match self.job.temperature_unit {
            TemperatureUnit::C => TemperatureInfo {
                heater_temperature: self.temperature_info.heater_temperature,
                triac_temperature: self.temperature_info.triac_temperature,
                water_temperature: self.temperature_info.water_temperature,
            },
            TemperatureUnit::F => TemperatureInfo {
                heater_temperature: c_to_f(self.temperature_info.heater_temperature),
                triac_temperature: c_to_f(self.temperature_info.triac_temperature),
                water_temperature: c_to_f(self.temperature_info.water_temperature),
            },
        }
    }
}
