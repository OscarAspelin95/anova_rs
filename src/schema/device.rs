use serde::{Deserialize, Serialize};
use serde_json::Value;
use tabled::{Table, Tabled, settings::Style};

/// Docstring
/// For now, we only deal with APC.
#[derive(Debug)]
pub enum DeviceType {
    APC, // Anova Precision Cooker
    APO, // Anova Precision Oven
}

/// Information about a given APC device.
#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct AnovaDevice {
    #[serde(rename = "cookerId")]
    pub cooker_id: String,
    #[serde(rename = "type")]
    #[tabled(rename = "type")]
    pub r#type: String,
    #[serde(default)]
    pub name: String,
    #[serde(default, rename = "pairedAt")]
    pub paired_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnovaDevices {
    pub devices: Vec<AnovaDevice>,
}

impl AnovaDevices {
    fn to_table(&self) -> String {
        let mut table = Table::builder(&self.devices).build();
        table.with(Style::modern_rounded());

        table.to_string()
    }

    pub fn show(&self) {
        println!("{}", self.to_table())
    }
}

/// Docstring
#[derive(Debug, Serialize, Deserialize)]
pub struct AnovaApcWifiListResponse {
    pub payload: Vec<AnovaDevice>,
}

/// Docstring
#[derive(Debug, Serialize, Deserialize)]
pub struct AnovaCommand {
    pub command: AnovaCommandType,
    pub payload: Value,
}

impl AnovaCommand {
    pub fn is_apc_wifi_list_response(&self) -> bool {
        return self.command == AnovaCommandType::EventApcWifiList;
    }
}

/// Docstring
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AnovaCommandType {
    #[serde(rename = "EVENT_APC_WIFI_LIST")]
    EventApcWifiList,
    #[serde(rename = "EVENT_APO_WIFI_LIST")]
    EventApoWifiList,
    #[serde(rename = "EVENT_APC_WIFI_VERSION")]
    EventApcWifiVersion,
}
