use std::{
    ffi::OsStr,
    io,
    ops::Deref,
    path::{Component, Path, PathBuf},
};

/// An absolute normalized path. Gives guarantee only about the string - symlinks and existence is
/// checked elsewhere. Yes, it'a a bad name - let me have fun
#[derive(Clone, Debug)]
pub struct SimplePath(PathBuf);

impl SimplePath {
    pub fn new(path: PathBuf) -> Option<Self> {
        (path.is_absolute() && Self::all_components_are_simple(&path)).then(|| Self(path))
    }

    pub fn canonicalize(path: impl AsRef<Path>) -> io::Result<Self> {
        let canonical_path = std::fs::canonicalize(path)?;

        Self::unreachable_path(&canonical_path);

        Ok(Self(canonical_path))
    }

    pub fn to_path_buf(self) -> PathBuf {
        self.0
    }

    // NOTE: not optimal, but we don't need ref parent currently
    pub fn owned_parent(&self) -> Option<Self> {
        self.0.parent().map(|parent| {
            Self::unreachable_path(parent);
            Self(parent.to_owned())
        })
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

    pub fn join(&self, path: impl AsRef<Path>) -> SimplePath {
        let mut buf = self.clone();
        buf.push(path);
        buf
    }

    pub fn with_file_name(&self, file_name: impl AsRef<OsStr>) -> Self {
        Self(self.0.with_file_name(file_name))
    }

    pub fn with_extension(&self, extension: impl AsRef<OsStr>) -> Self {
        Self(self.0.with_extension(extension))
    }

    fn unreachable_path(path: &Path) {
        if !path.is_absolute() {
            unreachable!("canonicalize always returns an absolute path")
        }

        if !Self::all_components_are_simple(&path) {
            unreachable!("canonicalize always returns an absolute path without indirection")
        }
    }

    fn all_components_are_simple(path: &Path) -> bool {
        path.components().all(|c| Self::is_simple_component(&c))
    }

    fn is_simple_component(component: &Component) -> bool {
        matches!(
            component,
            Component::Prefix(_) | Component::RootDir | Component::Normal(_)
        )
    }
}

impl Deref for SimplePath {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> AsRef<T> for SimplePath
where
    T: ?Sized,
    <SimplePath as Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref().as_ref()
    }
}

impl mlua::FromLua for SimplePath {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        let path = PathBuf::from_lua(value, lua)?;
        SimplePath::new(path).ok_or_else(|| mlua::Error::external("lua path is not simple"))
    }
}

impl mlua::IntoLua for SimplePath {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        self.0.into_lua(lua)
    }
}
