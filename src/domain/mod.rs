#[derive(Clone, Debug)]
pub struct Change {
    pub file_path: String,
    pub kind: ChangeKind,
}

#[derive(Clone, Debug)]
pub enum ChangeKind {
    Created(Result<(), String>),
    Modified(Result<ModifiedResult, String>),
    Removed,
}

#[derive(Clone, Debug)]
pub enum ModifiedResult {
    InitialSnapshot,
    Diff(Option<String>),
}
