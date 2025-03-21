use std::{fmt::Display, io};

impl std::error::Error for AppError {}

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    ConfigNotFound,
    LocationNotFound {
        field_name: &'static str,
        table_name: String,
    },
    IOError(io::Error),
    FromLua(mlua::Error),
}

impl From<io::Error> for AppError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}
impl From<mlua::Error> for AppError {
    fn from(value: mlua::Error) -> Self {
       Self::FromLua(value)
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::ConfigNotFound => f.write_str(&format!("[e] Config not found!")),
            AppError::LocationNotFound { field_name, table_name } => f.write_str(&format!("[e] Path Not found for field: \"{field_name}\" for table: \"{table_name}\", skipping!")),
            AppError::IOError(error) => f.write_str(&format!("{error}")),
            AppError::FromLua(error) => f.write_str(&format!("{error}")),
        }
    }
}
