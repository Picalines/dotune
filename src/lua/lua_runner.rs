use super::lua_api::{Config, ConfigItem};
use super::lua_exec::execute_root_script;
use crate::{
    cli_args::CliArgs,
    model::{UserCommand, UserCommandKind},
};
use mlua::prelude::*;
use std::collections::VecDeque;
use std::sync::Arc;

pub fn run_config_scripts(cli_args: &CliArgs) -> LuaResult<Vec<UserCommand>> {
    let lua = Lua::new();

    lua.load_std_libs(LuaStdLib::ALL_SAFE)?;

    let root_config = execute_root_script(&lua, cli_args)?;

    let mut commands: Vec<UserCommand> = Vec::new();
    let mut config_queue: VecDeque<Arc<Config>> = VecDeque::from([root_config.into()]);

    while let Some(config) = config_queue.pop_front() {
        for config_item in config.items.iter() {
            match config_item {
                ConfigItem::ChildConfig(config) => config_queue.push_back(config.clone()),
                ConfigItem::Link(link) => commands.push(UserCommand {
                    defined_in: config.script_path.clone(),
                    kind: UserCommandKind::Link {
                        source_path: link.source_path.clone(),
                        destination_path: link.destination_path.clone(),
                    },
                }),
            }
        }
    }

    Ok(commands)
}
