mod args;
mod domain;
mod log;
mod tui;
mod utils;
mod watcher;

use anyhow::Context;
use args::{Args, DfftCommand};
use clap::Parser;
use log::setup_logging;
use tui::TuiBehaviours;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.debug {
        print_debug_info(&args);
        return Ok(());
    }

    match &args.command {
        DfftCommand::Run {
            no_follow,
            no_prepopulation,
            no_watch,
        } => {
            setup_logging().context("couldn't set up logging")?;

            let root = tokio::fs::canonicalize(".")
                .await
                .context("couldn't determine directory path")?;

            let behaviours = TuiBehaviours {
                watch: !no_watch,
                follow_changes: !no_follow,
                prepopulate_cache: !no_prepopulation,
            };
            tui::run(root, behaviours).await?;
        }
    };

    Ok(())
}

fn print_debug_info(args: &Args) {
    print!(
        r#"DEBUG INFO:
{args}"#
    )
}
