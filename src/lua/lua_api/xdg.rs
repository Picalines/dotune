use std::sync::Arc;

use mlua::prelude::*;

use super::path::Path;
use crate::model::SystemInfo;

#[derive(LuaUserData)]
pub struct Xdg(#[lua(skip)] pub Arc<SystemInfo>);

#[mlua::userdata_impl]
impl Xdg {
    #[lua(getter, infallible)]
    fn config(&self) -> Path {
        Path(self.0.xdg.config.clone())
    }

    #[lua(getter, infallible)]
    fn data(&self) -> Path {
        Path(self.0.xdg.data.clone())
    }

    #[lua(getter, infallible)]
    fn state(&self) -> Path {
        Path(self.0.xdg.state.clone())
    }

    #[lua(getter, infallible)]
    fn cache(&self) -> Path {
        Path(self.0.xdg.cache.clone())
    }

    #[lua(meta, infallible)]
    fn __tostring(&self) -> String {
        "<XDG>".to_string()
    }
}
