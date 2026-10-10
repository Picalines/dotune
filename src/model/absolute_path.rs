use core::borrow::Borrow;
use core::ops::Deref;
use std::ffi::OsStr;
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;
use std::{fs, io};

/// An absolute normalized path. Gives guarantee only about the string -
/// symlinks and existence is checked elsewhere
#[repr(transparent)]
#[derive(Debug)]
pub struct AbsolutePath(Path);

impl AbsolutePath {
    pub fn new(path: &Path) -> Option<&Self> {
        (path.is_absolute()
            && path.components().all(|component| {
                matches!(
                    component,
                    Component::Prefix(_) | Component::RootDir | Component::Normal(_)
                )
            }))
        .then(|| Self::from_path(path))
    }

    pub fn parent(&self) -> Option<&Self> {
        self.0.parent().map(Self::from_path)
    }

    pub fn join(&self, path: impl AsRef<Path>) -> AbsolutePathBuf {
        let mut buf = self.to_owned();
        buf.push(path);
        buf
    }

    pub fn with_file_name(&self, file_name: impl AsRef<OsStr>) -> AbsolutePathBuf {
        AbsolutePathBuf(self.0.with_file_name(file_name))
    }

    pub fn with_extension(&self, extension: impl AsRef<OsStr>) -> AbsolutePathBuf {
        AbsolutePathBuf(self.0.with_extension(extension))
    }

    fn from_path(path: &Path) -> &Self {
        // SAFETY: same as for Path in PathBuf in std, valid as long as layout of Self
        // and Path are same. Keep repr(transparent) for that to be true
        unsafe { &*(path as *const Path as *const Self) }
    }
}

impl Deref for AbsolutePath {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToOwned for AbsolutePath {
    type Owned = AbsolutePathBuf;

    fn to_owned(&self) -> Self::Owned {
        AbsolutePathBuf(self.0.to_owned())
    }
}

impl AsRef<Path> for AbsolutePath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl AsRef<AbsolutePath> for AbsolutePath {
    fn as_ref(&self) -> &AbsolutePath {
        self
    }
}

impl From<&AbsolutePath> for Arc<AbsolutePath> {
    fn from(path: &AbsolutePath) -> Self {
        let path: Arc<Path> = Arc::from(&path.0);
        let ptr = Arc::into_raw(path) as *const AbsolutePath;
        unsafe { Arc::from_raw(ptr) }
    }
}

/// Owned version of [`AbsolutePath`]
#[derive(Clone, Debug)]
pub struct AbsolutePathBuf(PathBuf);

impl AbsolutePathBuf {
    pub fn new(path: PathBuf) -> Option<Self> {
        AbsolutePath::new(&path)?;
        Some(Self(path))
    }

    pub fn canonicalize(path: impl AsRef<Path>) -> io::Result<Self> {
        let canonical_path = fs::canonicalize(path)?;
        Ok(Self::new(canonical_path)
            .expect("canonicalize always returns an absolute path without indirection"))
    }

    pub fn push(&mut self, path: impl AsRef<Path>) {
        for component in path.as_ref().components() {
            match component {
                Component::Prefix(_) | Component::RootDir | Component::Normal(_) => {
                    self.0.push(component);
                }
                Component::CurDir => {}
                Component::ParentDir => {
                    self.0.pop();
                }
            }
        }
    }
}

impl Deref for AbsolutePathBuf {
    type Target = AbsolutePath;

    fn deref(&self) -> &Self::Target {
        AbsolutePath::from_path(&self.0)
    }
}

impl Borrow<AbsolutePath> for AbsolutePathBuf {
    fn borrow(&self) -> &AbsolutePath {
        self.deref()
    }
}

impl<T> AsRef<T> for AbsolutePathBuf
where
    T: ?Sized,
    <AbsolutePathBuf as Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref().as_ref()
    }
}

impl From<AbsolutePathBuf> for PathBuf {
    fn from(path: AbsolutePathBuf) -> Self {
        path.0
    }
}

impl From<AbsolutePathBuf> for Arc<AbsolutePath> {
    fn from(path: AbsolutePathBuf) -> Self {
        Self::from(path.deref())
    }
}

impl mlua::FromLua for AbsolutePathBuf {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        let path = PathBuf::from_lua(value, lua)?;
        AbsolutePathBuf::new(path).ok_or_else(|| mlua::Error::external("lua path is not absolute"))
    }
}

impl mlua::IntoLua for AbsolutePathBuf {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        self.0.into_lua(lua)
    }
}
