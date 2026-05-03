use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(
        short,
        long,
        default_value = "log.txt",
        help = "where to output log file."
    )]
    pub log_file: PathBuf,

    #[arg(short, long, required = false, help = "anova token (optional)")]
    pub anova_token: Option<String>,
}
