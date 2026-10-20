use std::sync::Arc;

use mlua::prelude::*;

use super::lua_app_data::LuaAppData;
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

    LuaAppData::init(&lua, system_info, inputs);

    execute_root_script(&lua, repo_path)?;

    Ok(LuaAppData::take_instructions(&lua))
}
