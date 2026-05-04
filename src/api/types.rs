use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Display)]
pub enum TemperatureUnit {
    #[serde(rename = "C")]
    C, // celsius
    #[serde(rename = "F")]
    F, // farenheit
}

pub trait TimeDisplay {
    fn to_display(&self) -> String;
}

impl<T: Into<u64> + Copy> TimeDisplay for T {
    fn to_display(&self) -> String {
        let s: u64 = (*self).into();

        let hours = s / 3600;
        let minutes = (s % 3600) / 60;
        let seconds = s % 60;

        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}
