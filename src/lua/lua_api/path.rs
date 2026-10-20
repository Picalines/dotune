use std::path::{Component, Path as StdPath};
use std::sync::Arc;

use itertools::Itertools;
use mlua::prelude::*;

use super::super::lua_app_data::LuaAppData;
use super::utils::bad_argument;
use crate::model::{AbsolutePath, AbsolutePathBuf};

#[derive(LuaUserData, FromLua, Clone)]
pub struct Path(#[lua(skip)] pub Arc<AbsolutePath>);

#[derive(LuaUserData)]
pub struct PathFactory {
    #[lua(skip)]
    pub script_path: Arc<AbsolutePath>,
}

#[mlua::userdata_impl]
impl PathFactory {
    #[lua(getter, infallible)]
    fn script(&self) -> Path {
        Path(self.script_path.clone())
    }

    #[lua(getter, infallible)]
    fn script_dir(&self) -> Path {
        self.script_path
            .parent()
            .map(|parent| Path(parent.into()))
            .expect("script path always lives in some directory")
    }

    #[lua(getter, infallible)]
    fn here(&self) -> Path {
        self.script_dir()
    }

    #[lua(getter, infallible)]
    fn home(&self, lua: &Lua) -> Path {
        Path(LuaAppData::get(lua).system_info.home.clone())
    }

    #[lua(getter, infallible)]
    fn root(&self, lua: &Lua) -> Path {
        Path(LuaAppData::get(lua).system_info.root.clone())
    }

    #[lua]
    fn which(path: &str) -> LuaResult<Option<Path>> {
        match which::which(path) {
            Err(which::Error::CannotFindBinaryPath) => Ok(None),
            Err(error) => Err(LuaError::external(error)),
            Ok(bin_path) => Ok(Some(Path(
                AbsolutePathBuf::new(bin_path)
                    .expect("which should return an absolute path")
                    .into(),
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
        self.0.parent().map(|parent| Self(parent.into()))
    }

    #[lua]
    fn join(&self, relative_path: &str) -> LuaResult<Self> {
        Ok(Self(self.0.join(relative_path).into()))
    }

    #[lua]
    fn to(&self, dir: &Path) -> LuaResult<(Self, Self)> {
        let file_name = self.0.file_name().ok_or_else(|| {
            bad_argument("Path.to", 1, Some("self"), "path with a file name expected")
        })?;

        Ok((self.clone(), Self(dir.0.join(file_name).into())))
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

        Ok(Self(self.0.with_file_name(normal_tail).into()))
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

        Ok(Self(self.0.with_extension(normal_extension).into()))
    }

    #[lua]
    fn exists(&self) -> LuaResult<bool> {
        self.0.try_exists().map_err(LuaError::external)
    }

    #[lua(infallible)]
    fn resolve(&self) -> Option<Self> {
        AbsolutePathBuf::canonicalize(self.0.as_ref())
            .map(|path| Self(path.into()))
            .ok()
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
