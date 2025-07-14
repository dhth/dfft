use crate::domain::ChangeKind;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::io::Cursor;

const CREATE_SOUND: &[u8] = include_bytes!("assets/create.wav");
const MODIFY_SOUND: &[u8] = include_bytes!("assets/modify.wav");
const REMOVE_SOUND: &[u8] = include_bytes!("assets/remove.wav");
const ERROR_SOUND: &[u8] = include_bytes!("assets/error.wav");

pub struct AudioManager {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
}

impl AudioManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        Ok(AudioManager {
            _stream: stream,
            stream_handle,
        })
    }

    pub fn play_for_change(&self, change_kind: &ChangeKind) {
        let sound_data = match change_kind {
            ChangeKind::Created(_) => CREATE_SOUND,
            ChangeKind::Modified { .. } => MODIFY_SOUND,
            ChangeKind::Removed => REMOVE_SOUND,
        };

        self.play_sound_async(sound_data);
    }

    pub fn play_error_sound(&self) {
        self.play_sound_async(ERROR_SOUND);
    }

    fn play_sound_async(&self, sound_data: &'static [u8]) {
        let stream_handle = self.stream_handle.clone();
        
        if tokio::runtime::Handle::try_current().is_ok() {
            tokio::spawn(async move {
                let _ = Self::play_sound_internal(sound_data, &stream_handle);
            });
        }
    }

    fn play_sound_internal(
        sound_data: &'static [u8],
        stream_handle: &OutputStreamHandle,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let sink = Sink::try_new(stream_handle)?;
        let cursor = Cursor::new(sound_data);
        let source = Decoder::new(cursor)?;
        
        sink.append(source);
        sink.sleep_until_end();
        
        Ok(())
    }
}


