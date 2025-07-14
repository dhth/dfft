#[derive(Debug, Clone)]
pub struct TuiBehaviours {
    pub watch: bool,
    pub follow_changes: bool,
    pub prepopulate_cache: bool,
    pub play_sound: bool,
}

#[cfg(test)]
impl TuiBehaviours {
    pub fn default_for_test() -> Self {
        Self {
            watch: true,
            follow_changes: false,
            prepopulate_cache: true,
            play_sound: false,
        }
    }

    pub fn with_watch(self, watch: bool) -> Self {
        Self {
            watch,
            follow_changes: self.follow_changes,
            prepopulate_cache: self.prepopulate_cache,
            play_sound: self.play_sound,
        }
    }
}
