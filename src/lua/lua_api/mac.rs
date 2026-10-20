use mlua::prelude::*;

use super::super::lua_app_data::LuaAppData;
use super::path::Path;

#[derive(LuaUserData)]
pub struct Mac;

#[mlua::userdata_impl]
impl Mac {
    #[lua(getter, infallible)]
    fn applications(&self, lua: &Lua) -> Path {
        Path(LuaAppData::get(lua).system_info.mac.applications.clone())
    }

    #[lua(meta, infallible)]
    fn __tostring(&self) -> String {
        "<Mac>".to_string()
    }
}
