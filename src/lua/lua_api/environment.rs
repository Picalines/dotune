use std::sync::Arc;

use mlua::prelude::*;

use super::dot::Dot;
use super::include::create_include;
use super::input::Input;
use super::mac::Mac;
use super::os::Os;
use super::path::PathFactory;
use super::windows::Windows;
use super::xdg::Xdg;
use crate::model::AbsolutePath;

pub fn script_environment(lua: &Lua, script_path: Arc<AbsolutePath>) -> LuaResult<LuaTable> {
    let environment = lua.create_table()?;

    environment.set("include", create_include(lua, &script_path)?)?;

    environment.set("Dot", Dot::new(lua, script_path.clone())?)?;
    environment.set("Input", Input::new(lua)?)?;

    environment.set("Path", PathFactory { script_path })?;

    environment.set("OS", Os::new(lua)?)?;
    environment.set("Windows", Windows)?;
    environment.set("Mac", Mac)?;
    environment.set("XDG", Xdg)?;

    let env_meta = lua.create_table()?;
    env_meta.set("__index", lua.globals())?;
    env_meta.set("__newindex", lua.globals())?;
    environment.set_metatable(Some(env_meta))?;

    Ok(environment)
}
