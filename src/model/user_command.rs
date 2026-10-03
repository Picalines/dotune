use super::AbsolutePath;
use std::sync::Arc;

/// A command (instruction) the user gave us in their config. It may not be valid yet
#[derive(Debug)]
pub struct UserCommand {
    pub defined_in: Arc<AbsolutePath>,
    pub kind: UserCommandKind,
}

#[derive(Debug)]
pub enum UserCommandKind {
    Link {
        source_path: Arc<AbsolutePath>,
        destination_path: Arc<AbsolutePath>,
    },
}
