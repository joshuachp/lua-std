//! Module for file system utilities
use std::fs;

use mlua::prelude::*;

/// Read the content of a directory returning a string of the file names
pub fn read_dir(lua: &Lua, path: String) -> LuaResult<LuaTable> {
    let content: Vec<String> = fs::read_dir(path)?
        .map(|entry| {
            entry
                .expect("Error reading file")
                .file_name()
                .to_str()
                // This shouldn't happen since the OS sting is from the directory name
                .expect("Invalid unicode in path")
                .to_string()
        })
        .collect();
    lua.create_sequence_from(content)
}

/// Lua module exporting fs functions
#[cfg(feature = "module")]
#[mlua::lua_module]
pub fn fs(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;
    module.set("read_dir", lua.create_function(read_dir)?)?;
    Ok(module)
}
