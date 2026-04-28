//! For testing purposes only.
use crate::errors::AnovaError;
use serde::Serialize;
use serde_json;
use tokio_tungstenite::tungstenite::Message;

use crate::{
    commands::{
        ApcCommands, Unit,
        set::{ApcSet, ApcSetPayload},
        start::{ApcStart, ApcStartPayload},
        stop::{ApcStop, ApcStopPayload},
    },
    schema::device::AnovaDevice,
};

use crate::types::Writer;
use futures_util::SinkExt;
use log::{error, info};
use std::time::Duration;

use tokio;

async fn send<T: Serialize>(content: T, writer: &mut Writer) -> Result<(), AnovaError> {
    let bytes = serde_json::to_vec(&content)?;
    let msg = Message::from(bytes);

    match writer.send(msg).await {
        Ok(response) => {
            info!("Success: {:?}", response);
        }
        Err(e) => {
            error!("Error: {:?}", e)
        }
    }

    tokio::time::sleep(Duration::from_secs(3)).await;

    Ok(())
}

pub async fn send_start(device: &AnovaDevice, writer: &mut Writer) -> Result<(), AnovaError> {
    let cmd = ApcStart {
        command: ApcCommands::CMD_APC_START,
        request_id: uuid::Uuid::new_v4(),
        payload: ApcStartPayload {
            cooker_id: device.cooker_id.clone(),
            r#type: device.r#type.clone(),
            target_temperature: 35.0,
            unit: Unit::C,
            timer: 100,
        },
    };

    send(cmd, writer).await?;
    Ok(())
}

pub async fn send_set(device: &AnovaDevice, writer: &mut Writer) -> Result<(), AnovaError> {
    let cmd = ApcSet {
        command: ApcCommands::CMD_APC_SET_TEMPERATURE_UNIT,
        request_id: uuid::Uuid::new_v4(),
        payload: ApcSetPayload {
            cooker_id: device.cooker_id.clone(),
            r#type: device.r#type.clone(),
            unit: Unit::C,
        },
    };

    send(cmd, writer).await?;
    Ok(())
}

pub async fn send_stop(device: &AnovaDevice, writer: &mut Writer) -> Result<(), AnovaError> {
    let cmd = ApcStop {
        command: ApcCommands::CMD_APC_STOP,
        request_id: uuid::Uuid::new_v4(),
        payload: ApcStopPayload {
            cooker_id: device.cooker_id.clone(),
            r#type: device.r#type.clone(),
        },
    };

    send(cmd, writer).await?;
    Ok(())
}
