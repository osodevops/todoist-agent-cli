mod commands;

use anyhow::Result;
use clap::Parser;
use td_cli::cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                if cli.global.verbose {
                    "td_api=debug,td_cache=debug,td_cli=debug".into()
                } else {
                    "warn".into()
                }
            }),
        )
        .with_writer(std::io::stderr)
        .init();

    commands::execute(cli).await
}
