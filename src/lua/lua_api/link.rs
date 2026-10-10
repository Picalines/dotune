use std::sync::Arc;

use mlua::prelude::*;

use super::path::Path;
use super::utils::bad_argument;
use crate::model::AbsolutePath;

#[derive(LuaUserData, Clone, Debug)]
pub struct Link {
    #[lua(skip)]
    pub source_path: Arc<AbsolutePath>,
    #[lua(skip)]
    pub destination_path: Arc<AbsolutePath>,
}

#[derive(LuaUserData)]
pub struct LinkFactory;

#[mlua::userdata_impl]
impl LinkFactory {
    #[lua(meta)]
    fn __call(&self, arg: LuaTable) -> LuaResult<Link> {
        let mut arg_paths = arg.sequence_values::<Path>();

        let (Some(Ok(path1)), Some(Ok(path2))) = (arg_paths.next(), arg_paths.next()) else {
            return Err(bad_argument("Link", 1, None, "expected two paths"));
        };

        Ok(Link {
            source_path: path1.0,
            destination_path: path2.0,
        })
    }
}

#[mlua::userdata_impl]
impl Link {
    #[lua(meta, infallible)]
    fn __tostring(&self) -> String {
        format!(
            "Link({}, {})",
            self.source_path.display(),
            self.destination_path.display()
        )
    }
}
