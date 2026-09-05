mod cli_args;
mod lua_api;
mod lua_require;

use clap::Parser;
use mlua::prelude::*;

fn main() -> LuaResult<()> {
    let args = cli_args::CliArgs::parse();

    let lua = Lua::new();
    lua.load_std_libs(LuaStdLib::ALL_SAFE)?;

    lua_require::add_package_searcher(&lua, &args)?;
    let config_table = lua_require::execute_root_script(&lua, &args)?;

    dbg!(config_table);

    Ok(())
}
