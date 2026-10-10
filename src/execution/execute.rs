#[cfg(windows)]
use std::path::Path;
use std::sync::Arc;
use std::{fs, io};
#[cfg(unix)]
use std::{fs::remove_file as remove_symlink, os::unix::fs::symlink as create_symlink};

use thiserror::Error;

use crate::model::{AbsolutePath, Command, CommandKind, SourcePath};

#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("could not create directory {}: {error}", .path.display())]
    CreateDirectory {
        path: Arc<AbsolutePath>,
        #[source]
        error: io::Error,
    },
    #[error("target already exists and will not be overwritten")]
    OverwriteConflict,
    #[error("could not remove existing symlink: {0}")]
    RemoveSymlink(#[source] io::Error),
    #[error("could not create symlink: {0}")]
    CreateSymlink(#[source] io::Error),
    #[error(transparent)]
    Io(#[from] io::Error),
}

#[derive(Debug)]
pub enum ExecutionLog {
    DirectoryCreated(Arc<AbsolutePath>),
    SymlinkRemoved(Arc<AbsolutePath>),
    SymlinkCreated {
        source_path: SourcePath,
        destination_path: Arc<AbsolutePath>,
    },
    Error {
        error: ExecutionError,
        defined_in: Arc<AbsolutePath>,
        source_path: SourcePath,
        destination_path: Arc<AbsolutePath>,
    },
}

pub fn execute_commands(
    commands: impl IntoIterator<Item = Command>,
    mut log: impl FnMut(ExecutionLog),
) {
    for command in commands {
        match command.kind {
            CommandKind::Link {
                source_path,
                destination_path,
            } => {
                if let Err(error) = execute_link(&source_path, &destination_path, &mut log) {
                    log(ExecutionLog::Error {
                        error,
                        defined_in: command.defined_in,
                        source_path,
                        destination_path,
                    });
                }
            }
        }
    }
}

fn execute_link(
    source_path: &SourcePath,
    destination_path: &AbsolutePath,
    log: &mut impl FnMut(ExecutionLog),
) -> Result<(), ExecutionError> {
    let parent = destination_path
        .parent()
        .expect("validated link destination has a parent directory");

    let parent_existed = parent.is_dir();
    fs::create_dir_all(parent).map_err(|error| ExecutionError::CreateDirectory {
        path: parent.into(),
        error,
    })?;
    if !parent_existed {
        log(ExecutionLog::DirectoryCreated(parent.into()));
    }

    match fs::symlink_metadata(destination_path) {
        Ok(metadata) if metadata.is_symlink() => {
            remove_symlink(destination_path).map_err(ExecutionError::RemoveSymlink)?;
            log(ExecutionLog::SymlinkRemoved(destination_path.into()));
        }
        Ok(_) => return Err(ExecutionError::OverwriteConflict),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }

    create_symlink(&**source_path, destination_path).map_err(ExecutionError::CreateSymlink)?;

    log(ExecutionLog::SymlinkCreated {
        source_path: source_path.clone(),
        destination_path: destination_path.into(),
    });
    Ok(())
}

#[cfg(windows)]
fn create_symlink(source_path: &Path, destination_path: &Path) -> io::Result<()> {
    use std::os::windows::fs::{symlink_dir, symlink_file};

    if source_path.metadata()?.is_dir() {
        symlink_dir(source_path, destination_path)
    } else {
        symlink_file(source_path, destination_path)
    }
}

#[cfg(windows)]
fn remove_symlink(path: &Path) -> io::Result<()> {
    use std::os::windows::fs::FileTypeExt;

    if path.symlink_metadata()?.file_type().is_symlink_dir() {
        fs::remove_dir(path)
    } else {
        fs::remove_file(path)
    }
}
