use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Display)]
pub enum TemperatureUnit {
    #[serde(rename = "C")]
    C, // celsius
    #[serde(rename = "F")]
    F, // farenheit
}

impl TemperatureUnit {
    pub fn to_display(&self) -> String {
        format!("°{}", self)
    }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Celsius(pub f64);

impl Celsius {
    pub fn new(temp: f64) -> Self {
        // temp solution. We could return Result<Self, AnovaError>
        // but not sure this is the way to go.
        if !(30.0..90.0).contains(&temp) {
            panic!("start temp must be >=30.0 and <= 90. Got `{}`", temp)
        }

        Self(temp)
    }

    pub fn to_f64(&self, unit: &TemperatureUnit) -> f64 {
        match unit {
            TemperatureUnit::C => self.0,
            TemperatureUnit::F => (self.0 * 1.8) + 32.0,
        }
    }

    pub fn to_display(&self, unit: &TemperatureUnit) -> String {
        format!("{:.1} °{}", self.to_f64(unit), unit)
    }
}

impl Celsius {
    pub fn increment(&mut self, delta: f64) {
        self.0 = (self.0 + delta).min(90.0)
    }

    pub fn decrement(&mut self, delta: f64) {
        self.0 = (self.0 - delta).max(30.0)
    }
}
