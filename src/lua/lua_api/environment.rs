use super::include::create_include;
use super::link::LinkFactory;
use super::path::PathFactory;
use crate::shared::AbsolutePathBuf;
use mlua::prelude::*;

pub fn script_environment(lua: &Lua, script_path: &AbsolutePathBuf) -> LuaResult<LuaTable> {
    let environment = lua.create_table()?;

    environment.set(
        "Path",
        PathFactory {
            script_path: script_path.clone(),
        },
    )?;

    environment.set("Link", LinkFactory)?;

    environment.set("include", create_include(lua, script_path)?)?;

    let env_meta = lua.create_table()?;
    env_meta.set("__index", lua.globals())?;
    environment.set_metatable(Some(env_meta))?;

    Ok(environment)
}
