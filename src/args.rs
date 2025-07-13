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
        /// Start with change following disabled
        #[arg(short = 'F', long = "no-follow")]
        no_follow: bool,
        /// Skip prepopulating cache with existing file snapshots
        #[arg(short = 'P', long = "no-prepopulation")]
        no_prepopulation: bool,
        /// Start with file watching disabled
        #[arg(short = 'W', long = "no-watch")]
        no_watch: bool,
    },
}

impl std::fmt::Display for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self.command {
            DfftCommand::Run {
                no_follow,
                no_prepopulation,
                no_watch,
            } => format!(
                r#"
Command:                                                  Run TUI
Start with change following disabled:                     {no_follow}
Skip prepopulating cache with existing file snapshots:    {no_prepopulation}
Start with file watching disabled:                        {no_watch}
"#,
            ),
        };

        f.write_str(&output)
    }
}
