use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum TemperatureUnit {
    #[serde(rename = "C")]
    C, // celsius
    #[serde(rename = "F")]
    F, // farenheit
}
