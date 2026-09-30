use std::path::{Component, Path};

use mlua::prelude::*;

use super::lua_api::script_environment;
use crate::cli_args;

pub fn execute_root_script(lua: &Lua, args: &cli_args::CliArgs) -> LuaResult<LuaTable> {
    let root_script_path = args.repo_path.join(&args.dir_script_name);
    load_script(lua, &root_script_path, &args.dir_script_name)?.call::<LuaTable>(())
}

fn load_script(lua: &Lua, file_path: &Path, dir_script_name: &Path) -> LuaResult<LuaFunction> {
    let script_dir = file_path
        .parent()
        .expect("script should be in some directory");

    let source_code = std::fs::read(file_path).map_err(|error| {
        LuaError::runtime(format!("could not read '{}': {error}", file_path.display()))
    })?;

    let environment = script_environment(lua, file_path)?;
    environment.set(
        "include",
        include_relative(lua, script_dir, dir_script_name)?,
    )?;

    lua.load(source_code)
        .set_name(format!("@{}", file_path.display()))
        .set_environment(environment)
        .into_function()
}

fn include_relative(
    lua: &Lua,
    script_dir: &Path,
    dir_script_name: &Path,
) -> LuaResult<LuaFunction> {
    let script_dir = script_dir.to_owned();
    let dir_script_name = dir_script_name.to_owned();

    lua.create_function(move |lua, relative_path: String| {
        if !is_include_path(&relative_path) {
            return Err(LuaError::runtime(format!(
                "invalid include path '{relative_path}'"
            )));
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
