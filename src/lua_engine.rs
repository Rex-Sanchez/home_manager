use std::path::{Path, PathBuf};

use mlua::{Lua, Table};

use crate::{
    Link,
    args::AppArgs,
    error::{AppResult, LuaParseError},
};

impl TryFrom<Result<(usize, Table), mlua::Error>> for Link {
    type Error = LuaParseError;
    fn try_from(link: Result<(usize, Table), mlua::Error>) -> Result<Self, Self::Error> {
        let (i, link) = link.map_err(LuaParseError::UnableToLoadLuaConfig)?;

        let name: String = link
            .get("name")
            .map_err(|_| LuaParseError::FieldHasNoName(i))?;

        Ok(Link {
            name: name.clone(),
            src: link.get("src").map_err(|_| LuaParseError::MissingField {
                field_name: "src",
                table_name: name.clone(),
            })?,
            dest: link.get("dest").map_err(|_| LuaParseError::MissingField {
                field_name: "dest",
                table_name: name.clone(),
            })?,
            force: link.get("force").unwrap_or(false),
            enable: link.get("enable").unwrap_or(true),
        })
    }
}

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
    pub fn set_globals(&self, args: &AppArgs) -> Result<(), mlua::Error> {
        let table = self.lua.create_table()?;

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

        Ok(())
    }
    pub fn parse_config(&self, config: &str) -> AppResult<Vec<Link>> {
        let chuck = self
            .lua
            .load(config)
            .eval::<Table>()
            .map_err(LuaParseError::UnableToLoadLuaConfig)?;

        let links = chuck
            .pairs::<usize, Table>()
            .map(Link::try_from)
            .collect::<Vec<AppResult<Link>>>();

        links.iter().for_each(|link| {
            if let Err(e) = link {
                eprintln!("{e}")
            }
        });

        Ok(links.into_iter().flatten().collect::<Vec<Link>>())
    }
}
