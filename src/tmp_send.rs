use crate::errors::AnovaError;
use serde::Serialize;
use serde_json;
use std::{io::Write, str::FromStr};
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
use log::{error, info, warn};
use std::time::Duration;

use tokio::{
    self,
    io::{AsyncBufReadExt, BufReader},
};

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

/// parse user input from stdin to a specific type.
async fn get_value_from_user<T: FromStr>(input_str: &str) -> T {
    let mut s = BufReader::new(tokio::io::stdin());
    let mut buf = String::new();

    let value = loop {
        print!("\nPlease input {}: ", input_str);
        std::io::stdout().flush().unwrap();

        buf.clear();
        match s.read_line(&mut buf).await {
            Ok(size) if size > 0 => {}
            unexpected => {
                warn!("{:?}", unexpected);
                continue;
            }
        }

        match buf.trim().parse::<T>() {
            Ok(timer) => break timer,
            Err(_) => {
                println!("invalid value `{}`", buf);
                continue;
            }
        }
    };

    value
}

pub async fn send_start(device: &AnovaDevice, writer: &mut Writer) -> Result<(), AnovaError> {
    let timer: u64 = get_value_from_user("time (seconds)").await;
    let temperature: f64 = get_value_from_user("temperature (celsius)").await;

    let cmd = ApcStart {
        command: ApcCommands::CmdApcStart,
        request_id: uuid::Uuid::new_v4(),
        payload: ApcStartPayload {
            cooker_id: device.cooker_id.clone(),
            r#type: device.r#type.clone(),
            target_temperature: temperature,
            unit: Unit::C,
            timer: timer,
        },
    };

    send(cmd, writer).await?;
    Ok(())
}

pub async fn send_set(device: &AnovaDevice, writer: &mut Writer) -> Result<(), AnovaError> {
    let unit: Unit = get_value_from_user("temperature unit (C/F)").await;

    let cmd = ApcSet {
        command: ApcCommands::CmdApcSetTemperatureUnit,
        request_id: uuid::Uuid::new_v4(),
        payload: ApcSetPayload {
            cooker_id: device.cooker_id.clone(),
            r#type: device.r#type.clone(),
            unit: unit,
        },
    };

    send(cmd, writer).await?;
    Ok(())
}

pub async fn send_stop(device: &AnovaDevice, writer: &mut Writer) -> Result<(), AnovaError> {
    let cmd = ApcStop {
        command: ApcCommands::CmdApcStop,
        request_id: uuid::Uuid::new_v4(),
        payload: ApcStopPayload {
            cooker_id: device.cooker_id.clone(),
            r#type: device.r#type.clone(),
        },
    };

    send(cmd, writer).await?;
    Ok(())
}
