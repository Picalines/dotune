use std::sync::Arc;

use mlua::prelude::*;

use super::path::Path;
use crate::model::{AbsolutePath, Instruction, InstructionKind};

#[derive(LuaUserData)]
pub struct Dot {
    #[lua(get)]
    link: LuaFunction,
    #[lua(get)]
    copy: LuaFunction,
}

impl Dot {
    pub fn new(lua: &Lua, script_path: Arc<AbsolutePath>) -> LuaResult<Self> {
        let link_script_path = script_path.clone();
        let link =
            lua.create_function(move |lua, (source_path, destination_path): (Path, Path)| {
                Self::push_instruction(
                    lua,
                    Instruction {
                        defined_in: link_script_path.clone(),
                        kind: InstructionKind::Link {
                            source_path: source_path.0,
                            destination_path: destination_path.0,
                        },
                    },
                );
                Ok(())
            })?;

        let copy =
            lua.create_function(move |lua, (source_path, destination_path): (Path, Path)| {
                Self::push_instruction(
                    lua,
                    Instruction {
                        defined_in: script_path.clone(),
                        kind: InstructionKind::Copy {
                            source_path: source_path.0,
                            destination_path: destination_path.0,
                        },
                    },
                );
                Ok(())
            })?;

        Ok(Self { link, copy })
    }

    fn push_instruction(lua: &Lua, instruction: Instruction) {
        lua.app_data_mut::<Vec<Instruction>>()
            .expect("instruction list should be defined before execution")
            .push(instruction);
    }
}
