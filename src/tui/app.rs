use super::TuiBehaviours;
use super::cmd::Cmd;
use super::common::*;
use super::handle::handle_command;
use super::model::*;
use super::msg::{Msg, get_event_handling_msg};
use super::update::update;
use super::view::view;
use crate::domain::WatchUpdate;
use anyhow::Context;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::poll;
use ratatui::{Terminal, try_restore};
use std::path::PathBuf;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

const EVENT_POLL_DURATION_MS: u64 = 16;

pub async fn run(root: PathBuf, behaviours: TuiBehaviours) -> anyhow::Result<()> {
    let mut tui = AppTui::new(root, behaviours)?;
    tui.run().await
}

struct AppTui {
    pub(super) terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    pub(super) event_tx: Sender<Msg>,
    pub(super) event_rx: Receiver<Msg>,
    pub(super) model: Model,
}

impl AppTui {
    pub fn new(root: PathBuf, behaviours: TuiBehaviours) -> anyhow::Result<Self> {
        let terminal = ratatui::try_init()?;
        let (event_tx, event_rx) = mpsc::channel(10);

        let (width, height) = ratatui::crossterm::terminal::size()?;

        let terminal_dimensions = TerminalDimensions { width, height };

        let debug = std::env::var("DFFT_DEBUG").unwrap_or_default().trim() == "1";

        let model = Model::new(behaviours, root, terminal_dimensions, debug);

        Ok(Self {
            terminal,
            event_tx,
            event_rx,
            model,
        })
    }

    async fn run(&mut self) -> anyhow::Result<()> {
        let result = self.run_inner().await;
        self.model.pause_watching();

        if let Err(restore_err) = try_restore()
            && result.is_ok()
        {
            return Err(restore_err).context("couldn't restore terminal to its original state");
        }

        result
    }

    async fn run_inner(&mut self) -> anyhow::Result<()> {
        let _ = self.terminal.clear();

        // first render
        self.model.render_counter += 1;
        self.terminal.draw(|f| view(&mut self.model, f))?;

        let mut initial_cmds = vec![];
        if self.model.behaviours.watch {
            let changes_tx = self.model.watch_updates_tx.clone();
            initial_cmds.push(Cmd::WatchForChanges {
                root: self.model.root.clone(), // TODO: prevent cloning here
                cache: self.model.cache(),
                sender: changes_tx,
                cancellation_token: self.model.get_cancellation_token(),
                prepopulate_cache: self.model.behaviours.prepopulate_cache,
            });
        }

        for cmd in initial_cmds {
            handle_command(cmd.clone(), self.event_tx.clone()).await;
        }

        loop {
            tokio::select! {
                Some(message) = self.event_rx.recv() => {
                    let cmds = update(&mut self.model, message);

                    if self.model.running_state == RunningState::Done {
                        break;
                    }

                        self.model.render_counter += 1;
                        self.terminal.draw(|f| view(&mut self.model, f))?;

                    for cmd in cmds {
                        handle_command(cmd.clone(), self.event_tx.clone()).await;
                    }
                }

                Some(watch_update) = self.model.watch_updates_rx.recv() => {
                    self.model.watch_counter += 1;
                    let msg = match watch_update {
                        WatchUpdate::ChangeReceived(change) => Msg::ChangeReceived(change),
                        WatchUpdate::PrepopulationFinished => Msg::PrepopulationFinished,
                        WatchUpdate::PrepopulationFailed(e) => Msg::PrepopulationFailed(e),
                    };
                    let _ = self.event_tx.try_send(msg);
                }

                Ok(ready) = tokio::task::spawn_blocking(|| poll(Duration::from_millis(EVENT_POLL_DURATION_MS))) => {
                    match ready {
                        Ok(true) => {
                            // non blocking read since poll returned Ok(true)
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

        Ok(())
    }
}
