use std::fs;
use std::sync::Arc;

use mlua::prelude::*;

use super::lua_api::script_environment;
use crate::model::{AbsolutePath, Inputs, RepoPath, SystemInfo};

pub const CONFIG_FILE_NAME: &str = ".dotune.lua";

pub fn execute_root_script(
    lua: &Lua,
    repo_path: &RepoPath,
    system_info: Arc<SystemInfo>,
    inputs: Arc<Inputs>,
) -> LuaResult<()> {
    let root_script_path = repo_path.join(CONFIG_FILE_NAME);
    execute_config_script(lua, root_script_path.into(), system_info, inputs)
}

pub(super) fn execute_config_script(
    lua: &Lua,
    script_path: Arc<AbsolutePath>,
    system_info: Arc<SystemInfo>,
    inputs: Arc<Inputs>,
) -> LuaResult<()> {
    let script_fn = load_script(lua, script_path, system_info, inputs)?;
    script_fn.call(())
}

fn load_script(
    lua: &Lua,
    file_path: Arc<AbsolutePath>,
    system_info: Arc<SystemInfo>,
    inputs: Arc<Inputs>,
) -> LuaResult<LuaFunction> {
    let source_code = fs::read(file_path.as_ref()).map_err(|error| {
        LuaError::external(format!("could not read '{}': {error}", file_path.display()))
    })?;

    let chunk_name = format!("@{}", file_path.display());
    let environment = script_environment(lua, file_path, system_info, inputs)?;

    lua.load(source_code)
        .set_name(chunk_name)
        .set_environment(environment)
        .into_function()
}
