use std::sync::Arc;

use mlua::prelude::*;

use super::utils::bad_argument;
use crate::model::SystemInfo;

#[derive(LuaUserData)]
pub struct Os {
    #[lua(get)]
    r#match: LuaFunction,
}

#[mlua::userdata_impl]
impl Os {
    #[lua(meta, infallible)]
    fn __tostring(&self) -> String {
        "<OS>".to_string()
    }
}

impl Os {
    pub fn new(lua: &Lua, system_info: Arc<SystemInfo>) -> LuaResult<Self> {
        let r#match = lua
            .create_function(move |_, options: LuaTable| Self::match_impl(&system_info, options))?;
        Ok(Self { r#match })
    }

    fn match_impl(system_info: &SystemInfo, options: LuaTable) -> LuaResult<LuaValue> {
        for pair in options.pairs::<LuaValue, LuaValue>() {
            let (key, _) = pair?;

            let LuaValue::String(key) = key else {
                return Err(bad_argument(
                    "OS.match",
                    1,
                    Some("options"),
                    &format!("OS key must be a string, got {}", key.type_name()),
                ));
            };

            if !matches!(
                key.as_bytes().as_ref(),
                b"windows" | b"macos" | b"linux" | b"unix" | b"_"
            ) {
                return Err(bad_argument(
                    "OS.match",
                    1,
                    Some("options"),
                    &format!("unknown OS key {:?}", key.to_string_lossy()),
                ));
            }
        }

        let result = [
            ("_", true),
            ("unix", system_info.is_unix),
            (system_info.os, true),
        ]
        .into_iter()
        .filter(|(_, applies)| *applies)
        .try_fold(None, |result, (key, _)| {
            options
                .raw_get::<Option<LuaValue>>(key)
                .map(|value| value.or(result))
        })?;

        result.ok_or_else(|| {
            bad_argument(
                "OS.match",
                1,
                Some("options"),
                &format!("no matching value for OS '{}'", system_info.os),
            )
        })
    }
}
