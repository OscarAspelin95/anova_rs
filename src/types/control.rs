use serde::Serialize;
use strum::EnumIter;

#[derive(Debug, Clone, EnumIter, Serialize)]
pub enum ControlType {
    Start,
    Set,
    Stop,
}
