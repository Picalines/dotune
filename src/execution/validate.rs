use crate::model::{
    AbsolutePath, Command, CommandKind, Instruction, InstructionKind, RepoPath, SourcePath,
    SourcePathError,
};
use std::{io, path::Path, sync::Arc};
use thiserror::Error;

pub struct ValidationContext<'a> {
    pub repo_path: &'a RepoPath,
}

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("{error}: {}", .path.display())]
    SourcePath {
        path: Arc<AbsolutePath>,
        #[source]
        error: SourcePathError,
    },
    #[error("a path in the repository cannot be a target: {}", .path.display())]
    WriteToRepo { path: Arc<AbsolutePath> },
    #[error("filesystem root cannot be a target")]
    WriteToRoot,
    #[error("path exists: {}", .path.display())]
    OverwriteConflict { path: Arc<AbsolutePath> },
    #[error(
        "malformed path '{}': '{}' is not a directory",
        .user_path.display(),
        .not_dir_path.display()
    )]
    MalformedDirPath {
        user_path: Arc<AbsolutePath>,
        not_dir_path: Arc<AbsolutePath>,
    },
    #[error(
        "malformed path '{}': '{}' is a broken symlink",
        .user_path.display(),
        .symlink_path.display()
    )]
    BrokenDirSymlink {
        user_path: Arc<AbsolutePath>,
        symlink_path: Arc<AbsolutePath>,
    },
    #[error(transparent)]
    Io(#[from] io::Error),
}

#[derive(Debug)]
pub struct ValidationErrorContext {
    pub error: ValidationError,
    pub defined_in: Arc<AbsolutePath>,
}

pub fn validate(
    context: &ValidationContext,
    instruction: Instruction,
) -> Result<Command, ValidationErrorContext> {
    let with_context = |error: ValidationError| ValidationErrorContext {
        error,
        defined_in: instruction.defined_in.clone(),
    };

    let kind = match instruction.kind {
        InstructionKind::Link {
            source_path,
            destination_path,
        } => {
            validate_link(context.repo_path, source_path, destination_path).map_err(with_context)?
        }
    };

    Ok(Command {
        defined_in: instruction.defined_in,
        kind,
    })
}

fn validate_link(
    repo_path: &RepoPath,
    source_path: Arc<AbsolutePath>,
    destination_path: Arc<AbsolutePath>,
) -> Result<CommandKind, ValidationError> {
    let source_path = validate_source(repo_path, source_path)?;
    validate_destination(repo_path, &destination_path)?;
    Ok(CommandKind::Link {
        source_path,
        destination_path,
    })
}

fn validate_source(
    repo_path: &RepoPath,
    source_path: Arc<AbsolutePath>,
) -> Result<SourcePath, ValidationError> {
    match SourcePath::new(repo_path, source_path.clone()) {
        Ok(path) => Ok(path),
        Err(error) => Err(ValidationError::SourcePath {
            path: source_path,
            error,
        }),
    }
}

fn validate_destination(
    repo_path: &RepoPath,
    destination_path: &AbsolutePath,
) -> Result<(), ValidationError> {
    let repo_path: &Path = repo_path;

    if destination_path.starts_with(repo_path) {
        return Err(ValidationError::WriteToRepo {
            path: destination_path.into(),
        });
    }

    let existing_parent = find_existing_parent(destination_path)?;

    let parent_metadata = match existing_parent.metadata() {
        Ok(metadata) => metadata,
        Err(error)
            if matches!(
                error.kind(),
                io::ErrorKind::NotFound | io::ErrorKind::NotADirectory
            ) =>
        {
            return Err(ValidationError::BrokenDirSymlink {
                user_path: destination_path.into(),
                symlink_path: existing_parent.into(),
            });
        }
        Err(error) => return Err(error.into()),
    };

    if !parent_metadata.is_dir() {
        return Err(ValidationError::MalformedDirPath {
            user_path: destination_path.into(),
            not_dir_path: existing_parent.into(),
        });
    }

    let resolved_parent = existing_parent.canonicalize()?;
    if resolved_parent.starts_with(repo_path) {
        return Err(ValidationError::WriteToRepo {
            path: destination_path.into(),
        });
    }

    match destination_path.symlink_metadata() {
        Ok(metadata) if metadata.is_symlink() => Ok(()),
        Ok(_) => Err(ValidationError::OverwriteConflict {
            path: destination_path.into(),
        }),
        Err(error) if matches!(error.kind(), io::ErrorKind::NotFound) => Ok(()),
        Err(error) => Err(error.into()),
    }
}

fn find_existing_parent(path: &AbsolutePath) -> Result<&AbsolutePath, ValidationError> {
    let mut parent = path.parent().ok_or(ValidationError::WriteToRoot)?;

    loop {
        match parent.symlink_metadata() {
            Ok(_) => return Ok(parent),
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::NotFound | io::ErrorKind::NotADirectory
                ) =>
            {
                parent = parent.parent().ok_or(error)?;
            }
            Err(error) => return Err(error.into()),
        }
    }
}
