use std::sync::Arc;

use mlua::prelude::*;

use super::path::Path;
use crate::model::{AbsolutePath, Instruction, InstructionKind};

#[derive(LuaUserData)]
pub struct Dot {
    #[lua(get)]
    link: LuaFunction,
}

impl Dot {
    pub fn new(lua: &Lua, script_path: Arc<AbsolutePath>) -> LuaResult<Self> {
        let link =
            lua.create_function(move |lua, (source_path, destination_path): (Path, Path)| {
                lua.app_data_mut::<Vec<Instruction>>()
                    .expect("instruction list should be defined before execution")
                    .push(Instruction {
                        defined_in: script_path.clone(),
                        kind: InstructionKind::Link {
                            source_path: source_path.0,
                            destination_path: destination_path.0,
                        },
                    });
                Ok(())
            })?;

        Ok(Self { link })
    }
}
