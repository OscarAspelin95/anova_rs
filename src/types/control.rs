use strum::{Display, EnumIter};

#[derive(Debug, EnumIter)]
pub enum DeviceControl {
    TargetTemperature(f64),
    Timer(u64),
}
