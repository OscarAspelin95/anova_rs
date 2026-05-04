use validator::Validate;

use crate::api::Celsius;

#[derive(Debug, Validate)]
pub struct DeviceControl {
    // not sure what the max temperature is.
    pub set_temperature: Celsius,

    // 1s -> 10hrs.
    #[validate(range(min = 1, max = 36_000))]
    pub set_timer: u64,
}

impl DeviceControl {
    pub fn increment_temperature(&mut self, delta: f64) {
        self.set_temperature.increment(delta);
    }

    pub fn decrement_temperature(&mut self, delta: f64) {
        self.set_temperature.decrement(delta);
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
            set_temperature: Celsius::new(50.0),
            set_timer: 30 * 60, // 30 minutes
        }
    }
}
