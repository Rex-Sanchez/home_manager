use std::{fmt::Display, io};

impl std::error::Error for LuaParseError {}

pub type AppResult<T> = std::result::Result<T, LuaParseError>;

#[derive(Debug)]
pub enum LuaParseError {
    FieldHasNoName(usize),
    MissingField {
        field_name: &'static str,
        table_name: String,
    },
    UnableToLoadLuaConfig(mlua::Error),
    ConfigNotFound(io::Error),
    LocationNotFound {
        field_name: &'static str,
        table_name: String,
    },
    IOError(io::Error),
}

impl From<io::Error> for LuaParseError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}

impl Display for LuaParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LuaParseError::FieldHasNoName(index) => f.write_str(&format!(
                        "![ Warning ] ->> Table at index: {index}, has no field name, skipping!"
                    )),
            LuaParseError::MissingField {
                        field_name,
                        table_name,
                    } => f.write_str(&format!(
                        "![ Warning ] ->> Field \"{field_name}\" in Link table \"{table_name}\" is required, skipping!"
                    )),
            LuaParseError::UnableToLoadLuaConfig(e) => f.write_str(&format!(
                        "![ Error ] ->> There was a error loading the lua config:\n {e}"
                    )),
            LuaParseError::ConfigNotFound(_) => f.write_str(&format!("![ Error ] ->> Config not found!")),
            LuaParseError::LocationNotFound { field_name, table_name } => 
                        f.write_str(&format!("![ Error ] ->> Location not found for field {field_name} for table {table_name}, skipping!")),
            LuaParseError::IOError(error) => f.write_str(&format!("{error}")),
        }
    }
}
