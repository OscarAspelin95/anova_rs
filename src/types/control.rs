use validator::Validate;

use crate::api::{TemperatureUnit, c_to_f};

#[derive(Debug, Validate)]
pub struct DeviceControl {
    // not sure what the max temperature is.
    #[validate(range(min = 30.0, max = 90.0))]
    pub set_temperature: f64,

    // 1s -> 10hrs.
    #[validate(range(min = 1, max = 36_000))]
    pub set_timer: u64,
}

impl DeviceControl {
    pub fn increment_temperature(&mut self, delta: f64) {
        self.set_temperature = (self.set_temperature + delta).min(90.0)
    }

    pub fn decrement_temperature(&mut self, delta: f64) {
        self.set_temperature = (self.set_temperature - delta).max(30.0)
    }

    pub fn increment_timer(&mut self, delta: u64) {
        self.set_timer = (self.set_timer + delta).min(10 * 60 * 60)
    }

    pub fn decrement_timer(&mut self, delta: u64) {
        self.set_timer = self.set_timer.saturating_sub(delta)
    }
}

impl Default for DeviceControl {
    fn default() -> Self {
        Self {
            set_temperature: 50.0,
            set_timer: 30 * 60, // 30 minutes
        }
    }
}

impl DeviceControl {
    pub fn set_temperature_auto(&self, unit: &TemperatureUnit) -> f64 {
        match unit {
            &TemperatureUnit::C => self.set_temperature,
            &TemperatureUnit::F => c_to_f(self.set_temperature),
        }
    }
}
