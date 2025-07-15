use crate::domain::ChangeKind;
use anyhow::Context;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::io::Cursor;
use tracing::error;

const CREATE_SOUND: &[u8] = include_bytes!("assets/create.wav");
const MODIFY_SOUND: &[u8] = include_bytes!("assets/modify.wav");
const REMOVE_SOUND: &[u8] = include_bytes!("assets/remove.wav");
const ERROR_SOUND: &[u8] = include_bytes!("assets/error.wav");

pub struct AudioPlayer {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
}

impl AudioPlayer {
    pub fn new() -> anyhow::Result<Self> {
        let (stream, stream_handle) = OutputStream::try_default()
            .context("couldn't open stream to default output audio device")?;
        Ok(AudioPlayer {
            _stream: stream,
            stream_handle,
        })
    }

    pub fn play_change_sound(&self, change_kind: &ChangeKind) {
        let sound_data = match change_kind {
            ChangeKind::Created(_) => CREATE_SOUND,
            ChangeKind::Modified { .. } => MODIFY_SOUND,
            ChangeKind::Removed => REMOVE_SOUND,
        };

        self.play_sound(sound_data);
    }

    pub fn play_error_sound(&self) {
        self.play_sound(ERROR_SOUND);
    }

    fn play_sound(&self, sound_data: &'static [u8]) {
        let stream_handle = self.stream_handle.clone();

        if tokio::runtime::Handle::try_current().is_ok() {
            tokio::task::spawn_blocking(move || {
                if let Err(e) = try_playing_sound(sound_data, &stream_handle) {
                    error!("couldn't play sound: {e}");
                }
            });
        }
    }
}

fn try_playing_sound(
    sound_data: &'static [u8],
    stream_handle: &OutputStreamHandle,
) -> anyhow::Result<()> {
    let sink = Sink::try_new(stream_handle)?;
    let cursor = Cursor::new(sound_data);
    let source = Decoder::new(cursor)?;

    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}
