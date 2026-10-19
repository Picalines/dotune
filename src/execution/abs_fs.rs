use core::iter;
use std::path::{Path, PathBuf};
use std::{fs, io};

use thiserror::Error;

use crate::model::{AbsolutePath, AbsolutePathBuf, RelativeDisplay};

#[derive(Error, Debug)]
pub enum FsError {
    #[error("'{}' is not a directory", .0.relative_display())]
    NotDirectory(PathBuf),
    #[error("'{}' is a broken symlink", .0.relative_display())]
    BrokenSymlink(PathBuf),
    #[error("unsupported file type: '{}'", .0.relative_display())]
    UnsupportedFileType(PathBuf),
    #[error("'{}': {}", .0.relative_display(), .1)]
    Io(PathBuf, #[source] io::Error),
}

fn fs_io_error(path: &Path, error: io::Error) -> FsError {
    FsError::Io(path.to_owned(), error)
}

pub fn symlink_metadata_if_exists(path: &Path) -> Result<Option<fs::Metadata>, FsError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) => Ok(Some(metadata)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(fs_io_error(path, error)),
    }
}

/// Returns the parent directory if it had to be created.
pub fn create_parent_dir(path: &AbsolutePath) -> Result<Option<&AbsolutePath>, FsError> {
    if let Some(parent) = path.parent()
        && !parent.is_dir()
    {
        fs::create_dir_all(parent).map_err(|error| fs_io_error(parent, error))?;
        return Ok(Some(parent));
    }
    Ok(None)
}

/// Resolves the path as if it exists. Imagine a path going through a symlink
/// and missing a tail - it will follow that symlink, but then append the tail
pub fn resolve_partially_existing_path(path: &AbsolutePath) -> Result<AbsolutePathBuf, FsError> {
    let (existing_path, metadata) = find_existing_ancestor(path)?;

    let metadata = follow_symlink(existing_path, metadata)?;
    if !metadata.is_dir() {
        return Err(FsError::NotDirectory(existing_path.to_path_buf()));
    }

    let resolved = AbsolutePathBuf::canonicalize(existing_path)
        .map_err(|error| fs_io_error(existing_path, error))?;

    Ok(resolved.join(
        path.strip_prefix(existing_path)
            .expect("existing is an ancestor"),
    ))
}

pub fn find_parent_symlink_into<'a>(
    path: &'a AbsolutePath,
    target_root: &AbsolutePath,
) -> Option<&'a AbsolutePath> {
    path.ancestors().skip(1).find(|parent| {
        fs::read_link(parent).is_ok()
            && AbsolutePathBuf::canonicalize(parent)
                .is_ok_and(|resolved| resolved.starts_with(target_root))
    })
}

pub fn create_symlink(
    source: &Path,
    destination: &Path,
    file_type: fs::FileType,
) -> Result<(), FsError> {
    #[cfg(unix)]
    let result = {
        use std::os::unix::fs::symlink;

        let _ = file_type;
        symlink(source, destination)
    };
    #[cfg(windows)]
    let result = {
        use std::os::windows::fs::{FileTypeExt, symlink_dir, symlink_file};

        if file_type.is_dir() || file_type.is_symlink_dir() {
            symlink_dir(source, destination)
        } else {
            symlink_file(source, destination)
        }
    };
    result.map_err(|error| fs_io_error(destination, error))
}

/// Remove an object at the path:
/// - file is deleted
/// - directory is deleted with all physical contents
/// - symlink itself is deleted, while physical contents are kept
pub fn remove_path(path: &Path, file_type: fs::FileType) -> Result<(), FsError> {
    #[cfg(windows)]
    {
        use std::os::windows::fs::FileTypeExt;

        if file_type.is_symlink_dir() {
            return fs::remove_dir(path).map_err(|error| fs_io_error(path, error));
        }
    }
    let result = if file_type.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    };
    result.map_err(|error| fs_io_error(path, error))
}

/// Yields paths relative to root, including an empty path for root itself.
pub fn iter_tree(
    root: &AbsolutePath,
) -> impl Iterator<Item = Result<(PathBuf, fs::Metadata), FsError>> + use<> {
    let root = root.to_owned();
    let mut pending = vec![PathBuf::new()];
    iter::from_fn(move || {
        pending.pop().map(|relative_path| {
            let path = root.join(&relative_path);
            let metadata = path
                .symlink_metadata()
                .map_err(|error| fs_io_error(&path, error))?;
            if metadata.is_dir() {
                for entry in fs::read_dir(&path).map_err(|error| fs_io_error(&path, error))? {
                    let entry = entry.map_err(|error| fs_io_error(&path, error))?;
                    pending.push(relative_path.join(entry.file_name()));
                }
            }
            Ok((relative_path, metadata))
        })
    })
}

/// The destination's parent must exist, and destination entries must not alias
/// the source.
pub fn copy_overlay(source: &AbsolutePath, destination: &AbsolutePath) -> Result<(), FsError> {
    let mut directories = Vec::new();

    for entry in iter_tree(source) {
        let (relative_entry_path, entry_metadata) = entry?;
        let entry_path = source.join(&relative_entry_path);
        let entry_destination = destination.join(&relative_entry_path);

        let entry_type = entry_metadata.file_type();
        if !entry_type.is_symlink() && !entry_type.is_dir() && !entry_type.is_file() {
            return Err(FsError::UnsupportedFileType(entry_path.into()));
        }

        let destination_metadata = symlink_metadata_if_exists(&entry_destination)?;
        let reuse_destination = destination_metadata
            .as_ref()
            .is_some_and(|meta| !entry_type.is_symlink() && meta.file_type() == entry_type);

        if !reuse_destination && let Some(destination_metadata) = destination_metadata {
            remove_path(&entry_destination, destination_metadata.file_type())?;
        }

        if entry_type.is_symlink() {
            let entry_link_target =
                fs::read_link(&entry_path).map_err(|error| fs_io_error(&entry_path, error))?;
            create_symlink(&entry_link_target, &entry_destination, entry_type)?;
        } else if entry_type.is_dir() {
            if !reuse_destination {
                fs::create_dir(&entry_destination)
                    .map_err(|error| fs_io_error(&entry_destination, error))?;
            }
            directories.push((entry_destination, entry_metadata.permissions()));
        } else {
            fs::copy(&entry_path, &entry_destination)
                .map_err(|error| fs_io_error(&entry_destination, error))?;
        }
    }

    // Read-only directory permissions must not prevent copying their children.
    for (path, permissions) in directories.into_iter().rev() {
        fs::set_permissions(&path, permissions).map_err(|error| fs_io_error(&path, error))?;
    }

    Ok(())
}

fn find_existing_ancestor(path: &AbsolutePath) -> Result<(&AbsolutePath, fs::Metadata), FsError> {
    let mut existing = path;
    loop {
        match existing.symlink_metadata() {
            Ok(metadata) => return Ok((existing, metadata)),
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::NotFound | io::ErrorKind::NotADirectory
                ) =>
            {
                existing = existing
                    .parent()
                    .ok_or_else(|| fs_io_error(existing, error))?;
            }
            Err(error) => return Err(fs_io_error(existing, error)),
        }
    }
}

fn follow_symlink(path: &Path, metadata: fs::Metadata) -> Result<fs::Metadata, FsError> {
    if !metadata.is_symlink() {
        return Ok(metadata);
    }
    path.metadata().map_err(|error| match error.kind() {
        io::ErrorKind::NotFound | io::ErrorKind::NotADirectory => {
            FsError::BrokenSymlink(path.to_owned())
        }
        _ => fs_io_error(path, error),
    })
}
