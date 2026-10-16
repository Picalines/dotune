use std::env;

use mlua::prelude::*;

use super::utils::bad_argument;

#[derive(LuaUserData)]
pub struct Os;

#[mlua::userdata_impl]
impl Os {
    #[lua]
    fn r#match(options: LuaTable) -> LuaResult<LuaValue> {
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
                b"windows" | b"macos" | b"linux" | b"unix" | b"__"
            ) {
                return Err(bad_argument(
                    "OS.match",
                    1,
                    Some("options"),
                    &format!("unknown OS key {:?}", key.to_string_lossy()),
                ));
            }
        }

        let result = [("__", true), ("unix", cfg!(unix)), (env::consts::OS, true)]
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
                &format!("no matching value for OS '{}'", env::consts::OS),
            )
        })
    }
}
