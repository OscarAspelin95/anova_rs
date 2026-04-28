use crate::commands::{CliCommand, CliCommands, Keyword};
use crate::errors::AnovaError;
use crate::schema::device::{AnovaCommand, AnovaDevice, AnovaDevices};
use crate::tmp_send::{send_set, send_start, send_stop};
use crate::types::Anova;

use futures_util::StreamExt;
use log::{info, warn};
use serde_json;
use std::io::Write;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::{
    self,
    sync::mpsc::{self, UnboundedReceiver},
};
use tokio_tungstenite::tungstenite::Message;

async fn get_devices(
    duration: std::time::Duration,
    rx: &mut UnboundedReceiver<Message>,
) -> Result<AnovaDevices, AnovaError> {
    let timeout_result = tokio::time::timeout(duration, async {
        while let Some(msg) = rx.recv().await {
            let msg_bytes = msg.into_data();

            // must be valid anova cmd.
            let anova_command = serde_json::from_slice::<AnovaCommand>(&msg_bytes)?;

            // only consider APC for now.
            if anova_command.is_apc_wifi_list_response() {
                let response = serde_json::from_value::<Vec<AnovaDevice>>(anova_command.payload)?;

                return Ok(AnovaDevices { devices: response });
            }
        }

        Err(AnovaError::TimeoutError("failed to find devices".into()))
    })
    .await;

    // Handle the Timeout (Elapsed error) and the internal Result
    match timeout_result {
        Ok(inner_result) => inner_result,
        Err(e) => Err(AnovaError::TimeoutError(e.to_string())),
    }
}

async fn get_device_from_user<'a>(anova_devices: &'a AnovaDevices) -> &'a AnovaDevice {
    let mut s = BufReader::new(tokio::io::stdin());
    let mut buf = String::new();

    let device = loop {
        println!("\nPlease select device ID:");
        anova_devices.show();
        print!("device ID: ");
        std::io::stdout().flush().unwrap();

        buf.clear();
        match s.read_line(&mut buf).await {
            Ok(size) if size > 0 => {}
            unexpected => {
                warn!("{:?}", unexpected);
                continue;
            }
        }

        match anova_devices
            .devices
            .iter()
            .find(|d| d.cooker_id == buf.trim())
        {
            Some(device) => break device,
            None => {
                println!("invalid device ID");
                continue;
            }
        }
    };

    device
}

async fn get_action_from_user<'a>(cli_cmds: &'a CliCommands) -> &'a CliCommand {
    let mut s = BufReader::new(tokio::io::stdin());
    let mut buf = String::new();

    let action = loop {
        println!("\nPlease select action (keyword):");
        cli_cmds.show();
        print!("action: ");
        std::io::stdout().flush().unwrap();

        buf.clear();
        match s.read_line(&mut buf).await {
            Ok(size) if size > 0 => {}
            unexpected => {
                warn!("{:?}", unexpected);
                continue;
            }
        }

        match cli_cmds
            .commands
            .iter()
            .find(|c| c.keyword.to_string().to_lowercase() == buf.trim().to_lowercase())
        {
            Some(action) => break action.to_owned(),
            None => {
                println!("invalid keyword `{}`", buf);
                continue;
            }
        }
    };

    action
}
pub async fn run(anova: Anova) -> Result<(), AnovaError> {
    info!("establishing connection...");
    let (mut writer, mut reader) = anova.get_stream().await?;

    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

    // background task for acknowledging messages.
    let txc = tx.clone();
    tokio::spawn(async move {
        while let Some(msg) = reader.next().await {
            let msg = match msg {
                Ok(msg) => msg,
                Err(e) => {
                    warn!("{:?}", e);
                    continue;
                }
            };

            match txc.send(msg) {
                Ok(_) => {}
                Err(e) => {
                    warn!("{:?}", e)
                }
            }
        }
    });

    info!("checking devices...");
    let anova_devices = get_devices(Duration::from_secs(30), &mut rx).await?;

    info!("waiting for user input...");
    let device = get_device_from_user(&anova_devices).await;

    let cli_cmds = CliCommands::default();
    loop {
        info!("waiting for user input...");
        let action = get_action_from_user(&cli_cmds).await;

        match action.keyword {
            Keyword::Start => {
                let _ = send_start(device, &mut writer).await?;
            }
            Keyword::Set => {
                let _ = send_set(device, &mut writer).await?;
            }
            Keyword::Stop => {
                let _ = send_stop(device, &mut writer).await?;
            }
            Keyword::Quit => {
                break;
            }
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}
