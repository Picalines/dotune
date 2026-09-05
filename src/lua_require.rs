use std::path::{Component, Path};

use mlua::prelude::*;

use crate::{cli_args, lua_api};

pub fn execute_root_script(lua: &Lua, args: &cli_args::CliArgs) -> LuaResult<LuaTable> {
    let root_script_path = args.root_dir.join(&args.config_name);
    load_script(lua, &root_script_path, Path::new(""))?.call::<LuaTable>(())
}

pub fn add_package_searcher(lua: &Lua, args: &cli_args::CliArgs) -> LuaResult<()> {
    let root_dir = args.root_dir.clone();
    let init_module_name = args.config_name.clone();

    let searchers = lua
        .globals()
        .get::<LuaTable>("package")?
        .get::<LuaTable>("searchers")?;

    let searcher_func = lua.create_function(move |lua, module_name: String| {
        if !is_relative_module_name(&module_name) {
            return Err(LuaError::runtime(format!(
                "invalid module name '{module_name}'"
            )));
        }

        let script_path = root_dir.join(&module_name).join(&init_module_name);

        load_script(lua, &script_path, Path::new(&module_name)).map(LuaValue::Function)
    })?;

    searchers.raw_insert(1, searcher_func)?;

    Ok(())
}

fn load_script(lua: &Lua, file_path: &Path, module_path: &Path) -> LuaResult<LuaFunction> {
    let source_code = std::fs::read(file_path).map_err(|error| {
        LuaError::runtime(format!(
            "failed to read dotune config '{}': {error}",
            file_path.display()
        ))
    })?;

    let environment = lua_api::script_environment(lua, file_path)?;
    environment.set("require", require_relative(lua, module_path)?)?;

    lua.load(source_code)
        .set_name(file_path.to_string_lossy().as_ref())
        .set_environment(environment)
        .into_function()
}

fn require_relative(lua: &Lua, script_path: &Path) -> LuaResult<LuaFunction> {
    let root_require = lua.globals().get::<LuaFunction>("require")?;
    let script_path = script_path.to_owned();

    lua.create_function(move |_, module_name: String| {
        if !is_relative_module_name(&module_name) {
            return Err(LuaError::runtime(format!(
                "invalid module name '{module_name}'"
            )));
        }

        root_require
            .call::<LuaMultiValue>(script_path.join(module_name).to_string_lossy().into_owned())
    })
}

fn is_relative_module_name(module_name: &str) -> bool {
    let path = Path::new(module_name);
    !module_name.is_empty()
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
}
