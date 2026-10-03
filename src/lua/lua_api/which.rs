use super::path::Path;
use crate::model::AbsolutePathBuf;
use mlua::prelude::*;

pub fn which(_: &Lua, path: String) -> LuaResult<Option<Path>> {
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
