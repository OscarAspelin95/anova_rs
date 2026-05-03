use super::consts::TOKEN_PREFIX;
use super::errors::AnovaError;
use futures_util::StreamExt;
use futures_util::stream::{SplitSink, SplitStream};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::Message};
use url::Url;
use validator::{Validate, ValidationError};

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
pub type Reader = SplitStream<WsStream>;
pub type Writer = SplitSink<WsStream, Message>;

fn validate_token(token: &str) -> Result<(), ValidationError> {
    if !token.starts_with(TOKEN_PREFIX) {
        return Err(ValidationError::new("Invalid token prefix"));
    }

    Ok(())
}

/// Main struct for establishing a websocket connection.
#[derive(Debug, Validate)]
pub struct Anova {
    #[validate(custom(function = "validate_token"))]
    token: String,
}

impl Anova {
    /// new instance from a provided token.
    pub fn new(token: String) -> Self {
        Self { token }
    }

    /// try reading token automatically from .env file.
    pub fn from_env() -> Result<Self, AnovaError> {
        let token = std::env::var("ANOVA_TOKEN").or(Err(AnovaError::EnvError(
            "env var `ANOVA_TOKEN` not found in .env file.".to_string(),
        )))?;

        // will panic if token is invalid.
        let anova = Self { token };

        Ok(anova)
    }

    /// We can prettify this later on.
    pub fn url(&self) -> Result<Url, AnovaError> {
        let url = Url::parse(&format!(
            "wss://devices.anovaculinary.io?token={}&supportedAccessories=APC",
            self.token
        ))?;

        Ok(url)
    }

    pub async fn get_stream(&self) -> Result<(Writer, Reader), AnovaError> {
        let (stream, _) = connect_async(self.url()?.to_string()).await?;
        let (writer, reader) = stream.split();

        Ok((writer, reader))
    }
}
