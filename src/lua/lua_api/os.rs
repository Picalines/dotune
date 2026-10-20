use mlua::prelude::*;

use super::super::lua_app_data::LuaAppData;
use super::utils::bad_argument;

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
    pub fn new(lua: &Lua) -> LuaResult<Self> {
        let (os, is_unix) = {
            let app_data = LuaAppData::get(lua);
            let system_info = &app_data.system_info;
            (system_info.os, system_info.is_unix)
        };

        let patterns = [
            ("windows", os == "windows"),
            ("macos", os == "macos"),
            ("linux", os == "linux"),
            ("unix", is_unix),
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
                &format!("no matching value for OS '{os}'"),
            ))
        })?;

        Ok(Self { r#match, matches })
    }
}
