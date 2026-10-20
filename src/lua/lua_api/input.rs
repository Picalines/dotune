use mlua::prelude::*;
use strum::{EnumString, VariantNames};

use super::super::lua_app_data::LuaAppData;
use super::path::Path;
use super::utils::bad_argument;

#[derive(LuaUserData)]
pub struct Input {
    #[lua(get)]
    param: LuaFunction,
}

#[derive(EnumString, VariantNames)]
#[strum(serialize_all = "lowercase")]
enum InputType {
    String,
    Path,
}

impl Input {
    pub fn new(lua: &Lua) -> LuaResult<Self> {
        let param = lua.create_function(|lua, (name, kind): (String, String)| {
            let kind = kind.parse::<InputType>().map_err(|_| {
                bad_argument(
                    "Input.param",
                    2,
                    Some("type"),
                    &format!(
                        "unknown input type '{kind}'; expected one of: {}",
                        InputType::VARIANTS.join(", ")
                    ),
                )
            })?;

            let app_data = LuaAppData::get(lua);
            let inputs = &app_data.inputs;

            match kind {
                InputType::String => inputs
                    .string(&name)
                    .map_err(LuaError::external)?
                    .into_lua(lua),
                InputType::Path => {
                    Path(inputs.path(&name).map_err(LuaError::external)?.into()).into_lua(lua)
                }
            }
        })?;

        Ok(Self { param })
    }
}
