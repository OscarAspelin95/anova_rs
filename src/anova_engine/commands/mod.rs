pub mod set;
pub mod start;
pub mod stop;

use serde::Serialize;
use strum::{Display, EnumString};
use tabled::{Table, Tabled, settings::Style};

#[derive(Debug, Serialize)]
pub enum ApcCommands {
    #[serde(rename = "CMD_APC_START")]
    CmdApcStart,
    #[serde(rename = "CMD_APC_STOP")]
    CmdApcStop,
    #[serde(rename = "CMD_APC_SET_TEMPERATURE_UNIT")]
    CmdApcSetTemperatureUnit,
}

#[derive(Debug, Serialize, EnumString)]
pub enum Unit {
    C,
    F,
}

#[derive(Debug, Tabled, Display, EnumString)]
pub enum Keyword {
    #[tabled(rename = "start")]
    Start,
    #[tabled(rename = "set")]
    Set,
    #[tabled(rename = "stop")]
    Stop,
    #[tabled(rename = "quit")]
    Quit,
}

#[derive(Debug, Tabled)]
pub struct CliCommand {
    pub keyword: Keyword,
    pub description: String,
}

#[derive(Debug)]
pub struct CliCommands {
    pub commands: Vec<CliCommand>,
}

impl CliCommands {
    pub fn default() -> Self {
        let cmd_start = CliCommand {
            keyword: Keyword::Start,
            description: "Start ANOVA cooking session".into(),
        };
        let cmd_set = CliCommand {
            keyword: Keyword::Set,
            description: "Set ANOVA temperature unit (C/F)".into(),
        };

        let cmd_stop = CliCommand {
            keyword: Keyword::Stop,
            description: "Stop ANOVA cooking session".into(),
        };

        let cmd_quit = CliCommand {
            keyword: Keyword::Quit,
            description: "Quit application".into(),
        };

        Self {
            commands: vec![cmd_start, cmd_set, cmd_stop, cmd_quit],
        }
    }
}

impl CliCommands {
    pub fn to_table(&self) -> String {
        let mut table = Table::builder(&self.commands).build();
        table.with(Style::modern_rounded());

        table.to_string()
    }

    pub fn show(&self) {
        println!("{}", self.to_table())
    }
}
