use futures::{SinkExt, StreamExt};
use serde_json;

use tokio;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, error, info};

use crate::anova_engine::schema::device::AnovaCommandType;
use crate::api::{
    AnovaResponsePayload, ApcStatePayload, ApcStatePayloadSimple, Cooker, UserStatePayload,
};
use crate::api::{ApcSet, ApcStart, ApcStop, ApiRequest};
use crate::event::{AppEvent, Event};

use super::errors::AnovaError;
use super::schema::device::AnovaCommand;
use super::types::Anova;
use crate::types::AnovaDevice;

/// Main Anova driver. Triggers a background task for continuously:
/// * fetching data from the API.
/// * sending data to the API.
///
/// args:
/// sender - send API responses to the App.
/// receiver - send API requests from the App.
pub async fn start(
    sender: UnboundedSender<Event>,
    mut receiver: UnboundedReceiver<ApiRequest>,
) -> Result<JoinHandle<()>, AnovaError> {
    //

    let handle = tokio::spawn(async move {
        info!("loading environment...");
        let _ = dotenv::dotenv().ok();

        info!("creating anova instance...");
        let anova = Anova::from_env().expect("no anova token in .env file.");

        info!("establishing websocket connection...");
        let (mut writer, mut reader) = anova
            .get_stream()
            .await
            .expect("failed to create websocker stream.");

        // we want to send the writer back to app for sending requests.
        // <implement this here>

        // continuously monitor all incoming messages from the cooker.
        // we need a lot of proper error handling/catching to not break the loop.
        loop {
            tokio::select! {
                // App -> Engine -> API
                Some(api_request) = receiver.recv() => {


                // We need to clean this up.
                match api_request{
                    ApiRequest::Start(apc_start_payload) => {
                        // fix, this is ugly
                        let apc_start: ApcStart = apc_start_payload.into();
                        debug!("APC_START - {:?}", apc_start);

                        let msg = match serde_json::to_vec(&apc_start){
                            Ok(msg) => msg,
                            Err(e) => {error!("{e}"); continue}
                        };

                        info!("sending APC_START to device {:?}", apc_start);
                        match writer.send(Message::from(&msg[..])).await{
                            Ok(_) => {},
                            Err(e) => error!("{e}")
                        };

                    },
                     ApiRequest::Set(apc_set_payload) => {
                        // fix, this is ugly
                        let apc_set: ApcSet = apc_set_payload.into();
                        debug!("APC_SET - {:?}", apc_set);

                        let msg = match serde_json::to_vec(&apc_set){
                            Ok(msg) => msg,
                             Err(e) => {error!("{e}"); continue}
                        };

                        info!("sending APC_SET to device {:?}", apc_set);
                        match writer.send(Message::from(&msg[..])).await{
                            Ok(_) => {},
                             Err(e) => error!("{e}")
                        };
                     },
                      ApiRequest::Stop(apc_stop_payload) => {
                         // fix, this is ugly
                         let apc_stop: ApcStop = apc_stop_payload.into();
                         let msg = match serde_json::to_vec(&apc_stop){
                             Ok(msg) => msg,
                              Err(e) => {error!("{e}"); continue}
                         };

                         info!("sending APC_STOP to device {:?}", apc_stop);
                         match writer.send(Message::from(&msg[..])).await{
                             Ok(_) => {},
                              Err(e) => error!("{e}")
                         };
                      }
                }
            },
                // API -> Engine -> App
                Some(Ok(msg)) = reader.next() => {

                // try parsing as valid api response
                let msg_clone = msg.clone();
                let anova_command = match serde_json::from_slice::<AnovaCommand>(&msg.into_data()) {
                    Ok(anova_command) => anova_command,
                    Err(e) => {error!("\n{e}"); error!("{:?}\n", msg_clone); continue},
                };

                // parse and dispatch msg type.
                match anova_command.command {
                // visible devices
                c @ AnovaCommandType::Response => {
                    let anova_response = match serde_json::from_value::<AnovaResponsePayload>(anova_command.payload){
                         Ok(v) => v,
                         Err(e) => {error!("{e}"); continue},
                        };

                    debug!("{} - {:?}", c, anova_response);
                }
                    // visible devices
                    c @ AnovaCommandType::EventApcWifiList => {
                        let anova_devices_list =
                            match serde_json::from_value::<Vec<AnovaDevice>>(anova_command.payload) {
                                Ok(v) => v,
                                Err(e) => {error!("{e}"); continue},
                            };

                        debug!("{} - {:?}", c, anova_devices_list);
                        match sender.send(Event::App(AppEvent::SetAppDevices(anova_devices_list))){
                            Ok(_) => {},
                            Err(e) => error!("{e}")
                        }
                    }
                    // available devices
                    c @ AnovaCommandType::EventApcWifiVersion => {
                        let anova_devices_version =
                            match serde_json::from_value::<Vec<Cooker>>(anova_command.payload) {
                                Ok(v) => v,
                                Err(e) => {error!("{e}"); continue},
                            };

                        debug!("{} - {:?}", c, anova_devices_version);
                        // we don't need this info (yet)

                    }
                    // information about user.
                    c @ AnovaCommandType::EventUserState => {
                        let user_state =
                            match serde_json::from_value::<UserStatePayload>(anova_command.payload) {
                                Ok(v) => v,
                                Err(e) => {error!("{e}"); continue},
                            };

                        debug!("{} - {:?}", c, user_state);
                        // we don't need this info (yet)
                    }

                    // detailed information about device
                    c @ AnovaCommandType::EventApcState => {
                        let apc_state_payload =
                            match serde_json::from_value::<ApcStatePayload>(anova_command.payload) {
                                Ok(v) => v,
                                Err(e) => {error!("{e}"); continue},
                            };

                        let apc_state_payload_simple: ApcStatePayloadSimple = apc_state_payload.into();

                        debug!("{} - {:?}", c, apc_state_payload_simple);
                        match sender.send(Event::App(AppEvent::SetApcState(apc_state_payload_simple))){
                            Ok(_) => {},
                            Err(e) => error!("{e}")
                        }
                    }
                };                },
            }
        }
    });

    Ok(handle)
}
