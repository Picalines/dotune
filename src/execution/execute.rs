use std::sync::Arc;

use super::abs_fs::{self, FsError};
use crate::model::{AbsolutePath, Command, CommandKind, SourcePath};

#[derive(Debug)]
pub enum ExecutionLog {
    DirectoryCreated(Arc<AbsolutePath>),
    PathDeleted(Arc<AbsolutePath>),
    SymlinkCreated {
        source_path: SourcePath,
        destination_path: Arc<AbsolutePath>,
    },
    Copied {
        source_path: SourcePath,
        destination_path: Arc<AbsolutePath>,
    },
    Error {
        operation: &'static str,
        error: FsError,
        source_path: SourcePath,
        destination_path: Arc<AbsolutePath>,
    },
}

pub fn execute_commands(
    commands: impl IntoIterator<Item = Command>,
    mut log: impl FnMut(&AbsolutePath, ExecutionLog),
) {
    for command in commands {
        let mut command_log = |entry| log(&command.defined_in, entry);

        let (operation, source_path, destination_path, result) = match command.kind {
            CommandKind::Link {
                source_path,
                destination_path,
            } => {
                let result = execute_link(&source_path, &destination_path, &mut command_log);
                ("link", source_path, destination_path, result)
            }
            CommandKind::Copy {
                source_path,
                destination_path,
            } => {
                let result = execute_copy(&source_path, &destination_path, &mut command_log);
                ("copy", source_path, destination_path, result)
            }
        };
        if let Err(error) = result {
            command_log(ExecutionLog::Error {
                operation,
                error,
                source_path,
                destination_path,
            });
        }
    }
}

fn execute_link(
    source_path: &SourcePath,
    destination_path: &AbsolutePath,
    log: &mut impl FnMut(ExecutionLog),
) -> Result<(), FsError> {
    if let Some(parent) = abs_fs::create_parent_dir(destination_path)? {
        log(ExecutionLog::DirectoryCreated(parent.into()));
    }

    if let Some(metadata) = abs_fs::symlink_metadata_if_exists(destination_path)?
        && metadata.is_symlink()
    {
        abs_fs::remove_path(destination_path, metadata.file_type())?;
        log(ExecutionLog::PathDeleted(destination_path.into()));
    }

    abs_fs::create_symlink(
        source_path,
        destination_path,
        source_path
            .metadata()
            .map_err(|error| FsError::Io(source_path.to_path_buf(), error))?
            .file_type(),
    )?;

    log(ExecutionLog::SymlinkCreated {
        source_path: source_path.clone(),
        destination_path: destination_path.into(),
    });
    Ok(())
}

fn execute_copy(
    source_path: &SourcePath,
    destination_path: &AbsolutePath,
    log: &mut impl FnMut(ExecutionLog),
) -> Result<(), FsError> {
    if let Some(parent) = abs_fs::create_parent_dir(destination_path)? {
        log(ExecutionLog::DirectoryCreated(parent.into()));
    }
    abs_fs::copy_overlay(source_path, destination_path)?;
    log(ExecutionLog::Copied {
        source_path: source_path.clone(),
        destination_path: destination_path.into(),
    });
    Ok(())
}
