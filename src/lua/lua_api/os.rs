use std::sync::Arc;

use mlua::prelude::*;

use super::utils::bad_argument;
use crate::model::SystemInfo;

#[derive(LuaUserData)]
pub struct Os {
    #[lua(get)]
    r#match: LuaFunction,
    #[lua(get)]
    matches: LuaFunction,
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
        let patterns = [
            ("windows", system_info.os == "windows"),
            ("macos", system_info.os == "macos"),
            ("linux", system_info.os == "linux"),
            ("unix", system_info.is_unix),
            ("_", true),
        ];

        let matches = lua.create_function(move |_, key: LuaString| {
            patterns
                .iter()
                .find(|(name, _)| key.as_bytes().as_ref() == name.as_bytes())
                .map(|(_, matches)| *matches)
                .ok_or_else(|| {
                    bad_argument(
                        "OS.matches",
                        1,
                        Some("key"),
                        &format!("unknown OS key {:?}", key.to_string_lossy()),
                    )
                })
        })?;

        let r#match = lua.create_function(move |_, options: LuaTable| {
            for pair in options.pairs::<LuaString, LuaValue>() {
                let (key, _) = pair?;

                if !patterns
                    .iter()
                    .any(|(name, _)| key.as_bytes().as_ref() == name.as_bytes())
                {
                    return Err(bad_argument(
                        "OS.match",
                        1,
                        Some("options"),
                        &format!("unknown OS key {:?}", key.to_string_lossy()),
                    ));
                }
            }

            for (key, applies) in patterns {
                if applies && let Some(value) = options.raw_get::<Option<LuaValue>>(key)? {
                    return Ok(value);
                }
            }

            Err(bad_argument(
                "OS.match",
                1,
                Some("options"),
                &format!("no matching value for OS '{}'", system_info.os),
            ))
        })?;

        Ok(Self { r#match, matches })
    }
}
