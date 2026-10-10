use mlua::prelude::*;

use super::lua_exec::execute_root_script;
use crate::cli;
use crate::model::Instruction;

pub fn run_config_scripts(cli_args: &cli::Args) -> LuaResult<Vec<Instruction>> {
    let lua = Lua::new();

    lua.load_std_libs(LuaStdLib::ALL_SAFE)?;

    lua.set_app_data(Vec::<Instruction>::new());

    execute_root_script(&lua, cli_args)?;

    Ok(lua
        .remove_app_data::<Vec<Instruction>>()
        .expect("instruction list is initialized before executing scripts"))
}
