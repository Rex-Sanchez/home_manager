mod args;
mod error;
mod lua_engine;

use std::{
    fs::read_to_string,
    os::unix::fs,
    path::{Path, PathBuf},
};

use args::AppArgs;
use clap::Parser;
use error::{AppResult, LuaParseError};
use lua_engine::LuaEngine;

#[derive(Debug)]
pub struct Link {
    pub name: String,
    pub src: PathBuf,
    pub dest: PathBuf,
    pub enable: bool,
    pub force: bool,
}

impl Link {
    pub fn create_link(&self, args: &AppArgs) -> AppResult<()> {
        let src = self
            .src
            .canonicalize()
            .map_err(|_| LuaParseError::LocationNotFound {
                field_name: "src",
                table_name: self.name.clone(),
            })?;

        let dest = self.dest.clone();

        // check if dest folder exists
        self.dest
            .parent()
            .unwrap_or(Path::new(""))
            .canonicalize()
            .map_err(|_e| LuaParseError::LocationNotFound {
                field_name: "dest",
                table_name: self.name.clone(),
            })?;

        println!("[#] Creating symlink: {dest:?}");
        if dest.exists() {

            if self.force && !args.update {
                println!("[!] Link Destination already exists..");
                println!("[#] Overwritting: {dest:?}\n");
                if dest.is_dir() && !dest.is_symlink() {
                    std::fs::remove_dir(&dest)?;
                } else if dest.is_file() || dest.is_symlink() {
                    std::fs::remove_file(&dest)?
                }
                fs::symlink(src, dest)?;
            } else {
                println!("[!] Link Destination already exists. Skipping!");
            }
        } else {
            fs::symlink(src, dest)?;
        }
        Ok(())
    }
}

fn main() -> AppResult<()> {
    let args = AppArgs::parse();
    let config_str =
        read_to_string(args.config.canonicalize()?).map_err(LuaParseError::ConfigNotFound)?;

    let lua = LuaEngine::new();
    let _ = lua.set_globals(&args);

    match lua.parse_config(&config_str) {
        Ok(links) => {
            links.into_iter().for_each(|e| {
                e.create_link(&args).unwrap();
            });
        }
        Err(e) => {
            eprintln!("{e}");
        }
    };

    Ok(())
}
