use std::{
    os::unix::fs,
    path::{Path, PathBuf},
};

use mlua::FromLua;

use crate::{
    args::AppArgs,
    error::{AppError, AppResult},
};

#[derive(Debug)]
pub struct Link {
    pub name: String,
    pub src: PathBuf,
    pub dest: PathBuf,
    pub enable: bool,
    pub force: bool,
}

impl FromLua for Link {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::Table(link) => Ok(Link {
                name: link.get("name")?,
                src: link.get("src")?,
                dest: link.get("dest")?,
                force: link.get("force").unwrap_or(false),
                enable: link.get("enable").unwrap_or(true),
            }),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: "Not a table",
                to: "Table".into(),
                message: None,
            }),
        }
    }
}

impl Link {
    fn validate_location(&self) -> AppResult<(PathBuf, PathBuf)> {
        let src = self
            .src
            .canonicalize()
            .map_err(|_| AppError::LocationNotFound {
                field_name: "src",
                table_name: self.name.clone(),
            })?;

        let dest = self.dest.clone();

        // check if dest folder exists
        self.dest
            .parent()
            .unwrap_or(Path::new(""))
            .canonicalize()
            .map_err(|_e| AppError::LocationNotFound {
                field_name: "dest",
                table_name: self.name.clone(),
            })?;

        Ok((src, dest))
    }
    pub fn create_link(&self, args: &AppArgs) -> AppResult<()> {

        let (src,dest) = self.validate_location()?;

        println!("[#] Creating symlink: {dest:?}");
        if dest.exists() {
            if self.force && !args.update {
                println!("[!] Link Destination already exists..");
                println!("[#] Overwritting: {dest:?}\n");

                if dest.is_dir() && !dest.is_symlink() {
                    std::fs::remove_dir_all(&dest)?;
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
