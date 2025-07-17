use super::diff::Diff;

pub enum WatchUpdate {
    ChangeReceived(Change),
    PrepopulationFailed(String),
    PrepopulationFinished,
}

#[derive(Clone, Debug)]
pub struct Change {
    pub path: String,
    pub kind: ChangeKind,
}

#[derive(Clone, Debug)]
pub enum ChangeKind {
    Created(Result<String, String>),
    Modified(Result<Modification, String>),
    RemovedFile,
    RemovedDir,
}

#[derive(Clone, Debug)]
pub enum Modification {
    InitialSnapshot,
    Diff(Diff),
}
