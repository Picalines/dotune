use std::sync::Arc;

use mlua::{AppDataRef, prelude::*};

use crate::model::{Inputs, Instruction, SystemInfo};

pub struct LuaAppData {
    pub system_info: Arc<SystemInfo>,
    pub inputs: Arc<Inputs>,
    instructions: Vec<Instruction>,
}

impl LuaAppData {
    pub fn init(lua: &Lua, system_info: Arc<SystemInfo>, inputs: Arc<Inputs>) {
        lua.set_app_data(Self {
            system_info,
            inputs,
            instructions: Vec::new(),
        });
    }

    pub fn get(lua: &Lua) -> AppDataRef<'_, Self> {
        lua.app_data_ref::<Self>()
            .expect("app data is initialized before executing scripts")
    }

    pub fn push_instruction(lua: &Lua, instruction: Instruction) {
        lua.app_data_mut::<Self>()
            .expect("app data is initialized before executing scripts")
            .instructions
            .push(instruction);
    }

    pub fn take_instructions(lua: &Lua) -> Vec<Instruction> {
        lua.remove_app_data::<Self>()
            .expect("app data is initialized before executing scripts")
            .instructions
    }
}
