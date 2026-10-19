use core::fmt;
use indoc::indoc;
use std::sync::Arc;

use thiserror::Error;

use super::abs_fs::FsError;
use crate::model::{AbsolutePath, AbsolutePathBuf, RelativeDisplay, SourcePathError};

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("{error}: {}", .path.relative_display())]
    SourcePath {
        path: Arc<AbsolutePath>,
        #[source]
        error: SourcePathError,
    },
    #[error("{}", display_write_to_repo(.modified_path, .via_symlink.as_deref()))]
    WriteToRepo {
        modified_path: Arc<AbsolutePath>,
        via_symlink: Option<AbsolutePathBuf>,
    },
    #[error("filesystem root cannot be a target")]
    WriteToRoot,
    #[error("a path with data exists: {}", .path.relative_display())]
    OverwriteConflict { path: Arc<AbsolutePath> },
    #[error(transparent)]
    Fs(#[from] FsError),
}

#[derive(Debug)]
pub struct ValidationErrorContext {
    pub error: ValidationError,
    pub defined_in: Arc<AbsolutePath>,
}

fn display_write_to_repo(
    modified_path: &AbsolutePath,
    via_symlink: Option<&AbsolutePath>,
) -> impl fmt::Display {
    fmt::from_fn(move |formatter| match via_symlink {
        Some(symlink) => write!(
            formatter,
            indoc! {"
                A symlink redirects a write into the repository:

                    symlink {}
                    overrides {}

                This is an ambiguous state, consider deleting the symlink.
                Perhaps you are migrating from an old version?
            "},
            symlink.relative_display(),
            modified_path.relative_display()
        ),
        None => write!(
            formatter,
            "an operation modifies the repository item: {}",
            modified_path.relative_display()
        ),
    })
}
