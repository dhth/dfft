use crate::domain::ChangeKind;
use anyhow::Context;
use rodio::mixer::Mixer;
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
use std::io::Cursor;
use tracing::error;

const CREATE_SOUND: &[u8] = include_bytes!("assets/create.wav");
const MODIFY_SOUND: &[u8] = include_bytes!("assets/modify.wav");
const REMOVE_SOUND: &[u8] = include_bytes!("assets/remove.wav");
const ERROR_SOUND: &[u8] = include_bytes!("assets/error.wav");

pub struct AudioPlayer {
    stream: OutputStream,
}

impl AudioPlayer {
    pub fn new() -> anyhow::Result<Self> {
        let mut stream = OutputStreamBuilder::open_default_stream()
            .context("couldn't open default output stream")?;
        stream.log_on_drop(false);

        Ok(AudioPlayer { stream })
    }

    pub fn play_change_sound(&self, change_kind: &ChangeKind) {
        let sound_data = match change_kind {
            ChangeKind::Created(_) => CREATE_SOUND,
            ChangeKind::Modified { .. } => MODIFY_SOUND,
            ChangeKind::RemovedFile | ChangeKind::RemovedDir => REMOVE_SOUND,
        };

        self.play_sound(sound_data);
    }

    pub fn play_error_sound(&self) {
        self.play_sound(ERROR_SOUND);
    }

    fn play_sound(&self, sound_data: &'static [u8]) {
        if tokio::runtime::Handle::try_current().is_ok() {
            let mixer = self.stream.mixer().clone();

            tokio::task::spawn_blocking(move || {
                if let Err(e) = try_playing_sound(sound_data, &mixer) {
                    error!("couldn't play sound: {e}");
                }
            });
        }
    }
}

fn try_playing_sound(sound_data: &'static [u8], mixer: &Mixer) -> anyhow::Result<()> {
    let sink = Sink::connect_new(mixer);
    let cursor = Cursor::new(sound_data);
    let source = Decoder::new(cursor)?;

    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}
