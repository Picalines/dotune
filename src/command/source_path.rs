use std::io;

use derived_deref::Deref;
use thiserror::Error;

use crate::shared::SimplePath;

/// A simple existing non-symbolic path to something in (or is the) repository
#[derive(Clone, Deref, Debug)]
pub struct SourcePath(SimplePath);

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
    pub fn new(repo_path: &RepoPath, path: SimplePath) -> Result<Self, SourcePathError> {
        let mut current = repo_path.to_path_buf();

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

    fn new_free(path: &SimplePath) -> Result<Self, SourcePathError> {
        if path.symlink_metadata().is_ok_and(|meta| meta.is_symlink()) {
            return Err(SourcePathError::IsOrInsideSymlink);
        }

        let canonical_path = match SimplePath::canonicalize(&path) {
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
    pub fn new(path: &SimplePath) -> Result<Self, RepoPathError> {
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
