use super::FileSystem;
use crate::lua::Setup;
use anyhow::Result;
use mlua::{Lua, String as LuaString, Table};

fn callback(_: &Lua, setup: &Setup, table: Table) -> Result<()> {
    tracing::info!("Writing /etc");

    let filesystem = FileSystem::new(&setup.root);
    filesystem.create_dir_all("etc")?;

    if let Ok(hostname) = table.get::<_, LuaString>("hostname") {
        filesystem.write("etc/hostname", &hostname.as_bytes())?;
    }

    Ok(())
}

pub fn init<'a>(lua: &'a Lua, setup: &'a Setup) -> Result<()> {
    lua.globals().set(
        "etc",
        lua.create_function({
            let setup = setup.clone();
            move |lua, table: Table| {
                callback(lua, &setup, table)
                    .map_err(|e| mlua::Error::RuntimeError(format!("{e:?}")))
            }
        })?,
    )?;
    Ok(())
}
