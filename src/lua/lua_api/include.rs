use std::path::{Component, Path};
use std::sync::Arc;

use mlua::prelude::*;

use super::super::lua_exec::{CONFIG_FILE_NAME, execute_config_script};
use super::utils::bad_argument;
use crate::model::{AbsolutePath, Inputs, SystemInfo};

pub(super) fn create_include(
    lua: &Lua,
    script_path: &AbsolutePath,
    system_info: Arc<SystemInfo>,
    inputs: Arc<Inputs>,
) -> LuaResult<LuaFunction> {
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

        let mut child_script_path = script_dir.clone();
        child_script_path.push(relative_path);
        child_script_path.push(CONFIG_FILE_NAME);

        execute_config_script(
            lua,
            child_script_path.into(),
            system_info.clone(),
            inputs.clone(),
        )
    })
}

fn is_include_path(relative_path: &str) -> bool {
    !relative_path.is_empty()
        && Path::new(relative_path)
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
}
