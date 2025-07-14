#[derive(Debug, Clone)]
pub struct TuiBehaviours {
    pub watch: bool,
    pub follow_changes: bool,
    pub prepopulate_cache: bool,
    #[cfg(feature = "sound")]
    pub play_sound: bool,
}

#[cfg(test)]
impl TuiBehaviours {
    pub fn default_for_test() -> Self {
        Self {
            watch: true,
            follow_changes: false,
            prepopulate_cache: true,
            #[cfg(feature = "sound")]
            play_sound: false,
        }
    }

    pub fn with_watch_off(self) -> Self {
        Self {
            watch: false,
            ..self
        }
    }
}
