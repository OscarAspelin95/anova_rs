use crate::errors::AnovaError;
use crate::schema::device::{AnovaCommand, AnovaDevice, AnovaDevices};
use crate::tmp_send::{send_set, send_start, send_stop};
use crate::types::Anova;

use futures_util::StreamExt;
use log::{info, warn};
use serde_json;
use std::time::Duration;
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
                println!("{:?}", &anova_command.payload);

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

    anova_devices.show();

    // Later on -> input from user.
    // For now, the first device available.
    let device = anova_devices.devices.first().unwrap();

    // test send start cook.
    let _ = send_start(device, &mut writer).await?;
    tokio::time::sleep(Duration::from_secs(3)).await;

    // test send set temp
    let _ = send_set(device, &mut writer).await?;
    tokio::time::sleep(Duration::from_secs(3)).await;

    // test send stop cook.
    let _ = send_stop(device, &mut writer).await?;

    Ok(())
}
