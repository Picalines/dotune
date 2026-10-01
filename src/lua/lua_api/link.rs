use std::path::PathBuf;

use mlua::prelude::*;

use super::path::Path;
use super::utils::bad_argument;

#[derive(LuaUserData, Clone, Debug)]
pub struct Link(#[lua(skip)] pub PathBuf, #[lua(skip)] pub PathBuf);

#[derive(LuaUserData)]
pub struct LinkFactory;

#[mlua::userdata_impl]
impl LinkFactory {
    #[lua(meta)]
    fn __call(&self, arg: LuaTable) -> LuaResult<Link> {
        let mut arg_paths = arg.sequence_values::<Path>();

        let (Some(Ok(path1)), Some(Ok(path2))) = (arg_paths.next(), arg_paths.next()) else {
            return Err(bad_argument("Link", 1, None, "expected two paths"));
        };

        Ok(Link(path1.0.to_path_buf(), path2.0.to_path_buf()))
    }
}

#[mlua::userdata_impl]
impl Link {
    #[lua(meta, infallible)]
    fn __tostring(&self) -> String {
        format!("Link({}, {})", self.0.display(), self.1.display())
    }
}
