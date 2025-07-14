use clap::{Parser, Subcommand};

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
        /// Start with sound notifications disabled
        #[cfg(feature = "sound")]
        #[arg(long = "no-sound")]
        no_sound: bool,
    },
}

impl std::fmt::Display for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match &self.command {
            DfftCommand::Run {
                follow_changes,
                no_prepopulation,
                no_watch,
                #[cfg(feature = "sound")]
                no_sound,
            } => {
                #[cfg(feature = "sound")]
                let output = format!(
                    r#"
command:            run TUI
follow changes:     {follow_changes}
no prepopulation:   {no_prepopulation}
no watch:           {no_watch}
no sound:           {no_sound}
"#,
                );
                #[cfg(not(feature = "sound"))]
                let output = format!(
                    r#"
command:            run TUI
follow changes:     {follow_changes}
no prepopulation:   {no_prepopulation}
no watch:           {no_watch}
"#,
                );
                output
            }
        };

        f.write_str(&output)
    }
}
