use clap::{Parser, Subcommand};

/// dfft shows you changes to files in a directory as they happen
#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: DfftCommand,
    /// Output debug information without doing anything
    #[arg(long = "debug", global = true)]
    pub debug: bool,
}

#[derive(Subcommand, Debug)]
pub enum DfftCommand {
    /// Run dfft's TUI
    Run {
        /// Start with the setting "follow changes" enabled
        #[arg(short = 'f', long = "follow-changes")]
        follow_changes: bool,
        /// Skip prepopulating cache with file snapshots
        #[arg(long = "no-prepop")]
        no_prepopulation: bool,
        /// Start with file watching disabled
        #[arg(long = "no-watch")]
        no_watch: bool,
    },
}

impl std::fmt::Display for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match &self.command {
            DfftCommand::Run {
                follow_changes,
                no_prepopulation,
                no_watch,
            } => format!(
                r#"
command:            run TUI
follow changes:     {follow_changes}
no prepopulation:   {no_prepopulation}
no watch:           {no_watch}
"#,
            ),
        };

        f.write_str(&output)
    }
}
