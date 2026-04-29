mod commands;
mod consts;
mod engine;
mod errors;
mod schema;
mod tmp_send;
mod types;

use errors::AnovaError;
use simple_logger::SimpleLogger;

use crate::types::Anova;

fn init() -> Result<(), AnovaError> {
    if dotenv::dotenv().is_err() {
        return Err(AnovaError::EnvError(".env file missing".into()));
    };

    if SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .is_err()
    {
        return Err(AnovaError::LogError("failed to set up logging".into()));
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), AnovaError> {
    init()?;

    let anova = Anova::from_env()?;
    let _ = engine::run(anova).await?;

    Ok(())
}
