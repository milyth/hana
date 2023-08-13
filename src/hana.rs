use anyhow::Result;
use mlua::{Lua, String, Table};

use crate::{lua::Setup, map_function};

use super::lua::FileSystem;

fn put(_lua: &Lua, setup: &Setup, (filename, content): (String, String)) -> Result<()> {
    let filesystem = FileSystem::new(&setup.root);

    filesystem.write(filename.to_str()?, content)
}

fn to_toml<'a, 'b>(lua: &'b Lua, _setup: &'a Setup, item: Table) -> Result<String<'b>> {
    Ok(lua.create_string(&toml::to_string_pretty(&item)?)?)
}

fn to_ini<'a, 'b>(lua: &'b Lua, _setup: &'a Setup, item: Table) -> Result<String<'b>> {
    Ok(lua.create_string(&serde_ini::to_string(&item)?)?)
}

fn makedir_all(_lua: &Lua, setup: &Setup, item: String) -> Result<()> {
    let filesystem = FileSystem::new(&setup.root);
    filesystem.create_dir_all(item.to_str()?)
}

pub fn init<'a>(lua: &'a Lua, setup: &'a Setup) -> Result<()> {
    let hana = lua.create_table()?;

    hana.set("writeFile", lua.create_function(map_function!(setup, put))?)?;
    hana.set(
        "toToml",
        lua.create_function(map_function!(setup, to_toml))?,
    )?;
    hana.set("toIni", lua.create_function(map_function!(setup, to_ini))?)?;
    hana.set(
        "makeDirAll",
        lua.create_function(map_function!(setup, makedir_all))?,
    )?;

    lua.globals().set("Hana", hana)?;
    Ok(())
}
