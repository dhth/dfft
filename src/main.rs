mod changes;
mod domain;
mod tui;
mod utils;
use anyhow::Context;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let root = tokio::fs::canonicalize(".")
        .await
        .context("couldn't determine directory path")?;
    tui::run(root).await?;

    Ok(())
}
