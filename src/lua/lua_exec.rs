use super::Config;
use super::lua_api::script_environment;
use crate::cli_args;
use crate::model::AbsolutePath;
use mlua::prelude::*;

pub const CONFIG_FILE_NAME: &str = ".dotune.lua";

pub fn execute_root_script(lua: &Lua, args: &cli_args::CliArgs) -> LuaResult<Config> {
    let root_script_path = args.repo_path.join(CONFIG_FILE_NAME);
    execute_config_script(lua, &root_script_path)
}

pub(super) fn execute_config_script(lua: &Lua, script_path: &AbsolutePath) -> LuaResult<Config> {
    let script_fn = load_script(lua, script_path)?;
    let items_table: LuaTable = script_fn.call(())?;
    let items = Config::items(items_table)?;
    Ok(Config {
        script_path: script_path.to_owned(),
        items,
    })
}

fn load_script(lua: &Lua, file_path: &AbsolutePath) -> LuaResult<LuaFunction> {
    let source_code = std::fs::read(file_path).map_err(|error| {
        LuaError::runtime(format!("could not read '{}': {error}", file_path.display()))
    })?;

    let environment = script_environment(lua, file_path)?;

    lua.load(source_code)
        .set_name(format!("@{}", file_path.display()))
        .set_environment(environment)
        .into_function()
}
