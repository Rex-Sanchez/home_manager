use std::path::{Path, PathBuf};

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

        Ok(table)
    }
    pub fn load(&self, config: &str) -> Result<(), mlua::Error> {
        self.lua.load(config).exec()
    }
}




fn link_config_fn(args: &AppArgs, links: Vec<Link>) -> mlua::Result<()> {
    links.iter().for_each(|link| {
        if link.enable {
            let _ = link.create_link(&args);
        }
    });
    Ok(())
}
