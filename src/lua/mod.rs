use std::path::PathBuf;

use anyhow::{Context, Result};
use mlua::Lua;

mod modules;
use modules::etc;

#[derive(Clone)]
pub struct Setup {
    pub root: PathBuf,
}

pub fn init(setup: &Setup) -> Result<Lua> {
    let lua = Lua::new();

    etc::init(&lua, setup).context("unable to load etc module")?;

    Ok(lua)
}
