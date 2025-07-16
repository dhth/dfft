mod args;
mod domain;
mod log;
mod notifs;
mod tui;
mod utils;
mod watcher;

use std::path::PathBuf;

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

    match args.command {
        DfftCommand::Run {
            path: maybe_path_str,
            follow_changes,
            no_prepopulation,
            no_watch,
            #[cfg(feature = "sound")]
            no_sound,
        } => {
            setup_logging().context("couldn't set up logging")?;

            let path_str = maybe_path_str.unwrap_or(".".to_string());
            let path = PathBuf::from(&path_str);

            if !path.try_exists().context("couldn't check if path exists")? {
                anyhow::bail!("path doesn't exist: {}", &path_str);
            }

            if !path.is_dir() {
                anyhow::bail!("path is not a directory: {}", &path_str);
            }

            let root = tokio::fs::canonicalize(path)
                .await
                .context("couldn't canonicalize directory path")?;

            let behaviours = TuiBehaviours {
                watch: !no_watch,
                follow_changes,
                prepopulate_cache: !no_prepopulation,
                #[cfg(feature = "sound")]
                play_sound: !no_sound,
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
