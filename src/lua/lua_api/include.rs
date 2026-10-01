use super::super::lua_exec::CONFIG_FILE_NAME;
use super::super::lua_exec::execute_config_script;
use super::utils::bad_argument;
use crate::shared::AbsolutePath;
use mlua::prelude::*;
use std::{
    path::{Component, Path},
    sync::Arc,
};

pub(super) fn create_include(lua: &Lua, script_path: &AbsolutePath) -> LuaResult<LuaFunction> {
    let script_dir = script_path
        .parent()
        .expect("script should be in some directory")
        .to_owned();

    lua.create_function(move |lua, relative_path: String| {
        if !is_include_path(&relative_path) {
            return Err(bad_argument(
                "include",
                0,
                Some("path"),
                "must be a relative path without indirection",
            ));
        }

        let mut script_path = script_dir.clone();
        script_path.push(relative_path);
        script_path.push(CONFIG_FILE_NAME);

        let config = execute_config_script(lua, &script_path)?;

        Ok(Arc::new(config))
    })
}

fn is_include_path(relative_path: &str) -> bool {
    !relative_path.is_empty()
        && Path::new(relative_path)
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
}
