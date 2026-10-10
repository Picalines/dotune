use std::collections::VecDeque;
use std::sync::Arc;

use mlua::prelude::*;

use super::lua_api::{Config, ConfigItem};
use super::lua_exec::execute_root_script;
use crate::cli;
use crate::model::{Instruction, InstructionKind};

pub fn run_config_scripts(cli_args: &cli::Args) -> LuaResult<Vec<Instruction>> {
    let lua = Lua::new();

    lua.load_std_libs(LuaStdLib::ALL_SAFE)?;

    let root_config = execute_root_script(&lua, cli_args)?;

    let mut instructions: Vec<Instruction> = Vec::new();
    let mut config_queue: VecDeque<Arc<Config>> = VecDeque::from([root_config.into()]);

    while let Some(config) = config_queue.pop_front() {
        for config_item in config.items.iter() {
            match config_item {
                ConfigItem::ChildConfig(config) => config_queue.push_back(config.clone()),
                ConfigItem::Link(link) => instructions.push(Instruction {
                    defined_in: config.script_path.clone(),
                    kind: InstructionKind::Link {
                        source_path: link.source_path.clone(),
                        destination_path: link.destination_path.clone(),
                    },
                }),
            }
        }
    }

    Ok(instructions)
}
