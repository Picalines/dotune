use std::sync::Arc;

use mlua::prelude::*;

use super::include::create_include;
use super::link::LinkFactory;
use super::path::PathFactory;
use super::which::which;
use crate::model::AbsolutePath;

pub fn script_environment(lua: &Lua, script_path: Arc<AbsolutePath>) -> LuaResult<LuaTable> {
    let environment = lua.create_table()?;

    environment.set("include", create_include(lua, &script_path)?)?;
    environment.set("which", lua.create_function(which)?)?;

    environment.set("Path", PathFactory { script_path })?;
    environment.set("Link", LinkFactory)?;

    let env_meta = lua.create_table()?;
    env_meta.set("__index", lua.globals())?;
    env_meta.set("__newindex", lua.globals())?;
    environment.set_metatable(Some(env_meta))?;

    Ok(environment)
}
