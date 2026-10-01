use super::utils::bad_argument;
use crate::shared::SimplePath;
use itertools::Itertools;
use mlua::prelude::*;
use std::path::{Component, Path as StdPath};

#[derive(LuaUserData, FromLua, Clone)]
pub struct Path(#[lua(skip)] pub SimplePath);

#[derive(LuaUserData)]
pub struct PathFactory {
    pub script_path: SimplePath,
}

#[mlua::userdata_impl]
impl PathFactory {
    #[lua(getter, infallible)]
    fn script(&self) -> Path {
        Path(self.script_path.clone())
    }

    #[lua(getter)]
    fn script_dir(&self) -> LuaResult<Path> {
        self.script_path
            .owned_parent()
            .map(Path)
            .ok_or_else(|| LuaError::runtime(""))
    }

    #[lua(getter)]
    fn here(&self) -> LuaResult<Path> {
        self.script_dir()
    }

    #[lua(getter)]
    fn home(&self) -> LuaResult<Path> {
        // TODO: cache
        let home_path = std::env::home_dir()
            .ok_or_else(|| LuaError::external("could not determine the home directory"))?;
        SimplePath::canonicalize(home_path)
            .map(Path)
            .map_err(LuaError::external)
    }

    #[lua]
    fn which(path: &str) -> LuaResult<Option<Path>> {
        match which::which(path) {
            Err(which::Error::CannotFindBinaryPath) => Ok(None),
            Err(error) => Err(LuaError::external(error)),
            Ok(bin_path) => Ok(Some(Path(
                SimplePath::new(bin_path).expect("which should return an absolute path"),
            ))),
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
        self.0.owned_parent().map(Self)
    }

    #[lua]
    fn join(&self, relative_path: &str) -> LuaResult<Self> {
        let mut buf = self.0.clone();

        for component in StdPath::new(relative_path).components() {
            buf.push(component);
        }

        Ok(Self(buf))
    }

    #[lua]
    fn with_tail(&self, tail: &str) -> LuaResult<Self> {
        let Ok(Component::Normal(normal_tail)) = StdPath::new(tail).components().exactly_one()
        else {
            return Err(bad_argument(
                "Path.with_tail",
                1,
                None,
                "normal path component expected",
            ));
        };

        Ok(Self(self.0.with_file_name(normal_tail)))
    }

    #[lua]
    fn with_extension(&self, extension: &str) -> LuaResult<Self> {
        let Ok(Component::Normal(normal_extension)) =
            StdPath::new(extension).components().exactly_one()
        else {
            return Err(bad_argument(
                "Path.with_extension",
                1,
                None,
                "normal path component expected",
            ));
        };

        Ok(Self(self.0.with_extension(normal_extension)))
    }

    #[lua]
    fn exists(&self) -> LuaResult<bool> {
        self.0.try_exists().map_err(LuaError::external)
    }

    #[lua(infallible)]
    fn resolve(&self) -> Option<Self> {
        SimplePath::canonicalize(&self.0).map(Self).ok()
    }

    #[lua(meta, infallible)]
    fn __tostring(&self) -> String {
        self.0.to_string_lossy().to_string()
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
