use thiserror::Error;
use url;

#[derive(Debug, Error)]
pub enum AnovaError {
    #[error("Environment Error: {0}")]
    EnvError(String),

    #[error("Invalid Token: {0}")]
    TokenError(String),

    #[error("Log Error: {0}")]
    LogError(String),

    #[error(transparent)]
    UrlError(#[from] url::ParseError),

    #[error(transparent)]
    WebSocketError(#[from] tokio_tungstenite::tungstenite::Error),

    #[error(transparent)]
    DeserializationError(#[from] serde_json::Error),

    #[error("")]
    TimeoutError(String),
}
