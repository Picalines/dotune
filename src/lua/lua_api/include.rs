use super::super::lua_exec::load_script;
use super::utils::bad_argument;
use mlua::prelude::*;
use std::path::{Component, Path};

pub(super) fn create_include(
    lua: &Lua,
    script_path: &Path,
    dir_script_name: &Path,
) -> LuaResult<LuaFunction> {
    let script_dir = script_path
        .parent()
        .expect("script should be in some directory")
        .to_owned();

    let dir_script_name = dir_script_name.to_owned();

    lua.create_function(move |lua, relative_path: String| {
        if !is_include_path(&relative_path) {
            return Err(bad_argument(
                "include",
                0,
                Some("path"),
                "must be a relative path without indirection",
            ));
        }

        let file_path = script_dir.join(relative_path).join(&dir_script_name);
        load_script(lua, &file_path, &dir_script_name)?.call::<LuaValue>(())
    })
}

fn is_include_path(relative_path: &str) -> bool {
    !relative_path.is_empty()
        && Path::new(relative_path)
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
}
