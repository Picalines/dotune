use std::path::Path;
use std::sync::Arc;

use thiserror::Error;

use super::abs_fs::{self, FsError};
use crate::model::{
    AbsolutePath, AbsolutePathBuf, Command, CommandKind, Instruction, InstructionKind, RepoPath,
    SourcePath, SourcePathError,
};

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
    #[error("an operation would have overwritten the repository: {}", .0.display())]
    WriteToRepo(Arc<AbsolutePath>),
    #[error("filesystem root cannot be a target")]
    WriteToRoot,
    #[error("path exists: {}", .path.display())]
    OverwriteConflict { path: Arc<AbsolutePath> },
    #[error(transparent)]
    Fs(#[from] FsError),
}

#[derive(Debug)]
pub struct ValidationErrorContext {
    pub error: ValidationError,
    pub defined_in: Arc<AbsolutePath>,
}

pub fn validate_instructions(
    instructions: impl IntoIterator<Item = Instruction>,
    context: ValidationContext,
) -> Result<Vec<Command>, ValidationErrorContext> {
    // TODO: instructions might conflict with each other
    instructions
        .into_iter()
        .map(|instruction| validate_instruction(&context, instruction))
        .collect()
}

fn validate_instruction(
    context: &ValidationContext,
    instruction: Instruction,
) -> Result<Command, ValidationErrorContext> {
    let with_context = |error: ValidationError| ValidationErrorContext {
        error,
        defined_in: instruction.defined_in.clone(),
    };

    let kind =
        match instruction.kind {
            InstructionKind::Link {
                source_path,
                destination_path,
            } => validate_link(context.repo_path, source_path, destination_path)
                .map_err(with_context)?,
            InstructionKind::Copy {
                source_path,
                destination_path,
            } => validate_copy(context.repo_path, source_path, destination_path)
                .map_err(with_context)?,
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
    if abs_fs::symlink_metadata_if_exists(&destination_path)?
        .is_some_and(|metadata| !metadata.is_symlink())
    {
        return Err(ValidationError::OverwriteConflict {
            path: destination_path,
        });
    }
    Ok(CommandKind::Link {
        source_path,
        destination_path,
    })
}

fn validate_copy(
    repo_path: &RepoPath,
    source_path: Arc<AbsolutePath>,
    destination_path: Arc<AbsolutePath>,
) -> Result<CommandKind, ValidationError> {
    let source_path = validate_source(repo_path, source_path)?;
    let resolved_destination = validate_destination(repo_path, &destination_path)?;

    if repo_path.starts_with(&resolved_destination) {
        for entry in abs_fs::iter_tree(&source_path) {
            let (relative_path, metadata) = entry?;
            let target = resolved_destination.join(relative_path);
            if target.starts_with(repo_path)
                || (repo_path.starts_with(&target) && !metadata.is_dir())
            {
                return Err(ValidationError::WriteToRepo(target.into()));
            }
        }
    }

    Ok(CommandKind::Copy {
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
) -> Result<AbsolutePathBuf, ValidationError> {
    let repo_path: &Path = repo_path;

    if destination_path.starts_with(repo_path) {
        return Err(ValidationError::WriteToRepo(destination_path.into()));
    }

    let destination_parent = destination_path
        .parent()
        .ok_or(ValidationError::WriteToRoot)?;

    let resolved_destination = abs_fs::resolve_partially_existing_path(destination_parent)?.join(
        destination_path
            .file_name()
            .expect("non-root absolute path has a file name"),
    );

    if resolved_destination.starts_with(repo_path) {
        return Err(ValidationError::WriteToRepo(destination_path.into()));
    }

    Ok(resolved_destination)
}
