use mlua::prelude::*;

use crate::cli_args::CliArgs;

use super::lua_exec::execute_root_script;

pub fn run_config_scripts(cli_args: &CliArgs) -> LuaResult<()> {
    let lua = Lua::new();

    lua.load_std_libs(LuaStdLib::ALL_SAFE)?;

    let lua_config = execute_root_script(&lua, cli_args)?;

    dbg!(lua_config);

    Ok(())
}
