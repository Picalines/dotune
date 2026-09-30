use std::{
    io,
    ops::Deref,
    path::{Component, Path, PathBuf},
};

/// An absolute normalized path. Gives guarantee only about the string - symlinks and existence is
/// checked elsewhere. Yes, it'a a bad name - let me have fun
#[derive(Clone, Debug)]
pub struct SimplePath(PathBuf);

impl SimplePath {
    pub fn canonicalize(path: impl AsRef<Path>) -> io::Result<Self> {
        let canonical_path = std::fs::canonicalize(path)?;

        if !canonical_path.is_absolute() {
            unreachable!("canonicalize always returns an absolute path")
        }

        let all_components_are_simple = canonical_path
            .components()
            .all(|c| Self::is_simple_component(&c));

        if !all_components_are_simple {
            unreachable!("canonicalize always returns an absolute path without indirection")
        }

        Ok(Self(canonical_path))
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
