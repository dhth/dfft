use super::cmd::Cmd;
use super::common::*;
use super::handle::handle_command;
use super::model::*;
use super::msg::{Msg, get_event_handling_msg};
use super::update::update;
use super::view::view;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::poll;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

const EVENT_POLL_DURATION_MS: u64 = 16;

pub async fn run() -> anyhow::Result<()> {
    let mut tui = AppTui::new()?;
    tui.run().await
}

struct AppTui {
    pub(super) terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    pub(super) event_tx: Sender<Msg>,
    pub(super) event_rx: Receiver<Msg>,
    pub(super) model: Model,
}

impl AppTui {
    pub fn new() -> anyhow::Result<Self> {
        let terminal = ratatui::try_init()?;
        let (event_tx, event_rx) = mpsc::channel(10);

        let (width, height) = ratatui::crossterm::terminal::size()?;

        let terminal_dimensions = TerminalDimensions { width, height };

        let debug = std::env::var("DFFT_DEBUG").unwrap_or_default().trim() == "1";

        let model = Model::new(terminal_dimensions, true, debug);

        Ok(Self {
            terminal,
            event_tx,
            event_rx,
            model,
        })
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        let _ = self.terminal.clear();

        // first render
        self.model.user_msg = Some(UserMsg::info("watching for changes..."));
        self.model.render_counter += 1;
        self.terminal.draw(|f| view(&mut self.model, f))?;

        let mut initial_cmds = vec![];
        let changes_tx = self.model.changes_tx.clone();
        initial_cmds.push(Cmd::WatchForChanges((
            changes_tx,
            self.model.get_cancellation_token(),
        )));

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

                Some(change) = self.model.changes_rx.recv() => {
                    let msg = Msg::ChangeReceived(change);
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

        self.exit()
    }

    fn exit(&mut self) -> anyhow::Result<()> {
        self.model.pause_watching();
        ratatui::try_restore()?;
        Ok(())
    }
}
