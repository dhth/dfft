#[derive(Debug, Clone)]
pub struct TuiBehaviours {
    pub watch: bool,
    pub follow_changes: bool,
    pub prepopulate_cache: bool,
}

#[cfg(test)]
impl TuiBehaviours {
    pub fn default_for_test() -> Self {
        Self {
            watch: true,
            follow_changes: false,
            prepopulate_cache: true,
        }
    }
}
