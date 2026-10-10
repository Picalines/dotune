use std::sync::Arc;

use super::{AbsolutePath, SourcePath};

/// An instruction the user gave us in their config. It may not be valid yet
#[derive(Debug)]
pub struct Instruction {
    pub defined_in: Arc<AbsolutePath>,
    pub kind: InstructionKind,
}

#[derive(Debug)]
pub enum InstructionKind {
    Link {
        source_path: Arc<AbsolutePath>,
        destination_path: Arc<AbsolutePath>,
    },
}

/// A command that we derived from an [`Instruction`], ready for execution
#[derive(Debug)]
pub struct Command {
    pub defined_in: Arc<AbsolutePath>,
    pub kind: CommandKind,
}

#[derive(Debug)]
pub enum CommandKind {
    Link {
        source_path: SourcePath,
        destination_path: Arc<AbsolutePath>,
    },
}
