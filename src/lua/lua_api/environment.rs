use super::include::create_include;
use super::link::LinkFactory;
use super::path::PathFactory;
use super::which::which;
use crate::model::AbsolutePath;
use mlua::prelude::*;
use std::sync::Arc;

pub fn script_environment(lua: &Lua, script_path: Arc<AbsolutePath>) -> LuaResult<LuaTable> {
    let environment = lua.create_table()?;

    environment.set("include", create_include(lua, &script_path)?)?;
    environment.set("which", lua.create_function(which)?)?;

    environment.set("Path", PathFactory { script_path })?;
    environment.set("Link", LinkFactory)?;

    let env_meta = lua.create_table()?;
    env_meta.set("__index", lua.globals())?;
    environment.set_metatable(Some(env_meta))?;

    Ok(environment)
}
