use std::sync::Arc;

use mlua::prelude::*;

use super::dot::Dot;
use super::include::create_include;
use super::path::PathFactory;
use crate::model::AbsolutePath;

// TODO: maybe store script_path in Lua?
pub fn script_environment(lua: &Lua, script_path: Arc<AbsolutePath>) -> LuaResult<LuaTable> {
    let environment = lua.create_table()?;

    environment.set("include", create_include(lua, &script_path)?)?;

    environment.set("Dot", Dot::new(lua, script_path.clone())?)?;

    environment.set("Path", PathFactory { script_path })?;

    let env_meta = lua.create_table()?;
    env_meta.set("__index", lua.globals())?;
    env_meta.set("__newindex", lua.globals())?;
    environment.set_metatable(Some(env_meta))?;

    Ok(environment)
}
