use std::{io, path::Path};

use derived_deref::Deref;
use thiserror::Error;

use crate::shared::AbsolutePathBuf;

/// An absolute existing non-symbolic path to something in (or is the) repository
#[derive(Clone, Deref, Debug)]
pub struct SourcePath(AbsolutePathBuf);

#[derive(Error, Debug)]
pub enum SourcePathError {
    #[error("source path does not exist")]
    DoesNotExist,
    #[error("source path is itself or inside a symlink")]
    IsOrInsideSymlink,
    #[error("source path does not belong to the repo")]
    NotInRepo,
    #[error("unexpected i/o error")]
    Io(#[from] io::Error),
}

#[derive(Clone, Deref, Debug)]
pub struct RepoPath(SourcePath);

#[derive(Error, Debug)]
pub enum RepoPathError {
    #[error("source path does not exist")]
    DoesNotExist,
    #[error("repository path is not a directory")]
    IsNotDirectory,
    #[error("source path is a symlink")]
    IsSymlink,
    #[error("unexpected i/o error")]
    Io(#[from] io::Error),
}

impl SourcePath {
    pub fn new(repo_path: &RepoPath, path: AbsolutePathBuf) -> Result<Self, SourcePathError> {
        let mut current = Path::to_path_buf(&repo_path);

        for component in path
            .strip_prefix(&current)
            .map_err(|_| SourcePathError::NotInRepo)?
            .components()
        {
            current.push(component);
            if current.symlink_metadata()?.is_symlink() {
                return Err(SourcePathError::IsOrInsideSymlink);
            }
        }

        Ok(Self(path))
    }

    fn new_free(path: &AbsolutePathBuf) -> Result<Self, SourcePathError> {
        if path.symlink_metadata().is_ok_and(|meta| meta.is_symlink()) {
            return Err(SourcePathError::IsOrInsideSymlink);
        }

        let canonical_path = match AbsolutePathBuf::canonicalize(&path) {
            Ok(path) => path,
            Err(error) => match error.kind() {
                io::ErrorKind::NotFound => return Err(SourcePathError::DoesNotExist),
                _ => return Err(error.into()),
            },
        };

        Ok(Self(canonical_path))
    }
}

impl RepoPath {
    pub fn new(path: &AbsolutePathBuf) -> Result<Self, RepoPathError> {
        let path = match SourcePath::new_free(path) {
            Ok(path) => path,
            Err(source_error) => {
                return Err(match source_error {
                    SourcePathError::NotInRepo => unreachable!(),
                    SourcePathError::DoesNotExist => RepoPathError::DoesNotExist,
                    SourcePathError::IsOrInsideSymlink => RepoPathError::IsSymlink,
                    SourcePathError::Io(error) => RepoPathError::Io(error),
                });
            }
        };

        if !path.is_dir() {
            return Err(RepoPathError::IsNotDirectory);
        }

        Ok(Self(path))
    }
}
