use anyhow::Context;
use etcetera::{BaseStrategy, choose_base_strategy};
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

const LOG_ENV_VAR: &str = "DFFT_LOG";

pub(super) fn setup_logging() -> anyhow::Result<()> {
    if std::env::var(LOG_ENV_VAR).map_or(true, |v| v.is_empty()) {
        return Ok(());
    }

    let log_file_path = get_log_file_path().context("couldn't determine log file path")?;

    let log_file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)
        .context("failed to open log file")?;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_env(LOG_ENV_VAR))
        .with_ansi(false)
        .with_writer(log_file)
        .init();

    Ok(())
}

fn get_log_file_path() -> anyhow::Result<PathBuf> {
    let log_dir = get_log_dir()?;
    std::fs::create_dir_all(&log_dir).context("couldn't create log directory")?;

    // TODO: add clean up for long log files
    Ok(log_dir.join("dfft.log"))
}

#[cfg(not(target_os = "windows"))]
fn get_log_dir() -> anyhow::Result<PathBuf> {
    let strategy = choose_base_strategy()?;

    // XDG spec suggests using XDG_STATE_HOME for logs
    // https://specifications.freedesktop.org/basedir-spec/latest/#variables
    let log_dir = strategy
        .state_dir() // this always returns Some on unix, but adding a fallback regardless
        .map(|d| d.join("dfft"))
        .unwrap_or_else(|| strategy.home_dir().join(".dfft"));

    Ok(log_dir)
}

#[cfg(target_os = "windows")]
fn get_log_dir() -> anyhow::Result<PathBuf> {
    let strategy = choose_base_strategy()?;

    let log_dir = strategy.cache_dir().join("dfft");

    Ok(log_dir)
}
