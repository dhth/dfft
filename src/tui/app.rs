use super::cmd::Cmd;
use super::common::*;
use super::handle::handle_command;
use super::model::*;
use super::msg::{Msg, get_event_handling_msg};
use super::update::update;
use super::view::view;
use crate::changes::listen_for_changes;
use crate::domain::Change;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::collections::HashMap;
use std::io::Error as IOError;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

const EVENT_POLL_DURATION_MS: u64 = 16;
pub const REFRESH_RESULTS_INTERVAL_SECS: u64 = 10;

pub async fn run() -> anyhow::Result<()> {
    let mut tui = AppTui::new()?;
    tui.run().await?;

    Ok(())
}

struct AppTui {
    pub(super) terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    pub(super) event_tx: Sender<Msg>,
    pub(super) event_rx: Receiver<Msg>,
    pub(super) model: Model,
    pub(super) initial_commands: Vec<Cmd>,
}

impl AppTui {
    pub fn new() -> anyhow::Result<Self> {
        let terminal = ratatui::try_init()?;
        let (event_tx, event_rx) = mpsc::channel(10);
        let mut initial_commands = Vec::new();
        // for cluster in &clusters {
        //     initial_commands.push(Command::GetServices(cluster.clone()));
        // }

        let (width, height) = ratatui::crossterm::terminal::size()?;

        let terminal_dimensions = TerminalDimensions { width, height };

        let debug = std::env::var("DFFT_DEBUG").unwrap_or_default().trim() == "1";

        let model = Model::new(terminal_dimensions, debug);

        Ok(Self {
            terminal,
            event_tx,
            event_rx,
            model,
            initial_commands,
        })
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        let message_clear_duration = Duration::from_secs(CLEAR_USER_MESSAGE_LOOP_INTERVAL_SECS);
        let mut message_clear_interval = tokio::time::interval(message_clear_duration);
        let _ = self.terminal.clear();

        for cmd in &self.initial_commands {
            handle_command(cmd.clone(), self.event_tx.clone()).await;
        }

        let (changes_sender, mut changes_rec) = mpsc::channel::<Change>(100);

        tokio::spawn(async move {
            listen_for_changes(changes_sender.clone()).await;
        });

        // first render
        self.model.render_counter += 1;
        self.terminal.draw(|f| view(&mut self.model, f))?;

        loop {
            tokio::select! {
                _instant = message_clear_interval.tick() => {
                    if self.model.user_message.is_some() {
                        _ = self.event_tx.try_send(Msg::ClearUserMessage);
                    }
                }

                Some(message) = self.event_rx.recv() => {
                    let cmds = update(&mut self.model, message);

                    if self.model.running_state == RunningState::Done {
                        self.exit()?;
                        return Ok(());
                    }

                        self.model.render_counter += 1;
                        self.terminal.draw(|f| view(&mut self.model, f))?;

                    for cmd in cmds {
                        handle_command(cmd.clone(), self.event_tx.clone()).await;
                    }
                }

                Some(change) = changes_rec.recv() => {
                    let msg = Msg::ChangeReceived(change);
                    let _ = self.event_tx.try_send(msg);
                }

                Ok(ready) = tokio::task::spawn_blocking(|| ratatui::crossterm::event::poll(Duration::from_millis(EVENT_POLL_DURATION_MS))) => {
                    match ready {
                        Ok(true) => {
                            let event = ratatui::crossterm::event::read()?;
                            self.model.event_counter += 1;
                            if let Some(handling_msg) = get_event_handling_msg(&self.model, event) {
                                self.event_tx.try_send(handling_msg)?;
                            }
                        }
                        Ok(false) => continue,
                        Err(e) => {
                                return Err(anyhow::anyhow!(e));
                        }
                    }
                }
            }
        }
    }

    fn exit(&mut self) -> Result<(), IOError> {
        ratatui::try_restore()
    }
}
