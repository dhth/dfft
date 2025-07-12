use anyhow::Context;
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

pub(super) fn setup_logging() -> anyhow::Result<()> {
    let log_file_path = get_log_file_path().context("couldn't determine log file path")?;

    let log_file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)
        .context("failed to open log file")?;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_env("DFFT_LOG"))
        .with_ansi(false)
        .with_writer(log_file)
        .init();

    Ok(())
}

fn get_log_file_path() -> anyhow::Result<PathBuf> {
    let cache_dir =
        dirs::cache_dir().ok_or_else(|| anyhow::anyhow!("couldn't determine cache directory"))?;

    let app_cache_dir = cache_dir.join("dfft");
    std::fs::create_dir_all(&app_cache_dir).context("couldn't create cache directory")?;

    Ok(app_cache_dir.join("dfft.log"))
}
