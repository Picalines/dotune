use mlua::prelude::*;

use crate::cli_args::CliArgs;

use super::lua_require::add_package_searcher;
use super::lua_require::execute_root_script;

pub fn run_config_scripts(cli_args: &CliArgs) -> LuaResult<()> {
    let lua = Lua::new();

    lua.load_std_libs(LuaStdLib::ALL_SAFE)?;

    add_package_searcher(&lua, &cli_args)?;
    let config_table = execute_root_script(&lua, &cli_args)?;

    dbg!(config_table);

    Ok(())
}
