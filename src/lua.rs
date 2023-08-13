use anyhow::{Context, Result};
use std::fs::{create_dir_all, write};
use std::path::PathBuf;

pub struct FileSystem<'fs> {
    root: &'fs PathBuf,
}

impl<'fs> FileSystem<'fs> {
    pub fn new(root: &'fs PathBuf) -> Self {
        Self { root }
    }

    pub fn create_dir_all(&self, path: &str) -> Result<()> {
        let target = self.root.join(path);

        tracing::debug!("mkdir {target:?}");

        create_dir_all(&target).with_context(|| {
            format!(
                "unable to create {}!",
                target.as_os_str().to_str().unwrap_or("<unknown>")
            )
        })
    }

    pub fn write<C: AsRef<[u8]>>(&self, path: &str, contents: C) -> Result<()> {
        let target = self.root.join(path);

        tracing::debug!("write {target:?}");

        write(&target, contents).with_context(|| {
            format!(
                "unable to write {}!",
                target.as_os_str().to_str().unwrap_or("<unknown>")
            )
        })
    }
}

#[macro_export]
macro_rules! map_function {
    ($setup:ident, $fn:ident) => {{
        let setup = $setup.clone();
        move |lua, arg| {
            $fn(&lua, &setup, arg).map_err(move |e| mlua::Error::RuntimeError(format!("{e:?}")))
        }
    }};
}

#[derive(Clone)]
pub struct Setup {
    pub root: PathBuf,
}
