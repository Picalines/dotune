use std::sync::Arc;

use mlua::prelude::*;

use super::path::Path;
use crate::model::SystemInfo;

#[derive(LuaUserData)]
pub struct Mac(#[lua(skip)] pub Arc<SystemInfo>);

#[mlua::userdata_impl]
impl Mac {
    #[lua(getter, infallible)]
    fn applications(&self) -> Path {
        Path(self.0.mac.applications.clone())
    }

    #[lua(meta, infallible)]
    fn __tostring(&self) -> String {
        "<Mac>".to_string()
    }
}
