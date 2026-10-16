use core::ops::Deref;
use std::io;
use std::path::Path;
use std::sync::Arc;

use derived_deref::Deref;
use thiserror::Error;

use crate::model::{AbsolutePath, AbsolutePathBuf};

/// An absolute existing non-symbolic path to something in (or is the)
/// repository
#[derive(Clone, Debug)]
pub struct SourcePath(Arc<AbsolutePath>);

#[derive(Error, Debug)]
pub enum SourcePathError {
    #[error("path in repository does not exist")]
    DoesNotExist,
    #[error("path in repository lives inside a symlink, aborting")]
    IsOrInsideSymlink,
    #[error("a path lives outside the repository")]
    NotInRepo,
    #[error("unexpected IO error")]
    Io(#[from] io::Error),
}

#[derive(Clone, Deref, Debug)]
pub struct RepoPath(SourcePath);

#[derive(Error, Debug)]
pub enum RepoPathError {
    #[error("repository path does not exist")]
    DoesNotExist,
    #[error("repository path is not a directory")]
    IsNotDirectory,
    #[error("repository path is a symlink, aborting")]
    IsSymlink,
    #[error("unexpected IO error")]
    Io(#[from] io::Error),
}

impl SourcePath {
    pub fn new(
        repo_path: &RepoPath,
        path: impl Into<Arc<AbsolutePath>>,
    ) -> Result<Self, SourcePathError> {
        let path = path.into();
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

    fn new_free(path: &AbsolutePath) -> Result<Self, SourcePathError> {
        if path.symlink_metadata().is_ok_and(|meta| meta.is_symlink()) {
            return Err(SourcePathError::IsOrInsideSymlink);
        }

        let canonical_path = match AbsolutePathBuf::canonicalize(path) {
            Ok(path) => path,
            Err(error) => match error.kind() {
                io::ErrorKind::NotFound => return Err(SourcePathError::DoesNotExist),
                _ => return Err(error.into()),
            },
        };

        Ok(Self(canonical_path.into()))
    }
}

impl RepoPath {
    pub fn new(path: &AbsolutePath) -> Result<Self, RepoPathError> {
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

impl Deref for SourcePath {
    type Target = AbsolutePath;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Path> for SourcePath {
    fn as_ref(&self) -> &Path {
        self.deref().as_ref()
    }
}

impl AsRef<Path> for RepoPath {
    fn as_ref(&self) -> &Path {
        self.deref().as_ref()
    }
}
