use crate::domain::ChangeKind;
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;

const CREATE_SOUND: &[u8] = include_bytes!("assets/create.wav");
const MODIFY_SOUND: &[u8] = include_bytes!("assets/modify.wav");
const REMOVE_SOUND: &[u8] = include_bytes!("assets/remove.wav");
const ERROR_SOUND: &[u8] = include_bytes!("assets/error.wav");

pub fn play_notification_for_change(change_kind: &ChangeKind) {
    let sound_data = match change_kind {
        ChangeKind::Created(_) => CREATE_SOUND,
        ChangeKind::Modified { .. } => MODIFY_SOUND,
        ChangeKind::Removed => REMOVE_SOUND,
    };

    if tokio::runtime::Handle::try_current().is_ok() {
        tokio::spawn(async move {
            let _ = play_sound_internal(sound_data).is_err();
        });
    }
}

pub fn play_error_notification() {
    if tokio::runtime::Handle::try_current().is_ok() {
        tokio::spawn(async {
            let _ = play_sound_internal(ERROR_SOUND);
        });
    }
}

fn play_sound_internal(
    sound_data: &'static [u8],
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    let cursor = Cursor::new(sound_data);

    let source = Decoder::new(cursor)?;

    sink.append(source);

    sink.sleep_until_end();

    Ok(())
}
