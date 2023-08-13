use std::ffi::CString;

use anyhow::{bail, Context, Result};
use mlua::{Lua, String, Table};
use nix::unistd::{getgrouplist, Group, User};

use crate::{lua::Setup, map_function};

use super::lua::FileSystem;

fn put(_lua: &Lua, setup: &Setup, (filename, content): (String, String)) -> Result<()> {
    let filesystem = FileSystem::new(&setup.root);

    filesystem.write(filename.to_str()?, content)
}

fn to_toml<'b>(lua: &'b Lua, _setup: &Setup, item: Table) -> Result<String<'b>> {
    Ok(lua.create_string(&toml::to_string_pretty(&item)?)?)
}

fn to_ini<'b>(lua: &'b Lua, _setup: &Setup, item: Table) -> Result<String<'b>> {
    Ok(lua.create_string(&serde_ini::to_string(&item)?)?)
}

fn makedir_all(_lua: &Lua, setup: &Setup, item: String) -> Result<()> {
    let filesystem = FileSystem::new(&setup.root);
    filesystem.create_dir_all(item.to_str()?)
}

fn getuserbyname<'b>(lua: &'b Lua, _setup: &Setup, name: String) -> Result<Option<Table<'b>>> {
    // FIXME: Use etc/passwd from target

    let table = lua.create_table()?;
    let name = name.to_str()?;
    let Some(user) = User::from_name(name)? else {
        return Ok(None);
    };

    let groups = getgrouplist(CString::new(user.name.clone())?.as_c_str(), user.gid)?
        .into_iter()
        .map(Group::from_gid)
        .flatten()
        .flatten()
        .map(|i| i.name)
        .collect::<Vec<_>>();

    table.set("name", user.name)?;
    table.set(
        "shell",
        user.shell.to_str().context("Shell path isn't UTF-8")?,
    )?;
    table.set("groupId", user.gid.as_raw())?;
    table.set("homeDir", user.dir.to_str().context("Path isn't UTF_8")?)?;
    table.set("groups", groups)?;

    Ok(Some(table))
}

fn assert(_lua: &Lua, _setup: &Setup, (item, why): (bool, String)) -> Result<()> {
    if !item {
        let why = why.to_str()?.to_string();
        bail!(why);
    }

    Ok(())
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

    hana.set("assert", lua.create_function(map_function!(setup, assert))?)?;

    hana.set(
        "getUserByName",
        lua.create_function(map_function!(setup, getuserbyname))?,
    )?;

    lua.globals().set("Hana", hana)?;
    Ok(())
}
