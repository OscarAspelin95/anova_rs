use futures_util::StreamExt;
use log::{info, warn};
use serde_json;
use std::time::Duration;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;
use tokio::{
    self,
    sync::mpsc::{self, UnboundedReceiver},
};
use tokio_tungstenite::tungstenite::Message;

use crate::event::{AppEvent, Event};

use super::errors::AnovaError;
use super::schema::device::{AnovaCommand, AnovaDevices};
use super::types::Anova;
use super::types::{Reader, Writer};
use crate::types::AnovaDevice;

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

async fn start_background_task(tx: UnboundedSender<Message>, mut reader: Reader) -> JoinHandle<()> {
    let handle = tokio::spawn(async move {
        while let Some(msg) = reader.next().await {
            let msg = match msg {
                Ok(msg) => msg,
                Err(e) => {
                    warn!("{:?}", e);
                    continue;
                }
            };

            match tx.send(msg) {
                Ok(_) => {}
                Err(e) => {
                    warn!("{:?}", e)
                }
            }
        }
    });

    handle
}

pub async fn start(sender: UnboundedSender<Event>) -> Result<JoinHandle<()>, AnovaError> {
    //
    let handle = tokio::spawn(async move {
        let _ = dotenv::dotenv().ok();

        let anova = Anova::from_env().expect("no anova token in .env file.");

        info!("establishing connection...");
        let (mut writer, reader) = anova
            .get_stream()
            .await
            .expect("failed to create websocker stream.");
        let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

        info!("Starting background task...");
        let bg_handle = start_background_task(tx.clone(), reader).await;

        info!("checking available devices...");
        let anova_devices = get_devices(Duration::from_secs(30), &mut rx)
            .await
            .expect("");

        sender
            .send(Event::App(AppEvent::SetAppDevices(anova_devices.devices)))
            .expect("");

        bg_handle.abort();
    });

    Ok(handle)
}
