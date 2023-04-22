use core::fmt;

pub enum TauriCommand{
    Evidence,
    Verification,
}

impl fmt::Display for TauriCommand{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TauriCommand::Evidence => write!(f, "evidence"),
            TauriCommand::Verification => write!(f, "verification"),
        }
    }
}