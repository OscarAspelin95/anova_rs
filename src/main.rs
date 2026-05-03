use crate::{app::App, args::Args};
use clap::Parser;
use std::fs::File;
use std::fs::create_dir_all;
use std::path::Path;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub mod anova_engine;
pub mod api;
pub mod app;
pub mod args;
pub mod event;
pub mod types;
pub mod ui;

fn logging_setup(log_file: &Path) {
    if let Some(parent) = log_file.parent() {
        create_dir_all(parent)
            .unwrap_or_else(|_| panic!("failed to create parent directory {}", parent.display()))
    }

    let f = File::create(log_file)
        .unwrap_or_else(|_| panic!("failed to create log file {}", log_file.display()));

    let file_layer = fmt::layer().with_writer(f).with_ansi(false);
    tracing_subscriber::registry()
        .with(file_layer)
        .with(LevelFilter::INFO)
        .init();
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    logging_setup(&args.log_file);

    let terminal = ratatui::init();
    let result = App::new().run(terminal, args.anova_token).await;
    ratatui::restore();

    result
}
