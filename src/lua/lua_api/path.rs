use std::path::{Component as StdPathComponent, Path as StdPath, PathBuf};

use itertools::Itertools;
use mlua::prelude::*;

use super::utils::bad_argument;

#[derive(LuaUserData, FromLua, Clone)]
pub struct Path {
    pub buf: PathBuf,
}

#[derive(LuaUserData)]
pub struct PathFactory {
    pub script_path: PathBuf,
}

#[mlua::userdata_impl]
impl PathFactory {
    #[lua(getter, infallible)]
    fn script(&self) -> Path {
        Path {
            buf: self.script_path.clone(),
        }
    }

    #[lua(getter)]
    fn script_dir(&self) -> LuaResult<Path> {
        self.script_path
            .parent()
            .map(|script_dir| Path {
                buf: script_dir.to_owned(),
            })
            .ok_or_else(|| LuaError::runtime(""))
    }

    #[lua(getter)]
    fn here(&self) -> LuaResult<Path> {
        self.script_dir()
    }

    #[lua(getter)]
    fn home(&self) -> LuaResult<Path> {
        std::env::home_dir()
            .map(|buf| Path { buf })
            .ok_or_else(|| LuaError::runtime("could not determine the home directory"))
    }

    #[lua]
    fn which(path: &str) -> LuaResult<Option<Path>> {
        match which::which(path) {
            Err(which::Error::CannotFindBinaryPath) => Ok(None),
            Err(error) => Err(LuaError::external(error)),
            Ok(bin_path) => Ok(Some(Path { buf: bin_path })),
        }
    }

    #[lua(meta, infallible)]
    fn __tostring(&self) -> String {
        "<Path>".to_string()
    }
}

#[mlua::userdata_impl]
impl Path {
    #[lua(getter, infallible)]
    fn parent(&self) -> Option<Self> {
        self.buf.parent().map(|parent_buf| Self {
            buf: parent_buf.to_owned(),
        })
    }

    #[lua]
    fn join(&self, relative_path: &str) -> LuaResult<Self> {
        let mut buf = self.buf.clone();
        let bad_relative_path = |message: &str| bad_argument("Path:join", 1, None, message);

        for component in StdPath::new(relative_path).components() {
            match component {
                StdPathComponent::Prefix(_) | StdPathComponent::RootDir => {
                    return Err(bad_relative_path("cannot append an absolute path"));
                }
                StdPathComponent::CurDir => { /* no-op */ }
                component => buf.push(component),
            }
        }

        Ok(Self { buf })
    }

    #[lua]
    fn with_tail(&self, tail: &str) -> LuaResult<Self> {
        let Ok(StdPathComponent::Normal(normal_tail)) =
            StdPath::new(tail).components().exactly_one()
        else {
            return Err(bad_argument(
                "Path.with_tail",
                1,
                None,
                "normal path component expected",
            ));
        };

        Ok(Self {
            buf: self.buf.with_file_name(normal_tail),
        })
    }

    #[lua]
    fn with_extension(&self, extension: &str) -> LuaResult<Self> {
        let Ok(StdPathComponent::Normal(normal_extension)) =
            StdPath::new(extension).components().exactly_one()
        else {
            return Err(bad_argument(
                "Path.with_extension",
                1,
                None,
                "normal path component expected",
            ));
        };

        Ok(Self {
            buf: self.buf.with_extension(normal_extension),
        })
    }

    #[lua]
    fn exists(&self) -> LuaResult<bool> {
        self.buf.try_exists().map_err(LuaError::external)
    }

    #[lua(infallible)]
    fn resolve(&self) -> Option<Self> {
        self.buf
            .canonicalize()
            .map(|canonical_buf| Self { buf: canonical_buf })
            .ok()
    }

    #[lua(meta, infallible)]
    fn __tostring(&self) -> String {
        self.buf.to_string_lossy().to_string()
    }

    #[lua(meta)]
    fn __div(&self, part: &str) -> LuaResult<Self> {
        self.join(part)
    }

    #[lua(meta)]
    fn __mod(&self, tail: &str) -> LuaResult<Self> {
        self.with_tail(tail)
    }

    #[lua(meta)]
    fn __mul(&self, extension: &str) -> LuaResult<Self> {
        self.with_extension(extension)
    }
}
