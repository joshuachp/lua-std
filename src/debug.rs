//! Module for debug utilities
use std::collections::BTreeMap;

use mlua::prelude::*;

pub fn inspect(lua: &Lua, value: LuaValue) -> LuaResult<String> {
    let value = match value {
        LuaNil => String::from("nil"),
        LuaValue::Boolean(_b @ true) => String::from("true"),
        LuaValue::Boolean(_b @ false) => String::from("false"),
        LuaValue::Integer(integer) => format!("{}", integer),
        LuaValue::Number(number) => format!("{}", number),
        LuaValue::String(string) => format!(r#""{}""#, string.to_str()?),
        LuaValue::Table(table) => {
            let mut str = String::from("{");
            let mut sorted_table: BTreeMap<String, String> = BTreeMap::new();
            for pair in table.pairs() {
                let (key, value) = pair?;
                let key = inspect(lua, key)?;
                let value = inspect(lua, value)?;
                sorted_table.insert(key, value);
            }
            let mut pairs = sorted_table.iter();
            if let Some((key, value)) = pairs.next() {
                str.push_str(&format!(" {} = {}", key, value));
                for (key, value) in pairs {
                    str.push_str(&format!(", {} = {}", key, value));
                }
            }
            str.push_str(" }");
            str
        }
        LuaValue::Function(function) => {
            String::from_utf8(function.dump(false)).expect("Error parsing UTF-8")
        }
        LuaValue::Thread(_) => String::from("thread"),
        LuaValue::UserData(data) => inspect(lua, data.to_lua(lua)?)?,
        LuaValue::LightUserData(data) => inspect(lua, data.to_lua(lua)?)?,
        LuaValue::Error(err) => format!("Error({})", err.to_string()),
    };
    Ok(value)
}

/// Lua module exporting table functions
#[cfg(feature = "module")]
pub fn debug(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;
    module.set("inspect", lua.create_function(inspect)?)?;
    Ok(module)
}

#[cfg(test)]
mod test {
    use mlua::prelude::*;

    use super::*;

    #[test]
    fn should_inspect_table() {
        let str = r#"{ [1] = 3, a = "b", c = { d = false } }"#;
        let lua = Lua::new();
        let table: LuaValue = lua.load(str).eval().unwrap();
        let result = inspect(&lua, table).unwrap();
        assert_eq!(result, r#"{ "a" = "b", "c" = { "d" = false }, 1 = 3 }"#);
    }
}
