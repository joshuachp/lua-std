//! Module for string manipulation

use mlua::prelude::*;

pub fn trim(_: &Lua, string: String) -> LuaResult<String> {
    Ok(string.trim().to_string())
}

/// Lua module exporting fs functions
#[cfg(feature = "module")]
pub fn string(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;
    module.set("trim", lua.create_function(trim)?)?;
    Ok(module)
}

#[cfg(test)]
mod test {
    use mlua::prelude::*;

    use super::*;

    #[test]
    fn should_trim() {
        let str = "'  foo '";
        let lua = Lua::new();
        let trim_fn = lua.create_function(trim).unwrap();
        let string: LuaString = lua.load(str).eval().unwrap();
        let result: String = trim_fn.call(string).unwrap();
        assert_eq!(result, "foo")
    }
}
