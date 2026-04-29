use futures_util::StreamExt;
use log::{debug, info};
use serde_json;

use tokio;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;

use crate::anova_engine::schema::device::AnovaCommandType;
use crate::event::{AppEvent, Event};
use crate::types::device::{ApcStatePayload, ApcStatePayloadSimple, Cooker, UserStatePayload};

use super::errors::AnovaError;
use super::schema::device::AnovaCommand;
use super::types::Anova;
use crate::types::AnovaDevice;

pub async fn start(sender: UnboundedSender<Event>) -> Result<JoinHandle<()>, AnovaError> {
    //
    let handle = tokio::spawn(async move {
        info!("loading environment...");
        let _ = dotenv::dotenv().ok();

        info!("creating anova instance...");
        let anova = Anova::from_env().expect("no anova token in .env file.");

        info!("establishing websocket connection...");
        let (writer, mut reader) = anova
            .get_stream()
            .await
            .expect("failed to create websocker stream.");

        // we want to send the writer back to app for sending requests.
        // <implement this here>

        // continuously monitor all incoming messages from the cooker.
        // we need a lot of proper error handling/catching to not break the loop.
        loop {
            // try get message.
            let msg = match reader.next().await {
                Some(Ok(msg)) => msg,
                _ => continue,
            };

            // try parsing as valid api response
            let anova_command = match serde_json::from_slice::<AnovaCommand>(&msg.into_data()) {
                Ok(anova_command) => anova_command,
                _ => continue,
            };

            // parse and dispatch msg type.
            match anova_command.command {
                // visible devices
                AnovaCommandType::EventApcWifiList => {
                    let anova_devices_list =
                        match serde_json::from_value::<Vec<AnovaDevice>>(anova_command.payload) {
                            Ok(v) => v,
                            Err(_) => continue,
                        };

                    let _ = sender.send(Event::App(AppEvent::SetAppDevices(anova_devices_list)));
                }
                // available devices
                AnovaCommandType::EventApcWifiVersion => {
                    let anova_devices_version =
                        match serde_json::from_value::<Vec<Cooker>>(anova_command.payload) {
                            Ok(v) => v,
                            Err(_) => continue,
                        };

                    debug!("{:?}", anova_devices_version);
                }
                // information about user.
                AnovaCommandType::EventUserState => {
                    let user_state =
                        match serde_json::from_value::<UserStatePayload>(anova_command.payload) {
                            Ok(v) => v,
                            Err(_) => continue,
                        };

                    debug!("{:?}", user_state);
                }

                // detailed information about device
                AnovaCommandType::EventApcState => {
                    let apc_state_payload =
                        match serde_json::from_value::<ApcStatePayload>(anova_command.payload) {
                            Ok(v) => v,
                            Err(_) => continue,
                        };

                    let apc_state_payload_simple: ApcStatePayloadSimple = apc_state_payload.into();

                    // send
                    let _ =
                        sender.send(Event::App(AppEvent::SetApcState(apc_state_payload_simple)));
                }
            };
        }
    });

    Ok(handle)
}
