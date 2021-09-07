//! Lua is missing a std library for common function, this is a gradula implementation in rust of ffi functions you don't want ot write twice.

use mlua::prelude::*;

pub mod fs;

#[mlua::lua_module]
fn lua_std(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;
    module.set("fs", fs::fs(lua)?)?;
    Ok(module)
}
