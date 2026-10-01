use std::path::Path;

use mlua::prelude::*;

use super::lua_api::script_environment;
use crate::cli_args;

pub fn execute_root_script(lua: &Lua, args: &cli_args::CliArgs) -> LuaResult<LuaTable> {
    let root_script_path = args.repo_path.join(&args.dir_script_name);
    load_script(lua, &root_script_path, &args.dir_script_name)?.call::<LuaTable>(())
}

pub(super) fn load_script(
    lua: &Lua,
    file_path: &Path,
    dir_script_name: &Path,
) -> LuaResult<LuaFunction> {
    let source_code = std::fs::read(file_path).map_err(|error| {
        LuaError::runtime(format!("could not read '{}': {error}", file_path.display()))
    })?;

    let environment = script_environment(lua, file_path, dir_script_name)?;

    lua.load(source_code)
        .set_name(format!("@{}", file_path.display()))
        .set_environment(environment)
        .into_function()
}
