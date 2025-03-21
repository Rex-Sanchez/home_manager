use std::{
    path::{Path, PathBuf},
    process::Command,
};

use mlua::{Lua, Table};

use crate::{args::AppArgs, error::AppResult, link::Link};

trait MapJoin {
    fn map_join(&self, s: &str) -> Option<PathBuf>;
}

impl MapJoin for Option<PathBuf> {
    fn map_join(&self, s: &str) -> Option<PathBuf> {
        self.as_ref()
            .map(|e| e.join(Path::new(s)).canonicalize().ok())
            .flatten()
    }
}

pub(crate) struct LuaEngine {
    pub lua: Lua,
}
impl LuaEngine {
    pub fn new() -> Self {
        Self { lua: Lua::new() }
    }
    pub fn set_globals(&self, args: &AppArgs) -> AppResult<()> {
        let table = self.lua.create_table()?;
        let utils = self.gen_utils_functions(args.clone())?;

        let home_dir = std::env::var("HOME").ok().map(|e| PathBuf::from(e));

        let script_dir = args
            .config
            .canonicalize()
            .map(|e| e.parent().map(|e| e.to_owned()))
            .ok()
            .flatten();

        let theme_dir = home_dir.map_join(".themes");
        let icon_dir = home_dir.map_join(".local/share/icons");
        let font_dir = home_dir.map_join(".local/share/fonts");
        let config_dir = home_dir.map_join(".config");

        table.set("HOME_DIR", home_dir)?;
        table.set("SCRIPT_DIR", script_dir)?;
        table.set("THEME_DIR", theme_dir)?;
        table.set("ICON_DIR", icon_dir)?;
        table.set("FONT_DIR", font_dir)?;
        table.set("CONFIG_DIR", config_dir)?;

        self.lua.globals().set("env", table)?;
        self.lua.globals().set("utils", utils)?;

        Ok(())
    }

    pub fn gen_utils_functions(&self, args: AppArgs) -> AppResult<Table> {
        let table = self.lua.create_table()?;
        table.set(
            "linker",
            self.lua
                .create_function(move |_, links: Vec<Link>| link_config_fn(&args, links))?,
        )?;

        table.set(
            "setFont",
            self.lua
                .create_function(move |_, (family, size): (String, f32)| {
                    set_font_fn(family, size)
                })?,
        )?;
        
        table.set(
            "setFontMonospace",
            self.lua
                .create_function(move |_, (family, size): (String, f32)| {
                    set_font_mono_fn(family, size)
                })?,
        )?;

        table.set(
            "setGtkIcons",
            self.lua
                .create_function(move |_, icon_name: String| set_icons_fn(icon_name))?,
        )?;

        table.set(
            "setGtkTheme",
            self.lua
                .create_function(move |_, theme_name: String| set_gtk_theme_fn(theme_name))?,
        )?;

        table.set(
            "setQtTheme",
            self.lua
                .create_function(move |_, theme_name: String| set_qt_theme_fn(theme_name))?,
        )?;

        Ok(table)
    }
    pub fn load(&self, config: &str) -> Result<(), mlua::Error> {
        self.lua.load(config).exec()
    }
}

fn set_font_mono_fn(family: String, size: f32) -> mlua::Result<()> {
    Command::new("dconf")
        .args([
            "write",
            "/org/gnome/desktop/interface/monospace-font-name",
            &format!("\"'{family} {size}'\""),
        ])
        .output()?;

    Ok(())
}

fn set_font_fn(family: String, size: f32) -> mlua::Result<()> {
    Command::new("dconf")
        .args([
            "write",
            "/org/gnome/desktop/interface/font-name",
            &format!("\"'{family} {size}'\""),
        ])
        .output()?;

    Ok(())
}

fn set_gtk_theme_fn(theme_name: String) -> mlua::Result<()> {
    Command::new("dconf")
        .args([
            "write",
            "/org/gnome/desktop/interface/gtk-theme",
            &format!("\"'{theme_name}'\""),
        ])
        .output()?;

    Ok(())
}
fn set_icons_fn(icon_name: String) -> mlua::Result<()> {
    Command::new("dconf")
        .args([
            "write",
            "/org/gnome/desktop/interface/icon-theme",
            &format!("\"'{icon_name}'\""),
        ])
        .output()?;

    Ok(())
}

fn set_qt_theme_fn(theme_name: String) -> mlua::Result<()> {
    todo!()
}

fn link_config_fn(args: &AppArgs, links: Vec<Link>) -> mlua::Result<()> {
    links.iter().for_each(|link| {
        if link.enable {
            if let Err(e) = link.create_link(&args) {
                eprintln!("{e}")
            }
        }
    });
    Ok(())
}
