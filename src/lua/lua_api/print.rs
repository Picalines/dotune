use std::io::{self, Write};

use mlua::prelude::*;

pub(super) fn create_print(lua: &Lua) -> LuaResult<LuaFunction> {
    let tostring: LuaFunction = lua.globals().get("tostring")?;

    lua.create_function(move |_, values: LuaVariadic<LuaValue>| {
        let mut stderr = io::stderr();

        for (index, value) in values.into_iter().enumerate() {
            let text: LuaString = tostring.call(value)?;
            if index > 0 {
                stderr.write_all(b"\t")?;
            }
            stderr.write_all(&text.as_bytes())?;
        }

        stderr.write_all(b"\n")?;

        Ok(())
    })
}
