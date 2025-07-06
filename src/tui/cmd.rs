#[derive(Clone, Debug)]
pub(super) enum Cmd {
    Dummy,
}

impl std::fmt::Display for Cmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cmd::Dummy => write!(f, "dummy"),
        }
    }
}
