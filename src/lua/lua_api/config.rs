use super::Link;
use crate::shared::AbsolutePathBuf;
use mlua::prelude::*;
use std::sync::Arc;

#[derive(LuaUserData, Debug)]
pub struct Config {
    #[lua(skip)]
    pub script_path: AbsolutePathBuf,
    #[lua(skip)]
    pub items: Arc<[ConfigItem]>,
}

#[derive(Clone, Debug)]
pub enum ConfigItem {
    ChildConfig(Arc<Config>),
    Link(Link),
}

impl Config {
    pub fn items(table: LuaTable) -> LuaResult<Arc<[ConfigItem]>> {
        table.sequence_values::<ConfigItem>().collect()
    }
}

impl FromLua for ConfigItem {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        if let LuaValue::UserData(ud) = &value {
            if ud.is::<Arc<Config>>() {
                return Ok(Self::ChildConfig(ud.borrow::<Arc<Config>>()?.clone()));
            }

            if ud.is::<Link>() {
                return Ok(Self::Link(ud.borrow::<Link>()?.clone()));
            }
        }

        return Err(LuaError::FromLuaConversionError {
            from: value.type_name(),
            to: "ConfigItem".into(),
            // TODO: would be nice to generate?
            message: Some("script must return a table array with: Config, Link".into()),
        });
    }
}

#[mlua::userdata_impl]
impl Config {
    #[lua(meta, infallible)]
    fn __tostring(&self) -> String {
        format!("Config({})", self.script_path.display())
    }

    #[lua(meta, infallible)]
    fn __todebugstring(&self) -> String {
        format!("{:#?}", self)
    }
}
