use super::lua_api::Config;
use super::lua_api::script_environment;
use crate::cli;
use crate::model::AbsolutePath;
use mlua::prelude::*;
use std::fs;
use std::sync::Arc;

pub const CONFIG_FILE_NAME: &str = ".dotune.lua";

pub fn execute_root_script(lua: &Lua, args: &cli::Args) -> LuaResult<Config> {
    let root_script_path = args.repo_path.join(CONFIG_FILE_NAME);
    execute_config_script(lua, root_script_path.into())
}

pub(super) fn execute_config_script(
    lua: &Lua,
    script_path: Arc<AbsolutePath>,
) -> LuaResult<Config> {
    let script_fn = load_script(lua, script_path.clone())?;
    let items_table: LuaTable = script_fn.call(())?;
    let items = Config::items(items_table)?;
    Ok(Config { script_path, items })
}

fn load_script(lua: &Lua, file_path: Arc<AbsolutePath>) -> LuaResult<LuaFunction> {
    let source_code = fs::read(file_path.as_ref()).map_err(|error| {
        LuaError::external(format!("could not read '{}': {error}", file_path.display()))
    })?;

    let chunk_name = format!("@{}", file_path.display());
    let environment = script_environment(lua, file_path)?;

    lua.load(source_code)
        .set_name(chunk_name)
        .set_environment(environment)
        .into_function()
}
