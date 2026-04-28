pub mod set;
pub mod start;
pub mod stop;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ApcCommands {
    CMD_APC_START,
    CMD_APC_STOP,
    CMD_APC_SET_TEMPERATURE_UNIT,
}

#[derive(Debug, Serialize)]
pub enum Unit {
    C,
    F,
}
