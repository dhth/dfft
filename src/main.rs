#![allow(dead_code, unused)]
mod changes;
mod diff;
mod domain;
mod tui;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    tui::run().await?;

    Ok(())
}
