use mlua::prelude::*;

use super::super::lua_app_data::LuaAppData;
use super::path::Path;

#[derive(LuaUserData)]
pub struct Windows;

#[mlua::userdata_impl]
impl Windows {
    #[lua(getter, infallible)]
    fn local_app_data(&self, lua: &Lua) -> Path {
        Path(
            LuaAppData::get(lua)
                .system_info
                .windows
                .local_app_data
                .clone(),
        )
    }

    #[lua(getter, infallible)]
    fn roaming_app_data(&self, lua: &Lua) -> Path {
        Path(
            LuaAppData::get(lua)
                .system_info
                .windows
                .roaming_app_data
                .clone(),
        )
    }

    #[lua(getter, infallible)]
    fn local_low_app_data(&self, lua: &Lua) -> Path {
        Path(
            LuaAppData::get(lua)
                .system_info
                .windows
                .local_low_app_data
                .clone(),
        )
    }

    #[lua(getter, infallible)]
    fn documents(&self, lua: &Lua) -> Path {
        Path(LuaAppData::get(lua).system_info.windows.documents.clone())
    }

    #[lua(meta, infallible)]
    fn __tostring(&self) -> String {
        "<Windows>".to_string()
    }
}
