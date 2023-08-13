use std::path::PathBuf;
mod lua;
use anyhow::{Context, Result};
use argh::FromArgs;
use lua::Setup;
use mlua::Lua;
use std::fs::read;
use tracing_subscriber::EnvFilter;
mod hana;

#[derive(FromArgs)]
/// Declare your system
struct Cli {
    #[argh(option, short = 's')]
    /// the source configuration
    source: PathBuf,

    #[argh(option, short = 'r')]
    /// the os root
    root: PathBuf,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive("hana=info".parse()?)
                .from_env_lossy(),
        )
        .pretty()
        .init();

    let cli: Cli = argh::from_env();
    let config = Setup {
        root: cli.root.canonicalize().context("invalid root directory")?,
    };

    let lua = Lua::new();
    hana::init(&lua, &config).context("unable to load Hana integration")?;

    let init_path = cli.source.canonicalize()?.join("init.lua");
    let contents = read(&init_path).context("failed to read init.lua")?;

    lua.load(&contents)
        .set_name(init_path.as_os_str().to_str().context("invalid str")?)?
        .exec()?;

    Ok(())
}
