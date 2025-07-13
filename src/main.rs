mod domain;
mod log;
mod tui;
mod utils;
mod watcher;
use anyhow::Context;
use log::setup_logging;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_logging().context("couldn't set up logging")?;

    let root = tokio::fs::canonicalize(".")
        .await
        .context("couldn't determine directory path")?;
    tui::run(root).await?;

    Ok(())
}
