mod args;
mod error;
mod link;
mod lua_engine;

use std::fs::read_to_string;

use args::AppArgs;
use clap::Parser;
use error::{AppError, AppResult};
use lua_engine::LuaEngine;

fn main() -> AppResult<()> {
    let args = AppArgs::parse();
    let config_str =
        read_to_string(args.config.canonicalize()?).map_err(|_| AppError::ConfigNotFound)?;

    let lua = LuaEngine::new();

    lua.set_globals(&args)?;
    lua.load(&config_str)?;

    Ok(())
}
