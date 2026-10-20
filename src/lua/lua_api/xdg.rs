use mlua::prelude::*;

use super::super::lua_app_data::LuaAppData;
use super::path::Path;

#[derive(LuaUserData)]
pub struct Xdg;

#[mlua::userdata_impl]
impl Xdg {
    #[lua(getter, infallible)]
    fn config(&self, lua: &Lua) -> Path {
        Path(LuaAppData::get(lua).system_info.xdg.config.clone())
    }

    #[lua(getter, infallible)]
    fn data(&self, lua: &Lua) -> Path {
        Path(LuaAppData::get(lua).system_info.xdg.data.clone())
    }

    #[lua(getter, infallible)]
    fn state(&self, lua: &Lua) -> Path {
        Path(LuaAppData::get(lua).system_info.xdg.state.clone())
    }

    #[lua(getter, infallible)]
    fn cache(&self, lua: &Lua) -> Path {
        Path(LuaAppData::get(lua).system_info.xdg.cache.clone())
    }

    #[lua(meta, infallible)]
    fn __tostring(&self) -> String {
        "<XDG>".to_string()
    }
}
