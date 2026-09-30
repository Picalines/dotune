use mlua::prelude::*;
use std::path::Path as StdPath;

use super::link::LinkFactory;
use super::path::PathFactory;

pub fn script_environment(lua: &Lua, script_path: &StdPath) -> LuaResult<LuaTable> {
    let environment = lua.create_table()?;

    environment.set(
        "Path",
        PathFactory {
            script_path: script_path.to_owned(),
        },
    )?;

    environment.set("Link", LinkFactory)?;

    let env_meta = lua.create_table()?;
    env_meta.set("__index", lua.globals())?;
    environment.set_metatable(Some(env_meta))?;

    Ok(environment)
}
