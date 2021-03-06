//! Lua is missing a std library for common function, this is a gradually implementation in rust of ffi functions you don't want to write twice.

#[cfg(feature = "module")]
use mlua::prelude::*;

pub mod debug;
pub mod fs;
pub mod string;

/// Lua module entry point
#[cfg(feature = "module")]
#[mlua::lua_module]
fn std(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;
    module.set("debug", debug::debug(lua)?)?;
    module.set("fs", fs::fs(lua)?)?;
    module.set("string", string::string(lua)?)?;
    Ok(module)
}
