use std::sync::Arc;

use mlua::prelude::*;

use super::dot::Dot;
use super::include::create_include;
use super::mac::Mac;
use super::os::Os;
use super::path::PathFactory;
use super::windows::Windows;
use super::xdg::Xdg;
use crate::model::{AbsolutePath, SystemInfo};

// TODO: maybe store script_path in Lua?
pub fn script_environment(
    lua: &Lua,
    script_path: Arc<AbsolutePath>,
    system_info: Arc<SystemInfo>,
) -> LuaResult<LuaTable> {
    let environment = lua.create_table()?;

    environment.set(
        "include",
        create_include(lua, &script_path, system_info.clone())?,
    )?;

    environment.set("Dot", Dot::new(lua, script_path.clone())?)?;

    environment.set(
        "Path",
        PathFactory {
            script_path,
            system_info: system_info.clone(),
        },
    )?;

    environment.set("OS", Os::new(lua, system_info.clone())?)?;
    environment.set("Windows", Windows(system_info.clone()))?;
    environment.set("Mac", Mac(system_info.clone()))?;
    environment.set("XDG", Xdg(system_info))?;

    let env_meta = lua.create_table()?;
    env_meta.set("__index", lua.globals())?;
    env_meta.set("__newindex", lua.globals())?;
    environment.set_metatable(Some(env_meta))?;

    Ok(environment)
}
