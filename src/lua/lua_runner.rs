use std::sync::Arc;

use mlua::prelude::*;

use super::lua_exec::execute_root_script;
use crate::model::{Inputs, Instruction, RepoPath, SystemInfo};

pub fn run_config_scripts(
    repo_path: &RepoPath,
    system_info: Arc<SystemInfo>,
    inputs: Arc<Inputs>,
) -> LuaResult<Vec<Instruction>> {
    let lua = Lua::new_with(
        LuaStdLib::STRING | LuaStdLib::TABLE | LuaStdLib::UTF8,
        LuaOptions::default(),
    )?;

    lua.set_app_data(Vec::<Instruction>::new());

    execute_root_script(&lua, repo_path, system_info, inputs)?;

    Ok(lua
        .remove_app_data::<Vec<Instruction>>()
        .expect("instruction list is initialized before executing scripts"))
}
