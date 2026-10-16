use std::sync::Arc;

use mlua::prelude::*;

use super::path::Path;
use crate::model::SystemInfo;

#[derive(LuaUserData)]
pub struct Windows(#[lua(skip)] pub Arc<SystemInfo>);

#[mlua::userdata_impl]
impl Windows {
    #[lua(getter, infallible)]
    fn local_app_data(&self) -> Path {
        Path(self.0.windows.local_app_data.clone())
    }

    #[lua(getter, infallible)]
    fn roaming_app_data(&self) -> Path {
        Path(self.0.windows.roaming_app_data.clone())
    }

    #[lua(getter, infallible)]
    fn local_low_app_data(&self) -> Path {
        Path(self.0.windows.local_low_app_data.clone())
    }

    #[lua(getter, infallible)]
    fn documents(&self) -> Path {
        Path(self.0.windows.documents.clone())
    }

    #[lua(meta, infallible)]
    fn __tostring(&self) -> String {
        "<Windows>".to_string()
    }
}
