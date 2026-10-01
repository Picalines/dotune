use std::path::PathBuf;

use mlua::prelude::*;

use super::path::Path;
use super::utils::bad_argument;

#[derive(LuaUserData, Debug)]
pub struct Link {
    #[lua(skip)]
    pub path1: PathBuf,
    #[lua(skip)]
    pub path2: PathBuf,
}

#[derive(LuaUserData)]
pub struct LinkFactory;

#[mlua::userdata_impl]
impl LinkFactory {
    #[lua(meta)]
    fn __call(&self, arg: LuaTable) -> LuaResult<Link> {
        let mut arg_paths = arg.pairs::<LuaNumber, Path>();

        let (Some(Ok((_, path1))), Some(Ok((_, path2)))) = (arg_paths.next(), arg_paths.next())
        else {
            return Err(bad_argument("Link", 1, None, "expected two paths"));
        };

        Ok(Link {
            path1: path1.buf,
            path2: path2.buf,
        })
    }
}

#[mlua::userdata_impl]
impl Link {
    #[lua(meta, infallible)]
    fn __tostring(&self) -> String {
        format!("Link({}, {})", self.path1.display(), self.path2.display())
    }
}
