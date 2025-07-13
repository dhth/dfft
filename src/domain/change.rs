use super::diff::Diff;

pub enum WatchUpdate {
    ChangeReceived(Change),
    PrepopulationError(String),
    PrepopulationFinished,
}

#[derive(Clone, Debug)]
pub struct Change {
    pub file_path: String,
    pub kind: ChangeKind,
}

#[derive(Clone, Debug)]
pub enum ChangeKind {
    Created(Result<String, String>),
    Modified(Result<Modification, String>),
    Removed,
}

#[derive(Clone, Debug)]
pub enum Modification {
    InitialSnapshot,
    Diff(Option<Diff>),
}
