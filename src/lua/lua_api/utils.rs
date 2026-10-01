use std::sync::Arc;

use mlua::prelude::*;

pub fn bad_argument(to_func: &str, pos: usize, name: Option<&str>, message: &str) -> LuaError {
    LuaError::BadArgument {
        to: Some(to_func.to_string()),
        pos,
        name: name.map(ToOwned::to_owned),
        cause: Arc::new(message.into_lua_err()),
    }
}
