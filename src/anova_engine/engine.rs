use futures::{SinkExt, StreamExt};
use serde_json;

use tokio;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;
use tokio_tungstenite::tungstenite::Message;
use tracing::instrument;
use tracing::{debug, error, info};

use crate::api::apc::events::apc_state::ApcStatePayload;
use crate::api::apc::events::apc_wifi_list::AnovaDevice;
use crate::api::apc::events::apc_wifi_version::Cooker;
use crate::api::apc::events::response::AnovaResponsePayload;
use crate::api::apc::events::user_state::UserStatePayload;
use crate::api::apc::types::AnovaCommandType;
use crate::api::{ApcSet, ApcStart, ApcStop, ApiRequest};
use crate::event::{AppEvent, Event};

use super::errors::AnovaError;
use super::schema::device::AnovaCommand;
use super::types::AnovaToken;

/// Main Anova driver. Triggers a background task for continuously:
/// * fetching data from the API.
/// * sending data to the API.
///
/// args:
/// sender - send API responses to the App.
/// receiver - send API requests from the App.
/// anova_token - optional anova token (otherwise, tries reading from ENV).
#[instrument]
pub async fn start(
    sender: UnboundedSender<Event>,
    mut receiver: UnboundedReceiver<ApiRequest>,
    anova_token: AnovaToken,
) -> Result<JoinHandle<()>, AnovaError> {
    info!("starting anova engine background task...");

    let handle = tokio::spawn(async move {
        info!("establishing websocket connection...");
        let (mut writer, mut reader) = anova_token
            .get_stream()
            .await
            .expect("failed to create websocket stream.");

        info!("listening for messages...");
        loop {
            tokio::select! {
                // App -> Engine -> API
                Some(api_request) = receiver.recv() => {

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
                            Ok(_) => debug!("successfully sent."),
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
                            Ok(_) => debug!("successfully sent."),
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
                             Ok(_) => debug!("successfully sent."),
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

                        debug!("sending {} - {:?}", c, anova_devices_list);
                        match sender.send(Event::App(AppEvent::SetAppDevices(Box::new(anova_devices_list)))){
                            Ok(_) => debug!("successfully sent."),
                            Err(e) => error!("{e}")
                        }
                    }
                    // available devices (currently unused).
                    c @ AnovaCommandType::EventApcWifiVersion => {
                        let anova_devices_version =
                            match serde_json::from_value::<Vec<Cooker>>(anova_command.payload) {
                                Ok(v) => v,
                                Err(e) => {error!("{e}"); continue},
                            };

                        debug!("{} - {:?}", c, anova_devices_version);
                    }
                    // information about user (current unused).
                    c @ AnovaCommandType::EventUserState => {
                        let user_state =
                            match serde_json::from_value::<UserStatePayload>(anova_command.payload) {
                                Ok(v) => v,
                                Err(e) => {error!("{e}"); continue},
                            };

                        debug!("{} - {:?}", c, user_state);
                    }

                    // detailed information about device
                    c @ AnovaCommandType::EventApcState => {
                        let apc_state_payload =
                            match serde_json::from_value::<ApcStatePayload>(anova_command.payload) {
                                Ok(v) => v,
                                Err(e) => {error!("{e}"); continue},
                            };

                        debug!("sending {} - {:?}", c, apc_state_payload);
                        match sender.send(Event::App(AppEvent::SetApcState(Box::new(apc_state_payload)))){
                            Ok(_) => debug!("successfully sent."),
                            Err(e) => error!("{e}")
                        }
                    }
                };                },
            }
        }
    });

    Ok(handle)
}
