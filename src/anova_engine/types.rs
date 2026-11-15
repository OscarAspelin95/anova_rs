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

#[derive(Debug, Validate)]
pub struct AnovaToken {
    #[validate(custom(function = "validate_token"))]
    token: String,
}

impl AnovaToken {
    /// new instance from a provided token.
    pub fn from(cli_token: Option<String>) -> Result<Self, AnovaError> {
        let _ = dotenv::dotenv();

        if let Ok(token) = Self::from_env() {
            return Ok(token);
        }

        if let Some(token) = cli_token {
            return Ok(Self { token });
        }

        Err(AnovaError::EnvError(
            "ANOVA_TOKEN not provided through cli or environment".to_string(),
        ))
    }

    pub fn from_env() -> Result<Self, AnovaError> {
        let token = std::env::var("ANOVA_TOKEN").or(Err(AnovaError::EnvError(
            "env var `ANOVA_TOKEN` not found in .env file.".to_string(),
        )))?;

        // will panic if token is invalid.
        let anova = Self { token };

        Ok(anova)
    }

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_no_token_available() {
        unsafe { std::env::remove_var("ANOVA_TOKEN") };
        let anova_token = AnovaToken::from(None);
        assert!(anova_token.is_err());
    }

    #[test]
    fn test_token_provided() {
        let anova_token = AnovaToken::from(Some("my_anova_token".into()));
        assert!(anova_token.is_ok());

        let inner = anova_token.unwrap().token;
        assert_eq!(inner, "my_anova_token");
    }

    #[test]
    fn test_token_provided_through_env() {
        unsafe { std::env::set_var("ANOVA_TOKEN", "my_anova_token") };

        let anova_token = AnovaToken::from(None);
        assert!(anova_token.is_ok());

        let inner = anova_token.unwrap().token;
        assert_eq!(inner, "my_anova_token");
    }
}
